use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};

use crate::core::RouteStore;
use crate::model::RouteToVerify;

#[derive(Clone, Debug)]
pub struct DynamoRouteStore {
    client: Client,
    routes_table: String,
}

impl DynamoRouteStore {
    pub fn new(sdk_config: &SdkConfig, routes_table: String) -> Self {
        Self {
            routes_table,
            client: Client::new(sdk_config),
        }
    }

    fn route_to_item(route: &RouteToVerify) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue::S(route.id.clone()));
        item.insert("link".to_string(), AttributeValue::S(route.link.clone()));

        let destinations: Vec<AttributeValue> = route
            .destinations
            .iter()
            .map(|d| AttributeValue::S(d.clone()))
            .collect();
        item.insert("destinations".to_string(), AttributeValue::L(destinations));

        if let Some(owner_id) = &route.owner_id {
            item.insert("owner_id".to_string(), AttributeValue::S(owner_id.clone()));
        }
        if let Some(workspace_id) = &route.workspace_id {
            item.insert(
                "workspace_id".to_string(),
                AttributeValue::S(workspace_id.clone()),
            );
        }
        item.insert("status".to_string(), AttributeValue::S(route.status.clone()));
        if let Some(reason) = &route.blocked_reason {
            item.insert("blocked_reason".to_string(), AttributeValue::S(reason.clone()));
        }
        if let Some(last_check) = route.last_safety_check {
            item.insert(
                "last_safety_check".to_string(),
                AttributeValue::N(last_check.to_string()),
            );
        }
        if let Some(next_check) = route.next_safety_check {
            item.insert(
                "next_safety_check".to_string(),
                AttributeValue::N(next_check.to_string()),
            );
        }
        item
    }

    fn item_to_route(item: &HashMap<String, AttributeValue>) -> Result<RouteToVerify> {
        let id = item
            .get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing id"))?
            .clone();

        let link = item
            .get("link")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing link"))?
            .clone();

        let destinations = item
            .get("destinations")
            .and_then(|v| v.as_l().ok())
            .map(|list| {
                list.iter()
                    .filter_map(|v| v.as_s().ok().map(|s| s.clone()))
                    .collect()
            })
            .unwrap_or_default();

        let owner_id = item
            .get("owner_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let workspace_id = item
            .get("workspace_id")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let status = item
            .get("status")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone())
            .unwrap_or_else(|| "Active".to_string());

        let blocked_reason = item
            .get("blocked_reason")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let last_safety_check = item
            .get("last_safety_check")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok());

        let next_safety_check = item
            .get("next_safety_check")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok());

        Ok(RouteToVerify {
            id,
            link,
            destinations,
            owner_id,
            workspace_id,
            status,
            blocked_reason,
            last_safety_check,
            next_safety_check,
        })
    }
}

#[async_trait::async_trait]
impl RouteStore for DynamoRouteStore {
    async fn store_route(&self, route: &RouteToVerify) -> Result<()> {
        let item = Self::route_to_item(route);

        self.client
            .put_item()
            .table_name(&self.routes_table)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to store route: {}", e))?;

        Ok(())
    }

    async fn update_route(&self, route: &RouteToVerify) -> Result<()> {
        self.client
            .update_item()
            .table_name(&self.routes_table)
            .key("id", AttributeValue::S(route.id.clone()))
            .update_expression(
                "SET link = :link, destinations = :destinations, owner_id = :owner_id, workspace_id = :workspace_id",
            )
            .expression_attribute_values(":link", AttributeValue::S(route.link.clone()))
            .expression_attribute_values(
                ":destinations",
                AttributeValue::L(
                    route
                        .destinations
                        .iter()
                        .map(|d| AttributeValue::S(d.clone()))
                        .collect(),
                ),
            )
            .expression_attribute_values(
                ":owner_id",
                route
                    .owner_id
                    .as_ref()
                    .map(|s| AttributeValue::S(s.clone()))
                    .unwrap_or(AttributeValue::Null(true)),
            )
            .expression_attribute_values(
                ":workspace_id",
                route
                    .workspace_id
                    .as_ref()
                    .map(|s| AttributeValue::S(s.clone()))
                    .unwrap_or(AttributeValue::Null(true)),
            )
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update route: {}", e))?;

        Ok(())
    }

    async fn delete_route(&self, id: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.routes_table)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete route: {}", e))?;

