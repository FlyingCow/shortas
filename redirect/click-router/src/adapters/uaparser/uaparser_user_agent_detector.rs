use std::sync::Arc;

use uaparser::{Parser, UserAgentParser};

use crate::core::base_user_agent_detector::{BaseUserAgentDetector, Device, UserAgent, OS};

#[derive(Clone, Debug)]
pub struct UAParserUserAgentDetector {
    parser: Arc<UserAgentParser>,
}

impl UAParserUserAgentDetector {
    pub fn new(path: &str) -> Self {
        println!("  yaml -> {}", path);
        let parser = UserAgentParser::builder()
            .build_from_yaml(path)
            .expect("Parser creation failed");

        UAParserUserAgentDetector {
            parser: Arc::new(parser),
        }
    }
}

impl BaseUserAgentDetector for UAParserUserAgentDetector {
    fn parse_device(&self, user_agent: &str) -> Device {
        let uaparser_device = self.parser.parse_device(user_agent);

        Device {
            brand: uaparser_device.brand.map_or(None, |u| Some(u.to_string())),
            family: uaparser_device.family.to_string(),
            model: uaparser_device.model.map_or(None, |u| Some(u.to_string())),
        }
    }

    fn parse_os(&self, user_agent: &str) -> OS {
        let uaparser_os = self.parser.parse_os(user_agent);

        OS {
            family: uaparser_os.family.to_string(),
            major: uaparser_os.major.map_or(None, |u| Some(u.to_string())),
            minor: uaparser_os.minor.map_or(None, |u| Some(u.to_string())),
            patch: uaparser_os.patch.map_or(None, |u| Some(u.to_string())),
            patch_minor: uaparser_os
                .patch_minor
                .map_or(None, |u| Some(u.to_string())),
        }
    }

    fn parse_user_agent(&self, user_agent: &str) -> UserAgent {
        let uaparser_user_agent = self.parser.parse_user_agent(user_agent);

        UserAgent {
            family: uaparser_user_agent.family.to_string(),
            major: uaparser_user_agent
                .major
                .map_or(None, |u| Some(u.to_string())),
            minor: uaparser_user_agent
                .minor
                .map_or(None, |u| Some(u.to_string())),
            patch: uaparser_user_agent
                .patch
                .map_or(None, |u| Some(u.to_string())),
        }
    }
}
