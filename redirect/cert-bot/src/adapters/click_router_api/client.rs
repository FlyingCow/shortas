use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::settings::ClickRouterApiSettings;

#[derive(Clone)]
pub struct ClickRouterApiClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize)]
struct ChallengeRequest {
    key_authorization: String,
}

#[derive(Serialize)]
struct CertificateRequest {
    key: String,
    cert: String,
    ocsp_resp: String,
}

#[derive(Deserialize)]
pub struct CertificateResponse {
    pub key: String,
    pub cert: String,
    pub ocsp_resp: String,
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
}

/// Information about a certificate that needs renewal
#[derive(Debug, Clone)]
pub struct CertificateExpiryInfo {
    pub domain: String,
    pub owner_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl ClickRouterApiClient {
    pub fn new(settings: &ClickRouterApiSettings) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(settings.timeout_seconds))
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
        })
    }

    /// Store an ACME challenge for a domain
    pub async fn store_challenge(
        &self,
        domain: &str,
        token: &str,
        key_authorization: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/v1/challenges/{}/{}",
            self.base_url, domain, token
        );

        let request = ChallengeRequest {
            key_authorization: key_authorization.to_string(),
        };

        let response = self.client.put(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to store challenge: {} - {}", status, body);
        }

        Ok(())
    }

    /// Delete all challenges for a domain
    pub async fn delete_domain_challenges(&self, domain: &str) -> Result<()> {
        let url = format!("{}/v1/challenges/{}", self.base_url, domain);

        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to delete challenges: {} - {}", status, body);
        }

        Ok(())
    }

    /// Store a certificate for a domain
    pub async fn store_certificate(
        &self,
        domain: &str,
        key: &str,
        cert: &str,
    ) -> Result<()> {
        let url = format!("{}/v1/certificates/{}", self.base_url, domain);

        let request = CertificateRequest {
            key: key.to_string(),
            cert: cert.to_string(),
            ocsp_resp: String::new(),
        };

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            // Try PUT if POST fails (certificate might already exist)
            let response = self.client.put(&url).json(&request).send().await?;
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                anyhow::bail!("Failed to store certificate: {} - {}", status, body);
            }
        }

        Ok(())
    }

    /// Get a certificate for a domain
    pub async fn get_certificate(&self, domain: &str) -> Result<Option<CertificateResponse>> {
        let url = format!("{}/v1/certificates/{}", self.base_url, domain);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(Some(response.json().await?))
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get certificate: {} - {}", status, body);
        }
    }

    /// Get all certificates that expire before the given date
    /// This requires a new endpoint in click-router-api
    pub async fn get_certificates_expiring_before(
        &self,
        before: DateTime<Utc>,
    ) -> Result<Vec<CertificateExpiryInfo>> {
        let url = format!(
            "{}/v1/certificates?expires_before={}",
            self.base_url,
            before.timestamp()
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            #[derive(Deserialize)]
            struct ExpiringCertificatesResponse {
                certificates: Vec<CertificateExpiryRecord>,
            }

            #[derive(Deserialize)]
            struct CertificateExpiryRecord {
                domain: String,
                owner_id: Option<String>,
                expires_at: Option<DateTime<Utc>>,
            }

            let resp: ExpiringCertificatesResponse = response.json().await?;
            Ok(resp
                .certificates
                .into_iter()
                .map(|c| CertificateExpiryInfo {
                    domain: c.domain,
                    owner_id: c.owner_id,
                    expires_at: c.expires_at,
                })
                .collect())
        } else if response.status() == reqwest::StatusCode::NOT_FOUND {
            // Endpoint might not exist yet, return empty
            Ok(vec![])
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to get expiring certificates: {} - {}", status, body);
        }
    }
}
