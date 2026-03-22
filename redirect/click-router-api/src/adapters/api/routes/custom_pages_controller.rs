use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use salvo::oapi::endpoint;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::adapters::api::{
    app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter,
    openapi_schemas::ErrorResponse,
};
use crate::adapters::rabbitmq::messages::{ChangeAction, RouteChangedMessage};
use crate::model::error::{ApiError, ValidationError};
use crate::model::route::{DestinationFormat, Route, RoutingPolicy, RouteProperties, RouteStatus, RoutingTerminal};

/// Custom page route switches
const INDEX_SWITCH: &str = "index";
const NOT_FOUND_SWITCH: &str = "404";

/// Create link format for custom pages: {domain}%2F
fn create_custom_page_link(domain_name: &str) -> String {
    format!("{}%2F", domain_name)
}

/// Generate a random route ID using base64-encoded random bytes
fn generate_route_id() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 16] = rng.random();
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Custom pages configuration DTO
#[derive(Debug, Clone, Serialize, Deserialize, salvo::oapi::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomPagesDto {
    /// Domain name these settings apply to
    pub domain_name: String,
    /// Custom index page URL (redirect destination when visiting domain root)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    /// Custom 404 page URL (redirect destination for non-existent paths)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
}

/// Request body for updating custom pages
#[derive(Debug, Clone, Deserialize, salvo::oapi::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomPagesRequest {
    /// Custom index page URL (set to null or empty to remove)
    #[serde(default)]
    pub custom_index_url: Option<String>,
    /// Custom 404 page URL (set to null or empty to remove)
    #[serde(default)]
    pub custom_not_found_url: Option<String>,
}

pub fn api_routes() -> Router {
    Router::with_path("/domains/<domain_name>/custom-pages")
        .get(get_custom_pages)
        .put(update_custom_pages)
        .delete(delete_custom_pages)
}

