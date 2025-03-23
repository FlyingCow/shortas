use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct UserSettingsCache {
    pub max_capacity: u64, 
    pub time_to_live_minutes: u64, 
    pub time_to_idle_minutes: u64
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Moka {
    pub user_settings_cache: UserSettingsCache,
}