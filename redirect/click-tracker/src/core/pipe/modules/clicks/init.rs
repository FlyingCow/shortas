use anyhow::Result;

use crate::core::{TrackingPipeContext, tracking_pipe::TrackingModule};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl TrackingModule for InitModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        // Log module execution for debug routes
        if let Some(ref trace) = context.hit.trace {
            tracing::warn!(
                trace_id = %trace.trace_id,
                route_id = %context.hit.route.as_ref().and_then(|r| r.id.as_ref()).map(|s| s.as_str()).unwrap_or(""),
                service = "click-tracker",
                step = "InitModule",
                "Debug trace: init module executed"
            );
        }
        Ok(())
    }
}
