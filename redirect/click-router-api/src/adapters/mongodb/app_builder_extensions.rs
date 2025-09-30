use mongodb::Client;
use tracing::info;

use crate::{
    adapters::mongodb::{
        crypto_store::MongodbCryptoStore, routes_store::MongodbRoutesStore,
        user_settings_store::MongodbUserSettingsStore,
    },
    app_builder::AppBuilder,
};

impl AppBuilder {
    pub async fn with_mongodb(&mut self) -> &mut Self {
        info!("{}", "WITH MONGODB PROVIDERS");

        let client = Client::with_uri_str(&self.settings.mongodb.connection_string)
            .await
            .expect("Failed to connect to MongoDB");

        let database = client.database(&self.settings.mongodb.database_name);

        let routes_store = Some(Box::new(MongodbRoutesStore::new(
            &database,
            &self.settings.mongodb.routes_collection,
        )) as Box<_>);

        let crypto_store = Some(Box::new(MongodbCryptoStore::new(
            &database,
            &self.settings.mongodb.crypto_collection,
        )) as Box<_>);

        let user_settings_store = Some(Box::new(MongodbUserSettingsStore::new(
            &database,
            &self.settings.mongodb.user_settings_collection,
        )) as Box<_>);

        self.routes_store = routes_store;
        self.crypto_store = crypto_store;
        self.user_settings_store = user_settings_store;

        self
    }
}
