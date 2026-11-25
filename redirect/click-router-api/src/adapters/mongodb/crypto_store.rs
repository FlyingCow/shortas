use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{self, oid::ObjectId, Bson};
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
    /// A BSON date. Rows written by older versions hold RFC 3339 text.
    pub expires_at: Option<Bson>,
}

fn to_bson_date(date: DateTime<Utc>) -> Bson {
    Bson::DateTime(bson::DateTime::from_millis(date.timestamp_millis()))
}

fn parse_expiry(value: &Bson) -> Option<DateTime<Utc>> {
    match value {
        Bson::DateTime(date) => DateTime::from_timestamp_millis(date.timestamp_millis()),
        Bson::String(text) => DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|date| date.with_timezone(&Utc)),
        _ => None,
    }
}

impl MongodbCryptoStore {
    pub fn new(database: &Database, collection_name: &str) -> Self {
        Self {
            collection: database.collection::<CryptoDocument>(collection_name),
        }
    }

    /// Replaces the domain's certificate, or adds it. An existing owner is kept
    /// unless a new one is given.
    async fn replace(
        &self,
        hostname: &str,
        certificate: &Keycert,
        owner_id: Option<&str>,
    ) -> Result<()> {
        let hostname = hostname.to_ascii_lowercase();
        let document = CryptoDocument {
            id: None,
            hostname: hostname.clone(),
            key: certificate.key.clone(),
            cert: certificate.cert.clone(),
            ocsp_resp: certificate.ocsp_resp.clone(),
            owner_id: owner_id.map(str::to_string),
            expires_at: Self::extract_expiry_from_cert(&certificate.cert).map(to_bson_date),
        };

        self.collection
            .update_one(
                doc! { "hostname": hostname },
                doc! { "$set": bson::to_document(&document)? },
            )
            .upsert(true)
            .await?;
        Ok(())
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
        self.replace(hostname, certificate, None).await
    }

    async fn store_certificate_with_owner(
        &self,
        hostname: &str,
        certificate: &Keycert,
        owner_id: Option<&str>,
    ) -> Result<()> {
        self.replace(hostname, certificate, owner_id).await
    }

