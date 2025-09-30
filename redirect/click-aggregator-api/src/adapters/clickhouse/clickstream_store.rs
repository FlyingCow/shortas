use async_trait::async_trait;
use anyhow::Result;
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::clickstream_store::ClickStreamStore;
use crate::model::clickstream::{ClickStreamItem, ClickStreamQuery, ClickStreamResponse};

/// ClickHouse row structure for click stream data
#[derive(Debug, Row, Serialize, Deserialize)]
struct ClickStreamRow {
    id: String,
    owner_id: String,
    creator_id: String,
    route_id: String,
    workspace_id: String,
    created: DateTime<Utc>,
    dest: String,
    ip: String,
    continent: Option<String>,
    country: Option<String>,
    location: Option<String>,
    os_family: Option<String>,
    os_version: Option<String>,
    user_agent_family: Option<String>,
    user_agent_version: Option<String>,
    device_brand: Option<String>,
    device_family: Option<String>,
    device_model: Option<String>,
    session_first: Option<DateTime<Utc>>,
    session_clicks: Option<u128>,
    is_unique: bool,
    is_bot: bool,
}

/// ClickHouse implementation of ClickStreamStore
#[derive(Clone)]
pub struct ClickHouseClickStreamStore {
    client: Client,
}

impl ClickHouseClickStreamStore {
    /// Create a new ClickHouse click stream store
    pub fn new(url: &str, user: &str, password: &str, database: &str) -> Result<Self> {
        let client = Client::default()
            .with_url(url)
            .with_user(user)
            .with_password(password)
            .with_database(database);

        Ok(Self { client })
    }

    /// Build WHERE clause from query filters
    fn build_where_clause(&self, query: &ClickStreamQuery) -> String {
        let mut conditions = Vec::new();

        if let Some(owner_id) = &query.owner_id {
            conditions.push(format!("owner_id = '{}'", owner_id));
        }

        if let Some(creator_id) = &query.creator_id {
            conditions.push(format!("creator_id = '{}'", creator_id));
        }

        if let Some(route_id) = &query.route_id {
            conditions.push(format!("route_id = '{}'", route_id));
        }

        if let Some(workspace_id) = &query.workspace_id {
            conditions.push(format!("workspace_id = '{}'", workspace_id));
        }

        if let Some(created_from) = &query.created_from {
            conditions.push(format!("created >= '{}'", created_from.format("%Y-%m-%d %H:%M:%S")));
        }

        if let Some(created_to) = &query.created_to {
            conditions.push(format!("created <= '{}'", created_to.format("%Y-%m-%d %H:%M:%S")));
        }

        if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        }
    }

    /// Convert ClickStreamRow to ClickStreamItem
    fn row_to_item(row: ClickStreamRow) -> ClickStreamItem {
        ClickStreamItem {
            id: row.id,
            owner_id: row.owner_id,
            creator_id: row.creator_id,
            route_id: row.route_id,
            workspace_id: row.workspace_id,
            created: row.created,
            dest: row.dest,
            ip: row.ip,
            continent: row.continent,
            country: row.country,
            location: row.location,
            os_family: row.os_family,
            os_version: row.os_version,
            user_agent_family: row.user_agent_family,
            user_agent_version: row.user_agent_version,
            device_brand: row.device_brand,
            device_family: row.device_family,
            device_model: row.device_model,
            session_first: row.session_first,
            session_clicks: row.session_clicks,
            is_unique: row.is_unique,
            is_bot: row.is_bot,
        }
    }
}

#[async_trait]
impl ClickStreamStore for ClickHouseClickStreamStore {
    async fn query_clickstream(&self, query: &ClickStreamQuery) -> Result<ClickStreamResponse> {
        let where_clause = self.build_where_clause(query);
        let limit = query.limit.unwrap_or(100);
        let offset = query.offset.unwrap_or(0);

        let sql = format!(
            "SELECT * FROM click_stream {} ORDER BY id DESC LIMIT {} OFFSET {}",
            where_clause, limit, offset
        );

        let cursor = self.client.query(&sql).fetch_all::<ClickStreamRow>().await?;
        
        let items: Vec<ClickStreamItem> = cursor
            .into_iter()
            .map(Self::row_to_item)
            .collect();

        let total = self.count_clickstream(query).await?;
        let has_more = (offset + limit as u32) < total as u32;

        Ok(ClickStreamResponse {
            items,
            total,
            offset,
            limit,
            has_more,
        })
    }

    async fn count_clickstream(&self, query: &ClickStreamQuery) -> Result<u64> {
        let where_clause = self.build_where_clause(query);
        
        let sql = format!(
            "SELECT COUNT(*) as count FROM click_stream {}",
            where_clause
        );

        let result: u64 = self.client.query(&sql).fetch_one().await?;
        Ok(result)
    }
}
