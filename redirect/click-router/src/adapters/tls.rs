//! HTTPS certificate selection by requested server name.

use std::{fs, future::Future, io, pin::Pin, sync::Arc, time::Duration};

use anyhow::{bail, Context, Result};
use axum_server::accept::Accept;
use lazy_static::lazy_static;
use moka::sync::Cache;
use prometheus::{register_int_counter_vec, IntCounterVec};
use rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    server::Acceptor,
    ServerConfig,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    time::timeout,
};
use tokio_rustls::{server::TlsStream, LazyConfigAcceptor};

use crate::{
    adapters::CryptoCacheType, core::crypto::CryptoCache, model::Keycert, settings::Tls,
};

const SELECTED: &str = "selected";
const NOT_FOUND: &str = "not_found";
const NO_SERVER_NAME: &str = "no_server_name";
const INVALID: &str = "invalid";
const ERROR: &str = "error";
const TIMEOUT: &str = "timeout";
const DISABLED: &str = "disabled";

const PARSED_CAPACITY: u64 = 10_000;

lazy_static! {
    /// Every result except `selected` was served the fallback certificate.
    static ref CERTIFICATES: IntCounterVec = {
        let counter = register_int_counter_vec!(
            "click_router_tls_certificates_total",
            "TLS handshakes by certificate selection result",
            &["result"]
        )
        .expect("TLS metrics are registered once");
        for result in [SELECTED, NOT_FOUND, NO_SERVER_NAME, INVALID, ERROR, TIMEOUT, DISABLED] {
            counter.with_label_values(&[result]);
        }
        counter
    };
}

