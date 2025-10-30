use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::model::conversion::{
    Conversion, ConversionAttribution, ConversionFunnel, ConversionGoal,
};
use crate::dto::conversion_dto::{
    ConversionRatesDto, ConversionAttributionAnalysisDto, ConversionFunnelPerformanceDto,
    RevenueAnalyticsDto, GeographicConversionDto, DeviceConversionDto, HourlyConversionDto,
    ConversionGoalsPerformanceDto, MultiTouchAttributionDto, ConversionCohortDto,
    ConversionQueryDto, ConversionResponseDto,
};

/// Trait for conversion data storage operations
#[async_trait]
pub trait ConversionStore {
    /// Store a new conversion
    async fn store_conversion(&self, conversion: Conversion) -> Result<()>;
    
    /// Store conversion attribution data
    async fn store_conversion_attribution(&self, attribution: ConversionAttribution) -> Result<()>;
    
    /// Store conversion funnel step
    async fn store_conversion_funnel(&self, funnel: ConversionFunnel) -> Result<()>;
    
    /// Store conversion goal
    async fn store_conversion_goal(&self, goal: ConversionGoal) -> Result<()>;
    
    /// Get conversions with optional filters
    async fn get_conversions(&self, query: ConversionQueryDto) -> Result<ConversionResponseDto>;
    
    /// Get conversion rates by route and time
    async fn get_conversion_rates(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConversionRatesDto>>;
    
    /// Get conversion attribution analysis
    async fn get_conversion_attribution_analysis(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConversionAttributionAnalysisDto>>;
    
    /// Get conversion funnel performance
    async fn get_conversion_funnel_performance(
        &self,
        owner_id: Option<&str>,
        funnel_name: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConversionFunnelPerformanceDto>>;
    
    /// Get revenue analytics
    async fn get_revenue_analytics(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<RevenueAnalyticsDto>>;
    
    /// Get geographic conversion analysis
    async fn get_geographic_conversion_analysis(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<GeographicConversionDto>>;
    
    /// Get device conversion analysis
    async fn get_device_conversion_analysis(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<DeviceConversionDto>>;
    
    /// Get hourly conversion tracking
    async fn get_hourly_conversion_tracking(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_hour: Option<&str>,
        to_hour: Option<&str>,
    ) -> Result<Vec<HourlyConversionDto>>;
    
    /// Get conversion goals performance
    async fn get_conversion_goals_performance(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConversionGoalsPerformanceDto>>;
    
    /// Get multi-touch attribution analysis
    async fn get_multi_touch_attribution_analysis(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<MultiTouchAttributionDto>>;
    
    /// Get conversion cohort analysis
    async fn get_conversion_cohort_analysis(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<ConversionCohortDto>>;
    
    /// Get conversion goals for a route
    async fn get_conversion_goals(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
    ) -> Result<Vec<ConversionGoal>>;
    
    /// Update conversion goal
    async fn update_conversion_goal(&self, goal: ConversionGoal) -> Result<()>;
    
    /// Delete conversion goal
    async fn delete_conversion_goal(&self, goal_id: &str) -> Result<()>;
    
    /// Get conversion funnels for a route
    async fn get_conversion_funnels(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
    ) -> Result<Vec<ConversionFunnel>>;
    
    /// Get conversion attribution for a specific conversion
    async fn get_conversion_attribution(
        &self,
        conversion_id: &str,
    ) -> Result<Vec<ConversionAttribution>>;
    
    /// Calculate conversion rate for a route
    async fn calculate_conversion_rate(
        &self,
        owner_id: &str,
        route_id: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<f64>;
    
    /// Calculate revenue per click
    async fn calculate_revenue_per_click(
        &self,
        owner_id: &str,
        route_id: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<f64>;
    
    /// Get conversion summary for dashboard
    async fn get_conversion_summary(
        &self,
        owner_id: Option<&str>,
        route_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<ConversionSummary>;
}

/// Conversion summary for dashboard display
#[derive(Debug, Clone)]
pub struct ConversionSummary {
    pub total_conversions: u64,
    pub total_revenue: f64,
    pub avg_conversion_value: f64,
    pub conversion_rate: f64,
    pub revenue_per_click: f64,
    pub unique_converting_users: u64,
    pub top_conversion_types: Vec<(String, u64)>,
    pub top_converting_routes: Vec<(String, u64)>,
}

impl Default for ConversionSummary {
    fn default() -> Self {
        Self {
            total_conversions: 0,
            total_revenue: 0.0,
            avg_conversion_value: 0.0,
            conversion_rate: 0.0,
            revenue_per_click: 0.0,
            unique_converting_users: 0,
            top_conversion_types: Vec::new(),
            top_converting_routes: Vec::new(),
        }
    }
}
