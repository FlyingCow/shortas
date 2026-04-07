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
        let route_id = route.id.clone().unwrap_or_default();

        let session = self
            .session_detector
            .detect(route_id.as_str(), &ip, &context.hit.utc)
            .await?;

        // Log module execution for debug routes
        if let Some(ref trace) = context.hit.trace {
            tracing::warn!(
                trace_id = %trace.trace_id,
                route_id = %route_id,
                service = "click-tracker",
                step = "SessionModule",
                session_clicks = %session.count,
                is_unique = %(session.count == 1),
                "Debug trace: session module executed"
            );
        }

        context.session = Some(session);

        Ok(())
    }
}
impl EnrichSessionModule {
    pub fn new(session_detector: SessionDetectorType) -> Self {
        Self { session_detector }
    }
}
