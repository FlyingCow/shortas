use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::base_routes_store::{BaseRoutesStore, Result};
use crate::domain::Route;

#[derive(Clone, Debug)]
pub struct RoutesStore {
    client: Client,
    routes_table: String,
}

impl RoutesStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }

    fn to_entity(&self, model: GetItemOutput) -> Option<Route> {
        model.item.map_or(None, |item| {
            let switch_str = String::from(item.get("switch").unwrap().as_s().unwrap());
            let link_str = String::from(item.get("link").unwrap().as_s().unwrap());

            let dest = match item.get("dest") {
                Some(dest) => Some(String::from(dest.as_s().unwrap())),
                None => None,
            };

            Some(Route::new(switch_str, link_str, dest))
        })
    }
}

impl BaseRoutesStore for RoutesStore {
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
            .await;

        let result = match item {
            Ok(item_output) => Ok(self.to_entity(item_output)),
            Err(e) => {
                println!("status: {}", e.raw_response().unwrap().status());
                Err(e)
            }
        };

        Ok(result?)
    }
}
