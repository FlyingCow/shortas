# User Settings Endpoints Documentation

This document provides comprehensive information about the user settings CRUD endpoints in the click-router-api.

## Overview

The user settings endpoints provide full CRUD (Create, Read, Update, Delete) operations for managing user configuration settings. All endpoints require JWT authentication and appropriate permissions.

## Route Structure

The user settings endpoints use a sub-route structure with `{user_id}` as a path parameter:

```
/v1/user-settings/{user_id}/
├── GET    - Retrieve user settings
├── POST   - Create user settings  
├── PUT    - Update user settings
└── DELETE - Delete user settings
```

### Router Configuration
```rust
pub fn api_routes() -> Router {
    Router::with_path("/user-settings").push(
        Router::with_path("/{user_id}")
            .get(get_user_settings)
            .post(create_user_settings)
            .put(update_user_settings)
            .delete(delete_user_settings),
    )
}
```

## Endpoints

### 1. GET /v1/user-settings/{user_id}

#### Description
Retrieves user settings for a specific user ID. The user_id is required as a path parameter in the new route structure.

#### Parameters
- **user_id** (path, required): The user ID for the settings
  - Type: `String`
  - Example: `"user123"`
  - Format: Alphanumeric with optional hyphens and underscores
  - Pattern: `^[a-zA-Z0-9]([a-zA-Z0-9\-_]{0,61}[a-zA-Z0-9])?$`
  - Length: 1-63 characters
  - Note: Required path parameter in new route structure

#### Request Example
```bash
curl -X GET "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>"
```

#### Response Example (200 OK)
```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking", "analytics"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["redirect", "target"]
}
```

#### Response Codes
- **200 OK**: User settings found successfully
- **404 Not Found**: User not found
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **500 Internal Server Error**: Server error

---

### 2. POST /v1/user-settings/{user_id}

#### Description
Creates new user settings for a specific user ID. The settings data must be provided in the request body.

#### Parameters
- **user_id** (path, required): The user ID for the settings
  - Type: `String`
  - Example: `"user123"`

#### Request Body
```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking", "analytics"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["redirect", "target"]
}
```

#### Request Example
```bash
curl -X POST "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "status": "active",
    "debug": false,
    "overflow": true,
    "skip_tracking": ["tracking"],
    "allowed_request_params": ["utm_source"],
    "allowed_destination_params": ["redirect"]
  }'
```

#### Response Example (201 Created)
```json
{
  "message": "User settings created successfully",
  "user_id": "user123"
}
```

#### Response Codes
- **201 Created**: User settings created successfully
- **400 Bad Request**: Invalid input data
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **409 Conflict**: User settings already exist
- **500 Internal Server Error**: Server error

---

### 3. PUT /v1/user-settings/{user_id}

#### Description
Updates existing user settings for a specific user ID. The settings data must be provided in the request body.

#### Parameters
- **user_id** (path, required): The user ID for the settings
  - Type: `String`
  - Example: `"user123"`

#### Request Body
```json
{
  "email": "user@example.com",
  "status": "active",
  "debug": true,
  "overflow": false,
  "skip_tracking": ["tracking", "analytics", "utm_source"],
  "allowed_request_params": ["utm_source", "utm_medium", "utm_campaign"],
  "allowed_destination_params": ["redirect", "target", "destination"]
}
```

#### Request Example
```bash
curl -X PUT "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "status": "active",
    "debug": true,
    "overflow": false,
    "skip_tracking": ["tracking", "analytics"],
    "allowed_request_params": ["utm_source", "utm_medium"],
    "allowed_destination_params": ["redirect", "target"]
  }'
```

#### Response Example (200 OK)
```json
{
  "message": "User settings updated successfully",
  "user_id": "user123"
}
```

#### Response Codes
- **200 OK**: User settings updated successfully
- **400 Bad Request**: Invalid input data
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: User settings not found
- **500 Internal Server Error**: Server error

---

### 4. DELETE /v1/user-settings/{user_id}

#### Description
Deletes user settings for a specific user ID. This action is irreversible.

#### Parameters
- **user_id** (path, required): The user ID for the settings
  - Type: `String`
  - Example: `"user123"`

#### Request Example
```bash
curl -X DELETE "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>"
```

#### Response Example (200 OK)
```json
{
  "message": "User settings deleted successfully",
  "user_id": "user123"
}
```

#### Response Codes
- **200 OK**: User settings deleted successfully
- **400 Bad Request**: Invalid user ID
- **401 Unauthorized**: Invalid or missing JWT token
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: User settings not found
- **500 Internal Server Error**: Server error

