use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::clickstream_store::ClickStreamStore;
use crate::model::clickstream::{ClickStreamItem, ClickStreamQuery, ClickStreamResponse};

/// ClickHouse row structure for click stream data (JSON deserialization)
#[derive(Debug, Serialize, Deserialize)]
struct ClickStreamRow {
    id: String,
    owner_id: String,
    creator_id: String,
    route_id: String,
    workspace_id: String,
    #[serde(with = "clickhouse_datetime_format")]
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
    #[serde(default, with = "clickhouse_datetime_format_opt")]
    session_first: Option<DateTime<Utc>>,
    session_clicks: Option<u64>,
    is_unique: bool,
    is_bot: bool,
}

/// Custom deserializer for ClickHouse DateTime format (YYYY-MM-DD HH:MM:SS.mmm)
mod clickhouse_datetime_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .map_err(serde::de::Error::custom)
    }
}

/// Custom deserializer for optional ClickHouse DateTime
mod clickhouse_datetime_format_opt {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => {
                let s = format!("{}", dt.format(FORMAT));
                serializer.serialize_some(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) if !s.is_empty() => {
                NaiveDateTime::parse_from_str(&s, FORMAT)
                    .map(|dt| Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)))
                    .map_err(serde::de::Error::custom)
            }
            _ => Ok(None),
        }
    }
}

/// ClickHouse row structure for count queries
#[derive(Debug, Serialize, Deserialize)]
struct CountRow {
    count: u64,
}

/// ClickHouse implementation of ClickStreamStore
#[derive(Clone)]
pub struct ClickHouseClickStreamStore {
    http_client: reqwest::Client,
    url: String,
    user: String,
    password: String,
    database: String,
}

impl ClickHouseClickStreamStore {
    /// Create a new ClickHouse click stream store
    pub fn new(url: &str, user: &str, password: &str, database: &str) -> Result<Self> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            url: url.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
        })
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
            "SELECT id, owner_id, creator_id, route_id, workspace_id, created, dest, ip, \
             continent, country, location, os_family, os_version, user_agent_family, \
             user_agent_version, device_brand, device_family, device_model, \
             session_first, session_clicks, is_unique, is_bot \
             FROM click_stream {} ORDER BY id DESC LIMIT {} OFFSET {} FORMAT JSONEachRow",
            where_clause, limit, offset
        );

        let response = self.http_client
            .get(&self.url)
            .query(&[
                ("user", &self.user),
                ("password", &self.password),
                ("database", &self.database),
                ("query", &sql),
            ])
            .send()
            .await?;

        let text = response.text().await?;
        let items: Vec<ClickStreamItem> = text
            .lines()
            .filter_map(|line| serde_json::from_str::<ClickStreamRow>(line).ok())
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
            "SELECT COUNT(*) as count FROM click_stream {} FORMAT JSONEachRow",
            where_clause
        );

        let response = self.http_client
            .get(&self.url)
            .query(&[
                ("user", &self.user),
                ("password", &self.password),
                ("database", &self.database),
                ("query", &sql),
            ])
            .send()
            .await?;

        let text = response.text().await?;
        let result: CountRow = serde_json::from_str(text.lines().next().unwrap_or("{\"count\":0}"))?;
        Ok(result.count)
    }
}
