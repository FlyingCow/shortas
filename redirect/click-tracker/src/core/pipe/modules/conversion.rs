use anyhow::Result;
use tracing::info;
use chrono::Utc;

use crate::core::{
    ConversionEvent, ConversionFunnelStep, HitData, TrackingPipeContext, 
    tracking_pipe::TrackingModule,
};

/// Module for processing conversion events in the tracking pipeline
#[derive(Clone)]
pub struct ConversionProcessingModule;

#[async_trait::async_trait]
impl TrackingModule for ConversionProcessingModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        // Process conversion events
        if let HitData::Conversion(conversion) = &context.hit.data {
            info!("Processing conversion: {} - {}", conversion.conversion_type, conversion.conversion_name);
            
            // Enrich conversion with user agent data
            if let Some(user_agent) = context.client_ua.clone() {
                // Note: We can't modify the conversion directly since it's in HitData
                // The enrichment will happen in the aggregator
                info!("Conversion enriched with user agent: {}", user_agent.family);
            }
            
            // Enrich with geographic data
            if let Some(country) = &context.client_country {
                info!("Conversion enriched with country: {}", country.iso_code);
            }
            
            // Enrich with device data
            if let Some(device) = context.client_device.clone() {
                info!("Conversion enriched with device: {} - {}", device.family, device.brand.unwrap_or_default());
            }
            
            // Log conversion for debugging
            info!("Conversion processed: {:?}", conversion);
        }
        
        // Process funnel step events
        if let HitData::FunnelStep(funnel_step) = &context.hit.data {
            info!("Processing funnel step: {} - {} (position {})", 
                  funnel_step.funnel_name, funnel_step.step_name, funnel_step.step_position);
            
            // Log funnel step for debugging
            info!("Funnel step processed: {:?}", funnel_step);
        }
        
        Ok(())
    }
}

impl ConversionProcessingModule {
    pub fn new() -> Self {
        Self
    }
}

/// Helper functions for conversion processing
impl ConversionProcessingModule {
    /// Create a conversion event from basic data
    pub fn create_conversion_event(
        conversion_type: String,
        conversion_name: String,
        conversion_value: Option<f64>,
        route_id: Option<String>,
        owner_id: Option<String>,
        creator_id: Option<String>,
        workspace_id: Option<String>,
    ) -> ConversionEvent {
        ConversionEvent {
            id: ulid::Ulid::new().to_string(),
            conversion_type,
            conversion_name,
            conversion_value,
            route_id,
            owner_id,
            creator_id,
            workspace_id,
            created: Utc::now(),
            ..Default::default()
        }
    }
    
    /// Create a funnel step event
    pub fn create_funnel_step_event(
        funnel_name: String,
        step_name: String,
        step_position: u8,
        step_value: Option<f64>,
        route_id: Option<String>,
        owner_id: Option<String>,
        workspace_id: Option<String>,
    ) -> ConversionFunnelStep {
        ConversionFunnelStep {
            id: ulid::Ulid::new().to_string(),
            funnel_name,
            step_name,
            step_position,
            step_completed: 1,
            step_value,
            route_id,
            owner_id,
            workspace_id,
            step_created: Utc::now(),
            funnel_started: Some(Utc::now()),
            ..Default::default()
        }
    }
}
