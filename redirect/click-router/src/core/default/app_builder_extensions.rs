use crate::{
    app_builder::AppBuilder,
    core::default::{
        default_crypto_manager::DefaultCryptoManager, default_routes_manager::DefaultRoutesManager, default_user_settings_manager::DefaultUserSettingsManager
    }, flow_router::modules::{not_found_module::NotFoundModule, root_module::RootModule},
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

        self.modules.push(Box::new(RootModule{}));
        self.modules.push(Box::new(NotFoundModule{}));


        println!("{}", "WITH DEFAULT MANAGERS");

        self
    }
}
