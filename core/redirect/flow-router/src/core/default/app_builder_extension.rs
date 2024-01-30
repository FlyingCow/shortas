use crate::core::default::{CryptoManager, FlowRouter};
use crate::core::{BaseCryptoCache, BaseCryptoStore, BaseFlowRouter};

pub struct DefaultsBuilder<S, C>
where
    C: BaseCryptoCache + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone,
{
    pub crypto_manager: CryptoManager<S, C>,
    pub flow_router: FlowRouter,
}

impl<S, C> DefaultsBuilder<S, C>
where
    C: BaseCryptoCache + Send + Sync + Clone,
    S: BaseCryptoStore + Send + Sync + Clone,
{
    pub async fn new(crypto_store: S, crypto_cache: C) -> Self {
        Self {
            crypto_manager: CryptoManager::new(crypto_store, crypto_cache),
            flow_router: FlowRouter {},
        }
    }
}
