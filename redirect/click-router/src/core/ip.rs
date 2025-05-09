use std::{net::IpAddr, str::FromStr};

use crate::adapters::RequestType;

use super::flow_router::Request;

#[derive(Clone, Debug)]
pub struct IPInfo {
    pub address: IpAddr,
}

const DEBUG_IP_PARAM: &'static str = "x_debug_ip";
const X_FORWARDED_FOR_HEADER: &'static str = "X-Forwarded-For";

#[derive(Clone)]
pub struct IPExtractor {}

impl IPExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn get_debug(request: &RequestType) -> Option<String> {
    let queries = request.queries();

    let param_value = queries.get(DEBUG_IP_PARAM);

    if param_value.is_some() {
        return param_value.cloned();
    }

    let header_value = request.headers().get(DEBUG_IP_PARAM).cloned();

    if let Some(header) = header_value {
        return Some(header.to_str().unwrap_or_default().to_string());
    }

    None
}

fn detect_for_headers(request: &RequestType) -> Option<String> {
    if let Some(proto_header) = request.headers().get(X_FORWARDED_FOR_HEADER) {
        let header = proto_header.to_str().unwrap_or_default().replace(" ", "");

        let client_details = header.split(",").last().unwrap();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_ascii_lowercase());
    }

    None
}

impl IPExtractor {
    pub fn detect(&self, request: &RequestType, debug: bool) -> Option<IPInfo> {
        if debug {
            if let Some(debug_ip) = get_debug(&request) {
                let addr = IpAddr::from_str(debug_ip.as_str());

                if let Ok(addr) = addr {
                    return Some(IPInfo { address: addr });
                }
            }
        }

        if let Some(ip_header) = detect_for_headers(&request) {
            let addr = IpAddr::from_str(ip_header.as_str());

            if let Ok(addr) = addr {
                return Some(IPInfo { address: addr });
            }
        }

        if let Some(ip) = request.remote_addr() {
            return Some(IPInfo { address: ip.ip() });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use crate::{
        adapters::ResponseType,
        core::flow_router::{FlowInRoute, FlowRouterContext, RequestData, ResponseData},
    };

    use super::*;

    fn get_default_request<'a>(
        header_value: &'a str,
        debug_header_value: &'a str,
        remote_addr: &'a str,
    ) -> RequestType<'a> {
        let mut request_data = RequestData {
            local_addr: Some(SocketAddr::new("192.168.0.1".parse().unwrap(), 80)),
            remote_addr: Some(SocketAddr::new(remote_addr.parse().unwrap(), 80)),
            tls_info: None,
            ..Default::default()
        };

        request_data
            .headers
            .insert(X_FORWARDED_FOR_HEADER, header_value.parse().unwrap());

        request_data
            .headers
            .insert(DEBUG_IP_PARAM, debug_header_value.parse().unwrap());

        let request = RequestType::Test(request_data);
        request
    }

    fn get_default_context<'a>(
        request: &'a RequestType<'a>,
        response: &'a mut ResponseType<'a>,
    ) -> FlowRouterContext<'a> {
        FlowRouterContext::new(
            FlowInRoute::new(
                String::from("http"),
                String::from("test.com"),
                80,
                String::from("/"),
                String::from(""),
            ),
            request,
            response,
        )
    }

    #[test]
    fn should_extract_debug_from_header_when_present() {
        let req = get_default_request("183.143.0.2", "183.143.0.3", "183.143.0.1");
        let mut res = ResponseType::Test(ResponseData::default());

        let context = get_default_context(&req, &mut res);

        let result = IPExtractor::new().detect(&context.request, true);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.3".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn should_extract_from_header_when_present() {
        let req = get_default_request("183.143.0.2", "183.143.0.3", "183.143.0.1");
        let mut res = ResponseType::Test(ResponseData::default());

        let context = get_default_context(&req, &mut res);

        let result = IPExtractor::new().detect(&context.request, false);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.2".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn should_use_last_value_from_header() {
        let req = get_default_request(
            "183.143.0.2, 183.143.0.3, 183.143.0.4",
            "183.143.0.3",
            "183.143.0.1",
        );
        let mut res = ResponseType::Test(ResponseData::default());

        let context = get_default_context(&req, &mut res);

        let result = IPExtractor::new().detect(&context.request, false);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.4".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn should_use_remote_addr_when_header_not_present() {
        let req = get_default_request("", "183.143.0.3", "183.143.0.1");
        let mut res = ResponseType::Test(ResponseData::default());

        let context = get_default_context(&req, &mut res);

        let result = IPExtractor::new().detect(&context.request, false);

        assert!(result.is_some());
        let ip_info = result.unwrap();
        assert_eq!(ip_info.address, "183.143.0.1".parse::<IpAddr>().unwrap());
    }
}
