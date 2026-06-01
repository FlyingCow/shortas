//! ClickStream controller for analytics data.

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use salvo::oapi::ToSchema;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::ApiError;
use crate::domain::traits::RouteInfo;
use crate::infrastructure::http_clients::{
    BrowserDistribution, BrowserStats, ClickStats, DailyStats, DeviceDistribution, DeviceStats,
    GeoDistribution, GeographicStats, HourlyStats, RoutePerformance, TimeSeriesPoint,
    TopDestination, TopRoute, TrafficTypeStats,
};

/// ClickStream event DTO for frontend (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClickStreamEventDto {
    pub id: String,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub route_id: Option<String>,
    pub route_name: Option<String>,
    pub route_domain_name: Option<String>,
    pub workspace_id: Option<String>,
    pub created: String,
    pub dest: Option<String>,
    pub ip: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    pub session_first: Option<String>,
    pub session_clicks: Option<i64>,
    pub is_unique: Option<bool>,
    pub is_bot: Option<bool>,
}

use crate::presentation::middleware::{render_error, render_success, DepotExt};

use super::cors_preflight;

/// Build clickstream controller router.
pub fn clickstream_controller() -> Router {
    Router::with_path("clickstream")
        .options(cors_preflight)
        // Raw clickstream data
        .get(get_clickstream)
        // Route-specific endpoints (legacy)
        .push(
            Router::with_path("routes/<route_id>")
                .get(get_route_stats)
                .options(cors_preflight)
                .push(
                    Router::with_path("timeseries")
                        .get(get_route_time_series)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("geo")
                        .get(get_route_geo)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("devices")
                        .get(get_route_devices)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("browsers")
                        .get(get_route_browsers)
                        .options(cors_preflight),
                ),
        )
        // Workspace-level endpoints (legacy)
        .push(
            Router::with_path("workspaces/<workspace_id>")
                .get(get_workspace_stats)
                .options(cors_preflight)
                .push(
                    Router::with_path("top-routes")
                        .get(get_workspace_top_routes)
                        .options(cors_preflight),
                ),
        )
        // Stats endpoints (new, matching click-aggregator-api)
        .push(
            Router::with_path("stats")
                .options(cors_preflight)
                .get(get_stats)
                .push(
                    Router::with_path("daily")
                        .get(get_daily_stats)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("hourly")
                        .get(get_hourly_stats)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("geographic")
                        .get(get_geographic_stats)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("devices")
                        .get(get_device_stats)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("browsers")
                        .get(get_browser_stats)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("route-performance")
                        .get(get_route_performance)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("top-destinations")
                        .get(get_top_destinations)
                        .options(cors_preflight),
                )
                .push(
                    Router::with_path("traffic-types")
                        .get(get_traffic_type_stats)
                        .options(cors_preflight),
                ),
        )
}

// ============================================================================
// Raw clickstream data endpoint
// ============================================================================

