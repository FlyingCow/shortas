use crate::{CryptoStore, Keycert};
use async_trait::async_trait;
use std::io::{Error as IoError, ErrorKind, Read, Result as IoResult};
use std::sync::Arc;

pub struct DynamoCryptoStore {}

impl DynamoCryptoStore {
    #[inline]
    pub fn new() -> Self {
        DynamoCryptoStore {}
    }
}

#[async_trait]
impl CryptoStore for DynamoCryptoStore {
    async fn get_certificate(&self, server_name: &str) -> IoResult<Arc<Keycert>> {
        println!("extracting certificate for:{}", server_name);

        IoResult::Ok(Arc::new(Keycert::new()))
    }
}

