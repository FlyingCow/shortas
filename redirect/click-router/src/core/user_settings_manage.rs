use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::model::UserSettings;

#[async_trait::async_trait(?Send)]
pub trait BaseUserSettingsManager: DynClone {
    async fn get_user_settings(
        &self,
        user_id: &str,
    ) -> Result<Option<UserSettings>>;
}
clone_trait_object!(BaseUserSettingsManager);
