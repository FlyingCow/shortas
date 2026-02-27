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
        model.item.map_or(None, |item| {
            let mut result = Keycert::new();

            if let Some(key) = item.get("key") {
                result = result.key(key.as_s().unwrap().as_bytes());
            }

            if let Some(cert) = item.get("cert") {
                result = result.cert(cert.as_s().unwrap().as_bytes());
            }

            Some(result)
        })
    }
}

#[async_trait::async_trait()]
impl CryptoStore for DynamoCryptoStore {
    async fn store_certificate(&self, _hostname: &str, _certificate: &Keycert) -> Result<()> {
        todo!()
    }

    async fn store_certificate_with_owner(
        &self,
        _hostname: &str,
        _certificate: &Keycert,
        _owner_id: Option<&str>,
    ) -> Result<()> {
        todo!()
    }

    async fn update_certificate(&self, _hostname: &str, _certificate: &Keycert) -> Result<()> {
        todo!()
    }
    async fn delete_certificate(&self, _hostname: &str) -> Result<()> {
        todo!()
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
