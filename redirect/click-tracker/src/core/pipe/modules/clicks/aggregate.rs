use anyhow::Result;

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
        // Handle regular click events
        if let HitData::Click(_) = &context.hit.data {
            let mut stream_item = ClickStreamItem {
                id: context.hit.id.clone(),
                created: context.utc,
                ..Default::default()
            };

            if let Some(ip) = context.hit.ip {
                stream_item.ip = Some(ip.to_string());
            }

            if let Some(route) = &context.hit.route {
                stream_item.route_id = route.id.as_ref().map(|s| s.to_owned());
                stream_item.creator_id = route.creator_id.as_ref().map(|s| s.to_owned());
                stream_item.owner_id = route.owner_id.as_ref().map(|s| s.to_owned());
                stream_item.workspace_id = route.workspace_id.as_ref().map(|s| s.to_owned());
            }

            if let HitData::Click(click) = &context.hit.data {
                stream_item.dest = click.dest.as_ref().map(|s| s.to_owned());
            }

            // Take ownership of user agent data since we don't need it after this module
            if let Some(user_agent) = context.client_ua.take() {
                stream_item.user_agent_family = Some(user_agent.family);
                stream_item.user_agent_version = user_agent.major;
            }

            if let Some(os) = context.client_os.take() {
                stream_item.os_family = Some(os.family);
                stream_item.os_version = os.major;
            }

            if let Some(device) = context.client_device.take() {
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

            stream_item.is_trackable = context.hit.trackable;

            // Propagate trace data from click-router
            stream_item.trace = context.hit.trace.clone();

            // Log trace event and stream_item JSON for debug routes
            if let Some(ref trace) = stream_item.trace {
                if let Ok(stream_item_json) = serde_json::to_string(&stream_item) {
                    tracing::warn!(
                        trace_id = %trace.trace_id,
                        route_id = %stream_item.route_id.as_deref().unwrap_or(""),
                        service = "click-tracker",
                        step = "AggregateModule",
                        total_ms = %trace.total_ms,
                        stream_item = %stream_item_json,
                        "Debug trace: sending to click-aggregator"
                    );
                }
            }

            self.click_aggs_registrar.register(stream_item).await?;
        }

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
