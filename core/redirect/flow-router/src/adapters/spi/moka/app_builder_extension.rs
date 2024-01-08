use crate::adapters::spi::moka::crypto_cache::CryptoCache;
use crate::adapters::spi::moka::settings::Moka;

pub struct MokaBuilder {
    pub crypto_cache: CryptoCache,
}

impl MokaBuilder {
    pub async fn new(moka_settings: Moka) -> Self {
        Self {
            crypto_cache: CryptoCache::new(
                moka_settings.crypto_cache.max_capacity,
                moka_settings.crypto_cache.time_to_live_minutes,
                moka_settings.crypto_cache.time_to_idle_minutes,
            ),
        }
    }
}
