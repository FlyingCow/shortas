use tracing::{error};

use crate::core::{ 
    BaseCryptoCache, 
    BaseCryptoStore, 
    base_crypto_manager:: { 
        BaseCryptoManager, 
        Result 
    }
};

use crate::domain::Keycert;


const DEFAULT: &'static str = "default";


#[derive(Copy, Clone, Debug)]
pub struct CryptoManager<S, C>
where
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{
    crypto_store: S,
    crypto_cache: C,
}

impl<S, C> BaseCryptoManager for CryptoManager<S, C>
where
    S: BaseCryptoStore,
    C: BaseCryptoCache,
{


    async fn get_default_certificate(&self) -> Result<Option<Keycert>> {
        self.get_certificate(DEFAULT).await
    }

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        
        let keycert: std::prelude::v1::Result<Option<Keycert>, anyhow::Error> = self.crypto_cache.get_certificate(server_name, async move {

            let keycert_result = self.crypto_store.get_certificate(server_name).await;

            match keycert_result {
                Ok(k) => k,
                Err(err) => {
                    error!("Can not extract encryption certificate for {}, with error {}", server_name, err);
                    None
                }
            }
        }).await;

        Ok(keycert.unwrap())
    }
}

impl<S, C> CryptoManager<S, C>
where
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{
    pub fn new(crypto_store: S, crypto_cache: C) -> Self {
        Self {
            crypto_store,
            crypto_cache,
        }
    }
}