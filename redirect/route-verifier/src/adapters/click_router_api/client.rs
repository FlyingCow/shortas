use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use tracing::{error, info};

use crate::settings::ClickRouterApiSettings;

#[derive(Clone)]
pub struct ClickRouterApiClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize)]
struct RouteStatusUpdate {
    status: RouteStatusDto,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "reason")]
enum RouteStatusDto {
    Active,
    Blocked(String),
}

impl ClickRouterApiClient {
    pub fn new(settings: &ClickRouterApiSettings) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(settings.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: settings.base_url.clone(),
        }
    }

    /// Block a route with a specific reason via click-router-api.
    /// This will update the route in MongoDB and trigger cache invalidation via RabbitMQ.
    pub async fn block_route(&self, route_id: &str, reason: &str) -> Result<()> {
        let url = format!("{}/routes/{}", self.base_url, route_id);

        let update = RouteStatusUpdate {
            status: RouteStatusDto::Blocked(reason.to_string()),
        };

        let response = self
            .client
            .put(&url)
            .json(&update)
            .send()
            .await?;

        if response.status().is_success() {
            info!(
                "Successfully blocked route {} with reason: {}",
                route_id, reason
            );
            Ok(())
        } else {
            let status_code = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(
                "Failed to block route {}: {} - {}",
                route_id, status_code, body
            );
            Err(anyhow::anyhow!(
                "Failed to block route: {} - {}",
                status_code,
                body
            ))
        }
    }

    /// Unblock a route (set status to Active).
    pub async fn unblock_route(&self, route_id: &str) -> Result<()> {
        let url = format!("{}/routes/{}", self.base_url, route_id);

        let update = RouteStatusUpdate {
            status: RouteStatusDto::Active,
        };

        let response = self
            .client
            .put(&url)
            .json(&update)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Successfully unblocked route {}", route_id);
            Ok(())
        } else {
            let status_code = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(
                "Failed to unblock route {}: {} - {}",
                route_id, status_code, body
            );
            Err(anyhow::anyhow!(
                "Failed to unblock route: {} - {}",
                status_code,
                body
            ))
        }
    }
}
