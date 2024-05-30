pub mod crypto_store;
pub mod routes_store;
pub mod user_settings_store;

pub mod crypto_manage;
pub mod flow_router;
pub mod routes_manage;
pub mod user_settings_manage;

pub mod location_detect;
pub mod user_agent_detect;

pub mod default;

pub use crypto_store::BaseCryptoStore;
pub use routes_store::BaseRoutesStore;
pub use user_settings_store::BaseUserSettingsStore;

pub use crypto_manage::BaseCryptoManager;
pub use routes_manage::BaseRoutesManager;
pub use user_settings_manage::BaseUserSettingsManager;

pub use flow_router::BaseFlowRouter;

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

    pub fn init_with(&mut self, value: T) {
        self.loaded = true;
        self.value = value;
    }

    pub fn has_value(&self) -> bool {
        self.loaded
    }

    pub fn get_value(self) -> T {
        self.value
    }
}
