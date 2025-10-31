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
        if let Some(user_agent_string) = &context.hit.user_agent {
            // Parse all components at once for better performance
            let client = self.user_agent_detector.parse_client(user_agent_string);

            // Check if device is a spider before moving values
            if let Some(brand) = &client.device.brand {
                if brand == SPIDER_DEVICE_BRAND {
                    context.spider = true;
                }
            }

            // Move parsed values into context
            context.client_ua = Some(client.user_agent);
            context.client_os = Some(client.os);
            context.client_device = Some(client.device);
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
