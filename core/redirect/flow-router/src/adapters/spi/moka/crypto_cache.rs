use std::time::Duration;

use moka::future::Cache;

use crate::domain::Keycert;
use crate::core::base_crypto_cache::{ 
    BaseCryptoCache,
    KeycertContainer,
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

impl BaseCryptoCache for CryptoCache {
    async fn get_certificate(&self, server_name: &str) -> Result<KeycertContainer> {
        let cache_result = self.cache.get(&server_name.to_ascii_lowercase()).await;
        match cache_result {
            Some(c) => Ok(KeycertContainer { 
                value: c.value,
                from_cache: true
            }),
            None => Ok(KeycertContainer { 
                value: None,
                from_cache: false
            })
        }
    }

    async fn add_certificate(&self, server_name: &str, keycert: Option<Keycert>) -> Result<()> {
        self.cache.insert(server_name.to_ascii_lowercase(), KeycertCacheItem{
            value: keycert
        }).await;

        Ok(())
    }

    async fn remove_certificate(&self, server_name: &str) -> Result<()> {
        self.cache.remove(&server_name.to_ascii_lowercase()).await;

        Ok(())
    }
}