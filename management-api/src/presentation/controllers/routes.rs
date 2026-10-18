//! Routes controller for managing URL shortcuts.

use std::collections::HashMap;

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::{
    BulkDeleteDto, BulkRoutesDto, CreateRouteDto, LinkSuggestionDto, PaginatedRoutesDto,
    PresignedUrlDto, QrSettingsDto, RouteDto, UpdateRouteDto,
};
use crate::domain::entities::ApiError;
use crate::domain::traits::RouteFilters;
use crate::presentation::middleware::{
    render_created, render_error, render_no_content, render_success, DepotExt, UserExt,
};

use super::cors_preflight;

/// Build routes controller router.
pub fn routes_controller() -> Router {
    Router::with_path("routes")
        .get(list_routes)
        .post(create_route)
        .options(cors_preflight)
        .push(Router::with_path("bulk").post(bulk_create_routes).put(bulk_update_routes).delete(bulk_delete_routes).options(cors_preflight))
        .push(Router::with_path("suggest-link").get(suggest_link).options(cors_preflight))
        .push(Router::with_path("search").get(search_routes).options(cors_preflight))
        .push(Router::with_path("search/reindex").post(reindex_routes).options(cors_preflight))
        .push(
            Router::with_path("{id}")
                .get(get_route)
                .put(update_route)
                .delete(delete_route)
                .options(cors_preflight)
                .push(Router::with_path("unblock").post(unblock_route).options(cors_preflight))
                .push(
                    Router::with_path("qr")
                        .options(cors_preflight)
                        .push(Router::with_path("settings").get(get_qr_settings).put(update_qr_settings).options(cors_preflight))
                        .push(Router::with_path("upload-url").post(get_qr_upload_url).options(cors_preflight))
                        .push(Router::with_path("logo-upload-url").post(get_qr_logo_upload_url).options(cors_preflight)),
                ),
        )
}

/// List routes with pagination and filters.
#[endpoint(
    operation_id = "list_routes",
    summary = "List routes",
    description = "List routes with pagination and optional filters",
    tags("Routes"),
    parameters(
        ("page" = Option<i32>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<i32>, Query, description = "Page size (default: 20, max: 100)"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("workspace_id" = Option<String>, Query, description = "Filter by workspace ID")
    ),
    responses(
        (status_code = 200, description = "Routes list", body = PaginatedRoutesDto)
    )
)]
pub async fn list_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let page: i32 = req.query("page").unwrap_or(1);
    let page_size: i32 = req.query("page_size").unwrap_or(20).min(100);
    let search: Option<String> = req.query("search");
    let status: Option<String> = req.query("status");
    let workspace_id: Option<String> = req.query("workspace_id");

    let filters = RouteFilters {
        search,
        status,
        workspace_id,
        owner_id: Some(user_id.clone()),
        ..Default::default()
    };

    match app_state.route_service.list(&user_id, page, page_size, filters).await {
        Ok(result) => {
            // Collect unique domain IDs
            let domain_ids: Vec<Uuid> = result
                .items
                .iter()
                .filter_map(|r| r.domain_id)
                .collect();

            // Batch fetch domains and create lookup map
            let domains_map: HashMap<Uuid, _> = app_state
                .domain_repo
                .get_by_ids(&domain_ids)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|d| (d.id, d))
                .collect();

            let routes: Vec<RouteDto> = result
                .items
                .into_iter()
                .map(|r| {
                    let domain = r.domain_id.and_then(|id| domains_map.get(&id).cloned());
                    RouteDto::from_entity(r, domain)
                })
                .collect();

            render_success(
                res,
                PaginatedRoutesDto::new(routes, result.page, result.page_size, result.total_count),
            );
        }
        Err(e) => render_error(res, e),
    }
}

/// Get a route by ID.
#[endpoint(
    operation_id = "get_route",
    summary = "Get route",
    description = "Get a route by its ID",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Route details", body = RouteDto),
        (status_code = 404, description = "Route not found")
    )
)]
pub async fn get_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(route) => {
            // Get domain if available
            let domain = if let Some(domain_id) = route.domain_id {
                app_state.domain_repo.get_by_id(domain_id).await.ok().flatten()
            } else {
                None
            };
            render_success(res, RouteDto::from_entity(route, domain));
        }
        Err(e) => render_error(res, e),
    }
}

