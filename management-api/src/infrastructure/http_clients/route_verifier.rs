//! Route Verifier API client.
//!
//! Registers routes so route-verifier's background worker can keep checking
//! their destinations against Google Safe Browsing and block them if flagged.

use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

use crate::settings::RouteVerifierSettings;

#[derive(Debug, Clone, Serialize)]
struct CreateRouteRequest {
    id: String,
    link: String,
    destinations: Vec<String>,
    owner_id: Option<String>,
    workspace_id: Option<String>,
}

/// Route Verifier API client.
pub struct RouteVerifierClient {
    client: Client,
    base_url: String,
}

impl RouteVerifierClient {
    /// Create a new Route Verifier client.
    pub fn new(settings: &RouteVerifierSettings) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(settings.timeout_ms))
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
        })
    }

    /// Register (or update) a route for ongoing Safe Browsing verification.
    pub async fn register_route(
        &self,
        route_id: &str,
        link: &str,
        destinations: Vec<String>,
        owner_id: Option<String>,
        workspace_id: Option<String>,
    ) -> anyhow::Result<()> {
        if destinations.is_empty() {
            return Ok(());
        }

        let url = format!("{}/v1/routes", self.base_url);
        let body = CreateRouteRequest {
            id: route_id.to_string(),
            link: link.to_string(),
            destinations,
            owner_id,
            workspace_id,
        };

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Route Verifier API error: {} - {}", status, text);
        }

        Ok(())
    }

    /// Remove a route from ongoing verification (e.g. the route was deleted).
    pub async fn remove_route(&self, route_id: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/routes/{}", self.base_url, route_id);
        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Route Verifier API error: {} - {}", status, text);
        }

        Ok(())
    }
}
