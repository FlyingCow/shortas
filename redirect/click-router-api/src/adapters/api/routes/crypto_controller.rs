use chrono::{DateTime, Utc};
use salvo::oapi::endpoint;
use salvo::oapi::ToSchema;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::crypto_store::certificate_expiry;
use crate::adapters::rabbitmq::messages::{CertificateChangedMessage, ChangeAction};
use crate::adapters::api::{app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter, openapi_schemas::ErrorResponse};
use crate::dto::KeycertDto;
use crate::model::error::{ApiError, RouteError, ValidationError};
use crate::model::keycert::Keycert;

/// Tells every router to drop its cached copy of the domain's certificate.
async fn publish_change(app_state: &AppState, domain: &str, action: ChangeAction) {
    if let Some(ref publisher) = app_state.rabbitmq_publisher {
        publisher
            .publish_certificate_changed(&CertificateChangedMessage {
                domain: domain.to_string(),
                action,
            })
            .await;
    }
}

/// Query parameters for certificate listing
#[derive(Debug, Deserialize)]
pub struct CertificateListQuery {
    /// Unix timestamp - only return certificates expiring before this time
    pub expires_before: Option<i64>,
}

/// Response for listing expiring certificates
#[derive(Debug, Serialize, ToSchema)]
pub struct ExpiringCertificatesResponse {
    pub certificates: Vec<CertificateExpiryRecord>,
}

