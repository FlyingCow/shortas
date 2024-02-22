use std::error::Error as StdError;
use std::io::{Error as IoError, ErrorKind};
use std::sync::Arc;

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;

use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::{ClientHello, ServerConfig},
};
use tokio::net::TcpStream;
use tokio_rustls::LazyConfigAcceptor;
use tracing::{error, warn};

use crate::adapters::spi::hyper::flow_router_service::FlowRouterService;
use crate::core::BaseCryptoManager;
use crate::core::{BaseFlowRouter, BaseTlsConnectionHandler};
use crate::domain::Keycert;

#[derive(Clone, Debug)]
pub struct TlsConnectionHandler<F, C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone + 'static,
{
    http: server::conn::auto::Builder<TokioExecutor>,
    crypto_manager: C,
    flow_router_service: FlowRouterService<F>,
}

impl<F, C> BaseTlsConnectionHandler for TlsConnectionHandler<F, C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone + 'static,
{
    async fn handle(&self, stream: TcpStream) {
        let http = &self.http.clone();
        let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), stream);
        tokio::pin!(acceptor);

        match acceptor.as_mut().await {
            Ok(start) => {
                let client_hello = start.client_hello();
                let config = self.get_tls_config(client_hello).await.unwrap();
                let stream = start.into_stream(Arc::new(config)).await.unwrap();
                let io = TokioIo::new(stream);
                let router = self.flow_router_service.clone();

                if let Err(err) = http.serve_connection_with_upgrades(io, router).await {
                    println!("Error serving connection: {:?}", err);
                }
            }
            Err(_err) => {}
        }
    }
}

impl<F, C> TlsConnectionHandler<F, C>
where
    C: BaseCryptoManager + Send + Sync + Clone,
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    pub fn new(flow_router: F, crypto_manager: C) -> Self {
        Self {
            crypto_manager: crypto_manager,
            http: server::conn::auto::Builder::new(TokioExecutor::new()),
            flow_router_service: FlowRouterService::new(flow_router),
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

            return Ok(default_cert.unwrap());
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

            return Ok(default_cert.unwrap());
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

            return Ok(default_cert.unwrap());
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

        tls_config.alpn_protocols = vec![/*b"h2".to_vec(), */ b"http/1.1".to_vec()];

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
