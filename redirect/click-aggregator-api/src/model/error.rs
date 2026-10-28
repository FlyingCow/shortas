use std::{error::Error, fmt};
use thiserror::Error;
use anyhow::Result;

/// Comprehensive error type hierarchy for the Click Aggregator API
#[derive(Error, Debug, Clone)]
pub enum ApiError {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    
    /// Authentication and authorization errors
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
    
    /// Validation errors for input data
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    /// Route-specific errors
    #[error("Route error: {0}")]
    Route(#[from] RouteError),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigurationError),
    
    /// External service errors (AWS, MongoDB, etc.)
    #[error("External service error: {0}")]
    ExternalService(#[from] ExternalServiceError),
    
    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(#[from] InternalError),
}

impl ApiError {
    pub fn get_error_message(&self) -> String {
        self.to_string()
    }

    pub fn get_error_code(&self) -> u16 {
        match self {
            ApiError::Database(e) => e.get_error_code(),
            ApiError::Authentication(e) => e.get_error_code(),
            ApiError::Validation(e) => e.get_error_code(),
            ApiError::Route(e) => e.get_error_code(),
            ApiError::Configuration(e) => e.get_error_code(),
            ApiError::ExternalService(e) => e.get_error_code(),
            ApiError::Internal(e) => e.get_error_code(),
        }
    }
}

/// Database-specific errors
#[derive(Error, Debug, Clone)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Query failed: {0}")]
    QueryFailed(String),
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Data serialization failed: {0}")]
    SerializationFailed(String),
    
    #[error("Data deserialization failed: {0}")]
    DeserializationFailed(String),
    
    #[error("Table/Collection not found: {0}")]
    TableNotFound(String),
    
    #[error("Duplicate key error: {0}")]
    DuplicateKey(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl DatabaseError {
    fn get_error_code(&self) -> u16 {
        match self {
            DatabaseError::ConnectionFailed(_) => 503,
            DatabaseError::QueryFailed(_) => 500,
            DatabaseError::TransactionFailed(_) => 500,
            DatabaseError::SerializationFailed(_) => 500,
            DatabaseError::DeserializationFailed(_) => 500,
            DatabaseError::TableNotFound(_) => 500,
            DatabaseError::DuplicateKey(_) => 409,
            DatabaseError::Timeout(_) => 504,
        }
    }
}

/// Authentication and authorization errors
#[derive(Error, Debug, Clone)]
pub enum AuthenticationError {
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Missing authentication token")]
    MissingToken,
    
    #[error("Expired token")]
    ExpiredToken,
    
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),
    
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Account blocked: {0}")]
    AccountBlocked(String),
}

impl AuthenticationError {
    fn get_error_code(&self) -> u16 {
        match self {
            AuthenticationError::InvalidApiKey => 401,
            AuthenticationError::MissingToken => 401,
            AuthenticationError::ExpiredToken => 401,
            AuthenticationError::InsufficientPermissions(_) => 403,
            AuthenticationError::UserNotFound(_) => 404,
            AuthenticationError::AccountBlocked(_) => 403,
        }
    }
}

/// Input validation errors
#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Invalid input: {field} - {message}")]
    InvalidInput { field: String, message: String },
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid format: {field} - expected {expected}")]
    InvalidFormat { field: String, expected: String },
    
    #[error("Value out of range: {field} - {value} is not between {min} and {max}")]
    OutOfRange { field: String, value: String, min: String, max: String },
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Invalid domain: {0}")]
    InvalidDomain(String),
}

impl ValidationError {
    fn get_error_code(&self) -> u16 {
        400
    }
}

/// Route-specific errors
#[derive(Error, Debug, Clone)]
pub enum RouteError {
    #[error("Route not found: {switch}/{domain}/{path}")]
    NotFound { switch: String, domain: String, path: String },
    
    #[error("Route blocked: {reason}")]
    Blocked { reason: String },
    
    #[error("Route expired: {expires_at}")]
    Expired { expires_at: String },
    
    #[error("Invalid routing policy: {0}")]
    InvalidPolicy(String),
    
    #[error("Route creation failed: {0}")]
    CreationFailed(String),
    
    #[error("Route update failed: {0}")]
    UpdateFailed(String),
    
    #[error("Route deletion failed: {0}")]
    DeletionFailed(String),
}

impl RouteError {
    fn get_error_code(&self) -> u16 {
        match self {
            RouteError::NotFound { .. } => 404,
            RouteError::Blocked { .. } => 403,
            RouteError::Expired { .. } => 410,
            RouteError::InvalidPolicy(_) => 400,
            RouteError::CreationFailed(_) => 500,
            RouteError::UpdateFailed(_) => 500,
            RouteError::DeletionFailed(_) => 500,
        }
    }
}

/// Configuration errors
#[derive(Error, Debug, Clone)]
pub enum ConfigurationError {
    #[error("Missing configuration: {0}")]
    Missing(String),
    
    #[error("Invalid configuration: {0}")]
    Invalid(String),
    
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),
    
    #[error("Environment variable not set: {0}")]
    MissingEnvVar(String),
}

impl ConfigurationError {
    fn get_error_code(&self) -> u16 {
        500
    }
}

/// External service errors
#[derive(Error, Debug, Clone)]
pub enum ExternalServiceError {
    #[error("Service unavailable: {0}")]
    Unavailable(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimited(String),
    
    #[error("Service timeout: {0}")]
    Timeout(String),
}

impl ExternalServiceError {
    fn get_error_code(&self) -> u16 {
        match self {
            ExternalServiceError::Unavailable(_) => 503,
            ExternalServiceError::RateLimited(_) => 429,
            ExternalServiceError::Timeout(_) => 504,
        }
    }
}

/// Internal server errors
#[derive(Error, Debug, Clone)]
pub enum InternalError {
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("Memory allocation failed")]
    MemoryAllocation,
    
