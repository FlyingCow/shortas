use crate::core::{BaseCryptoStore, BaseRoutesStore, BaseUserSettingsStore};

#[derive(Clone)]
pub struct AppState {
    pub routes_store: Box<dyn BaseRoutesStore + Send + Sync>,
    pub crypto_store: Box<dyn BaseCryptoStore + Send + Sync>,
    pub user_settings_store: Box<dyn BaseUserSettingsStore + Send + Sync>,
}

impl AppState {
    pub fn new(
        routes_store: Box<dyn BaseRoutesStore + Send + Sync>,
        crypto_store: Box<dyn BaseCryptoStore + Send + Sync>,
        user_settings_store: Box<dyn BaseUserSettingsStore + Send + Sync>,
    ) -> Self {
        AppState {
            routes_store,
            crypto_store,
            user_settings_store,
        }
    }
}
