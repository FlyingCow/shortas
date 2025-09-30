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
    #[error("AWS service error: {0}")]
    Aws(String),
    
    #[error("MongoDB service error: {0}")]
    MongoDB(String),
    
    #[error("DynamoDB service error: {0}")]
    DynamoDB(String),
    
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
            ExternalServiceError::Aws(_) => 502,
            ExternalServiceError::MongoDB(_) => 502,
            ExternalServiceError::DynamoDB(_) => 502,
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
    pub fn external_service_error(service: String, message: String) -> Self {
        match service.to_lowercase().as_str() {
            "mongodb" => ApiError::ExternalService(ExternalServiceError::MongoDB(message)),
            "dynamodb" => ApiError::ExternalService(ExternalServiceError::DynamoDB(message)),
            "aws" => ApiError::ExternalService(ExternalServiceError::Aws(message)),
            _ => ApiError::ExternalService(ExternalServiceError::Unavailable(message)),
        }
    }
}

/// Conversion from anyhow::Error to ApiError
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        // Try to downcast to specific error types first
        if let Some(api_error) = err.downcast_ref::<ApiError>() {
            return api_error.clone();
        }
        
        // Check for common database errors
        let error_msg = err.to_string().to_lowercase();
        if error_msg.contains("mongodb") {
            return ApiError::external_service_error("mongodb".to_string(), err.to_string());
        }
        if error_msg.contains("dynamodb") {
            return ApiError::external_service_error("dynamodb".to_string(), err.to_string());
        }
        if error_msg.contains("aws") {
            return ApiError::external_service_error("aws".to_string(), err.to_string());
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