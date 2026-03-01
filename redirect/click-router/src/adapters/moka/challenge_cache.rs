use anyhow::Result;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

use crate::adapters::moka::settings::ChallengeCacheSettings;
use crate::adapters::ChallengeStoreType;
use crate::core::challenge::{Challenge, ChallengeCache, ChallengeStore};

#[derive(Clone)]
pub struct MokaChallengeCache {
    cache: Cache<String, Arc<Challenge>>,
    store: ChallengeStoreType,
}

impl MokaChallengeCache {
    pub fn new(store: ChallengeStoreType, settings: ChallengeCacheSettings) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(settings.time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(settings.time_to_idle_minutes * 60))
            .max_capacity(settings.max_capacity)
            .build();

        Self { cache, store }
    }

    fn cache_key(domain: &str, token: &str) -> String {
        format!("{}:{}", domain.to_lowercase(), token)
    }
}

#[async_trait::async_trait()]
impl ChallengeCache for MokaChallengeCache {
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>> {
        let key = Self::cache_key(domain, token);

        if let Some(challenge) = self.cache.get(&key).await {
            return Ok(Some((*challenge).clone()));
        }

        // Fetch from store
        match self.store.get_challenge(domain, token).await? {
            Some(challenge) => {
                let challenge_arc: Arc<Challenge> = Arc::new(challenge.clone());
                self.cache.insert(key, challenge_arc).await;
                Ok(Some(challenge))
            }
            None => Ok(None),
        }
    }

    async fn invalidate(&self, domain: &str, token: &str) -> Result<()> {
        let key = Self::cache_key(domain, token);
        self.cache.invalidate(&key).await;
        Ok(())
    }
}
