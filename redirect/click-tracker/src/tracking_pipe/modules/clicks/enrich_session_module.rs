use anyhow::Result;

use crate::{
    core::{session_detect::BaseSessionDetector, tracking_pipe::TrackingPipeContext},
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct EnrichSessionModule {
    session_detector: Box<dyn BaseSessionDetector + Sync + Send + 'static>,
}

#[async_trait::async_trait()]
impl BaseTrackingModule for EnrichSessionModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        if context.hit.ip.is_none() || context.hit.route.is_none() {
            return Ok(());
        }

        let ip = context.hit.ip.unwrap();
        let route = context.hit.route.clone().unwrap();

        let session = self.session_detector.detect_session(
            route.id.unwrap().as_str(),
            &ip,
            &context.hit.utc,
        )?;

        context.session = Some(session);

        Ok(())
    }
}
impl EnrichSessionModule {
    pub fn new(session_detector: Box<dyn BaseSessionDetector + Sync + Send + 'static>) -> Self {
        Self { session_detector }
    }
}
