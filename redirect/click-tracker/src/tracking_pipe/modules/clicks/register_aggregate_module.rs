use anyhow::Result;

use crate::{
    core::{click_aggs_register::BaseClickAggsRegistrar, tracking_pipe::TrackingPipeContext},
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct RegisterAggregateModule {
    click_aggs_registrar: Box<dyn BaseClickAggsRegistrar + Sync + Send + 'static>,
}

#[async_trait::async_trait()]
impl BaseTrackingModule for RegisterAggregateModule {
    async fn execute(&self, _context: &mut TrackingPipeContext) -> Result<()> {
        println!("{}", "Executing RegisterAggregateModule");
        Ok(())
    }
}

impl RegisterAggregateModule {
    pub fn new(click_aggs_registrar: Box<dyn BaseClickAggsRegistrar + Sync + Send + 'static>) -> Self {
        Self { click_aggs_registrar }
    }
}