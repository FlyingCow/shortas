//! Click Router API client for route propagation.

use reqwest::Client;
use std::time::Duration;

use crate::domain::entities::Route;
use crate::settings::ClickRouterSettings;

/// Click Router API client.
pub struct ClickRouterClient {
    client: Client,
    base_url: String,
}

impl ClickRouterClient {
    /// Create a new Click Router client.
    pub fn new(settings: &ClickRouterSettings) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(settings.timeout_ms))
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
        })
    }

    /// Upsert a route in the click router.
    pub async fn upsert_route(&self, domain: &str, route: &Route) -> anyhow::Result<()> {
        let url = format!(
            "{}/api/v1/domains/{}/routes/{}/{}",
            self.base_url, domain, route.link, route.switch
        );

        let response = self
            .client
            .put(&url)
            .json(&route)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Router API error: {} - {}", status, body);
        }

        Ok(())
    }

    /// Delete a route from the click router.
    pub async fn delete_route(&self, domain: &str, link: &str, switch: &str) -> anyhow::Result<()> {
        let url = format!(
            "{}/api/v1/domains/{}/routes/{}/{}",
            self.base_url, domain, link, switch
        );

        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Router API error: {} - {}", status, body);
        }

        Ok(())
    }

    /// Get a route from the click router.
    pub async fn get_route(&self, domain: &str, link: &str, switch: &str) -> anyhow::Result<Option<Route>> {
        let url = format!(
            "{}/api/v1/domains/{}/routes/{}/{}",
            self.base_url, domain, link, switch
        );

        let response = self.client.get(&url).send().await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Router API error: {} - {}", status, body);
        }

        let route: Route = response.json().await?;
        Ok(Some(route))
    }

    /// Bulk upsert routes.
    pub async fn bulk_upsert(&self, domain: &str, routes: &[Route]) -> anyhow::Result<()> {
        let url = format!("{}/api/v1/domains/{}/routes/bulk", self.base_url, domain);

        let response = self
            .client
            .put(&url)
            .json(&routes)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Router API error: {} - {}", status, body);
        }

        Ok(())
    }

    /// Bulk delete routes.
    pub async fn bulk_delete(&self, domain: &str, routes: &[(String, String)]) -> anyhow::Result<()> {
        for (link, switch) in routes {
            self.delete_route(domain, link, switch).await?;
        }
        Ok(())
    }

    /// Health check.
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}
