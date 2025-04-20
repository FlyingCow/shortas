use anyhow::Result;
use tracing::info;

use crate::{
    adapters::ClickAggsRegistrarType,
    core::{
        ClickStreamItem, HitData, TrackingPipeContext, aggs::ClickAggsRegistrar,
        tracking_pipe::TrackingModule,
    },
};

#[derive(Clone)]
pub struct AggregateModule {
    click_aggs_registrar: ClickAggsRegistrarType,
}

#[async_trait::async_trait()]
impl TrackingModule for AggregateModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
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

            stream_item.is_bot = context.spider;
        }

        if let Some(country) = &context.client_country {
            stream_item.country = Some(country.iso_code.clone());
        }

        if let Some(session) = &context.session {
            stream_item.session_clicks = Some(session.count);
            stream_item.session_first = Some(session.first);
            stream_item.is_unique = session.count == 1;
        }

        info!("{}", serde_json::json!(stream_item));

        self.click_aggs_registrar.register(stream_item).await?;
        Ok(())
    }
}

impl AggregateModule {
    pub fn new(click_aggs_registrar: ClickAggsRegistrarType) -> Self {
        Self {
            click_aggs_registrar,
        }
    }
}
