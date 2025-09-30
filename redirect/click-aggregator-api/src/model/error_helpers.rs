//! Error handling helpers and utilities for the Click Aggregator API
//! 
//! This module provides utilities for converting common database and service errors
//! into our structured ApiError types.

use anyhow::Result;
use aws_sdk_dynamodb::Error as DynamoError;
use aws_sdk_dynamodb::error::SdkError;

use crate::model::error::{ApiError, DatabaseError, ExternalServiceError};

/// MongoDB error conversion utilities
pub mod mongodb {
    use super::*;
    
    /// Convert MongoDB errors to ApiError
    pub fn convert_mongo_error(err: anyhow::Error) -> ApiError {
        let error_msg = err.to_string().to_lowercase();
        
        if error_msg.contains("connection") || error_msg.contains("pool") {
            ApiError::Database(DatabaseError::ConnectionFailed(err.to_string()))
        } else if error_msg.contains("timeout") {
            ApiError::Database(DatabaseError::Timeout(err.to_string()))
        } else if error_msg.contains("duplicate") || error_msg.contains("conflict") {
            ApiError::Database(DatabaseError::DuplicateKey(err.to_string()))
        } else if error_msg.contains("invalid") || error_msg.contains("argument") {
            ApiError::Validation(crate::model::error::ValidationError::InvalidInput {
                field: "query".to_string(),
                message: err.to_string(),
            })
        } else {
            ApiError::ExternalService(ExternalServiceError::MongoDB(err.to_string()))
        }
    }
    
    /// Helper function to wrap MongoDB operations with proper error handling
    pub async fn handle_mongo_result<T, F, Fut>(operation: F) -> Result<T, ApiError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
    {
        operation().await.map_err(convert_mongo_error)
    }
}

/// DynamoDB error conversion utilities
pub mod dynamodb {
    use super::*;
    
    /// Convert DynamoDB errors to ApiError
    pub fn convert_dynamo_error(err: SdkError<DynamoError>) -> ApiError {
        let error_msg = err.to_string().to_lowercase();
        
        if error_msg.contains("resource not found") || error_msg.contains("table not found") {
            ApiError::Database(DatabaseError::TableNotFound("DynamoDB table not found".to_string()))
        } else if error_msg.contains("conditional check") || error_msg.contains("duplicate") {
            ApiError::Database(DatabaseError::DuplicateKey("Conditional check failed".to_string()))
        } else if error_msg.contains("throttle") || error_msg.contains("provisioned throughput") {
            ApiError::ExternalService(ExternalServiceError::RateLimited("DynamoDB throttled".to_string()))
        } else if error_msg.contains("timeout") {
            ApiError::Database(DatabaseError::Timeout("DynamoDB timeout".to_string()))
        } else if error_msg.contains("connection") || error_msg.contains("dispatch") {
            ApiError::Database(DatabaseError::ConnectionFailed("DynamoDB connection failed".to_string()))
        } else {
            ApiError::ExternalService(ExternalServiceError::DynamoDB(err.to_string()))
        }
    }
    
    /// Helper function to wrap DynamoDB operations with proper error handling
    pub async fn handle_dynamo_result<T, F, Fut>(operation: F) -> Result<T, ApiError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, SdkError<DynamoError>>>,
    {
        operation().await.map_err(convert_dynamo_error)
    }
}

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
