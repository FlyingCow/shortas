use std::future::Future;
use std::time::Duration;

use moka::future::Cache;

use crate::domain::Keycert;
use crate::core::base_crypto_cache::{ 
    BaseCryptoCache,
    Result
};

#[derive(Clone, Debug)]
pub struct KeycertCacheItem {
    value: Option<Keycert>
}

#[derive(Clone, Debug)]
pub struct CryptoCache {
    cache:  Cache<String, KeycertCacheItem>
}

impl CryptoCache {
    pub fn new(max_capacity: u64, time_to_live_minutes: u64, time_to_idle_minutes: u64) -> Self {

        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(time_to_idle_minutes * 60))
            // .eviction_listener(|key, value, cause| {
            //     println!("Evicted ({key:?},{value:?}) because {cause:?}")
            // })
            .build();
    
        Self {
            cache
        }
    }
}

async fn init(init: impl Future<Output = Option<Keycert>>) -> KeycertCacheItem {

    let keycert_result = init.await;

    KeycertCacheItem{
        value: keycert_result
    }
}

impl BaseCryptoCache for CryptoCache {
    async fn get_certificate(&self, server_name: &str, init_fn: impl Future<Output = Option<Keycert>>) -> Result<Option<Keycert>> {

        let cache_result = self.cache.get_with(server_name.to_ascii_lowercase(), async move {
            let cache_item = init(init_fn).await;

            cache_item
        }).await;

        Ok(cache_result.value)
    }

    async fn remove_certificate(&self, server_name: &str) -> Result<()> {
        self.cache.remove(&server_name.to_ascii_lowercase()).await;

        Ok(())
    }
}