//! OpenAPI schemas for the Click Router API
//! 
//! This module provides comprehensive OpenAPI schema definitions for all data models.

use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// Health check response schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Current timestamp
    pub timestamp: String,
    /// API version
    pub version: String,
}

/// Metrics response schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct MetricsResponse {
    /// Total number of requests
    pub requests_total: u64,
    /// Total number of errors
    pub errors_total: u64,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
}

/// Error response schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Error message
    pub error: String,
    /// Additional error details
    pub details: Option<String>,
}

/// Create route response schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateRouteResponse {
    /// Success message
    pub message: String,
    /// Created route information
    pub route: serde_json::Value,
}

/// Simplified Route schema for OpenAPI documentation
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RouteSchema {
    /// Route switch identifier
    pub switch: String,
    /// Route link URL
    pub link: String,
    /// Destination URL (optional)
    pub dest: Option<String>,
    /// HTTP status code (optional)
    pub code: Option<u16>,
    /// Time to live in seconds (optional)
    pub ttl: Option<u128>,
}

/// Authentication error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum AuthenticationErrorType {
    /// Invalid API key
    InvalidApiKey,
    /// Missing authentication token
    MissingToken,
    /// Expired token
    ExpiredToken,
    /// Insufficient permissions
    InsufficientPermissions(String),
    /// User not found
    UserNotFound(String),
    /// Account blocked
    AccountBlocked(String),
}

/// Database error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum DatabaseErrorType {
    /// Connection failed
    ConnectionFailed(String),
    /// Query failed
    QueryFailed(String),
    /// Transaction failed
    TransactionFailed(String),
    /// Serialization failed
    SerializationFailed(String),
    /// Deserialization failed
    DeserializationFailed(String),
    /// Table not found
    TableNotFound(String),
    /// Duplicate key
    DuplicateKey(String),
    /// Timeout
    Timeout(String),
}

/// Validation error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum ValidationErrorType {
    /// Invalid input
    InvalidInput { field: String, message: String },
    /// Missing field
    MissingField(String),
    /// Invalid format
    InvalidFormat { field: String, expected: String },
    /// Out of range
    OutOfRange { field: String, value: String, min: String, max: String },
    /// Invalid URL
    InvalidUrl(String),
    /// Invalid domain
    InvalidDomain(String),
}

/// Route error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum RouteErrorType {
    /// Route not found
    NotFound { switch: String, domain: String, path: String },
    /// Route blocked
    Blocked { reason: String },
    /// Route expired
    Expired { expires_at: String },
    /// Invalid policy
    InvalidPolicy(String),
    /// Creation failed
    CreationFailed(String),
    /// Update failed
    UpdateFailed(String),
    /// Deletion failed
    DeletionFailed(String),
}

/// External service error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum ExternalServiceErrorType {
    /// AWS service error
    Aws(String),
    /// MongoDB error
    MongoDB(String),
    /// DynamoDB error
    DynamoDB(String),
    /// Rate limited
    RateLimited(String),
    /// Service unavailable
    ServiceUnavailable(String),
}

/// Configuration error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum ConfigurationErrorType {
    /// Missing configuration
    MissingConfiguration(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Configuration file not found
    FileNotFound(String),
    /// Configuration parsing error
    ParsingError(String),
}

/// Internal error types
#[derive(Serialize, Deserialize, ToSchema)]
pub enum InternalErrorType {
    /// Serialization error
    Serialization(String),
    /// Deserialization error
    Deserialization(String),
    /// Database connection error
    DatabaseConnection(String),
    /// Cache error
    Cache(String),
    /// Unknown error
    Unknown(String),
}

/// API error schema
#[derive(Serialize, Deserialize, ToSchema)]
pub enum ApiErrorSchema {
    /// Database error
    Database(DatabaseErrorType),
    /// Authentication error
    Authentication(AuthenticationErrorType),
    /// Validation error
    Validation(ValidationErrorType),
    /// Route error
    Route(RouteErrorType),
    /// Configuration error
    Configuration(ConfigurationErrorType),
    /// External service error
    ExternalService(ExternalServiceErrorType),
    /// Internal error
    Internal(InternalErrorType),
}

/// JWT authentication context schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct JwtAuthContextSchema {
    /// User ID
    pub user_id: String,
    /// Username
    pub username: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Full name
    pub name: Option<String>,
    /// Realm roles
    pub realm_roles: Vec<String>,
    /// Resource roles
    pub resource_roles: std::collections::HashMap<String, Vec<String>>,
    /// OAuth scopes
    pub scope: Option<String>,
    /// Authentication status
    pub is_authenticated: bool,
    /// Token type
    pub token_type: String,
}

/// JWT claims schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct JwtClaimsSchema {
    /// Subject (user ID)
    pub sub: String,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Expiration time
    pub exp: i64,
    /// Issued at
    pub iat: i64,
    /// Realm access
    pub realm_access: Option<RealmAccessSchema>,
    /// Resource access
    pub resource_access: Option<std::collections::HashMap<String, ResourceAccessSchema>>,
    /// Preferred username
    pub preferred_username: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Full name
    pub name: Option<String>,
    /// OAuth scopes
    pub scope: Option<String>,
}

/// Realm access schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RealmAccessSchema {
    /// Realm roles
    pub roles: Vec<String>,
}

/// Resource access schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResourceAccessSchema {
    /// Resource roles
    pub roles: Vec<String>,
}

/// Keycloak configuration schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct KeycloakConfigSchema {
    /// Keycloak base URL
    pub keycloak_base_url: String,
    /// Realm name
    pub realm: String,
    /// Client ID
    pub client_id: String,
    /// Client secret (optional)
    pub client_secret: Option<String>,
    /// Audience (optional)
    pub audience: Option<String>,
    /// Issuer
    pub issuer: String,
    /// JWKS endpoint
    pub jwks_endpoint: String,
    /// Introspection endpoint
    pub introspection_endpoint: String,
    /// Token endpoint
    pub token_endpoint: String,
    /// User info endpoint
    pub userinfo_endpoint: String,
}

/// Permission mapper schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct PermissionMapperSchema {
    /// Role mappings
    pub role_mappings: std::collections::HashMap<String, Vec<String>>,
    /// Scope mappings
    pub scope_mappings: std::collections::HashMap<String, Vec<String>>,
}

/// Token validation configuration schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokenValidationConfigSchema {
    /// Validate issuer
    pub validate_issuer: bool,
    /// Validate audience
    pub validate_audience: bool,
    /// Validate expiration
    pub validate_expiration: bool,
    /// Clock skew tolerance in seconds
    pub clock_skew_seconds: u64,
    /// Require scope
    pub require_scope: bool,
    /// Allowed algorithms
    pub allowed_algorithms: Vec<String>,
}

/// RPT configuration schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RptConfigSchema {
    /// RPT enabled
    pub enabled: bool,
    /// Introspection timeout in seconds
    pub introspection_timeout_seconds: u64,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Require UMA scope
    pub require_uma_scope: bool,
}

/// Rate limit information schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RateLimitInfoSchema {
    /// Rate limit per minute
    pub limit: u32,
    /// Remaining requests
    pub remaining: u32,
    /// Reset time
    pub reset_time: u64,
}

/// Security headers schema
#[derive(Serialize, Deserialize, ToSchema)]
pub struct SecurityHeadersSchema {
    /// Content type options
    pub x_content_type_options: String,
    /// Frame options
    pub x_frame_options: String,
    /// XSS protection
    pub x_xss_protection: String,
    /// Strict transport security
    pub strict_transport_security: String,
    /// Content security policy
    pub content_security_policy: String,
}
