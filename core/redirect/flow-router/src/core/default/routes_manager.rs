use tracing::error;

use crate::core::{ 
    BaseRoutesCache, 
    BaseRoutesStore, 
    base_routes_manager:: { 
        BaseRoutesManager, 
        Result 
    }
};

use crate::domain::Route;

#[derive(Copy, Clone, Debug)]
pub struct RoutesManager<S, C>
where
    S: BaseRoutesStore + Send + Sync + Clone,
    C: BaseRoutesCache + Send + Sync + Clone,
{
    routes_store: S,
    routes_cache: C,
}


fn get_key(domain: &str, link: &str) -> String {
    let domain_str = domain.to_ascii_lowercase();
    let link_str = link.to_ascii_lowercase();

    domain_str + "%2F" + &link_str
}

impl<S, C> BaseRoutesManager for RoutesManager<S, C>
where
    S: BaseRoutesStore,
    C: BaseRoutesCache,
{

    async fn get_route(
        &self,
        switch: &str,
        domain: &str,
        path: &str) -> Result<Option<Route>> {
        
        let key = get_key(domain, path);

        let route = self.routes_cache.get_route(switch, &key.clone(), async move {

            let route_result = self.routes_store.get_route(switch, &key.clone()).await;

            match route_result {
                Ok(k) => k,
                Err(err) => {
                    error!("Can not extract route for {}|{}, with error {}", switch, &key.clone(), err);
                    None
                }
            }
        }).await;

        Ok(route.unwrap())
    }
}

impl<S, C> RoutesManager<S, C>
where
    S: BaseRoutesStore + Send + Sync + Clone,
    C: BaseRoutesCache + Send + Sync + Clone,
{
    pub fn new(routes_store: S, routes_cache: C) -> Self {
        Self {
            routes_store,
            routes_cache,
        }
    }
}