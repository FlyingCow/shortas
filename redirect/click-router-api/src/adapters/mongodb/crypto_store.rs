use anyhow::Result;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::core::CryptoStore;
use crate::model::Keycert;

#[derive(Clone, Debug)]
pub struct MongodbCryptoStore {
    collection: Collection<CryptoDocument>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct CryptoDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub hostname: String,
    pub key: Vec<u8>,
    pub cert: Vec<u8>,
    pub ocsp_resp: Vec<u8>,
}

impl MongodbCryptoStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<CryptoDocument>(collection_name),
        }
    }
}

#[async_trait::async_trait()]
impl CryptoStore for MongodbCryptoStore {
    async fn store_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        let doc = CryptoDocument {
            hostname: hostname.to_string(), // This should be passed as parameter
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            ..Default::default()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        let doc = CryptoDocument {
            hostname: hostname.to_string(), // This should be passed as parameter
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            ..Default::default()
        };

        self.collection
            .replace_one(doc! { "hostname": &doc.hostname }, doc)
            .await?;
        Ok(())
    }

    async fn delete_certificate(&self, hostname: &str) -> Result<()> {
        // This should use hostname from certificate
        self.collection
            .delete_one(doc! { "hostname": hostname })
            .await?;
        Ok(())
    }

    async fn get_certificate(&self, hostname: &str) -> Result<Option<Keycert>> {
        let filter = doc! { "hostname": hostname.to_string() };

        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(Keycert {
                key: doc.key,
                cert: doc.cert,
                ocsp_resp: doc.ocsp_resp,
            })),
            None => Ok(None),
        }
    }

    async fn invalidate_certificate(&self, _server_name: &str) -> Result<()> {
        Ok(())
    }
}
