use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::CryptoStore;
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

    fn to_entity(&self, model: GetItemOutput) -> Result<Option<Keycert>> {
        let Some(item) = model.item else {
            return Ok(None);
        };

        let text = |name: &str| -> Result<Option<&String>> {
            item.get(name)
                .map(|value| value.as_s())
                .transpose()
                .map_err(|_| anyhow::anyhow!("certificate attribute {name} is not a string"))
        };

        let mut result = Keycert::new();

        if let Some(key) = text("key")? {
            result = result.key(key.as_bytes());
        }

        if let Some(cert) = text("cert")? {
            result = result.cert(cert.as_bytes());
        }

        Ok(Some(result))
    }
}

#[async_trait::async_trait()]
impl CryptoStore for DynamoCryptoStore {
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

        self.to_entity(item?)
    }
}
