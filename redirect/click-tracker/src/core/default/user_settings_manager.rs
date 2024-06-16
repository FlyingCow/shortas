use anyhow::Result;

use crate::core::{BaseUserSettingsManager, BaseUserSettingsStore};

use crate::model::UserSettings;

#[derive(Clone)]
pub struct DefaultUserSettingsManager {
    user_settings_store: Box<dyn BaseUserSettingsStore + Send + Sync>,
}

#[async_trait::async_trait()]
impl BaseUserSettingsManager for DefaultUserSettingsManager {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let user_settings_result = self.user_settings_store.get_user_settings(user_id).await;

        Ok(user_settings_result.unwrap())
    }
}

impl DefaultUserSettingsManager {
    pub fn new(user_settings_store: Box<dyn BaseUserSettingsStore + Send + Sync>) -> Self {
        Self {
            user_settings_store,
        }
    }
}
