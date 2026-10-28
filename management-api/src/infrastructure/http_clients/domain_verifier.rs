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

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_client(base_url: String) -> DomainVerifierClient {
        DomainVerifierClient::new(&DomainVerifierSettings { base_url, timeout_ms: 5000 }).unwrap()
    }

    #[tokio::test]
    async fn test_register_domain_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/domains"))
            .respond_with(ResponseTemplate::new(201))
            .expect(1)
            .mount(&server)
            .await;

        let client = test_client(server.uri());
        client.register_domain("d1", "example.com", "owner-1").await.unwrap();
    }

    #[tokio::test]
    async fn test_register_domain_treats_conflict_as_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/domains"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = test_client(server.uri());
        let result = client.register_domain("d1", "example.com", "owner-1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_domain_errors_on_other_failure() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .mount(&server)
            .await;

        let client = test_client(server.uri());
        let err = client.register_domain("d1", "example.com", "owner-1").await.unwrap_err();
        assert!(err.to_string().contains("500"));
    }

    #[tokio::test]
    async fn test_trigger_verification_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/domains/d1/verify"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let client = test_client(server.uri());
        client.trigger_verification("d1").await.unwrap();
    }

    #[tokio::test]
    async fn test_trigger_verification_errors_when_domain_unregistered() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = test_client(server.uri());
        let err = client.trigger_verification("unknown").await.unwrap_err();
        assert!(err.to_string().contains("404"));
    }
}
