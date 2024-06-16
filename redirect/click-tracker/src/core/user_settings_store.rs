use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::UserSettings;

#[async_trait::async_trait()]
pub trait BaseUserSettingsStore: DynClone {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>>;
    async fn invalidate(&self, user_id: &str) -> Result<()>;
}
clone_trait_object!(BaseUserSettingsStore);