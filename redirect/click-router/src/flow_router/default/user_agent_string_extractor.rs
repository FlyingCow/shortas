use crate::{
    core::flow_router::RequestData,
    flow_router::user_agent_string_extract::BaseUserAgentStringExtractor,
};

static USER_AGENT_HEADER: &str = "User-Agent";

#[derive(Clone)]
pub struct DefaultUserAgentStringExtractor {}

impl DefaultUserAgentStringExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn detect_from_headers(request: &RequestData) -> Option<String> {
    if let Some(user_agent_header) = *&request.headers.get(USER_AGENT_HEADER) {
        let client_details = user_agent_header.to_str().unwrap_or_default();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_string());
    }

    None
}

impl BaseUserAgentStringExtractor for DefaultUserAgentStringExtractor {
    fn detect(&self, request: &RequestData) -> Option<String> {
        detect_from_headers(&request)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_from_user_agent_header_when_present() {
        let mut request: RequestData = RequestData {
            ..Default::default()
        };

        request
            .headers
            .insert(USER_AGENT_HEADER, "test user agent 0.1".parse().unwrap());

        let result = DefaultUserAgentStringExtractor::new().detect(&request);

        assert!(result.is_some());
        let user_agent = result.unwrap();
        assert_eq!(user_agent, "test user agent 0.1");
    }
}
