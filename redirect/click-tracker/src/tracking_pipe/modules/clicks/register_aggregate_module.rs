use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    core::{click_aggs_register::BaseClickAggsRegistrar, tracking_pipe::TrackingPipeContext},
    model::ClickStreamItem,
    tracking_pipe::tracking_module::BaseTrackingModule,
};

#[derive(Clone)]
pub struct RegisterAggregateModule {
    click_aggs_registrar: Box<dyn BaseClickAggsRegistrar + Sync + Send + 'static>,
}

#[async_trait::async_trait()]
impl BaseTrackingModule for RegisterAggregateModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        println!("{}", "Executing RegisterAggregateModule");
        self
            .click_aggs_registrar
            .as_mut()
            .register(ClickStreamItem {
                id: context.hit.id.clone(),
                owner_id: context.hit.id.clone(),
                creator_id: context.hit.id.clone(),
                route_id: context.hit.id.clone(),
                workspace_id: context.hit.id.clone(),
                created: context.utc,
                dest: None,
                ip: None,
                continent: None,
                country: None,
                location: None,
                os_family: None,
                os_version: None,
                user_agent_family: None,
                user_agent_version: None,
                device_brand: None,
                device_family: None,
                device_model: None,
                first_click: None,
                is_uniqueu: true,
                is_bot: false,
            }).await?;
        Ok(())
    }
}

impl RegisterAggregateModule {
    pub fn new(
        click_aggs_registrar: Box<dyn BaseClickAggsRegistrar + Sync + Send + 'static>,
    ) -> Self {
        Self {
            click_aggs_registrar,
        }
    }
}
