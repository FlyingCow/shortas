use anyhow::Result;

use crate::core::{aggs_pipe::AggsModule, AggsPipeContext};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl AggsModule for InitModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        // Log module execution for debug routes
        if let Some(ref trace) = context.click.trace {
            tracing::warn!(
                trace_id = %trace.trace_id,
                route_id = %context.click.route_id,
                service = "click-aggregator",
                step = "InitModule",
                "Debug trace: init module executed"
            );
        }
        Ok(())
    }
}
