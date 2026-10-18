//! Domains controller for managing route domains.

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::{CreateDomainDto, DnsConfigDto, DomainDto, PaginatedDomainsDto, UpdateDomainDto};
use crate::domain::entities::ApiError;
use crate::presentation::middleware::{
    render_created, render_error, render_no_content, render_success, DepotExt, UserExt,
};

use super::cors_preflight;

/// Build domains controller router.
pub fn domains_controller() -> Router {
    Router::with_path("domains")
        .get(list_domains)
        .post(create_domain)
        .options(cors_preflight)
        .push(Router::with_path("dns-config").get(get_global_dns_config).options(cors_preflight))
        .push(Router::with_path("shared").get(list_shared_domains).options(cors_preflight))
        .push(
            Router::with_path("{id}")
                .get(get_domain)
                .put(update_domain)
                .delete(delete_domain)
                .options(cors_preflight)
                .push(Router::with_path("dns-config").get(get_dns_config).options(cors_preflight))
                .push(Router::with_path("verify").post(verify_domain).options(cors_preflight)),
        )
}

/// List domains for the current user with pagination.
#[endpoint(
    operation_id = "list_domains",
    summary = "List domains",
    description = "List all domains accessible by the current user with pagination",
    tags("Domains"),
    parameters(
        ("page" = Option<i32>, Query, description = "Page number (default: 1)"),
        ("pageSize" = Option<i32>, Query, description = "Page size (default: 20)"),
        ("search" = Option<String>, Query, description = "Search term for domain name")
    ),
    responses(
        (status_code = 200, description = "Paginated domains list", body = PaginatedDomainsDto)
    )
)]
pub async fn list_domains(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
    let page_size: i32 = req.query("pageSize").unwrap_or(20);
    let search: Option<String> = req.query("search");

    match app_state.domain_repo.list_accessible(&user_id).await {
        Ok(domains) => {
            // Apply search filter if provided
            let filtered: Vec<_> = if let Some(ref search_term) = search {
                let term = search_term.to_lowercase();
                domains.into_iter()
                    .filter(|d| d.name.to_lowercase().contains(&term))
                    .collect()
            } else {
                domains
            };

            let total_count = filtered.len() as i64;

            // Apply pagination
            let start = ((page - 1) * page_size) as usize;
            let paginated: Vec<_> = filtered.into_iter()
                .skip(start)
                .take(page_size as usize)
                .collect();

            let dtos: Vec<DomainDto> = paginated.into_iter().map(DomainDto::from_entity).collect();
            render_success(res, PaginatedDomainsDto::new(dtos, page, page_size, total_count));
        }
        Err(e) => render_error(res, e),
    }
}

/// Get global DNS configuration for domain verification.
#[endpoint(
    operation_id = "get_global_dns_config",
    summary = "Get DNS config",
    description = "Get DNS configuration for domain verification",
    tags("Domains"),
    responses(
        (status_code = 200, description = "DNS configuration")
    )
)]
pub async fn get_global_dns_config(res: &mut Response) {
    // Return DNS config from settings or defaults
    render_success(res, serde_json::json!({
        "txtRecordName": "_shortas-domain-challenge",
        "allowedIpv4": ["203.0.113.10"],
        "allowedIpv6": []
    }));
}

/// List shared domains.
#[endpoint(
    operation_id = "list_shared_domains",
    summary = "List shared domains",
    description = "List all shared domains available to all users",
    tags("Domains"),
    responses(
        (status_code = 200, description = "Shared domains list", body = Vec<DomainDto>)
    )
)]
pub async fn list_shared_domains(depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    match app_state.domain_repo.list_shared().await {
        Ok(domains) => {
            let dtos: Vec<DomainDto> = domains.into_iter().map(DomainDto::from_entity).collect();
            render_success(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Get a domain by ID.
#[endpoint(
    operation_id = "get_domain",
    summary = "Get domain",
    description = "Get a domain by its ID",
    tags("Domains"),
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "Domain details", body = DomainDto),
        (status_code = 404, description = "Domain not found")
    )
)]
pub async fn get_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid domain ID"));
            return;
        }
    };

    match app_state.domain_repo.get_by_id(id).await {
        Ok(Some(domain)) => {
            if !domain.can_use(&user_id) {
                render_error(res, ApiError::forbidden());
                return;
            }
            render_success(res, DomainDto::from_entity(domain));
        }
        Ok(None) => render_error(res, ApiError::not_found("Domain", &id_str)),
        Err(e) => render_error(res, e),
    }
}

