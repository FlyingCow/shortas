use std::fmt;
use thiserror::Error;

use crate::domain::Keycert;


pub type Result<T> = std::result::Result<T, CryptoStoreError>;

pub trait BaseCryptoStore: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;
}

#[derive(Error, Debug)]
pub enum CryptoStoreError {

    #[error("The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be ACTIVE.")]
    ResourceNotFoundException,

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
