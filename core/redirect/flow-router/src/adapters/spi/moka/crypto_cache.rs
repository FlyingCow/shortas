use crate::domain::Keycert;
use crate::core::base_crypto_store::{ 
    BaseCryptoStore, 
    CryptoStoreError, 
    Result
};

#[derive(Clone, Debug)]
pub struct CryptoStore {}

impl CryptoStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl BaseCryptoStore for CryptoStore {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        Ok(Some(
            Keycert::new()
        ))
    }
}