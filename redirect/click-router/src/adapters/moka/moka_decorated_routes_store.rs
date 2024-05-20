use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::core::BaseRoutesStore;
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct RouteCacheItem {
    value: Option<Route>,
}

#[derive(Clone)]
pub struct MokaDecoratedRoutesStore {
    cache: Cache<String, RouteCacheItem>,
    routes_store: Box<dyn BaseRoutesStore>,
}

impl MokaDecoratedRoutesStore {
    pub fn new(
        routes_store: Box<dyn BaseRoutesStore>,
        max_capacity: u64,
        time_to_live_minutes: u64,
        time_to_idle_minutes: u64,
    ) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(time_to_idle_minutes * 60))
            // .eviction_listener(|key, value, cause| {
            //     println!("Evicted ({key:?},{value:?}) because {cause:?}")
            // })
            .build();

        Self {
            cache,
            routes_store,
        }
    }
}

fn get_key(switch: &str, link: &str) -> String {
    format!("{}|{}", switch, link).to_ascii_lowercase()
}

#[async_trait::async_trait(?Send)]
impl BaseRoutesStore for MokaDecoratedRoutesStore {
    async fn invalidate(&self, switch: &str, path: &str) -> Result<()> {
        let key = get_key(switch, path);

        self.cache.invalidate(&key).await;

        Ok(())
    }

    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let key = get_key(switch, path);

        let cache_result = self
            .cache
            .get_with(key, async move {
                let route_result = self.routes_store.get_route(switch, path).await;
                RouteCacheItem {
                    value: route_result.unwrap(),
                }
            })
            .await;

        Ok(cache_result.value)
    }
}
