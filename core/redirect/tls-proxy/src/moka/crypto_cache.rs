use crate::{CryptoCache, Keycert};
use async_trait::async_trait;
use std::io::{Error as IoError, ErrorKind, Read, Result as IoResult};
use std::sync::Arc;


pub struct MokaCryptoCache {}

impl MokaCryptoCache {
    #[inline]
    pub fn new() -> Self {
        MokaCryptoCache {}
    }
}

#[async_trait]
impl CryptoCache for MokaCryptoCache {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Arc<Keycert>> {
        println!("extracting cached certificate for:{}", server_name);
        Err(IoError::new(ErrorKind::NotFound, "Certificate not found"))
        //IoResult::Ok(Arc::new(Keycert::new()))
    }
}