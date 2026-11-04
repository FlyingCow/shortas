use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::adapters::UserSettingsStoreType;
use crate::core::user_settings::UserSettingsCache;
use crate::core::UserSettingsStore;
use crate::model::UserSettings;

use super::settings::UserSettingsCacheSettings;

const KEY_PREFIX: &'static str = "settings";

#[derive(Clone)]
pub struct MokaUserSettingsCache {
    // `Option<Arc<UserSettings>>` so a cache hit is a refcount bump instead
    // of a deep clone of the settings struct.
    cache: Cache<String, Option<Arc<UserSettings>>>,
    user_settings_store: UserSettingsStoreType,
}

impl MokaUserSettingsCache {
    pub fn new(
        user_settings_store: UserSettingsStoreType,
        settings: UserSettingsCacheSettings,
    ) -> Self {
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
            user_settings_store,
        }
    }
}

fn get_key(user_id: &str) -> String {
    let user_id_str = format!("{}_{}", KEY_PREFIX, user_id);

    user_id_str
}

#[async_trait::async_trait()]
impl UserSettingsCache for MokaUserSettingsCache {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<Arc<UserSettings>>> {
        let key = get_key(user_id);

        // `get_with` also dedupes concurrent misses for the same key; since
        // the cached value is now `Option<Arc<UserSettings>>`, both the hit
        // and miss paths only ever clone an `Arc`, never the settings struct.
        let cache_result = self
            .cache
            .get_with(key, async move {
                let user_settings_result =
                    self.user_settings_store.get_user_settings(user_id).await;
                user_settings_result.unwrap().map(Arc::new)
            })
            .await;

        Ok(cache_result)
    }

    async fn invalidate(&self, user_id: &str) -> Result<()> {
        let key = get_key(user_id);
        self.cache.invalidate(&key).await;

        Ok(())
    }
}
