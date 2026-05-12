//! Error types for the management API.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error codes that map to specific HTTP status codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ErrorCode {
    /// 400 Bad Request - Invalid input or validation failure.
    ValidationError,
    /// 401 Unauthorized - Authentication required.
    Unauthorized,
    /// 403 Forbidden - Insufficient permissions.
    Forbidden,
    /// 404 Not Found - Resource doesn't exist.
    NotFound,
    /// 409 Conflict - Resource already exists or state conflict.
    Conflict,
    /// 422 Unprocessable Entity - Semantic validation failure.
    UnprocessableEntity,
    /// 429 Too Many Requests - Rate limit exceeded.
    RateLimitExceeded,
    /// 500 Internal Server Error - Unexpected error.
    InternalError,
    /// 502 Bad Gateway - External service failure.
    ExternalServiceError,
    /// 503 Service Unavailable - Temporary unavailability.
    ServiceUnavailable,
}

impl ErrorCode {
    /// Get the HTTP status code for this error code.
    pub fn status_code(&self) -> u16 {
        match self {
            ErrorCode::ValidationError => 400,
            ErrorCode::Unauthorized => 401,
            ErrorCode::Forbidden => 403,
            ErrorCode::NotFound => 404,
            ErrorCode::Conflict => 409,
            ErrorCode::UnprocessableEntity => 422,
            ErrorCode::RateLimitExceeded => 429,
            ErrorCode::InternalError => 500,
            ErrorCode::ExternalServiceError => 502,
            ErrorCode::ServiceUnavailable => 503,
        }
    }

    /// Get the error code as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::ValidationError => "VALIDATION_ERROR",
            ErrorCode::Unauthorized => "UNAUTHORIZED",
            ErrorCode::Forbidden => "FORBIDDEN",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::Conflict => "CONFLICT",
            ErrorCode::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            ErrorCode::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            ErrorCode::InternalError => "INTERNAL_ERROR",
            ErrorCode::ExternalServiceError => "EXTERNAL_SERVICE_ERROR",
            ErrorCode::ServiceUnavailable => "SERVICE_UNAVAILABLE",
        }
    }
}

/// API error with code and message.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Error)]
#[error("{message}")]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ApiError {
    /// Create a new API error.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    /// Create a new API error with details.
    pub fn with_details(code: ErrorCode, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: Some(details.into()),
        }
    }

    /// Create a validation error.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationError, message)
    }

    /// Create a validation error for a required field.
    pub fn required(field: &str) -> Self {
        Self::validation(format!("Field '{}' is required", field))
    }

    /// Create an unauthorized error.
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message)
    }

    /// Create a forbidden error.
    pub fn forbidden() -> Self {
        Self::new(ErrorCode::Forbidden, "You don't have permission to perform this action")
    }

    /// Create a not found error.
    pub fn not_found(entity: &str, id: &str) -> Self {
        Self::new(ErrorCode::NotFound, format!("{} with id '{}' not found", entity, id))
    }

    /// Create a conflict error.
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Conflict, message)
    }

    /// Create an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message)
    }

    /// Create an external service error.
    pub fn external_service(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ExternalServiceError, message)
    }

    /// Get the HTTP status code.
    pub fn status_code(&self) -> u16 {
        self.code.status_code()
    }
}

/// Result type for API operations.
pub type Result<T> = std::result::Result<T, ApiError>;

/// Extension trait for Option to convert to Result.
pub trait OptionExt<T> {
    /// Convert Option to Result with not found error.
    fn ok_or_not_found(self, entity: &str, id: &str) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_not_found(self, entity: &str, id: &str) -> Result<T> {
        self.ok_or_else(|| ApiError::not_found(entity, id))
    }
}

/// Extension trait for std::result::Result to map errors.
pub trait ResultExt<T, E> {
    /// Map any error to an internal API error.
    fn map_internal_err(self, message: &str) -> Result<T>;

    /// Map any error to an external service error.
    fn map_external_err(self, message: &str) -> Result<T>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for std::result::Result<T, E> {
    fn map_internal_err(self, message: &str) -> Result<T> {
        self.map_err(|e| ApiError::with_details(ErrorCode::InternalError, message, e.to_string()))
    }

    fn map_external_err(self, message: &str) -> Result<T> {
        self.map_err(|e| ApiError::with_details(ErrorCode::ExternalServiceError, message, e.to_string()))
    }
}
