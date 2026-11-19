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
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            )
            .build()?;

        Ok(Self {
            client,
            max_image_size,
        })
    }

    pub async fn scrape_favicon(&self, dest_url: &str) -> Result<FaviconResult> {
        let initial_url = Url::parse(dest_url)?;

        // Single GET: fetch page and get final URL after redirects + optional favicon link from HTML
        let (favicon_link, base_url) = self.fetch_page_and_find_favicon_link(&initial_url).await?;

        if let Some(favicon_url) = favicon_link {
            if let Ok(result) = self.fetch_image(&favicon_url).await {
                info!("Found favicon via HTML link: {}", favicon_url);
                return Ok(result);
            }
        }

        // Fallback: /favicon.ico on the origin we actually reached (after redirects)
        let favicon_url = base_url.join("/favicon.ico")?;
        info!("Trying fallback favicon.ico: {}", favicon_url);
        if let Ok(result) = self.fetch_image(favicon_url.as_str()).await {
            return Ok(result);
        }

        // Last resort: Google's favicon service (works for gmail.com and many sites that block or don't expose favicons)
        let host = base_url.host_str().unwrap_or("unknown");
        let google_favicon_url = format!(
            "https://www.google.com/s2/favicons?domain={}&sz=64",
            host
        );
        info!("Trying Google favicon service: {}", google_favicon_url);
        self.fetch_image(&google_favicon_url).await
    }

    /// Fetch the page (following redirects) and return (favicon href if found, final base URL).
    async fn fetch_page_and_find_favicon_link(&self, initial_url: &Url) -> Result<(Option<String>, Url)> {
        let response = self.client.get(initial_url.as_str()).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch page: {}", response.status()));
        }

        // Use final URL after redirects (e.g. gmail.com -> accounts.google.com or mail.google.com)
        let base_url = Url::parse(response.url().as_str())?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Match any link whose rel contains "icon" (covers rel="icon", rel="shortcut icon", rel="apple-touch-icon", etc.)
        let selectors = [
            r#"link[rel~="icon"]"#,
            r#"link[rel~="apple-touch-icon"]"#,
        ];

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    if let Some(href) = element.value().attr("href") {
                        if href.is_empty() || href.starts_with("data:") {
                            continue;
                        }
                        if let Ok(favicon_url) = self.resolve_url(&base_url, href) {
                            debug!("Found favicon link: {} -> {}", selector_str, favicon_url);
                            return Ok((Some(favicon_url), base_url));
                        }
                    }
                }
            }
        }

        Ok((None, base_url))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn scraper() -> FaviconScraper {
        FaviconScraper::new(5, 1_000_000).expect("client should build without network access")
    }

    #[test]
    fn test_resolve_url_absolute_is_returned_as_is() {
        let scraper = scraper();
        let base = Url::parse("https://example.com/path/").unwrap();
        let resolved = scraper.resolve_url(&base, "https://cdn.example.com/favicon.png").unwrap();
        assert_eq!(resolved, "https://cdn.example.com/favicon.png");
    }

    #[test]
    fn test_resolve_url_protocol_relative_uses_base_scheme() {
        let scraper = scraper();
        let base = Url::parse("https://example.com/").unwrap();
        let resolved = scraper.resolve_url(&base, "//cdn.example.com/favicon.png").unwrap();
        assert_eq!(resolved, "https://cdn.example.com/favicon.png");
    }

    #[test]
    fn test_resolve_url_relative_joins_against_base() {
        let scraper = scraper();
        let base = Url::parse("https://example.com/sub/page.html").unwrap();
        let resolved = scraper.resolve_url(&base, "/favicon.png").unwrap();
        assert_eq!(resolved, "https://example.com/favicon.png");

        let resolved = scraper.resolve_url(&base, "favicon.png").unwrap();
        assert_eq!(resolved, "https://example.com/sub/favicon.png");
    }

    #[test]
    fn test_resolve_url_rejects_data_urls() {
        let scraper = scraper();
        let base = Url::parse("https://example.com/").unwrap();
        let result = scraper.resolve_url(&base, "data:image/png;base64,abcd");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_image_type_accepts_known_image_types() {
        let scraper = scraper();
        assert!(scraper.is_valid_image_type("image/png"));
        assert!(scraper.is_valid_image_type("image/x-icon"));
        assert!(scraper.is_valid_image_type("image/svg+xml"));
        assert!(scraper.is_valid_image_type("image/webp"));
    }

    #[test]
    fn test_is_valid_image_type_rejects_non_image_types() {
        let scraper = scraper();
        assert!(!scraper.is_valid_image_type("text/html"));
        assert!(!scraper.is_valid_image_type("application/json"));
        assert!(!scraper.is_valid_image_type(""));
    }

    // --- scrape_favicon against a mock HTTP server ---
    //
    // NB: on exhaustion, `scrape_favicon` falls through to a hardcoded
    // `https://www.google.com/s2/favicons` request with no way to redirect
    // that to a mock server, so every test here is written to resolve via
    // either the HTML <link> or the /favicon.ico fallback - never reaching
    // that branch (which would hit the real network).

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_scrape_favicon_finds_link_in_html() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"<html><head><link rel="icon" href="/static/favicon.png"></head></html>"#,
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/static/favicon.png"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(vec![1, 2, 3, 4]),
            )
            .mount(&server)
            .await;

        let scraper = scraper();
        let result = scraper.scrape_favicon(&server.uri()).await.unwrap();
        assert_eq!(result.content_type, "image/png");
        assert_eq!(result.data, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_scrape_favicon_falls_back_to_favicon_ico_when_no_link_in_html() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<html><head></head></html>"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/favicon.ico"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/x-icon")
                    .set_body_bytes(vec![9, 9, 9]),
            )
            .mount(&server)
            .await;

        let scraper = scraper();
        let result = scraper.scrape_favicon(&server.uri()).await.unwrap();
        assert_eq!(result.content_type, "image/x-icon");
        assert_eq!(result.data, vec![9, 9, 9]);
    }

    #[tokio::test]
    async fn test_scrape_favicon_errors_when_initial_page_fetch_fails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let scraper = scraper();
        let result = scraper.scrape_favicon(&server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_scrape_favicon_skips_data_url_links_and_falls_back() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"<html><head><link rel="icon" href="data:image/png;base64,abcd"></head></html>"#,
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/favicon.ico"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/x-icon")
                    .set_body_bytes(vec![7, 7]),
            )
            .mount(&server)
            .await;

        let scraper = scraper();
        let result = scraper.scrape_favicon(&server.uri()).await.unwrap();
        assert_eq!(result.data, vec![7, 7]);
    }
}
