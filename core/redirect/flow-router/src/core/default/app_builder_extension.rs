use crate::core::{BaseCryptoCache, BaseCryptoStore};
use crate::core::default::CryptoManager;
pub struct DefaultsBuilder<S, C>
where 
    C: BaseCryptoCache + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone
{
    pub crypto_manager: CryptoManager<S, C>,
}

impl<S, C> DefaultsBuilder<S, C>
where 
    C: BaseCryptoCache + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone 
{
    pub async fn new(crypto_store: S, crypto_cache: C) -> Self {
        Self {
            crypto_manager: CryptoManager::new(crypto_store, crypto_cache),
        }
    }
}
