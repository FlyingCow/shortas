use crate::adapters::RequestType;

use super::flow_router::Request;

const DEBUG_UA_PARAM: &'static str = "x_debug_ua";
const USER_AGENT_HEADER: &str = "User-Agent";

#[derive(Clone)]
pub struct UserAgentStringExtractor {}

impl UserAgentStringExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

fn get_debug(request: &RequestType) -> Option<String> {
    let queries = request.queries();

    let param_value = queries.get(DEBUG_UA_PARAM);

    if param_value.is_some() {
        return param_value.cloned();
    }

    let header_value = request.headers().get(DEBUG_UA_PARAM).cloned();

    if let Some(header) = header_value {
        return Some(header.to_str().unwrap_or_default().to_string());
    }

    None
}

fn detect_from_headers(request: &RequestType) -> Option<String> {
    if let Some(user_agent_header) = *&request.headers().get(USER_AGENT_HEADER) {
        let client_details = user_agent_header.to_str().unwrap_or_default();

        if client_details.is_empty() {
            return None;
        }

        return Some(client_details.to_string());
    }

    None
}

impl UserAgentStringExtractor {
    pub fn detect(&self, request: &RequestType, debug: bool) -> Option<String> {
        if debug {
            if let Some(debug_ua) = get_debug(&request) {
                return Some(debug_ua);
            }
        }

        detect_from_headers(&request)
    }
}

#[cfg(test)]
mod tests {

    use crate::core::flow_router::RequestData;

    use super::*;

    #[test]
    fn should_extract_from_user_agent_header_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert(USER_AGENT_HEADER, "test user agent 0.1".parse().unwrap());

        let request: RequestType = RequestType::Test(request_data);
        let result = UserAgentStringExtractor::new().detect(&request, false);

        assert!(result.is_some());
        let user_agent = result.unwrap();
        assert_eq!(user_agent, "test user agent 0.1");
    }

    #[test]
    fn should_extract_from_debug_when_present() {
        let mut request_data = RequestData {
            ..Default::default()
        };

        request_data
            .headers
            .insert(USER_AGENT_HEADER, "test user agent 0.1".parse().unwrap());

        request_data
            .headers
            .insert(DEBUG_UA_PARAM, "test user agent 0.2".parse().unwrap());

        let request = RequestType::Test(request_data);
        let result = UserAgentStringExtractor::new().detect(&request, true);

        assert!(result.is_some());
        let user_agent = result.unwrap();
        assert_eq!(user_agent, "test user agent 0.2");
    }
}
