
use std::{error::Error as StdError, io, time::Duration};

use http::{Request, Response};
use hyper::body::{Body, Incoming};
use hyper::service::Service;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;

use server::conn::auto::Builder;
use tokio::net::TcpStream;
use tower::limit::RateLimitLayer;
use tower::make::Shared;
use tower::ServiceBuilder;

use crate::core::BaseFlowRouter;

use super::BaseConnectionHandler;

#[derive(Clone, Debug)]
pub struct ConnectionHandler<S, B>
where
    S: Service<Request<Incoming>, Response = Response<B>>,
    S::Future: 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: Body + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    http: Builder<TokioExecutor>,
    service: S,
}

impl<S, B> BaseConnectionHandler for ConnectionHandler<S, B>
where
    S: Service<Request<Incoming>, Response = Response<B>>,
    S::Future: 'static + Send,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: Body + 'static + Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    async fn handle(&self, stream: TcpStream) {
        let http = &self.http.clone();
        let io = TokioIo::new(stream);

        if let Err(err) = http.serve_connection_with_upgrades(io, self.service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}

impl<F> ConnectionHandler<F>
where
    F: BaseFlowRouter + Send + Sync + Clone,
{
    pub fn new(flow_router: F) -> Self {
        let svc = ServiceBuilder::new().layer(RateLimitLayer::new(1, Duration::from_secs(1)));

        let shared = Shared::new(svc);

        Self {
            http: Builder::new(TokioExecutor::new()),
            flow_router_service: FlowRouterService::new(flow_router),
        }
    }
}