    async fn update_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        self.replace(hostname, certificate, None).await
    }

    async fn delete_certificate(&self, hostname: &str) -> Result<()> {
        // Older renewals may have left several rows for one domain.
        self.collection
            .delete_many(doc! { "hostname": hostname.to_ascii_lowercase() })
            .await?;
        Ok(())
    }

    async fn get_certificate(&self, hostname: &str) -> Result<Option<Keycert>> {
        let filter = doc! { "hostname": hostname.to_ascii_lowercase() };

        match self
            .collection
            .find_one(filter)
            .sort(doc! { "expires_at": -1 })
            .await?
        {
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
        // Judge each domain by its current certificate, so a leftover row from
        // an earlier renewal doesn't trigger another one. `$lt` only matches
        // values of its own type, hence one branch per stored format.
        let mut cursor = self
            .collection
            .aggregate([
                doc! { "$sort": { "expires_at": -1 } },
                doc! { "$group": {
                    "_id": "$hostname",
                    "owner_id": { "$first": "$owner_id" },
                    "expires_at": { "$first": "$expires_at" },
                } },
                doc! { "$match": { "$or": [
                    { "expires_at": { "$lt": to_bson_date(before) } },
                    { "expires_at": { "$lt": before.to_rfc3339() } },
                ] } },
            ])
            .await?;
        let mut results = Vec::new();

        while let Some(row) = cursor.try_next().await? {
            results.push(CertificateInfo {
                domain: row.get_str("_id")?.to_string(),
                owner_id: row.get_str("owner_id").ok().map(str::to_string),
                expires_at: row.get("expires_at").and_then(parse_expiry),
            });
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run with `TEST_MONGODB_URI=mongodb://... cargo test -- --ignored`.
    #[tokio::test]
    #[ignore = "needs MongoDB"]
    async fn storing_replaces_keeps_owner_and_handles_duplicates() {
        let uri = std::env::var("TEST_MONGODB_URI").unwrap();
        let client = mongodb::Client::with_uri_str(uri).await.unwrap();
        let database = client.database("shortas_certificate_test");
        database.drop().await.unwrap();
        let store = MongodbCryptoStore::new(&database, "certificates");
        let keycert = |cert: &str| Keycert {
            key: b"key".to_vec(),
            cert: cert.as_bytes().to_vec(),
            ocsp_resp: vec![],
        };

        store
            .store_certificate_with_owner("a.test", &keycert("old"), Some("owner"))
            .await
            .unwrap();
        store.store_certificate("A.test", &keycert("new")).await.unwrap();
        store.update_certificate("b.test", &keycert("added by update")).await.unwrap();

        let row = store.collection.find_one(doc! { "hostname": "a.test" }).await.unwrap().unwrap();
        assert_eq!(row.cert, b"new");
        assert_eq!(row.owner_id.as_deref(), Some("owner"));
        assert_eq!(store.collection.count_documents(doc! {}).await.unwrap(), 2);

        for (cert, days) in [("expiring", 1), ("latest", 90), ("middle", 30)] {
            store
                .collection
                .insert_one(CryptoDocument {
                    hostname: "dup.test".into(),
                    cert: cert.into(),
                    expires_at: Some(to_bson_date(Utc::now() + chrono::Duration::days(days))),
                    ..Default::default()
                })
                .await
                .unwrap();
        }
        let current = store.get_certificate("dup.test").await.unwrap().unwrap();
        assert_eq!(current.cert, b"latest");

        store.delete_certificate("dup.test").await.unwrap();
        assert!(store.get_certificate("dup.test").await.unwrap().is_none());
        database.drop().await.unwrap();
    }

    #[tokio::test]
    #[ignore = "needs MongoDB"]
    async fn lists_domains_whose_current_certificate_expires() {
        let uri = std::env::var("TEST_MONGODB_URI").unwrap();
        let client = mongodb::Client::with_uri_str(uri).await.unwrap();
        let database = client.database("shortas_expiry_test");
        database.drop().await.unwrap();
        let store = MongodbCryptoStore::new(&database, "certificates");

        let date = |days| to_bson_date(Utc::now() + chrono::Duration::days(days));
        let text = |days| {
            Bson::String(serde_json::to_value(Utc::now() + chrono::Duration::days(days)).unwrap().as_str().unwrap().into())
        };
        for (hostname, expires_at) in [
            ("date-soon.test", Some(date(5))),
            ("date-later.test", Some(date(80))),
            ("text-soon.test", Some(text(5))),
            ("text-later.test", Some(text(80))),
            ("renewed.test", Some(text(5))),
            ("renewed.test", Some(date(80))),
            ("twice-soon.test", Some(text(2))),
            ("twice-soon.test", Some(text(5))),
            ("unknown.test", None),
        ] {
            store
                .collection
                .insert_one(CryptoDocument {
                    hostname: hostname.into(),
                    owner_id: Some("owner".into()),
                    expires_at,
                    ..Default::default()
                })
                .await
                .unwrap();
        }

        // A real certificate stored through the API path is a date as well.
        let pem = include_bytes!("../../../../click-router/certs/cert.pem");
        store
            .store_certificate("stored.test", &Keycert { key: vec![], cert: pem.to_vec(), ocsp_resp: vec![] })
            .await
            .unwrap();
        let stored = store.collection.find_one(doc! { "hostname": "stored.test" }).await.unwrap().unwrap();
        assert!(matches!(stored.expires_at, Some(Bson::DateTime(_))));

        let mut listed = store
            .get_certificates_expiring_before(Utc::now() + chrono::Duration::days(30))
            .await
            .unwrap();
        listed.sort_by(|a, b| a.domain.cmp(&b.domain));
        let domains: Vec<_> = listed.iter().map(|c| c.domain.as_str()).collect();
        assert_eq!(domains, ["date-soon.test", "text-soon.test", "twice-soon.test"]);
        assert!(listed.iter().all(|c| c.expires_at.is_some() && c.owner_id.as_deref() == Some("owner")));

        database.drop().await.unwrap();
    }
}
