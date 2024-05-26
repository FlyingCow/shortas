
use http::Request;

use crate::flow_router::base_user_agent_extractor::BaseUserAgentExtractor;

static USER_AGENT_HEADER: &str = "User-Agent";

#[derive(Clone)]
pub struct DefaultUserAgentExtractor {}

impl DefaultUserAgentExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_headers(request: &http::Request<()>) -> Option<String> {
    if let Some(user_agent_header) = *&request.headers().get(USER_AGENT_HEADER) {
        let client_details = user_agent_header
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

impl BaseUserAgentExtractor for DefaultUserAgentExtractor {
    fn detect(&self, request: &Request<()>) -> Option<String> {
        detect_from_headers(&request)
    }
}
