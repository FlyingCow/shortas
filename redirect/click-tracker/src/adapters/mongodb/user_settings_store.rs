use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::{UserSettings, UserSettingsStore};

#[derive(Clone, Debug)]
pub struct MongodbUserSettingsStore {
    collection: Collection<UserSettings>,
}

impl MongodbUserSettingsStore {
    pub async fn new(settings: &Mongodb) -> Self {
        let client = Client::with_uri_str(&settings.uri)
            .await
            .expect("can not connect to mongodb");

        let db = client.database(&settings.database);
        let collection = db.collection::<UserSettings>(&settings.user_settings_collection);
        Self { collection }
    }
}

#[async_trait::async_trait]
impl UserSettingsStore for MongodbUserSettingsStore {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let filter = doc! { "user_id": user_id };

        Ok(self.collection.find_one(filter).await?)
    }
}
