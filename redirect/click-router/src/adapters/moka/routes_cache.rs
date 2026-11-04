use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::adapters::RoutesStoreType;
use crate::core::routes::RoutesCache;
use crate::core::RoutesStore;
use crate::core::metrics::{METRICS, Timer};
use crate::model::Route;

use super::settings::RoutesCacheSettings;

// Thread-local buffer for cache key generation to reduce allocations
thread_local! {
    static KEY_BUFFER: RefCell<String> = RefCell::new(String::with_capacity(256));
}

#[derive(Clone)]
pub struct MokaRoutesCache {
    // `Option<Arc<Route>>` as the value type means a cache hit only clones an
    // `Arc` (a refcount bump) instead of deep-cloning the whole `Route`
    // (its Strings, Vec<String> tags/scripts, and JSON blobs) on every
    // request that hits the cache.
    cache: Cache<String, Option<Arc<Route>>>,
    routes_store: RoutesStoreType,
}

impl MokaRoutesCache {
    pub fn new(routes_store: RoutesStoreType, settings: RoutesCacheSettings) -> Self {
        let cache = Cache::builder()
            .max_capacity(settings.max_capacity)
            .time_to_live(Duration::from_secs(settings.time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(settings.time_to_idle_minutes * 60))
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

/// Optimized cache key generation using thread-local buffer to reduce allocations
/// This eliminates 2-4 allocations per request on the hot path
fn get_key(switch: &str, link: &str) -> String {
    KEY_BUFFER.with(|buf| {
        let mut buffer = buf.borrow_mut();
        buffer.clear();
        buffer.reserve(switch.len() + link.len() + 1);

        // Build the key: switch|link (lowercase)
        for c in switch.chars() {
            buffer.push(c.to_ascii_lowercase());
        }
        buffer.push('|');
        for c in link.chars() {
            buffer.push(c.to_ascii_lowercase());
        }

        buffer.clone()
    })
}

#[async_trait::async_trait()]
impl RoutesCache for MokaRoutesCache {
    async fn invalidate(&self, switch: &str, path: &str) -> Result<()> {
        let key = get_key(switch, path);

        self.cache.invalidate(&key).await;

        Ok(())
    }

    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Arc<Route>>> {
        let key = get_key(switch, path);
        let cache_timer = Timer::new();

        // Check if key exists in cache first - cheap clone (Arc refcount bump)
        if let Some(cached) = self.cache.get(&key).await {
            METRICS.route_cache_hits.inc();
            cache_timer.observe_duration_seconds(&METRICS.cache_lookup_duration);
            return Ok(cached);
        }

        // Cache miss - fetch from database
        METRICS.route_cache_misses.inc();
        METRICS.route_db_queries.inc();

        let db_timer = Timer::new();
        let route_result = self.routes_store.get_route(switch, path).await;
        db_timer.observe_duration_seconds(&METRICS.db_query_duration);

        let value: Option<Arc<Route>> = route_result.unwrap().map(Arc::new);

        // Store in cache (cheap: cloning Option<Arc<Route>> is a refcount bump)
        self.cache.insert(key, value.clone()).await;
        cache_timer.observe_duration_seconds(&METRICS.cache_lookup_duration);

        Ok(value)
    }
}
