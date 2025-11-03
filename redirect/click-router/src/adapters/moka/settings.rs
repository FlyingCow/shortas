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
pub struct QrCodeCacheSettings {
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
    #[serde(default = "default_qr_code_cache")]
    pub qr_code_cache: QrCodeCacheSettings,
}

fn default_qr_code_cache() -> QrCodeCacheSettings {
    QrCodeCacheSettings {
        max_capacity: 10_000,        // 10k QR codes
        time_to_live_minutes: 1440,  // 24 hours
        time_to_idle_minutes: 60,    // 1 hour
    }
}
