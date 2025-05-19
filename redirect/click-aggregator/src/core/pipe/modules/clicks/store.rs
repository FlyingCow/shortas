use anyhow::Result;
use tracing::info;

use crate::core::{aggs_pipe::AggsModule, AggsPipeContext};

#[derive(Clone)]
pub struct StoreModule;

#[async_trait::async_trait()]
impl AggsModule for StoreModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        info!("Storing click info.");

        Ok(())
    }
}

impl StoreModule {
    pub fn new() -> Self {
        Self {}
    }
}
