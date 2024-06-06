use tracing::info;

use crate::{
    adapters::moka::{crypto_store::MokaDecoratedCryptoStore, routes_store::MokaDecoratedRoutesStore, user_settings_store::MokaDecoratedUserSettingsStore}, app::AppBuilder,
};

impl AppBuilder {
    pub fn with_moka(&mut self) -> &mut Self {
        info!("{}", "WITH MOKA CACHE");

        let crypto_store = Some(Box::new(MokaDecoratedCryptoStore::new(
            self.crypto_store.clone().unwrap(),
            self.settings.moka.crypto_cache.max_capacity,
            self.settings.moka.crypto_cache.time_to_live_minutes,
            self.settings.moka.crypto_cache.time_to_idle_minutes,
        ))as Box<_>);

        let routes_store = Some(Box::new(MokaDecoratedRoutesStore::new(
            self.routes_store.clone().unwrap(),
            self.settings.moka.routes_cache.max_capacity,
            self.settings.moka.routes_cache.time_to_live_minutes,
            self.settings.moka.routes_cache.time_to_idle_minutes,
        ))as Box<_>);

        let user_settings_store = Some(Box::new(MokaDecoratedUserSettingsStore::new(
            self.user_settings_store.clone().unwrap(),
            self.settings.moka.user_settings_cache.max_capacity,
            self.settings.moka.user_settings_cache.time_to_live_minutes,
            self.settings.moka.user_settings_cache.time_to_idle_minutes,
        ))as Box<_>);

        self.crypto_store = crypto_store;
        self.routes_store = routes_store;
        self.user_settings_store = user_settings_store;
        
        self
    }
}