/// Create a new route.
#[endpoint(
    operation_id = "create_route",
    summary = "Create route",
    description = "Create a new route",
    tags("Routes"),
    request_body(content = CreateRouteDto, description = "Route to create"),
    responses(
        (status_code = 201, description = "Route created", body = RouteDto),
        (status_code = 400, description = "Invalid input")
    )
)]
pub async fn create_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let dto: CreateRouteDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let route = match dto.to_entity(&user_id) {
        Ok(r) => r,
        Err(e) => {
            render_error(res, ApiError::validation(e));
            return;
        }
    };

    match app_state.route_service.create(route, &user_id).await {
        Ok(created) => {
            let domain = if let Some(domain_id) = created.domain_id {
                app_state.domain_repo.get_by_id(domain_id).await.ok().flatten()
            } else {
                None
            };
            render_created(res, RouteDto::from_entity(created, domain));
        }
        Err(e) => render_error(res, e),
    }
}

/// Update a route.
#[endpoint(
    operation_id = "update_route",
    summary = "Update route",
    description = "Update an existing route",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    request_body(content = UpdateRouteDto, description = "Route updates"),
    responses(
        (status_code = 200, description = "Route updated", body = RouteDto),
        (status_code = 404, description = "Route not found")
    )
)]
pub async fn update_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    let dto: UpdateRouteDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    // Get existing route
    let existing = match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(r) => r,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let updated = dto.apply_to(existing);

    match app_state.route_service.update(id, updated, &user_id).await {
        Ok(saved) => {
            let domain = if let Some(domain_id) = saved.domain_id {
                app_state.domain_repo.get_by_id(domain_id).await.ok().flatten()
            } else {
                None
            };
            render_success(res, RouteDto::from_entity(saved, domain));
        }
        Err(e) => render_error(res, e),
    }
}

/// Delete a route.
#[endpoint(
    operation_id = "delete_route",
    summary = "Delete route",
    description = "Delete a route",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 204, description = "Route deleted"),
        (status_code = 404, description = "Route not found")
    )
)]
pub async fn delete_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    match app_state.route_service.delete(id, &user_id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}

/// Unblock a route.
#[endpoint(
    operation_id = "unblock_route",
    summary = "Unblock route",
    description = "Reset a blocked route's status to active",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Route unblocked", body = RouteDto),
        (status_code = 404, description = "Route not found")
    )
)]
pub async fn unblock_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    match app_state.route_service.unblock(id, &user_id).await {
        Ok(route) => render_success(res, RouteDto::from_entity(route, None)),
        Err(e) => render_error(res, e),
    }
}

