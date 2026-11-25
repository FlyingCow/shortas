use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;
use tokio::sync::Semaphore;

use crate::adapters::CryptoStoreType;
use crate::core::crypto::CryptoCache;
use crate::core::CryptoStore;
use crate::model::Keycert;

use super::settings::CryptoCacheSettings;

const KEY_PREFIX: &'static str = "crypto";

/// Why a load left nothing in the certificate cache.
#[derive(Debug)]
enum LoadError {
    NotFound,
    Store(anyhow::Error),
}

#[derive(Clone)]
pub struct MokaCryptoCache {
    cache: Cache<String, Arc<Keycert>>,
    /// Names without a certificate. Kept apart, bounded and short-lived, so a
    /// flood of unknown names can't evict real certificates and a newly issued
    /// one shows up quickly even if its notification is lost.
    misses: Cache<String, ()>,
    lookups: Arc<Semaphore>,
    /// Bumped on every invalidation, so a load that was already in flight
    /// can't leave a stale certificate behind.
    generation: Arc<AtomicU64>,
    crypto_store: CryptoStoreType,
}

impl MokaCryptoCache {
    pub fn new(crypto_store: CryptoStoreType, settings: CryptoCacheSettings) -> Self {
        let cache = Cache::builder()
            .max_capacity(settings.max_capacity)
            .time_to_live(Duration::from_secs(settings.time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(settings.time_to_idle_minutes * 60))
            .build();

        let misses = Cache::builder()
            .max_capacity(settings.miss_max_capacity)
            .time_to_live(Duration::from_secs(settings.miss_time_to_live_seconds))
            .build();

        Self {
            cache,
            misses,
            lookups: Arc::new(Semaphore::new(settings.max_concurrent_lookups)),
            generation: Arc::new(AtomicU64::new(0)),
            crypto_store,
        }
    }

    async fn load(&self, server_name: &str) -> Result<Arc<Keycert>, LoadError> {
        let _permit = self
            .lookups
            .acquire()
            .await
            .map_err(|e| LoadError::Store(e.into()))?;

        match self.crypto_store.get_certificate(server_name).await {
            Ok(Some(keycert)) => Ok(Arc::new(keycert)),
            Ok(None) => Err(LoadError::NotFound),
            Err(e) => Err(LoadError::Store(e)),
        }
    }
}

fn get_key(domain: &str) -> String {
    // DNS names are case-insensitive, so every lookup and invalidation has to
    // agree on one spelling.
    format!("{}_{}", KEY_PREFIX, domain.to_ascii_lowercase())
}

#[async_trait::async_trait()]
impl CryptoCache for MokaCryptoCache {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Arc<Keycert>>> {
        let key = get_key(server_name);

        if let Some(keycert) = self.cache.get(&key).await {
            return Ok(Some(keycert));
        }

        if self.misses.get(&key).await.is_some() {
            return Ok(None);
        }

        let server_name = server_name.to_ascii_lowercase();
        let generation = self.generation.load(Ordering::SeqCst);

        // `try_get_with` dedupes concurrent misses and caches nothing on error,
        // so the next handshake retries instead of being stuck with a failure.
        let result = self
            .cache
            .try_get_with(key.clone(), self.load(&server_name))
            .await;
        let invalidated = self.generation.load(Ordering::SeqCst) != generation;

        match result {
            Ok(keycert) => {
                if invalidated {
                    self.cache.invalidate(&key).await;
                }
                Ok(Some(keycert))
            }
            Err(e) => match e.as_ref() {
                LoadError::NotFound => {
                    if !invalidated {
                        self.misses.insert(key, ()).await;
                    }
                    Ok(None)
                }
                LoadError::Store(e) => Err(anyhow::anyhow!(
                    "certificate lookup for {server_name} failed: {e:#}"
                )),
            },
        }
    }

    async fn invalidate(&self, server_name: &str) -> Result<()> {
        let key = get_key(server_name);
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.cache.invalidate(&key).await;
        self.misses.invalidate(&key).await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Returns whatever it currently holds; `Err` simulates an outage.
    struct FakeStore {
        response: Mutex<Result<Option<Keycert>, String>>,
        requested: Mutex<Vec<String>>,
    }

    impl FakeStore {
        fn new(response: Result<Option<Keycert>, String>) -> Arc<Self> {
            Arc::new(Self {
                response: Mutex::new(response),
                requested: Mutex::new(vec![]),
            })
        }

        fn set(&self, response: Result<Option<Keycert>, String>) {
            *self.response.lock().unwrap() = response;
        }

        fn requested(&self) -> Vec<String> {
            self.requested.lock().unwrap().clone()
        }
    }

    #[async_trait::async_trait()]
    impl CryptoStore for FakeStore {
        async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
            self.requested.lock().unwrap().push(server_name.to_string());
            self.response
                .lock()
                .unwrap()
                .clone()
                .map_err(|e| anyhow::anyhow!(e))
        }
    }

