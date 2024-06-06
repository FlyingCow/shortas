use tracing::info;

use crate::{
    app::AppBuilder,
    core::default::{
        crypto_manager::DefaultCryptoManager, routes_manager::DefaultRoutesManager,
        user_settings_manager::DefaultUserSettingsManager,
    },
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_defaults(&mut self) -> &mut Self {
        let routes_manager = Some(Box::new(DefaultRoutesManager::new(
            self.routes_store.clone().unwrap(),
        )) as Box<_>);

        let crypto_manager = Some(Box::new(DefaultCryptoManager::new(
            self.crypto_store.clone().unwrap(),
        )) as Box<_>);

        let user_settings_manager = Some(Box::new(DefaultUserSettingsManager::new(
            self.user_settings_store.clone().unwrap(),
        )) as Box<_>);

        self.routes_manager = routes_manager;
        self.crypto_manager = crypto_manager;
        self.user_settings_manager = user_settings_manager;

        info!("{}", "WITH DEFAULT MANAGERS");

        self
    }
}
