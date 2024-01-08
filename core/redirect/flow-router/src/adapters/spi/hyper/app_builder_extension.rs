use crate::adapters::spi::hyper::connection_handler::ConnectionHandler;
use crate::adapters::spi::hyper::tls_connection_handler::TlsConnectionHandler;
use  crate::core::BaseCryptoManager;

pub struct HyperBuilder<M>
where 
    M: BaseCryptoManager 
{
    pub unsecure_handler: ConnectionHandler,
    pub secure_handler: TlsConnectionHandler<M>,
}

impl<M> HyperBuilder<M>
where 
    M: BaseCryptoManager  
{
    pub async fn new(crypto_manager: M) -> Self {
        Self {
            unsecure_handler: ConnectionHandler::new(),
            secure_handler: TlsConnectionHandler::new(crypto_manager)
        }
    }
}
