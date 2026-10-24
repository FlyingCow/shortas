//! Click Aggregator API client for analytics data.

use chrono::{DateTime, NaiveDate, Utc};
use reqwest::Client;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::settings::ClickAggregatorSettings;

/// Click Aggregator API client.
pub struct ClickAggregatorClient {
    client: Client,
    base_url: String,
}

// ============================================================================
// Response DTOs matching click-aggregator-api
// ============================================================================

/// Daily statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DailyStats {
    pub date: String,
    pub total_clicks: i64,
    pub unique_clicks: i64,
    pub bot_clicks: i64,
    pub human_clicks: i64,
    pub unique_ips: i64,
}

/// Hourly statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HourlyStats {
    pub hour: String,
    pub total_clicks: i64,
    pub unique_clicks: i64,
    pub bot_clicks: i64,
    pub human_clicks: i64,
    pub unique_ips: i64,
}

/// Geographic statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeographicStats {
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub total_clicks: i64,
    pub unique_clicks: i64,
    pub unique_ips: i64,
}

/// Device statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceStats {
    pub device_family: Option<String>,
    pub os_family: Option<String>,
    pub total_clicks: i64,
    pub unique_clicks: i64,
}

/// Browser statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BrowserStats {
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub total_clicks: i64,
    pub unique_clicks: i64,
}

/// Route performance statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoutePerformance {
    pub route_id: String,
    pub route_name: Option<String>,
    pub route_domain_name: Option<String>,
    pub total_clicks: i64,
    pub unique_visitors: i64,
    pub bot_clicks: i64,
    pub human_clicks: i64,
    pub countries_reached: i64,
    pub device_types: i64,
}

/// Top destination statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopDestination {
    pub dest: String,
    pub total_clicks: i64,
    pub unique_visitors: i64,
}

/// Traffic type statistics from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TrafficTypeStats {
    pub is_bot: bool,
    pub total_clicks: i64,
    pub unique_ips: i64,
}

/// Clickstream item from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamItem {
    pub id: String,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub route_name: Option<String>,
    pub route_domain_name: Option<String>,
    pub workspace_id: Option<String>,
    pub created: String,
    pub dest: Option<String>,
    pub ip: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    pub session_first: Option<String>,
    pub session_clicks: Option<i64>,
    pub is_unique: Option<bool>,
    pub is_bot: Option<bool>,
}

/// Clickstream response from click-aggregator-api.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamResponse {
    pub items: Vec<ClickStreamItem>,
    pub total: i64,
    pub offset: u32,
    pub limit: u32,
    pub has_more: bool,
}

// ============================================================================
// Legacy DTOs for backwards compatibility with clickstream controller
// ============================================================================

/// Click statistics response (aggregated from daily/hourly stats).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClickStats {
    pub total_clicks: i64,
    pub unique_clicks: i64,
    pub qr_scans: i64,
}

/// Time series data point.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimeSeriesPoint {
    pub timestamp: String,
    pub clicks: i64,
    pub unique_clicks: i64,
}

/// Geographic distribution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeoDistribution {
    pub country: String,
    pub clicks: i64,
    pub percentage: f64,
}

/// Device distribution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceDistribution {
    pub device_type: String,
    pub clicks: i64,
    pub percentage: f64,
}

/// Browser distribution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BrowserDistribution {
    pub browser: String,
    pub clicks: i64,
    pub percentage: f64,
}

/// Top routes by clicks.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TopRoute {
    pub route_id: String,
    pub link: String,
    pub clicks: i64,
}