/// Get raw clickstream data.
#[endpoint(
    operation_id = "get_clickstream",
    summary = "Get clickstream data",
    description = "Get raw click events with optional filtering",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("creator_id" = Option<String>, Query, description = "Filter by creator ID"),
        ("routeId" = Option<String>, Query, description = "Filter by route ID"),
        ("workspace_id" = Option<String>, Query, description = "Filter by workspace ID"),
        ("startDate" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("endDate" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("limit" = Option<u32>, Query, description = "Maximum results (default: 100)"),
        ("offset" = Option<u32>, Query, description = "Offset for pagination")
    ),
    responses(
        (status_code = 200, description = "Clickstream data", body = Vec<ClickStreamEventDto>)
    )
)]
pub async fn get_clickstream(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let creator_id: Option<String> = req.query("creator_id");
    let route_id: Option<String> = req.query("routeId");
    let workspace_id: Option<String> = req.query("workspace_id");
    let from: Option<DateTime<Utc>> = req
        .query::<String>("startDate")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("endDate")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let limit: Option<u32> = req.query("limit");
    let offset: Option<u32> = req.query("offset");

    let data = match app_state
        .click_aggregator
        .get_clickstream(
            owner_id.as_deref(),
            creator_id.as_deref(),
            route_id.as_deref(),
            workspace_id.as_deref(),
            from,
            to,
            limit,
            offset,
        )
        .await
    {
        Ok(data) => data,
        Err(e) => {
            render_error(res, ApiError::external_service(e.to_string()));
            return;
        }
    };

    // Collect unique route IDs for enrichment
    let route_ids: Vec<Uuid> = data
        .items
        .iter()
        .filter_map(|item| item.route_id.as_ref())
        .filter_map(|id| Uuid::parse_str(id).ok())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Fetch route info in a single query
    let route_info_map: HashMap<Uuid, RouteInfo> = match app_state
        .route_repo
        .get_route_info_by_ids(&route_ids)
        .await
    {
        Ok(infos) => infos.into_iter().map(|info| (info.id, info)).collect(),
        Err(_) => HashMap::new(), // Continue without enrichment if query fails
    };

    // Convert and enrich events
    let events: Vec<ClickStreamEventDto> = data
        .items
        .into_iter()
        .map(|item| {
            let route_info = item
                .route_id
                .as_ref()
                .and_then(|id| Uuid::parse_str(id).ok())
                .and_then(|id| route_info_map.get(&id));

            ClickStreamEventDto {
                id: item.id,
                owner_id: item.owner_id,
                creator_id: item.creator_id,
                route_id: item.route_id,
                route_name: route_info.map(|r| r.link.clone()),
                route_domain_name: route_info.and_then(|r| r.domain_name.clone()),
                workspace_id: item.workspace_id,
                created: item.created,
                dest: item.dest,
                ip: item.ip,
                continent: item.continent,
                country: item.country,
                location: item.location,
                os_family: item.os_family,
                os_version: item.os_version,
                user_agent_family: item.user_agent_family,
                user_agent_version: item.user_agent_version,
                device_brand: item.device_brand,
                device_family: item.device_family,
                device_model: item.device_model,
                session_first: item.session_first,
                session_clicks: item.session_clicks,
                is_unique: item.is_unique,
                is_bot: item.is_bot,
            }
        })
        .collect();

    render_success(res, events)
}

// ============================================================================
// Stats endpoints (matching click-aggregator-api)
// ============================================================================

/// Country count for stats.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CountryCount {
    pub country: String,
    pub count: i64,
}

/// Device count for stats.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCount {
    pub device: String,
    pub count: i64,
}

/// Click trend for stats.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClickTrend {
    pub date: String,
    pub clicks: i64,
}

/// Combined stats response.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[salvo(schema(name = "CombinedStats"))]
pub struct CombinedStats {
    pub total_clicks: i64,
    pub unique_clicks: i64,
    pub bot_clicks: i64,
    pub top_countries: Vec<CountryCount>,
    pub top_devices: Vec<DeviceCount>,
    pub click_trends: Vec<ClickTrend>,
}

