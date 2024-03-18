use std::{fmt, net::SocketAddr};

use http::{Request, StatusCode, Uri};

use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;
pub enum RedirectType {
    Permanent,
    Temporary,
}
pub enum FlowRouterResult {
    Empty(StatusCode),
    Json(String, StatusCode),
    PlainText(String, StatusCode),
    Proxied(Uri, StatusCode),
    Redirect(Uri, RedirectType),
    Retargeting(Uri, Vec<Uri>),
    Error,
}

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
    fn handle(
        &self,
        req: PerRequestData<Req>,
    ) -> impl std::future::Future<Output = Result<FlowRouterResult>> + Send;
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
