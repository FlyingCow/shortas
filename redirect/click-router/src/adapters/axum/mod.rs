use cookie::Cookie;
use cookie::CookieJar;
use http::header::IntoHeaderName;
use http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use indexmap::IndexMap;
use multimap::MultiMap;
use std::net::SocketAddr;

use crate::core::flow_router::Request;
use crate::core::flow_router::Response;

pub mod axum_proxy;

/// Wrapper around Axum request that implements the flow_router Request trait.
/// Since Axum uses extractors, we need to pre-extract and store all the data needed.
pub struct AxumRequest {
    uri: Uri,
    headers: HeaderMap,
    method: Method,
    scheme: http::uri::Scheme,
    params: IndexMap<String, String>,
    queries: MultiMap<String, String>,
    remote_addr: Option<SocketAddr>,
    cookies: CookieJar,
}

impl AxumRequest {
    /// Create a new AxumRequest by extracting data from an axum request
    pub fn new(
        req: &axum::extract::Request,
        params: IndexMap<String, String>,
        remote_addr: Option<SocketAddr>,
        cookies: CookieJar,
    ) -> Self {
        let uri = req.uri().clone();
        let headers = req.headers().clone();
        let method = req.method().clone();
        let scheme = uri.scheme().cloned().unwrap_or(http::uri::Scheme::HTTP);

        // Parse query string
        let queries = parse_queries(uri.query());

        Self {
            uri,
            headers,
            method,
            scheme,
            params,
            queries,
            remote_addr,
            cookies,
        }
    }

    /// Create from individual components (useful for testing or when you already have parsed data)
    pub fn from_parts(
        uri: Uri,
        headers: HeaderMap,
        method: Method,
        scheme: http::uri::Scheme,
        params: IndexMap<String, String>,
        queries: MultiMap<String, String>,
        remote_addr: Option<SocketAddr>,
        cookies: CookieJar,
    ) -> Self {
        Self {
            uri,
            headers,
            method,
            scheme,
            params,
            queries,
            remote_addr,
            cookies,
        }
    }
}

impl Request for AxumRequest {
    fn uri(&self) -> &Uri {
        &self.uri
    }

    fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    fn method(&self) -> &Method {
        &self.method
    }

    fn scheme(&self) -> &http::uri::Scheme {
        &self.scheme
    }

    fn remote_addr(&self) -> Option<SocketAddr> {
        self.remote_addr
    }

    fn params(&self) -> &IndexMap<String, String> {
        &self.params
    }

    fn queries(&self) -> &MultiMap<String, String> {
        &self.queries
    }

    fn cookies(&self) -> &CookieJar {
        &self.cookies
    }

    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>,
    {
        self.cookies.get(name.as_ref())
    }
}

/// Response buffer for Axum that implements the flow_router Response trait.
/// Since Axum uses functional return values instead of mutation, we buffer
/// the response data and convert it to an axum Response at the end.
pub struct AxumResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
    cookies: CookieJar,
}

impl AxumResponse {
    pub fn new(cookies: CookieJar) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Vec::new(),
            cookies,
        }
    }

    /// Set the response status code
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Set the response body
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// Get the response body
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Get the response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get the response status
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Convert the buffered response into an axum Response
    pub fn into_axum_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::builder()
            .status(self.status);

        // Add all headers
        for (name, value) in self.headers.iter() {
            response = response.header(name, value);
        }

        // Add cookies as Set-Cookie headers
        for cookie in self.cookies.iter() {
            if let Ok(header_value) = HeaderValue::from_str(&cookie.to_string()) {
                response = response.header(http::header::SET_COOKIE, header_value);
            }
        }

        // Build the response with the body
        response
            .body(axum::body::Body::from(self.body))
            .unwrap_or_else(|_| {
                axum::response::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(axum::body::Body::from("Failed to build response"))
                    .unwrap()
            })
    }
}

impl Response for AxumResponse {
    fn add_header<N, V>(&mut self, name: N, value: V, overwrite: bool) -> anyhow::Result<()>
    where
        N: IntoHeaderName,
        V: TryInto<HeaderValue>,
    {
        let header_value = value
            .try_into()
            .map_err(|_| anyhow::anyhow!("Invalid header value"))?;

        if overwrite {
            self.headers.insert(name, header_value);
        } else {
            self.headers.append(name, header_value);
        }
        Ok(())
    }

    fn cookies(&self) -> &CookieJar {
        &self.cookies
    }

    fn cookies_mut(&mut self) -> &mut CookieJar {
        &mut self.cookies
    }

    fn cookie<T>(&self, name: T) -> Option<&Cookie<'static>>
    where
        T: AsRef<str>,
    {
        self.cookies.get(name.as_ref())
    }

    fn add_cookie(&mut self, cookie: Cookie<'static>) {
        self.cookies.add(cookie);
    }
}

/// Parse query string into a MultiMap
pub fn parse_queries(query: Option<&str>) -> MultiMap<String, String> {
    let mut queries = MultiMap::new();

    if let Some(query_str) = query {
        for pair in query_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = urlencoding::decode(key).unwrap_or_else(|_| key.into()).into_owned();
                let value = urlencoding::decode(value).unwrap_or_else(|_| value.into()).into_owned();
                queries.insert(key, value);
            } else {
                let key = urlencoding::decode(pair).unwrap_or_else(|_| pair.into()).into_owned();
                queries.insert(key, String::new());
            }
        }
    }

    queries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_queries() {
        let queries = parse_queries(Some("key1=value1&key2=value2&key3"));

        assert_eq!(queries.get("key1"), Some(&"value1".to_string()));
        assert_eq!(queries.get("key2"), Some(&"value2".to_string()));
        assert_eq!(queries.get("key3"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_queries_with_encoding() {
        let queries = parse_queries(Some("key=hello%20world&special=%26%3D"));

        assert_eq!(queries.get("key"), Some(&"hello world".to_string()));
        assert_eq!(queries.get("special"), Some(&"&=".to_string()));
    }

    #[test]
    fn test_parse_queries_empty() {
        let queries = parse_queries(None);
        assert!(queries.is_empty());
    }
}
