use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::adapters::CryptoStoreType;
use crate::core::crypto::CryptoCache;
use crate::core::CryptoStore;
use crate::model::Keycert;

use super::settings::CryptoCacheSettings;

const KEY_PREFIX: &'static str = "crypto";

#[derive(Clone, Debug)]
pub struct KeycertCacheItem {
    value: Option<Keycert>,
}

#[derive(Clone)]
pub struct MokaCryptoCache {
    cache: Cache<String, KeycertCacheItem>,
    crypto_store: CryptoStoreType,
}

impl MokaCryptoCache {
    pub fn new(crypto_store: CryptoStoreType, settings: CryptoCacheSettings) -> Self {
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
            crypto_store,
        }
    }
}

fn get_key(domain: &str) -> String {
    let domain_str = format!("{}_{}", KEY_PREFIX, domain);

    domain_str
}

#[async_trait::async_trait()]
impl CryptoCache for MokaCryptoCache {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let key = get_key(server_name);

        let cache_result = self
            .cache
            .get_with(key, async move {
                let cert_result = self.crypto_store.get_certificate(server_name).await;
                KeycertCacheItem {
                    value: cert_result.unwrap(),
                }
            })
            .await;

        Ok(cache_result.value)
    }

    async fn invalidate(&self, server_name: &str) -> Result<()> {
        self.cache.invalidate(server_name).await;

        Ok(())
    }
}
