use std::fmt;
use crate::domain::Keycert;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CryptoStoreError>;

pub trait BaseCryptoStore: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;
}

#[derive(Error, Debug)]
pub enum CryptoStoreError {

    #[error("A source table with the name {} does not currently exist within the subscriber's account or the subscriber is operating in the wrong Services Region.", .0)]
    TableNotFoundException(String),

    #[error("The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be ACTIVE.")]
    ResourceNotFoundException,

    #[error("Throughput exceeds the current throughput quota for your account. Please contact Support to request a quota increase.")]
    RequestLimitExceeded,

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
