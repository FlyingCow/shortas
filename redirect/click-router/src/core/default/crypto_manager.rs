use anyhow::Result;

use crate::core::{crypto_manage::BaseCryptoManager, BaseCryptoStore};

use crate::model::Keycert;

const DEFAULT: &'static str = "default";

#[derive(Clone)]
pub struct DefaultCryptoManager {
    crypto_store: Box<dyn BaseCryptoStore + Send + Sync>,
}

#[async_trait::async_trait()]
impl BaseCryptoManager for DefaultCryptoManager {
    async fn get_default_certificate(&self) -> Result<Option<Keycert>> {
        self.get_certificate(DEFAULT).await
    }

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let keycert_result = self.crypto_store.get_certificate(server_name).await;

        Ok(keycert_result.unwrap())
    }
}

impl DefaultCryptoManager {
    pub fn new(crypto_store: Box<dyn BaseCryptoStore + Send + Sync>) -> Self {
        Self { crypto_store }
    }
}
