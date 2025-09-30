# UserSettingsDto Documentation

This document provides comprehensive information about the `UserSettingsDto` used in the user settings controller.

## Overview

The `UserSettingsDto` is a Data Transfer Object (DTO) that provides a clean, secure API interface for user settings. It excludes sensitive information like API keys and internal user IDs while exposing only the necessary configuration data.

## DTO Structure

### Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `email` | `String` | User's email address | `"user@example.com"` |
| `status` | `String` | Current active status | `"active"` or `"blocked"` |
| `debug` | `bool` | Debug mode enabled | `true` or `false` |
| `overflow` | `bool` | Overflow handling enabled | `true` or `false` |
| `skip_tracking` | `Vec<String>` | Skip tracking parameters | `["tracking", "analytics"]` |
| `allowed_request_params` | `Vec<String>` | Allowed request parameters | `["utm_source", "utm_medium"]` |
| `allowed_destination_params` | `Vec<String>` | Allowed destination parameters | `["redirect", "target"]` |

### API Response Example

```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking", "analytics"],
  "allowed_request_params": ["utm_source", "utm_medium", "utm_campaign"],
  "allowed_destination_params": ["redirect", "target", "destination"]
}
```

## Security Features

### 1. Sensitive Data Exclusion
- **API Keys**: Not exposed in DTO (security)
- **Internal IDs**: User ID not included in response
- **Internal State**: Only user-facing configuration exposed

### 2. Data Sanitization
- **Email Validation**: Email format validation
- **Status Normalization**: Status values normalized to lowercase
- **Parameter Filtering**: Only allowed parameters exposed

### 3. Access Control
- **JWT Authentication**: Required for all operations
- **User Context**: Settings retrieved based on authenticated user
- **Permission Validation**: User can only access their own settings

## API Endpoints

### GET /v1/user-settings/{user_id}

#### Description
Retrieves user settings for a specific user ID. If no user ID is provided in the URL, the user ID from the JWT token context will be used.

#### Parameters
- **user_id** (path, optional): The user ID for the settings
  - Type: `String`
  - Example: `"user123"`
  - Note: If not provided, uses JWT context

#### Responses

##### 200 OK - Success
```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking"],
  "allowed_request_params": ["utm_source"],
  "allowed_destination_params": ["redirect"]
}
```

##### 404 Not Found - User Not Found
```json
{
  "status_code": 404,
  "error": "Authentication error: User not found",
  "details": "AuthenticationError::UserNotFound(\"user123\")"
}
```

##### 401 Unauthorized - Invalid JWT
```json
{
  "status_code": 401,
  "error": "Authentication error: Invalid JWT token",
  "details": "AuthenticationError::InvalidToken"
}
```

##### 403 Forbidden - Insufficient Permissions
```json
{
  "status_code": 403,
  "error": "Authentication error: Insufficient permissions",
  "details": "AuthenticationError::InsufficientPermissions"
}
```

##### 500 Internal Server Error
```json
{
  "status_code": 500,
  "error": "Internal server error",
  "details": "Database connection failed"
}
```

## Usage Examples

### 1. Get User Settings with User ID
```bash
curl -X GET "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>"
```

### 2. Get User Settings from JWT Context
```bash
curl -X GET "https://api.example.com/v1/user-settings" \
  -H "Authorization: Bearer <jwt_token>"
```

### 3. Response Handling
```javascript
// JavaScript example
fetch('/v1/user-settings/user123', {
  headers: {
    'Authorization': 'Bearer ' + jwtToken
  }
})
.then(response => response.json())
.then(settings => {
  console.log('User email:', settings.email);
  console.log('Debug mode:', settings.debug);
  console.log('Status:', settings.status);
  console.log('Skip tracking:', settings.skip_tracking);
});
```

## DTO Methods

### Constructor Methods
```rust
// Create with all parameters
let dto = UserSettingsDto::new(
    "user@example.com".to_string(),
    "active".to_string(),
    false,
    true,
    vec!["tracking".to_string()],
    vec!["utm_source".to_string()],
    vec!["redirect".to_string()]
);

// Create with default values
let dto = UserSettingsDto::default();
```

### Builder Pattern
```rust
let dto = UserSettingsDto::default()
    .email("user@example.com".to_string())
    .status("active".to_string())
    .debug(false)
    .overflow(true)
    .skip_tracking(vec!["tracking".to_string()])
    .allowed_request_params(vec!["utm_source".to_string()])
    .allowed_destination_params(vec!["redirect".to_string()]);
```

