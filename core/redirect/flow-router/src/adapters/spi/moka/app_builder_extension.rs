use crate::adapters::spi::moka::crypto_cache::CryptoCache;
use crate::adapters::spi::moka::routes_cache::RoutesCache;
use crate::adapters::spi::moka::settings::Moka;

pub struct MokaBuilder {
    pub crypto_cache: CryptoCache,
    pub routes_cache: RoutesCache,
}

impl MokaBuilder {
    pub async fn new(moka_settings: Moka) -> Self {
        Self {
            crypto_cache: CryptoCache::new(
                moka_settings.crypto_cache.max_capacity,
                moka_settings.crypto_cache.time_to_live_minutes,
                moka_settings.crypto_cache.time_to_idle_minutes,
            ),
            routes_cache: RoutesCache::new(
                moka_settings.routes_cache.max_capacity,
                moka_settings.routes_cache.time_to_live_minutes,
                moka_settings.routes_cache.time_to_idle_minutes,
            ),
        }
    }
}
