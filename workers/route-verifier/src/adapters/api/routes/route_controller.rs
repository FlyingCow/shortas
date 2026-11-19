use chrono::{Duration, Utc};
use salvo::oapi::endpoint;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::info;

use crate::adapters::api::app_state::AppState;
use crate::adapters::rabbitmq::messages::RouteStatusChangedMessage;
use crate::dto::{
    CreateRouteRequest, ErrorResponse, PaginationInfo, RouteDto, RouteListResponse,
};
use crate::model::RouteToVerify;

pub fn api_routes() -> Router {
    Router::with_path("/routes")
        .get(list_routes)
        .post(create_route)
        .push(Router::with_path("/{id}").get(get_route).delete(delete_route))
        .push(Router::with_path("/{id}/verify").post(trigger_verification))
}

#[endpoint(
    operation_id = "list_routes",
    summary = "List all routes for verification",
    description = "Retrieves a list of all routes registered for safety verification.",
    parameters(
        ("page" = u32, Query, description = "Page number (default: 1)"),
        ("pageSize" = u32, Query, description = "Number of items per page (default: 20)"),
        ("ownerId" = String, Query, description = "Filter by owner ID")
    ),
    responses(
        (status_code = 200, description = "List of routes retrieved successfully", body = RouteListResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn list_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let page: u32 = req.query::<u32>("page").unwrap_or(1).max(1);
    let page_size: u32 = req.query::<u32>("pageSize").unwrap_or(20).min(100);
    let owner_id = req.query::<String>("ownerId");

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    match app_state
        .route_store
        .list_routes(owner_id.as_deref(), page, page_size)
        .await
    {
        Ok((routes, total_count)) => {
            let total_pages = (total_count as f64 / page_size as f64).ceil() as u64;
            let response = RouteListResponse {
                data: routes.into_iter().map(RouteDto::from).collect(),
                pagination: PaginationInfo {
                    page,
                    page_size,
                    total_count,
                    total_pages,
                },
            };
            res.render(Json(response));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "create_route",
    summary = "Register a route for verification",
    description = "Registers a new route for safety verification. Uses upsert - updates if exists.",
    responses(
        (status_code = 201, description = "Route registered successfully", body = RouteDto),
        (status_code = 400, description = "Invalid request", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let create_req: CreateRouteRequest = match req.parse_json().await {
        Ok(r) => r,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(ErrorResponse::validation("body", &e.to_string())));
            return;
        }
    };

    if create_req.id.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("id", "Route ID is required")));
        return;
    }

    if create_req.link.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("link", "Link is required")));
        return;
    }

    if create_req.destinations.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("destinations", "At least one destination is required")));
        return;
    }

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Convert request to RouteToVerify (upsert will handle create or update)
    let route: RouteToVerify = create_req.into();

    match app_state.route_store.store_route(&route).await {
        Ok(_) => {
            info!("Route registered for verification: {} with {} destinations",
                  route.link, route.destinations.len());
            res.status_code(StatusCode::CREATED);
            res.render(Json(RouteDto::from(route)));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "get_route",
    summary = "Get route by ID",
    description = "Retrieves a route by its ID.",
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Route found", body = RouteDto),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    match app_state.route_store.get_route(&id).await {
        Ok(Some(route)) => {
            res.render(Json(RouteDto::from(route)));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Route", &id)));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "delete_route",
    summary = "Remove route from verification",
    description = "Removes a route from safety verification.",
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Route removed successfully"),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Check if route exists
    match app_state.route_store.get_route(&id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Route", &id)));
            return;
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    }

    match app_state.route_store.delete_route(&id).await {
        Ok(_) => {
            info!("Route removed from verification: {}", id);
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Route removed from verification",
                "id": id
            })));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "trigger_verification",
    summary = "Trigger immediate verification",
    description = "Triggers an immediate safety verification for a route.",
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Verification completed", body = RouteDto),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn trigger_verification(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Get route
    let route = match app_state.route_store.get_route(&id).await {
        Ok(Some(r)) => r,
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Route", &id)));
            return;
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    };

    // Check destinations against Safe Browsing
    if route.destinations.is_empty() {
        // No destinations to check
        res.render(Json(RouteDto::from(route)));
        return;
    }

    let result = match app_state.safe_browsing_client.check_urls(&route.destinations).await {
        Ok(r) => r,
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    };

    let previous_status = route.status.clone();
    let now = Utc::now();

    if !result.is_safe {
        // URL is unsafe - block the route
        let threat_type = result.first_threat_type().unwrap_or("UNKNOWN");
        let threat_url = result.first_threat_url();
        let reason = format!("Safe Browsing: {}", threat_type);

        info!(
            "Route {} flagged as unsafe: {} - blocking",
            id, reason
        );

        // Update local status
        if let Err(e) = app_state
            .route_store
            .update_route_status(&id, "Blocked", Some(&reason))
            .await
        {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }

        // Update timestamps
        let next_check = now + Duration::hours(1); // Recheck blocked routes in 1 hour
        if let Err(e) = app_state
            .route_store
            .update_safety_check_timestamps(&id, now, next_check)
            .await
        {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }

        // Publish status change to RabbitMQ
        if let Some(ref publisher) = app_state.rabbitmq_publisher {
            publisher
                .publish_route_status_changed(&RouteStatusChangedMessage {
                    route_id: id.clone(),
                    link: route.link.clone(),
                    owner_id: route.owner_id.clone(),
                    workspace_id: route.workspace_id.clone(),
                    previous_status,
                    new_status: "Blocked".to_string(),
                    blocked_reason: Some(reason),
                    threat_type: Some(threat_type.to_string()),
                    threat_url: threat_url.map(|s| s.to_string()),
                    checked_at: now.timestamp_millis(),
                    next_check_at: Some(next_check.timestamp_millis()),
                })
                .await;
        }

        // Get updated route
        match app_state.route_store.get_route(&id).await {
            Ok(Some(updated)) => {
                res.render(Json(RouteDto::from(updated)));
            }
            _ => {
                res.render(Json(RouteDto::from(route)));
            }
        }
    } else {
        // URL is safe
        let next_check = now + Duration::hours(24);
        if let Err(e) = app_state
            .route_store
            .update_safety_check_timestamps(&id, now, next_check)
            .await
        {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }

        info!("Route {} verification complete: safe", id);

        // Get updated route
        match app_state.route_store.get_route(&id).await {
            Ok(Some(updated)) => {
                res.render(Json(RouteDto::from(updated)));
            }
            _ => {
                res.render(Json(RouteDto::from(route)));
            }
        }
    }
}
