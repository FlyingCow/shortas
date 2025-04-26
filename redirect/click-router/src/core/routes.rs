use anyhow::Result;

use crate::adapters::RoutesCacheType;

use crate::model::Route;

#[async_trait::async_trait()]
pub trait RoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>>;
}

#[async_trait::async_trait()]
pub trait RoutesCache {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>>;
    async fn invalidate(&self, switch: &str, path: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct RoutesManager {
    routes_cache: RoutesCacheType,
}

fn get_key(domain: &str, link: &str) -> String {
    let domain_str = domain.to_ascii_lowercase();
    let link_str = link.to_ascii_lowercase();

    domain_str + "%2F" + &link_str
}

impl RoutesManager {
    pub async fn get_route(&self, switch: &str, domain: &str, path: &str) -> Result<Option<Route>> {
        let key = get_key(domain, path);

        let route_result = self.routes_cache.get_route(switch, key.as_str()).await;

        Ok(route_result.unwrap())
    }
}

impl RoutesManager {
    pub fn new(routes_cache: RoutesCacheType) -> Self {
        Self { routes_cache }
    }
}
