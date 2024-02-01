use std::future::Future;
use std::time::Duration;

use moka::future::Cache;

use crate::core::base_routes_cache::{BaseRoutesCache, Result};
use crate::domain::Route;

#[derive(Clone, Debug)]
pub struct RouteCacheItem {
    value: Option<Route>,
}

#[derive(Clone, Debug)]
pub struct RoutesCache {
    cache: Cache<String, RouteCacheItem>,
}

impl RoutesCache {
    pub fn new(max_capacity: u64, time_to_live_minutes: u64, time_to_idle_minutes: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(time_to_idle_minutes * 60))
            // .eviction_listener(|key, value, cause| {
            //     println!("Evicted ({key:?},{value:?}) because {cause:?}")
            // })
            .build();

        Self { cache }
    }
}

async fn init(init: impl Future<Output = Option<Route>>) -> RouteCacheItem {
    let route_result = init.await;

    RouteCacheItem {
        value: route_result,
    }
}

fn get_key(switch: &str, link: &str) -> String {
    let switch_str = switch.to_ascii_lowercase();
    let link_str = link.to_ascii_lowercase();

    switch_str + "|" + &link_str
}

impl BaseRoutesCache for RoutesCache {
    async fn get_route(
        &self,
        switch: &str,
        path: &str,
        init_fn: impl std::future::Future<Output = Option<Route>>,
    ) -> Result<Option<Route>> {
        let key = get_key(switch, path);

        let cache_result = self
            .cache
            .get_with(key, async move {
                let cache_item = init(init_fn).await;

                cache_item
            })
            .await;

        Ok(cache_result.value)
    }

    async fn remove_route(&self, switch: &str, path: &str) -> Result<()> {
        let key = get_key(switch, path);

        self.cache
            .remove(&key)
            .await;

        Ok(())
    }
}