        Ok(())
    }

    async fn get_route(&self, id: &str) -> Result<Option<RouteToVerify>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.routes_table)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get route: {}", e))?;

        match result.item {
            Some(item) => Ok(Some(Self::item_to_route(&item)?)),
            None => Ok(None),
        }
    }

    async fn list_routes(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<RouteToVerify>, u64)> {
        let mut routes = Vec::new();
        let mut exclusive_start_key = None;
        let skip = ((page - 1) * page_size) as usize;

        loop {
            let mut scan = self.client.scan().table_name(&self.routes_table);

            if let Some(oid) = owner_id {
                scan = scan
                    .filter_expression("owner_id = :owner_id")
                    .expression_attribute_values(":owner_id", AttributeValue::S(oid.to_string()));
            }

            if let Some(key) = exclusive_start_key {
                scan = scan.set_exclusive_start_key(Some(key));
            }

            let result = scan
                .send()
                .await
                .map_err(|e| anyhow!("Failed to scan routes: {}", e))?;

            if let Some(items) = result.items {
                for item in items {
                    if let Ok(route) = Self::item_to_route(&item) {
                        routes.push(route);
                    }
                }
            }

            if result.last_evaluated_key.is_none() {
                break;
            }
            exclusive_start_key = result.last_evaluated_key;
        }

        // Sort by link
        routes.sort_by(|a, b| a.link.cmp(&b.link));

        let total = routes.len() as u64;
        let page_routes: Vec<RouteToVerify> = routes
            .into_iter()
            .skip(skip)
            .take(page_size as usize)
            .collect();

        Ok((page_routes, total))
    }

    async fn get_routes_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<RouteToVerify>> {
        let before_millis = before.timestamp_millis();
        let mut routes = Vec::new();
        let mut exclusive_start_key = None;

        loop {
            let mut scan = self
                .client
                .scan()
                .table_name(&self.routes_table)
                .filter_expression(
                    "(next_safety_check <= :before OR attribute_not_exists(next_safety_check)) AND size(destinations) > :zero",
                )
                .expression_attribute_values(
                    ":before",
                    AttributeValue::N(before_millis.to_string()),
                )
                .expression_attribute_values(":zero", AttributeValue::N("0".to_string()));

            if let Some(key) = exclusive_start_key {
                scan = scan.set_exclusive_start_key(Some(key));
            }

            let result = scan
                .send()
                .await
                .map_err(|e| anyhow!("Failed to scan routes for verification: {}", e))?;

            if let Some(items) = result.items {
                for item in items {
                    if let Ok(route) = Self::item_to_route(&item) {
                        routes.push(route);
                        if routes.len() >= limit {
                            // Sort by next_safety_check before returning
                            routes.sort_by(|a, b| {
                                a.next_safety_check
                                    .unwrap_or(i64::MIN)
                                    .cmp(&b.next_safety_check.unwrap_or(i64::MIN))
                            });
                            routes.truncate(limit);
                            return Ok(routes);
                        }
                    }
                }
            }

            if result.last_evaluated_key.is_none() {
                break;
            }
            exclusive_start_key = result.last_evaluated_key;
        }

        // Sort by next_safety_check
        routes.sort_by(|a, b| {
            a.next_safety_check
                .unwrap_or(i64::MIN)
                .cmp(&b.next_safety_check.unwrap_or(i64::MIN))
        });

        routes.truncate(limit);
        Ok(routes)
    }

    async fn update_safety_check_timestamps(
        &self,
        route_id: &str,
        last_check: DateTime<Utc>,
        next_check: DateTime<Utc>,
    ) -> Result<()> {
        self.client
            .update_item()
            .table_name(&self.routes_table)
            .key("id", AttributeValue::S(route_id.to_string()))
            .update_expression(
                "SET last_safety_check = :last_check, next_safety_check = :next_check",
            )
            .expression_attribute_values(
                ":last_check",
                AttributeValue::N(last_check.timestamp_millis().to_string()),
            )
            .expression_attribute_values(
                ":next_check",
                AttributeValue::N(next_check.timestamp_millis().to_string()),
            )
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update safety check timestamps: {}", e))?;

        Ok(())
    }

    async fn update_route_status(
        &self,
        route_id: &str,
        status: &str,
        blocked_reason: Option<&str>,
    ) -> Result<()> {
        let mut update_expr = "SET #status = :status".to_string();
        let mut expr_attr_names = HashMap::new();
        expr_attr_names.insert("#status".to_string(), "status".to_string());

        let mut builder = self
            .client
            .update_item()
            .table_name(&self.routes_table)
            .key("id", AttributeValue::S(route_id.to_string()))
            .expression_attribute_values(":status", AttributeValue::S(status.to_string()));

        if let Some(reason) = blocked_reason {
            update_expr.push_str(", blocked_reason = :blocked_reason");
            builder =
                builder.expression_attribute_values(":blocked_reason", AttributeValue::S(reason.to_string()));
        } else {
            update_expr.push_str(" REMOVE blocked_reason");
        }

        builder
            .update_expression(update_expr)
            .set_expression_attribute_names(Some(expr_attr_names))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update route status: {}", e))?;

        Ok(())
    }
}
