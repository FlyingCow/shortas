use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::{
    adapters::moka::settings::ConversionCacheSettings, core::conversion_definitions::ConversionStore,
    model::Conversion,
};

/// Public keys are short; anything longer is not looked up.
const MAX_KEY_LENGTH: usize = 64;

#[derive(Debug)]
enum LoadError {
    NotFound,
    Store(anyhow::Error),
}

/// Conversion definitions by public key. Entries live for a minute, so a pause
/// or key rotation applies even if its notification is lost.
#[derive(Clone)]
pub struct MokaConversionCache {
    cache: Cache<String, Arc<Conversion>>,
    /// Unknown keys, apart and bounded so they can't evict real entries.
    misses: Cache<String, ()>,
    /// Bumped on every invalidation, so a load already in flight can't leave
    /// an outdated definition behind.
    generation: Arc<AtomicU64>,
    store: Arc<dyn ConversionStore>,
}

impl MokaConversionCache {
    pub fn new(store: Arc<dyn ConversionStore>, settings: &ConversionCacheSettings) -> Self {
        Self {
            cache: Cache::builder()
                .max_capacity(settings.max_capacity)
                .time_to_live(Duration::from_secs(settings.time_to_live_seconds))
                .build(),
            misses: Cache::builder()
                .max_capacity(settings.miss_max_capacity)
                .time_to_live(Duration::from_secs(settings.miss_time_to_live_seconds))
                .build(),
            generation: Arc::new(AtomicU64::new(0)),
            store,
        }
    }

    pub async fn get(&self, public_key: &str) -> Result<Option<Arc<Conversion>>> {
        if public_key.len() > MAX_KEY_LENGTH {
            return Ok(None);
        }
        if let Some(conversion) = self.cache.get(public_key).await {
            return Ok(Some(conversion));
        }
        if self.misses.get(public_key).await.is_some() {
            return Ok(None);
        }

        let generation = self.generation.load(Ordering::SeqCst);
        // Errors are not cached, so the next event retries.
        let result = self
            .cache
            .try_get_with(public_key.to_string(), async {
                match self.store.get_conversion(public_key).await {
                    Ok(Some(conversion)) => Ok(Arc::new(conversion)),
                    Ok(None) => Err(LoadError::NotFound),
                    Err(e) => Err(LoadError::Store(e)),
                }
            })
            .await;
        let invalidated = self.generation.load(Ordering::SeqCst) != generation;

        match result {
            Ok(conversion) => {
                if invalidated {
                    self.cache.invalidate(public_key).await;
                }
                Ok(Some(conversion))
            }
            Err(e) => match e.as_ref() {
                LoadError::NotFound => {
                    if !invalidated {
                        self.misses.insert(public_key.to_string(), ()).await;
                    }
                    Ok(None)
                }
                LoadError::Store(e) => Err(anyhow::anyhow!("conversion lookup failed: {e:#}")),
            },
        }
    }

    pub async fn invalidate(&self, public_key: &str) {
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.cache.invalidate(public_key).await;
        self.misses.invalidate(public_key).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct FakeStore {
        answer: Mutex<Result<Option<i64>, ()>>,
        calls: AtomicU64,
    }

    #[async_trait::async_trait()]
    impl ConversionStore for FakeStore {
        async fn get_conversion(&self, public_key: &str) -> Result<Option<Conversion>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let answer = *self.answer.lock().unwrap();
            match answer {
                Err(()) => anyhow::bail!("store down"),
                Ok(version) => Ok(version.map(|version| Conversion {
                    public_key: public_key.into(),
                    conversion_id: "c".into(),
                    workspace_id: "w".into(),
                    version,
                    active: true,
                    secret_hash: String::new(),
                    currency: "EUR".into(),
                    window_days: 30,
                    counting: "Every".into(),
                    allowed_origins: vec![],
                    server_values_only: false,
                    steps: vec![],
                    route_filter: None,
                })),
            }
        }
    }

    impl FakeStore {
        fn set(&self, answer: Result<Option<i64>, ()>) {
            *self.answer.lock().unwrap() = answer;
        }

        fn calls(&self) -> u64 {
            self.calls.load(Ordering::SeqCst)
        }
    }

    fn setup(answer: Result<Option<i64>, ()>) -> (Arc<FakeStore>, MokaConversionCache) {
        let store = Arc::new(FakeStore {
            answer: Mutex::new(answer),
            calls: AtomicU64::new(0),
        });
        let settings = ConversionCacheSettings {
            max_capacity: 10,
            time_to_live_seconds: 1,
            miss_max_capacity: 10,
            miss_time_to_live_seconds: 1,
        };
        (store.clone(), MokaConversionCache::new(store, &settings))
    }

    async fn version(cache: &MokaConversionCache) -> Option<i64> {
        cache.get("pk_a").await.unwrap().map(|c| c.version)
    }

    #[tokio::test]
    async fn caches_until_invalidated() {
        let (store, cache) = setup(Ok(Some(1)));
        assert_eq!(version(&cache).await, Some(1));
        store.set(Ok(Some(2)));
        assert_eq!(version(&cache).await, Some(1));
        assert_eq!(store.calls(), 1);
        cache.invalidate("pk_a").await;
        assert_eq!(version(&cache).await, Some(2));
    }

    #[tokio::test]
    async fn entries_and_misses_expire_without_a_notification() {
        let (store, cache) = setup(Ok(None));
        assert_eq!(version(&cache).await, None);
        store.set(Ok(Some(1)));
        assert_eq!(version(&cache).await, None);
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(version(&cache).await, Some(1));
        store.set(Ok(Some(2)));
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(version(&cache).await, Some(2));
    }

    #[tokio::test]
    async fn store_errors_are_returned_and_not_cached() {
        let (store, cache) = setup(Err(()));
        assert!(cache.get("pk_a").await.is_err());
        store.set(Ok(Some(1)));
        assert_eq!(version(&cache).await, Some(1));
    }

    #[tokio::test]
    async fn unknown_keys_are_bounded_and_keep_real_entries() {
        let (store, cache) = setup(Ok(Some(1)));
        version(&cache).await;
        store.set(Ok(None));
        for i in 0..100 {
            assert!(cache.get(&format!("pk_x{i}")).await.unwrap().is_none());
        }
        cache.misses.run_pending_tasks().await;
        assert!(cache.misses.entry_count() <= 10);
        assert_eq!(version(&cache).await, Some(1));

        let calls = store.calls();
        assert!(cache.get(&"x".repeat(65)).await.unwrap().is_none());
        assert_eq!(store.calls(), calls);
    }
}
