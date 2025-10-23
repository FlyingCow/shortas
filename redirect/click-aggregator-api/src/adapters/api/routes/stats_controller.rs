use salvo::prelude::*;
use tracing::error;
use crate::adapters::api::app_state::AppState;
use crate::dto::clickstream_dto::{
    DailyStatsDto, HourlyStatsDto, GeographicStatsDto, DeviceStatsDto,
    BrowserStatsDto, RoutePerformanceDto, TopDestinationDto, TrafficTypeStatsDto
};
use crate::core::clickstream_store::ClickStreamStore;

/// Get daily statistics
#[endpoint(
    operation_id = "get_daily_stats",
    summary = "Get daily aggregated statistics",
    description = "Retrieve daily click statistics with optional filtering",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Daily statistics retrieved successfully", body = Vec<DailyStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_daily_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");

    match app_state.clickstream_store.get_daily_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching daily stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch daily statistics"})));
        }
    }
}

/// Get hourly statistics
#[endpoint(
    operation_id = "get_hourly_stats",
    summary = "Get hourly aggregated statistics",
    description = "Retrieve hourly click statistics with optional filtering",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_hour" = Option<String>, Query, description = "Start hour (YYYY-MM-DD HH:MM:SS)"),
        ("to_hour" = Option<String>, Query, description = "End hour (YYYY-MM-DD HH:MM:SS)")
    ),
    responses(
        (status_code = 200, description = "Hourly statistics retrieved successfully", body = Vec<HourlyStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_hourly_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_hour = req.query::<String>("from_hour");
    let to_hour = req.query::<String>("to_hour");

    match app_state.clickstream_store.get_hourly_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_hour.as_deref(),
        to_hour.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching hourly stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch hourly statistics"})));
        }
    }
}

/// Get geographic statistics
#[endpoint(
    operation_id = "get_geographic_stats",
    summary = "Get geographic statistics",
    description = "Retrieve click statistics by geographic location",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Geographic statistics retrieved successfully", body = Vec<GeographicStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_geographic_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");

    match app_state.clickstream_store.get_geographic_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching geographic stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch geographic statistics"})));
        }
    }
}

/// Get device statistics
#[endpoint(
    operation_id = "get_device_stats",
    summary = "Get device statistics",
    description = "Retrieve click statistics by device type",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Device statistics retrieved successfully", body = Vec<DeviceStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_device_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");

    match app_state.clickstream_store.get_device_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching device stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch device statistics"})));
        }
    }
}

/// Get browser statistics
#[endpoint(
    operation_id = "get_browser_stats",
    summary = "Get browser statistics",
    description = "Retrieve click statistics by browser/user agent",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Browser statistics retrieved successfully", body = Vec<BrowserStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_browser_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");

    match app_state.clickstream_store.get_browser_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching browser stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch browser statistics"})));
        }
    }
}

/// Get route performance statistics
#[endpoint(
    operation_id = "get_route_performance",
    summary = "Get route performance statistics",
    description = "Retrieve performance metrics for routes",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results (default: 50)")
    ),
    responses(
        (status_code = 200, description = "Route performance retrieved successfully", body = Vec<RoutePerformanceDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_route_performance(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");
    let limit = req.query::<u32>("limit");

    match app_state.clickstream_store.get_route_performance(
        owner_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref(),
        limit
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching route performance: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch route performance"})));
        }
    }
}

/// Get top destinations
#[endpoint(
    operation_id = "get_top_destinations",
    summary = "Get top destinations",
    description = "Retrieve most clicked destination URLs",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results (default: 20)")
    ),
    responses(
        (status_code = 200, description = "Top destinations retrieved successfully", body = Vec<TopDestinationDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_top_destinations(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");
    let limit = req.query::<u32>("limit");

    match app_state.clickstream_store.get_top_destinations(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref(),
        limit
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching top destinations: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch top destinations"})));
        }
    }
}

/// Get traffic type statistics (bot vs human)
#[endpoint(
    operation_id = "get_traffic_type_stats",
    summary = "Get traffic type statistics",
    description = "Retrieve bot vs human traffic breakdown",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_hour" = Option<String>, Query, description = "Start hour (YYYY-MM-DD HH:MM:SS)"),
        ("to_hour" = Option<String>, Query, description = "End hour (YYYY-MM-DD HH:MM:SS)")
    ),
    responses(
        (status_code = 200, description = "Traffic type statistics retrieved successfully", body = Vec<TrafficTypeStatsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_traffic_type_stats(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to get application state"})));
            return;
        }
    };

    let owner_id = req.query::<String>("owner_id");
    let route_id = req.query::<String>("route_id");
    let from_hour = req.query::<String>("from_hour");
    let to_hour = req.query::<String>("to_hour");

    match app_state.clickstream_store.get_traffic_type_stats(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_hour.as_deref(),
        to_hour.as_deref()
    ).await {
        Ok(stats) => res.render(Json(stats)),
        Err(e) => {
            error!("Error fetching traffic type stats: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch traffic type statistics"})));
        }
    }
}

/// Create the statistics API routes
pub fn stats_routes() -> Router {
    Router::with_path("/stats")
        .push(Router::with_path("/daily").get(get_daily_stats))
        .push(Router::with_path("/hourly").get(get_hourly_stats))
        .push(Router::with_path("/geographic").get(get_geographic_stats))
        .push(Router::with_path("/devices").get(get_device_stats))
        .push(Router::with_path("/browsers").get(get_browser_stats))
        .push(Router::with_path("/route-performance").get(get_route_performance))
        .push(Router::with_path("/top-destinations").get(get_top_destinations))
        .push(Router::with_path("/traffic-types").get(get_traffic_type_stats))
}
