//! Certificates controller for SSL/TLS management.

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::{CertificateDto, CreateCertificateDto, UpdateCertificateDto};
use crate::domain::entities::ApiError;
use crate::presentation::middleware::{
    render_created, render_error, render_no_content, render_success, DepotExt, UserExt,
};

use super::cors_preflight;

/// Build certificates controller router.
pub fn certificates_controller() -> Router {
    Router::with_path("certificates")
        .get(list_certificates)
        .post(create_certificate)
        .options(cors_preflight)
        .push(
            Router::with_path("{id}")
                .get(get_certificate)
                .put(update_certificate)
                .delete(delete_certificate)
                .options(cors_preflight),
        )
}

/// List certificates for the current user.
#[endpoint(
    operation_id = "list_certificates",
    summary = "List certificates",
    description = "List all certificates owned by the current user",
    tags("Certificates"),
    responses(
        (status_code = 200, description = "Certificates list", body = Vec<CertificateDto>)
    )
)]
pub async fn list_certificates(depot: &mut Depot, res: &mut Response) {
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

    match app_state.certificate_repo.list_by_owner(&user_id).await {
        Ok(certs) => {
            let dtos: Vec<CertificateDto> = certs.into_iter().map(CertificateDto::from_entity).collect();
            render_success(res, dtos);
        }
        Err(e) => render_error(res, e),
    }
}

/// Get a certificate by ID.
#[endpoint(
    operation_id = "get_certificate",
    summary = "Get certificate",
    description = "Get a certificate by its ID",
    tags("Certificates"),
    parameters(
        ("id" = String, Path, description = "Certificate ID")
    ),
    responses(
        (status_code = 200, description = "Certificate details", body = CertificateDto),
        (status_code = 404, description = "Certificate not found")
    )
)]
pub async fn get_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid certificate ID"));
            return;
        }
    };

    match app_state.certificate_repo.get_by_id(id).await {
        Ok(Some(cert)) => {
            if cert.owner_id != user_id {
                render_error(res, ApiError::forbidden());
                return;
            }
            render_success(res, CertificateDto::from_entity(cert));
        }
        Ok(None) => render_error(res, ApiError::not_found("Certificate", &id_str)),
        Err(e) => render_error(res, e),
    }
}

/// Create a new certificate.
#[endpoint(
    operation_id = "create_certificate",
    summary = "Create certificate",
    description = "Upload a new SSL/TLS certificate",
    tags("Certificates"),
    request_body(content = CreateCertificateDto, description = "Certificate to create"),
    responses(
        (status_code = 201, description = "Certificate created", body = CertificateDto),
        (status_code = 400, description = "Invalid input")
    )
)]
pub async fn create_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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

    let dto: CreateCertificateDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    // Verify domain ownership
    let domain_id = match Uuid::parse_str(&dto.domain_id) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid domain_id"));
            return;
        }
    };

    match app_state.domain_repo.get_by_id(domain_id).await {
        Ok(Some(domain)) => {
            if domain.owner_id != user_id {
                render_error(res, ApiError::forbidden());
                return;
            }
        }
        Ok(None) => {
            render_error(res, ApiError::not_found("Domain", &dto.domain_id));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    }

    let cert = match dto.to_entity(&user_id) {
        Ok(c) => c,
        Err(e) => {
            render_error(res, ApiError::validation(e));
            return;
        }
    };

    match app_state.certificate_repo.create(&cert).await {
        Ok(created) => render_created(res, CertificateDto::from_entity(created)),
        Err(e) => render_error(res, e),
    }
}

/// Update a certificate.
#[endpoint(
    operation_id = "update_certificate",
    summary = "Update certificate",
    description = "Update an existing certificate",
    tags("Certificates"),
    parameters(
        ("id" = String, Path, description = "Certificate ID")
    ),
    request_body(content = UpdateCertificateDto, description = "Certificate updates"),
    responses(
        (status_code = 200, description = "Certificate updated", body = CertificateDto)
    )
)]
pub async fn update_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid certificate ID"));
            return;
        }
    };

    let dto: UpdateCertificateDto = match req.parse_json().await {
        Ok(dto) => dto,
        Err(e) => {
            render_error(res, ApiError::validation(e.to_string()));
            return;
        }
    };

    let cert = match app_state.certificate_repo.get_by_id(id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            render_error(res, ApiError::not_found("Certificate", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if cert.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    let updated = dto.apply_to(cert);

    match app_state.certificate_repo.update(&updated).await {
        Ok(saved) => render_success(res, CertificateDto::from_entity(saved)),
        Err(e) => render_error(res, e),
    }
}

/// Delete a certificate.
#[endpoint(
    operation_id = "delete_certificate",
    summary = "Delete certificate",
    description = "Delete a certificate",
    tags("Certificates"),
    parameters(
        ("id" = String, Path, description = "Certificate ID")
    ),
    responses(
        (status_code = 204, description = "Certificate deleted")
    )
)]
pub async fn delete_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
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
            render_error(res, ApiError::validation("Invalid certificate ID"));
            return;
        }
    };

    let cert = match app_state.certificate_repo.get_by_id(id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            render_error(res, ApiError::not_found("Certificate", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if cert.owner_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.certificate_repo.delete(id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}
