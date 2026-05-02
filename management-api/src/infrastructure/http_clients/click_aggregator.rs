//! Click Aggregator API client for analytics data.

use chrono::{DateTime, Utc};
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

/// Click stream query parameters.
#[derive(Debug, Clone, Serialize)]
pub struct ClickStreamQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Click statistics response.
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

/// Referrer distribution.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReferrerDistribution {
    pub referrer: String,
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

/// Click stream response with all analytics.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClickStreamResponse {
    pub stats: ClickStats,
    pub time_series: Vec<TimeSeriesPoint>,
    pub geo_distribution: Vec<GeoDistribution>,
    pub device_distribution: Vec<DeviceDistribution>,
    pub browser_distribution: Vec<BrowserDistribution>,
    pub referrer_distribution: Vec<ReferrerDistribution>,
    pub top_routes: Vec<TopRoute>,
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

    /// Get click statistics for a route.
    pub async fn get_route_stats(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<ClickStats> {
        let mut url = format!("{}/api/v1/stats/routes/{}", self.base_url, route_id);

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let stats: ClickStats = response.json().await?;
        Ok(stats)
    }

    /// Get time series data for a route.
    pub async fn get_route_time_series(
        &self,
        route_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        interval: &str,
    ) -> anyhow::Result<Vec<TimeSeriesPoint>> {
        let url = format!(
            "{}/api/v1/stats/routes/{}/timeseries?from={}&to={}&interval={}",
            self.base_url,
            route_id,
            from.to_rfc3339(),
            to.to_rfc3339(),
            interval
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let series: Vec<TimeSeriesPoint> = response.json().await?;
        Ok(series)
    }

    /// Get geographic distribution for a route.
    pub async fn get_route_geo(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<GeoDistribution>> {
        let mut url = format!("{}/api/v1/stats/routes/{}/geo", self.base_url, route_id);

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let geo: Vec<GeoDistribution> = response.json().await?;
        Ok(geo)
    }

    /// Get device distribution for a route.
    pub async fn get_route_devices(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<DeviceDistribution>> {
        let mut url = format!("{}/api/v1/stats/routes/{}/devices", self.base_url, route_id);

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let devices: Vec<DeviceDistribution> = response.json().await?;
        Ok(devices)
    }

    /// Get browser distribution for a route.
    pub async fn get_route_browsers(
        &self,
        route_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<BrowserDistribution>> {
        let mut url = format!("{}/api/v1/stats/routes/{}/browsers", self.base_url, route_id);

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let browsers: Vec<BrowserDistribution> = response.json().await?;
        Ok(browsers)
    }

    /// Get workspace statistics.
    pub async fn get_workspace_stats(
        &self,
        workspace_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<ClickStats> {
        let mut url = format!("{}/api/v1/stats/workspaces/{}", self.base_url, workspace_id);

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let stats: ClickStats = response.json().await?;
        Ok(stats)
    }

    /// Get top routes for workspace.
    pub async fn get_workspace_top_routes(
        &self,
        workspace_id: &str,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<TopRoute>> {
        let mut url = format!(
            "{}/api/v1/stats/workspaces/{}/top-routes",
            self.base_url, workspace_id
        );

        let mut params = Vec::new();
        if let Some(f) = from {
            params.push(format!("from={}", f.to_rfc3339()));
        }
        if let Some(t) = to {
            params.push(format!("to={}", t.to_rfc3339()));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Aggregator API error: {} - {}", status, body);
        }

        let routes: Vec<TopRoute> = response.json().await?;
        Ok(routes)
    }

    /// Health check.
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}
