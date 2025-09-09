use std::collections::HashMap;

use anyhow::Result;

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::BaseCryptoStore;
use crate::model::Keycert;

#[derive(Clone, Debug)]
pub struct MongodbCryptoStore {
    collection: Collection<Keycert>,
}

impl MongodbCryptoStore {
    pub async fn new(settings: &Mongodb) -> Self {
        let client = Client::with_uri_str(&settings.uri)
            .await
            .expect("can not connect to mongodb");

        let db = client.database(&settings.database);
        let collection = db.collection::<Keycert>(&settings.encryption_collection);
        Self { collection }
    }
}

#[async_trait::async_trait()]
impl BaseCryptoStore for MongodbCryptoStore {
    async fn store_certificate(&self, _certificate: &Keycert) -> Result<()> {
        todo!()
    }
    async fn update_certificate(&self, _certificate: &Keycert) -> Result<()> {
        todo!()
    }
    async fn delete_certificate(&self, _certificate: &Keycert) -> Result<()> {
        todo!()
    }

    async fn invalidate_certificate(&self, _: &str) -> Result<()> {
        Ok(())
    }

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let target_object_id = ObjectId::parse_str(server_name)?;

        let filter = doc! { "_id": target_object_id };
        Ok(self.collection.find_one(filter).await?)
    }
}
