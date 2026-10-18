#![cfg_attr(docsrs, feature(doc_cfg))]

use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::sync::Arc;

use axum::body::Body;
use hyper::upgrade::OnUpgrade;
use percent_encoding::{utf8_percent_encode, CONTROLS};
use http::header::{HeaderMap, HeaderName, HeaderValue, CONNECTION, HOST, UPGRADE};
use http::uri::Uri;
use http::{Request as HttpRequest, Response as HttpResponse, StatusCode};

pub mod hyper_client;

type HyperRequest = hyper::Request<Body>;
type HyperResponse = hyper::Response<Body>;

/// Simple error type for proxy operations
#[derive(Debug)]
pub struct ProxyError(String);

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for ProxyError {}

impl From<String> for ProxyError {
    fn from(s: String) -> Self {
        ProxyError(s)
    }
}

impl From<&str> for ProxyError {
    fn from(s: &str) -> Self {
        ProxyError(s.to_string())
    }
}

/// Encode url path. This can be used when building custom url path getters.
#[inline]
pub(crate) fn encode_url_path(path: &str) -> String {
    path.split('/')
        .map(|s| utf8_percent_encode(s, CONTROLS).to_string())
        .collect::<Vec<_>>()
        .join("/")
}

/// Client trait for implementing different HTTP clients for proxying.
///
/// Implement this trait to create custom proxy clients with different
/// backends or configurations.
pub trait Client: Send + Sync + 'static {
    /// Error type returned by the client.
    type Error: StdError + Send + Sync + 'static;

    /// Execute a request through the proxy client.
    fn execute(
        &self,
        req: HyperRequest,
        upgraded: Option<OnUpgrade>,
    ) -> impl Future<Output = Result<HyperResponse, Self::Error>> + Send;
}

/// Upstreams trait for selecting target servers.
///
/// Implement this trait to customize how target servers are selected
/// for proxying requests. This can be used to implement load balancing,
/// failover, or other server selection strategies.
pub trait Upstreams: Send + Sync + 'static {
    /// Error type returned when selecting a server fails.
    type Error: StdError + Send + Sync + 'static;

    /// Elect a server to handle the current request.
    fn elect(&self) -> impl Future<Output = Result<&str, Self::Error>> + Send;
}

impl Upstreams for &'static str {
    type Error = Infallible;

    async fn elect(&self) -> Result<&str, Self::Error> {
        Ok(*self)
    }
}

impl Upstreams for String {
    type Error = Infallible;
    async fn elect(&self) -> Result<&str, Self::Error> {
        Ok(self.as_str())
    }
}

impl<const N: usize> Upstreams for [&'static str; N] {
    type Error = ProxyError;
    async fn elect(&self) -> Result<&str, Self::Error> {
        if self.is_empty() {
            return Err(ProxyError::from("upstreams is empty"));
        }
        let index = fastrand::usize(..self.len());
        Ok(self[index])
    }
}

impl<T> Upstreams for Vec<T>
where
    T: AsRef<str> + Send + Sync + 'static,
{
    type Error = ProxyError;
    async fn elect(&self) -> Result<&str, Self::Error> {
        if self.is_empty() {
            return Err(ProxyError::from("upstreams is empty"));
        }
        let index = fastrand::usize(..self.len());
        Ok(self[index].as_ref())
    }
}

/// Url part getter function type for extracting path/query from requests.
pub type UrlPartGetter = Arc<dyn Fn(&HttpRequest<Body>) -> Option<String> + Send + Sync + 'static>;

/// Default url path getter.
///
/// This getter extracts the full path from the request URI.
pub fn default_url_path_getter(req: &HttpRequest<Body>) -> Option<String> {
    Some(encode_url_path(req.uri().path()))
}

/// Default url query getter. This getter returns the query string from the request URI.
pub fn default_url_query_getter(req: &HttpRequest<Body>) -> Option<String> {
    req.uri().query().map(Into::into)
}

/// Handler that can proxy requests to other servers.
#[non_exhaustive]
pub struct Proxy<U, C>
where
    U: Upstreams,
    C: Client,
{
    /// Upstreams list.
    pub upstreams: U,
    /// [`Client`] for proxy.
    pub client: C,
    /// Url path getter.
    pub url_path_getter: UrlPartGetter,
    /// Url query getter.
    pub url_query_getter: UrlPartGetter,
}