    #[error("Thread pool exhausted")]
    ThreadPoolExhausted,
    
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl InternalError {
    fn get_error_code(&self) -> u16 {
        500
    }
}

// Legacy ApiError struct for backward compatibility
#[derive(Debug)]
pub struct LegacyApiError {
    pub code: u16,
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

impl fmt::Display for LegacyApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!")
    }
}

impl LegacyApiError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }

    pub fn get_error_code(&self) -> u16 {
        self.code
    }
}

/// Helper functions for creating specific error types
impl ApiError {
    /// Create a database connection error
    pub fn database_connection_failed(message: String) -> Self {
        ApiError::Database(DatabaseError::ConnectionFailed(message))
    }
    
    /// Create a database query error
    pub fn database_query_failed(message: String) -> Self {
        ApiError::Database(DatabaseError::QueryFailed(message))
    }
    
    /// Create a route not found error
    pub fn route_not_found(switch: String, domain: String, path: String) -> Self {
        ApiError::Route(RouteError::NotFound { switch, domain, path })
    }
    
    /// Create a user not found error
    pub fn user_not_found(user_id: String) -> Self {
        ApiError::Authentication(AuthenticationError::UserNotFound(user_id))
    }
    
    /// Create an invalid API key error
    pub fn invalid_api_key() -> Self {
        ApiError::Authentication(AuthenticationError::InvalidApiKey)
    }
    
    /// Create a validation error
    pub fn validation_error(field: String, message: String) -> Self {
        ApiError::Validation(ValidationError::InvalidInput { field, message })
    }
    
    /// Create an external service error
    pub fn external_service_error(_service: String, message: String) -> Self {
        ApiError::ExternalService(ExternalServiceError::Unavailable(message))
    }
}

/// Conversion from anyhow::Error to ApiError
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        // Try to downcast to specific error types first
        if let Some(api_error) = err.downcast_ref::<ApiError>() {
            return api_error.clone();
        }
        
        
        // Default to internal error
        ApiError::Internal(InternalError::Unexpected(err.to_string()))
    }
}

/// Helper trait for converting Result<T, E> to Result<T, ApiError>
pub trait IntoApiError<T> {
    fn into_api_error(self) -> Result<T, ApiError>;
}

