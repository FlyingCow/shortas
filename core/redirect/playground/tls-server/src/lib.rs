use std::{
    boxed::Box,
    convert::Infallible,
    error::Error as StdError,
    fs::File,
    future::Future,
    io::{Error as IoError, ErrorKind, Read, Result as IoResult},
    net::SocketAddr,
    num::NonZeroUsize,
    path::Path,
    sync::Arc
};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex
};
use tokio_rustls::LazyConfigAcceptor;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::{ ClientHello, ServerConfig }
};

use tracing::{error, info, warn};
use futures_util::future::join_all;

use hyper_util::rt::TokioIo;

/// Private key and certificate
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Keycert {
    /// Private key.
    pub key: Vec<u8>,
    /// Certificate.
    pub cert: Vec<u8>,
    /// OCSP response.
    pub ocsp_resp: Vec<u8>,
}

impl Keycert {
    /// Create a new keycert.
    #[inline]
    pub fn new() -> Self {
        Self {
            key: vec![],
            cert: vec![],
            ocsp_resp: vec![],
        }
    }
    /// Sets the Tls private key via File Path, returns [`IoError`] if the file cannot be open.
    #[inline]
    pub fn key_from_path(mut self, path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = File::open(path.as_ref())?;
        file.read_to_end(&mut self.key)?;
        Ok(self)
    }

    /// Sets the Tls private key via bytes slice.
    #[inline]
    pub fn key(mut self, key: impl Into<Vec<u8>>) -> Self {
        self.key = key.into();
        self
    }

    /// Specify the file path for the TLS certificate to use.
    #[inline]
    pub fn cert_from_path(mut self, path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = File::open(path)?;
        file.read_to_end(&mut self.cert)?;
        Ok(self)
    }

    /// Sets the Tls certificate via bytes slice
    #[inline]
    pub fn cert(mut self, cert: impl Into<Vec<u8>>) -> Self {
        self.cert = cert.into();
        self
    }

    /// Get ocsp_resp.
    #[inline]
    pub fn ocsp_resp(&self) -> &[u8] {
        &self.ocsp_resp
    }
}

//---------------Crypto Manager-----------------------

pub trait BaseCryptoStore: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = IoResult<Option<Keycert>>> + Send;
}

#[derive(Clone, Debug)]
pub struct DynamoCryptoStore {}

impl DynamoCryptoStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl BaseCryptoStore for DynamoCryptoStore {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Option<Keycert>> {
        Ok(Some(
            Keycert::new()
                .cert_from_path("./certs/cert.pem")
                .unwrap()
                .key_from_path("/certs/key.pem")
                .unwrap(),
        ))
    }
}

pub trait BaseCryptoCache: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = IoResult<Option<Keycert>>> + Send;
}

#[derive(Clone, Debug)]
pub struct InMemoryCryptoCache {}

impl InMemoryCryptoCache {
    pub fn new() -> Self {
        Self {}
    }
}

impl BaseCryptoCache for InMemoryCryptoCache {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Option<Keycert>> {
        Ok(Some(
            Keycert::new()
                .cert_from_path("./certs/cert.pem")
                .unwrap()
                .key_from_path("/certs/key.pem")
                .unwrap(),
        ))
    }
}

//---------------Crypto Manager-----------------------

pub trait BaseCryptoManager: Send + Sync + Clone {
    fn get_default_certificate(
        &self,
    ) -> impl std::future::Future<Output = IoResult<Keycert>> + Send;
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = IoResult<Option<Keycert>>> + Send;
}

#[derive(Copy, Clone, Debug)]
pub struct CryptoManager<S, C>
where
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{
    crypto_store: S,
    crypto_cache: C,
}

impl<S, C> BaseCryptoManager for CryptoManager<S, C>
where
    S: BaseCryptoStore,
    C: BaseCryptoCache,
{
    async fn get_default_certificate(&self) -> IoResult<Keycert> {
        Ok(Keycert::new()
            .cert_from_path("./certs/cert.pem")
            .unwrap()
            .key_from_path("/certs/key.pem")
            .unwrap())
    }

    async fn get_certificate(&self, server_name: &str) -> IoResult<Option<Keycert>> {
        let keycert = self.crypto_cache.get_certificate(server_name).await;

        if let Ok(keycert) = keycert {
            return Ok(keycert);
        }

        let keycert = self.crypto_store.get_certificate(server_name).await?;
        //self.crypto_cache.set_certificate(server_name, keycert.clone()).await?;
        Ok(keycert)
    }
}

