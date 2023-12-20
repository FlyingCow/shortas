use std::error::Error;

use salvo::conn::rustls_selectable::{Keycert, RustlsConfig, CertificateResolver};
use salvo::prelude::*;
use crate::async_trait;

#[handler]
async fn hello(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}

struct Resolver();

#[async_trait]
impl CertificateResolver for Resolver {
    async fn get_certificate(&self, server_name: String) -> Result<Keycert, String>{
        println!(server_name);

        Ok(Keycert::new()
            .cert(include_bytes!("../certs/cert.pem").as_ref())
            .key(include_bytes!("../certs/key.pem").as_ref()))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let config = RustlsConfig::new(
        Keycert::new()
            .cert(include_bytes!("../certs/cert.pem").as_ref())
            .key(include_bytes!("../certs/key.pem").as_ref()),
    );
    let acceptor = TcpListener::new("0.0.0.0:5800").rustls_selectable(config).bind().await;
    Server::new(acceptor).serve(router).await;
}