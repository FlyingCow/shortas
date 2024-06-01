// pub mod flow_router_service;
// pub mod handler;

use std::future::Future;

use tokio::net::TcpStream;

pub trait BaseConnectionHandler {
    fn handle(&self, stream: TcpStream) -> impl Future<Output = ()> + Send;
}
