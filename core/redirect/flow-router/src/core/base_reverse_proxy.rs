use std::{convert::Infallible, fmt, pin::Pin};

use bytes::Bytes;
use futures_util::Future;
use http::{uri::InvalidUri, Response};
use http_body_util::combinators::BoxBody;
use thiserror::Error;


pub type Result<T> = std::result::Result<T, ProxyError>;

pub trait BaseReverseProxy: Clone
{
    fn forward_uri(&self, forward_url: &str)
        -> Pin<Box<dyn Future<Output = Result<Response<BoxBody<Bytes, Infallible>>>> + Send>>;
}


#[derive(Error, Debug)]
pub enum ProxyError {

    #[error("Invalid Uri: `{0}`")]
    InvalidUri(InvalidUri),
    #[error("Forward headers error")]
    ForwardHeaderError,
    #[error("Upgrade error `{0}`")]
    UpgradeError(String),
    #[error("Unknown server error `{0}`")]
    ServerError(ServerError),
}

#[derive(Error, Debug)]
pub struct ServerError {
    msg: String,
    #[source] // optional if field name is `source`
    source: anyhow::Error,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}