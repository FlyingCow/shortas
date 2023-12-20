use std::fmt;
use crate::domain::Keycert;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CryptoCacheError>;

pub trait BaseCryptoCache: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;
}

#[derive(Error, Debug)]
pub enum CryptoCacheError {
    #[error("unknown data store error")]
    Other(CryptoStoreOtherError),
}

#[derive(Error, Debug)]
pub struct CryptoStoreOtherError {
    msg: String,
    #[source]  // optional if field name is `source`
    source: anyhow::Error,
}

impl fmt::Display for CryptoStoreOtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}