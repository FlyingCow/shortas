use tracing::info;

use crate::{
    app::AppBuilder,
    core::default::user_settings_manager::DefaultUserSettingsManager,
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_tracking_defaults(&mut self) -> &mut Self {
        let user_settings_manager = Some(Box::new(DefaultUserSettingsManager::new(
            self.user_settings_store.clone().unwrap(),
        )) as Box<_>);

        self.user_settings_manager = user_settings_manager;

        info!("{}", "WITH DEFAULT MANAGERS");

        self
    }
}
