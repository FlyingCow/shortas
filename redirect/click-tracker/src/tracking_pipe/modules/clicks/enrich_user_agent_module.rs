use anyhow::Result;

use crate::{
    core::{tracking_pipe::TrackingPipeContext, user_agent_detect::BaseUserAgentDetector},
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct EnrichUserAgentModule {
    user_agent_detector: Box<dyn BaseUserAgentDetector + Sync + Send + 'static>,
}

#[async_trait::async_trait()]
impl BaseTrackingModule for EnrichUserAgentModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        println!("{}", "Executing EnrichUserAgentModule");

        if let Some(user_agent_string) = context.hit.user_agent.clone() {
            let user_agent = &self
                .user_agent_detector
                .parse_user_agent(&user_agent_string);
            context.client_ua = Some(user_agent.clone());

            let user_os = &self.user_agent_detector.parse_os(&user_agent_string);
            context.client_os = Some(user_os.clone());

            let user_device = &self.user_agent_detector.parse_device(&user_agent_string);
            context.client_device = Some(user_device.clone());
        }

        Ok(())
    }
}

impl EnrichUserAgentModule {
    pub fn new(
        user_agent_detector: Box<dyn BaseUserAgentDetector + Sync + Send + 'static>,
    ) -> Self {
        Self {
            user_agent_detector,
        }
    }
}
