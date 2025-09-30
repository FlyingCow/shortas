# Error Handling Guide

This document explains the comprehensive error handling system implemented in the Click Router API.

## Overview

The API now uses a structured error type hierarchy that provides:
- **Specific error types** for different failure modes
- **Proper HTTP status codes** for each error type
- **Detailed error messages** with context
- **Easy error conversion** from database and service errors

## Error Type Hierarchy

### Main Error Types

```rust
pub enum ApiError {
    Database(DatabaseError),
    Authentication(AuthenticationError),
    Validation(ValidationError),
    Route(RouteError),
    Configuration(ConfigurationError),
    ExternalService(ExternalServiceError),
    Internal(InternalError),
}
```

### Database Errors

```rust
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
    TransactionFailed(String),
    SerializationFailed(String),
    DeserializationFailed(String),
    TableNotFound(String),
    DuplicateKey(String),
    Timeout(String),
}
```

### Authentication Errors

```rust
pub enum AuthenticationError {
    InvalidApiKey,
    MissingToken,
    ExpiredToken,
    InsufficientPermissions(String),
    UserNotFound(String),
    AccountBlocked(String),
}
```

### Validation Errors

```rust
pub enum ValidationError {
    InvalidInput { field: String, message: String },
    MissingField(String),
    InvalidFormat { field: String, expected: String },
    OutOfRange { field: String, value: String, min: String, max: String },
    InvalidUrl(String),
    InvalidDomain(String),
}
```

### Route Errors

```rust
pub enum RouteError {
    NotFound { switch: String, domain: String, path: String },
    Blocked { reason: String },
    Expired { expires_at: String },
    InvalidPolicy(String),
    CreationFailed(String),
    UpdateFailed(String),
    DeletionFailed(String),
}
```

## Usage Examples

### 1. Creating Specific Errors

```rust
use crate::model::error::{ApiError, RouteError, AuthenticationError};

// Create a route not found error
let error = ApiError::route_not_found("main".to_string(), "example.com".to_string(), "path".to_string());

// Create a user not found error
let error = ApiError::user_not_found("user123".to_string());

// Create a validation error
let error = ApiError::validation_error("email".to_string(), "Invalid email format".to_string());
```

### 2. Using Error Helpers

```rust
use crate::model::error_helpers::{mongodb, dynamodb, validation, route, auth};

// MongoDB error handling
let result = mongodb::handle_mongo_result(|| async {
    collection.find_one(filter).await
}).await?;

// DynamoDB error handling
let result = dynamodb::handle_dynamo_result(|| async {
    client.get_item().send().await
}).await?;

// Validation
validation::validate_not_empty("domain", &domain)?;
validation::validate_url("destination", &url)?;

// Route errors
return Err(route::route_not_found("main", "example.com", "path"));

// Authentication errors
return Err(auth::invalid_api_key());
```

### 3. Error Conversion

```rust
use crate::model::error::IntoApiError;

// Convert any Result<T, E> to Result<T, ApiError>
let result: Result<String, ApiError> = some_operation()
    .into_api_error();
```

### 4. Controller Error Handling

```rust
#[endpoint]
async fn get_route(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let path = req.param::<String>("path").unwrap_or_default();
    let switch = req.param::<String>("switch").unwrap_or_default();

    let app_state = depot.get::<AppState>("app_state").unwrap();

    let route = app_state
        .routes_store
        .get_route(switch.as_str(), domain.as_str(), path.as_str())
        .await;

    match route {
        Ok(Some(route)) => {
            res.render(Json(route));
        }
        Ok(None) => {
            // Use specific error type for not found
            let error_response = ErrorResponse::from_api_error(&ApiError::Route(
                RouteError::NotFound {
                    switch: switch.clone(),
                    domain: domain.clone(),
                    path: path.clone(),
                }
            ));
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
        Err(e) => {
            // Convert anyhow::Error to proper error response
            let error_response = ErrorResponse::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
```

## HTTP Status Code Mapping

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| `DatabaseError::ConnectionFailed` | 503 | Service Unavailable |
| `DatabaseError::QueryFailed` | 500 | Internal Server Error |
| `DatabaseError::DuplicateKey` | 409 | Conflict |
| `DatabaseError::Timeout` | 504 | Gateway Timeout |
| `AuthenticationError::InvalidApiKey` | 401 | Unauthorized |
| `AuthenticationError::UserNotFound` | 404 | Not Found |
| `AuthenticationError::AccountBlocked` | 403 | Forbidden |
| `ValidationError::*` | 400 | Bad Request |
| `RouteError::NotFound` | 404 | Not Found |
| `RouteError::Blocked` | 403 | Forbidden |
| `RouteError::Expired` | 410 | Gone |
| `ExternalServiceError::MongoDB` | 502 | Bad Gateway |
| `ExternalServiceError::DynamoDB` | 502 | Bad Gateway |
| `ExternalServiceError::RateLimited` | 429 | Too Many Requests |

## Error Response Format

All errors are returned in a consistent JSON format:

```json
{
  "code": 404,
  "error": "Route not found: main/example.com/path",
  "message": "Not Found",
  "details": "RouteError::NotFound { switch: \"main\", domain: \"example.com\", path: \"path\" }"
}
```

## Best Practices

1. **Use specific error types** instead of generic errors
2. **Provide context** in error messages (domain, path, user_id, etc.)
3. **Use error helpers** for common error creation patterns
4. **Convert database errors** using the provided helper functions
5. **Handle errors at the controller level** with proper HTTP status codes
6. **Log detailed errors** for debugging while returning user-friendly messages

## Migration from Legacy Error Handling

The old `ErrorReponse` struct has been renamed to `ErrorResponse` (fixed typo) and enhanced with:
- Better error classification
- Detailed error information
- Proper HTTP status code mapping
- Support for error context

Legacy code can be updated by:
1. Replacing `ErrorReponse` with `ErrorResponse`
2. Using `ErrorResponse::from_api_error()` for specific error types
3. Using `ErrorResponse::map_error()` for anyhow::Error conversion
