use anyhow::Result;

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::BaseUserSettingsStore;
use crate::model::UserSettings;

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

#[async_trait::async_trait()]
impl BaseUserSettingsStore for MongodbUserSettingsStore {
    async fn store_user_settings(&self, _user_settings: &UserSettings) -> Result<()> {
        todo!()
    }
    async fn update_user_settings(&self, _user_settings: &UserSettings) -> Result<()> {
        todo!()
    }
    async fn delete_user_settings(&self, _user_settings: &UserSettings) -> Result<()> {
        todo!()
    }

    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let target_object_id = ObjectId::parse_str(user_id)?;

        let filter = doc! { "_id": target_object_id };
        Ok(self.collection.find_one(filter).await?)
    }
    async fn invalidate_user_settings(&self, _: &str) -> Result<()> {
        Ok(())
    }
}
