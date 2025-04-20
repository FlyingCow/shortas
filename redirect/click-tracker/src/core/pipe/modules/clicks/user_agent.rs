use anyhow::Result;

use crate::{
    adapters::UserAgentDetectorType,
    core::{TrackingPipeContext, UserAgentDetector, tracking_pipe::TrackingModule},
};
const SPIDER_DEVICE_BRAND: &'static str = "Spider";

#[derive(Clone)]
pub struct EnrichUserAgentModule {
    user_agent_detector: UserAgentDetectorType,
}

#[async_trait::async_trait()]
impl TrackingModule for EnrichUserAgentModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        if let Some(user_agent_string) = context.hit.user_agent.clone() {
            let user_agent = &self
                .user_agent_detector
                .parse_user_agent(&user_agent_string);
            context.client_ua = Some(user_agent.clone());

            let user_os = &self.user_agent_detector.parse_os(&user_agent_string);
            context.client_os = Some(user_os.clone());

            let user_device = &self.user_agent_detector.parse_device(&user_agent_string);
            context.client_device = Some(user_device.clone());

            if let Some(brand) = &user_device.brand {
                if brand == SPIDER_DEVICE_BRAND {
                    context.spider = true;
                }
            }
        }

        Ok(())
    }
}

impl EnrichUserAgentModule {
    pub fn new(user_agent_detector: UserAgentDetectorType) -> Self {
        Self {
            user_agent_detector,
        }
    }
}
