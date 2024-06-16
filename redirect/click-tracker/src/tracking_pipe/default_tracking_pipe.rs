
use anyhow::Result;

use crate::{
    core::{
        location_detect::BaseLocationDetector, tracking_pipe::BaseTrackingPipe, user_agent_detect::BaseUserAgentDetector
    },
    model::Hit,
};

const MAIN_SWITCH: &'static str = "main";

#[derive(Clone)]
pub struct DefaultTrackingPipe {

    user_agent_detector: Box<dyn BaseUserAgentDetector + Send + Sync>,
    location_detector: Box<dyn BaseLocationDetector + Send + Sync>,
    //modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
}

impl DefaultTrackingPipe {
    pub fn new(

        user_agent_detector: Box<dyn BaseUserAgentDetector + Send + Sync>,
        location_detector: Box<dyn BaseLocationDetector + Send + Sync>,
        //modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
    ) -> Self {
        DefaultTrackingPipe {

            user_agent_detector,
            location_detector,
            //modules,
        }
    }
}

#[async_trait::async_trait()]
impl BaseTrackingPipe for DefaultTrackingPipe {
    async fn handle(&self, hit: Hit) -> Result<()> {
        Ok(())
    }
}
