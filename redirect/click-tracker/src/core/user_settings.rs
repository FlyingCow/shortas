use anyhow::Result;

use super::UserSettings;

use crate::adapters::UserSettingsCacheType;

#[async_trait::async_trait]
pub trait UserSettingsStore {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>>;
}

#[async_trait::async_trait]
pub trait UserSettingsCache {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>>;
    async fn invalidate(&self, user_id: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct UserSettingsManager {
    user_settings_cache: UserSettingsCacheType,
}

impl UserSettingsManager {
    pub fn new(user_settings_cache: UserSettingsCacheType) -> Self {
        Self {
            user_settings_cache,
        }
    }

    pub async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        self.user_settings_cache.get_user_settings(user_id).await
    }
}
