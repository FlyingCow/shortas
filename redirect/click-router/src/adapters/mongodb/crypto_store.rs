use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{Client, Collection};

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::CryptoStore;
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
impl CryptoStore for MongodbCryptoStore {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let filter = doc! { "hostname": server_name };

        Ok(match self.collection.find_one(filter).await? {
            Some(cert) => Some(cert),
            None => None,
        })
    }
}
