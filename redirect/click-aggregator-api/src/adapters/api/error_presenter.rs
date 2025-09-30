use salvo::{http::StatusCode, Response, Scribe};
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::model::error::{
    ApiError, AuthenticationError, DatabaseError, ExternalServiceError, RouteError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPresenter {
    pub code: u16,
    pub error: String,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Error, Debug)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub error: String,
    pub details: Option<String>,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Scribe for ErrorResponse {
    fn render(self, res: &mut Response) {
        let error_response = ErrorPresenter {
            code: self.status_code.as_u16(),
            message: self.status_code.to_string(),
            error: self.error.clone(),
            details: self.details,
        };
        res.status_code(self.status_code);
        res.render(serde_json::to_string(&error_response).unwrap());
    }
}

impl ErrorResponse {
    /// Maps anyhow::Error to ErrorResponse with proper error classification
    pub fn map_error(e: anyhow::Error) -> ErrorResponse {
        // Try to downcast to our specific error types
        if let Some(api_error) = e.downcast_ref::<ApiError>() {
            return Self::from_api_error(api_error);
        }

        // If it's a database error from MongoDB or DynamoDB
        if e.to_string().contains("mongodb") || e.to_string().contains("dynamodb") {
            return ErrorResponse {
                status_code: StatusCode::BAD_GATEWAY,
                error: "Database service error".to_string(),
                details: Some(e.to_string()),
            };
        }

        // If it's an AWS error
        if e.to_string().contains("aws") || e.to_string().contains("AWS") {
            return ErrorResponse {
                status_code: StatusCode::BAD_GATEWAY,
                error: "External service error".to_string(),
                details: Some(e.to_string()),
            };
        }

        // Generic internal server error
        ErrorResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error: "Internal server error".to_string(),
            details: Some(e.to_string()),
        }
    }

    /// Maps ApiError to ErrorResponse with specific handling
    pub fn from_api_error(api_error: &ApiError) -> ErrorResponse {
        let status_code = match api_error {
            ApiError::Database(db_error) => match db_error {
                DatabaseError::ConnectionFailed(_) => StatusCode::SERVICE_UNAVAILABLE,
                DatabaseError::QueryFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseError::TransactionFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseError::SerializationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseError::DeserializationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseError::TableNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
                DatabaseError::DuplicateKey(_) => StatusCode::CONFLICT,
                DatabaseError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            },
            ApiError::Authentication(auth_error) => match auth_error {
                AuthenticationError::InvalidApiKey => StatusCode::UNAUTHORIZED,
                AuthenticationError::MissingToken => StatusCode::UNAUTHORIZED,
                AuthenticationError::ExpiredToken => StatusCode::UNAUTHORIZED,
                AuthenticationError::InsufficientPermissions(_) => StatusCode::FORBIDDEN,
                AuthenticationError::UserNotFound(_) => StatusCode::NOT_FOUND,
                AuthenticationError::AccountBlocked(_) => StatusCode::FORBIDDEN,
            },
            ApiError::Validation(_) => StatusCode::BAD_REQUEST,
            ApiError::Route(route_error) => match route_error {
                RouteError::NotFound { .. } => StatusCode::NOT_FOUND,
                RouteError::Blocked { .. } => StatusCode::FORBIDDEN,
                RouteError::Expired { .. } => StatusCode::GONE,
                RouteError::InvalidPolicy(_) => StatusCode::BAD_REQUEST,
                RouteError::CreationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                RouteError::UpdateFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                RouteError::DeletionFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ApiError::Configuration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ExternalService(ext_error) => match ext_error {
                ExternalServiceError::Aws(_) => StatusCode::BAD_GATEWAY,
                ExternalServiceError::MongoDB(_) => StatusCode::BAD_GATEWAY,
                ExternalServiceError::DynamoDB(_) => StatusCode::BAD_GATEWAY,
                ExternalServiceError::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
                ExternalServiceError::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
                ExternalServiceError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            },
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ErrorResponse {
            status_code,
            error: api_error.to_string(),
            details: Some(format!("{:?}", api_error)),
        }
    }

    /// Legacy method for backward compatibility
    pub fn map_io_error(e: ApiError) -> ErrorResponse {
        Self::from_api_error(&e)
    }
}
