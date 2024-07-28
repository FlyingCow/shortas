use std::collections::HashMap;

use anyhow::Result;

use super::dynamo_routes_mapper::to_entity;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_1::to_attribute_value;

use crate::core::BaseRoutesStore;
use crate::model::route::RoutingPolicy;
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
impl BaseRoutesStore for DynamoRoutesStore {
    async fn store_route(&self, route: &Route) -> Result<()> {
        let mut request = self
            .client
            .put_item()
            .table_name(&self.routes_table)
            .item("switch", AttributeValue::S(route.switch.clone()))
            .item("link", AttributeValue::S(route.link.clone()))
            .item(
                "owner.id",
                AttributeValue::S(route.properties.owner_id.clone().unwrap()),
            );

        if let RoutingPolicy::Conditional(conditions) = &route.policy {
            let mut routing = HashMap::new();

            routing.insert(
                "policy".to_string(),
                AttributeValue::S("conditional".to_string()),
            );

            routing.insert("conditions".to_string(), to_attribute_value(conditions)?);

            request = request.item("routing", AttributeValue::M(routing));
        }

        request.send().await?;

        return Ok(());
    }

    async fn update_route(&self, _: &Route) -> Result<()> {
        todo!()
    }
    async fn delete_route(&self, _: &Route) -> Result<()> {
        todo!()
    }

    async fn invalidate_route(&self, _switch: &str, _domain: &str, _path: &str) -> Result<()> {
        todo!()
    }

    async fn get_route(&self, switch: &str, domain: &str, path: &str) -> Result<Option<Route>> {
        /*
                let expression = Condition {
                    ua: Some(UA::IN(vec![
                        "Edge".into(),
                        "Chrome".into(),
                        "Firefox".into(),
                    ])),
                    day_of_month: Some(DayOfMonth::IN(vec![7, 14, 30, 26])),
                    and: Some(vec![Box::new(Condition {
                        os: Some(OS::EQ("Windows".into())),
                        ..Default::default()
                    })]),
                    ..Default::default()
                };

                &self
                    .store_route(&Route::new(
                        "main".to_string(),
                        "localhost%2fcond".to_string(),
                        Some("http://google.com".to_string()),
                        DestinationFormat::Http,
                        Some(302),
                        RouteStatus::Active,
                        None,
                        RoutingTerminal::External,
                        RoutingPolicy::Conditional(vec![ConditionalRouting {
                            key: "test".to_string(),
                            condition: expression,
                        }]),
                        RouteProperties {
                            owner_id: Some("my_users_id".to_string()),
                            creator_id: None,
                            domain_id: None,
                            route_id: None,
                            workspace_id: None,
                            bundling: None,
                            custom: None,
                            native: None,
                            opengraph: false,
                            scripts: None,
                            tags: None,
                        },
                    ))
                    .await
                    .unwrap();
        */
        let link = format!("{}%2f{}", domain, path);

        let item = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .set_key(Some(HashMap::from([
                (
                    "link".to_string(),
                    AttributeValue::S(link.to_ascii_lowercase()),
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