/// Get combined statistics.
#[endpoint(
    operation_id = "get_stats",
    summary = "Get combined statistics",
    description = "Get aggregated click statistics for a date range",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("startDate" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("endDate" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Combined statistics", body = CombinedStats)
    )
)]
pub async fn get_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let start_date: Option<NaiveDate> = req
        .query::<String>("startDate")
        .and_then(|s| {
            // Try ISO 8601 datetime first, then date-only
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.date_naive())
                .ok()
                .or_else(|| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
        });
    let end_date: Option<NaiveDate> = req
        .query::<String>("endDate")
        .and_then(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.date_naive())
                .ok()
                .or_else(|| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
        });

    // Fetch all data concurrently
    let (daily_result, geo_result, device_result) = tokio::join!(
        app_state.click_aggregator.get_daily_stats(
            owner_id.as_deref(),
            route_id.as_deref(),
            start_date,
            end_date
        ),
        app_state.click_aggregator.get_geographic_stats(
            owner_id.as_deref(),
            route_id.as_deref(),
            start_date,
            end_date
        ),
        app_state.click_aggregator.get_device_stats(
            owner_id.as_deref(),
            route_id.as_deref(),
            start_date,
            end_date
        )
    );

    let daily_stats = match daily_result {
        Ok(stats) => stats,
        Err(e) => {
            render_error(res, ApiError::external_service(e.to_string()));
            return;
        }
    };

    let geo_stats = geo_result.unwrap_or_default();
    let device_stats = device_result.unwrap_or_default();

    // Aggregate daily stats
    let total_clicks: i64 = daily_stats.iter().map(|s| s.total_clicks).sum();
    let unique_clicks: i64 = daily_stats.iter().map(|s| s.unique_clicks).sum();
    let bot_clicks: i64 = daily_stats.iter().map(|s| s.bot_clicks).sum();

    // Build top countries (limit to 10)
    let mut top_countries: Vec<CountryCount> = geo_stats
        .into_iter()
        .map(|g| CountryCount {
            country: g.country.unwrap_or_else(|| "Unknown".to_string()),
            count: g.total_clicks,
        })
        .collect();
    top_countries.sort_by(|a, b| b.count.cmp(&a.count));
    top_countries.truncate(10);

    // Build top devices (limit to 10)
    let mut top_devices: Vec<DeviceCount> = device_stats
        .into_iter()
        .map(|d| DeviceCount {
            device: d.device_family.unwrap_or_else(|| "Unknown".to_string()),
            count: d.total_clicks,
        })
        .collect();
    top_devices.sort_by(|a, b| b.count.cmp(&a.count));
    top_devices.truncate(10);

    // Build click trends from daily stats
    let click_trends: Vec<ClickTrend> = daily_stats
        .into_iter()
        .map(|d| ClickTrend {
            date: d.date,
            clicks: d.total_clicks,
        })
        .collect();

    let combined = CombinedStats {
        total_clicks,
        unique_clicks,
        bot_clicks,
        top_countries,
        top_devices,
        click_trends,
    };

    render_success(res, combined)
}

/// Get daily statistics.
#[endpoint(
    operation_id = "get_daily_stats",
    summary = "Get daily statistics",
    description = "Get daily aggregated click statistics",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Daily statistics", body = Vec<DailyStats>)
    )
)]
pub async fn get_daily_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

    match app_state
        .click_aggregator
        .get_daily_stats(owner_id.as_deref(), route_id.as_deref(), from_date, to_date)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get hourly statistics.
#[endpoint(
    operation_id = "get_hourly_stats",
    summary = "Get hourly statistics",
    description = "Get hourly aggregated click statistics",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_hour" = Option<String>, Query, description = "Start hour (ISO 8601)"),
        ("to_hour" = Option<String>, Query, description = "End hour (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Hourly statistics", body = Vec<HourlyStats>)
    )
)]
pub async fn get_hourly_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_hour: Option<DateTime<Utc>> = req
        .query::<String>("from_hour")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to_hour: Option<DateTime<Utc>> = req
        .query::<String>("to_hour")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_hourly_stats(owner_id.as_deref(), route_id.as_deref(), from_hour, to_hour)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get geographic statistics.
#[endpoint(
    operation_id = "get_geographic_stats",
    summary = "Get geographic statistics",
    description = "Get click statistics by geographic location",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Geographic statistics", body = Vec<GeographicStats>)
    )
)]
pub async fn get_geographic_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

    match app_state
        .click_aggregator
        .get_geographic_stats(owner_id.as_deref(), route_id.as_deref(), from_date, to_date)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get device statistics.
#[endpoint(
    operation_id = "get_device_stats",
    summary = "Get device statistics",
    description = "Get click statistics by device type",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Device statistics", body = Vec<DeviceStats>)
    )
)]
pub async fn get_device_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

    match app_state
        .click_aggregator
        .get_device_stats(owner_id.as_deref(), route_id.as_deref(), from_date, to_date)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get browser statistics.
