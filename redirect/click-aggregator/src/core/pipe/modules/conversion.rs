use anyhow::Result;
use tracing::info;

use crate::core::{AggsPipeContext, aggs_pipe::AggsModule, ClickStreamItem};

/// Module for processing conversion events in the aggregation pipeline
#[derive(Clone)]
pub struct ConversionProcessingModule;

#[async_trait::async_trait]
impl AggsModule for ConversionProcessingModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        let dest = context.click.dest.clone();

        // Check if this is a conversion event by looking at the dest field
        if dest.starts_with("conversion:") {
            info!("Processing conversion event: {}", dest);

            // Parse conversion type and name from dest field
            let parts: Vec<&str> = dest.split(':').collect();
            if parts.len() >= 3 {
                let conversion_type = parts[1].to_string();
                let conversion_name = parts[2].to_string();

                // Add conversion metadata to context
                context.add_string("conversion_type", conversion_type.clone());
                context.add_string("conversion_name", conversion_name.clone());
                context.add_bool("is_conversion", true);

                info!("Conversion processed: {} - {}", conversion_type, conversion_name);
            }
        } else if dest.starts_with("funnel:") {
            info!("Processing funnel step event: {}", dest);

            // Parse funnel data from dest field
            let parts: Vec<&str> = dest.split(':').collect();
            if parts.len() >= 4 {
                let funnel_name = parts[1].to_string();
                let step_name = parts[2].to_string();
                let step_position = parts[3].parse::<u8>().unwrap_or(0);

                // Add funnel metadata to context
                context.add_string("funnel_name", funnel_name.clone());
                context.add_string("step_name", step_name.clone());
                context.add_num("step_position", step_position as f64);
                context.add_bool("is_funnel_step", true);

                info!("Funnel step processed: {} - {} (position {})",
                      funnel_name, step_name, step_position);
            }
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
    /// Check if a ClickStreamItem is a conversion event
    pub fn is_conversion_event(click: &ClickStreamItem) -> bool {
        click.dest.starts_with("conversion:")
    }

    /// Check if a ClickStreamItem is a funnel step event
    pub fn is_funnel_step_event(click: &ClickStreamItem) -> bool {
        click.dest.starts_with("funnel:")
    }

    /// Extract conversion type from dest field
    pub fn extract_conversion_type(click: &ClickStreamItem) -> Option<String> {
        if click.dest.starts_with("conversion:") {
            let parts: Vec<&str> = click.dest.split(':').collect();
            if parts.len() >= 2 {
                return Some(parts[1].to_string());
            }
        }
        None
    }

    /// Extract conversion name from dest field
    pub fn extract_conversion_name(click: &ClickStreamItem) -> Option<String> {
        if click.dest.starts_with("conversion:") {
            let parts: Vec<&str> = click.dest.split(':').collect();
            if parts.len() >= 3 {
                return Some(parts[2].to_string());
            }
        }
        None
    }

    /// Extract funnel name from dest field
    pub fn extract_funnel_name(click: &ClickStreamItem) -> Option<String> {
        if click.dest.starts_with("funnel:") {
            let parts: Vec<&str> = click.dest.split(':').collect();
            if parts.len() >= 2 {
                return Some(parts[1].to_string());
            }
        }
        None
    }

    /// Extract step name from dest field
    pub fn extract_step_name(click: &ClickStreamItem) -> Option<String> {
        if click.dest.starts_with("funnel:") {
            let parts: Vec<&str> = click.dest.split(':').collect();
            if parts.len() >= 3 {
                return Some(parts[2].to_string());
            }
        }
        None
    }

    /// Extract step position from dest field
    pub fn extract_step_position(click: &ClickStreamItem) -> Option<u8> {
        if click.dest.starts_with("funnel:") {
            let parts: Vec<&str> = click.dest.split(':').collect();
            if parts.len() >= 4 {
                return parts[3].parse::<u8>().ok();
            }
        }
        None
    }
}
