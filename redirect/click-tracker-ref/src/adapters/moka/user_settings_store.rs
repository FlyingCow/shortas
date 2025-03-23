use std::time::Duration;

use anyhow::Result;
use moka::future::Cache;

use crate::core::{UserSettings, UserSettingsStore};

const KEY_PREFIX: &'static str = "settings";

#[derive(Clone, Debug)]
pub struct UserSettingsCacheItem {
    value: Option<UserSettings>,
}

pub struct MokaDecoratedUserSettingsStore<S>
where
    S: UserSettingsStore + Send + Sync,
{
    cache: Cache<String, UserSettingsCacheItem>,
    user_settings_store: S,
}

impl<S> MokaDecoratedUserSettingsStore<S>
where
    S: UserSettingsStore + Send + Sync,
{
    pub fn new(
        user_settings_store: S,
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
            user_settings_store,
        }
    }
}

fn get_key(user_id: &str) -> String {
    let user_id_str = format!("{}_{}", KEY_PREFIX, user_id);

    user_id_str
}

#[async_trait::async_trait()]
impl<S> UserSettingsStore for MokaDecoratedUserSettingsStore<S>
where
    S: UserSettingsStore + Send + Sync,
{
    async fn get(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let key = get_key(user_id);

        let cache_result = self
            .cache
            .get_with(key, async move {
                let user_settings_result = self.user_settings_store.get(user_id).await;
                UserSettingsCacheItem {
                    value: user_settings_result.unwrap(),
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
