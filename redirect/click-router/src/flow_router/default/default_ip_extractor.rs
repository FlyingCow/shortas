use std::{net::{IpAddr, SocketAddr}, str::FromStr};

use crate::{
    core::base_flow_router::FlowRouterContext,
    flow_router::base_ip_extractor::{BaseIPExtractor, IPInfo},
};

static X_FORWARDED_FOR_HEADER: &str = "X-Forwarded-For";

#[derive(Clone)]
pub struct DefaultIPExtractor {}

impl DefaultIPExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_for_headers(request: &http::Request<()>) -> Option<String> {
    if let Some(proto_header) = *&request.headers().get(X_FORWARDED_FOR_HEADER) {
        let client_details = proto_header
            .to_str()
            .unwrap_or_default()
            .split(",")
            .last()
            .unwrap();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_ascii_lowercase());
    }

    None
}

impl BaseIPExtractor for DefaultIPExtractor {
    fn detect(&self, context: &FlowRouterContext) -> Option<IPInfo> {
        let ip: SocketAddr = context.request.remote_addr;

        if let Some(ip_header) = detect_for_headers(&context.request.request) {

            let addr = IpAddr::from_str(ip_header.as_str());

            if let Ok(addr) = addr {
                return Some(IPInfo { address: addr });
            }
        }

        return Some(IPInfo { address: ip.ip() });
    }
}
