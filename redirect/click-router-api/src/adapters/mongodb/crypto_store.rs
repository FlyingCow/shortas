use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use x509_parser::prelude::*;

use crate::core::CryptoStore;
use crate::model::{CertificateInfo, Keycert};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl MongodbCryptoStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<CryptoDocument>(collection_name),
        }
    }

    /// Extract expiry date from PEM certificate
    fn extract_expiry_from_cert(cert_pem: &[u8]) -> Option<DateTime<Utc>> {
        let pem_str = std::str::from_utf8(cert_pem).ok()?;
        let (_, pem) = x509_parser::pem::parse_x509_pem(pem_str.as_bytes()).ok()?;
        let (_, cert) = X509Certificate::from_der(&pem.contents).ok()?;
        let not_after = cert.validity().not_after;
        DateTime::from_timestamp(not_after.timestamp(), 0)
    }
}

#[async_trait::async_trait()]
impl CryptoStore for MongodbCryptoStore {
    async fn store_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        let expires_at = Self::extract_expiry_from_cert(&certificate.cert);

        let doc = CryptoDocument {
            hostname: hostname.to_string(),
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            owner_id: None,
            expires_at,
            ..Default::default()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn store_certificate_with_owner(
        &self,
        hostname: &str,
        certificate: &Keycert,
        owner_id: Option<&str>,
    ) -> Result<()> {
        let expires_at = Self::extract_expiry_from_cert(&certificate.cert);

        let doc = CryptoDocument {
            hostname: hostname.to_string(),
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            owner_id: owner_id.map(|s| s.to_string()),
            expires_at,
            ..Default::default()
        };

        self.collection.insert_one(doc).await?;
        Ok(())
    }

    async fn update_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        let expires_at = Self::extract_expiry_from_cert(&certificate.cert);

        let doc = CryptoDocument {
            hostname: hostname.to_string(),
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            owner_id: None,
            expires_at,
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

    async fn get_certificates_expiring_before(
        &self,
        before: DateTime<Utc>,
    ) -> Result<Vec<CertificateInfo>> {
        let bson_before = bson::DateTime::from_millis(before.timestamp_millis());
        let filter = doc! {
            "expires_at": {
                "$lt": bson_before,
                "$ne": null
            }
        };

        let mut cursor = self.collection.find(filter).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(CertificateInfo {
                domain: doc.hostname,
                owner_id: doc.owner_id,
                expires_at: doc.expires_at,
            });
        }

        Ok(results)
    }
}
