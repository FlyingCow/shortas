use std::sync::Arc;

use anyhow::Result;

use crate::adapters::RoutesCacheType;

use crate::model::Route;

#[async_trait::async_trait()]
pub trait RoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>>;
}

#[async_trait::async_trait()]
pub trait RoutesCache {
    /// Returns an `Arc<Route>` so a cache hit only bumps a refcount instead of
    /// deep-cloning the whole `Route` (including its nested `Vec`s/JSON blobs).
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Arc<Route>>>;
    async fn invalidate(&self, switch: &str, path: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct RoutesManager {
    routes_cache: RoutesCacheType,
}

/// Builds the cache key in a single pass/allocation. `domain` and `path` are
/// already lowercased by the caller (see `FlowRouter::build_route_uri`), but
/// we still lowercase defensively here without paying for two extra
/// intermediate `String` allocations plus the concatenation.
fn get_key(domain: &str, link: &str) -> String {
    let mut key = String::with_capacity(domain.len() + link.len() + 3);
    key.extend(domain.chars().map(|c| c.to_ascii_lowercase()));
    key.push_str("%2F");
    key.extend(link.chars().map(|c| c.to_ascii_lowercase()));
    key
}

impl RoutesManager {
    pub async fn get_route(
        &self,
        switch: &str,
        domain: &str,
        path: &str,
    ) -> Result<Option<Arc<Route>>> {
        let key = get_key(domain, path);

        let route_result = self.routes_cache.get_route(switch, key.as_str()).await;

        Ok(route_result.unwrap())
    }

    /// Get a route by switch and raw path (without key construction)
    /// Used for special routes like custom index and 404 pages
    pub async fn get_route_by_switch(
        &self,
        switch: &str,
        path: &str,
    ) -> Result<Option<Arc<Route>>> {
        let route_result = self.routes_cache.get_route(switch, path).await;

        Ok(route_result.unwrap())
    }
}

impl RoutesManager {
    pub fn new(routes_cache: RoutesCacheType) -> Self {
        Self { routes_cache }
    }
}
