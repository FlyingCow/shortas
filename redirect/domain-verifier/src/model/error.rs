use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Domain not found: {0}")]
    NotFound(String),

    #[error("Domain already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid domain name: {0}")]
    InvalidName(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("DNS resolution error: {0}")]
    DnsResolution(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("Internal server error")]
    Internal,
}
