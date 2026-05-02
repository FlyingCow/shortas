//! ClickStream controller for analytics data.

use chrono::{DateTime, Utc};
use salvo::prelude::*;

use crate::domain::entities::ApiError;
use crate::infrastructure::http_clients::{
    BrowserDistribution, ClickStats, DeviceDistribution, GeoDistribution, TimeSeriesPoint, TopRoute,
};
use crate::presentation::middleware::{render_error, render_success, DepotExt};

/// Build clickstream controller router.
pub fn clickstream_controller() -> Router {
    Router::with_path("clickstream")
        .push(
            Router::with_path("routes/<route_id>")
                .get(get_route_stats)
                .push(Router::with_path("timeseries").get(get_route_time_series))
                .push(Router::with_path("geo").get(get_route_geo))
                .push(Router::with_path("devices").get(get_route_devices))
                .push(Router::with_path("browsers").get(get_route_browsers)),
        )
        .push(
            Router::with_path("workspaces/<workspace_id>")
                .get(get_workspace_stats)
                .push(Router::with_path("top-routes").get(get_workspace_top_routes)),
        )
}

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
