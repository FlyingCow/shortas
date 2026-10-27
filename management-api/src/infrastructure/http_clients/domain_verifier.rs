//! Domain Verifier API client.
//!
//! Registers domains so domain-verifier's background worker can keep
//! checking their DNS records (TXT ownership proof + A/AAAA pointing at
//! the right IPs) and reports status back over RabbitMQ.

use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::time::Duration;

use crate::settings::DomainVerifierSettings;

#[derive(Debug, Clone, Serialize)]
struct CreateDomainRequest {
    id: String,
    name: String,
    owner_id: String,
}

/// Domain Verifier API client.
pub struct DomainVerifierClient {
    client: Client,
    base_url: String,
}

impl DomainVerifierClient {
    /// Create a new Domain Verifier client.
    pub fn new(settings: &DomainVerifierSettings) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_millis(settings.timeout_ms))
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
        })
    }

    /// Register a domain for ongoing DNS verification. Idempotent: domain-verifier
    /// keys the conflict check on (name, owner_id), so calling this again for a
    /// domain it already knows about is a no-op, not an error.
    pub async fn register_domain(&self, id: &str, name: &str, owner_id: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/domains", self.base_url);
        let body = CreateDomainRequest {
            id: id.to_string(),
            name: name.to_string(),
            owner_id: owner_id.to_string(),
        };

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() && response.status() != StatusCode::CONFLICT {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Domain Verifier API error: {} - {}", status, text);
        }

        Ok(())
    }

    /// Trigger an immediate DNS verification check instead of waiting for
    /// domain-verifier's next poll cycle. The domain must already be
    /// registered (see `register_domain`) or this 404s.
    pub async fn trigger_verification(&self, id: &str) -> anyhow::Result<()> {
        let url = format!("{}/v1/domains/{}/verify", self.base_url, id);
        let response = self.client.post(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Domain Verifier API error: {} - {}", status, text);
        }

        Ok(())
    }
}
