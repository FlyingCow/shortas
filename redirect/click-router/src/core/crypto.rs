use anyhow::Result;

use crate::adapters::CryptoCacheType;

use crate::model::Keycert;

const DEFAULT: &'static str = "default";

#[async_trait::async_trait()]
pub trait CryptoStore {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>>;
}

#[async_trait::async_trait()]
pub trait CryptoCache {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>>;

    async fn invalidate(&self, server_name: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct CryptoManager {
    crypto_cache: CryptoCacheType,
}

impl CryptoManager {
    async fn get_default_certificate(&self) -> Result<Option<Keycert>> {
        self.get_certificate(DEFAULT).await
    }

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let keycert_result = self.crypto_cache.get_certificate(server_name).await;

        Ok(keycert_result.unwrap())
    }
}

impl CryptoManager {
    pub fn new(crypto_cache: CryptoCacheType) -> Self {
        Self { crypto_cache }
    }
}