fn server_config(
    certs: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> Result<ServerConfig> {
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Ok(config)
}

fn parse_pem(cert: &[u8], key: &[u8]) -> Result<ServerConfig> {
    let certs = CertificateDer::pem_slice_iter(cert)
        .collect::<Result<Vec<_>, _>>()
        .context("invalid certificate PEM")?;
    if certs.is_empty() {
        bail!("no certificate in PEM");
    }
    let key = PrivateKeyDer::from_pem_slice(key).context("invalid private key PEM")?;

    server_config(certs, key)
}

/// The configured fallback, or a generated self-signed certificate.
fn fallback_config(settings: &Tls) -> Result<ServerConfig> {
    match (&settings.fallback_cert_path, &settings.fallback_key_path) {
        (Some(cert), Some(key)) => {
            let cert_pem =
                fs::read(cert).with_context(|| format!("cannot read TLS certificate {cert}"))?;
            let key_pem = fs::read(key).with_context(|| format!("cannot read TLS key {key}"))?;
            parse_pem(&cert_pem, &key_pem)
                .with_context(|| format!("cannot use TLS certificate {cert} with key {key}"))
        }
        (None, None) => {
            let generated = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;
            let key = PrivatePkcs8KeyDer::from(generated.signing_key.serialize_der());
            server_config(vec![generated.cert.der().clone()], key.into())
        }
        _ => bail!("fallback_cert_path and fallback_key_path must be set together"),
    }
}

/// A parsed certificate with the stored one it came from. `config` is `None`
/// when it could not be parsed.
#[derive(Clone)]
struct Parsed {
    source: Arc<Keycert>,
    config: Option<Arc<ServerConfig>>,
}

#[derive(Clone)]
pub struct CertificateSelector {
    cache: Option<CryptoCacheType>,
    parsed: Cache<String, Parsed>,
    fallback: Arc<ServerConfig>,
    lookup_timeout: Duration,
}

impl CertificateSelector {
    /// `cache` is ignored unless SNI selection is enabled.
    pub fn new(settings: &Tls, cache: Option<CryptoCacheType>) -> Result<Self> {
        lazy_static::initialize(&CERTIFICATES);

        Ok(Self {
            cache: cache.filter(|_| settings.sni_enabled),
            parsed: Cache::new(PARSED_CAPACITY),
            fallback: Arc::new(fallback_config(settings)?),
            lookup_timeout: Duration::from_millis(settings.lookup_timeout_millis),
        })
    }

    async fn select(&self, server_name: Option<&str>) -> Arc<ServerConfig> {
        let (result, config) = self.lookup(server_name).await;
        CERTIFICATES.with_label_values(&[result]).inc();

        config.unwrap_or_else(|| self.fallback.clone())
    }

    async fn lookup(&self, server_name: Option<&str>) -> (&'static str, Option<Arc<ServerConfig>>) {
        let Some(cache) = &self.cache else {
            return (DISABLED, None);
        };
        let Some(server_name) = server_name else {
            return (NO_SERVER_NAME, None);
        };
        let server_name = server_name.to_ascii_lowercase();

        match timeout(self.lookup_timeout, cache.get_certificate(&server_name)).await {
            Ok(Ok(Some(keycert))) => match self.parse(server_name, keycert) {
                Some(config) => (SELECTED, Some(config)),
                None => (INVALID, None),
            },
            Ok(Ok(None)) => (NOT_FOUND, None),
            Ok(Err(e)) => {
                tracing::error!(server_name, "{e:#}");
                (ERROR, None)
            }
            Err(_) => {
                tracing::warn!(server_name, "certificate lookup timed out");
                (TIMEOUT, None)
            }
        }
    }

    fn parse(&self, server_name: String, source: Arc<Keycert>) -> Option<Arc<ServerConfig>> {
        if let Some(parsed) = self.parsed.get(&server_name) {
            if Arc::ptr_eq(&parsed.source, &source) {
                return parsed.config;
            }
        }

        let config = match parse_pem(&source.cert, &source.key) {
            Ok(config) => Some(Arc::new(config)),
            Err(e) => {
                tracing::error!(server_name, "unusable stored certificate: {e:#}");
                None
            }
        };
        self.parsed.insert(
            server_name,
            Parsed {
                source,
                config: config.clone(),
            },
        );

        config
    }
}

/// Reads the ClientHello, waits for the matching certificate, then finishes
/// the handshake with it.
#[derive(Clone)]
pub struct SniAcceptor {
    selector: CertificateSelector,
    handshake_timeout: Duration,
}

impl SniAcceptor {
    pub fn new(settings: &Tls, cache: Option<CryptoCacheType>) -> Result<Self> {
        Ok(Self {
            selector: CertificateSelector::new(settings, cache)?,
            handshake_timeout: Duration::from_secs(settings.handshake_timeout_seconds),
        })
    }
}

impl<I, S> Accept<I, S> for SniAcceptor
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: Send + 'static,
{
    type Stream = TlsStream<I>;
    type Service = S;
    type Future = Pin<Box<dyn Future<Output = io::Result<(Self::Stream, S)>> + Send>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        let selector = self.selector.clone();
        let limit = self.handshake_timeout;

        Box::pin(async move {
            let handshake = async {
                let start = LazyConfigAcceptor::new(Acceptor::default(), stream).await?;
                let server_name = start.client_hello().server_name().map(str::to_owned);
                let config = selector.select(server_name.as_deref()).await;
                start.into_stream(config).await
            };
            let stream = timeout(limit, handshake)
                .await
                .map_err(|_| io::Error::from(io::ErrorKind::TimedOut))??;

            Ok((stream, service))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapters::{
            moka::{crypto_cache::MokaCryptoCache, settings::CryptoCacheSettings},
            CryptoStoreType,
        },
        core::CryptoStore,
    };
    use axum::{extract::ConnectInfo, routing::get, Router};
    use std::{
        collections::HashMap,
        net::{SocketAddr, TcpListener},
        path::PathBuf,
    };

    fn keycert(name: &str) -> (Keycert, String) {
        let generated = rcgen::generate_simple_self_signed(vec![name.to_string()]).unwrap();
        let pem = generated.cert.pem();
        (
            Keycert::new()
                .key(generated.signing_key.serialize_pem())
                .cert(pem.clone()),
            pem,
        )
    }

    #[derive(Default)]
    struct FakeStore {
        certs: std::sync::Mutex<HashMap<String, Keycert>>,
        delay: Duration,
        fail: bool,
    }

    #[async_trait::async_trait()]
    impl CryptoStore for FakeStore {
        async fn get_certificate(&self, name: &str) -> Result<Option<Keycert>> {
            tokio::time::sleep(self.delay).await;
            if self.fail {
                bail!("store down");
            }
            Ok(self.certs.lock().unwrap().get(name).cloned())
        }
    }

    fn cache(store: Arc<FakeStore>) -> CryptoCacheType {
        CryptoCacheType::Moka(MokaCryptoCache::new(
            CryptoStoreType::Test(store),
            CryptoCacheSettings {
                max_capacity: 10,
                time_to_live_minutes: 1,
                time_to_idle_minutes: 1,
                miss_max_capacity: 10,
                miss_time_to_live_seconds: 30,
                max_concurrent_lookups: 4,
            },
        ))
    }

    fn settings() -> Tls {
        Tls {
            sni_enabled: true,
            lookup_timeout_millis: 100,
            ..Default::default()
        }
    }

    fn serve(settings: &Tls, cache: CryptoCacheType) -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let addr = listener.local_addr().unwrap();
        let acceptor = SniAcceptor::new(settings, Some(cache)).unwrap();
        let app = Router::new()
            .route(
                "/",
                get(|ConnectInfo(addr): ConnectInfo<SocketAddr>| async move { addr.to_string() }),
            )
            .into_make_service_with_connect_info::<SocketAddr>();
        tokio::spawn(axum_server::from_tcp(listener).acceptor(acceptor).serve(app));
        addr
    }

    async fn connect(
        addr: SocketAddr,
        name: Option<&str>,
        alpn: &[u8],
    ) -> tokio_rustls::client::TlsStream<tokio::net::TcpStream> {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let mut config = rustls::ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .unwrap()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAny(provider)))
            .with_no_client_auth();
        config.enable_sni = name.is_some();
        config.alpn_protocols = vec![alpn.to_vec()];
        let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let server = name.unwrap_or("ignored.test").to_string().try_into().unwrap();
        tokio_rustls::TlsConnector::from(Arc::new(config))
            .connect(server, stream)
            .await
            .unwrap()
    }

    /// The leaf certificate presented for `name`, or for no name at all.
    async fn presented(addr: SocketAddr, name: Option<&str>) -> CertificateDer<'static> {
        let tls = connect(addr, name, b"http/1.1").await;
        tls.get_ref().1.peer_certificates().unwrap()[0].clone()
    }

    #[derive(Debug)]
    struct AcceptAny(Arc<rustls::crypto::CryptoProvider>);

    impl rustls::client::danger::ServerCertVerifier for AcceptAny {
        fn verify_server_cert(
            &self,
            _: &CertificateDer<'_>,
            _: &[CertificateDer<'_>],
            _: &rustls::pki_types::ServerName<'_>,
            _: &[u8],
            _: rustls::pki_types::UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }
        fn verify_tls12_signature(
            &self,
            _: &[u8],
            _: &CertificateDer<'_>,
            _: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }
        fn verify_tls13_signature(
            &self,
            _: &[u8],
            _: &CertificateDer<'_>,
            _: &rustls::DigitallySignedStruct,
        ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
            Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
        }
        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            self.0.signature_verification_algorithms.supported_schemes()
        }
    }

    fn der(pem: &str) -> CertificateDer<'static> {
        CertificateDer::from_pem_slice(pem.as_bytes()).unwrap()
    }

    fn store_with(entries: &[(&str, Keycert)]) -> Arc<FakeStore> {
        let store = FakeStore::default();
        for (name, keycert) in entries {
            store.certs.lock().unwrap().insert(name.to_string(), keycert.clone());
        }
        Arc::new(store)
    }

    #[tokio::test]
    async fn certificate_follows_server_name() {
        let (a, a_pem) = keycert("a.test");
        let (b, b_pem) = keycert("b.test");
        let addr = serve(&settings(), cache(store_with(&[("a.test", a), ("b.test", b)])));

        assert_eq!(presented(addr, Some("a.test")).await, der(&a_pem));
        assert_eq!(presented(addr, Some("b.test")).await, der(&b_pem));
        assert_eq!(presented(addr, Some("A.TEST")).await, der(&a_pem));
        assert!(CERTIFICATES.with_label_values(&[SELECTED]).get() >= 3);
    }

    #[tokio::test]
    async fn http1_and_http2_see_the_client_address() {
        use http_body_util::{BodyExt, Empty};
        use hyper_util::rt::{TokioExecutor, TokioIo};

        let (a, _) = keycert("a.test");
        let addr = serve(&settings(), cache(store_with(&[("a.test", a)])));
        let request = || {
            http::Request::get("https://a.test/")
                .header("host", "a.test")
                .body(Empty::<axum::body::Bytes>::new())
                .unwrap()
        };

        let tls = connect(addr, Some("a.test"), b"http/1.1").await;
        let (mut client, connection) =
            hyper::client::conn::http1::handshake(TokioIo::new(tls)).await.unwrap();
        tokio::spawn(connection);
        let http1 = client.send_request(request()).await.unwrap();

        let tls = connect(addr, Some("a.test"), b"h2").await;
        assert_eq!(tls.get_ref().1.alpn_protocol(), Some(&b"h2"[..]));
        let (mut client, connection) =
            hyper::client::conn::http2::handshake(TokioExecutor::new(), TokioIo::new(tls))
                .await
                .unwrap();
        tokio::spawn(connection);
        let http2 = client.send_request(request()).await.unwrap();

        assert_eq!(http1.version(), http::Version::HTTP_11);
        assert_eq!(http2.version(), http::Version::HTTP_2);
        for response in [http1, http2] {
            let body = response.into_body().collect().await.unwrap().to_bytes();
            assert!(body.starts_with(b"127.0.0.1:"));
        }
    }

    #[tokio::test]
    async fn fallback_for_unknown_missing_malformed_failing_and_slow() {
        let (known, known_pem) = keycert("known.test");
        let bad = Keycert::new().key("nope").cert("nope");
        let addr = serve(
            &settings(),
            cache(store_with(&[("known.test", known), ("bad.test", bad)])),
        );
        let fallback = presented(addr, Some("unknown.test")).await;
        assert_ne!(fallback, der(&known_pem));
        assert_eq!(presented(addr, None).await, fallback);
        assert_eq!(presented(addr, Some("bad.test")).await, fallback);
        assert_eq!(presented(addr, Some("known.test")).await, der(&known_pem));

        for store in [
            FakeStore { fail: true, ..Default::default() },
            FakeStore { delay: Duration::from_secs(30), ..Default::default() },
        ] {
            let addr = serve(&settings(), cache(Arc::new(store)));
            let first = presented(addr, Some("x.test")).await;
            assert_eq!(presented(addr, None).await, first);
        }
    }

    #[tokio::test]
    async fn replacement_is_served_after_invalidation() {
        let (old, old_pem) = keycert("a.test");
        let (new, new_pem) = keycert("a.test");
        let store = store_with(&[("a.test", old)]);
        let cache = cache(store.clone());
        let addr = serve(&settings(), cache.clone());
        assert_eq!(presented(addr, Some("a.test")).await, der(&old_pem));

        store.certs.lock().unwrap().insert("a.test".into(), new);
        cache.invalidate("a.test").await.unwrap();
        assert_eq!(presented(addr, Some("a.test")).await, der(&new_pem));

        store.certs.lock().unwrap().clear();
        cache.invalidate("a.test").await.unwrap();
        assert_ne!(presented(addr, Some("a.test")).await, der(&new_pem));
    }

    #[tokio::test]
    async fn every_result_is_exposed_at_metrics() {
        use axum::response::IntoResponse;
        use http_body_util::BodyExt;

        CertificateSelector::new(&settings(), None).unwrap();
        let response = crate::core::metrics_endpoint::metrics_handler().await.into_response();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();
        for result in [SELECTED, NOT_FOUND, NO_SERVER_NAME, INVALID, ERROR, TIMEOUT, DISABLED] {
            assert!(body.contains(&format!("click_router_tls_certificates_total{{result=\"{result}\"}}")));
        }
    }

    #[tokio::test]
    async fn disabled_selection_always_uses_the_fallback() {
        let (a, pem) = keycert("a.test");
        let settings = Tls { sni_enabled: false, ..settings() };
        let addr = serve(&settings, cache(store_with(&[("a.test", a)])));

        assert_ne!(presented(addr, Some("a.test")).await, der(&pem));
    }

    #[tokio::test]
    async fn configured_fallback_is_used_and_bad_paths_fail() {
        let (fallback, pem) = keycert("fallback.test");
        let dir: PathBuf = std::env::temp_dir().join(format!("shortas-tls-{}", ulid::Ulid::new()));
        fs::create_dir(&dir).unwrap();
        let path = |name: &str| dir.join(name).to_str().unwrap().to_string();
        fs::write(path("cert.pem"), &fallback.cert).unwrap();
        fs::write(path("key.pem"), &fallback.key).unwrap();

        let configured = Tls {
            fallback_cert_path: Some(path("cert.pem")),
            fallback_key_path: Some(path("key.pem")),
            ..settings()
        };
        let addr = serve(&configured, cache(store_with(&[])));
        assert_eq!(presented(addr, Some("unknown.test")).await, der(&pem));

        let missing = Tls { fallback_key_path: Some(path("missing.pem")), ..configured.clone() };
        let err = fallback_config(&missing).unwrap_err();
        assert!(format!("{err:#}").contains("missing.pem"));

        fs::write(path("cert.pem"), "garbage").unwrap();
        let err = fallback_config(&configured).unwrap_err();
        assert!(format!("{err:#}").contains("cert.pem"));

        assert!(fallback_config(&Tls { fallback_key_path: None, ..configured }).is_err());
        fs::remove_dir_all(dir).unwrap();
    }

    /// The certificate `openssl s_client` receives for `name`.
    async fn s_client(addr: SocketAddr, name: &str) -> CertificateDer<'static> {
        let output = tokio::process::Command::new("openssl")
            .args(["s_client", "-connect", &addr.to_string(), "-servername", name])
            .stdin(std::process::Stdio::null())
            .output()
            .await
            .unwrap();
        CertificateDer::from_pem_slice(&output.stdout).unwrap()
    }

    async fn served_within_60s(addr: SocketAddr, name: &str, expected: &CertificateDer<'_>) {
        let served = async {
            while s_client(addr, name).await != *expected {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        };
        timeout(Duration::from_secs(60), served).await.expect("not served in time");
    }

    /// Issue, serve, renew and delete through a running click-router-api.
    /// Needs TEST_API_URL, TEST_MONGODB_URI and TEST_RABBITMQ_URI.
    #[tokio::test]
    #[ignore = "needs click-router-api, MongoDB and RabbitMQ"]
    async fn issued_renewed_and_deleted_certificates_apply_without_restart() {
        use crate::adapters::{
            moka::{
                routes_cache::MokaRoutesCache,
                settings::{RoutesCacheSettings, UserSettingsCacheSettings},
                user_settings_cache::MokaUserSettingsCache,
            },
            mongodb::{crypto_store::MongodbCryptoStore, settings::Mongodb},
            rabbitmq::{consumer::start_cache_invalidation_consumer, settings::RabbitMqSettings},
            RoutesCacheType, RoutesStoreType, UserSettingsCacheType, UserSettingsStoreType,
        };

        let env = |name: &str| std::env::var(name).unwrap();
        let mongodb = Mongodb {
            uri: env("TEST_MONGODB_URI"),
            database: "shortas".into(),
            encryption_collection: "core_routes_encryption_main".into(),
            ..Default::default()
        };
        // A long lifetime, so only a notification can explain a quick change.
        let cache = CryptoCacheType::Moka(MokaCryptoCache::new(
            CryptoStoreType::Mongodb(MongodbCryptoStore::new(&mongodb).await),
            CryptoCacheSettings {
                max_capacity: 10,
                time_to_live_minutes: 60,
                time_to_idle_minutes: 60,
                miss_max_capacity: 10,
                miss_time_to_live_seconds: 3600,
                max_concurrent_lookups: 4,
            },
        ));
        start_cache_invalidation_consumer(
            RabbitMqSettings {
                uri: env("TEST_RABBITMQ_URI"),
                route_exchange: "cache.invalidation.routes".into(),
                user_settings_exchange: "cache.invalidation.user_settings".into(),
                certificate_exchange: "cache.invalidation.certificates".into(),
                reconnect_seconds: 1,
            },
            RoutesCacheType::Moka(MokaRoutesCache::new(
                RoutesStoreType::InMemory(Default::default()),
                RoutesCacheSettings { max_capacity: 1, time_to_live_minutes: 1, time_to_idle_minutes: 1 },
            )),
            UserSettingsCacheType::Moka(MokaUserSettingsCache::new(
                UserSettingsStoreType::InMemory(Default::default()),
                UserSettingsCacheSettings { max_capacity: 1, time_to_live_minutes: 1, time_to_idle_minutes: 1 },
            )),
            Some(cache.clone()),
        );
        tokio::time::sleep(Duration::from_secs(3)).await;

        let name = "e2e.test";
        let url = format!("{}/v1/certificates/{name}", env("TEST_API_URL"));
        let client = reqwest::Client::new();
        let addr = serve(&Tls { lookup_timeout_millis: 3000, ..settings() }, cache);
        let fallback = s_client(addr, name).await;

        for _ in ["issue", "renew"] {
            let (keycert, pem) = keycert(name);
            client
                .post(&url)
                .json(&serde_json::json!({
                    "key": String::from_utf8(keycert.key).unwrap(),
                    "cert": pem,
                    "ocspResp": "",
                    "ocsp_resp": "",
                }))
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap();
            served_within_60s(addr, name, &der(&pem)).await;
        }

        client.delete(&url).send().await.unwrap().error_for_status().unwrap();
        served_within_60s(addr, name, &fallback).await;
    }
}
