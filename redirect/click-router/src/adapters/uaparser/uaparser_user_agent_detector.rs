use std::sync::Arc;

use uaparser::{Parser, UserAgentParser};

use crate::core::base_user_agent_detector::{BaseUserAgentDetector, Device, UserAgent, OS};

#[derive(Clone, Debug)]
pub struct UAParserUserAgentDetector {
    parser: Arc<UserAgentParser>,
}

impl UAParserUserAgentDetector {
    pub fn new(path: &str) -> Self {
        let parser = UserAgentParser::builder()
            .build_from_yaml(path)
            .expect("Parser creation failed");

        UAParserUserAgentDetector { parser: Arc::new(parser) }
    }
}

impl BaseUserAgentDetector for UAParserUserAgentDetector {
    fn parse_device<'a>(&self, user_agent: &'a str) -> Device<'a> {
        let uaparser_device = self.parser.parse_device(user_agent);

        Device {
            brand: uaparser_device.brand,
            family: uaparser_device.family,
            model: uaparser_device.model,
        }
    }

    fn parse_os<'a>(&self, user_agent: &'a str) -> OS<'a> {
        let uaparser_os = self.parser.parse_os(user_agent);

        OS {
            family: uaparser_os.family,
            major: uaparser_os.major,
            minor: uaparser_os.minor,
            patch: uaparser_os.patch,
            patch_minor: uaparser_os.patch_minor,
        }
    }

    fn parse_user_agent<'a>(&self, user_agent: &'a str) -> UserAgent<'a> {
        let uaparser_user_agent = self.parser.parse_user_agent(user_agent);

        UserAgent {
            family: uaparser_user_agent.family,
            major: uaparser_user_agent.major,
            minor: uaparser_user_agent.minor,
            patch: uaparser_user_agent.patch,
        }
    }
}
