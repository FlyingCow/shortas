use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use std::time::Duration;

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::CryptoStore;
use crate::model::Keycert;

#[derive(Clone, Debug)]
pub struct MongodbCryptoStore {
    collection: Collection<Keycert>,
}

impl MongodbCryptoStore {
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