### Validation Methods
```rust
// Check if DTO is valid
if dto.is_valid() {
    println!("DTO is valid");
}

// Get status
let status = dto.get_status(); // "active" or "blocked"

// Check debug mode
if dto.is_debug_enabled() {
    println!("Debug mode is enabled");
}

// Check overflow handling
if dto.is_overflow_enabled() {
    println!("Overflow handling is enabled");
}

// Get parameter counts
println!("Skip tracking parameters: {}", dto.skip_tracking_count());
println!("Allowed request parameters: {}", dto.allowed_request_params_count());
println!("Allowed destination parameters: {}", dto.allowed_destination_params_count());
```

## Conversion Methods

### From UserSettings to UserSettingsDto
```rust
use crate::dto::UserSettingsDto;
use crate::model::user_settings::UserSettings;

// Convert from UserSettings
let user_settings = UserSettings::new(/* ... */);
let dto = UserSettingsDto::from(user_settings);

// Convert from &UserSettings
let dto = UserSettingsDto::from(&user_settings);
```

### From UserSettingsDto to UserSettings
```rust
use crate::dto::UserSettingsDto;

let dto = UserSettingsDto::new(/* ... */);
let user_settings: UserSettings = dto.into();
```

## Status Values

### Active Status
- **"active"**: User account is active and functional
- **"blocked"**: User account is blocked and cannot perform operations

### Status Conversion
```rust
// From UserSettings to DTO
let status = match user_settings.active_status {
    ActiveStatus::Active => "active".to_string(),
    ActiveStatus::Blocked => "blocked".to_string(),
};

// From DTO to UserSettings
let active_status = match dto.status.as_str() {
    "active" => ActiveStatus::Active,
    "blocked" => ActiveStatus::Blocked,
    _ => ActiveStatus::Active, // Default to active
};
```

## Configuration Options

### Debug Mode
- **Purpose**: Enable detailed logging and debugging information
- **Default**: `false`
- **Usage**: Helps with troubleshooting and development

### Overflow Handling
- **Purpose**: Enable overflow protection for high-traffic scenarios
- **Default**: `false`
- **Usage**: Prevents system overload during peak usage

### Skip Tracking Parameters
- **Purpose**: Define parameters to skip during request tracking
- **Default**: `[]` (empty vector)
- **Common Values**: `["tracking", "analytics", "utm_source"]`

### Allowed Request Parameters
- **Purpose**: Define which request parameters are allowed
- **Default**: `[]` (empty vector)
- **Common Values**: `["utm_source", "utm_medium", "utm_campaign"]`

### Allowed Destination Parameters
- **Purpose**: Define which destination parameters are allowed
- **Default**: `[]` (empty vector)
- **Common Values**: `["redirect", "target", "destination"]`

## Error Handling

### Validation Errors
```rust
// Check if DTO is valid before processing
if !dto.is_valid() {
    return Err("Invalid user settings DTO");
}
```

### Conversion Errors
```rust
// Handle conversion errors gracefully
match UserSettingsDto::from(user_settings) {
    Ok(dto) => Ok(dto),
    Err(e) => Err(format!("Conversion failed: {}", e)),
}
```

## Best Practices

### 1. Data Security
- Never expose API keys or internal IDs
- Validate all input data before conversion
- Use DTO for all API responses

### 2. Performance
- Use references when possible (`&UserSettings`)
- Implement efficient conversion methods
- Cache frequently accessed settings

### 3. Error Handling
- Always validate DTO before use
- Provide clear error messages
- Handle conversion errors gracefully

### 4. API Design
- Use consistent field naming
- Provide comprehensive documentation
- Include examples in API responses

## Integration with OpenAPI

The `UserSettingsDto` is fully integrated with OpenAPI documentation:

- **Schema Generation**: Automatic schema generation with `ToSchema`
- **Type Safety**: Compile-time validation of response types
- **Documentation**: Comprehensive API documentation
- **Validation**: Request/response validation

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dto_creation() {
        let dto = UserSettingsDto::new(
            "test@example.com".to_string(),
            "active".to_string(),
            false,
            true,
            vec!["tracking".to_string()],
            vec!["utm_source".to_string()],
            vec!["redirect".to_string()]
        );
        
        assert!(dto.is_valid());
        assert_eq!(dto.email, "test@example.com");
        assert_eq!(dto.status, "active");
    }

    #[test]
    fn test_conversion() {
        let user_settings = UserSettings::new(/* ... */);
        let dto = UserSettingsDto::from(&user_settings);
        assert!(dto.is_valid());
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_get_user_settings() {
    let response = get_user_settings(&mut request, &mut depot, &mut response).await;
    assert_eq!(response.status_code, 200);
    
    let body: UserSettingsDto = response.json().await.unwrap();
    assert!(body.is_valid());
}
```

This comprehensive DTO implementation provides a secure, efficient, and well-documented interface for user settings management in the API.
