pub mod user_settings_manage;

pub mod default;

pub mod click_aggs_register;
pub mod hit_stream;

pub mod location_detect;
pub mod user_agent_detect;
pub mod user_settings_store;

pub mod tracking_pipe;

pub use user_settings_manage::BaseUserSettingsManager;

pub use user_settings_store::BaseUserSettingsStore;
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
