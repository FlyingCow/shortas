use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;
use tracing::{debug, info, warn};
use url::Url;

pub struct FaviconScraper {
    client: Client,
    max_image_size: usize,
}

pub struct FaviconResult {
    pub data: Vec<u8>,
    pub content_type: String,
}

impl FaviconScraper {
    pub fn new(timeout_seconds: u64, max_image_size: usize) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .user_agent("Mozilla/5.0 (compatible; RouteIconWorker/1.0)")
            .build()?;

        Ok(Self {
            client,
            max_image_size,
        })
    }

    pub async fn scrape_favicon(&self, dest_url: &str) -> Result<FaviconResult> {
        let base_url = Url::parse(dest_url)?;

        // Try to find favicon link in HTML first
        if let Ok(favicon_url) = self.find_favicon_in_html(&base_url).await {
            if let Ok(result) = self.fetch_image(&favicon_url).await {
                info!("Found favicon via HTML link: {}", favicon_url);
                return Ok(result);
            }
        }

        // Fallback to /favicon.ico
        let favicon_url = base_url.join("/favicon.ico")?;
        info!("Trying fallback favicon.ico: {}", favicon_url);
        self.fetch_image(favicon_url.as_str()).await
    }

    async fn find_favicon_in_html(&self, base_url: &Url) -> Result<String> {
        let response = self.client.get(base_url.as_str()).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch page: {}", response.status()));
        }

        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Try different favicon selectors in order of preference
        let selectors = [
            r#"link[rel="icon"]"#,
            r#"link[rel="shortcut icon"]"#,
            r#"link[rel="apple-touch-icon"]"#,
            r#"link[rel="apple-touch-icon-precomposed"]"#,
        ];

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(href) = element.value().attr("href") {
                        let favicon_url = self.resolve_url(base_url, href)?;
                        debug!("Found favicon link: {} -> {}", selector_str, favicon_url);
                        return Ok(favicon_url);
                    }
                }
            }
        }

        Err(anyhow!("No favicon link found in HTML"))
    }

    fn resolve_url(&self, base_url: &Url, href: &str) -> Result<String> {
        // Handle data URLs
        if href.starts_with("data:") {
            return Err(anyhow!("Data URLs not supported"));
        }

        // Handle absolute URLs
        if href.starts_with("http://") || href.starts_with("https://") {
            return Ok(href.to_string());
        }

        // Handle protocol-relative URLs
        if href.starts_with("//") {
            return Ok(format!("{}:{}", base_url.scheme(), href));
        }

        // Handle relative URLs
        let resolved = base_url.join(href)?;
        Ok(resolved.to_string())
    }

    async fn fetch_image(&self, url: &str) -> Result<FaviconResult> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch image: {}", response.status()));
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(';').next().unwrap_or(s).trim().to_string())
            .unwrap_or_else(|| "image/x-icon".to_string());

        // Validate content type
        if !self.is_valid_image_type(&content_type) {
            warn!("Invalid content type for favicon: {}", content_type);
            return Err(anyhow!("Invalid content type: {}", content_type));
        }

        let bytes = response.bytes().await?;

        if bytes.len() > self.max_image_size {
            return Err(anyhow!(
                "Image too large: {} bytes (max: {})",
                bytes.len(),
                self.max_image_size
            ));
        }

        if bytes.is_empty() {
            return Err(anyhow!("Empty image response"));
        }

        Ok(FaviconResult {
            data: bytes.to_vec(),
            content_type,
        })
    }

    fn is_valid_image_type(&self, content_type: &str) -> bool {
        matches!(
            content_type,
            "image/x-icon"
                | "image/vnd.microsoft.icon"
                | "image/ico"
                | "image/icon"
                | "image/png"
                | "image/gif"
                | "image/jpeg"
                | "image/jpg"
                | "image/svg+xml"
                | "image/webp"
        )
    }
}
