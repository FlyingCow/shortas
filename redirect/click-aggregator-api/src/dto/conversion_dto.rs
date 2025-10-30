use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Conversion DTO for API responses
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionDto {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub conversion_type: String,
    pub conversion_name: String,
    pub conversion_value: f64,
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
    pub metadata: serde_json::Value,
    pub referrer: String,
    pub is_unique: bool,
}

/// Conversion attribution DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionAttributionDto {
    pub conversion_id: String,
    pub click_id: String,
    pub owner_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub attribution_weight: f64,
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

/// Conversion funnel DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionFunnelDto {
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
    pub step_completed: bool,
    pub step_value: f64,
    pub step_created: DateTime<Utc>,
    pub funnel_started: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Conversion goal DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionGoalDto {
    pub id: String,
    pub owner_id: String,
    pub workspace_id: String,
    pub route_id: String,
    pub goal_name: String,
    pub goal_type: String,
    pub target_value: f64,
    pub target_period: String,
    pub attribution_window_hours: u32,
    pub is_active: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// Conversion rates DTO for analytics
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRatesDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub conversion_type: String,
    pub conversion_name: String,
    pub total_conversions: u64,
    pub total_conversion_value: f64,
    pub avg_conversion_value: f64,
    pub max_conversion_value: f64,
    pub min_conversion_value: f64,
    pub unique_converting_users: u64,
    pub unique_converting_sessions: u64,
    pub unique_converting_ips: u64,
}

/// Conversion attribution analysis DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionAttributionAnalysisDto {
    pub owner_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub attribution_type: String,
    pub attribution_count: u64,
    pub total_attribution_weight: f64,
    pub avg_time_to_conversion: f64,
    pub min_time_to_conversion: f64,
    pub max_time_to_conversion: f64,
    pub unique_conversions: u64,
    pub unique_clicks: u64,
    pub unique_users: u64,
}

/// Conversion funnel performance DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionFunnelPerformanceDto {
    pub owner_id: String,
    pub workspace_id: String,
    pub funnel_name: String,
    pub date: String,
    pub step_name: String,
    pub step_position: u8,
    pub step_completions: u64,
    pub unique_users_at_step: u64,
    pub unique_sessions_at_step: u64,
    pub total_step_value: f64,
    pub avg_step_value: f64,
    pub completion_rate: f64,
    pub drop_off_rate: f64,
}

/// Revenue analytics DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevenueAnalyticsDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub total_conversions: u64,
    pub total_revenue: f64,
    pub avg_order_value: f64,
    pub unique_customers: u64,
    pub unique_converting_sessions: u64,
    pub revenue_per_click: f64,
    pub conversion_rate: f64,
}

/// Geographic conversion analysis DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeographicConversionDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub continent: String,
    pub country: String,
    pub location: String,
    pub conversion_type: String,
    pub total_conversions: u64,
    pub total_conversion_value: f64,
    pub unique_converting_users: u64,
    pub unique_converting_ips: u64,
}

/// Device conversion analysis DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConversionDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub device_family: String,
    pub device_brand: String,
    pub device_model: String,
    pub os_family: String,
    pub os_version: String,
    pub user_agent_family: String,
    pub user_agent_version: String,
    pub conversion_type: String,
    pub total_conversions: u64,
    pub total_conversion_value: f64,
    pub unique_converting_users: u64,
}

/// Hourly conversion tracking DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HourlyConversionDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub hour: DateTime<Utc>,
    pub conversion_type: String,
    pub total_conversions: u64,
    pub total_conversion_value: f64,
    pub unique_converting_users: u64,
    pub unique_converting_sessions: u64,
}

/// Conversion goals performance DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionGoalsPerformanceDto {
    pub owner_id: String,
    pub workspace_id: String,
    pub route_id: String,
    pub goal_name: String,
    pub goal_type: String,
    pub target_value: f64,
    pub target_period: String,
    pub date: String,
    pub actual_conversions: u64,
    pub actual_value: f64,
    pub goal_achievement_percentage: f64,
}

/// Multi-touch attribution analysis DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MultiTouchAttributionDto {
    pub owner_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub date: String,
    pub attribution_position: u8,
    pub attribution_count: u64,
    pub total_weight: f64,
    pub avg_time_to_conversion: f64,
    pub unique_conversions: u64,
    pub unique_clicks: u64,
}

/// Conversion cohort analysis DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionCohortDto {
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    pub cohort_date: String,
    pub conversion_date: String,
    pub funnel_name: String,
    pub step_name: String,
    pub conversions: u64,
    pub unique_users: u64,
    pub total_value: f64,
}

/// Conversion query parameters DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionQueryDto {
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub workspace_id: Option<String>,
    pub conversion_type: Option<String>,
    pub conversion_name: Option<String>,
    pub created_from: Option<String>,
    pub created_to: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Conversion response DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionResponseDto {
    pub items: Vec<ConversionDto>,
    pub total: u64,
    pub offset: u32,
    pub limit: u32,
    pub has_more: bool,
}

/// Create conversion request DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversionDto {
    pub route_id: String,
    pub conversion_type: String,
    pub conversion_name: String,
    pub conversion_value: Option<f64>,
    pub attributed_click_id: Option<String>,
    pub attribution_type: Option<String>,
    pub attribution_window_hours: Option<u32>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Create conversion goal request DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversionGoalDto {
    pub route_id: String,
    pub goal_name: String,
    pub goal_type: String,
    pub target_value: f64,
    pub target_period: String,
    pub attribution_window_hours: Option<u32>,
}

/// Create conversion funnel request DTO
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversionFunnelDto {
    pub funnel_name: String,
    pub funnel_steps: Vec<String>,
    pub route_id: String,
    pub step_name: String,
    pub step_position: u8,
    pub step_value: Option<f64>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
