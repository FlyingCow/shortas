use async_trait::async_trait;
use anyhow::Result;
use crate::model::clickstream::{ClickStreamQuery, ClickStreamResponse};
use crate::dto::clickstream_dto::{
    DailyStatsDto, HourlyStatsDto, GeographicStatsDto, DeviceStatsDto,
    BrowserStatsDto, RoutePerformanceDto, TopDestinationDto, TrafficTypeStatsDto
};

/// Trait for click stream data operations
#[async_trait]
pub trait ClickStreamStore: Send + Sync {
    /// Query click stream data with filters
    async fn query_clickstream(&self, query: &ClickStreamQuery) -> Result<ClickStreamResponse>;

    /// Get total count of click stream items matching the query
    async fn count_clickstream(&self, query: &ClickStreamQuery) -> Result<u64>;

    /// Get daily statistics
    async fn get_daily_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<DailyStatsDto>>;

    /// Get hourly statistics
    async fn get_hourly_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_hour: Option<&str>, to_hour: Option<&str>) -> Result<Vec<HourlyStatsDto>>;

    /// Get geographic statistics
    async fn get_geographic_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<GeographicStatsDto>>;

    /// Get device statistics
    async fn get_device_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<DeviceStatsDto>>;

    /// Get browser statistics
    async fn get_browser_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>) -> Result<Vec<BrowserStatsDto>>;

    /// Get route performance statistics
    async fn get_route_performance(&self, owner_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>, limit: Option<u32>) -> Result<Vec<RoutePerformanceDto>>;

    /// Get top destinations
    async fn get_top_destinations(&self, owner_id: Option<&str>, route_id: Option<&str>, from_date: Option<&str>, to_date: Option<&str>, limit: Option<u32>) -> Result<Vec<TopDestinationDto>>;

    /// Get traffic type statistics (bot vs human)
    async fn get_traffic_type_stats(&self, owner_id: Option<&str>, route_id: Option<&str>, from_hour: Option<&str>, to_hour: Option<&str>) -> Result<Vec<TrafficTypeStatsDto>>;
}
