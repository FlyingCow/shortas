use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::CryptoStore;
use crate::model::{CertificateInfo, Keycert};

#[derive(Clone, Debug)]
pub struct DynamoCryptoStore {
    client: Client,
    encryption_table: String,
}

impl DynamoCryptoStore {
    pub fn new(sdk_config: &SdkConfig, encryption_table: String) -> Self {
        Self {
            encryption_table,
            client: Client::new(sdk_config),
        }
    }

    fn to_entity(&self, model: GetItemOutput) -> Option<Keycert> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        model.item.map_or(None, |item| {
            let mut result = Keycert::new();

            if let Some(key) = item.get("key") {
                let key_str = key.as_s().unwrap();
                // Try to decode as base64, fallback to raw bytes
                let key_bytes = STANDARD.decode(key_str).unwrap_or_else(|_| key_str.as_bytes().to_vec());
                result = result.key(key_bytes);
            }

            if let Some(cert) = item.get("cert") {
                let cert_str = cert.as_s().unwrap();
                // Try to decode as base64, fallback to raw bytes
                let cert_bytes = STANDARD.decode(cert_str).unwrap_or_else(|_| cert_str.as_bytes().to_vec());
                result = result.cert(cert_bytes);
            }

            if let Some(ocsp) = item.get("ocsp_resp") {
                let ocsp_str = ocsp.as_s().unwrap();
                let ocsp_bytes = STANDARD.decode(ocsp_str).unwrap_or_else(|_| ocsp_str.as_bytes().to_vec());
                result.ocsp_resp = ocsp_bytes;
            }

            Some(result)
        })
    }

    /// Extract expiry date from PEM certificate
    fn extract_expiry_from_cert(cert_pem: &[u8]) -> Option<DateTime<Utc>> {
        use x509_parser::prelude::*;

        let pem_str = std::str::from_utf8(cert_pem).ok()?;
        let (_, pem) = x509_parser::pem::parse_x509_pem(pem_str.as_bytes()).ok()?;
        let (_, cert) = X509Certificate::from_der(&pem.contents).ok()?;
        let not_after = cert.validity().not_after;
        DateTime::from_timestamp(not_after.timestamp(), 0)
    }
}

#[async_trait::async_trait()]
impl CryptoStore for DynamoCryptoStore {
    async fn store_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        self.store_certificate_with_owner(hostname, certificate, None).await
    }

    async fn store_certificate_with_owner(
        &self,
        hostname: &str,
        certificate: &Keycert,
        owner_id: Option<&str>,
    ) -> Result<()> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let mut request = self
            .client
            .put_item()
            .table_name(&self.encryption_table)
            .item("hostname", AttributeValue::S(hostname.to_ascii_lowercase()))
            .item("key", AttributeValue::S(STANDARD.encode(&certificate.key)))
            .item("cert", AttributeValue::S(STANDARD.encode(&certificate.cert)));

        if !certificate.ocsp_resp.is_empty() {
            request = request.item("ocsp_resp", AttributeValue::S(STANDARD.encode(&certificate.ocsp_resp)));
        }

        if let Some(oid) = owner_id {
            request = request.item("owner_id", AttributeValue::S(oid.to_string()));
        }

        // Extract expiry from certificate
        if let Some(expires_at) = Self::extract_expiry_from_cert(&certificate.cert) {
            request = request.item("expires_at", AttributeValue::N(expires_at.timestamp_millis().to_string()));
        }

        request
            .condition_expression("attribute_not_exists(hostname)")
            .send()
            .await?;

        Ok(())
    }

    async fn update_certificate(&self, hostname: &str, certificate: &Keycert) -> Result<()> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let mut request = self
            .client
            .put_item()
            .table_name(&self.encryption_table)
            .item("hostname", AttributeValue::S(hostname.to_ascii_lowercase()))
            .item("key", AttributeValue::S(STANDARD.encode(&certificate.key)))
            .item("cert", AttributeValue::S(STANDARD.encode(&certificate.cert)));

        if !certificate.ocsp_resp.is_empty() {
            request = request.item("ocsp_resp", AttributeValue::S(STANDARD.encode(&certificate.ocsp_resp)));
        }

        // Extract expiry from certificate
        if let Some(expires_at) = Self::extract_expiry_from_cert(&certificate.cert) {
            request = request.item("expires_at", AttributeValue::N(expires_at.timestamp_millis().to_string()));
        }

        request.send().await?;
        Ok(())
    }

    async fn delete_certificate(&self, hostname: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.encryption_table)
            .key("hostname", AttributeValue::S(hostname.to_ascii_lowercase()))
            .send()
            .await?;
        Ok(())
    }

    async fn invalidate_certificate(&self, _: &str) -> Result<()> {
        Ok(())
    }

    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.encryption_table)
            .set_key(Some(HashMap::from([(
                String::from("hostname"),
                AttributeValue::S(server_name.to_ascii_lowercase()),
            )])))
            .send()
            .await;

        let result = match item {
            Ok(item_output) => Ok(self.to_entity(item_output)),
            Err(e) => Err(e),
        };

        Ok(result?)
    }

    async fn get_certificates_expiring_before(
        &self,
        _before: DateTime<Utc>,
    ) -> Result<Vec<CertificateInfo>> {
        // DynamoDB implementation would require a scan with filter
        // For now, return empty vec - MongoDB is the primary store
        Ok(vec![])
    }
}
