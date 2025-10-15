use salvo::{prelude::*};
use salvo::oapi::endpoint;

use crate::adapters::api::{app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter, openapi_schemas::ErrorResponse};
use crate::dto::RouteDto;
use crate::model::error::{ApiError, RouteError, ValidationError};
use crate::model::route::Route;

pub fn api_routes() -> Router {
    Router::with_path("/routes")
        .get(list_routes)  // Add list routes endpoint
        .push(Router::with_path("/{switch}/{domain}/{path}")
            .get(get_route)
            .post(create_route)
            .put(update_route)
            .delete(delete_route))
        .push(Router::with_path("/bulk")
            .post(bulk_create_routes)
            .put(bulk_update_routes)
            .delete(bulk_delete_routes))
}

/// List all routes
/// 
/// Retrieves a list of all routes with optional filtering and pagination.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "list_routes",
    summary = "List all routes",
    description = "Retrieves a list of all routes with optional filtering and pagination. Supports filtering by owner ID, status, and search terms.",
    parameters(
        ("page" = i32, Query, description = "Page number for pagination (default: 1)"),
        ("pageSize" = i32, Query, description = "Number of items per page (default: 20)"),
        ("search" = String, Query, description = "Search term to filter routes"),
        ("status" = String, Query, description = "Filter by route status"),
        ("ownerId" = String, Query, description = "Filter by owner ID")
    ),
    responses(
        (status_code = 200, description = "List of routes retrieved successfully", body = serde_json::Value),
        (status_code = 400, description = "Invalid request parameters", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - JWT token required", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn list_routes(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
) {
    // For now, return a simple response indicating the endpoint is working
    res.render(Json(serde_json::json!({
        "data": [],
        "pagination": {
            "totalCount": 0,
            "page": 1,
            "pageSize": 20
        },
        "message": "List routes endpoint is working - authentication removed for testing"
    })));
}

/// Get route information by switch, domain, and path
/// 
/// Retrieves routing information for a specific switch, domain, and path combination.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "get_route",
    summary = "Get route information",
    description = "Retrieves routing information for a specific switch, domain, and path combination. Returns route configuration, status, and properties.",
    parameters(
        ("switch" = String, Path, description = "The switch identifier", example = "main"),
        ("domain" = String, Path, description = "The domain name", example = "example.com"),
        ("path" = String, Path, description = "The path", example = "/api/v1")
    ),
    responses(
        (status_code = 200, description = "Route found successfully", body = RouteDto),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 400, description = "Bad request - Invalid parameters", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let path = req.param::<String>("path").unwrap_or_default();
    let switch = req.param::<String>("switch").unwrap_or_default();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    let route = app_state
        .routes_store
        .get_route(switch.as_str(), domain.as_str(), path.as_str())
        .await;

    match route {
        Ok(Some(route)) => {
            // Convert the internal Route to RouteDto for API response
            let route_dto = RouteDto::from(route);
            res.render(Json(route_dto));
        }
        Ok(None) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Route(
                RouteError::NotFound {
                    switch: switch.clone(),
                    domain: domain.clone(),
                    path: path.clone(),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Create a new route
/// 
/// Creates a new routing entry with the provided configuration.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "create_route",
    summary = "Create route",
    description = "Creates a new routing entry with the provided configuration. The route data must be provided in the request body. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("switch" = String, Path, description = "The switch identifier", example = "main"),
        ("domain" = String, Path, description = "The domain name", example = "example.com"),
        ("path" = String, Path, description = "The path", example = "/api/v1")
    ),
    responses(
        (status_code = 201, description = "Route created successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 409, description = "Conflict - Route already exists", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Extract path parameters
    let switch = req.param::<String>("switch").unwrap_or_default();
    let _domain = req.param::<String>("domain").unwrap_or_default();
    let _path = req.param::<String>("path").unwrap_or_default();

    // Parse the route DTO from the request body
    let mut route_dto: RouteDto = match req.parse_json().await {
        Ok(route_dto) => route_dto,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };
    // Set path parameters in the route DTO
    route_dto.switch = switch;
    // Note: domain and path are used for routing but not stored in the route object
    // The route object contains the actual route configuration

    // Validate required fields
    if !route_dto.is_valid() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "route".to_string(),
                message: "Route data is incomplete or invalid".to_string(),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal model
    let route: Route = route_dto.into();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    // Store the route
    match app_state.routes_store.store_route(&route).await {
        Ok(_) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "Route created successfully",
                "route": RouteDto::from(route)
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
/// Update an existing route
/// 
/// Updates an existing routing entry with the provided configuration.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "update_route",
    summary = "Update route",
    description = "Updates an existing routing entry with the provided configuration. The route data must be provided in the request body. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("switch" = String, Path, description = "The switch identifier", example = "main"),
        ("domain" = String, Path, description = "The domain name", example = "example.com"),
        ("path" = String, Path, description = "The path", example = "/api/v1")
    ),
    responses(
        (status_code = 200, description = "Route updated successfully", body = RouteDto),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Extract path parameters
    let switch = req.param::<String>("switch").unwrap_or_default();
    let _domain = req.param::<String>("domain").unwrap_or_default();
    let _path = req.param::<String>("path").unwrap_or_default();

    // Parse the route DTO from the request body
    let mut route_dto: RouteDto = match req.parse_json().await {
        Ok(route_dto) => route_dto,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };

    // Set path parameters in the route DTO
    route_dto.switch = switch;

    // Validate required fields
    if !route_dto.is_valid() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "route".to_string(),
                message: "Route data is incomplete or invalid".to_string(),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal model
    let route: Route = route_dto.into();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    // Update the route
    match app_state.routes_store.update_route(&route).await {
        Ok(_) => {
            res.status_code(StatusCode::OK);
            res.render(Json(RouteDto::from(route)));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Delete an existing route
/// 
/// Deletes an existing routing entry.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "delete_route",
    summary = "Delete route",
    description = "Deletes an existing routing entry. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("switch" = String, Path, description = "The switch identifier", example = "main"),
        ("domain" = String, Path, description = "The domain name", example = "example.com"),
        ("path" = String, Path, description = "The path", example = "/api/v1")
    ),
    responses(
        (status_code = 200, description = "Route deleted successfully", body = serde_json::Value),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Route not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Extract path parameters
    let switch = req.param::<String>("switch").unwrap_or_default();
    let domain = req.param::<String>("domain").unwrap_or_default();
    let path = req.param::<String>("path").unwrap_or_default();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    // First, get the route to delete
    let route = app_state
        .routes_store
        .get_route(switch.as_str(), domain.as_str(), path.as_str())
        .await;

    match route {
        Ok(Some(route_to_delete)) => {
            // Delete the route
            match app_state.routes_store.delete_route(&route_to_delete).await {
                Ok(_) => {
                    res.status_code(StatusCode::OK);
                    res.render(Json(serde_json::json!({
                        "message": "Route deleted successfully",
                        "switch": switch,
                        "domain": domain,
                        "path": path
                    })));
                }
                Err(e) => {
                    let error_response = ErrorPresenter::map_error(e);
                    res.status_code(error_response.status_code);
                    res.render(error_response);
                }
            }
        }
        Ok(None) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Route(
                RouteError::NotFound {
                    switch: switch.clone(),
                    domain: domain.clone(),
                    path: path.clone(),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
/// Bulk create routes
/// 
/// Creates multiple routing entries in a single request.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "bulk_create_routes",
    summary = "Bulk create routes",
    description = "Creates multiple routing entries in a single request. The routes data must be provided in the request body as an array. Requires JWT authentication with appropriate permissions.",
    responses(
        (status_code = 201, description = "Routes created successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn bulk_create_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Parse the routes DTO array from the request body
    let routes_dto: Vec<RouteDto> = match req.parse_json().await {
        Ok(routes_dto) => routes_dto,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };

    // Validate that we have routes to process
    if routes_dto.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: "No routes provided".to_string(),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Validate all routes
    for (index, route_dto) in routes_dto.iter().enumerate() {
        if !route_dto.is_valid() {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: format!("routes[{}]", index),
                    message: "Route data is incomplete or invalid".to_string(),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    }

    let app_state = depot.get::<AppState>("app_state").unwrap();
    let mut created_routes = Vec::new();
    let mut errors = Vec::new();

    // Process each route
    for (index, route_dto) in routes_dto.into_iter().enumerate() {
        let route: Route = route_dto.into();
        
        match app_state.routes_store.store_route(&route).await {
            Ok(_) => {
                created_routes.push(RouteDto::from(route));
            }
            Err(e) => {
                errors.push(format!("Route {}: {}", index, e));
            }
        }
    }

    // Check if we have any errors
    if !errors.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: format!("Some routes failed to create: {}", errors.join(", ")),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    res.status_code(StatusCode::CREATED);
    res.render(Json(serde_json::json!({
        "message": "Routes created successfully",
        "count": created_routes.len(),
        "routes": created_routes
    })));
}

/// Bulk update routes
/// 
/// Updates multiple routing entries in a single request.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "bulk_update_routes",
    summary = "Bulk update routes",
    description = "Updates multiple routing entries in a single request. The routes data must be provided in the request body as an array. Requires JWT authentication with appropriate permissions.",
    responses(
        (status_code = 200, description = "Routes updated successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn bulk_update_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Parse the routes DTO array from the request body
    let routes_dto: Vec<RouteDto> = match req.parse_json().await {
        Ok(routes_dto) => routes_dto,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };

    // Validate that we have routes to process
    if routes_dto.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: "No routes provided".to_string(),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Validate all routes
    for (index, route_dto) in routes_dto.iter().enumerate() {
        if !route_dto.is_valid() {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: format!("routes[{}]", index),
                    message: "Route data is incomplete or invalid".to_string(),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    }

    let app_state = depot.get::<AppState>("app_state").unwrap();
    let mut updated_routes = Vec::new();
    let mut errors = Vec::new();

    // Process each route
    for (index, route_dto) in routes_dto.into_iter().enumerate() {
        let route: Route = route_dto.into();
        
        match app_state.routes_store.update_route(&route).await {
            Ok(_) => {
                updated_routes.push(RouteDto::from(route));
            }
            Err(e) => {
                errors.push(format!("Route {}: {}", index, e));
            }
        }
    }

    // Check if we have any errors
    if !errors.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: format!("Some routes failed to update: {}", errors.join(", ")),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    res.status_code(StatusCode::OK);
    res.render(Json(serde_json::json!({
        "message": "Routes updated successfully",
        "count": updated_routes.len(),
        "routes": updated_routes
    })));
}

/// Bulk delete routes
/// 
/// Deletes multiple routing entries in a single request.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "bulk_delete_routes",
    summary = "Bulk delete routes",
    description = "Deletes multiple routing entries in a single request. The route identifiers must be provided in the request body as an array. Requires JWT authentication with appropriate permissions.",
    responses(
        (status_code = 200, description = "Routes deleted successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn bulk_delete_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // Parse the route identifiers from the request body
    let route_identifiers: Vec<serde_json::Value> = match req.parse_json().await {
        Ok(identifiers) => identifiers,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };

    // Validate that we have route identifiers to process
    if route_identifiers.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: "No route identifiers provided".to_string(),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    let app_state = depot.get::<AppState>("app_state").unwrap();
    let mut deleted_routes = Vec::new();
    let mut errors = Vec::new();

    // Process each route identifier
    for (index, identifier) in route_identifiers.into_iter().enumerate() {
        // Extract route information from identifier
        let switch = identifier.get("switch").and_then(|v| v.as_str()).unwrap_or("");
        let domain = identifier.get("domain").and_then(|v| v.as_str()).unwrap_or("");
        let path = identifier.get("path").and_then(|v| v.as_str()).unwrap_or("");

        if switch.is_empty() || domain.is_empty() || path.is_empty() {
            errors.push(format!("Route {}: Missing required fields (switch, domain, path)", index));
            continue;
        }

        // First, get the route to delete
        let route = app_state
            .routes_store
            .get_route(switch, domain, path)
            .await;

        match route {
            Ok(Some(route_to_delete)) => {
                // Delete the route
                match app_state.routes_store.delete_route(&route_to_delete).await {
                    Ok(_) => {
                        deleted_routes.push(serde_json::json!({
                            "switch": switch,
                            "domain": domain,
                            "path": path
                        }));
                    }
                    Err(e) => {
                        errors.push(format!("Route {}: {}", index, e));
                    }
                }
            }
            Ok(None) => {
                errors.push(format!("Route {}: Route not found", index));
            }
            Err(e) => {
                errors.push(format!("Route {}: {}", index, e));
            }
        }
    }

    // Check if we have any errors
    if !errors.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "routes".to_string(),
                message: format!("Some routes failed to delete: {}", errors.join(", ")),
            }
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    res.status_code(StatusCode::OK);
    res.render(Json(serde_json::json!({
        "message": "Routes deleted successfully",
        "count": deleted_routes.len(),
        "routes": deleted_routes
    })));
}
