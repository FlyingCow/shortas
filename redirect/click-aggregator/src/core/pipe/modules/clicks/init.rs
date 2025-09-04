use anyhow::Result;
use tracing::info;

use crate::core::{aggs_pipe::AggsModule, AggsPipeContext};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl AggsModule for InitModule {
    async fn execute(&mut self, _context: &mut AggsPipeContext) -> Result<()> {
        info!("Initializing click info.");

        Ok(())
    }
}
