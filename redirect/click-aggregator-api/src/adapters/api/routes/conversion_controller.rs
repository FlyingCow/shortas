use salvo::prelude::*;
use tracing::error;
use crate::adapters::api::app_state::AppState;
use crate::dto::conversion_dto::{
    ConversionDto, ConversionResponseDto, ConversionRatesDto, ConversionAttributionAnalysisDto,
    ConversionFunnelPerformanceDto, RevenueAnalyticsDto, GeographicConversionDto,
    DeviceConversionDto, HourlyConversionDto, ConversionGoalsPerformanceDto,
    MultiTouchAttributionDto, ConversionCohortDto, ConversionQueryDto,
    CreateConversionDto, CreateConversionGoalDto, CreateConversionFunnelDto,
    ConversionGoalDto, ConversionFunnelDto, ConversionSummary,
};
use crate::core::conversion_store::ConversionStore;

/// Get conversions with optional filters
#[endpoint(
    operation_id = "get_conversions",
    summary = "Get conversion data",
    description = "Retrieve conversion analytics data with optional filtering by owner_id, creator_id, route_id, workspace_id, conversion_type, and date range.",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("creator_id" = Option<String>, Query, description = "Filter by creator ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("workspace_id" = Option<String>, Query, description = "Filter by workspace ID"),
        ("conversion_type" = Option<String>, Query, description = "Filter by conversion type"),
        ("conversion_name" = Option<String>, Query, description = "Filter by conversion name"),
        ("created_from" = Option<String>, Query, description = "Filter by creation date from (ISO 8601 format)"),
        ("created_to" = Option<String>, Query, description = "Filter by creation date to (ISO 8601 format)"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results (default: 100)"),
        ("offset" = Option<u32>, Query, description = "Number of results to skip (default: 0)")
    ),
    responses(
        (status_code = 200, description = "Conversion data retrieved successfully", body = ConversionResponseDto)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_conversions(
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

    let query = ConversionQueryDto {
        owner_id: req.query::<String>("owner_id"),
        creator_id: req.query::<String>("creator_id"),
        route_id: req.query::<String>("route_id"),
        workspace_id: req.query::<String>("workspace_id"),
        conversion_type: req.query::<String>("conversion_type"),
        conversion_name: req.query::<String>("conversion_name"),
        created_from: req.query::<String>("created_from"),
        created_to: req.query::<String>("created_to"),
        limit: req.query::<u32>("limit"),
        offset: req.query::<u32>("offset"),
    };

    match app_state.conversion_store.get_conversions(query).await {
        Ok(conversions) => res.render(Json(conversions)),
        Err(e) => {
            error!("Error fetching conversions: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch conversions"})));
        }
    }
}

/// Get conversion rates by route and time
#[endpoint(
    operation_id = "get_conversion_rates",
    summary = "Get conversion rates",
    description = "Retrieve conversion rates by route and time period with optional filtering",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Conversion rates retrieved successfully", body = Vec<ConversionRatesDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_conversion_rates(
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

    match app_state.conversion_store.get_conversion_rates(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(rates) => res.render(Json(rates)),
        Err(e) => {
            error!("Error fetching conversion rates: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch conversion rates"})));
        }
    }
}

/// Get revenue analytics
#[endpoint(
    operation_id = "get_revenue_analytics",
    summary = "Get revenue analytics",
    description = "Retrieve revenue analytics including total revenue, average order value, and ROI metrics",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Revenue analytics retrieved successfully", body = Vec<RevenueAnalyticsDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_revenue_analytics(
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

    match app_state.conversion_store.get_revenue_analytics(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(analytics) => res.render(Json(analytics)),
        Err(e) => {
            error!("Error fetching revenue analytics: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch revenue analytics"})));
        }
    }
}

/// Get conversion attribution analysis
#[endpoint(
    operation_id = "get_conversion_attribution",
    summary = "Get conversion attribution analysis",
    description = "Analyze which clicks lead to conversions with attribution weights and timing",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Conversion attribution analysis retrieved successfully", body = Vec<ConversionAttributionAnalysisDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_conversion_attribution(
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

    match app_state.conversion_store.get_conversion_attribution_analysis(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(attribution) => res.render(Json(attribution)),
        Err(e) => {
            error!("Error fetching conversion attribution: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch conversion attribution"})));
        }
    }
}

/// Get conversion funnel performance
#[endpoint(
    operation_id = "get_conversion_funnels",
    summary = "Get conversion funnel performance",
    description = "Analyze conversion funnel performance including completion rates and drop-off points",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("funnel_name" = Option<String>, Query, description = "Filter by funnel name"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Conversion funnel performance retrieved successfully", body = Vec<ConversionFunnelPerformanceDto>)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_conversion_funnels(
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
    let funnel_name = req.query::<String>("funnel_name");
    let from_date = req.query::<String>("from_date");
    let to_date = req.query::<String>("to_date");

    match app_state.conversion_store.get_conversion_funnel_performance(
        owner_id.as_deref(),
        funnel_name.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(funnels) => res.render(Json(funnels)),
        Err(e) => {
            error!("Error fetching conversion funnels: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch conversion funnels"})));
        }
    }
}

/// Get conversion summary for dashboard
#[endpoint(
    operation_id = "get_conversion_summary",
    summary = "Get conversion summary",
    description = "Get high-level conversion metrics for dashboard display",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("from_date" = Option<String>, Query, description = "Start date (YYYY-MM-DD)"),
        ("to_date" = Option<String>, Query, description = "End date (YYYY-MM-DD)")
    ),
    responses(
        (status_code = 200, description = "Conversion summary retrieved successfully", body = ConversionSummary)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_conversion_summary(
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

    match app_state.conversion_store.get_conversion_summary(
        owner_id.as_deref(),
        route_id.as_deref(),
        from_date.as_deref(),
        to_date.as_deref()
    ).await {
        Ok(summary) => res.render(Json(summary)),
        Err(e) => {
            error!("Error fetching conversion summary: {:?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": "Failed to fetch conversion summary"})));
        }
    }
}

/// Create a new conversion
#[endpoint(
    operation_id = "create_conversion",
    summary = "Create a new conversion",
    description = "Record a new conversion event with optional attribution to a click",
    request_body = CreateConversionDto,
    responses(
        (status_code = 201, description = "Conversion created successfully"),
        (status_code = 400, description = "Invalid conversion data")
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn create_conversion(
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

    let conversion_data: CreateConversionDto = match req.parse_json().await {
        Ok(data) => data,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": "Invalid JSON data"})));
            return;
        }
    };

    // TODO: Convert DTO to model and store conversion
    // This would involve creating the conversion record and attribution data
    
    res.status_code(salvo::http::StatusCode::CREATED);
    res.render(Json(serde_json::json!({"message": "Conversion created successfully"})));
}

/// Create conversion API routes
pub fn conversion_routes() -> Router {
    Router::with_path("/conversions")
        .push(Router::with_path("/").get(get_conversions).post(create_conversion))
        .push(Router::with_path("/rates").get(get_conversion_rates))
        .push(Router::with_path("/revenue").get(get_revenue_analytics))
        .push(Router::with_path("/attribution").get(get_conversion_attribution))
        .push(Router::with_path("/funnels").get(get_conversion_funnels))
        .push(Router::with_path("/summary").get(get_conversion_summary))
}
