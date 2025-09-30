use crate::core::{CryptoStore, RoutesStore, UserSettingsStore};

#[derive(Clone)]
pub struct AppState {
    pub routes_store: Box<dyn RoutesStore + Send + Sync>,
    pub crypto_store: Box<dyn CryptoStore + Send + Sync>,
    pub user_settings_store: Box<dyn UserSettingsStore + Send + Sync>,
}

impl AppState {
    pub fn new(
        routes_store: Box<dyn RoutesStore + Send + Sync>,
        crypto_store: Box<dyn CryptoStore + Send + Sync>,
        user_settings_store: Box<dyn UserSettingsStore + Send + Sync>,
    ) -> Self {
        AppState {
            routes_store,
            crypto_store,
            user_settings_store,
        }
    }
}
