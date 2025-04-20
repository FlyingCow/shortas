use super::{Device, OS, UserAgent};

pub trait UserAgentDetector {
    fn parse_device(&self, user_agent: &str) -> Device;
    fn parse_os(&self, user_agent: &str) -> OS;
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent;
}
