use http::Request;

use crate::flow_router::base_user_agent_string_extractor::BaseUserAgentStringExtractor;

static USER_AGENT_HEADER: &str = "User-Agent";

#[derive(Clone)]
pub struct DefaultUserAgentStringExtractor {}

impl DefaultUserAgentStringExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_headers(request: &http::Request<()>) -> Option<String> {
    if let Some(user_agent_header) = *&request.headers().get(USER_AGENT_HEADER) {
        let client_details = user_agent_header.to_str().unwrap_or_default();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_ascii_lowercase());
    }

    None
}

impl BaseUserAgentStringExtractor for DefaultUserAgentStringExtractor {
    fn detect(&self, request: &Request<()>) -> Option<String> {
        detect_from_headers(&request)
    }
}
