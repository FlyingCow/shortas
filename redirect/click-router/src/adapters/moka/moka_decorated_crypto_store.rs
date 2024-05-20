use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::core::BaseCryptoStore;
use crate::model::Keycert;

const KEY_PREFIX: &'static str = "crypto";

#[derive(Clone, Debug)]
pub struct KeycertCacheItem {
    value: Option<Keycert>,
}

#[derive(Clone)]
pub struct MokaDecoratedCryptoStore {
    cache: Cache<String, KeycertCacheItem>,
    crypto_store: Box<dyn BaseCryptoStore>,
}

impl MokaDecoratedCryptoStore {
    pub fn new(
        crypto_store: Box<dyn BaseCryptoStore>,
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
            crypto_store,
        }
    }
}

fn get_key(domain: &str) -> String {
    let domain_str = format!("{}_{}", KEY_PREFIX, domain);

    domain_str
}

#[async_trait::async_trait(?Send)]
impl BaseCryptoStore for MokaDecoratedCryptoStore {
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