/// Create a new domain.
#[endpoint(
    operation_id = "create_domain",
    summary = "Create domain",
    description = "Create a new domain",
    tags("Domains"),
    request_body(content = CreateDomainDto, description = "Domain to create"),
    responses(
        (status_code = 201, description = "Domain created", body = DomainDto),
        (status_code = 400, description = "Invalid input"),
        (status_code = 409, description = "Domain already exists")
    )
)]
pub async fn create_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let dto: CreateDomainDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    // Check if domain already exists
    if app_state.domain_repo.name_exists(&dto.name).await.unwrap_or(false) {
        render_error(res, ApiError::conflict("Domain already exists"));
        return;
    }

    let domain = dto.to_entity(&user_id);

    match app_state.domain_repo.create(&domain).await {
        Ok(created) => render_created(res, DomainDto::from_entity(created)),
        Err(e) => render_error(res, e),
    }
}

/// Update a domain.
#[endpoint(
    operation_id = "update_domain",
    summary = "Update domain",
    description = "Update an existing domain",
    tags("Domains"),
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    request_body(content = UpdateDomainDto, description = "Domain updates"),
    responses(
        (status_code = 200, description = "Domain updated", body = DomainDto),
        (status_code = 404, description = "Domain not found")
    )
)]
pub async fn update_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid domain ID"));
            return;
        }
    };

    let dto: UpdateDomainDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let domain = match app_state.domain_repo.get_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            render_error(res, ApiError::not_found("Domain", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if domain.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    let updated = dto.apply_to(domain);

    match app_state.domain_repo.update(&updated).await {
        Ok(saved) => render_success(res, DomainDto::from_entity(saved)),
        Err(e) => render_error(res, e),
    }
}

/// Delete a domain.
#[endpoint(
    operation_id = "delete_domain",
    summary = "Delete domain",
    description = "Delete a domain",
    tags("Domains"),
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 204, description = "Domain deleted"),
        (status_code = 404, description = "Domain not found")
    )
)]
pub async fn delete_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid domain ID"));
            return;
        }
    };

    let domain = match app_state.domain_repo.get_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            render_error(res, ApiError::not_found("Domain", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if domain.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.domain_repo.delete(id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}

/// Get DNS configuration for a domain.
#[endpoint(
    operation_id = "get_dns_config",
    summary = "Get DNS config",
    description = "Get DNS configuration required for domain verification",
    tags("Domains"),
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 200, description = "DNS configuration", body = DnsConfigDto)
    )
)]
pub async fn get_dns_config(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid domain ID"));
            return;
        }
    };

    let domain = match app_state.domain_repo.get_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            render_error(res, ApiError::not_found("Domain", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if domain.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    // Generate verification records
    let txt_record = format!("shortas-verify={}", id);
    let cname_target = "redirect.shortas.work".to_string();
    let a_records = vec!["1.2.3.4".to_string()]; // Replace with actual IPs

    render_success(
        res,
        DnsConfigDto {
            txt_record,
            cname_target,
            a_records,
        },
    );
}

/// Trigger domain verification.
#[endpoint(
    operation_id = "verify_domain",
    summary = "Verify domain",
    description = "Trigger DNS verification for a domain",
    tags("Domains"),
    parameters(
        ("id" = String, Path, description = "Domain ID")
    ),
    responses(
        (status_code = 202, description = "Verification started"),
        (status_code = 404, description = "Domain not found")
    )
)]
pub async fn verify_domain(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid domain ID"));
            return;
        }
    };

    let domain = match app_state.domain_repo.get_by_id(id).await {
        Ok(Some(d)) => d,
        Ok(None) => {
            render_error(res, ApiError::not_found("Domain", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if domain.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    // Queue verification job
    let outbox_msg = crate::domain::entities::OutboxMessage::verify_domain(id);
    let _ = app_state.outbox_repo.create(&outbox_msg).await;

    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(serde_json::json!({
        "message": "Verification started"
    })));
}
