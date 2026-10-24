//! Click Router API client for route propagation.

use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

use crate::domain::entities::{Route, RoutingPolicy};
use crate::settings::ClickRouterSettings;

/// Route DTO for click-router API communication.
/// Uses the format expected by click-router-api.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClickRouterRouteDto {
    pub switch: String,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest: Option<String>,
    pub dest_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,
    pub status: String,
    pub terminal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Convert snake_case keys to camelCase in a JSON value.
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Recursively convert all object keys from snake_case to camelCase.
fn convert_keys_to_camel_case(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let new_map: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| (to_camel_case(&k), convert_keys_to_camel_case(v)))
                .collect();
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(convert_keys_to_camel_case).collect())
        }
        other => other,
    }
}

/// Convert RoutingPolicy to click-router-api's expected format (externally tagged).
/// Management API uses `#[serde(tag = "type")]` but click-router expects externally tagged enums.
fn convert_policy_for_click_router(policy: &RoutingPolicy) -> serde_json::Value {
    match policy {
        RoutingPolicy::Basic => json!("Basic"),
        RoutingPolicy::Conditional { conditions } => {
            json!({ "Conditional": conditions })
        }
        RoutingPolicy::Challenge { challenge } => {
            if let Some(ch) = challenge {
                json!({ "Challenge": ch })
            } else {
                json!("Unknown")
            }
        }
        RoutingPolicy::File { file } => {
            if let Some(f) = file {
                json!({ "File": f })
            } else {
                json!("Unknown")
            }
        }
        RoutingPolicy::Mirroring => json!("Mirroring"),
        RoutingPolicy::Unknown => json!("Unknown"),
    }
}

impl From<&Route> for ClickRouterRouteDto {
    fn from(route: &Route) -> Self {
        Self {
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: format!("{:?}", route.dest_format),
            code: route.code,
            ttl: route.ttl,
            status: route.status.as_str().to_string(),
            terminal: format!("{:?}", route.terminal),
            policy: Some(convert_keys_to_camel_case(convert_policy_for_click_router(&route.policy))),
            properties: serde_json::to_value(&route.properties)
                .ok()
                .map(convert_keys_to_camel_case),
        }
    }
}

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

    /// Build the link format expected by click-router: domain%2Fpath
    fn encode_link(domain: &str, path: &str) -> String {
        format!("{}%2F{}", domain, path)
    }

    /// Upsert a route in the click router using route_id endpoint.
    pub async fn upsert_route(&self, domain: &str, route: &Route) -> anyhow::Result<()> {
        // Use route_id endpoint if available for cleaner updates
        if let Some(ref route_id) = route.properties.route_id {
            let url = format!("{}/v1/routes/{}", self.base_url, route_id);

            // Build DTO with encoded link (domain%2Fpath)
            let mut dto = ClickRouterRouteDto::from(route);
            dto.link = Self::encode_link(domain, &route.link);

            let response = self
                .client
                .put(&url)
                .json(&dto)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Click Router API error: {} - {}", status, body);
            }

            return Ok(());
        }

        // Fallback to switch/domain/path endpoint
        let url = format!(
            "{}/v1/routes/{}/{}/{}",
            self.base_url, route.switch, domain, route.link
        );

        let dto = ClickRouterRouteDto::from(route);

        let response = self
            .client
            .put(&url)
            .json(&dto)
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
            "{}/v1/routes/{}/{}/{}",
            self.base_url, switch, domain, link
        );

        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Click Router API error: {} - {}", status, body);
        }

        Ok(())
    }

    /// Delete a route by route_id.
    pub async fn delete_route_by_id(&self, route_id: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/routes/{}", self.base_url, route_id);

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
            "{}/v1/routes/{}/{}/{}",
            self.base_url, switch, domain, link
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
    pub async fn bulk_upsert(&self, _domain: &str, routes: &[Route]) -> anyhow::Result<()> {
        let url = format!("{}/v1/routes/bulk", self.base_url);

        let dtos: Vec<ClickRouterRouteDto> = routes.iter().map(ClickRouterRouteDto::from).collect();

        let response = self
            .client
            .put(&url)
            .json(&dtos)
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
        let url = format!("{}/public/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}
