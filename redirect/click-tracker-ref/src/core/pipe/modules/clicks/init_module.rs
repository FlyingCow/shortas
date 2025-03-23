use anyhow::Result;

use crate::core::{tracking_pipe::TrackingModule, TrackingPipeContext};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait]
impl TrackingModule for InitModule {
    async fn execute(&mut self, _context: &mut TrackingPipeContext) -> Result<()> {
        Ok(())
    }
}