impl ClickAggregatorClient {
    /// Create a new Click Aggregator client.
    pub fn new(settings: &ClickAggregatorSettings) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(settings.timeout_ms))
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
        })
    }

    // ========================================================================
    // Direct API methods (matching click-aggregator-api endpoints)
    // ========================================================================

    /// Get daily statistics.
    pub async fn get_daily_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> anyhow::Result<Vec<DailyStats>> {
        let mut url = format!("{}/v1/stats/daily", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get hourly statistics.
    pub async fn get_hourly_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_hour: Option<DateTime<Utc>>,
        to_hour: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<HourlyStats>> {
        let mut url = format!("{}/v1/stats/hourly", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(dt) = from_hour {
            params.push(format!("from_hour={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        if let Some(dt) = to_hour {
            params.push(format!("to_hour={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get geographic statistics.
    pub async fn get_geographic_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> anyhow::Result<Vec<GeographicStats>> {
        let mut url = format!("{}/v1/stats/geographic", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get device statistics.
    pub async fn get_device_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> anyhow::Result<Vec<DeviceStats>> {
        let mut url = format!("{}/v1/stats/devices", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get browser statistics.
    pub async fn get_browser_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> anyhow::Result<Vec<BrowserStats>> {
        let mut url = format!("{}/v1/stats/browsers", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get route performance statistics.
    pub async fn get_route_performance(
        &self,
        owner_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<RoutePerformance>> {
        let mut url = format!("{}/v1/stats/route-performance", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get top destinations.
    pub async fn get_top_destinations(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<TopDestination>> {
        let mut url = format!("{}/v1/stats/top-destinations", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(d) = from_date {
            params.push(format!("from_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(d) = to_date {
            params.push(format!("to_date={}", d.format("%Y-%m-%d")));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get traffic type statistics.
    pub async fn get_traffic_type_stats(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_hour: Option<DateTime<Utc>>,
        to_hour: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<TrafficTypeStats>> {
        let mut url = format!("{}/v1/stats/traffic-types", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(dt) = from_hour {
            params.push(format!("from_hour={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        if let Some(dt) = to_hour {
            params.push(format!("to_hour={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    /// Get raw clickstream data.
    pub async fn get_clickstream(
        &self,
        owner_id: Option<&str>,
        creator_id: Option<&str>,
        route_id: Option<&str>,
        workspace_id: Option<&str>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> anyhow::Result<ClickStreamResponse> {
        let mut url = format!("{}/v1/clickstream", self.base_url);
        let mut params = Vec::new();

        if let Some(id) = owner_id {
            params.push(format!("owner_id={}", id));
        }
        if let Some(id) = creator_id {
            params.push(format!("creator_id={}", id));
        }
        if let Some(id) = route_id {
            params.push(format!("route_id={}", id));
        }
        if let Some(id) = workspace_id {
            params.push(format!("workspace_id={}", id));
        }
        if let Some(dt) = from {
            params.push(format!("created_from={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        if let Some(dt) = to {
            params.push(format!("created_to={}", dt.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    // ========================================================================
    // Legacy methods for backwards compatibility with clickstream controller
    // ========================================================================

    /// Get click statistics for a route (legacy method).
    pub async fn get_route_stats(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<ClickStats> {
        let from_date = from.map(|dt| dt.date_naive());
        let to_date = to.map(|dt| dt.date_naive());

        let daily_stats = self
            .get_daily_stats(None, Some(route_id), from_date, to_date)
            .await?;

        let total_clicks: i64 = daily_stats.iter().map(|s| s.total_clicks).sum();
        let unique_clicks: i64 = daily_stats.iter().map(|s| s.unique_clicks).sum();

        // QR scans not available from daily stats, would need clickstream query
        Ok(ClickStats {
            total_clicks,
            unique_clicks,
            qr_scans: 0,
        })
    }

    /// Get time series data for a route (legacy method).
    pub async fn get_route_time_series(
        &self,
        route_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        interval: &str,
    ) -> anyhow::Result<Vec<TimeSeriesPoint>> {
        match interval {
            "hour" => {
                let hourly = self
                    .get_hourly_stats(None, Some(route_id), Some(from), Some(to))
                    .await?;
                Ok(hourly
                    .into_iter()
                    .map(|h| TimeSeriesPoint {
                        timestamp: h.hour,
                        clicks: h.total_clicks,
                        unique_clicks: h.unique_clicks,
                    })
                    .collect())
            }
            _ => {
                let daily = self
                    .get_daily_stats(
                        None,
                        Some(route_id),
                        Some(from.date_naive()),
                        Some(to.date_naive()),
                    )
                    .await?;
                Ok(daily
                    .into_iter()
                    .map(|d| TimeSeriesPoint {
                        timestamp: d.date,
                        clicks: d.total_clicks,
                        unique_clicks: d.unique_clicks,
                    })
                    .collect())
            }
        }
    }

    /// Get geographic distribution for a route (legacy method).
    pub async fn get_route_geo(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<GeoDistribution>> {
        let from_date = from.map(|dt| dt.date_naive());
        let to_date = to.map(|dt| dt.date_naive());

        let geo_stats = self
            .get_geographic_stats(None, Some(route_id), from_date, to_date)
            .await?;

        let total: i64 = geo_stats.iter().map(|g| g.total_clicks).sum();
        let mut result: Vec<GeoDistribution> = geo_stats
            .into_iter()
            .map(|g| GeoDistribution {
                country: g.country.unwrap_or_else(|| "Unknown".to_string()),
                clicks: g.total_clicks,
                percentage: if total > 0 {
                    (g.total_clicks as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();

        if let Some(l) = limit {
            result.truncate(l as usize);
        }

        Ok(result)
    }

    /// Get device distribution for a route (legacy method).
    pub async fn get_route_devices(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<DeviceDistribution>> {
        let from_date = from.map(|dt| dt.date_naive());
        let to_date = to.map(|dt| dt.date_naive());

        let device_stats = self
            .get_device_stats(None, Some(route_id), from_date, to_date)
            .await?;

        let total: i64 = device_stats.iter().map(|d| d.total_clicks).sum();
        Ok(device_stats
            .into_iter()
            .map(|d| DeviceDistribution {
                device_type: d.device_family.unwrap_or_else(|| "Unknown".to_string()),
                clicks: d.total_clicks,
                percentage: if total > 0 {
                    (d.total_clicks as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect())
    }

    /// Get browser distribution for a route (legacy method).
    pub async fn get_route_browsers(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<BrowserDistribution>> {
        let from_date = from.map(|dt| dt.date_naive());
        let to_date = to.map(|dt| dt.date_naive());

        let browser_stats = self
            .get_browser_stats(None, Some(route_id), from_date, to_date)
            .await?;

        let total: i64 = browser_stats.iter().map(|b| b.total_clicks).sum();
        Ok(browser_stats
            .into_iter()
            .map(|b| BrowserDistribution {
                browser: b.user_agent_family.unwrap_or_else(|| "Unknown".to_string()),
                clicks: b.total_clicks,
                percentage: if total > 0 {
                    (b.total_clicks as f64 / total as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect())
    }

    /// Get workspace statistics (legacy method).
    pub async fn get_workspace_stats(
        &self,
        workspace_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<ClickStats> {
        // Workspace stats require querying clickstream with workspace_id filter
        let response = self
            .get_clickstream(
                None,
                None,
                None,
                Some(workspace_id),
                from,
                to,
                Some(1),
                None,
            )
            .await?;

        // For now return total from clickstream count
        Ok(ClickStats {
            total_clicks: response.total,
            unique_clicks: response.total, // Would need distinct counting
            qr_scans: 0,
        })
    }

    /// Get top routes for workspace (legacy method).
    pub async fn get_workspace_top_routes(
        &self,
        _workspace_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<TopRoute>> {
        let from_date = from.map(|dt| dt.date_naive());
        let to_date = to.map(|dt| dt.date_naive());

        let performance = self
            .get_route_performance(None, from_date, to_date, limit.map(|l| l as u32))
            .await?;

        Ok(performance
            .into_iter()
            .map(|p| TopRoute {
                route_id: p.route_id,
                link: p.route_name.unwrap_or_default(),
                clicks: p.total_clicks,
            })
            .collect())
    }

    /// Health check.
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let url = format!("{}/public/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }

    // ========================================================================
    // Helper methods
    // ========================================================================

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> anyhow::Result<T> {
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }
        Ok(response.json().await?)
    }
}
