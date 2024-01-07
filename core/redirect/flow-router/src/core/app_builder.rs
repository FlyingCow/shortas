
use crate::core::BaseCryptoManager;
use crate::core::BaseCryptoStore;
use crate::core::BaseCryptoCache;
use crate::core::default::CryptoManager;

pub struct AppBuilder<M> 
where 
    M: BaseCryptoManager + Send + Sync + Clone,
{
    crypto_manager: Option<M>,
} 

pub struct CryptoBuilder<M, S, C>
where 
    M: BaseCryptoManager + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{
    crypto_cache: Option<C>,
    crypto_store: Option<S>,
}

impl<M, S, C> CryptoBuilder<M, S, C>
where 
    M: BaseCryptoManager + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone,
    C: BaseCryptoCache + Send + Sync + Clone,
{



    pub fn with_crypto_store(&mut self, crypto_store: S) -> &Self {

        self.crypto_store = Some(crypto_store);
        self
    }

    pub fn with_crypto_cache(&mut self, crypto_cache: C) -> &Self {

        self.crypto_cache = Some(crypto_cache);
        self
    }
    
    fn build(&self) -> M
    {
        let manager = CryptoManager::new(
            self.crypto_store.unwrap(), 
            self.crypto_cache.unwrap()
        );

        manager
    }
}