impl<U, C> Proxy<U, C>
where
    U: Upstreams,
    C: Client,
{
    /// Create new `Proxy` with upstreams list.
    pub fn new(upstreams: U, client: C) -> Self {
        Proxy {
            upstreams,
            client,
            url_path_getter: Arc::new(default_url_path_getter),
            url_query_getter: Arc::new(default_url_query_getter),
        }
    }

    /// Set url path getter.
    #[inline]
    pub fn url_path_getter<G>(mut self, url_path_getter: G) -> Self
    where
        G: Fn(&HttpRequest<Body>) -> Option<String> + Send + Sync + 'static,
    {
        self.url_path_getter = Arc::new(url_path_getter);
        self
    }

    /// Set url query getter.
    #[inline]
    pub fn url_query_getter<G>(mut self, url_query_getter: G) -> Self
    where
        G: Fn(&HttpRequest<Body>) -> Option<String> + Send + Sync + 'static,
    {
        self.url_query_getter = Arc::new(url_query_getter);
        self
    }

    /// Get upstreams list.
    #[inline]
    pub fn upstreams(&self) -> &U {
        &self.upstreams
    }

    /// Get upstreams mutable list.
    #[inline]
    pub fn upstreams_mut(&mut self) -> &mut U {
        &mut self.upstreams
    }

    /// Get client reference.
    #[inline]
    pub fn client(&self) -> &C {
        &self.client
    }

    /// Get client mutable reference.
    #[inline]
    pub fn client_mut(&mut self) -> &mut C {
        &mut self.client
    }

    /// Build the proxied request from the incoming request.
    pub async fn build_proxied_request(
        &self,
        req: &HttpRequest<Body>,
    ) -> Result<HyperRequest, anyhow::Error> {
        let upstream = self.upstreams.elect().await.map_err(|e| anyhow::anyhow!("Failed to elect upstream: {}", e))?;
        if upstream.is_empty() {
            tracing::error!("upstreams is empty");
            return Err(anyhow::anyhow!("upstreams is empty"));
        }

        let path = encode_url_path(&(self.url_path_getter)(req).unwrap_or_default());
        let query = (self.url_query_getter)(req);
        let rest = if let Some(query) = query {
            if query.starts_with('?') {
                format!("{}{}", path, query)
            } else {
                format!("{}?{}", path, query)
            }
        } else {
            path
        };

        let forward_url = if upstream.ends_with('/') && rest.starts_with('/') {
            format!("{}{}", upstream.trim_end_matches('/'), rest)
        } else if upstream.ends_with('/') || rest.starts_with('/') {
            format!("{}{}", upstream, rest)
        } else if rest.is_empty() {
            upstream.to_string()
        } else {
            format!("{}/{}", upstream, rest)
        };

        let forward_url: Uri = forward_url.parse().map_err(|e| anyhow::anyhow!("Invalid URI: {}", e))?;
        let mut build = hyper::Request::builder()
            .method(req.method())
            .uri(&forward_url);

        // Copy headers except HOST
        for (key, value) in req.headers() {
            if key != HOST {
                build = build.header(key, value);
            }
        }

        // Set proper HOST header for the upstream
        if let Some(host) = forward_url
            .host()
            .and_then(|host| HeaderValue::from_str(host).ok())
        {
            build = build.header(HeaderName::from_static("host"), host);
        }

        // Note: We're using an empty body here for now. In a real implementation,
        // you'd need to handle the request body properly.
        build.body(Body::empty()).map_err(|e| anyhow::anyhow!("Failed to build request: {}", e))
    }

    /// Execute the proxy request and return the response.
    pub async fn execute_proxy(
        &self,
        req: HttpRequest<Body>,
        upgraded: Option<OnUpgrade>,
    ) -> Result<HttpResponse<Body>, anyhow::Error> {
        let proxied_request = self.build_proxied_request(&req).await?;

        let response = self
            .client
            .execute(proxied_request, upgraded)
            .await
            .map_err(|e| anyhow::anyhow!("Proxy request failed: {}", e))?;

        Ok(response)
    }
}

#[inline]
#[allow(dead_code)]
pub(crate) fn get_upgrade_type(headers: &HeaderMap) -> Option<&str> {
    if headers
        .get(&CONNECTION)
        .map(|value| {
            value
                .to_str()
                .unwrap_or_default()
                .split(',')
                .any(|e| e.trim() == UPGRADE)
        })
        .unwrap_or(false)
    {
        if let Some(upgrade_value) = headers.get(&UPGRADE) {
            tracing::debug!(
                "Found upgrade header with value: {:?}",
                upgrade_value.to_str()
            );
            return upgrade_value.to_str().ok();
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_url_path() {
        let path = "/test/path";
        let encoded_path = encode_url_path(path);
        assert_eq!(encoded_path, "/test/path");
    }

    #[test]
    fn test_get_upgrade_type() {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("upgrade"));
        headers.insert(UPGRADE, HeaderValue::from_static("websocket"));
        let upgrade_type = get_upgrade_type(&headers);
        assert_eq!(upgrade_type, Some("websocket"));
    }

    #[tokio::test]
    async fn test_upstreams_string() {
        let upstream = "https://example.com".to_string();
        let result = upstream.elect().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://example.com");
    }

    #[tokio::test]
    async fn test_upstreams_vec() {
        let upstreams = vec!["https://example1.com", "https://example2.com"];
        let result = upstreams.elect().await;
        assert!(result.is_ok());
        assert!(upstreams.contains(&result.unwrap()));
    }
}
