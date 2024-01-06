use std::future::Future;

use tokio::net::TcpStream;

pub trait BaseTlsConnectionHandler: Send + Sync + Clone {
    fn handle(
        &self, 
        stream: TcpStream
    ) -> impl Future<Output = ()> + Send;
}