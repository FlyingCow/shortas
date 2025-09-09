use tracing::info;

use crate::{
    adapters::mongodb::{
        mongodb_crypto_store::MongodbCryptoStore, mongodb_routes_store::MongodbRoutesStore,
        mongodb_user_settings_store::MongodbUserSettingsStore,
    },
    app_builder::AppBuilder,
};

impl AppBuilder {
    pub async fn with_mongodb(&mut self) -> &mut Self {
        info!("{}", "WITH MONGODB PROVIDERS");

        let routes_store =
            Some(Box::new(MongodbRoutesStore::new(&self.settings.mongodb).await) as Box<_>);

        let crypto_store =
            Some(Box::new(MongodbCryptoStore::new(&self.settings.mongodb).await) as Box<_>);

        let user_settings_store =
            Some(Box::new(MongodbUserSettingsStore::new(&self.settings.mongodb).await) as Box<_>);

        self.routes_store = routes_store;
        self.crypto_store = crypto_store;
        self.user_settings_store = user_settings_store;

        self
    }
}