impl<T, E> IntoApiError<T> for Result<T, E>
where
    E: Into<ApiError>
{
    fn into_api_error(self) -> Result<T, ApiError> {
        self.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_error_codes() {
        assert_eq!(DatabaseError::ConnectionFailed("x".into()).get_error_code(), 503);
        assert_eq!(DatabaseError::QueryFailed("x".into()).get_error_code(), 500);
        assert_eq!(DatabaseError::DuplicateKey("x".into()).get_error_code(), 409);
        assert_eq!(DatabaseError::Timeout("x".into()).get_error_code(), 504);
    }

    #[test]
    fn test_authentication_error_codes() {
        assert_eq!(AuthenticationError::InvalidApiKey.get_error_code(), 401);
        assert_eq!(AuthenticationError::InsufficientPermissions("x".into()).get_error_code(), 403);
        assert_eq!(AuthenticationError::UserNotFound("x".into()).get_error_code(), 404);
    }

    #[test]
    fn test_route_error_codes() {
        let not_found = RouteError::NotFound {
            switch: "s".into(),
            domain: "d".into(),
            path: "p".into(),
        };
        assert_eq!(not_found.get_error_code(), 404);
        assert_eq!(RouteError::Blocked { reason: "spam".into() }.get_error_code(), 403);
        assert_eq!(RouteError::Expired { expires_at: "2024".into() }.get_error_code(), 410);
    }

    #[test]
    fn test_external_service_error_codes() {
        assert_eq!(ExternalServiceError::Unavailable("x".into()).get_error_code(), 503);
        assert_eq!(ExternalServiceError::RateLimited("x".into()).get_error_code(), 429);
        assert_eq!(ExternalServiceError::Timeout("x".into()).get_error_code(), 504);
    }

    #[test]
    fn test_validation_configuration_internal_error_codes() {
        assert_eq!(ValidationError::MissingField("x".into()).get_error_code(), 400);
        assert_eq!(ConfigurationError::Missing("x".into()).get_error_code(), 500);
        assert_eq!(InternalError::MemoryAllocation.get_error_code(), 500);
    }

    #[test]
    fn test_api_error_get_error_code_dispatches_to_variant() {
        assert_eq!(
            ApiError::Database(DatabaseError::Timeout("x".into())).get_error_code(),
            504
        );
        assert_eq!(
            ApiError::Route(RouteError::Blocked { reason: "x".into() }).get_error_code(),
            403
        );
        assert_eq!(
            ApiError::ExternalService(ExternalServiceError::RateLimited("x".into())).get_error_code(),
            429
        );
    }

    #[test]
    fn test_api_error_get_error_message_uses_display() {
        let err = ApiError::Database(DatabaseError::ConnectionFailed("pool exhausted".into()));
        assert_eq!(err.get_error_message(), "Database error: Connection failed: pool exhausted");
    }

    #[test]
    fn test_helper_constructors() {
        assert!(matches!(
            ApiError::database_connection_failed("x".into()),
            ApiError::Database(DatabaseError::ConnectionFailed(_))
        ));
        assert!(matches!(
            ApiError::database_query_failed("x".into()),
            ApiError::Database(DatabaseError::QueryFailed(_))
        ));
        assert!(matches!(
            ApiError::route_not_found("s".into(), "d".into(), "p".into()),
            ApiError::Route(RouteError::NotFound { .. })
        ));
        assert!(matches!(
            ApiError::user_not_found("u1".into()),
            ApiError::Authentication(AuthenticationError::UserNotFound(ref id)) if id == "u1"
        ));
        assert!(matches!(
            ApiError::invalid_api_key(),
            ApiError::Authentication(AuthenticationError::InvalidApiKey)
        ));
        assert!(matches!(
            ApiError::validation_error("field".into(), "bad".into()),
            ApiError::Validation(ValidationError::InvalidInput { .. })
        ));
        assert!(matches!(
            ApiError::external_service_error("s3".into(), "down".into()),
            ApiError::ExternalService(ExternalServiceError::Unavailable(_))
        ));
    }

    #[test]
    fn test_from_anyhow_error_downcasts_existing_api_error() {
        let original = ApiError::invalid_api_key();
        let wrapped: anyhow::Error = anyhow::Error::new(original);
        let converted: ApiError = wrapped.into();
        assert!(matches!(
            converted,
            ApiError::Authentication(AuthenticationError::InvalidApiKey)
        ));
    }

    #[test]
    fn test_from_anyhow_error_falls_back_to_internal() {
        let wrapped = anyhow::anyhow!("boom");
        let converted: ApiError = wrapped.into();
        assert!(matches!(converted, ApiError::Internal(InternalError::Unexpected(ref msg)) if msg == "boom"));
    }

    #[test]
    fn test_into_api_error_trait_maps_err_variant() {
        let result: Result<(), anyhow::Error> = Err(anyhow::anyhow!("failure"));
        let converted = result.into_api_error();
        assert!(converted.is_err());
        assert!(matches!(converted.unwrap_err(), ApiError::Internal(InternalError::Unexpected(_))));
    }

    #[test]
    fn test_legacy_api_error_display_is_generic() {
        let err = LegacyApiError {
            code: 500,
            message: "boom".to_string(),
            error: None,
        };
        assert_eq!(err.to_string(), "An Error Occurred, Please Try Again!");
        assert_eq!(err.get_error_message(), "boom");
        assert_eq!(err.get_error_code(), 500);
    }
}