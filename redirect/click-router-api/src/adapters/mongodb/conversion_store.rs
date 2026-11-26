use anyhow::Result;
use mongodb::bson::{doc, to_bson};
use mongodb::error::{Error, ErrorKind, WriteFailure};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::core::ConversionStore;
use crate::model::Conversion;

const DUPLICATE_KEY: i32 = 11000;

#[derive(Clone, Debug)]
pub struct MongodbConversionStore {
    collection: Collection<ConversionDocument>,
}

/// Keyed by public key, so there is never a second copy.
#[derive(Serialize, Deserialize, Debug)]
struct ConversionDocument {
    #[serde(rename = "_id")]
    public_key: String,
    conversion: Conversion,
}

impl MongodbConversionStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection(collection_name),
        }
    }
}

fn is_duplicate_key(error: &Error) -> bool {
    matches!(
        error.kind.as_ref(),
        ErrorKind::Write(WriteFailure::WriteError(e)) if e.code == DUPLICATE_KEY
    )
}

#[async_trait::async_trait()]
impl ConversionStore for MongodbConversionStore {
    async fn put_conversion(&self, conversion: &Conversion) -> Result<bool> {
        let update = || -> Result<_> {
            Ok(self
                .collection
                .update_one(
                    doc! {
                        "_id": &conversion.public_key,
                        "conversion.version": { "$lt": conversion.version },
                    },
                    doc! { "$set": { "conversion": to_bson(conversion)? } },
                )
                .upsert(true))
        };

        // With a newer version stored the filter misses and the upsert hits
        // the existing `_id`. A concurrent first write does the same, so try
        // once more against the row it created.
        let result = match update()?.await {
            Err(e) if is_duplicate_key(&e) => update()?.await,
            result => result,
        };
        match result {
            Ok(_) => Ok(true),
            Err(e) if is_duplicate_key(&e) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

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
    use crate::core::conversion_store::tests::*;

    /// Run with `TEST_MONGODB_URI=mongodb://... cargo test -- --ignored`.
    #[tokio::test]
    #[ignore = "needs MongoDB"]
    async fn older_versions_are_ignored() {
        let uri = std::env::var("TEST_MONGODB_URI").unwrap();
        let client = mongodb::Client::with_uri_str(uri).await.unwrap();
        let database = client.database("shortas_conversion_test");
        database.drop().await.unwrap();
        let store = MongodbConversionStore::new(&database, "conversions");
        concurrent_writes_keep_the_newest(&store).await;
        database.drop().await.unwrap();
        older_versions_never_replace_newer(&store).await;
        database.drop().await.unwrap();
    }
}
