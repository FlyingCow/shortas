
use anyhow::Result;

use crate::core::{hit_stream::BaseHitStream, tracking_pipe::BaseTrackingPipe};

pub struct DefaultTrackingPipe {
    hit_stream: Box<dyn BaseHitStream + Send + Sync + 'static>, // user_agent_detector: Box<dyn BaseUserAgentDetector + Send + Sync>,
                                                      // location_detector: Box<dyn BaseLocationDetector + Send + Sync>,
                                                      // modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
}

impl DefaultTrackingPipe {
    pub fn new(
        hit_stream: Box<dyn BaseHitStream + Send + Sync + 'static>,
        //modules: Vec<Box<dyn BaseFlowModule + Send + Sync>>,
    ) -> Self {
        DefaultTrackingPipe {
            hit_stream: hit_stream,
            //modules,
        }
    }
}

#[async_trait::async_trait()]
impl BaseTrackingPipe for DefaultTrackingPipe {
    async fn start(&mut self) -> Result<()> {
        let a = self.hit_stream.as_mut().pull().await?;
        Ok(())
    }
}
