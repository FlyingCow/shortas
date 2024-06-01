
/// Fake certs generated using
/// openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 3650 -nodes
pub struct SSLHyperServer<T: 'static + Context + Clone + Send + Sync, S: Send> {
    app: App<HyperRequest, T, S>,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    upgrade: bool,
}