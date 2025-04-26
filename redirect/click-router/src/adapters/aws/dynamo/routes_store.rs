use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use super::routes_mapper::to_entity;
use crate::core::RoutesStore;
use crate::model::Route;

#[derive(Clone, Debug)]
pub struct DynamoRoutesStore {
    client: Client,
    routes_table: String,
}

impl DynamoRoutesStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }
}

#[async_trait::async_trait()]
impl RoutesStore for DynamoRoutesStore {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .set_key(Some(HashMap::from([
                (
                    "link".to_string(),
                    AttributeValue::S(path.to_ascii_lowercase()),
                ),
                (
                    "switch".to_string(),
                    AttributeValue::S(switch.to_ascii_lowercase()),
                ),
            ])))
            .send()
            .await?;

        Ok(to_entity(item)?)
    }
}
