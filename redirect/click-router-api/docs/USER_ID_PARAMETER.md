# User ID Parameter Documentation

This document provides comprehensive information about the `user_id` parameter used in the user settings endpoints.

## Parameter Details

### Parameter Specification

| Property | Value |
|----------|-------|
| **Name** | `user_id` |
| **Type** | `string` |
| **Location** | `path` |
| **Required** | `true` |
| **Description** | The user ID for the settings |
| **Example** | `"user123"` |

### OpenAPI Schema Definition

```yaml
parameters:
  - name: user_id
    in: path
    required: true
    description: The user ID for the settings
    schema:
      type: string
      pattern: '^[a-zA-Z0-9]([a-zA-Z0-9\-_]{0,61}[a-zA-Z0-9])?$'
      minLength: 1
      maxLength: 63
      examples:
        - user123
        - user-456
        - user_789
        - admin
```

### Parameter Validation Rules

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

#### 4. User ID Structure
- **Format**: `[a-zA-Z0-9][a-zA-Z0-9\-_]*[a-zA-Z0-9]`
- **Examples**: `user123`, `user-456`, `user_789`, `admin`
- **Invalid**: `-user123`, `user123-`, `user__123`, `user 123`

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

### Parameter Processing

#### 1. Input Validation
```rust
// User ID parameter validation
if user_id.is_empty() {
    return ErrorResponse::from_api_error(&ApiError::Validation(
        ValidationError::MissingField("user_id".to_string())
    ));
}
```

#### 2. Normalization
- **Case Preservation**: User ID case is preserved as provided
- **Whitespace**: Trim leading/trailing whitespace
- **Encoding**: Handle UTF-8 encoding properly

#### 3. Storage
- **Format**: Stored as provided string
- **Indexing**: Used as primary key for user settings lookup
- **Retrieval**: Exact match lookup

### Error Responses

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

### Usage in API Endpoints

#### GET /v1/user-settings/{user_id}
```yaml
get:
  summary: Get user settings
  description: Retrieves user settings for a specific user ID
  parameters:
    - name: user_id
      in: path
      required: true
      description: The user ID for the settings
      schema:
        type: string
        example: user123
```

#### POST /v1/user-settings/{user_id}
```yaml
post:
  summary: Create user settings
  description: Creates new user settings for a specific user ID
  parameters:
    - name: user_id
      in: path
      required: true
      description: The user ID for the settings
      schema:
        type: string
        example: user123
```

#### PUT /v1/user-settings/{user_id}
```yaml
put:
  summary: Update user settings
  description: Updates existing user settings for a specific user ID
  parameters:
    - name: user_id
      in: path
      required: true
      description: The user ID for the settings
      schema:
        type: string
        example: user123
```

#### DELETE /v1/user-settings/{user_id}
```yaml
delete:
  summary: Delete user settings
  description: Deletes user settings for a specific user ID
  parameters:
    - name: user_id
      in: path
      required: true
      description: The user ID for the settings
      schema:
        type: string
        example: user123
```

### Testing User ID Parameters

#### Valid Test Cases
```bash
# Standard user ID
curl -X GET "https://api.example.com/v1/user-settings/user123"

# User ID with hyphen
curl -X GET "https://api.example.com/v1/user-settings/user-456"

# User ID with underscore
curl -X GET "https://api.example.com/v1/user-settings/user_789"

# Complex user ID
curl -X GET "https://api.example.com/v1/user-settings/user-123_test"
```

#### Invalid Test Cases
```bash
# Missing user ID (should return 404)
curl -X GET "https://api.example.com/v1/user-settings/"

# Invalid format (should return 400)
curl -X GET "https://api.example.com/v1/user-settings/user123-"

# Special characters (should return 400)
curl -X GET "https://api.example.com/v1/user-settings/user@123"
```

### Security Considerations

#### 1. Input Sanitization
- **Validation**: Strict user ID format validation
- **Normalization**: Whitespace trimming and encoding handling
- **Encoding**: Proper UTF-8 encoding support

#### 2. Access Control
- **Authentication**: JWT token required
- **Authorization**: User can only access their own settings
- **Rate Limiting**: Applied to all endpoints

#### 3. Data Protection
- **Storage**: User IDs stored securely
- **Logging**: User IDs may be logged for audit purposes
- **Privacy**: No sensitive data in user ID parameter

### Implementation Notes

#### 1. Parameter Extraction
```rust
let user_id = req.param::<String>("user_id").unwrap_or_default();
```

#### 2. Validation Logic
```rust
if user_id.is_empty() {
    // Return validation error
}
```

#### 3. Normalization
```rust
let normalized_user_id = user_id.trim().to_string();
```

#### 4. Storage Integration
```rust
match app_state.user_settings_store.get_user_settings(&user_id).await {
    // Handle user settings operations
}
```

### Best Practices

#### 1. User ID Format
- Use consistent naming conventions
- Avoid special characters when possible
- Keep user IDs reasonably short
- Use meaningful prefixes (e.g., `user_`, `admin_`)

#### 2. Validation
- Always validate user ID format
- Check for empty or null values
- Implement proper error handling
- Provide clear error messages

#### 3. Security
- Sanitize user input
- Implement proper access control
- Use secure storage mechanisms
- Log access attempts for auditing

#### 4. Performance
- Use efficient string operations
- Consider user ID length limits
- Implement proper indexing
- Cache frequently accessed user settings

### Route Structure

#### New Route Structure
```
/v1/user-settings/{user_id}/
├── GET    - Retrieve user settings
├── POST   - Create user settings
├── PUT    - Update user settings
└── DELETE - Delete user settings
```

#### Route Examples
```
GET    /v1/user-settings/user123
POST   /v1/user-settings/user123
PUT    /v1/user-settings/user123
DELETE /v1/user-settings/user123
```

#### Router Configuration
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

### Migration Notes

#### From Old Structure
- **Old**: `/user-settings` with query parameters
- **New**: `/user-settings/{user_id}` with path parameters
- **Benefits**: Cleaner URLs, better RESTful design, improved caching

#### Backward Compatibility
- **Breaking Change**: Route structure has changed
- **Migration**: Update client code to use new route structure
- **Documentation**: Update API documentation with new routes

This comprehensive parameter documentation ensures that developers understand the user_id parameter requirements, validation rules, and usage patterns for the user settings endpoints.
