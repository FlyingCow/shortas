use std::{convert::Infallible, fmt, net::SocketAddr, pin::Pin};

use bytes::Bytes;
use futures_util::Future;
use http::{Request, Response};
use http_body_util::Full;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Infallible>;

#[derive(Clone)]
pub struct PerConnHandler {
    pub local_addr: SocketAddr,
    pub remote_addr: SocketAddr,
    pub server_name: String,
    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone)]
pub struct PerRequestData<B>
where
    B: Send + Sync + 'static,
{
    // pub local_addr: SocketAddr,
    // pub remote_addr: SocketAddr,
    pub tls_info: Option<TlsInfo>,
    pub request: Request<B>,
}

#[derive(Clone)]
pub struct TlsInfo {
    pub sni_hostname: Option<String>,
    pub alpn_protocol: Option<String>,
    pub has_certificate: bool,
}

pub trait BaseFlowRouter<Req>: Clone
where
    Req: Send + Sync,
{
    fn handle(&self, req: PerRequestData<Req>)
        -> Pin<Box<dyn Future<Output = Result<Response<Full<Bytes>>>> + Send>>;
}

#[derive(Error, Debug)]
pub enum FlowRouterError {
    #[error("The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be ACTIVE.")]
    ResourceNotFoundException,

    #[error("unknown data store error")]
    Other(FlowRouterErrorOtherError),
}

#[derive(Error, Debug)]
pub struct FlowRouterErrorOtherError {
    msg: String,
    #[source] // optional if field name is `source`
    source: anyhow::Error,
}

impl fmt::Display for FlowRouterErrorOtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
