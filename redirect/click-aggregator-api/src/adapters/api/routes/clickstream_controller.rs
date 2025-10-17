use salvo::prelude::*;
use crate::adapters::api::app_state::AppState;
use crate::model::clickstream::ClickStreamQuery;
use crate::dto::clickstream_dto::{ClickStreamItemDto, ClickStreamResponseDto};
use crate::core::clickstream_store::ClickStreamStore;

/// Get click stream data with optional filters
#[endpoint(
    operation_id = "get_clickstream",
    summary = "Get click stream data",
    description = "Retrieve click stream analytics data with optional filtering by owner_id, creator_id, route_id, workspace_id, and date range. Results are ordered by ID in descending order.",
    parameters(
        ("owner_id" = Option<String>, Query, description = "Filter by owner ID"),
        ("creator_id" = Option<String>, Query, description = "Filter by creator ID"),
        ("route_id" = Option<String>, Query, description = "Filter by route ID"),
        ("workspace_id" = Option<String>, Query, description = "Filter by workspace ID"),
        ("created_from" = Option<String>, Query, description = "Filter by creation date from (ISO 8601 format)"),
        ("created_to" = Option<String>, Query, description = "Filter by creation date to (ISO 8601 format)"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results (default: 100)"),
        ("offset" = Option<u32>, Query, description = "Number of results to skip (default: 0)")
    ),
    responses(
        (status_code = 200, description = "Click stream data retrieved successfully", body = ClickStreamResponseDto)
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn get_clickstream(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) {
    let app_state = match depot.obtain::<AppState>() {
        Ok(state) => state,
        Err(_) => {
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to get application state"
            })));
            return;
        }
    };

    // Parse query parameters
    let query = match parse_query_params(req) {
        Ok(q) => q,
        Err(e) => {
            res.status_code(salvo::http::StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({
                "error": "Invalid query parameters",
                "details": e.to_string()
            })));
            return;
        }
    };

    // Query click stream data
    let response = match app_state.clickstream_store.query_clickstream(&query).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ClickStream query error: {:?}", e);
            eprintln!("Error chain: {:#?}", e);
            res.status_code(salvo::http::StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({
                "error": "Failed to query click stream",
                "details": format!("{:?}", e)
            })));
            return;
        }
    };

    // Convert to DTO
    let dto_response = ClickStreamResponseDto {
        items: response.items.into_iter().map(ClickStreamItemDto::from).collect(),
        total: response.total,
        offset: response.offset,
        limit: response.limit,
        has_more: response.has_more,
    };

    res.render(Json(dto_response));
}

/// Parse query parameters into ClickStreamQuery
fn parse_query_params(req: &Request) -> anyhow::Result<ClickStreamQuery> {
    let mut query = ClickStreamQuery::new();

    // Parse owner_id
    if let Some(owner_id) = req.query::<String>("owner_id") {
        if !owner_id.is_empty() {
            query = query.with_owner_id(owner_id);
        }
    }

    // Parse creator_id
    if let Some(creator_id) = req.query::<String>("creator_id") {
        if !creator_id.is_empty() {
            query = query.with_creator_id(creator_id);
        }
    }

    // Parse route_id
    if let Some(route_id) = req.query::<String>("route_id") {
        if !route_id.is_empty() {
            query = query.with_route_id(route_id);
        }
    }

    // Parse workspace_id
    if let Some(workspace_id) = req.query::<String>("workspace_id") {
        if !workspace_id.is_empty() {
            query = query.with_workspace_id(workspace_id);
        }
    }

    // Parse date range
    if let (Some(created_from_str), Some(created_to_str)) = (
        req.query::<String>("created_from"),
        req.query::<String>("created_to")
    ) {
        if !created_from_str.is_empty() && !created_to_str.is_empty() {
            let created_from = created_from_str.parse::<chrono::DateTime<chrono::Utc>>()
                .map_err(|_| anyhow::anyhow!("Invalid created_from date format. Use ISO 8601 format (e.g., 2023-01-01T00:00:00Z)"))?;
            
            let created_to = created_to_str.parse::<chrono::DateTime<chrono::Utc>>()
                .map_err(|_| anyhow::anyhow!("Invalid created_to date format. Use ISO 8601 format (e.g., 2023-12-31T23:59:59Z)"))?;

            query = query.with_date_range(created_from, created_to);
        }
    }

    // Parse pagination
    if let (Some(limit), Some(offset)) = (
        req.query::<u32>("limit"),
        req.query::<u32>("offset")
    ) {
        query = query.with_pagination(limit, offset);
    }

    Ok(query)
}

/// Create the clickstream API routes
pub fn api_routes() -> Router {
    Router::with_path("/clickstream")
        .get(get_clickstream)
}
