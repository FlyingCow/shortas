use rustls::server::ClientHello;
use salvo::conn::rustls_async::{Keycert, ResolvesServerConfig, RustlsConfig};
use salvo::writing::Text;
use salvo::{async_trait, handler, Listener, Response};
use salvo::{conn::TcpListener, Router, Server};
use std::io::{Error as IoError, Result as IoResult};
use std::sync::Arc;

#[handler]
fn hello(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

struct ServerConfigResolverMock;

#[async_trait]
impl ResolvesServerConfig<IoError> for ServerConfigResolverMock {
    async fn resolve(&self, client_hello: ClientHello<'_>) -> IoResult<Arc<RustlsConfig>> {
        // println!("host:{}", client_hello.server_name().unwrap());

        let config = RustlsConfig::new(
            Keycert::new()
                .cert(include_bytes!("../certs/cert.pem").as_ref())
                .key(include_bytes!("../certs/key.pem").as_ref()),
        );

        Ok(Arc::new(config))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let router = Router::new().get(hello);

    let config = ServerConfigResolverMock {};

    let acceptor = TcpListener::new("0.0.0.0:5800").rustls_async(config).bind().await;
    Server::new(acceptor).serve(router).await;
}
