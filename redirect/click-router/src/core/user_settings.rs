use std::sync::Arc;

use anyhow::Result;

use crate::adapters::UserSettingsCacheType;

use crate::model::UserSettings;

#[async_trait::async_trait()]
pub trait UserSettingsStore {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>>;
}

#[async_trait::async_trait()]
pub trait UserSettingsCache {
    /// Returns an `Arc<UserSettings>` so a cache hit only bumps a refcount
    /// instead of deep-cloning the settings struct.
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<Arc<UserSettings>>>;
    async fn invalidate(&self, user_id: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct UserSettingsManager {
    user_settings_cache: UserSettingsCacheType,
}

impl UserSettingsManager {
    pub async fn get_user_settings(&self, user_id: &str) -> Result<Option<Arc<UserSettings>>> {
        let user_settings_result = self.user_settings_cache.get_user_settings(user_id).await;

        Ok(user_settings_result.unwrap())
    }
}

impl UserSettingsManager {
    pub fn new(user_settings_cache: UserSettingsCacheType) -> Self {
        Self {
            user_settings_cache,
        }
    }
}
