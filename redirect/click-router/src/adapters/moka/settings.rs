use serde_derive::Deserialize;
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct CryptoCacheSettings {
    pub max_capacity: u64,
    pub time_to_live_minutes: u64,
    pub time_to_idle_minutes: u64,
}
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct RoutesCacheSettings {
    pub max_capacity: u64,
    pub time_to_live_minutes: u64,
    pub time_to_idle_minutes: u64,
}
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct UserSettingsCacheSettings {
    pub max_capacity: u64,
    pub time_to_live_minutes: u64,
    pub time_to_idle_minutes: u64,
}
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Moka {
    pub crypto_cache: CryptoCacheSettings,
    pub routes_cache: RoutesCacheSettings,
    pub user_settings_cache: UserSettingsCacheSettings,
}
