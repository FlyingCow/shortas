use anyhow::Result;
use tracing::info;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::core::{AggsPipeContext, aggs_pipe::AggsModule, ClickStreamItem};

/// Module for processing conversion events in the aggregation pipeline
#[derive(Clone)]
pub struct ConversionProcessingModule;

#[async_trait::async_trait]
impl AggsModule for ConversionProcessingModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        // Check if this is a conversion event by looking at the dest field
        if let Some(dest) = &context.click.dest {
            if dest.starts_with("conversion:") {
                info!("Processing conversion event: {}", dest);
                
                // Parse conversion type and name from dest field
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 3 {
                    let conversion_type = parts[1];
                    let conversion_name = parts[2];
                    
                    // Add conversion metadata to context
                    context.add_string("conversion_type", conversion_type);
                    context.add_string("conversion_name", conversion_name);
                    context.add_bool("is_conversion", true);
                    
                    info!("Conversion processed: {} - {}", conversion_type, conversion_name);
                }
            }
            else if dest.starts_with("funnel:") {
                info!("Processing funnel step event: {}", dest);
                
                // Parse funnel data from dest field
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 4 {
                    let funnel_name = parts[1];
                    let step_name = parts[2];
                    let step_position = parts[3].parse::<u8>().unwrap_or(0);
                    
                    // Add funnel metadata to context
                    context.add_string("funnel_name", funnel_name);
                    context.add_string("step_name", step_name);
                    context.add_num("step_position", step_position as f64);
                    context.add_bool("is_funnel_step", true);
                    
                    info!("Funnel step processed: {} - {} (position {})", 
                          funnel_name, step_name, step_position);
                }
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
    /// Extract conversion value from ClickStreamItem metadata
    pub fn extract_conversion_value(click: &ClickStreamItem) -> Option<Decimal> {
        // This would need to be implemented based on how conversion value is stored
        // For now, we'll return None as the value would need to be passed through the pipeline
        None
    }
    
    /// Check if a ClickStreamItem is a conversion event
    pub fn is_conversion_event(click: &ClickStreamItem) -> bool {
        click.dest.as_ref()
            .map(|dest| dest.starts_with("conversion:"))
            .unwrap_or(false)
    }
    
    /// Check if a ClickStreamItem is a funnel step event
    pub fn is_funnel_step_event(click: &ClickStreamItem) -> bool {
        click.dest.as_ref()
            .map(|dest| dest.starts_with("funnel:"))
            .unwrap_or(false)
    }
    
    /// Extract conversion type from dest field
    pub fn extract_conversion_type(click: &ClickStreamItem) -> Option<String> {
        if let Some(dest) = &click.dest {
            if dest.starts_with("conversion:") {
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }
    
    /// Extract conversion name from dest field
    pub fn extract_conversion_name(click: &ClickStreamItem) -> Option<String> {
        if let Some(dest) = &click.dest {
            if dest.starts_with("conversion:") {
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 3 {
                    return Some(parts[2].to_string());
                }
            }
        }
        None
    }
    
    /// Extract funnel name from dest field
    pub fn extract_funnel_name(click: &ClickStreamItem) -> Option<String> {
        if let Some(dest) = &click.dest {
            if dest.starts_with("funnel:") {
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                }
            }
        }
        None
    }
    
    /// Extract step name from dest field
    pub fn extract_step_name(click: &ClickStreamItem) -> Option<String> {
        if let Some(dest) = &click.dest {
            if dest.starts_with("funnel:") {
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 3 {
                    return Some(parts[2].to_string());
                }
            }
        }
        None
    }
    
    /// Extract step position from dest field
    pub fn extract_step_position(click: &ClickStreamItem) -> Option<u8> {
        if let Some(dest) = &click.dest {
            if dest.starts_with("funnel:") {
                let parts: Vec<&str> = dest.split(':').collect();
                if parts.len() >= 4 {
                    return parts[3].parse::<u8>().ok();
                }
            }
        }
        None
    }
}
