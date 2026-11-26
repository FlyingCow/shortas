use anyhow::{Context, Result};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::ConversionStore;
use crate::model::Conversion;

/// Items hold the public key, the version and the definition as JSON.
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
    async fn put_conversion(&self, conversion: &Conversion) -> Result<bool> {
        let version = AttributeValue::N(conversion.version.to_string());
        let result = self
            .client
            .put_item()
            .table_name(&self.table)
            .item("public_key", AttributeValue::S(conversion.public_key.clone()))
            .item("version", version.clone())
            .item("definition", AttributeValue::S(serde_json::to_string(conversion)?))
            .condition_expression("attribute_not_exists(public_key) OR #version < :version")
            .expression_attribute_names("#version", "version")
            .expression_attribute_values(":version", version)
            .send()
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(e) if e.as_service_error().is_some_and(|e| e.is_conditional_check_failed_exception()) => {
                Ok(false)
            }
            Err(e) => Err(e.into()),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::conversion_store::tests::*;
    use aws_sdk_dynamodb::types::{
        AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType,
    };

    /// Run against DynamoDB Local with `TEST_DYNAMODB_URL` and any AWS
    /// credentials and region set.
    #[tokio::test]
    #[ignore = "needs DynamoDB"]
    async fn older_versions_are_ignored() {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .endpoint_url(std::env::var("TEST_DYNAMODB_URL").unwrap())
            .load()
            .await;
        for table in ["conversions-a", "conversions-b"] {
            let store = DynamoConversionStore::new(&config, table.into());
            let _ = store.client.delete_table().table_name(table).send().await;
            store
                .client
                .create_table()
                .table_name(table)
                .billing_mode(BillingMode::PayPerRequest)
                .attribute_definitions(
                    AttributeDefinition::builder()
                        .attribute_name("public_key")
                        .attribute_type(ScalarAttributeType::S)
                        .build()
                        .unwrap(),
                )
                .key_schema(
                    KeySchemaElement::builder()
                        .attribute_name("public_key")
                        .key_type(KeyType::Hash)
                        .build()
                        .unwrap(),
                )
                .send()
                .await
                .unwrap();
            if table == "conversions-a" {
                older_versions_never_replace_newer(&store).await;
            } else {
                concurrent_writes_keep_the_newest(&store).await;
            }
        }
    }
}
