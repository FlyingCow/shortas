use chrono::{Duration, Utc};
use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::adapters::api::{
    app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter,
    openapi_schemas::ErrorResponse,
};
use crate::dto::ChallengeDto;
use crate::model::error::{ApiError, RouteError, ValidationError};

pub fn api_routes() -> Router {
    Router::with_path("/challenges")
        .push(
            Router::with_path("/{domain}/{token}")
                .get(get_challenge)
                .put(store_challenge)
                .delete(delete_challenge),
        )
        .push(Router::with_path("/{domain}").delete(delete_domain_challenges))
}

/// Store ACME HTTP-01 challenge for a domain
///
/// Stores or updates an ACME HTTP-01 challenge that will be served at
/// `/.well-known/acme-challenge/{token}` by click-router.
#[endpoint(
    operation_id = "store_challenge",
    summary = "Store ACME challenge",
    description = "Stores or updates an ACME HTTP-01 challenge for the specified domain. The challenge will be served by click-router at /.well-known/acme-challenge/{token}",
    parameters(
        ("domain" = String, Path, description = "The domain name for the challenge", example = "example.com"),
        ("token" = String, Path, description = "The challenge token from Let's Encrypt", example = "abc123xyz")
    ),
    responses(
        (status_code = 201, description = "Challenge stored successfully"),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn store_challenge(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let token = req.param::<String>("token").unwrap_or_default();

    if domain.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("domain".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    if token.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("token".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    let challenge_dto: ChallengeDto = match req.parse_json().await {
        Ok(dto) => dto,
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

    if challenge_dto.key_authorization.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("key_authorization".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Default expiry is 1 hour from now if not specified
    let expires_at = challenge_dto
        .expires_at
        .unwrap_or_else(|| Utc::now() + Duration::hours(1));

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match app_state
        .challenge_store
        .store_challenge(&domain, &token, &challenge_dto.key_authorization, expires_at)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "Challenge stored successfully",
                "domain": domain,
                "token": token,
                "expires_at": expires_at
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Get ACME HTTP-01 challenge
///
/// Retrieves an ACME HTTP-01 challenge for the specified domain and token.
#[endpoint(
    operation_id = "get_challenge",
    summary = "Get ACME challenge",
    description = "Retrieves an ACME HTTP-01 challenge for the specified domain and token",
    parameters(
        ("domain" = String, Path, description = "The domain name for the challenge", example = "example.com"),
        ("token" = String, Path, description = "The challenge token", example = "abc123xyz")
    ),
    responses(
        (status_code = 200, description = "Challenge found", body = ChallengeDto),
        (status_code = 404, description = "Challenge not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_challenge(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let token = req.param::<String>("token").unwrap_or_default();

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match app_state.challenge_store.get_challenge(&domain, &token).await {
        Ok(Some(challenge)) => {
            res.render(Json(ChallengeDto::from(challenge)));
        }
        Ok(None) => {
            let error_response =
                ErrorPresenter::from_api_error(&ApiError::Route(RouteError::NotFound {
                    switch: "challenge".to_string(),
                    domain: domain.clone(),
                    path: token.clone(),
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

/// Delete ACME HTTP-01 challenge
///
/// Deletes a specific ACME HTTP-01 challenge for the given domain and token.
#[endpoint(
    operation_id = "delete_challenge",
    summary = "Delete ACME challenge",
    description = "Deletes a specific ACME HTTP-01 challenge for the given domain and token",
    parameters(
        ("domain" = String, Path, description = "The domain name for the challenge", example = "example.com"),
        ("token" = String, Path, description = "The challenge token", example = "abc123xyz")
    ),
    responses(
        (status_code = 204, description = "Challenge deleted successfully"),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_challenge(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let token = req.param::<String>("token").unwrap_or_default();

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match app_state
        .challenge_store
        .delete_challenge(&domain, &token)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::NO_CONTENT);
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Delete all ACME HTTP-01 challenges for a domain
///
/// Deletes all ACME HTTP-01 challenges associated with the specified domain.
#[endpoint(
    operation_id = "delete_domain_challenges",
    summary = "Delete all challenges for domain",
    description = "Deletes all ACME HTTP-01 challenges associated with the specified domain",
    parameters(
        ("domain" = String, Path, description = "The domain name", example = "example.com")
    ),
    responses(
        (status_code = 200, description = "Challenges deleted successfully"),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_domain_challenges(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match app_state
        .challenge_store
        .delete_domain_challenges(&domain)
        .await
    {
        Ok(deleted_count) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "Challenges deleted successfully",
                "domain": domain,
                "deleted_count": deleted_count
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
