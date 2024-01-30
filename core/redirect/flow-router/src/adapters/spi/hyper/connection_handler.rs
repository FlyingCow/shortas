use std::convert::Infallible;

use http_body_util::Full;

use hyper::body::Bytes;
use hyper::{Request, Response};

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;

use tokio::net::TcpStream;

use crate::adapters::spi::hyper::flow_router_service::FlowRouterService;
use crate::core::{BaseConnectionHandler, BaseFlowRouter};

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[derive(Clone, Debug)]
pub struct ConnectionHandler<F>
where
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    http: server::conn::auto::Builder<TokioExecutor>,
    flow_router_service: FlowRouterService<F>,
}

impl<F> BaseConnectionHandler for ConnectionHandler<F>
where
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    async fn handle(&self, stream: TcpStream) {
        let http = &self.http.clone();
        let io = TokioIo::new(stream);
        let router = self.flow_router_service.clone();

        if let Err(err) = http.serve_connection_with_upgrades(io, router).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}

impl<F> ConnectionHandler<F>
where
    F: BaseFlowRouter<hyper::body::Incoming> + Send + Sync + Clone,
{
    pub fn new(flow_router: F) -> Self {
        Self {
            http: server::conn::auto::Builder::new(TokioExecutor::new()),
            flow_router_service: FlowRouterService::new(flow_router),
        }
    }
}
