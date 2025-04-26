pub mod crypto;

pub mod routes;
pub mod user_settings;

pub mod expression;
pub mod flow_module;
pub mod flow_router;
pub mod host;
pub mod ip;
pub mod language;
pub mod modules;
pub mod protocol;
pub mod user_agent;
pub mod user_agent_string;

pub mod hits_register;
pub mod location;

pub use crypto::CryptoStore;
pub use routes::RoutesStore;
pub use user_settings::UserSettingsStore;

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
