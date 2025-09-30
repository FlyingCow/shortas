use anyhow::Result;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::core::UserSettingsStore;
use crate::model::{ActiveStatus, UserSettings};

#[derive(Clone, Debug)]
pub struct MongodbUserSettingsStore {
    collection: Collection<UserSettingsDocument>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct UserSettingsDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub user_email: String,
    pub api_key: Option<String>,
    #[serde(default)]
    pub active_status: ActiveStatus,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub overflow: bool,
    #[serde(default)]
    pub skip: Vec<String>,
    #[serde(default)]
    pub allowed_request_params: Vec<String>,
    #[serde(default)]
    pub allowed_destination_params: Vec<String>,
}

impl MongodbUserSettingsStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<UserSettingsDocument>(collection_name),
        }
    }
}

#[async_trait::async_trait()]
impl UserSettingsStore for MongodbUserSettingsStore {
    async fn store_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        let doc = UserSettingsDocument {
            user_id: user_settings.user_id.clone(),
            user_email: user_settings.user_email.clone(),
            api_key: user_settings.api_key.clone(),
            active_status: user_settings.active_status.clone(),
            debug: user_settings.debug,
            overflow: user_settings.overflow,
            skip: user_settings.skip.clone(),
            allowed_request_params: user_settings.allowed_request_params.clone(),
            allowed_destination_params: user_settings.allowed_destination_params.clone(),
            ..Default::default()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        let filter = doc! { "user_id": &user_settings.user_id };

        let doc = UserSettingsDocument {
            user_id: user_settings.user_id.clone(),
            user_email: user_settings.user_email.clone(),
            api_key: user_settings.api_key.clone(),
            active_status: user_settings.active_status.clone(),
            debug: user_settings.debug,
            overflow: user_settings.overflow,
            skip: user_settings.skip.clone(),
            allowed_request_params: user_settings.allowed_request_params.clone(),
            allowed_destination_params: user_settings.allowed_destination_params.clone(),
            ..Default::default()
        };

        self.collection.replace_one(filter, doc).await?;
        Ok(())
    }

    async fn delete_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        let filter = doc! { "user_id": &user_settings.user_id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let filter = doc! { "user_id": &user_id };
        match self.collection.find_one(filter).await? {
            Some(doc) => {
                let user_settings = UserSettings {
                    user_id: doc.user_id.clone(),
                    user_email: doc.user_email.clone(),
                    api_key: doc.api_key.clone(),
                    active_status: doc.active_status.clone(),
                    debug: doc.debug,
                    overflow: doc.overflow,
                    skip: doc.skip.clone(),
                    allowed_request_params: doc.allowed_request_params.clone(),
                    allowed_destination_params: doc.allowed_destination_params.clone(),
                    ..Default::default()
                };

                Ok(Some(user_settings))
            }
            None => Ok(None),
        }
    }

    async fn invalidate_user_settings(&self, _user_id: &str) -> Result<()> {
        // MongoDB doesn't need explicit invalidation like DynamoDB
        Ok(())
    }
}
