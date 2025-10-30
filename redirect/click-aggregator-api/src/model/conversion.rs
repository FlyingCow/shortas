use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Conversion data model for ClickHouse storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversion {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub conversion_type: String,
    pub conversion_name: String,
    pub conversion_value: Decimal,
    pub attributed_click_id: String,
    pub attribution_type: String,
    pub attribution_window_hours: u32,
    pub user_id: String,
    pub session_id: String,
    pub ip: String,
    pub continent: String,
    pub country: String,
    pub location: String,
    pub device_family: String,
    pub device_brand: String,
    pub device_model: String,
    pub os_family: String,
    pub os_version: String,
    pub user_agent_family: String,
    pub user_agent_version: String,
    pub created: DateTime<Utc>,
    pub click_created: DateTime<Utc>,
    pub metadata: String,
    pub referrer: String,
    pub is_unique: u8,
}

impl Default for Conversion {
    fn default() -> Self {
        Self {
            id: String::new(),
            owner_id: String::new(),
            creator_id: String::new(),
            route_id: String::new(),
            workspace_id: String::new(),
            conversion_type: String::new(),
            conversion_name: String::new(),
            conversion_value: Decimal::ZERO,
            attributed_click_id: String::new(),
            attribution_type: "direct".to_string(),
            attribution_window_hours: 24,
            user_id: "_unknown".to_string(),
            session_id: "_unknown".to_string(),
            ip: String::new(),
            continent: "_unknown".to_string(),
            country: "_unknown".to_string(),
            location: "_unknown".to_string(),
            device_family: "_unknown".to_string(),
            device_brand: "_unknown".to_string(),
            device_model: "_unknown".to_string(),
            os_family: "_unknown".to_string(),
            os_version: "_unknown".to_string(),
            user_agent_family: "_unknown".to_string(),
            user_agent_version: "_unknown".to_string(),
            created: Utc::now(),
            click_created: Utc::now(),
            metadata: "{}".to_string(),
            referrer: "_unknown".to_string(),
            is_unique: 1,
        }
    }
}

/// Conversion attribution data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionAttribution {
    pub conversion_id: String,
    pub click_id: String,
    pub owner_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub attribution_weight: Decimal,
    pub attribution_position: u8,
    pub attribution_type: String,
    pub click_created: DateTime<Utc>,
    pub conversion_created: DateTime<Utc>,
    pub time_to_conversion_seconds: u32,
    pub session_id: String,
    pub user_id: String,
    pub country: String,
    pub device_family: String,
    pub user_agent_family: String,
}

impl Default for ConversionAttribution {
    fn default() -> Self {
        Self {
            conversion_id: String::new(),
            click_id: String::new(),
            owner_id: String::new(),
            route_id: String::new(),
            workspace_id: String::new(),
            attribution_weight: Decimal::ONE,
            attribution_position: 1,
            attribution_type: "direct".to_string(),
            click_created: Utc::now(),
            conversion_created: Utc::now(),
            time_to_conversion_seconds: 0,
            session_id: "_unknown".to_string(),
            user_id: "_unknown".to_string(),
            country: "_unknown".to_string(),
            device_family: "_unknown".to_string(),
            user_agent_family: "_unknown".to_string(),
        }
    }
}

/// Conversion funnel data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionFunnel {
    pub id: String,
    pub owner_id: String,
    pub workspace_id: String,
    pub funnel_name: String,
    pub funnel_steps: Vec<String>,
    pub user_id: String,
    pub session_id: String,
    pub route_id: String,
    pub step_name: String,
    pub step_position: u8,
    pub step_completed: u8,
    pub step_value: Decimal,
    pub step_created: DateTime<Utc>,
    pub funnel_started: DateTime<Utc>,
    pub metadata: String,
}

impl Default for ConversionFunnel {
    fn default() -> Self {
        Self {
            id: String::new(),
            owner_id: String::new(),
            workspace_id: String::new(),
            funnel_name: String::new(),
            funnel_steps: Vec::new(),
            user_id: "_unknown".to_string(),
            session_id: "_unknown".to_string(),
            route_id: String::new(),
            step_name: String::new(),
            step_position: 1,
            step_completed: 1,
            step_value: Decimal::ZERO,
            step_created: Utc::now(),
            funnel_started: Utc::now(),
            metadata: "{}".to_string(),
        }
    }
}

/// Conversion goal data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionGoal {
    pub id: String,
    pub owner_id: String,
    pub workspace_id: String,
    pub route_id: String,
    pub goal_name: String,
    pub goal_type: String,
    pub target_value: Decimal,
    pub target_period: String,
    pub attribution_window_hours: u32,
    pub is_active: u8,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Default for ConversionGoal {
    fn default() -> Self {
        Self {
            id: String::new(),
            owner_id: String::new(),
            workspace_id: String::new(),
            route_id: String::new(),
            goal_name: String::new(),
            goal_type: "conversion_rate".to_string(),
            target_value: Decimal::ZERO,
            target_period: "daily".to_string(),
            attribution_window_hours: 24,
            is_active: 1,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

/// Conversion types enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ConversionType {
    Purchase,
    Signup,
    Download,
    FormSubmission,
    Custom(String),
}

impl ToString for ConversionType {
    fn to_string(&self) -> String {
        match self {
            ConversionType::Purchase => "purchase".to_string(),
            ConversionType::Signup => "signup".to_string(),
            ConversionType::Download => "download".to_string(),
            ConversionType::FormSubmission => "form_submission".to_string(),
            ConversionType::Custom(name) => format!("custom_{}", name),
        }
    }
}

/// Attribution types enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum AttributionType {
    Direct,
    Session,
    TimeBased,
    MultiTouch,
}

impl ToString for AttributionType {
    fn to_string(&self) -> String {
        match self {
            AttributionType::Direct => "direct".to_string(),
            AttributionType::Session => "session".to_string(),
            AttributionType::TimeBased => "time_based".to_string(),
            AttributionType::MultiTouch => "multi_touch".to_string(),
        }
    }
}

/// Goal types enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum GoalType {
    ConversionRate,
    Revenue,
    Custom(String),
}

impl ToString for GoalType {
    fn to_string(&self) -> String {
        match self {
            GoalType::ConversionRate => "conversion_rate".to_string(),
            GoalType::Revenue => "revenue".to_string(),
            GoalType::Custom(name) => format!("custom_{}", name),
        }
    }
}

/// Target period enum
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum TargetPeriod {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl ToString for TargetPeriod {
    fn to_string(&self) -> String {
        match self {
            TargetPeriod::Hourly => "hourly".to_string(),
            TargetPeriod::Daily => "daily".to_string(),
            TargetPeriod::Weekly => "weekly".to_string(),
            TargetPeriod::Monthly => "monthly".to_string(),
        }
    }
}
