use anyhow::Result;

use crate::{
    core::{click_aggs_register::BaseClickAggsRegistrar, tracking_pipe::TrackingPipeContext},
    model::{hit::HitData, ClickStreamItem},
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

        let mut stream_item = ClickStreamItem {
            id: context.hit.id.clone(),
            created: context.utc,
            ..Default::default()
        };

        if let Some(ip) = context.hit.ip {
            stream_item.ip = Some(ip.to_string());
        }

        if let Some(route) = &context.hit.route {
            stream_item.route_id = route.id.clone();
            stream_item.creator_id = route.creator_id.clone();
            stream_item.owner_id = route.owner_id.clone();
            stream_item.workspace_id = route.workspace_id.clone();
        }

        if let HitData::Click(click) = &context.hit.data {
            stream_item.dest = click.dest.clone();
        }

        if let Some(user_agent) = context.client_ua.clone() {
            stream_item.user_agent_family = Some(user_agent.family);
            stream_item.user_agent_version = user_agent.major;
        }

        if let Some(os) = context.client_os.clone() {
            stream_item.os_family = Some(os.family);
            stream_item.os_version = os.major;
        }

        if let Some(device) = context.client_device.clone() {
            stream_item.device_brand = device.brand;
            stream_item.device_family = Some(device.family);
            stream_item.device_model = device.model;
        }

        if let Some(country) = context.client_country.clone() {
            stream_item.country = Some(country.iso_code);
        }

        println!("{}", serde_json::json!(stream_item));

        self.click_aggs_registrar
            .as_mut()
            .register(stream_item)
            .await?;
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
