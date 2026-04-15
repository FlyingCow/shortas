use std::collections::HashMap;

use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{DateTime, Utc};

use crate::core::DomainStore;
use crate::model::{Domain, VerificationReason, VerificationStatus};

#[derive(Clone, Debug)]
pub struct DynamoDomainStore {
    client: Client,
    domains_table: String,
}

impl DynamoDomainStore {
    pub fn new(sdk_config: &SdkConfig, domains_table: String) -> Self {
        Self {
            domains_table,
            client: Client::new(sdk_config),
        }
    }

    fn domain_to_item(domain: &Domain) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue::S(domain.id.clone()));
        item.insert("name".to_string(), AttributeValue::S(domain.name.clone()));
        item.insert(
            "owner_id".to_string(),
            AttributeValue::S(domain.owner_id.clone()),
        );
        item.insert(
            "status".to_string(),
            AttributeValue::S(domain.status.to_string()),
        );
        item.insert(
            "verification_reason".to_string(),
            AttributeValue::S(serde_json::to_string(&domain.verification_reason).unwrap_or_default()),
        );
        if let Some(last_check) = domain.last_check_at {
            item.insert(
                "last_check_at".to_string(),
                AttributeValue::N(last_check.timestamp_millis().to_string()),
            );
        }
        if let Some(next_check) = domain.next_check_at {
            item.insert(
                "next_check_at".to_string(),
                AttributeValue::N(next_check.timestamp_millis().to_string()),
            );
        }
        item.insert(
            "created_at".to_string(),
            AttributeValue::N(domain.created_at.timestamp_millis().to_string()),
        );
        // GSI for owner_id lookups
        item.insert(
            "owner_name".to_string(),
            AttributeValue::S(format!("{}#{}", domain.owner_id, domain.name.to_lowercase())),
        );
        item
    }

    fn item_to_domain(item: &HashMap<String, AttributeValue>) -> Result<Domain> {
        let id = item
            .get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing id"))?
            .clone();

        let name = item
            .get("name")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing name"))?
            .clone();

        let owner_id = item
            .get("owner_id")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| anyhow!("Missing owner_id"))?
            .clone();

        let status_str = item
            .get("status")
            .and_then(|v| v.as_s().ok())
            .unwrap_or(&"pending".to_string())
            .clone();

        let status = match status_str.as_str() {
            "verified" => VerificationStatus::Verified,
            "failed" => VerificationStatus::Failed,
            _ => VerificationStatus::Pending,
        };

        let verification_reason = item
            .get("verification_reason")
            .and_then(|v| v.as_s().ok())
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(VerificationReason::NotChecked);

        let last_check_at = item
            .get("last_check_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts));

        let next_check_at = item
            .get("next_check_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts));

        let created_at = item
            .get("created_at")
            .and_then(|v| v.as_n().ok())
            .and_then(|n| n.parse::<i64>().ok())
            .and_then(|ts| DateTime::from_timestamp_millis(ts))
            .unwrap_or_else(Utc::now);

        Ok(Domain {
            id,
            name,
            owner_id,
            status,
            verification_reason,
            last_check_at,
            next_check_at,
            created_at,
        })
    }
}

#[async_trait::async_trait()]
impl DomainStore for DynamoDomainStore {
    async fn store_domain(&self, domain: &Domain) -> Result<()> {
        let item = Self::domain_to_item(domain);

        self.client
            .put_item()
            .table_name(&self.domains_table)
            .set_item(Some(item))
            .condition_expression("attribute_not_exists(id)")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to store domain: {}", e))?;

        Ok(())
    }

    async fn update_domain(&self, domain: &Domain) -> Result<()> {
        let item = Self::domain_to_item(domain);

        self.client
            .put_item()
            .table_name(&self.domains_table)
            .set_item(Some(item))
            .condition_expression("attribute_exists(id)")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to update domain: {}", e))?;

        Ok(())
    }

