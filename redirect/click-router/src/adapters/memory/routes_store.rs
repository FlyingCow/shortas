use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::core::RoutesStore;
use crate::model::Route;

/// In-memory routes store for testing and benchmarking
///
/// This store provides a simple HashMap-based implementation that doesn't
/// require any external dependencies like databases or caches. It's designed
/// for use in tests and benchmarks where you want full control over the data
/// and minimal setup overhead.
#[derive(Clone)]
pub struct InMemoryRoutesStore {
    routes: Arc<RwLock<HashMap<String, Route>>>,
}

impl InMemoryRoutesStore {
    /// Create a new empty in-memory routes store
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a store pre-populated with routes
    pub fn with_routes(routes: Vec<Route>) -> Self {
        let mut map = HashMap::new();
        for route in routes {
            let key = Self::get_key(&route.switch, &route.link);
            map.insert(key, route);
        }

        Self {
            routes: Arc::new(RwLock::new(map)),
        }
    }

    /// Insert a route into the store
    pub async fn insert(&self, switch: &str, path: &str, route: Route) {
        let key = Self::get_key(switch, path);
        let mut routes = self.routes.write().await;
        routes.insert(key, route);
    }

    /// Remove a route from the store
    pub async fn remove(&self, switch: &str, path: &str) {
        let key = Self::get_key(switch, path);
        let mut routes = self.routes.write().await;
        routes.remove(&key);
    }

    /// Clear all routes from the store
    pub async fn clear(&self) {
        let mut routes = self.routes.write().await;
        routes.clear();
    }

    /// Get the number of routes in the store
    pub async fn len(&self) -> usize {
        let routes = self.routes.read().await;
        routes.len()
    }

    /// Generate the cache key for a switch/path combination
    fn get_key(switch: &str, path: &str) -> String {
        format!("{}|{}", switch.to_lowercase(), path.to_lowercase())
    }
}

impl Default for InMemoryRoutesStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RoutesStore for InMemoryRoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let key = Self::get_key(switch, path);
        let routes = self.routes.read().await;
        Ok(routes.get(&key).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::route::{RouteStatus, RoutingPolicy, RoutingTerminal, DestinationFormat, RouteProperties};

    fn create_test_route(switch: &str, link: &str) -> Route {
        Route {
            switch: switch.to_string(),
            link: link.to_string(),
            dest: Some(format!("https://example.com/{}", link)),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties::default(),
        }
    }

    #[tokio::test]
    async fn test_empty_store() {
        let store = InMemoryRoutesStore::new();
        let result = store.get_route("main", "test").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_insert_and_retrieve() {
        let store = InMemoryRoutesStore::new();
        let route = create_test_route("main", "test");

        store.insert("main", "test", route.clone()).await;

        let result = store.get_route("main", "test").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().link, "test");
    }

    #[tokio::test]
    async fn test_with_routes() {
        let routes = vec![
            create_test_route("main", "route1"),
            create_test_route("main", "route2"),
            create_test_route("alt", "route3"),
        ];

        let store = InMemoryRoutesStore::with_routes(routes);

        assert!(store.get_route("main", "route1").await.unwrap().is_some());
        assert!(store.get_route("main", "route2").await.unwrap().is_some());
        assert!(store.get_route("alt", "route3").await.unwrap().is_some());
        assert!(store.get_route("main", "notfound").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_case_insensitive() {
        let store = InMemoryRoutesStore::new();
        let route = create_test_route("main", "TeSt");

        store.insert("MAIN", "test", route).await;

        let result = store.get_route("main", "TEST").await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_remove() {
        let store = InMemoryRoutesStore::new();
        let route = create_test_route("main", "test");

        store.insert("main", "test", route).await;
        assert!(store.get_route("main", "test").await.unwrap().is_some());

        store.remove("main", "test").await;
        assert!(store.get_route("main", "test").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_clear() {
        let store = InMemoryRoutesStore::new();
        store.insert("main", "route1", create_test_route("main", "route1")).await;
        store.insert("main", "route2", create_test_route("main", "route2")).await;

        assert_eq!(store.len().await, 2);

        store.clear().await;
        assert_eq!(store.len().await, 0);
    }
}
