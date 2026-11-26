use anyhow::Result;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use serde::Deserialize;
use std::time::Duration;

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::conversion_definitions::ConversionStore;
use crate::model::Conversion;

#[derive(Deserialize, Debug)]
struct ConversionDocument {
    conversion: Conversion,
}

#[derive(Clone, Debug)]
pub struct MongodbConversionStore {
    collection: Collection<ConversionDocument>,
}

impl MongodbConversionStore {
    pub async fn new(settings: &Mongodb) -> Self {
        let mut client_options = ClientOptions::parse(&settings.uri)
            .await
            .expect("Failed to parse MongoDB URI");
        // Separate from the redirect pools, and small, so conversion lookups
        // can't take connections from redirects.
        client_options.max_pool_size = Some(20);
        client_options.connect_timeout = Some(Duration::from_secs(5));
        client_options.server_selection_timeout = Some(Duration::from_secs(5));

        let client =
            Client::with_options(client_options).expect("Failed to create MongoDB client");
        Self {
            collection: client
                .database(&settings.database)
                .collection(&settings.conversions_collection),
        }
    }
}

#[async_trait::async_trait()]
impl ConversionStore for MongodbConversionStore {
    async fn get_conversion(&self, public_key: &str) -> Result<Option<Conversion>> {
        Ok(self
            .collection
            .find_one(doc! { "_id": public_key })
            .await?
            .map(|d| d.conversion))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reads a document in the form click-router-api writes it.
    #[tokio::test]
    #[ignore = "needs MongoDB"]
    async fn reads_stored_definitions() {
        let settings = Mongodb {
            uri: std::env::var("TEST_MONGODB_URI").unwrap(),
            database: "shortas_router_conversion_test".into(),
            conversions_collection: "conversions".into(),
            ..Default::default()
        };
        let store = MongodbConversionStore::new(&settings).await;
        let raw = store.collection.clone_with_type::<mongodb::bson::Document>();
        raw.drop().await.unwrap();
        raw.insert_one(doc! { "_id": "pk_a", "conversion": {
            "publicKey": "pk_a", "conversionId": "c", "workspaceId": "w", "version": 1_i64,
            "active": true, "secretHash": "h", "currency": "EUR", "windowDays": 30_i64,
            "counting": "Every", "allowedOrigins": [], "serverValuesOnly": false,
            "steps": [{ "key": "s", "order": 0_i64, "isFinal": true, "defaultValue": null }],
            "routeFilter": null,
        }})
        .await
        .unwrap();

        let conversion = store.get_conversion("pk_a").await.unwrap().unwrap();
        assert_eq!(conversion.window_days, 30);
        assert_eq!(store.get_conversion("pk_b").await.unwrap(), None);
        raw.drop().await.unwrap();
    }
}
