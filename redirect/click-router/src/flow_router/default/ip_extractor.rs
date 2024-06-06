use std::{net::IpAddr, str::FromStr};

use crate::{
    core::flow_router::RequestData,
    flow_router::ip_extract::{BaseIPExtractor, IPInfo},
};

static X_FORWARDED_FOR_HEADER: &str = "X-Forwarded-For";

#[derive(Clone)]
pub struct DefaultIPExtractor {}

impl DefaultIPExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_for_headers(request: &RequestData) -> Option<String> {
    if let Some(proto_header) = *&request.headers.get(X_FORWARDED_FOR_HEADER) {
        let header = proto_header.to_str().unwrap_or_default().replace(" ", "");

        let client_details = header.split(",").last().unwrap();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_ascii_lowercase());
    }

    None
}

impl BaseIPExtractor for DefaultIPExtractor {
    fn detect(&self, request: &RequestData) -> Option<IPInfo> {
        if let Some(ip_header) = detect_for_headers(&request) {
            let addr = IpAddr::from_str(ip_header.as_str());

            if let Ok(addr) = addr {
                return Some(IPInfo { address: addr });
            }
        }

        if let Some(ip) = request.remote_addr {
            return Some(IPInfo { address: ip.ip() });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::core::flow_router::{FlowInRoute, FlowRouterContext, RequestData, ResponseData};

    use super::*;

    fn get_default_context(header_value: &str, remote_addr: &str) -> FlowRouterContext {
        let mut request = RequestData {
            local_addr: Some(SocketAddr::new("192.168.0.1".parse().unwrap(), 80)),
            remote_addr: Some(SocketAddr::new(remote_addr.parse().unwrap(), 80)),
            tls_info: None,
            ..Default::default()
        };

        request
            .headers
            .insert(X_FORWARDED_FOR_HEADER, header_value.parse().unwrap());

        FlowRouterContext::new(
            FlowInRoute::new(
                String::from("http"),
                String::from("test.com"),
                80,
                String::from("/"),
                String::from(""),
            ),
            request,
            ResponseData::default(),
        )
    }

    #[test]
    fn should_extract_from_header_when_present() {
        let context = get_default_context("183.143.0.2", "183.143.0.1");

        let result = DefaultIPExtractor::new().detect(&context.request);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.2".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn should_use_last_value_from_header() {
        let context = get_default_context("183.143.0.2, 183.143.0.3, 183.143.0.4", "183.143.0.1");

        let result = DefaultIPExtractor::new().detect(&context.request);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.4".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn should_use_remote_addr_when_header_not_present() {
        let context = get_default_context("", "183.143.0.1");

        let result = DefaultIPExtractor::new().detect(&context.request);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.1".parse::<IpAddr>().unwrap());
    }
}
