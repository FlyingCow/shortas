use anyhow::Result;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, options::FindOptions, Collection, Database};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;

use crate::core::DomainStore;
use crate::model::{Domain, VerificationReason, VerificationStatus};

#[derive(Clone, Debug)]
pub struct MongodbDomainStore {
    collection: Collection<DomainDocument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DomainDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub mongo_id: Option<ObjectId>,
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub status: VerificationStatus,
    pub verification_reason: VerificationReason,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub last_check_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub next_check_at: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

impl From<&Domain> for DomainDocument {
    fn from(domain: &Domain) -> Self {
        Self {
            mongo_id: None,
            id: domain.id.clone(),
            name: domain.name.clone(),
            owner_id: domain.owner_id.clone(),
            status: domain.status.clone(),
            verification_reason: domain.verification_reason.clone(),
            last_check_at: domain.last_check_at,
            next_check_at: domain.next_check_at,
            created_at: domain.created_at,
        }
    }
}

impl From<DomainDocument> for Domain {
    fn from(doc: DomainDocument) -> Self {
        Self {
            id: doc.id,
            name: doc.name,
            owner_id: doc.owner_id,
            status: doc.status,
            verification_reason: doc.verification_reason,
            last_check_at: doc.last_check_at,
            next_check_at: doc.next_check_at,
            created_at: doc.created_at,
        }
    }
}

impl MongodbDomainStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<DomainDocument>(collection_name),
        }
    }
}

#[async_trait::async_trait()]
impl DomainStore for MongodbDomainStore {
    async fn store_domain(&self, domain: &Domain) -> Result<()> {
        let doc = DomainDocument::from(domain);
        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update_domain(&self, domain: &Domain) -> Result<()> {
        let filter = doc! { "id": &domain.id };
        let doc = DomainDocument::from(domain);
        self.collection.replace_one(filter, doc).await?;
        Ok(())
    }

    async fn delete_domain(&self, id: &str) -> Result<()> {
        let filter = doc! { "id": id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    async fn get_domain(&self, id: &str) -> Result<Option<Domain>> {
        let filter = doc! { "id": id };
        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(Domain::from(doc))),
            None => Ok(None),
        }
    }

    async fn get_domain_by_name(&self, name: &str, owner_id: &str) -> Result<Option<Domain>> {
        let filter = doc! { "name": name.to_lowercase(), "owner_id": owner_id };
        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(Domain::from(doc))),
            None => Ok(None),
        }
    }

    async fn list_domains(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Domain>, u64)> {
        let filter = match owner_id {
            Some(id) => doc! { "owner_id": id },
            None => doc! {},
        };

        let total_count = self.collection.count_documents(filter.clone()).await?;

        let skip = ((page - 1) * page_size) as u64;
        let options = FindOptions::builder()
            .skip(skip)
            .limit(page_size as i64)
            .sort(doc! { "name": 1 })
            .build();

        let mut cursor = self.collection.find(filter).with_options(options).await?;
        let mut domains = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            domains.push(Domain::from(doc));
        }

        Ok((domains, total_count))
    }

    async fn get_domains_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<Domain>> {
        let before_millis = before.timestamp_millis();
        let filter = doc! {
            "$or": [
                { "next_check_at": { "$lte": before_millis } },
                { "next_check_at": { "$exists": false } },
                { "next_check_at": null }
            ]
        };

        let options = FindOptions::builder()
            .limit(limit as i64)
            .sort(doc! { "next_check_at": 1 })
            .build();

        let mut cursor = self.collection.find(filter).with_options(options).await?;
        let mut domains = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            domains.push(Domain::from(doc));
        }

        Ok(domains)
    }
}
