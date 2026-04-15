use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};

use crate::core::OrderStore;
use crate::model::{CertificateOrder, OrderStatus};

#[derive(Clone)]
pub struct DynamoOrderStore {
    client: Client,
    orders_table: String,
}

impl DynamoOrderStore {
    pub fn new(sdk_config: &SdkConfig, orders_table: String) -> Self {
        Self {
            orders_table,
            client: Client::new(sdk_config),
        }
    }

    fn order_to_item(order: &CertificateOrder) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert(
            "order_id".to_string(),
            AttributeValue::S(order.order_id.clone()),
        );
        item.insert("domain".to_string(), AttributeValue::S(order.domain.clone()));
        item.insert(
            "owner_id".to_string(),
            AttributeValue::S(order.owner_id.clone()),
        );
        item.insert(
            "status".to_string(),
            AttributeValue::S(Self::status_to_string(&order.status)),
        );
        if let Some(msg) = &order.error_message {
            item.insert("error_message".to_string(), AttributeValue::S(msg.clone()));
        }
        item.insert(
            "retry_count".to_string(),
            AttributeValue::N(order.retry_count.to_string()),
        );
        item.insert(
            "max_retries".to_string(),
            AttributeValue::N(order.max_retries.to_string()),
        );

        // ACME fields
        if let Some(url) = &order.acme_order_url {
            item.insert("acme_order_url".to_string(), AttributeValue::S(url.clone()));
        }
        if let Some(url) = &order.acme_authorization_url {
            item.insert(
                "acme_authorization_url".to_string(),
                AttributeValue::S(url.clone()),
            );
        }
        if let Some(url) = &order.acme_finalize_url {
            item.insert(
                "acme_finalize_url".to_string(),
                AttributeValue::S(url.clone()),
            );
        }
        if let Some(url) = &order.acme_certificate_url {
            item.insert(
                "acme_certificate_url".to_string(),
                AttributeValue::S(url.clone()),
            );
        }

        // Timestamps
        item.insert(
            "created_at".to_string(),
            AttributeValue::N(order.created_at.timestamp_millis().to_string()),
        );
        item.insert(
            "updated_at".to_string(),
            AttributeValue::N(order.updated_at.timestamp_millis().to_string()),
        );
        if let Some(expires) = order.expires_at {
            item.insert(
                "expires_at".to_string(),
                AttributeValue::N(expires.timestamp_millis().to_string()),
            );
        }
        if let Some(next_retry) = order.next_retry_at {
            item.insert(
                "next_retry_at".to_string(),
                AttributeValue::N(next_retry.timestamp_millis().to_string()),
            );
        }

        // GSI for domain lookups with active status
        let is_active = !matches!(
            order.status,
            OrderStatus::Valid | OrderStatus::Failed | OrderStatus::Expired
        );
        if is_active {
            item.insert(
                "domain_active".to_string(),
                AttributeValue::S(format!("{}#active", order.domain.to_lowercase())),
            );
        }

        // GSI for status queries
        item.insert(
            "status_key".to_string(),
            AttributeValue::S(Self::status_to_string(&order.status)),
        );