impl<S, C> CryptoManager<S, C>
where
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{
    pub fn new(crypto_store: S, crypto_cache: C) -> Self {
        Self {
            crypto_store,
            crypto_cache,
        }
    }
}

pub trait BaseConnectionHandler: Send + Sync + Clone {
    fn handle(
        &self, 
        stream: TcpStream
    ) -> impl Future<Output = ()> + Send;
}

#[derive(Clone, Debug)]
pub struct ConnectionHandler {
    http: hyper::server::conn::http1::Builder
}

impl BaseConnectionHandler for ConnectionHandler {
    async fn handle(&self, stream: TcpStream) {
        let http = &self.http.clone();
        let io = TokioIo::new(stream);

        if let Err(err) = http.serve_connection(io, service_fn(hello)).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}

impl ConnectionHandler {
    pub fn new() -> Self {
        Self {
            http: http1::Builder::new()
        }
    }
}

pub trait BaseTlsConnectionHandler: Send + Sync + Clone {
    fn handle(
        &self, 
        stream: TcpStream
    ) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(Clone, Debug)]
pub struct TlsConnectionHandler<C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
{
    http: hyper::server::conn::http1::Builder,
    crypto_manager: C,
}

impl<C> BaseTlsConnectionHandler for TlsConnectionHandler<C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
{
    async fn handle(
        &self, 
        stream: TcpStream
    ) {
        let http = &self.http.clone();
        let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), stream);
        tokio::pin!(acceptor);

        match acceptor.as_mut().await {
            Ok(start) => {
                let client_hello = start.client_hello();
                let config = self.get_tls_config(client_hello).await.unwrap();
                let stream = start.into_stream(Arc::new(config)).await.unwrap();
                let io = TokioIo::new(stream);

                if let Err(err) = http.serve_connection(io, service_fn(hello)).await {
                    println!("Error serving connection: {:?}", err);
                }
            }
            Err(err) => {}
        }
    }
}

impl<C> TlsConnectionHandler<C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
{
    pub fn new(crypto_manager: C) -> Self {
        Self {
            crypto_manager: crypto_manager,
            http: http1::Builder::new()
        }
    }

    async fn get_certificate(
        &self,
        client_hello: ClientHello<'_>,
    ) -> Result<Keycert, Box<dyn StdError>> {
        let server_name = client_hello.server_name();

        //no SNI supplied case
        if let None = server_name {
            warn!("No server name identifier supplied");

            let default_cert = self.crypto_manager.get_default_certificate().await.unwrap();

            return Ok(default_cert);
        }

        //error extracting certificate by domain name case
        let cert_result = self
            .crypto_manager
            .get_certificate(client_hello.server_name().unwrap())
            .await;

        if let Err(err) = cert_result {
            error!(
                "Error occured extracting certificate for domain: {}, err: {}",
                client_hello.server_name().unwrap(),
                err
            );

            let default_cert = self.crypto_manager.get_default_certificate().await.unwrap();

            return Ok(default_cert);
        }

        let certificate = cert_result.unwrap();

        if let Some(cert) = certificate {
            return Ok(cert);
        } else {
            //no certificate found for a specified domain case
            warn!(
                "No certificate found for {}",
                client_hello.server_name().unwrap()
            );

            let default_cert = self.crypto_manager.get_default_certificate().await.unwrap();

            return Ok(default_cert);
        }
    }

