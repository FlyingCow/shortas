use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct CryptoCacheSettings {
    pub max_capacity: u64,
    pub time_to_live_minutes: u64,
    pub time_to_idle_minutes: u64,
    /// How many names without a certificate to remember.
    #[serde(default = "default_miss_max_capacity")]
    pub miss_max_capacity: u64,
    /// How long to remember that a name has no certificate.
    #[serde(default = "default_miss_time_to_live_seconds")]
    pub miss_time_to_live_seconds: u64,
    /// Concurrent certificate store lookups.
    #[serde(default = "default_max_concurrent_lookups")]
    pub max_concurrent_lookups: usize,
}

fn default_miss_max_capacity() -> u64 {
    10_000
}

fn default_miss_time_to_live_seconds() -> u64 {
    30
}

fn default_max_concurrent_lookups() -> usize {
    64
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
pub struct ChallengeCacheSettings {
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
    #[serde(default = "default_challenge_cache")]
    pub challenge_cache: ChallengeCacheSettings,
}

fn default_qr_code_cache() -> QrCodeCacheSettings {
    QrCodeCacheSettings {
        max_capacity: 10_000,        // 10k QR codes
        time_to_live_minutes: 1440,  // 24 hours
        time_to_idle_minutes: 60,    // 1 hour
    }
}

fn default_challenge_cache() -> ChallengeCacheSettings {
    ChallengeCacheSettings {
        max_capacity: 10_000,        // 10k challenges
        time_to_live_minutes: 5,     // 5 minutes
        time_to_idle_minutes: 2,     // 2 minutes
    }
}
