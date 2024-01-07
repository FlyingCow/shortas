use std::fmt;
use crate::domain::Keycert;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CryptoCacheError>;

pub trait BaseCryptoCache: Send + Sync + Clone {
    fn get_certificate(
        &self,
        server_name: &str,
        init: impl std::future::Future<Output = Option<Keycert>> + Send
    ) -> impl std::future::Future<Output = Result<Option<Keycert>>> + Send;

    fn remove_certificate(&self,
        server_name: &str
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

//Even null certificates should be cached to minimize db hit 
//in case there is no certificate for a specified domain name
#[derive(Clone, Debug)]
pub struct KeycertContainer {
    
    pub value: Option<Keycert>,

    ///Specifies if current container is coming from cache
    ///If false - db should be hit and the cache fulfilled 
    pub from_cache: bool
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