## User ID Parameter Details

### Parameter Specification

| Property | Value |
|----------|-------|
| **Name** | `user_id` |
| **Type** | `string` |
| **Location** | `path` |
| **Required** | `true` |
| **Description** | The user ID for the settings |
| **Example** | `"user123"` |

### Validation Rules

#### 1. Format Requirements
- **Pattern**: Must match alphanumeric characters with optional hyphens and underscores
- **Structure**: Must start and end with alphanumeric characters
- **Special Characters**: Only hyphens (-) and underscores (_) allowed in the middle

#### 2. Length Constraints
- **Minimum**: 1 character
- **Maximum**: 63 characters (RFC 1123 limit)
- **Recommended**: Keep under 32 characters for better compatibility

#### 3. Character Restrictions
- **Allowed**: Letters (a-z, A-Z), numbers (0-9), hyphens (-), underscores (_)
- **Not Allowed**: Spaces, special characters, consecutive hyphens/underscores
- **Case**: Case-sensitive (will be preserved as provided)

### Valid User ID Examples

#### Standard User IDs
```yaml
examples:
  - user123
  - user-456
  - user_789
  - admin
  - test-user
  - test_user
  - user123-test
  - user_123_test
```

#### Complex User IDs
```yaml
examples:
  - user-123-test
  - user_123_test
  - admin-user-456
  - test_user_789
  - user-123_test-456
  - admin_user-123_test
```

### Invalid User ID Examples

| Invalid User ID | Error Reason |
|----------------|--------------|
| `user123-` | Ends with hyphen |
| `-user123` | Starts with hyphen |
| `user__123` | Consecutive underscores |
| `user--123` | Consecutive hyphens |
| `user 123` | Contains space |
| `user@123` | Contains invalid character |
| `user.123` | Contains dot |
| `user/123` | Contains slash |
| `user:123` | Contains colon |
| `` | Empty string |

### Error Responses for Invalid User ID

#### 400 Bad Request - Missing User ID
```json
{
  "status_code": 400,
  "error": "Validation error: Missing field 'user_id'",
  "details": "ValidationError::MissingField(\"user_id\")"
}
```

#### 400 Bad Request - Invalid Format
```json
{
  "status_code": 400,
  "error": "Validation error: Invalid user ID format",
  "details": "ValidationError::InvalidInput { field: \"user_id\", message: \"Invalid user ID format\" }"
}
```

## Data Models

### UserSettingsDto

The API uses a DTO (Data Transfer Object) for all user settings operations:

```json
{
  "email": "string",                    // User's email address
  "status": "string",                  // "active" or "blocked"
  "debug": "boolean",                  // Debug mode enabled
  "overflow": "boolean",               // Overflow handling enabled
  "skip_tracking": ["string"],         // Skip tracking parameters
  "allowed_request_params": ["string"], // Allowed request parameters
  "allowed_destination_params": ["string"] // Allowed destination parameters
}
```

### Field Descriptions

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `email` | `String` | User's email address | `"user@example.com"` |
| `status` | `String` | Account status | `"active"` or `"blocked"` |
| `debug` | `Boolean` | Debug mode enabled | `true` or `false` |
| `overflow` | `Boolean` | Overflow handling enabled | `true` or `false` |
| `skip_tracking` | `Array<String>` | Parameters to skip during tracking | `["tracking", "analytics"]` |
| `allowed_request_params` | `Array<String>` | Allowed request parameters | `["utm_source", "utm_medium"]` |
| `allowed_destination_params` | `Array<String>` | Allowed destination parameters | `["redirect", "target"]` |

## Validation Rules

### Required Fields
- `email`: Must be a valid email address
- `status`: Must be "active" or "blocked"

### Optional Fields
- `debug`: Boolean, defaults to `false`
- `overflow`: Boolean, defaults to `false`
- `skip_tracking`: Array of strings, defaults to `[]`
- `allowed_request_params`: Array of strings, defaults to `[]`
- `allowed_destination_params`: Array of strings, defaults to `[]`

### Validation Examples

#### Valid Request
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

#### Invalid Request (Missing Required Fields)
```json
{
  "status": "active",
  "debug": false
}
```
**Error Response:**
```json
{
  "status_code": 400,
  "error": "Validation error: User settings data is incomplete or invalid",
  "details": "ValidationError::InvalidInput"
}
```

## Error Handling

### Common Error Responses

