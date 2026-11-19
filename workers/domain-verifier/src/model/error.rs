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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_error_not_found_display() {
        let err = DomainError::NotFound("d123".into());
        assert_eq!(err.to_string(), "Domain not found: d123");
    }

    #[test]
    fn test_domain_error_already_exists_display() {
        let err = DomainError::AlreadyExists("example.com".into());
        assert_eq!(err.to_string(), "Domain already exists: example.com");
    }

    #[test]
    fn test_domain_error_invalid_name_display() {
        let err = DomainError::InvalidName("bad!domain".into());
        assert_eq!(err.to_string(), "Invalid domain name: bad!domain");
    }

    #[test]
    fn test_domain_error_database_display() {
        let err = DomainError::Database("connection refused".into());
        assert_eq!(err.to_string(), "Database error: connection refused");
    }

    #[test]
    fn test_domain_error_dns_resolution_display() {
        let err = DomainError::DnsResolution("timeout".into());
        assert_eq!(err.to_string(), "DNS resolution error: timeout");
    }

    #[test]
    fn test_domain_error_internal_display() {
        let err = DomainError::Internal("unexpected".into());
        assert_eq!(err.to_string(), "Internal error: unexpected");
    }

    #[test]
    fn test_api_error_from_domain_error() {
        let domain_err = DomainError::NotFound("d1".into());
        let api_err = ApiError::Domain(domain_err);
        assert_eq!(api_err.to_string(), "Domain error: Domain not found: d1");
    }

    #[test]
    fn test_api_error_validation_display() {
        let err = ApiError::Validation {
            field: "name".into(),
            message: "cannot be empty".into(),
        };
        assert_eq!(err.to_string(), "Validation error: name - cannot be empty");
    }

    #[test]
    fn test_api_error_internal_display() {
        let err = ApiError::Internal;
        assert_eq!(err.to_string(), "Internal server error");
    }

    #[test]
    fn test_api_error_from_domain_error_conversion() {
        let domain_err = DomainError::AlreadyExists("test.com".into());
        let api_err: ApiError = domain_err.into();
        assert!(matches!(api_err, ApiError::Domain(DomainError::AlreadyExists(_))));
    }
}
