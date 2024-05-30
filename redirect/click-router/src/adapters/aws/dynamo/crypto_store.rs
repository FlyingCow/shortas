use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::BaseCryptoStore;
use crate::model::Keycert;

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

#[async_trait::async_trait(?Send)]
impl BaseCryptoStore for DynamoCryptoStore {
    async fn invalidate(&self, _: &str) -> Result<()> {
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
            Err(e) => {
                Err(e)
            }
        };

        Ok(result?)
    }
}
