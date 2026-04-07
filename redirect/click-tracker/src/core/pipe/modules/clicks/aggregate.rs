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
        
        // Handle conversion events
        else if let HitData::Conversion(conversion) = &context.hit.data {
            // Create a ClickStreamItem for conversion (this will be processed by click-aggregator)
            let mut stream_item = ClickStreamItem {
                id: conversion.id.clone(),
                created: conversion.created,
                ..Default::default()
            };

            // Copy conversion data to stream item - use as_ref().map() to avoid unnecessary clones
            stream_item.owner_id = conversion.owner_id.as_ref().map(|s| s.to_owned());
            stream_item.creator_id = conversion.creator_id.as_ref().map(|s| s.to_owned());
            stream_item.route_id = conversion.route_id.as_ref().map(|s| s.to_owned());
            stream_item.workspace_id = conversion.workspace_id.as_ref().map(|s| s.to_owned());
            stream_item.ip = conversion.ip.map(|ip| ip.to_string());

            // Geographic data
            stream_item.continent = conversion.continent.as_ref().map(|s| s.to_owned());
            stream_item.country = conversion.country.as_ref().map(|s| s.to_owned());
            stream_item.location = conversion.location.as_ref().map(|s| s.to_owned());

            // Device data
            stream_item.device_family = conversion.device_family.as_ref().map(|s| s.to_owned());
            stream_item.device_brand = conversion.device_brand.as_ref().map(|s| s.to_owned());
            stream_item.device_model = conversion.device_model.as_ref().map(|s| s.to_owned());
            stream_item.os_family = conversion.os_family.as_ref().map(|s| s.to_owned());
            stream_item.os_version = conversion.os_version.as_ref().map(|s| s.to_owned());
            stream_item.user_agent_family = conversion.user_agent_family.as_ref().map(|s| s.to_owned());
            stream_item.user_agent_version = conversion.user_agent_version.as_ref().map(|s| s.to_owned());

            // Set conversion-specific fields
            stream_item.dest = Some(format!("conversion:{}:{}", conversion.conversion_type, conversion.conversion_name));
            stream_item.is_unique = conversion.is_unique.unwrap_or(1) == 1;
            stream_item.is_bot = false; // Conversions are typically not bots

            self.click_aggs_registrar.register(stream_item).await?;
        }
        
        // Handle funnel step events
        else if let HitData::FunnelStep(funnel_step) = &context.hit.data {
            // Create a ClickStreamItem for funnel step
            let mut stream_item = ClickStreamItem {
                id: funnel_step.id.clone(),
                created: funnel_step.step_created,
                ..Default::default()
            };

            // Copy funnel step data to stream item - use as_ref().map() to avoid unnecessary clones
            stream_item.owner_id = funnel_step.owner_id.as_ref().map(|s| s.to_owned());
            stream_item.workspace_id = funnel_step.workspace_id.as_ref().map(|s| s.to_owned());
            stream_item.route_id = funnel_step.route_id.as_ref().map(|s| s.to_owned());

            // Set funnel-specific fields
            stream_item.dest = Some(format!("funnel:{}:{}:{}",
                funnel_step.funnel_name, funnel_step.step_name, funnel_step.step_position));
            stream_item.is_unique = funnel_step.step_completed == 1;
            stream_item.is_bot = false; // Funnel steps are typically not bots

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
