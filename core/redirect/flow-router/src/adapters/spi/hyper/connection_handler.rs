use std::convert::Infallible;

use http_body_util::Full;

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server;

use tokio::net::TcpStream;

use crate::core::BaseConnectionHandler;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[derive(Clone, Debug)]
pub struct ConnectionHandler {
    http: server::conn::auto::Builder<TokioExecutor>,
}

impl BaseConnectionHandler for ConnectionHandler {
    async fn handle(&self, stream: TcpStream) {
        let http = &self.http.clone();
        let io = TokioIo::new(stream);

        if let Err(err) = http.serve_connection_with_upgrades(io, service_fn(hello)).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}

impl ConnectionHandler{
    pub fn new() -> Self {
        Self {
            http: server::conn::auto::Builder::new(TokioExecutor::new()),
        }
    }
}
