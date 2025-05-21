use anyhow::Result;
use tracing::info;

use crate::core::{TrackingPipeContext, tracking_pipe::TrackingModule};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl TrackingModule for InitModule {
    async fn execute(&mut self, _context: &mut TrackingPipeContext) -> Result<()> {
        Ok(())
    }
}
