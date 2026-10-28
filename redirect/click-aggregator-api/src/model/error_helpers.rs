//! Error handling helpers and utilities for the Click Aggregator API
//! 
//! This module provides utilities for converting common database and service errors
//! into our structured ApiError types.


/// Validation error helpers
pub mod validation {
    use crate::model::error::{ApiError, ValidationError};
    
    /// Validate that a string is not empty
    pub fn validate_not_empty(field: &str, value: &str) -> Result<(), ApiError> {
        if value.trim().is_empty() {
            Err(ApiError::Validation(ValidationError::MissingField(field.to_string())))
        } else {
            Ok(())
        }
    }
    
    /// Validate URL format
    pub fn validate_url(_field: &str, url: &str) -> Result<(), ApiError> {
        // Simple URL validation - could be enhanced with a proper URL crate
        if url.is_empty() || !url.starts_with("http") {
            Err(ApiError::Validation(ValidationError::InvalidUrl(url.to_string())))
        } else {
            Ok(())
        }
    }
    
    /// Validate domain format
    pub fn validate_domain(_field: &str, domain: &str) -> Result<(), ApiError> {
        if domain.is_empty() || !domain.contains('.') {
            Err(ApiError::Validation(ValidationError::InvalidDomain(domain.to_string())))
        } else {
            Ok(())
        }
    }
}

/// Route-specific error helpers
pub mod route {
    use crate::model::error::{ApiError, RouteError};
    
    /// Create a route not found error with proper context
    pub fn route_not_found(switch: &str, domain: &str, path: &str) -> ApiError {
        ApiError::Route(RouteError::NotFound {
            switch: switch.to_string(),
            domain: domain.to_string(),
            path: path.to_string(),
        })
    }
    
    /// Create a route blocked error
    pub fn route_blocked(reason: &str) -> ApiError {
        ApiError::Route(RouteError::Blocked {
            reason: reason.to_string(),
        })
    }
    
    /// Create a route expired error
    pub fn route_expired(expires_at: &str) -> ApiError {
        ApiError::Route(RouteError::Expired {
            expires_at: expires_at.to_string(),
        })
    }
}

/// Authentication error helpers
pub mod auth {
    use crate::model::error::{ApiError, AuthenticationError};
    
    /// Create an invalid API key error
    pub fn invalid_api_key() -> ApiError {
        ApiError::Authentication(AuthenticationError::InvalidApiKey)
    }
    
    /// Create a user not found error
    pub fn user_not_found(user_id: &str) -> ApiError {
        ApiError::Authentication(AuthenticationError::UserNotFound(user_id.to_string()))
    }
    
    /// Create an account blocked error
    pub fn account_blocked(reason: &str) -> ApiError {
        ApiError::Authentication(AuthenticationError::AccountBlocked(reason.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::error::{ApiError, AuthenticationError, RouteError};

    #[test]
    fn test_validate_not_empty() {
        assert!(validation::validate_not_empty("name", "shortas").is_ok());
        assert!(validation::validate_not_empty("name", "   ").is_err());
        assert!(validation::validate_not_empty("name", "").is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validation::validate_url("dest", "https://example.com").is_ok());
        assert!(validation::validate_url("dest", "ftp://example.com").is_err());
        assert!(validation::validate_url("dest", "").is_err());
    }

    #[test]
    fn test_validate_domain() {
        assert!(validation::validate_domain("domain", "example.com").is_ok());
        assert!(validation::validate_domain("domain", "localhost").is_err());
        assert!(validation::validate_domain("domain", "").is_err());
    }

    #[test]
    fn test_route_error_helpers() {
        assert!(matches!(
            route::route_not_found("abc", "example.com", "/abc"),
            ApiError::Route(RouteError::NotFound { .. })
        ));
        assert!(matches!(
            route::route_blocked("spam"),
            ApiError::Route(RouteError::Blocked { .. })
        ));
        assert!(matches!(
            route::route_expired("2024-01-01"),
            ApiError::Route(RouteError::Expired { .. })
        ));
    }

    #[test]
    fn test_auth_error_helpers() {
        assert!(matches!(
            auth::invalid_api_key(),
            ApiError::Authentication(AuthenticationError::InvalidApiKey)
        ));
        assert!(matches!(
            auth::user_not_found("user-1"),
            ApiError::Authentication(AuthenticationError::UserNotFound(ref id)) if id == "user-1"
        ));
        assert!(matches!(
            auth::account_blocked("fraud"),
            ApiError::Authentication(AuthenticationError::AccountBlocked(ref reason)) if reason == "fraud"
        ));
    }

}
