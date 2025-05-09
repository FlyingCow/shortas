use crate::{adapters::RequestType, core::flow_router::RequestData};

use super::flow_router::Request;

#[derive(Clone, Debug)]
pub struct ProtoInfo {
    pub proto: String,
    pub ssl_on: bool,
}

static HTTP: &'static str = "http";
static HTTPS: &'static str = "https";
static SSL_ON: &'static str = "on";
static X_FORWARDED_PROTO_HEADER: &str = "X-Forwarded-Proto";
static X_FORWARDED_SSL_HEADER: &str = "X-Forwarded-Ssl";

#[derive(Clone)]
pub struct ProtocolExtractor {}

impl ProtocolExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_proto_uri(request: &RequestType) -> Option<String> {
    if let Some(scheme) = *&request.uri().scheme_str() {
        return Some(scheme.to_ascii_lowercase());
    }

    None
}

fn detect_ssl_on_uri(request: &RequestType) -> Option<bool> {
    if let Some(scheme) = request.uri().scheme_str() {
        return Some(scheme.to_ascii_lowercase() == HTTPS);
    }

    None
}

fn detect_proto_headers(request: &RequestType) -> Option<String> {
    if let Some(proto_header) = request.headers().get(X_FORWARDED_PROTO_HEADER) {
        return Some(
            proto_header
                .to_str()
                .unwrap_or_default()
                .to_ascii_lowercase(),
        );
    }

    None
}

fn detect_ssl_on_headers(request: &RequestType) -> Option<bool> {
    if let Some(proto_header) = *&request.headers().get(X_FORWARDED_SSL_HEADER) {
        return Some(
            proto_header
                .to_str()
                .unwrap_or_default()
                .to_ascii_lowercase()
                == SSL_ON,
        );
    }

    None
}

impl ProtocolExtractor {
    pub fn detect(&self, request: &RequestType, _debug: bool) -> Option<ProtoInfo> {
        let mut proto: String = HTTP.to_string();
        let mut ssl_on: bool = false;

        if let Some(proto_header) = detect_proto_headers(request) {
            proto = proto_header;
        } else if let Some(proto_uri) = detect_proto_uri(request) {
            proto = proto_uri;
        }

        if let Some(ssl_on_header) = detect_ssl_on_headers(request) {
            ssl_on = ssl_on_header;
        } else if let Some(ssl_on_uri) = detect_ssl_on_uri(request) {
            ssl_on = ssl_on_uri;
        }

        Some(ProtoInfo {
            proto: proto,
            ssl_on: ssl_on,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_proto_header_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert("X-Forwarded-Proto", "https".parse().unwrap());

        request_data
            .headers
            .insert("X-Forwarded-Ssl", "on".parse().unwrap());

        let request = RequestType::Test(request_data);

        let result = ProtocolExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let proto_info = result.unwrap();
        assert_eq!(proto_info.proto, "https");
        assert!(proto_info.ssl_on);
    }

    #[test]
    fn should_extract_https_uri_proto_when_host_header_not_present() {
        let request = RequestType::Test(RequestData {
            uri: "https://www.rust-lang.org:443/".parse().unwrap(),
            ..Default::default()
        });

        let result = ProtocolExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let proto_info = result.unwrap();
        assert_eq!(proto_info.proto, "https");
        assert_eq!(proto_info.ssl_on, true);
    }

    #[test]
    fn should_extract_http_uri_proto_when_host_header_not_present() {
        let request = RequestType::Test(RequestData {
            uri: "http://www.rust-lang.org/".parse().unwrap(),
            ..Default::default()
        });

        let result = ProtocolExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let proto_info = result.unwrap();
        assert_eq!(proto_info.proto, "http");
        assert_eq!(proto_info.ssl_on, false);
    }
}
