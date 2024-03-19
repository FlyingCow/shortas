use crate::core::default::{CryptoManager, FlowRouter};
use crate::core::{
    BaseCryptoCache, 
    BaseCryptoStore,
    BaseRoutesCache, 
    BaseRoutesStore
};

use super::flow_router::flow_router::FlowRouterContext;
use super::flow_router::Middleware;
use super::RoutesManager;

pub struct DefaultsBuilder<CS, CC, RS, RC>
where
    CC: BaseCryptoCache + Send + Sync + Clone + 'static,
    CS: BaseCryptoStore + Send + Sync + Clone + 'static,
    RC: BaseRoutesCache + Send + Sync + Clone + 'static,
    RS: BaseRoutesStore + Send + Sync + Clone + 'static,
{
    pub crypto_manager: CryptoManager<CS, CC>,
    pub routes_manager: RoutesManager<RS, RC>,
    pub flow_router: FlowRouter<RoutesManager<RS, RC>>,
}

impl<CS, CC, RS, RC> DefaultsBuilder<CS, CC, RS, RC>
where
    CC: BaseCryptoCache + Send + Sync + Clone,
    CS: BaseCryptoStore + Send + Sync + Clone,
    RC: BaseRoutesCache + Send + Sync + Clone,
    RS: BaseRoutesStore + Send + Sync + Clone,
{
    pub async fn new(crypto_store: CS, crypto_cache: CC, routes_store: RS, routes_cache: RC) -> Self {
        let routes_manager = RoutesManager::new(routes_store, routes_cache);
        let modules: Vec<Box<dyn Middleware<FlowRouter<RoutesManager<RS, RC>>, FlowRouterContext>>> = vec![];
        Self {
            crypto_manager: CryptoManager::new(crypto_store, crypto_cache),
            routes_manager: routes_manager.clone(),
            flow_router: FlowRouter::new(routes_manager.clone(), modules),
        }
    }
}
