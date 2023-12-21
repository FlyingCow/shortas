use crate::Server;

use anyhow::{anyhow, Error, Result};
use bytes::{Buf, Bytes};
use futures_util;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::Http;
use hyper::service::{Service, make_service_fn, service_fn};
use hyper::{body::HttpBody, header, Body, Method, Request, Response, StatusCode};
use rustls::{
    server::{Acceptor, ClientHello},
    Certificate, PrivateKey, ServerConfig, ServerConnection,
    SupportedCipherSuite, ProtocolVersion
};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::{net::TcpListener, task};
use tokio_rustls::LazyConfigAcceptor;
use tracing::{/*debug, error, */ info, warn};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static INDEX: &[u8] = b"INDEX";
static NOTFOUND: &[u8] = b"Not Found";

impl<'a> Server<'a> {
    async fn get_tls_config(
        &self,
        client_hello: ClientHello<'_>,
    ) -> Result<Arc<ServerConfig>, Error> {
        let cert = self
            .crypto_manager
            .get_certificate(client_hello.server_name().unwrap())
            .await?;

        let certs: Vec<Certificate> = rustls_pemfile::certs(&mut &*cert.cert)
            .map(|mut certs| certs.drain(..).map(Certificate).collect())?;
        if certs.len() < 1 {
            return Err(anyhow!("No certificates found."));
        }

        let mut keys: Vec<PrivateKey> = rustls_pemfile::pkcs8_private_keys(&mut &*cert.key)
            .map(|mut keys| keys.drain(..).map(PrivateKey).collect())?;
        if keys.len() < 1 {
            return Err(anyhow!("No private keys found."));
        }

        let mut tls_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, keys.remove(0))?;

        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        Ok(Arc::new(tls_config))
    }

    pub async fn run_tls(&self, addr: SocketAddr) -> Result<()> {
        info!("Running tls...");

            let listener = TcpListener::bind(addr).await?;
            let http = Http::new();

            loop {
                let (conn, remote_addr) = listener.accept().await?;
                let acceptor = LazyConfigAcceptor::new(Acceptor::default(), conn);
                let local_addr = listener.local_addr()?;

                tokio::spawn(async move {
                    futures_util::pin_mut!(acceptor);

                    match acceptor.as_mut().await {
                        Ok(start) => {
                            let client_hello = start.client_hello();

                            
                            let config = self.get_tls_config(client_hello).await.unwrap();

                            match start.into_stream(config).await {
                                Ok(stream) => {
                                    let (_io, tls_connection) = stream.get_ref();

                                    let handler = PerConnHandler {
                                        local_addr: local_addr,
                                        remote_addr: remote_addr,
                                        server_name: String::from(""), //clientHello.server_name().unwrap_or_default().to_string(),
                                        tls_info: Some(TlsInfo::from_tls_connection(
                                            tls_connection,
                                        )),
                                    };

                                    if let Err(e) = http.serve_connection(stream, handler).await {
                                        eprintln!("HYPER: {}", e);
                                    }
                                }
                                Err(e) => eprintln!("TLS: {}", e),
                            }
                        }

                        Err(e) => warn!("TLS {}", e),
                    }
                });
            };

        Ok(())
    }
}

#[derive(Clone)]
pub struct PerConnHandler {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub server_name: String,
    pub tls_info: Option<TlsInfo>,
}

impl Service<Request<Body>> for PerConnHandler
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>>
    {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future
    {
        let data = PerRequestData {
            local_addr: self.local_addr,
            remote_addr: self.remote_addr,
            tls_info: self.tls_info.clone(),
            request: req,
            response: Response::new("".into()),
        };
        Box::pin(handle(data))
    }
}

pub struct PerRequestData {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub tls_info: Option<TlsInfo>,
    pub request: Request<Body>,
    pub response: Response<Body>,
}

pub async fn handle(mut data: PerRequestData) -> std::result::Result<Response<Body>, Infallible> {
    // Handle requests here

    // elided: drop requests if 'accept' is not acceptable
    // elided: handle OPTIONS requests
    // elided: read session cookie and establish session
    // elided: route to page

    *data.response.status_mut() = StatusCode::OK;
    *data.response.body_mut() = format!(
        "<!DOCTYPE html><html><body>Remote Addr: {}<br>Cipher suite: {:?}</body></html>",
        data.remote_addr,
        data.tls_info.as_ref().unwrap().ciphersuite
    )
    .into();

    // elided: write access log
    return Ok(data.response);
}

#[derive(Clone)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub ciphersuite: Option<SupportedCipherSuite>,
    pub version: Option<ProtocolVersion>,
}

impl TlsInfo {
    pub fn from_tls_connection(conn: &ServerConnection) -> TlsInfo {
        TlsInfo {
            sni_hostname: conn.sni_hostname().map(|s| s.to_owned()),
            alpn_protocol: conn
                .alpn_protocol()
                .map(|s| String::from_utf8_lossy(s).into_owned()),
            ciphersuite: conn.negotiated_cipher_suite(),
            version: conn.protocol_version(),
        }
    }
}
