use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};
use chrono::{DateTime, Utc};
use tracing::{debug, error, warn};
use crate::core::clickstream_store::ClickStreamStore;
use crate::model::clickstream::{ClickStreamItem, ClickStreamQuery, ClickStreamResponse};

/// Custom deserializer for ClickHouse UInt8 (0/1) to bool
fn deserialize_uint8_as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrInt {
        Bool(bool),
        Int(u8),
    }
    
    match BoolOrInt::deserialize(deserializer)? {
        BoolOrInt::Bool(b) => Ok(b),
        BoolOrInt::Int(i) => match i {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::custom(format!("Invalid boolean value: {}", i))),
        },
    }
}

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
    #[serde(deserialize_with = "deserialize_uint8_as_bool")]
    is_unique: bool,
    #[serde(deserialize_with = "deserialize_uint8_as_bool")]
    is_bot: bool,
}

/// Custom deserializer for ClickHouse DateTime format (YYYY-MM-DD HH:MM:SS.mmm)
mod clickhouse_datetime_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        
        // Try parsing with milliseconds first (DateTime64(3))
        if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
        
        // Try parsing with full fractional seconds (%.f)
        if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
        
        // Fallback to DateTime without fractional seconds
        if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
        
        Err(serde::de::Error::custom(format!("Failed to parse datetime: {}", s)))
    }
}

/// Custom deserializer for optional ClickHouse DateTime
mod clickhouse_datetime_format_opt {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => {
                let s = format!("{}", dt.format("%Y-%m-%d %H:%M:%S%.3f"));
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
                // Try parsing with milliseconds first (DateTime64(3))
                if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f") {
                    return Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)));
                }
                
                // Try parsing with full fractional seconds (%.f)
                if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f") {
                    return Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)));
                }
                
                // Fallback to DateTime without fractional seconds
                if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                    return Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)));
                }
                
                Err(serde::de::Error::custom(format!("Failed to parse datetime: {}", s)))
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

