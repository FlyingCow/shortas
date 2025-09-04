use anyhow::Result;

use crate::core::{aggs_pipe::AggsModule, AggsPipeContext};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl AggsModule for InitModule {
    async fn execute(&mut self, _context: &mut AggsPipeContext) -> Result<()> {
        Ok(())
    }
}