/// Bulk create routes.
#[endpoint(
    operation_id = "bulk_create_routes",
    summary = "Bulk create routes",
    description = "Create multiple routes at once",
    tags("Routes"),
    request_body(content = BulkRoutesDto, description = "Routes to create"),
    responses(
        (status_code = 201, description = "Routes created", body = Vec<RouteDto>)
    )
)]
pub async fn bulk_create_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let dto: BulkRoutesDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let routes: Result<Vec<_>, _> = dto
        .routes
        .into_iter()
        .map(|r| r.to_entity(&user_id))
        .collect();

    let routes = match routes {
        Ok(r) => r,
        Err(e) => {
            render_error(res, ApiError::validation(e));
            return;
        }
    };

    match app_state.route_service.bulk_create(routes, &user_id).await {
        Ok(created) => {
            // Collect unique domain IDs and batch fetch
            let domain_ids: Vec<Uuid> = created
                .iter()
                .filter_map(|r| r.domain_id)
                .collect();

            let domains_map: HashMap<Uuid, _> = app_state
                .domain_repo
                .get_by_ids(&domain_ids)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|d| (d.id, d))
                .collect();

            let dtos: Vec<RouteDto> = created
                .into_iter()
                .map(|r| {
                    let domain = r.domain_id.and_then(|id| domains_map.get(&id).cloned());
                    RouteDto::from_entity(r, domain)
                })
                .collect();
            render_created(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Bulk update routes.
#[endpoint(
    operation_id = "bulk_update_routes",
    summary = "Bulk update routes",
    description = "Update multiple routes at once",
    tags("Routes"),
    request_body(content = BulkRoutesDto, description = "Routes to update"),
    responses(
        (status_code = 200, description = "Routes updated", body = Vec<RouteDto>)
    )
)]
pub async fn bulk_update_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let dto: BulkRoutesDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let routes: Result<Vec<_>, _> = dto
        .routes
        .into_iter()
        .map(|r| r.to_entity(&user_id))
        .collect();

    let routes = match routes {
        Ok(r) => r,
        Err(e) => {
            render_error(res, ApiError::validation(e));
            return;
        }
    };

    match app_state.route_service.bulk_update(routes, &user_id).await {
        Ok(updated) => {
            // Collect unique domain IDs and batch fetch
            let domain_ids: Vec<Uuid> = updated
                .iter()
                .filter_map(|r| r.domain_id)
                .collect();

            let domains_map: HashMap<Uuid, _> = app_state
                .domain_repo
                .get_by_ids(&domain_ids)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|d| (d.id, d))
                .collect();

            let dtos: Vec<RouteDto> = updated
                .into_iter()
                .map(|r| {
                    let domain = r.domain_id.and_then(|id| domains_map.get(&id).cloned());
                    RouteDto::from_entity(r, domain)
                })
                .collect();
            render_success(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Bulk delete routes.
#[endpoint(
    operation_id = "bulk_delete_routes",
    summary = "Bulk delete routes",
    description = "Delete multiple routes at once",
    tags("Routes"),
    request_body(content = BulkDeleteDto, description = "Route IDs to delete"),
    responses(
        (status_code = 204, description = "Routes deleted")
    )
)]
pub async fn bulk_delete_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let dto: BulkDeleteDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let ids: Result<Vec<Uuid>, _> = dto.ids.iter().map(|s| Uuid::parse_str(s)).collect();

    let ids = match ids {
        Ok(ids) => ids,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route IDs"));
            return;
        }
    };

    match app_state.route_service.bulk_delete(ids, &user_id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}

/// Suggest a unique link for a domain.
#[endpoint(
    operation_id = "suggest_link",
    summary = "Suggest link",
    description = "Generate a unique link suggestion for a domain",
    tags("Routes"),
    parameters(
        ("domainId" = String, Query, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "Link suggestion", body = LinkSuggestionDto)
    )
)]
pub async fn suggest_link(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    // Accept both camelCase (domainId) and snake_case (domain_id) for compatibility
    let domain_id_str: String = req.query("domainId")
        .or_else(|| req.query("domain_id"))
        .unwrap_or_default();
    let domain_id = match Uuid::parse_str(&domain_id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid domainId"));
            return;
        }
    };

    match app_state.route_service.suggest_link(domain_id).await {
        Ok(link) => render_success(res, LinkSuggestionDto { link }),
        Err(e) => render_error(res, e),
    }
}

/// Search routes.
#[endpoint(
    operation_id = "search_routes",
    summary = "Search routes",
    description = "Full-text search for routes",
    tags("Routes"),
    parameters(
        ("q" = String, Query, description = "Search query"),
        ("page" = Option<i32>, Query, description = "Page number"),
        ("page_size" = Option<i32>, Query, description = "Page size"),
        ("workspace_id" = Option<String>, Query, description = "Filter by workspace")
    ),
    responses(
        (status_code = 200, description = "Search results", body = PaginatedRoutesDto)
    )
)]
pub async fn search_routes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let query: String = req.query("q").unwrap_or_default();
    let page: i32 = req.query("page").unwrap_or(1);
    let page_size: i32 = req.query("page_size").unwrap_or(20).min(100);
    let workspace_id: Option<String> = req.query("workspace_id");

    if query.is_empty() {
        render_error(res, ApiError::validation("Search query is required"));
        return;
    }

    match app_state
        .search_service
        .search(&query, Some(&user_id), workspace_id.as_deref(), page, page_size)
        .await
    {
        Ok(result) => {
            // Fetch full route data for each ID
            let mut route_entities = Vec::new();
            for id in result.route_ids {
                if let Ok(Some(route)) = app_state.route_repo.get_by_id(id).await {
                    route_entities.push(route);
                }
            }

            // Collect unique domain IDs and batch fetch
            let domain_ids: Vec<Uuid> = route_entities
                .iter()
                .filter_map(|r| r.domain_id)
                .collect();

            let domains_map: HashMap<Uuid, _> = app_state
                .domain_repo
                .get_by_ids(&domain_ids)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(|d| (d.id, d))
                .collect();

            let routes: Vec<RouteDto> = route_entities
                .into_iter()
                .map(|r| {
                    let domain = r.domain_id.and_then(|id| domains_map.get(&id).cloned());
                    RouteDto::from_entity(r, domain)
                })
                .collect();

            render_success(
                res,
                PaginatedRoutesDto::new(routes, page, page_size, result.total_count),
            );
        }
        Err(e) => render_error(res, ApiError::internal(e.to_string())),
    }
}

