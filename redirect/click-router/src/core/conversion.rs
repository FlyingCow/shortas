use std::net::IpAddr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// Conversion data structure that flows through the pipeline
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversionEvent {
    pub id: String,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub workspace_id: Option<String>,
    
    // Conversion details
    pub conversion_type: String,
    pub conversion_name: String,
    pub conversion_value: Option<f64>,
    
    // Attribution data
    pub attributed_click_id: Option<String>,
    pub attribution_type: String,
    pub attribution_window_hours: u32,
    
    // User and session data
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip: Option<IpAddr>,
    
    // Geographic data
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    
    // Device data
    pub device_family: Option<String>,
    pub device_brand: Option<String>,
    pub device_model: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    
    // Timestamps
    pub created: DateTime<Utc>,
    pub click_created: Option<DateTime<Utc>>,
    
    // Additional metadata
    pub metadata: Option<String>,
    pub referrer: Option<String>,
    
    // Flags
    pub is_unique: Option<u8>,
}

impl Default for ConversionEvent {
    fn default() -> Self {
        Self {
            id: Ulid::new().to_string(),
            owner_id: None,
            creator_id: None,
            route_id: None,
            workspace_id: None,
            conversion_type: String::new(),
            conversion_name: String::new(),
            conversion_value: None,
            attributed_click_id: None,
            attribution_type: "direct".to_string(),
            attribution_window_hours: 24,
            user_id: None,
            session_id: None,
            ip: None,
            continent: None,
            country: None,
            location: None,
            device_family: None,
            device_brand: None,
            device_model: None,
            os_family: None,
            os_version: None,
            user_agent_family: None,
            user_agent_version: None,
            created: Utc::now(),
            click_created: None,
            metadata: None,
            referrer: None,
            is_unique: Some(1),
        }
    }
}

/// Conversion funnel step data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversionFunnelStep {
    pub id: String,
    pub owner_id: Option<String>,
    pub workspace_id: Option<String>,
    
    // Funnel definition
    pub funnel_name: String,
    pub funnel_steps: Vec<String>,
    
    // User journey
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub route_id: Option<String>,
    
    // Step completion data
    pub step_name: String,
    pub step_position: u8,
    pub step_completed: u8,
    pub step_value: Option<f64>,
    
    // Timestamps
    pub step_created: DateTime<Utc>,
    pub funnel_started: Option<DateTime<Utc>>,
    
    // Additional data
    pub metadata: Option<String>,
}

impl Default for ConversionFunnelStep {
    fn default() -> Self {
        Self {
            id: Ulid::new().to_string(),
            owner_id: None,
            workspace_id: None,
            funnel_name: String::new(),
            funnel_steps: Vec::new(),
            user_id: None,
            session_id: None,
            route_id: None,
            step_name: String::new(),
            step_position: 1,
            step_completed: 1,
            step_value: None,
            step_created: Utc::now(),
            funnel_started: None,
            metadata: None,
        }
    }
}
