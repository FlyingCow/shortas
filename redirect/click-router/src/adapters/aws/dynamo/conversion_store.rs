use anyhow::{Context, Result};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::conversion_definitions::ConversionStore;
use crate::model::Conversion;

#[derive(Clone, Debug)]
pub struct DynamoConversionStore {
    client: Client,
    table: String,
}

impl DynamoConversionStore {
    pub fn new(sdk_config: &SdkConfig, table: String) -> Self {
        Self {
            client: Client::new(sdk_config),
            table,
        }
    }
}

#[async_trait::async_trait()]
impl ConversionStore for DynamoConversionStore {
    async fn get_conversion(&self, public_key: &str) -> Result<Option<Conversion>> {
        let output = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("public_key", AttributeValue::S(public_key.to_string()))
            .send()
            .await?;

        let Some(item) = output.item else {
            return Ok(None);
        };
        let definition = item
            .get("definition")
            .and_then(|v| v.as_s().ok())
            .context("conversion item has no definition")?;
        Ok(Some(serde_json::from_str(definition)?))
    }
}
