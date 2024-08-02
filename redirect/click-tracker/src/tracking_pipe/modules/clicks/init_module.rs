use anyhow::Result;

use crate::{
    core::tracking_pipe::TrackingPipeContext, tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait()]
impl BaseTrackingModule for InitModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {

        println!("{}", serde_json::json!(context.hit));
        println!("{}", "Executing InitModule");
        Ok(())
    }
}