    fn cache(store: &Arc<FakeStore>) -> MokaCryptoCache {
        MokaCryptoCache::new(
            CryptoStoreType::Test(store.clone()),
            CryptoCacheSettings {
                max_capacity: 10,
                time_to_live_minutes: 1,
                time_to_idle_minutes: 1,
                miss_max_capacity: 10,
                miss_time_to_live_seconds: 1,
                max_concurrent_lookups: 4,
            },
        )
    }

    fn keycert(cert: &str) -> Keycert {
        Keycert::new().key("key").cert(cert)
    }

    async fn cert(cache: &MokaCryptoCache, name: &str) -> Vec<u8> {
        cache.get_certificate(name).await.unwrap().unwrap().cert.clone()
    }

    #[tokio::test]
    async fn store_error_is_returned_and_not_cached() {
        let store = FakeStore::new(Err("store down".into()));
        let cache = cache(&store);

        let err = cache.get_certificate("go.example.com").await.unwrap_err();
        assert!(err.to_string().contains("store down"));

        store.set(Ok(Some(keycert("recovered"))));
        assert_eq!(cert(&cache, "go.example.com").await, b"recovered");
    }

    #[tokio::test]
    async fn cached_certificate_survives_store_outage() {
        let store = FakeStore::new(Ok(Some(keycert("cached"))));
        let cache = cache(&store);
        cert(&cache, "go.example.com").await;

        store.set(Err("store down".into()));
        assert_eq!(cert(&cache, "go.example.com").await, b"cached");
    }

    #[tokio::test]
    async fn invalidate_makes_replacement_visible() {
        let store = FakeStore::new(Ok(Some(keycert("old"))));
        let cache = cache(&store);
        assert_eq!(cert(&cache, "go.example.com").await, b"old");

        store.set(Ok(Some(keycert("new"))));
        assert_eq!(cert(&cache, "go.example.com").await, b"old");

        cache.invalidate("GO.example.com").await.unwrap();
        assert_eq!(cert(&cache, "go.example.com").await, b"new");
    }

    #[tokio::test]
    async fn server_name_case_is_ignored() {
        let store = FakeStore::new(Ok(Some(keycert("cert"))));
        let cache = cache(&store);

        cert(&cache, "Go.Example.com").await;
        cert(&cache, "go.example.com").await;

        assert_eq!(store.requested(), vec!["go.example.com"]);
    }

    #[tokio::test]
    async fn miss_is_cached_then_expires() {
        let store = FakeStore::new(Ok(None));
        let cache = cache(&store);

        for _ in 0..3 {
            assert!(cache.get_certificate("new.example.com").await.unwrap().is_none());
        }
        assert_eq!(store.requested().len(), 1);

        store.set(Ok(Some(keycert("issued"))));
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(cert(&cache, "new.example.com").await, b"issued");
    }

    #[tokio::test]
    async fn invalidate_clears_a_miss() {
        let store = FakeStore::new(Ok(None));
        let cache = cache(&store);
        assert!(cache.get_certificate("new.example.com").await.unwrap().is_none());

        store.set(Ok(Some(keycert("issued"))));
        cache.invalidate("new.example.com").await.unwrap();
        assert_eq!(cert(&cache, "new.example.com").await, b"issued");
    }

    #[tokio::test]
    async fn unknown_names_are_bounded_and_keep_real_certificates() {
        let store = FakeStore::new(Ok(Some(keycert("real"))));
        let cache = cache(&store);
        cert(&cache, "go.example.com").await;

        store.set(Ok(None));
        for i in 0..100 {
            cache.get_certificate(&format!("x{i}.invalid")).await.unwrap();
        }
        cache.misses.run_pending_tasks().await;

        assert!(cache.misses.entry_count() <= 10);
        assert_eq!(cert(&cache, "go.example.com").await, b"real");
    }
}