        item
    }

    fn item_to_order(item: &HashMap<String, AttributeValue>) -> Result<CertificateOrder> {
        let order_id = item
            .get("order_id")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing order_id"))?
            .clone();

        let domain = item
            .get("domain")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing domain"))?
            .clone();

        let owner_id = item
            .get("owner_id")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing owner_id"))?
            .clone();

        let status_str = item
            .get("status")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.as_str())
            .unwrap_or("pending");

        let status = Self::string_to_status(status_str);

        let error_message = item
            .get("error_message")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let retry_count = item
            .get("retry_count")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<u32>().ok())
            .unwrap_or(0);

        let max_retries = item
            .get("max_retries")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<u32>().ok())
            .unwrap_or(3);

        let acme_order_url = item
            .get("acme_order_url")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let acme_authorization_url = item
            .get("acme_authorization_url")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let acme_finalize_url = item
            .get("acme_finalize_url")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let acme_certificate_url = item
            .get("acme_certificate_url")
            .and_then(|v| v.as_s().ok())
            .map(|s| s.clone());

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts))
            .unwrap_or_else(Utc::now);

        let updated_at = item
            .get("updated_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts))
            .unwrap_or_else(Utc::now);

        let expires_at = item
            .get("expires_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts));

        let next_retry_at = item
            .get("next_retry_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts));

        Ok(CertificateOrder {
            id: None, // DynamoDB doesn't use MongoDB ObjectId
            order_id,
            domain,
            owner_id,
            status,
            error_message,
            retry_count,
            max_retries,
            acme_order_url,
            acme_authorization_url,
            acme_finalize_url,
            acme_certificate_url,
            created_at,
            updated_at,
            expires_at,
            next_retry_at,
        })
    }

    fn status_to_string(status: &OrderStatus) -> String {
        match status {
            OrderStatus::Pending => "pending".to_string(),
            OrderStatus::ChallengeCreated => "challenge_created".to_string(),
            OrderStatus::ChallengeReady => "challenge_ready".to_string(),
            OrderStatus::Processing => "processing".to_string(),
            OrderStatus::Valid => "valid".to_string(),
            OrderStatus::Failed => "failed".to_string(),
            OrderStatus::Expired => "expired".to_string(),
        }
    }

    fn string_to_status(s: &str) -> OrderStatus {
        match s {
            "pending" => OrderStatus::Pending,
            "challenge_created" => OrderStatus::ChallengeCreated,
            "challenge_ready" => OrderStatus::ChallengeReady,
            "processing" => OrderStatus::Processing,
            "valid" => OrderStatus::Valid,
            "failed" => OrderStatus::Failed,
            "expired" => OrderStatus::Expired,
            _ => OrderStatus::Pending,
        }
    }
}

#[async_trait]
impl OrderStore for DynamoOrderStore {
    async fn store_order(&self, order: &CertificateOrder) -> Result<()> {
        let item = Self::order_to_item(order);

        self.client
            .put_item()
            .table_name(&self.orders_table)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(order_id)")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to store order: {}", e))?;

        Ok(())
    }

    async fn update_order(&self, order: &CertificateOrder) -> Result<()> {
        let item = Self::order_to_item(order);

        self.client
            .put_item()
            .table_name(&self.orders_table)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update order: {}", e))?;

        Ok(())
    }

    async fn get_order(&self, order_id: &str) -> Result<Option<CertificateOrder>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.orders_table)
            .key("order_id", AttributeValue::S(order_id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get order: {}", e))?;

        match result.item {
            Some(item) => Ok(Some(Self::item_to_order(&item)?)),
            None => Ok(None),
        }
    }

    async fn get_orders_by_status(
        &self,
        status: OrderStatus,
        limit: usize,
    ) -> Result<Vec<CertificateOrder>> {
        let status_str = Self::status_to_string(&status);

        let result = self
            .client
            .query()
            .table_name(&self.orders_table)
            .index_name("status-index")
            .key_condition_expression("status_key = :status")
            .expression_attribute_values(":status", AttributeValue::S(status_str))
            .limit(limit as i32)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to query orders by status: {}", e))?;

        let mut orders = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Ok(order) = Self::item_to_order(&item) {
                    orders.push(order);
                }
            }
        }

        Ok(orders)
    }

    async fn get_active_order_for_domain(
        &self,
        domain: &str,
    ) -> Result<Option<CertificateOrder>> {
        let domain_active = format!("{}#active", domain.to_lowercase());

        let result = self
            .client
            .query()
            .table_name(&self.orders_table)
            .index_name("domain-active-index")
            .key_condition_expression("domain_active = :domain_active")
            .expression_attribute_values(":domain_active", AttributeValue::S(domain_active))
            .limit(1)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to query active order for domain: {}", e))?;

        match result.items {
            Some(items) if !items.is_empty() => Ok(Some(Self::item_to_order(&items[0])?)),
            _ => Ok(None),
        }
    }

    async fn delete_order(&self, order_id: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.orders_table)
            .key("order_id", AttributeValue::S(order_id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete order: {}", e))?;

        Ok(())
    }
}
