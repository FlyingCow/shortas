use std::net::{AddrParseError, SocketAddr};

use crate::{
    core::base_flow_router::FlowRouterContext,
    flow_router::base_ip_detector::{BaseIPDetector, IPInfo},
};

static DEFAULT_PORT: u16 = 80;
static X_FORWARDED_FOR_HEADER: &str = "X-Forwarded-For";

#[derive(Clone)]
pub struct DefaultIPDetector {}

impl DefaultIPDetector {
    pub fn new() -> Self {
        DefaultIPDetector {}
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

impl BaseIPDetector for DefaultIPDetector {
    fn detect(&self, context: &FlowRouterContext) -> Option<IPInfo> {
        let ip: SocketAddr = context.request.remote_addr;

        if let Some(ip_header) = detect_for_headers(&context.request.request) {
            let mut port: u16 = DEFAULT_PORT;

            if let Some(host) = &context.host {
                port = host.port;
            }

            let server: Result<SocketAddr, AddrParseError> =
                format!("{}:{}", ip_header, port).parse();

            if let Ok(server) = server {
                return Some(IPInfo { address: server });
            }
        }

        return Some(IPInfo { address: ip });
    }
}
