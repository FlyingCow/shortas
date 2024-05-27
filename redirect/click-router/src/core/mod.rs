pub mod base_crypto_store;
pub mod base_routes_store;
pub mod base_user_settings_store;

pub mod base_crypto_manager;
pub mod base_flow_router;
pub mod base_routes_manager;
pub mod base_user_settings_manager;

pub mod base_location_detector;
pub mod base_user_agent_detector;

pub mod default;

pub use base_crypto_store::BaseCryptoStore;
pub use base_routes_store::BaseRoutesStore;
pub use base_user_settings_store::BaseUserSettingsStore;

pub use base_crypto_manager::BaseCryptoManager;
pub use base_routes_manager::BaseRoutesManager;
pub use base_user_settings_manager::BaseUserSettingsManager;

pub use base_flow_router::BaseFlowRouter;

#[derive(Debug, Clone)]
pub struct InitOnce<T> {
    loaded: bool,
    value: T,
}

impl<T> InitOnce<T> {
    pub fn default(default: T) -> Self {
        Self {
            loaded: false,
            value: default,
        }
    }

    pub fn init_with(&self, value: T) -> Self {
        Self {
            loaded: true,
            value: value,
        }
    }

    pub fn has_value(&self) -> bool {
        self.loaded
    }

    pub fn get_value(self) -> T {
        self.value
    }
}