#[endpoint(
    operation_id = "get_browser_stats",
    summary = "Get browser statistics",
    description = "Get click statistics by browser/user agent",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Browser statistics", body = Vec<BrowserStats>)
    )
)]
pub async fn get_browser_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

    match app_state
        .click_aggregator
        .get_browser_stats(owner_id.as_deref(), route_id.as_deref(), from_date, to_date)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get route performance statistics.
#[endpoint(
    operation_id = "get_route_performance",
    summary = "Get route performance",
    description = "Get performance metrics for routes",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("limit" = Option<u32>, Query, description = "Maximum results (default: 50)")
    ),
    responses(
        (status_code = 200, description = "Route performance", body = Vec<RoutePerformance>)
    )
)]
pub async fn get_route_performance(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let limit: Option<u32> = req.query("limit");

    match app_state
        .click_aggregator
        .get_route_performance(owner_id.as_deref(), from_date, to_date, limit)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get top destinations.
#[endpoint(
    operation_id = "get_top_destinations",
    summary = "Get top destinations",
    description = "Get most clicked destination URLs",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("limit" = Option<u32>, Query, description = "Maximum results (default: 20)")
    ),
    responses(
        (status_code = 200, description = "Top destinations", body = Vec<TopDestination>)
    )
)]
pub async fn get_top_destinations(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_date: Option<NaiveDate> = req
        .query::<String>("from_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let to_date: Option<NaiveDate> = req
        .query::<String>("to_date")
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let limit: Option<u32> = req.query("limit");

    match app_state
        .click_aggregator
        .get_top_destinations(
            owner_id.as_deref(),
            route_id.as_deref(),
            from_date,
            to_date,
            limit,
        )
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get traffic type statistics.
#[endpoint(
    operation_id = "get_traffic_type_stats",
    summary = "Get traffic type statistics",
    description = "Get bot vs human traffic breakdown",
    tags("ClickStream"),
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_hour" = Option<String>, Query, description = "Start hour (ISO 8601)"),
        ("to_hour" = Option<String>, Query, description = "End hour (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Traffic type statistics", body = Vec<TrafficTypeStats>)
    )
)]
pub async fn get_traffic_type_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let owner_id: Option<String> = req.query("owner_id");
    let route_id: Option<String> = req.query("route_id");
    let from_hour: Option<DateTime<Utc>> = req
        .query::<String>("from_hour")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to_hour: Option<DateTime<Utc>> = req
        .query::<String>("to_hour")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_traffic_type_stats(owner_id.as_deref(), route_id.as_deref(), from_hour, to_hour)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

// ============================================================================
// Legacy route-specific endpoints
// ============================================================================