#### 400 Bad Request - Invalid Input
```json
{
  "status_code": 400,
  "error": "Validation error: Invalid input data",
  "details": "ValidationError::InvalidInput { field: \"user_settings\", message: \"User settings data is incomplete or invalid\" }"
}
```

#### 401 Unauthorized - Invalid JWT
```json
{
  "status_code": 401,
  "error": "Authentication error: Invalid JWT token",
  "details": "AuthenticationError::InvalidToken"
}
```

#### 403 Forbidden - Insufficient Permissions
```json
{
  "status_code": 403,
  "error": "Authentication error: Insufficient permissions",
  "details": "AuthenticationError::InsufficientPermissions"
}
```

#### 404 Not Found - User Not Found
```json
{
  "status_code": 404,
  "error": "Authentication error: User not found",
  "details": "AuthenticationError::UserNotFound(\"user123\")"
}
```

#### 409 Conflict - User Already Exists
```json
{
  "status_code": 409,
  "error": "Database error: User settings already exist",
  "details": "DatabaseError::Conflict"
}
```

## Security

### Authentication
- **JWT Token**: Required for all operations
- **Bearer Token**: Use `Authorization: Bearer <jwt_token>` header
- **Token Validation**: Tokens are validated against Keycloak

### Authorization
- **User Context**: Users can only access their own settings
- **Permission Validation**: JWT claims are validated for appropriate permissions
- **Access Control**: Role-based access control through JWT claims

### Data Protection
- **Sensitive Data**: API keys and internal IDs are not exposed in DTOs
- **Input Validation**: All input data is validated and sanitized
- **Error Handling**: Sensitive information is not exposed in error messages

## Usage Examples

### Complete CRUD Workflow

#### 1. Create User Settings
```bash
curl -X POST "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "status": "active",
    "debug": false,
    "overflow": true,
    "skip_tracking": ["tracking"],
    "allowed_request_params": ["utm_source"],
    "allowed_destination_params": ["redirect"]
  }'
```

#### 2. Get User Settings
```bash
curl -X GET "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>"
```

#### 3. Update User Settings
```bash
curl -X PUT "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "status": "active",
    "debug": true,
    "overflow": false,
    "skip_tracking": ["tracking", "analytics"],
    "allowed_request_params": ["utm_source", "utm_medium"],
    "allowed_destination_params": ["redirect", "target"]
  }'
```

#### 4. Delete User Settings
```bash
curl -X DELETE "https://api.example.com/v1/user-settings/user123" \
  -H "Authorization: Bearer <jwt_token>"
```

### JavaScript/TypeScript Examples

#### Using Fetch API
```javascript
// Create user settings
async function createUserSettings(userId, settings, jwtToken) {
  const response = await fetch(`/v1/user-settings/${userId}`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${jwtToken}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(settings)
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}

// Get user settings
async function getUserSettings(userId, jwtToken) {
  const response = await fetch(`/v1/user-settings/${userId}`, {
    headers: {
      'Authorization': `Bearer ${jwtToken}`
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}

// Update user settings
async function updateUserSettings(userId, settings, jwtToken) {
  const response = await fetch(`/v1/user-settings/${userId}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${jwt_token}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(settings)
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}

// Delete user settings
async function deleteUserSettings(userId, jwtToken) {
  const response = await fetch(`/v1/user-settings/${userId}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${jwtToken}`
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}
```

## Best Practices

### 1. Error Handling
- Always check HTTP status codes
- Handle authentication errors gracefully
- Provide user-friendly error messages
- Log errors for debugging

### 2. Data Validation
- Validate all input data before sending
- Use appropriate data types
- Check required fields
- Sanitize user input

### 3. Security
- Always use HTTPS in production
- Store JWT tokens securely
- Implement proper token refresh
- Use least privilege principle

### 4. Performance
- Cache user settings when appropriate
- Use efficient data structures
- Implement proper pagination for large datasets
- Monitor API response times

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user_settings() {
        // Test user settings creation
        let settings = UserSettingsDto::new(
            "test@example.com".to_string(),
            "active".to_string(),
            false,
            true,
            vec!["tracking".to_string()],
            vec!["utm_source".to_string()],
            vec!["redirect".to_string()]
        );
        
        assert!(settings.is_valid());
    }

    #[tokio::test]
    async fn test_update_user_settings() {
        // Test user settings update
        // Implementation details...
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_user_settings_crud() {
    // Test complete CRUD workflow
    // 1. Create user settings
    // 2. Get user settings
    // 3. Update user settings
    // 4. Delete user settings
}
```

This comprehensive documentation provides all the information needed to effectively use the user settings CRUD endpoints in the click-router-api.
