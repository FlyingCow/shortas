use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use url::Url;

use crate::settings::SafeBrowsingSettings;

#[derive(Clone)]
pub struct SafeBrowsingClient {
    client: Client,
    base_url: String,
}

/// Response format from gglsbl-rest API
#[derive(Debug, Deserialize)]
struct GglsblResponse {
    #[serde(default)]
    matches: Option<Vec<GglsblMatch>>,
    #[serde(default)]
    url: Option<String>,
}

/// Match entry from gglsbl-rest
#[derive(Debug, Deserialize)]
struct GglsblMatch {
    platform: String,
    threat: String,
    threat_entry: String,
}

#[derive(Debug, Clone)]
pub struct ThreatMatch {
    pub threat_type: String,
    pub platform_type: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SafeBrowsingResult {
    pub is_safe: bool,
    pub threats: Vec<ThreatMatch>,
    pub checked_url: Option<String>,
}

impl SafeBrowsingResult {
    pub fn safe() -> Self {
        Self {
            is_safe: true,
            threats: Vec::new(),
            checked_url: None,
        }
    }

    pub fn unsafe_with_threats(threats: Vec<ThreatMatch>, url: Option<String>) -> Self {
        Self {
            is_safe: false,
            threats,
            checked_url: url,
        }
    }

    pub fn first_threat_type(&self) -> Option<&str> {
        self.threats.first().map(|t| t.threat_type.as_str())
    }

    pub fn first_threat_url(&self) -> Option<&str> {
        self.checked_url.as_deref().or_else(|| {
            self.threats.first().map(|t| t.url.as_str())
        })
    }
}

impl SafeBrowsingClient {
    pub fn new(settings: &SafeBrowsingSettings) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(settings.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: settings.base_url.clone(),
        }
    }

    /// Check if a URL is safe according to Google Safe Browsing.
    /// Returns SafeBrowsingResult indicating if the URL is safe and any detected threats.
    pub async fn check_url(&self, url: &str) -> Result<SafeBrowsingResult> {
        // Validate URL first
        if let Err(e) = Url::parse(url) {
            warn!("Invalid URL for safe browsing check: {} - {}", url, e);
            // Consider invalid URLs as safe (we can't check them anyway)
            return Ok(SafeBrowsingResult::safe());
        }

        // Build the lookup URL with properly percent-encoded URL parameter
        let base_url = self.base_url.trim_end_matches('/');
        // Encode the URL to ensure all special chars (including :) are percent-encoded
        let encoded_url = urlencoding::encode(url);
        let lookup_url = Url::parse(&format!("{}/gglsbl/v1/lookup/{}", base_url, encoded_url))?;

        debug!("Checking URL against Safe Browsing: {} -> {}", url, lookup_url);

        let response = match self.client.get(lookup_url).send().await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to connect to Safe Browsing service: {}", e);
                // On connection error, assume safe to avoid blocking legitimate traffic
                return Ok(SafeBrowsingResult::safe());
            }
        };

        let status = response.status();

        if status == reqwest::StatusCode::NOT_FOUND {
            // 404 means URL is not in threat lists (safe)
            debug!("URL {} is safe (not in threat lists)", url);
            return Ok(SafeBrowsingResult::safe());
        }

        if !status.is_success() {
            warn!(
                "Safe Browsing API returned error status {} for URL: {}",
                status, url
            );
            // On API error, assume safe to avoid blocking legitimate traffic
            return Ok(SafeBrowsingResult::safe());
        }

        // Parse the response
        let body = response.text().await?;

        if body.is_empty() || body == "null" {
            // Empty response means safe
            return Ok(SafeBrowsingResult::safe());
        }

        match serde_json::from_str::<GglsblResponse>(&body) {
            Ok(gglsbl_response) => {
                if let Some(matches) = gglsbl_response.matches {
                    if !matches.is_empty() {
                        let threats: Vec<ThreatMatch> = matches
                            .into_iter()
                            .map(|m| ThreatMatch {
                                threat_type: m.threat,
                                platform_type: m.platform,
                                url: gglsbl_response.url.clone().unwrap_or_else(|| url.to_string()),
                            })
                            .collect();

                        info!(
                            "URL {} flagged as unsafe: {} threats detected ({:?})",
                            url,
                            threats.len(),
                            threats.first().map(|t| &t.threat_type)
                        );
                        return Ok(SafeBrowsingResult::unsafe_with_threats(
                            threats,
                            gglsbl_response.url.clone(),
                        ));
                    }
                }
                Ok(SafeBrowsingResult::safe())
            }
            Err(e) => {
                warn!("Failed to parse Safe Browsing response: {} - body: {}", e, body);
                // On parse error, assume safe
                Ok(SafeBrowsingResult::safe())
            }
        }
    }

    /// Check multiple URLs and return the first unsafe result, if any.
    pub async fn check_urls(&self, urls: &[String]) -> Result<SafeBrowsingResult> {
        for url in urls {
            let result = self.check_url(url).await?;
            if !result.is_safe {
                return Ok(result);
            }
        }
        Ok(SafeBrowsingResult::safe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_browsing_result_safe() {
        let result = SafeBrowsingResult::safe();
        assert!(result.is_safe);
        assert!(result.threats.is_empty());
        assert!(result.first_threat_type().is_none());
    }

    #[test]
    fn test_safe_browsing_result_unsafe() {
        let threats = vec![ThreatMatch {
            threat_type: "MALWARE".to_string(),
            platform_type: "ANY_PLATFORM".to_string(),
            url: "https://malware.example.com".to_string(),
        }];

        let result = SafeBrowsingResult::unsafe_with_threats(threats, Some("https://malware.example.com".to_string()));
        assert!(!result.is_safe);
        assert_eq!(result.threats.len(), 1);
        assert_eq!(result.first_threat_type(), Some("MALWARE"));
        assert_eq!(
            result.first_threat_url(),
            Some("https://malware.example.com")
        );
    }

    #[test]
    fn test_parse_gglsbl_response() {
        let json = r#"{
            "matches": [
                {"platform": "WINDOWS", "threat": "MALWARE", "threat_entry": "URL"},
                {"platform": "LINUX", "threat": "MALWARE", "threat_entry": "URL"}
            ],
            "url": "http://example.com/malware"
        }"#;

        let response: GglsblResponse = serde_json::from_str(json).unwrap();
        assert!(response.matches.is_some());
        assert_eq!(response.matches.as_ref().unwrap().len(), 2);
        assert_eq!(response.url, Some("http://example.com/malware".to_string()));
    }
}