    fn build_config(&self, keycert: Keycert) -> Result<ServerConfig, Box<dyn StdError>> {
        let certs: Vec<CertificateDer> = rustls_pemfile::certs(&mut &*keycert.cert)
            .map(|c| c.unwrap())
            .collect();

        if certs.len() < 1 {
            return Err(Box::new(IoError::new(
                ErrorKind::NotFound,
                "No certificates found.",
            )));
        }

        let mut keys: Vec<PrivateKeyDer> = rustls_pemfile::pkcs8_private_keys(&mut &*keycert.key)
            .map(|c| PrivateKeyDer::Pkcs8(c.unwrap()))
            .collect();

        if keys.len() < 1 {
            return Err(Box::new(IoError::new(
                ErrorKind::NotFound,
                "No private keys found.",
            )));
        }

        let mut tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, keys.remove(0))
            .expect("No certificate found");

        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        Ok(tls_config)
    }

    async fn get_tls_config(
        &self,
        client_hello: ClientHello<'_>,
    ) -> Result<ServerConfig, Box<dyn StdError>> {
        let keycert = self.get_certificate(client_hello).await.unwrap();

        let config = self.build_config(keycert);

        config
    }
}

#[derive(Clone, Copy)]
pub enum Socket {
    Unsecured(SocketAddr),
    Secured(SocketAddr),
}
//---------------Server-----------------------
pub struct Server<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    handler: C,
    tls_handler: T,
    threads: usize,
    listen_os_signals: bool,
    exit: bool,
    sockets: Vec<Socket>,
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

impl<C, T> Server<C, T>
where
    C: BaseConnectionHandler + Send + Sync + Clone,
    T: BaseTlsConnectionHandler + Send + Sync + Clone,
{
    fn new_from_builder(builder: ServerBuilder<C, T>) -> Self {
        Self {
            handler: builder.handler,
            tls_handler: builder.tls_handler,
            threads: builder.threads,
            listen_os_signals: builder.listen_os_signals,
            exit: builder.exit,
            sockets: builder.sockets,
        }
    }

    fn get_unsecure_handler(&self) -> Arc<Mutex<C>> {
        Arc::new(Mutex::new(self.handler.clone()))
    }

    fn get_secure_handler(&self) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(self.tls_handler.clone()))
    }

    async fn listen_unsecured(&self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        let unsecure_handler = self.get_unsecure_handler();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let unsecure_handler = unsecure_handler.clone();

            tokio::spawn(async move {
                let handler = unsecure_handler.lock().await;
                handler.handle(stream).await;
            });
        }
    }

    async fn listen_secured(&self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        let secure_handler = self.get_secure_handler();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let secure_handler = secure_handler.clone();

            tokio::spawn(async move {
                let handler = secure_handler.lock().await;
                handler.handle(stream).await
            });
        }
    }
    async fn listen(&self, socket: Socket){
        match socket {
            Socket::Secured(addr) => self.listen_secured(addr).await,
            Socket::Unsecured(addr) => self.listen_unsecured(addr).await,
        }
    }

    pub async fn run(&self) {

        let futures = self.sockets.iter().map(|s| self.listen(s.clone()));

        join_all(futures).await;
    }

}

pub struct ServerBuilder<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    threads: usize,
    listen_os_signals: bool,
    exit: bool,
    sockets: Vec<Socket>,
    handler: C,
    tls_handler: T,
}

impl<C, T> ServerBuilder<C, T>
where
    C: BaseConnectionHandler + Send + Clone + 'static,
    T: BaseTlsConnectionHandler + Send + Clone + 'static,
{
    pub fn new(handler: C, tls_handler: T) -> Self {
        ServerBuilder {
            threads: std::thread::available_parallelism().map_or(2, NonZeroUsize::get),
            listen_os_signals: true,
            exit: false,
            sockets: Vec::new(),
            handler: handler,
            tls_handler: tls_handler,
        }
    }

    pub fn bind(mut self, address: SocketAddr) -> Self {
        self.sockets.push(Socket::Unsecured(address));
        self
    }

    pub fn bind_tls(mut self, address: SocketAddr) -> Self {
        self.sockets.push(Socket::Secured(address));
        self
    }

    pub fn workers(mut self, num: usize) -> Self {
        assert_ne!(num, 0, "workers must be greater than 0");
        self.threads = num;
        self
    }

    /// Disable OS signal handling.
    pub fn disable_signals(mut self) -> Self {
        self.listen_os_signals = false;
        self
    }

    pub fn system_exit(mut self) -> Self {
        self.exit = true;
        self
    }

    pub fn build(self) -> Server<C, T> {
        if self.sockets.is_empty() {
            panic!("Server should have at least one bound socket");
        }
        info!("starting {} workers", self.threads);
        Server::new_from_builder(self)
    }
}