/// Trigger route reindexing.
#[endpoint(
    operation_id = "reindex_routes",
    summary = "Reindex routes",
    description = "Trigger reindexing of all routes in Elasticsearch",
    tags("Routes"),
    responses(
        (status_code = 202, description = "Reindexing started")
    )
)]
pub async fn reindex_routes(depot: &mut Depot, res: &mut Response) {
    let _app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    // In a real implementation, this would trigger a background job
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(serde_json::json!({
        "message": "Reindexing started"
    })));
}

/// Get QR settings for a route.
#[endpoint(
    operation_id = "get_qr_settings",
    summary = "Get QR settings",
    description = "Get QR code generation settings for a route",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "QR settings", body = QrSettingsDto)
    )
)]
pub async fn get_qr_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(route) => {
            let settings = route
                .properties
                .qr_settings
                .map(|s| QrSettingsDto::from_entity(&s))
                .unwrap_or_default();
            render_success(res, settings);
        }
        Err(e) => render_error(res, e),
    }
}

/// Update QR settings for a route.
#[endpoint(
    operation_id = "update_qr_settings",
    summary = "Update QR settings",
    description = "Update QR code generation settings for a route",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    request_body(content = QrSettingsDto, description = "QR settings"),
    responses(
        (status_code = 200, description = "QR settings updated", body = QrSettingsDto)
    )
)]
pub async fn update_qr_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    let settings: QrSettingsDto = match req.parse_json().await {
        Ok(s) => s,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let mut route = match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(r) => r,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    route.properties.qr_settings = Some(settings.clone().to_entity());

    match app_state.route_service.update(id, route, &user_id).await {
        Ok(_) => render_success(res, settings),
        Err(e) => render_error(res, e),
    }
}

/// Get presigned URL for QR code upload.
#[endpoint(
    operation_id = "get_qr_upload_url",
    summary = "Get QR upload URL",
    description = "Get a presigned URL for uploading a QR code image",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Presigned URL", body = PresignedUrlDto)
    )
)]
pub async fn get_qr_upload_url(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    // Verify ownership and get route
    let route = match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(r) => r,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let owner_id = route.properties.owner_id.as_deref().unwrap_or(&user_id);

    match app_state.storage_service.get_qr_upload_url(owner_id, &id.to_string()).await {
        Ok(presigned) => render_success(
            res,
            PresignedUrlDto {
                url: presigned.url,
                expires_at: presigned.expires_at.to_rfc3339(),
            },
        ),
        Err(e) => render_error(res, ApiError::internal(e.to_string())),
    }
}

/// Get presigned URL for QR logo upload.
#[endpoint(
    operation_id = "get_qr_logo_upload_url",
    summary = "Get QR logo upload URL",
    description = "Get a presigned URL for uploading a QR code logo image",
    tags("Routes"),
    parameters(
        ("id" = String, Path, description = "Route ID")
    ),
    responses(
        (status_code = 200, description = "Presigned URL", body = PresignedUrlDto)
    )
)]
pub async fn get_qr_logo_upload_url(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid route ID"));
            return;
        }
    };

    // Verify ownership and get route
    let route = match app_state.route_service.get_by_id(id, &user_id).await {
        Ok(r) => r,
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    let owner_id = route.properties.owner_id.as_deref().unwrap_or(&user_id);
    let extension = req.query::<String>("extension").unwrap_or_else(|| "png".to_string());

    match app_state
        .storage_service
        .get_qr_logo_upload_url(owner_id, &id.to_string(), &extension)
        .await
    {
        Ok(presigned) => render_success(
            res,
            PresignedUrlDto {
                url: presigned.url,
                expires_at: presigned.expires_at.to_rfc3339(),
            },
        ),
        Err(e) => render_error(res, ApiError::internal(e.to_string())),
    }
}