/// Get route statistics.
#[endpoint(
    operation_id = "get_route_stats",
    summary = "Get route stats",
    description = "Get click statistics for a route",
    tags("ClickStream"),
    parameters(
        ("route_id" = String, Path, description = "Route ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Route statistics", body = ClickStats)
    )
)]
pub async fn get_route_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let route_id: String = req.param("route_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_route_stats(&route_id, from, to)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get route time series data.
#[endpoint(
    operation_id = "get_route_time_series",
    summary = "Get route time series",
    description = "Get time series click data for a route",
    tags("ClickStream"),
    parameters(
        ("route_id" = String, Path, description = "Route ID"),
        ("from" = String, Query, description = "Start date (ISO 8601)"),
        ("to" = String, Query, description = "End date (ISO 8601)"),
        ("interval" = Option<String>, Query, description = "Interval (hour, day, week, month)")
    ),
    responses(
        (status_code = 200, description = "Time series data", body = Vec<TimeSeriesPoint>)
    )
)]
pub async fn get_route_time_series(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let route_id: String = req.param("route_id").unwrap_or_default();
    let from_str: String = req.query("from").unwrap_or_default();
    let to_str: String = req.query("to").unwrap_or_default();
    let interval: String = req.query("interval").unwrap_or_else(|| "day".to_string());

    let from = match DateTime::parse_from_rfc3339(&from_str) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            render_error(res, ApiError::validation("Invalid 'from' date format"));
            return;
        }
    };

    let to = match DateTime::parse_from_rfc3339(&to_str) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => {
            render_error(res, ApiError::validation("Invalid 'to' date format"));
            return;
        }
    };

    match app_state
        .click_aggregator
        .get_route_time_series(&route_id, from, to, &interval)
        .await
    {
        Ok(series) => render_success(res, series),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get route geographic distribution.
#[endpoint(
    operation_id = "get_route_geo",
    summary = "Get route geo distribution",
    description = "Get geographic distribution of clicks for a route",
    tags("ClickStream"),
    parameters(
        ("route_id" = String, Path, description = "Route ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("limit" = Option<i32>, Query, description = "Maximum results")
    ),
    responses(
        (status_code = 200, description = "Geographic distribution", body = Vec<GeoDistribution>)
    )
)]
pub async fn get_route_geo(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let route_id: String = req.param("route_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let limit: Option<i32> = req.query("limit");

    match app_state
        .click_aggregator
        .get_route_geo(&route_id, from, to, limit)
        .await
    {
        Ok(geo) => render_success(res, geo),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get route device distribution.
#[endpoint(
    operation_id = "get_route_devices",
    summary = "Get route device distribution",
    description = "Get device type distribution of clicks for a route",
    tags("ClickStream"),
    parameters(
        ("route_id" = String, Path, description = "Route ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Device distribution", body = Vec<DeviceDistribution>)
    )
)]
pub async fn get_route_devices(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let route_id: String = req.param("route_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_route_devices(&route_id, from, to)
        .await
    {
        Ok(devices) => render_success(res, devices),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get route browser distribution.
#[endpoint(
    operation_id = "get_route_browsers",
    summary = "Get route browser distribution",
    description = "Get browser distribution of clicks for a route",
    tags("ClickStream"),
    parameters(
        ("route_id" = String, Path, description = "Route ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Browser distribution", body = Vec<BrowserDistribution>)
    )
)]
pub async fn get_route_browsers(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let route_id: String = req.param("route_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_route_browsers(&route_id, from, to)
        .await
    {
        Ok(browsers) => render_success(res, browsers),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

// ============================================================================
// Legacy workspace endpoints
// ============================================================================

/// Get workspace statistics.
#[endpoint(
    operation_id = "get_workspace_stats",
    summary = "Get workspace stats",
    description = "Get aggregated click statistics for a workspace",
    tags("ClickStream"),
    parameters(
        ("workspace_id" = String, Path, description = "Workspace ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)")
    ),
    responses(
        (status_code = 200, description = "Workspace statistics", body = ClickStats)
    )
)]
pub async fn get_workspace_stats(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let workspace_id: String = req.param("workspace_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    match app_state
        .click_aggregator
        .get_workspace_stats(&workspace_id, from, to)
        .await
    {
        Ok(stats) => render_success(res, stats),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}

/// Get top routes for a workspace.
#[endpoint(
    operation_id = "get_workspace_top_routes",
    summary = "Get workspace top routes",
    description = "Get top performing routes in a workspace by clicks",
    tags("ClickStream"),
    parameters(
        ("workspace_id" = String, Path, description = "Workspace ID"),
        ("from" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("to" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("limit" = Option<i32>, Query, description = "Maximum results")
    ),
    responses(
        (status_code = 200, description = "Top routes", body = Vec<TopRoute>)
    )
)]
pub async fn get_workspace_top_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let workspace_id: String = req.param("workspace_id").unwrap_or_default();
    let from: Option<DateTime<Utc>> = req
        .query::<String>("from")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let to: Option<DateTime<Utc>> = req
        .query::<String>("to")
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    let limit: Option<i32> = req.query("limit");

    match app_state
        .click_aggregator
        .get_workspace_top_routes(&workspace_id, from, to, limit)
        .await
    {
        Ok(routes) => render_success(res, routes),
        Err(e) => render_error(res, ApiError::external_service(e.to_string())),
    }
}
