use std::fmt;
use thiserror::Error;

use crate::domain::Keycert;

pub type Result<T> = std::result::Result<T, CryptoManagerError>;

pub trait BaseCryptoManager: Send + Sync + Clone {
    fn get_default_certificate(
        &self,
    ) -> impl std::future::Future<Output = Result<Keycert>> + Send;

    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;
}

#[derive(Error, Debug)]
pub enum CryptoManagerError {
    #[error("unknown data store error")]
    Other(CryptoManagerOtherError),
}

#[derive(Error, Debug)]
pub struct CryptoManagerOtherError {
    msg: String,
    #[source]  // optional if field name is `source`
    source: anyhow::Error,
}

impl fmt::Display for CryptoManagerOtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}