use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use std::time::Duration;

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::UserSettingsStore;
use crate::model::UserSettings;

#[derive(Clone, Debug)]
pub struct MongodbUserSettingsStore {
    collection: Collection<UserSettings>,
}

impl MongodbUserSettingsStore {
    pub async fn new(settings: &Mongodb) -> Self {
        // Parse URI and configure connection pool
        let mut client_options = ClientOptions::parse(&settings.uri)
            .await
            .expect("Failed to parse MongoDB URI");

        // Configure connection pool for optimal performance
        client_options.min_pool_size = Some(50);
        client_options.max_pool_size = Some(200);
        client_options.max_idle_time = Some(Duration::from_secs(300));  // 5 minutes
        client_options.connect_timeout = Some(Duration::from_secs(5));
        client_options.server_selection_timeout = Some(Duration::from_secs(5));

        let client = Client::with_options(client_options)
            .expect("Failed to create MongoDB client");

        let db = client.database(&settings.database);
        let collection = db.collection::<UserSettings>(&settings.user_settings_collection);
        Self { collection }
    }
}

#[async_trait::async_trait()]
impl UserSettingsStore for MongodbUserSettingsStore {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let filter = doc! { "user_id": user_id };

        Ok(match self.collection.find_one(filter).await? {
            Some(settings) => Some(settings),
            None => None,
        })
    }
}
