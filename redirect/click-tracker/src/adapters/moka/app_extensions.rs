use tracing::info;

use crate::{
    adapters::moka::user_settings_store::MokaDecoratedUserSettingsStore, app::AppBuilder,
};

impl AppBuilder {
    pub fn with_moka(&mut self) -> &mut Self {
        info!("{}", "WITH MOKA CACHE");


        let user_settings_store = Some(Box::new(MokaDecoratedUserSettingsStore::new(
            self.user_settings_store.clone().unwrap(),
            self.settings.moka.user_settings_cache.max_capacity,
            self.settings.moka.user_settings_cache.time_to_live_minutes,
            self.settings.moka.user_settings_cache.time_to_idle_minutes,
        ))as Box<_>);

        self.user_settings_store = user_settings_store;
        
        self
    }
}
