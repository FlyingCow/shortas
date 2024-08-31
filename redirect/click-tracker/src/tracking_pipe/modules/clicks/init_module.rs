use anyhow::Result;

use crate::{
    core::tracking_pipe::TrackingPipeContext, model::hit::{Click, HitData},
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct InitModule;

#[async_trait::async_trait()]
impl BaseTrackingModule for InitModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {


        println!("{}", "Executing InitModule");
        Ok(())
    }
}