/// Escape a value before interpolating it into a ClickHouse string literal.
/// This HTTP interface doesn't support real parameter binding, so this is a
/// defense-in-depth escape (doubles backslashes and single quotes) applied
/// to every user-supplied value before it's placed inside `'...'`.
fn escape_sql_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "\\'")
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
            conditions.push(format!("owner_id = '{}'", escape_sql_string(owner_id)));
        }

        if let Some(creator_id) = &query.creator_id {
            conditions.push(format!("creator_id = '{}'", escape_sql_string(creator_id)));
        }

        if let Some(route_id) = &query.route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(route_id)));
        }

        if let Some(workspace_id) = &query.workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(workspace_id)));
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
    /// Converts nullable database fields to non-nullable model fields with defaults
    fn row_to_item(row: ClickStreamRow) -> ClickStreamItem {
        use crate::model::clickstream::{UNKNOWN, epoch_datetime};

        ClickStreamItem {
            id: row.id,
            owner_id: row.owner_id,
            creator_id: row.creator_id,
            route_id: row.route_id,
            workspace_id: row.workspace_id,
            created: row.created,
            dest: row.dest,
            ip: row.ip,
            // Convert None to "_unknown" for all string fields
            continent: row.continent.unwrap_or_else(|| UNKNOWN.to_string()),
            country: row.country.unwrap_or_else(|| UNKNOWN.to_string()),
            location: row.location.unwrap_or_else(|| UNKNOWN.to_string()),
            os_family: row.os_family.unwrap_or_else(|| UNKNOWN.to_string()),
            os_version: row.os_version.unwrap_or_else(|| UNKNOWN.to_string()),
            user_agent_family: row.user_agent_family.unwrap_or_else(|| UNKNOWN.to_string()),
            user_agent_version: row.user_agent_version.unwrap_or_else(|| UNKNOWN.to_string()),
            device_brand: row.device_brand.unwrap_or_else(|| UNKNOWN.to_string()),
            device_family: row.device_family.unwrap_or_else(|| UNKNOWN.to_string()),
            device_model: row.device_model.unwrap_or_else(|| UNKNOWN.to_string()),
            // Convert None to epoch for DateTime
            session_first: row.session_first.unwrap_or_else(epoch_datetime),
            // Convert None to 0 for numeric fields
            session_clicks: row.session_clicks.unwrap_or(0),
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
        
        // Debug logging
        let line_count = text.lines().count();
        debug!("ClickHouse returned {} lines", line_count);
        if line_count > 0 && line_count <= 3 {
            debug!("First line sample: {}", text.lines().next().unwrap_or(""));
        }
        
        let mut parse_errors = 0;
        let items: Vec<ClickStreamItem> = text
            .lines()
            .filter_map(|line| {
                match serde_json::from_str::<ClickStreamRow>(line) {
                    Ok(row) => Some(row),
                    Err(e) => {
                        if parse_errors < 3 {
                            error!("Failed to parse ClickStream row: {}", e);
                            error!("Line content: {}", line);
                        }
                        parse_errors += 1;
                        None
                    }
                }
            })
            .map(Self::row_to_item)
            .collect();
        
        if parse_errors > 0 {
            warn!("Total parse errors: {}", parse_errors);
        }

        debug!("Successfully parsed {} items", items.len());

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

    async fn get_daily_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::DailyStatsDto>> {
        use crate::dto::clickstream_dto::DailyStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT toString(date) as date, sum(total_clicks) as total_clicks, sum(unique_clicks) as unique_clicks, \
             sum(bot_clicks) as bot_clicks, sum(human_clicks) as human_clicks, sum(unique_ips) as unique_ips \
             FROM click_stream_daily_mv {} GROUP BY date ORDER BY date DESC FORMAT JSONEachRow",
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
        let stats: Vec<DailyStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(stats)
    }

    async fn get_hourly_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_hour: Option<&str>, to_hour: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::HourlyStatsDto>> {
        use crate::dto::clickstream_dto::HourlyStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fh) = from_hour {
            conditions.push(format!("hour >= '{}'", escape_sql_string(fh)));
        }
        if let Some(th) = to_hour {
            conditions.push(format!("hour <= '{}'", escape_sql_string(th)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT hour, sum(total_clicks) as total_clicks, sum(unique_clicks) as unique_clicks, \
             sum(bot_clicks) as bot_clicks, sum(human_clicks) as human_clicks, sum(unique_ips) as unique_ips \
             FROM click_stream_hourly_mv {} GROUP BY hour ORDER BY hour DESC FORMAT JSONEachRow",
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
        let stats: Vec<HourlyStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(stats)
    }

    async fn get_geographic_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::GeographicStatsDto>> {
        use crate::dto::clickstream_dto::GeographicStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT country, continent, location, sum(total_clicks) as total_clicks, \
             sum(unique_clicks) as unique_clicks, sum(unique_ips) as unique_ips \
             FROM click_stream_geographic_mv {} \
             GROUP BY country, continent, location ORDER BY total_clicks DESC LIMIT 100 FORMAT JSONEachRow",
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

        #[derive(Deserialize)]
        struct GeoRow {
            country: String,
            continent: Option<String>,
            location: Option<String>,
            total_clicks: u64,
            unique_clicks: u64,
            unique_ips: u64,
        }

        let stats: Vec<GeographicStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str::<GeoRow>(line).ok())
            .map(|row| GeographicStatsDto {
                country: row.country,
                continent: row.continent,
                location: row.location,
                total_clicks: row.total_clicks,
                unique_clicks: row.unique_clicks,
                unique_ips: row.unique_ips,
            })
            .collect();

        Ok(stats)
    }

    async fn get_device_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::DeviceStatsDto>> {
        use crate::dto::clickstream_dto::DeviceStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT device_family, os_family, sum(total_clicks) as total_clicks, sum(unique_clicks) as unique_clicks \
             FROM click_stream_device_mv {} \
             GROUP BY device_family, os_family ORDER BY total_clicks DESC LIMIT 100 FORMAT JSONEachRow",
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
        let stats: Vec<DeviceStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(stats)
    }

    async fn get_browser_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::BrowserStatsDto>> {
        use crate::dto::clickstream_dto::BrowserStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT user_agent_family, user_agent_version, sum(total_clicks) as total_clicks, sum(unique_clicks) as unique_clicks \
             FROM click_stream_browser_mv {} \
             GROUP BY user_agent_family, user_agent_version ORDER BY total_clicks DESC LIMIT 100 FORMAT JSONEachRow",
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

        #[derive(Deserialize)]
        struct BrowserRow {
            user_agent_family: String,
            user_agent_version: Option<String>,
            total_clicks: u64,
            unique_clicks: u64,
        }

        let stats: Vec<BrowserStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str::<BrowserRow>(line).ok())
            .map(|row| BrowserStatsDto {
                user_agent_family: row.user_agent_family,
                user_agent_version: row.user_agent_version,
                total_clicks: row.total_clicks,
                unique_clicks: row.unique_clicks,
            })
            .collect();

        Ok(stats)
    }

    async fn get_route_performance(&self, owner_id: Option<&str>, workspace_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>, limit: Option<u32>) -> Result<Vec<crate::dto::clickstream_dto::RoutePerformanceDto>> {
        use crate::dto::clickstream_dto::RoutePerformanceDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let limit_clause = limit.unwrap_or(50);

        let sql = format!(
            "SELECT route_id, sum(total_clicks) as total_clicks, sum(unique_visitors) as unique_visitors, \
             sum(bot_clicks) as bot_clicks, sum(human_clicks) as human_clicks, \
             sum(countries_reached) as countries_reached, sum(device_types) as device_types \
             FROM click_stream_route_performance_mv {} \
             GROUP BY route_id ORDER BY total_clicks DESC LIMIT {} FORMAT JSONEachRow",
            where_clause, limit_clause
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
        let stats: Vec<RoutePerformanceDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(stats)
    }

    async fn get_top_destinations(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>, limit: Option<u32>) -> Result<Vec<crate::dto::clickstream_dto::TopDestinationDto>> {
        use crate::dto::clickstream_dto::TopDestinationDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fd) = from_date {
            conditions.push(format!("date >= '{}'", escape_sql_string(fd)));
        }
        if let Some(td) = to_date {
            conditions.push(format!("date <= '{}'", escape_sql_string(td)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let limit_clause = limit.unwrap_or(20);

        let sql = format!(
            "SELECT dest, sum(total_clicks) as total_clicks, sum(unique_visitors) as unique_visitors \
             FROM click_stream_top_destinations_mv {} \
             GROUP BY dest ORDER BY total_clicks DESC LIMIT {} FORMAT JSONEachRow",
            where_clause, limit_clause
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
        let stats: Vec<TopDestinationDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(stats)
    }

    async fn get_traffic_type_stats(&self, owner_id: Option<&str>, workspace_id: Option<&str>, route_id: Option<&str>, from_hour: Option<&str>, to_hour: Option<&str>) -> Result<Vec<crate::dto::clickstream_dto::TrafficTypeStatsDto>> {
        use crate::dto::clickstream_dto::TrafficTypeStatsDto;

        let mut conditions = Vec::new();
        if let Some(oid) = owner_id {
            conditions.push(format!("owner_id = '{}'", escape_sql_string(oid)));
        }
        if let Some(wid) = workspace_id {
            conditions.push(format!("workspace_id = '{}'", escape_sql_string(wid)));
        }
        if let Some(rid) = route_id {
            conditions.push(format!("route_id = '{}'", escape_sql_string(rid)));
        }
        if let Some(fh) = from_hour {
            conditions.push(format!("hour >= '{}'", escape_sql_string(fh)));
        }
        if let Some(th) = to_hour {
            conditions.push(format!("hour <= '{}'", escape_sql_string(th)));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT is_bot, sum(total_clicks) as total_clicks, sum(unique_ips) as unique_ips \
             FROM click_stream_traffic_type_mv {} \
             GROUP BY is_bot ORDER BY is_bot FORMAT JSONEachRow",
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

        #[derive(Deserialize)]
        struct TrafficRow {
            #[serde(deserialize_with = "deserialize_uint8_as_bool")]
            is_bot: bool,
            total_clicks: u64,
            unique_ips: u64,
        }

        let stats: Vec<TrafficTypeStatsDto> = text
            .lines()
            .filter_map(|line| serde_json::from_str::<TrafficRow>(line).ok())
            .map(|row| TrafficTypeStatsDto {
                is_bot: row.is_bot,
                total_clicks: row.total_clicks,
                unique_ips: row.unique_ips,
            })
            .collect();

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::clickstream::ClickStreamQuery;

    // --- pure helpers (no HTTP) ---

    #[test]
    fn test_escape_sql_string_escapes_quotes_and_backslashes() {
        assert_eq!(escape_sql_string("O'Brien"), "O\\'Brien");
        assert_eq!(escape_sql_string(r"back\slash"), r"back\\slash");
        assert_eq!(escape_sql_string("plain"), "plain");
    }

    #[test]
    fn test_escape_sql_string_neutralizes_injection_attempt() {
        // A naive `owner_id = '{value}'` with this input would close the
        // string literal early and append arbitrary SQL; escaping must
        // prevent that.
        let malicious = "x' OR '1'='1";
        let escaped = escape_sql_string(malicious);
        assert_eq!(escaped, "x\\' OR \\'1\\'=\\'1");
        assert!(!escaped.contains("' OR '"));
    }

    fn store() -> ClickHouseClickStreamStore {
        ClickHouseClickStreamStore::new("http://localhost:8123", "user", "pass", "db").unwrap()
    }

    #[test]
    fn test_build_where_clause_empty_when_no_filters() {
        let store = store();
        let query = ClickStreamQuery::default();
        assert_eq!(store.build_where_clause(&query), "");
    }

    #[test]
    fn test_build_where_clause_combines_filters_with_and() {
        let store = store();
        let mut query = ClickStreamQuery::default();
        query.owner_id = Some("owner-1".to_string());
        query.route_id = Some("route-1".to_string());

        let clause = store.build_where_clause(&query);
        assert_eq!(clause, "WHERE owner_id = 'owner-1' AND route_id = 'route-1'");
    }

    #[test]
    fn test_build_where_clause_escapes_filter_values() {
        let store = store();
        let mut query = ClickStreamQuery::default();
        query.owner_id = Some("x' OR '1'='1".to_string());

        let clause = store.build_where_clause(&query);
        assert!(!clause.contains("' OR '"), "owner_id filter must be escaped: {clause}");
    }

    #[test]
    fn test_row_to_item_converts_nulls_to_unknown_defaults() {
        use crate::model::clickstream::UNKNOWN;

        let row = ClickStreamRow {
            id: "1".to_string(),
            owner_id: "o1".to_string(),
            creator_id: "c1".to_string(),
            route_id: "r1".to_string(),
            workspace_id: "w1".to_string(),
            created: Utc::now(),
            dest: "https://example.com".to_string(),
            ip: "1.2.3.4".to_string(),
            continent: None,
            country: Some("US".to_string()),
            location: None,
            os_family: None,
            os_version: None,
            user_agent_family: None,
            user_agent_version: None,
            device_brand: None,
            device_family: None,
            device_model: None,
            session_first: None,
            session_clicks: None,
            is_unique: true,
            is_bot: false,
        };

        let item = ClickHouseClickStreamStore::row_to_item(row);
        assert_eq!(item.continent, UNKNOWN);
        assert_eq!(item.country, "US");
        assert_eq!(item.session_clicks, 0);
        assert_eq!(item.session_first, crate::model::clickstream::epoch_datetime());
    }

    // --- HTTP behavior against a mock ClickHouse server ---

    /// Distinguishes the row-fetching query from the COUNT(*) query, since
    /// both hit the exact same URL with different `query` params.
    struct QueryContains(&'static str);
    impl wiremock::Match for QueryContains {
        fn matches(&self, request: &wiremock::Request) -> bool {
            request
                .url
                .query_pairs()
                .any(|(key, value)| key == "query" && value.contains(self.0))
        }
    }

    fn test_store(url: String) -> ClickHouseClickStreamStore {
        ClickHouseClickStreamStore::new(&url, "user", "pass", "db").unwrap()
    }

    #[tokio::test]
    async fn test_query_clickstream_parses_rows_and_computes_has_more() {
        let server = wiremock::MockServer::start().await;

        let row = r#"{"id":"1","owner_id":"o1","creator_id":"c1","route_id":"r1","workspace_id":"w1","created":"2024-01-01 00:00:00.000","dest":"https://example.com","ip":"1.2.3.4","continent":null,"country":"US","location":null,"os_family":null,"os_version":null,"user_agent_family":null,"user_agent_version":null,"device_brand":null,"device_family":null,"device_model":null,"session_first":null,"session_clicks":null,"is_unique":1,"is_bot":0}"#;

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(QueryContains("COUNT"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("{\"count\":5}\n"))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(QueryContains("SELECT id"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(format!("{row}\n")))
            .mount(&server)
            .await;

        let store = test_store(server.uri());
        let mut query = ClickStreamQuery::default();
        query.limit = Some(1);
        query.offset = Some(0);

        let response = store.query_clickstream(&query).await.unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].owner_id, "o1");
        assert_eq!(response.total, 5);
        assert!(response.has_more, "offset(0) + limit(1) < total(5)");
    }

    #[tokio::test]
    async fn test_query_clickstream_skips_malformed_rows() {
        let server = wiremock::MockServer::start().await;

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(QueryContains("COUNT"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("{\"count\":0}\n"))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(QueryContains("SELECT id"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("not valid json\n"))
            .mount(&server)
            .await;

        let store = test_store(server.uri());
        let response = store.query_clickstream(&ClickStreamQuery::default()).await.unwrap();
        assert!(response.items.is_empty());
        assert!(!response.has_more);
    }

    #[tokio::test]
    async fn test_count_clickstream_returns_count() {
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("{\"count\":42}\n"))
            .mount(&server)
            .await;

        let store = test_store(server.uri());
        let count = store.count_clickstream(&ClickStreamQuery::default()).await.unwrap();
        assert_eq!(count, 42);
    }
}
