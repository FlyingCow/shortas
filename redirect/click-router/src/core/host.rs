use crate::adapters::RequestType;

use super::flow_router::Request;

static HOST_HEADER: &str = "Host";
const DEFAULT_PORT: u16 = 80;

#[derive(Clone, Debug)]
pub struct HostInfo {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct HostExtractor {}

impl HostExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_uri(request: &RequestType) -> Option<HostInfo> {
    if let Some(authority) = request.uri().authority() {
        return Some(HostInfo {
            host: authority.host().to_ascii_lowercase(),
            port: authority.port_u16().unwrap_or(DEFAULT_PORT),
        });
    }

    if let Some(host) = request.uri().host() {
        return Some(HostInfo {
            host: host.to_ascii_lowercase(),
            port: DEFAULT_PORT,
        });
    }

    None
}

fn detect_from_headers(request: &RequestType) -> Option<HostInfo> {
    if let Some(host_header) = request.headers().get(HOST_HEADER) {
        let mut host_and_port = host_header.to_str().unwrap_or_default().split(":");

        let host = host_and_port.next().unwrap_or_default();

        let port = host_and_port
            .next()
            .unwrap_or_default()
            .parse::<u16>()
            .unwrap_or(DEFAULT_PORT);

        return Some(HostInfo {
            host: host.to_ascii_lowercase(),
            port: port,
        });
    }

    None
}

impl HostExtractor {
    pub fn detect(&self, request: &RequestType, _debug: bool) -> Option<HostInfo> {
        if let Some(host_info) = detect_from_headers(request) {
            return Some(host_info);
        }

        if let Some(host_info) = detect_from_uri(request) {
            return Some(host_info);
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use crate::core::flow_router::RequestData;

    use super::*;

    #[test]
    fn should_extract_host_header_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert("Host", "test.com:80".parse().unwrap());

        let request = RequestType::Test(request_data);

        let result = HostExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let host_info = result.unwrap();
        assert_eq!(host_info.host, "test.com");
        assert_eq!(host_info.port, 80);
    }

    #[test]
    fn should_extract_host_header_port_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert("Host", "test.com:443".parse().unwrap());

        let request = RequestType::Test(request_data);

        let result = HostExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let host_info = result.unwrap();
        assert_eq!(host_info.host, "test.com");
        assert_eq!(host_info.port, 443);
    }

    #[test]
    fn should_extract_uri_host_when_host_header_not_present() {
        let request = RequestType::Test(RequestData {
            uri: "http://www.rust-lang.org:80/".parse().unwrap(),
            ..Default::default()
        });

        let result = HostExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let host_info = result.unwrap();
        assert_eq!(host_info.host, "www.rust-lang.org");
        assert_eq!(host_info.port, 80);
    }

    #[test]
    fn should_extract_uri_port_when_present() {
        let request = RequestType::Test(RequestData {
            uri: "https://www.rust-lang.org:443/".parse().unwrap(),
            ..Default::default()
        });

        let result = HostExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let host_info = result.unwrap();
        assert_eq!(host_info.host, "www.rust-lang.org");
        assert_eq!(host_info.port, 443);
    }
}
