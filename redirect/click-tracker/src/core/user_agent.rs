use super::{Client, Device, OS, UserAgent};

pub trait UserAgentDetector {
    fn parse_device(&self, user_agent: &str) -> Device;
    fn parse_os(&self, user_agent: &str) -> OS;
    fn parse_user_agent(&self, user_agent: &str) -> UserAgent;

    /// Parse all components at once for better performance
    fn parse_client(&self, user_agent: &str) -> Client {
        Client {
            device: self.parse_device(user_agent),
            os: self.parse_os(user_agent),
            user_agent: self.parse_user_agent(user_agent),
        }
    }
}