    async fn delete_domain(&self, id: &str) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.domains_table)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to delete domain: {}", e))?;

        Ok(())
    }

    async fn get_domain(&self, id: &str) -> Result<Option<Domain>> {
        let result = self
            .client
            .get_item()
            .table_name(&self.domains_table)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| anyhow!("Failed to get domain: {}", e))?;

        match result.item {
            Some(item) => Ok(Some(Self::item_to_domain(&item)?)),
            None => Ok(None),
        }
    }

    async fn get_domain_by_name(&self, name: &str, owner_id: &str) -> Result<Option<Domain>> {
        let owner_name = format!("{}#{}", owner_id, name.to_lowercase());

        let result = self
            .client
            .query()
            .table_name(&self.domains_table)
            .index_name("owner_name-index")
            .key_condition_expression("owner_name = :owner_name")
            .expression_attribute_values(":owner_name", AttributeValue::S(owner_name))
            .limit(1)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to query domain by name: {}", e))?;

        match result.items {
            Some(items) if !items.is_empty() => Ok(Some(Self::item_to_domain(&items[0])?)),
            _ => Ok(None),
        }
    }

    async fn list_domains(
        &self,
        owner_id: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<Domain>, u64)> {
        let mut domains = Vec::new();
        let mut exclusive_start_key = None;
        let skip = ((page - 1) * page_size) as usize;

        loop {
            let mut scan = self.client.scan().table_name(&self.domains_table);

            if let Some(oid) = owner_id {
                scan = scan
                    .filter_expression("owner_id = :owner_id")
                    .expression_attribute_values(":owner_id", AttributeValue::S(oid.to_string()));
            }

            if let Some(key) = exclusive_start_key.take() {
                scan = scan.set_exclusive_start_key(Some(key));
            }

            let result = scan.send().await.map_err(|e| anyhow!("Failed to scan domains: {}", e))?;

            if let Some(items) = &result.items {
                for item in items {
                    if let Ok(domain) = Self::item_to_domain(item) {
                        domains.push(domain);
                    }
                }
            }

            match result.last_evaluated_key {
                Some(key) => exclusive_start_key = Some(key),
                None => break,
            }
        }

        // Sort by name
        domains.sort_by(|a, b| a.name.cmp(&b.name));

        let total = domains.len() as u64;
        let page_domains: Vec<Domain> = domains
            .into_iter()
            .skip(skip)
            .take(page_size as usize)
            .collect();

        Ok((page_domains, total))
    }

    async fn get_domains_for_verification(
        &self,
        before: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<Domain>> {
        let before_millis = before.timestamp_millis();
        let mut domains = Vec::new();
        let mut exclusive_start_key = None;

        loop {
            let mut scan = self
                .client
                .scan()
                .table_name(&self.domains_table)
                .filter_expression(
                    "next_check_at <= :before OR attribute_not_exists(next_check_at)",
                )
                .expression_attribute_values(
                    ":before",
                    AttributeValue::N(before_millis.to_string()),
                );

            if let Some(key) = exclusive_start_key {
                scan = scan.set_exclusive_start_key(Some(key));
            }

            let result = scan
                .send()
                .await
                .map_err(|e| anyhow!("Failed to scan domains for verification: {}", e))?;

            if let Some(items) = result.items {
                for item in items {
                    if let Ok(domain) = Self::item_to_domain(&item) {
                        domains.push(domain);
                        if domains.len() >= limit {
                            return Ok(domains);
                        }
                    }
                }
            }

            if result.last_evaluated_key.is_none() {
                break;
            }
            exclusive_start_key = result.last_evaluated_key;
        }

        // Sort by next_check_at
        domains.sort_by(|a, b| {
            a.next_check_at
                .unwrap_or(DateTime::<Utc>::MIN_UTC)
                .cmp(&b.next_check_at.unwrap_or(DateTime::<Utc>::MIN_UTC))
        });

        domains.truncate(limit);
        Ok(domains)
    }
}
