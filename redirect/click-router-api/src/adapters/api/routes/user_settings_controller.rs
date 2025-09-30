use salvo::{prelude::*};
use salvo::oapi::endpoint;

use crate::adapters::api::{app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter, middleware::JwtAuthContext, openapi_schemas::ErrorResponse};
use crate::dto::UserSettingsDto;
use crate::model::error::{ApiError, AuthenticationError, ValidationError};
use crate::model::user_settings::UserSettings;

pub fn api_routes() -> Router {
    Router::with_path("/user-settings").push(
        Router::with_path("/{user_id}")
            .get(get_user_settings)
            .post(create_user_settings)
            .put(update_user_settings)
            .delete(delete_user_settings),
    )
}

/// Get user settings
/// 
/// Retrieves user settings for a specific user ID. If no user ID is provided in the URL,
/// the user ID from the JWT token context will be used.
#[endpoint(
    operation_id = "get_user_settings",
    summary = "Get user settings",
    description = "Retrieves user settings for a specific user ID. If no user ID is provided in the URL, the user ID from the JWT token context will be used. Returns user preferences, debug settings, and configuration options.",
    parameters(
        ("user_id" = String, Path, description = "The user ID for the settings", example = "user123")
    ),
    responses(
        (status_code = 200, description = "User settings found successfully", body = UserSettingsDto),
        (status_code = 404, description = "User not found", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_user_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = req.param::<String>("user_id").unwrap_or_default();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    let user_settings = app_state
        .user_settings_store
        .get_user_settings(user_id.as_str())
        .await;

    match user_settings {
        Ok(Some(user_settings)) => {
            // Convert the internal UserSettings to UserSettingsDto for API response
            let user_settings_dto = UserSettingsDto::from(user_settings);
            res.render(Json(user_settings_dto));
        }
        Ok(None) => {
            // Get user ID from JWT context if not provided in URL
            let final_user_id = if user_id.is_empty() {
                match depot.get::<JwtAuthContext>("jwt_auth_context") {
                    Ok(context) => context.user_id.clone(),
                    Err(_) => user_id,
                }
            } else {
                user_id
            };
            
            let error_response = ErrorPresenter::from_api_error(&ApiError::Authentication(
                AuthenticationError::UserNotFound(final_user_id)
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

/// Create new user settings
///
/// Creates new user settings for a specific user ID.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "create_user_settings",
    summary = "Create user settings",
    description = "Creates new user settings for a specific user ID. The settings data must be provided in the request body. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("user_id" = String, Path, description = "The user ID for the settings", example = "user123")
    ),
    responses(
        (status_code = 201, description = "User settings created successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 409, description = "Conflict - User settings already exist", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn create_user_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = req.param::<String>("user_id").unwrap_or_default();

    if user_id.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("user_id".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Parse the request body
    let keycert_dto: Result<UserSettingsDto, _> = req.parse_json().await;
    let keycert_dto = match keycert_dto {
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

    // Validate the user settings data
    if !keycert_dto.is_valid() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "user_settings".to_string(),
                message: "User settings data is incomplete or invalid".to_string(),
            },
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal model
    let user_settings: UserSettings = keycert_dto.into();
    let user_settings = UserSettings::new(
        user_id.clone(),
        user_settings.user_email,
        user_settings.api_key,
        user_settings.active_status,
        user_settings.debug,
        user_settings.overflow,
        user_settings.skip,
        user_settings.allowed_request_params,
        user_settings.allowed_destination_params,
    );

    // Get AppState from depot
    let app_state = depot.get::<AppState>("app_state").unwrap();

    // Store the user settings
    match app_state
        .user_settings_store
        .store_user_settings(&user_settings)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::CREATED);
            res.render(Json(serde_json::json!({
                "message": "User settings created successfully",
                "user_id": user_id
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
/// Update existing user settings
///
/// Updates existing user settings for a specific user ID.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "update_user_settings",
    summary = "Update user settings",
    description = "Updates existing user settings for a specific user ID. The settings data must be provided in the request body. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("user_id" = String, Path, description = "The user ID for the settings", example = "user123")
    ),
    responses(
        (status_code = 200, description = "User settings updated successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid input data", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Not found - User settings not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn update_user_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = req.param::<String>("user_id").unwrap_or_default();

    if user_id.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("user_id".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Parse the request body
    let keycert_dto: Result<UserSettingsDto, _> = req.parse_json().await;
    let keycert_dto = match keycert_dto {
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

    // Validate the user settings data
    if !keycert_dto.is_valid() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::InvalidInput {
                field: "user_settings".to_string(),
                message: "User settings data is incomplete or invalid".to_string(),
            },
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Convert DTO to internal model
    let user_settings: UserSettings = keycert_dto.into();
    let user_settings = UserSettings::new(
        user_id.clone(),
        user_settings.user_email,
        user_settings.api_key,
        user_settings.active_status,
        user_settings.debug,
        user_settings.overflow,
        user_settings.skip,
        user_settings.allowed_request_params,
        user_settings.allowed_destination_params,
    );

    // Get AppState from depot
    let app_state = depot.get::<AppState>("app_state").unwrap();

    // Update the user settings
    match app_state
        .user_settings_store
        .update_user_settings(&user_settings)
        .await
    {
        Ok(_) => {
            res.status_code(StatusCode::OK);
            res.render(Json(serde_json::json!({
                "message": "User settings updated successfully",
                "user_id": user_id
            })));
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}

/// Delete user settings
///
/// Deletes user settings for a specific user ID.
/// This endpoint requires JWT authentication and appropriate permissions.
#[endpoint(
    operation_id = "delete_user_settings",
    summary = "Delete user settings",
    description = "Deletes user settings for a specific user ID. This action is irreversible. Requires JWT authentication with appropriate permissions.",
    parameters(
        ("user_id" = String, Path, description = "The user ID for the settings", example = "user123")
    ),
    responses(
        (status_code = 200, description = "User settings deleted successfully", body = serde_json::Value),
        (status_code = 400, description = "Bad request - Invalid user ID", body = ErrorResponse),
        (status_code = 401, description = "Unauthorized - Invalid or missing JWT token", body = ErrorResponse),
        (status_code = 403, description = "Forbidden - Insufficient permissions", body = ErrorResponse),
        (status_code = 404, description = "Not found - User settings not found", body = ErrorResponse),
        (status_code = 500, description = "Internal server error", body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_user_settings(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user_id = req.param::<String>("user_id").unwrap_or_default();

    if user_id.is_empty() {
        let error_response = ErrorPresenter::from_api_error(&ApiError::Validation(
            ValidationError::MissingField("user_id".to_string()),
        ));
        res.status_code(error_response.status_code);
        res.render(error_response);
        return;
    }

    // Get AppState from depot
    let app_state = depot.get::<AppState>("app_state").unwrap();

    // First, get the existing user settings to pass to delete
    match app_state
        .user_settings_store
        .get_user_settings(&user_id)
        .await
    {
        Ok(Some(user_settings)) => {
            // Delete the user settings
            match app_state
                .user_settings_store
                .delete_user_settings(&user_settings)
                .await
            {
                Ok(_) => {
                    res.status_code(StatusCode::OK);
                    res.render(Json(serde_json::json!({
                        "message": "User settings deleted successfully",
                        "user_id": user_id
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
            let error_response = ErrorPresenter::from_api_error(&ApiError::Authentication(
                AuthenticationError::UserNotFound(user_id),
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
