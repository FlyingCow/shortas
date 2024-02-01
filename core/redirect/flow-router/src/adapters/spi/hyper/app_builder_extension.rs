use crate::adapters::spi::hyper::connection_handler::ConnectionHandler;
use crate::adapters::spi::hyper::tls_connection_handler::TlsConnectionHandler;
use crate::core::{BaseCryptoManager, BaseFlowRouter};

pub struct HyperBuilder<M, F>
where
    M: BaseCryptoManager,
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    pub unsecure_handler: ConnectionHandler<F>,
    pub secure_handler: TlsConnectionHandler<F, M>,
}

impl<M, F> HyperBuilder<M, F>
where
    M: BaseCryptoManager,
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    pub async fn new(crypto_manager: M, flow_router: F) -> Self {
        Self {
            unsecure_handler: ConnectionHandler::new(flow_router.clone()),
            secure_handler: TlsConnectionHandler::new(flow_router.clone(), crypto_manager),
        }
    }
}
