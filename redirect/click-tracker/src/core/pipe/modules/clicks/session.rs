use anyhow::Result;

use crate::{
    adapters::SessionDetectorType,
    core::{TrackingPipeContext, session::SessionDetector, tracking_pipe::TrackingModule},
};

#[derive(Clone)]
pub struct EnrichSessionModule {
    session_detector: SessionDetectorType,
}

#[async_trait::async_trait()]
impl TrackingModule for EnrichSessionModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        if context.hit.ip.is_none() || context.hit.route.is_none() {
            return Ok(());
        }

        let ip = context.hit.ip.unwrap();
        let route = context.hit.route.clone().unwrap();

        let session = self
            .session_detector
            .detect(route.id.unwrap().as_str(), &ip, &context.hit.utc)
            .await?;

        context.session = Some(session);

        Ok(())
    }
}
impl EnrichSessionModule {
    pub fn new(session_detector: SessionDetectorType) -> Self {
        Self { session_detector }
    }
}
