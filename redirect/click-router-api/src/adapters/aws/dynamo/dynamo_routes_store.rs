use std::collections::HashMap;

use anyhow::Result;

use super::dynamo_routes_mapper::to_entity;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_1::to_attribute_value;

use crate::core::RoutesStore;
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
impl RoutesStore for DynamoRoutesStore {
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

    async fn update_route(&self, route: &Route) -> Result<()> {
        let mut request = self
            .client
            .put_item()
            .table_name(&self.routes_table)
            .item("switch", AttributeValue::S(route.switch.clone()))
            .item("link", AttributeValue::S(route.link.clone()));

        if let Some(dest) = &route.dest {
            request = request.item("dest", AttributeValue::S(dest.clone()));
        }

        if let Some(code) = route.code {
            request = request.item("code", AttributeValue::N(code.to_string()));
        }

        if let Some(ttl) = route.ttl {
            request = request.item("ttl", AttributeValue::N(ttl.to_string()));
        }

        if let Some(owner_id) = &route.properties.owner_id {
            request = request.item("owner.id", AttributeValue::S(owner_id.clone()));
        }

        if let Some(route_id) = &route.properties.route_id {
            request = request.item("route.id", AttributeValue::S(route_id.clone()));
        }

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
        Ok(())
    }

    async fn delete_route(&self, route: &Route) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.routes_table)
            .key("switch", AttributeValue::S(route.switch.to_ascii_lowercase()))
            .key("link", AttributeValue::S(route.link.to_ascii_lowercase()))
            .send()
            .await?;
        Ok(())
    }

    async fn get_route_by_route_id(&self, route_id: &str) -> Result<Option<Route>> {
        // Query using GSI on route.id
        let result = self
            .client
            .query()
            .table_name(&self.routes_table)
            .index_name("route-id-index")
            .key_condition_expression("#rid = :route_id")
            .expression_attribute_names("#rid", "route.id")
            .expression_attribute_values(":route_id", AttributeValue::S(route_id.to_string()))
            .limit(1)
            .send()
            .await;

        match result {
            Ok(output) => {
                if let Some(items) = output.items {
                    if !items.is_empty() {
                        return Ok(to_entity(aws_sdk_dynamodb::operation::get_item::GetItemOutput::builder()
                            .set_item(Some(items[0].clone()))
                            .build())?);
                    }
                }
                Ok(None)
            }
            Err(_) => Ok(None),
        }
    }

    async fn invalidate_route(&self, _switch: &str, _link: &str) -> Result<()> {
        // DynamoDB doesn't have a cache to invalidate in the same way
        // This is a no-op for DynamoDB since we always read fresh data
        Ok(())
    }

    async fn get_route(&self, switch: &str, link: &str) -> Result<Option<Route>> {
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

    async fn get_routes_by_link(&self, link: &str) -> Result<Vec<Route>> {
        // Query using GSI on link
        let result = self
            .client
            .query()
            .table_name(&self.routes_table)
            .index_name("link-index")
            .key_condition_expression("link = :link")
            .expression_attribute_values(":link", AttributeValue::S(link.to_ascii_lowercase()))
            .send()
            .await?;

        let mut routes = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Ok(Some(route)) = to_entity(
                    aws_sdk_dynamodb::operation::get_item::GetItemOutput::builder()
                        .set_item(Some(item))
                        .build(),
                ) {
                    routes.push(route);
                }
            }
        }
        Ok(routes)
    }

    async fn delete_routes_by_link(&self, link: &str) -> Result<u64> {
        // First, query all routes with this link
        let routes = self.get_routes_by_link(link).await?;
        let count = routes.len() as u64;

        // Delete each route
        for route in routes {
            self.client
                .delete_item()
                .table_name(&self.routes_table)
                .key("switch", AttributeValue::S(route.switch.to_ascii_lowercase()))
                .key("link", AttributeValue::S(route.link.to_ascii_lowercase()))
                .send()
                .await?;
        }

        Ok(count)
    }

    async fn store_route_family(&self, routes: &[Route]) -> Result<()> {
        use aws_sdk_dynamodb::types::{Put, TransactWriteItem, Delete};

        if routes.is_empty() {
            return Ok(());
        }

        let link = &routes[0].link;

        // First, delete all existing routes with this link
        self.delete_routes_by_link(link).await?;

        // Batch the new routes in groups of 25 (DynamoDB limit)
        for chunk in routes.chunks(25) {
            let mut transact_items: Vec<TransactWriteItem> = Vec::new();

            for route in chunk {
                let mut item = HashMap::new();
                item.insert("switch".to_string(), AttributeValue::S(route.switch.clone()));
                item.insert("link".to_string(), AttributeValue::S(route.link.clone()));

                if let Some(dest) = &route.dest {
                    item.insert("dest".to_string(), AttributeValue::S(dest.clone()));
                }

                if let Some(code) = route.code {
                    item.insert("code".to_string(), AttributeValue::N(code.to_string()));
                }

                if let Some(owner_id) = &route.properties.owner_id {
                    item.insert("owner.id".to_string(), AttributeValue::S(owner_id.clone()));
                }

                if let Some(route_id) = &route.properties.route_id {
                    item.insert("route.id".to_string(), AttributeValue::S(route_id.clone()));
                }

                if let RoutingPolicy::Conditional(conditions) = &route.policy {
                    let mut routing = HashMap::new();
                    routing.insert(
                        "policy".to_string(),
                        AttributeValue::S("conditional".to_string()),
                    );
                    routing.insert("conditions".to_string(), to_attribute_value(conditions)?);
                    item.insert("routing".to_string(), AttributeValue::M(routing));
                }

                let put = Put::builder()
                    .table_name(&self.routes_table)
                    .set_item(Some(item))
                    .build()?;

                transact_items.push(TransactWriteItem::builder().put(put).build());
            }

            self.client
                .transact_write_items()
                .set_transact_items(Some(transact_items))
                .send()
                .await?;
        }

        Ok(())
    }

    async fn delete_route_by_switch_and_link(&self, switch: &str, link: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.routes_table)
            .key("switch", AttributeValue::S(switch.to_ascii_lowercase()))
            .key("link", AttributeValue::S(link.to_ascii_lowercase()))
            .send()
            .await?;
        Ok(())
    }
}