/// Certificate expiry record
#[derive(Debug, Serialize, ToSchema)]
pub struct CertificateExpiryRecord {
    pub domain: String,
    pub owner_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub fn api_routes() -> Router {
    Router::with_path("/certificates")
        .get(list_expiring_certificates)
        .push(
            Router::with_path("/{domain}")
                .get(get_certificate)
                .post(create_certificate)
                .put(update_certificate)
                .delete(delete_certificate)
                .push(Router::with_path("/status").get(get_certificate_status)),
        )
}

/// List certificates expiring before a given date
///
/// Returns a list of certificates that will expire before the specified timestamp.
/// This is used by cert-bot to find certificates that need renewal.
#[endpoint(
    operation_id = "list_expiring_certificates",
    summary = "List expiring certificates",
    description = "Returns certificates expiring before the given Unix timestamp. Used for automated renewal workflows.",
    parameters(
        ("expires_before" = i64, Query, description = "Unix timestamp - only return certificates expiring before this time")
    ),
    responses(
        (status_code = 200, description = "List of expiring certificates", body = ExpiringCertificatesResponse),
        (status_code = 400, description = "Bad request - Invalid query parameter", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn list_expiring_certificates(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let query: CertificateListQuery = req.parse_queries().unwrap_or(CertificateListQuery {
        expires_before: None,
    });

    let Some(expires_before) = query.expires_before else {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("expires_before".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    };

    let Some(before_datetime) = DateTime::from_timestamp(expires_before, 0) else {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "expires_before".to_string(),
                message: "Invalid Unix timestamp".to_string(),
            },
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    };

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match app_state
        .crypto_store
        .get_certificates_expiring_before(before_datetime)
        .await
    {
        Ok(certificates) => {
            let response = ExpiringCertificatesResponse {
                certificates: certificates
                    .into_iter()
                    .map(|c| CertificateExpiryRecord {
                        domain: c.domain,
                        owner_id: c.owner_id,
                        expires_at: c.expires_at,
                    })
                    .collect(),
            };
            res.render(Json(response));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Get a certificate's expiry without its key material
#[endpoint(
    operation_id = "get_certificate_status",
    summary = "Get certificate expiry",
    parameters(("domain" = String, Path, description = "The domain name", example = "example.com")),
    responses(
        (status_code = 200, description = "Expiry", body = CertificateExpiryRecord),
        (status_code = 404, description = "No certificate", body = ErrorResponse)
    )
)]
pub async fn get_certificate_status(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    let error_response = match app_state.crypto_store.get_certificate(&domain).await {
        Ok(Some(certificate)) => {
            res.render(Json(CertificateExpiryRecord {
                domain,
                owner_id: None,
                expires_at: certificate_expiry(&certificate.cert),
            }));
            return;
        }
        Ok(None) => ErrorPresenter::from_api_error(&ApiError::Route(RouteError::NotFound {
            switch: "certificate".to_string(),
            domain,
            path: "".to_string(),
        })),
        Err(e) => ErrorPresenter::map_error(e),
    };
    res.status_code(error_response.status_code);
    res.render(error_response);
}

/// Get SSL certificate for a domain
///
/// Retrieves the SSL certificate information for a specific domain.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "get_certificate",
    summary = "Get SSL certificate",
    description = "Retrieves the SSL certificate information for a specific domain. The certificate includes the private key, certificate, and OCSP response in PEM format. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("domain" = String, Path, description = "The domain name for the certificate", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Certificate found successfully", body = KeycertDto),
        (status_code = 404, description = "Certificate not found", body = ErrorResponse),
        (status_code = 400, description = "Bad request - Invalid domain parameter", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    // Get AppState from depot
    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    let certificate = app_state
        .crypto_store
        .get_certificate(domain.as_str())
        .await;

    match certificate {
        Ok(Some(certificate)) => {
            // Convert the internal Keycert to KeycertDto for API response
            let keycert_dto = KeycertDto::from(certificate);
            res.render(Json(keycert_dto));
        }
        Ok(None) => {
        let error_response =
            ErrorPresenter::from_api_error(&ApiError::Route(RouteError::NotFound {
                switch: "certificate".to_string(),
                domain: domain.clone(),
                path: "".to_string(),
            }));
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

/// Create a new SSL certificate for a domain
///
/// Creates a new SSL certificate entry for the specified domain.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "create_certificate",
    summary = "Create SSL certificate",
    description = "Creates a new SSL certificate entry for the specified domain. The certificate data must be provided in PEM format. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("domain" = String, Path, description = "The domain name for the certificate", example = "example.com")
    ),
    responses(
        (status_code = 201, description = "Certificate created successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 409, description = "Conflict - Certificate already exists", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    if domain.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("domain".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Parse the certificate data from the request body
    let keycert_dto: KeycertDto = match req.parse_json().await {
        Ok(keycert_dto) => keycert_dto,
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

    // Validate the certificate data
    if !keycert_dto.is_valid() {
        let error_response =
            ErrorPresenter::from_api_error(&ApiError::Validation(ValidationError::InvalidInput {
                field: "certificate".to_string(),
                message: "Certificate data is incomplete or invalid".to_string(),
            }));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal format
    let keycert = Keycert::from(keycert_dto);

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    // Store the certificate
    match app_state
        .crypto_store
        .store_certificate(&domain, &keycert)
        .await
    {
        Ok(_) => {
            publish_change(app_state, &domain, ChangeAction::Created).await;
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "Certificate created successfully",
                "domain": domain
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Update an existing SSL certificate for a domain
///
/// Updates the SSL certificate information for the specified domain.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "update_certificate",
    summary = "Update SSL certificate",
    description = "Updates the SSL certificate information for the specified domain. The certificate data must be provided in PEM format. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("domain" = String, Path, description = "The domain name for the certificate", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Certificate updated successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Not found - Certificate not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    if domain.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("domain".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Parse the certificate data from the request body
    let keycert_dto: KeycertDto = match req.parse_json().await {
        Ok(keycert_dto) => keycert_dto,
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

    // Validate the certificate data
    if !keycert_dto.is_valid() {
        let error_response =
            ErrorPresenter::from_api_error(&ApiError::Validation(ValidationError::InvalidInput {
                field: "certificate".to_string(),
                message: "Certificate data is incomplete or invalid".to_string(),
            }));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal format
    let keycert = Keycert::from(keycert_dto);

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    // Update the certificate
    match app_state
        .crypto_store
        .update_certificate(&domain, &keycert)
        .await
    {
        Ok(_) => {
            publish_change(app_state, &domain, ChangeAction::Updated).await;
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Certificate updated successfully",
                "domain": domain
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Delete an SSL certificate for a domain
///
/// Deletes the SSL certificate information for the specified domain.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "delete_certificate",
    summary = "Delete SSL certificate",
    description = "Deletes the SSL certificate information for the specified domain. This action is irreversible. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("domain" = String, Path, description = "The domain name for the certificate", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Certificate deleted successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid domain parameter", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Not found - Certificate not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_certificate(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    if domain.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("domain".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    // Delete the certificate
    match app_state.crypto_store.delete_certificate(&domain).await {
        Ok(_) => {
            publish_change(app_state, &domain, ChangeAction::Deleted).await;
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Certificate deleted successfully",
                "domain": domain
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
