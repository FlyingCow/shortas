use chrono::Utc;
use salvo::oapi::endpoint;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::info;

use crate::adapters::api::app_state::AppState;
use crate::adapters::rabbitmq::messages::DomainStateChangedMessage;
use crate::dto::{
    CreateDomainRequest, DnsConfigResponse, DomainDto, DomainListResponse, ErrorResponse,
    PaginationInfo,
};
use crate::model::Domain;

pub fn api_routes() -> Router {
    Router::with_path("/domains")
        .get(list_domains)
        .post(create_domain)
        .push(Router::with_path("/{id}").get(get_domain).delete(delete_domain))
        .push(Router::with_path("/{id}/verify").post(trigger_verification))
        .push(Router::with_path("/dns-config").get(get_dns_config))
}

#[endpoint(
    operation_id = "list_domains",
    summary = "List all domains",
    description = "Retrieves a list of all domains with optional filtering and pagination.",
    parameters(
        ("page" = u32, Query, description = "Page number (default: 1)"),
        ("pageSize" = u32, Query, description = "Number of items per page (default: 20)"),
        ("ownerId" = String, Query, description = "Filter by owner ID")
    ),
    responses(
        (status_code = 200, description = "List of domains retrieved successfully", body = DomainListResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn list_domains(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let page: u32 = req.query::<u32>("page").unwrap_or(1).max(1);
    let page_size: u32 = req.query::<u32>("pageSize").unwrap_or(20).min(100);
    let owner_id = req.query::<String>("ownerId");

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    match app_state
        .domain_store
        .list_domains(owner_id.as_deref(), page, page_size)
        .await
    {
        Ok((domains, total_count)) => {
            let total_pages = (total_count as f64 / page_size as f64).ceil() as u64;
            let response = DomainListResponse {
                data: domains.into_iter().map(DomainDto::from).collect(),
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
    operation_id = "create_domain",
    summary = "Create a new domain",
    description = "Creates a new domain for verification.",
    responses(
        (status_code = 201, description = "Domain created successfully", body = DomainDto),
        (status_code = 400, description = "Invalid request", body = ErrorResponse),
        (status_code = 409, description = "Domain already exists", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let create_req: CreateDomainRequest = match req.parse_json().await {
        Ok(r) => r,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(ErrorResponse::validation("body", &e.to_string())));
            return;
        }
    };

    if create_req.name.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("name", "Domain name is required")));
        return;
    }

    if create_req.id.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("id", "Domain ID is required")));
        return;
    }

    if create_req.owner_id.is_empty() {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorResponse::validation("owner_id", "Owner ID is required")));
        return;
    }

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Check if domain already exists
    match app_state
        .domain_store
        .get_domain_by_name(&create_req.name.to_lowercase(), &create_req.owner_id)
        .await
    {
        Ok(Some(_)) => {
            res.status_code(StatusCode::CONFLICT);
            res.render(Json(ErrorResponse::conflict("Domain with this name already exists")));
            return;
        }
        Ok(None) => {}
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    }

    let domain = Domain::from(create_req);

    match app_state.domain_store.store_domain(&domain).await {
        Ok(_) => {
            info!("Domain created: {} ({})", domain.name, domain.id);
            res.status_code(StatusCode::CREATED);
            res.render(Json(DomainDto::from(domain)));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "get_domain",
    summary = "Get domain by ID",
    description = "Retrieves a domain by its ID.",
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "Domain found", body = DomainDto),
        (status_code = 404, description = "Domain not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    match app_state.domain_store.get_domain(&id).await {
        Ok(Some(domain)) => {
            res.render(Json(DomainDto::from(domain)));
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Domain", &id)));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "delete_domain",
    summary = "Delete domain",
    description = "Deletes a domain by its ID.",
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "Domain deleted successfully"),
        (status_code = 404, description = "Domain not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Check if domain exists
    match app_state.domain_store.get_domain(&id).await {
        Ok(Some(_)) => {}
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Domain", &id)));
            return;
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    }

    match app_state.domain_store.delete_domain(&id).await {
        Ok(_) => {
            info!("Domain deleted: {}", id);
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Domain deleted successfully",
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
    description = "Triggers an immediate DNS verification for a domain.",
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "Verification completed", body = DomainDto),
        (status_code = 404, description = "Domain not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn trigger_verification(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let id = req.param::<String>("id").unwrap_or_default();

    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    // Get domain
    let mut domain = match app_state.domain_store.get_domain(&id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render(Json(ErrorResponse::not_found("Domain", &id)));
            return;
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
            return;
        }
    };

    // Perform verification
    let result = app_state.dns_verifier.verify(&domain).await;

    // Update domain
    domain.status = result.status.clone();
    domain.verification_reason = result.reason.clone();
    domain.last_check_at = Some(Utc::now());
    domain.next_check_at = Some(Utc::now() + chrono::Duration::minutes(30));

    match app_state.domain_store.update_domain(&domain).await {
        Ok(_) => {
            // Publish state change
            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                publisher
                    .publish_domain_state_changed(&DomainStateChangedMessage {
                        domain_id: domain.id.clone(),
                        domain_name: domain.name.clone(),
                        owner_id: domain.owner_id.clone(),
                        status: domain.status.clone(),
                        verification_reason: domain.verification_reason.clone(),
                        last_check_at: domain.last_check_at.map(|dt| dt.timestamp_millis()),
                        next_check_at: domain.next_check_at.map(|dt| dt.timestamp_millis()),
                    })
                    .await;
            }

            info!(
                "Domain verification triggered: {} -> {:?}",
                domain.name, domain.status
            );
            res.render(Json(DomainDto::from(domain)));
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse::internal(&e.to_string())));
        }
    }
}

#[endpoint(
    operation_id = "get_dns_config",
    summary = "Get DNS configuration",
    description = "Returns the DNS configuration values needed for domain verification.",
    responses(
        (status_code = 200, description = "DNS configuration", body = DnsConfigResponse)
    )
)]
pub async fn get_dns_config(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = depot.obtain::<Arc<AppState>>().unwrap();

    res.render(Json(DnsConfigResponse {
        txt_record_name: app_state.dns_settings.txt_record_name.clone(),
        allowed_ipv4: app_state.dns_settings.allowed_ipv4.clone(),
        allowed_ipv6: app_state.dns_settings.allowed_ipv6.clone(),
    }));
}