/// Get custom pages for a domain
///
/// Retrieves the custom index and 404 page URLs configured for a domain.
#[endpoint(
    operation_id = "get_custom_pages",
    summary = "Get custom pages for a domain",
    description = "Retrieves the custom index and 404 page URLs configured for a domain. Returns null for URLs that are not set.",
    parameters(
        ("domain_name" = String, Path, description = "The domain name", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Custom pages retrieved successfully", body = CustomPagesDto),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_custom_pages(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain_name = req.param::<String>("domain_name").unwrap_or_default();
    let link = create_custom_page_link(&domain_name);

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    // Look up index route
    let index_route = app_state
        .routes_store
        .get_route(INDEX_SWITCH, &link)
        .await
        .ok()
        .flatten();

    // Look up 404 route
    let not_found_route = app_state
        .routes_store
        .get_route(NOT_FOUND_SWITCH, &link)
        .await
        .ok()
        .flatten();

    let dto = CustomPagesDto {
        domain_name,
        custom_index_url: index_route.and_then(|r| r.dest),
        custom_not_found_url: not_found_route.and_then(|r| r.dest),
    };

    res.render(Json(dto));
}

/// Update custom pages for a domain
///
/// Sets or updates the custom index and 404 page URLs for a domain.
/// Setting a URL to null or empty string removes that custom page.
#[endpoint(
    operation_id = "update_custom_pages",
    summary = "Update custom pages for a domain",
    description = "Sets or updates the custom index and 404 page URLs for a domain. Setting a URL to null or empty string removes that custom page. URLs must be valid HTTP/HTTPS URLs.",
    parameters(
        ("domain_name" = String, Path, description = "The domain name", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Custom pages updated successfully", body = CustomPagesDto),
        (status_code = 400, description = "Invalid URL format", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_custom_pages(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain_name = req.param::<String>("domain_name").unwrap_or_default();
    let link = create_custom_page_link(&domain_name);

    // Parse request body
    let update_req: UpdateCustomPagesRequest = match req.parse_json().await {
        Ok(r) => r,
        Err(e) => {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "body".to_string(),
                    message: format!("Invalid JSON: {}", e),
                },
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    };

    // Validate URLs
    if let Some(ref url) = update_req.custom_index_url {
        if !url.is_empty() && !is_valid_url(url) {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "customIndexUrl".to_string(),
                    message: "Must be a valid HTTP or HTTPS URL".to_string(),
                },
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    }

    if let Some(ref url) = update_req.custom_not_found_url {
        if !url.is_empty() && !is_valid_url(url) {
            let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
                ValidationError::InvalidInput {
                    field: "customNotFoundUrl".to_string(),
                    message: "Must be a valid HTTP or HTTPS URL".to_string(),
                },
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
            return;
        }
    }

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    // Handle index page route
    let final_index_url = handle_custom_page_route(
        &app_state,
        INDEX_SWITCH,
        &link,
        update_req.custom_index_url,
    )
    .await;

    // Handle 404 page route
    let final_not_found_url = handle_custom_page_route(
        &app_state,
        NOT_FOUND_SWITCH,
        &link,
        update_req.custom_not_found_url,
    )
    .await;

    let dto = CustomPagesDto {
        domain_name,
        custom_index_url: final_index_url,
        custom_not_found_url: final_not_found_url,
    };

    res.render(Json(dto));
}

/// Delete all custom pages for a domain
///
/// Removes both the custom index and 404 page configurations for a domain.
#[endpoint(
    operation_id = "delete_custom_pages",
    summary = "Delete custom pages for a domain",
    description = "Removes both the custom index and 404 page configurations for a domain.",
    parameters(
        ("domain_name" = String, Path, description = "The domain name", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Custom pages deleted successfully", body = serde_json::Value),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_custom_pages(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain_name = req.param::<String>("domain_name").unwrap_or_default();
    let link = create_custom_page_link(&domain_name);

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    let mut deleted_count = 0;

    // Delete index route if exists
    if let Ok(Some(route)) = app_state.routes_store.get_route(INDEX_SWITCH, &link).await {
        if app_state.routes_store.delete_route(&route).await.is_ok() {
            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                publisher
                    .publish_route_changed(&RouteChangedMessage::from_route(
                        &route,
                        ChangeAction::Deleted,
                    ))
                    .await;
            }
            deleted_count += 1;
        }
    }

    // Delete 404 route if exists
    if let Ok(Some(route)) = app_state.routes_store.get_route(NOT_FOUND_SWITCH, &link).await {
        if app_state.routes_store.delete_route(&route).await.is_ok() {
            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                publisher
                    .publish_route_changed(&RouteChangedMessage::from_route(
                        &route,
                        ChangeAction::Deleted,
                    ))
                    .await;
            }
            deleted_count += 1;
        }
    }

    res.render(Json(serde_json::json!({
        "message": "Custom pages deleted successfully",
        "domainName": domain_name,
        "deletedCount": deleted_count
    })));
}

/// Handle creating, updating, or deleting a custom page route
async fn handle_custom_page_route(
    app_state: &std::sync::Arc<AppState>,
    switch: &str,
    link: &str,
    new_url: Option<String>,
) -> Option<String> {
    // Get existing route
    let existing_route = app_state
        .routes_store
        .get_route(switch, link)
        .await
        .ok()
        .flatten();

    match new_url {
        Some(url) if !url.is_empty() => {
            // Create or update route
            let route = Route {
                switch: switch.to_string(),
                link: link.to_string(),
                dest: Some(url.clone()),
                dest_format: DestinationFormat::Http,
                code: Some(302),
                ttl: Some(0),
                status: RouteStatus::Active,
                terminal: RoutingTerminal::External,
                policy: RoutingPolicy::Basic,
                properties: RouteProperties {
                    route_id: existing_route
                        .as_ref()
                        .and_then(|r| r.properties.route_id.clone())
                        .or_else(|| Some(generate_route_id())),
                    domain_id: None,
                    owner_id: existing_route
                        .as_ref()
                        .and_then(|r| r.properties.owner_id.clone()),
                    creator_id: existing_route
                        .as_ref()
                        .and_then(|r| r.properties.creator_id.clone()),
                    workspace_id: existing_route
                        .as_ref()
                        .and_then(|r| r.properties.workspace_id.clone()),
                    scripts: None,
                    tags: Some(vec![format!("custom-page:{}", switch)]),
                    custom: None,
                    native: None,
                    bundling: None,
                    opengraph: false,
                    allow_debug: false,
                },
            };

            let action = if existing_route.is_some() {
                ChangeAction::Updated
            } else {
                ChangeAction::Created
            };

            if existing_route.is_some() {
                let _ = app_state.routes_store.update_route(&route).await;
            } else {
                let _ = app_state.routes_store.store_route(&route).await;
            }

            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                publisher
                    .publish_route_changed(&RouteChangedMessage::from_route(&route, action))
                    .await;
            }

            Some(url)
        }
        _ => {
            // Delete route if exists
            if let Some(route) = existing_route {
                let _ = app_state.routes_store.delete_route(&route).await;
                if let Some(ref publisher) = app_state.rabbitmq_publisher {
                    publisher
                        .publish_route_changed(&RouteChangedMessage::from_route(
                            &route,
                            ChangeAction::Deleted,
                        ))
                        .await;
                }
            }
            None
        }
    }
}

/// Validate that a URL is a valid HTTP/HTTPS URL
fn is_valid_url(url: &str) -> bool {
    match url::Url::parse(url) {
        Ok(parsed) => {
            let scheme = parsed.scheme();
            scheme == "http" || scheme == "https"
        }
        Err(_) => false,
    }
}
