use anyhow::Result;

use super::UserSettings;

#[async_trait::async_trait]
pub trait UserSettingsStore {
    async fn get(&self, user_id: &str) -> Result<Option<UserSettings>>;
    async fn invalidate(&self, user_id: &str) -> Result<()>;
}

#[async_trait::async_trait]
pub trait UserSettingsManager {
    async fn get(&self, user_id: &str) -> Result<Option<UserSettings>>;
}

pub struct DefaultUserSettingsManager<S>
where
    S: UserSettingsStore + Send + Sync,
{
    user_settings_store: S,
}

#[async_trait::async_trait]
impl<S> UserSettingsManager for DefaultUserSettingsManager<S>
where
    S: UserSettingsStore + Send + Sync,
{
    async fn get(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let user_settings_result = self.user_settings_store.get(user_id).await;

        user_settings_result
    }
}

impl<S> DefaultUserSettingsManager<S>
where
    S: UserSettingsStore + Send + Sync,
{
    pub fn new(user_settings_store: S) -> Self {
        Self {
            user_settings_store,
        }
    }
}
