use anyhow::Result;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection, Database, IndexModel};
use mongodb::options::IndexOptions;
use serde::{Deserialize, Serialize};

use crate::core::ChallengeStore;
use crate::model::challenge::Challenge;

#[derive(Clone, Debug)]
pub struct MongodbChallengeStore {
    collection: Collection<ChallengeDocument>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct ChallengeDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub domain: String,
    pub token: String,
    pub key_authorization: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}

impl MongodbChallengeStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<ChallengeDocument>(collection_name),
        }
    }

    /// Create indexes for the challenges collection
    /// Should be called once during application startup
    pub async fn create_indexes(&self) -> Result<()> {
        // Compound unique index on domain + token
        let compound_index = IndexModel::builder()
            .keys(doc! { "domain": 1, "token": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        // TTL index on expires_at for automatic cleanup
        let ttl_index = IndexModel::builder()
            .keys(doc! { "expires_at": 1 })
            .options(IndexOptions::builder().expire_after(std::time::Duration::from_secs(0)).build())
            .build();

        self.collection.create_indexes([compound_index, ttl_index]).await?;
        Ok(())
    }
}

#[async_trait::async_trait()]
impl ChallengeStore for MongodbChallengeStore {
    async fn store_challenge(
        &self,
        domain: &str,
        token: &str,
        key_authorization: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        let doc = ChallengeDocument {
            id: None,
            domain: domain.to_lowercase(),
            token: token.to_string(),
            key_authorization: key_authorization.to_string(),
            expires_at,
        };

        // Use upsert to handle both create and update
        let filter = doc! {
            "domain": &doc.domain,
            "token": &doc.token,
        };

        let update = doc! {
            "$set": {
                "domain": &doc.domain,
                "token": &doc.token,
                "key_authorization": &doc.key_authorization,
                "expires_at": doc.expires_at.timestamp_millis(),
            }
        };

        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        self.collection.update_one(filter, update).with_options(options).await?;
        Ok(())
    }

    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>> {
        let filter = doc! {
            "domain": domain.to_lowercase(),
            "token": token,
        };

        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(Challenge {
                domain: doc.domain,
                token: doc.token,
                key_authorization: doc.key_authorization,
                expires_at: doc.expires_at,
            })),
            None => Ok(None),
        }
    }

    async fn delete_challenge(&self, domain: &str, token: &str) -> Result<()> {
        let filter = doc! {
            "domain": domain.to_lowercase(),
            "token": token,
        };

        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn delete_domain_challenges(&self, domain: &str) -> Result<u64> {
        let filter = doc! {
            "domain": domain.to_lowercase(),
        };

        let result = self.collection.delete_many(filter).await?;
        Ok(result.deleted_count)
    }

    async fn cleanup_expired(&self) -> Result<u64> {
        let now = Utc::now();
        let filter = doc! {
            "expires_at": { "$lt": now.timestamp_millis() }
        };

        let result = self.collection.delete_many(filter).await?;
        Ok(result.deleted_count)
    }
}
