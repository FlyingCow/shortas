use anyhow::Result;

use crate::{
    core::{tracking_pipe::TrackingPipeContext, user_agent_detect::BaseUserAgentDetector}, tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct EnrichUserAgentModule{
    user_agent_detector: Box<dyn BaseUserAgentDetector + Sync +Send +'static>
}

#[async_trait::async_trait()]
impl BaseTrackingModule for EnrichUserAgentModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        println!("{}", "Executing EnrichUserAgentModule");
        let user_agent = &self.user_agent_detector.parse_user_agent(context.hit.user_agent.as_ref().unwrap());
        println!("{}", user_agent.family);
        Ok(())
    }
}

impl EnrichUserAgentModule {
    pub fn new(user_agent_detector: Box<dyn BaseUserAgentDetector + Sync +Send +'static>) -> Self{
        Self{
            user_agent_detector
        }
    } 
}
