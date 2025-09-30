# OpenAPI Parameters Documentation

This document provides comprehensive information about the domain parameter used in the crypto controller endpoints.

## Domain Parameter Specification

### Parameter Details

| Property | Value |
|----------|-------|
| **Name** | `domain` |
| **Type** | `string` |
| **Location** | `path` |
| **Required** | `true` |
| **Description** | The domain name for the certificate |
| **Example** | `example.com` |

### OpenAPI Schema Definition

```yaml
parameters:
  - name: domain
    in: path
    required: true
    description: The domain name for the certificate
    schema:
      type: string
      pattern: '^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?)*$'
      minLength: 1
      maxLength: 253
      examples:
        - example.com
        - api.example.com
        - subdomain.example.org
        - localhost
```

### Parameter Validation Rules

#### 1. Format Requirements
- **Pattern**: Must match RFC 1123 domain name format
- **Structure**: Must contain at least one dot (.) for standard domains
- **Exceptions**: `localhost` is allowed without dots for development

#### 2. Length Constraints
- **Minimum**: 1 character (for `localhost`)
- **Maximum**: 253 characters (RFC 1123 limit)
- **Recommended**: Keep under 100 characters for better compatibility

#### 3. Character Restrictions
- **Allowed**: Letters (a-z, A-Z), numbers (0-9), hyphens (-)
- **Not Allowed**: Spaces, special characters, consecutive dots
- **Case**: Case-insensitive (will be normalized to lowercase)

#### 4. Domain Structure
- **Subdomains**: Supported (e.g., `api.example.com`)
- **Internationalized**: Supported (e.g., `例え.jp`)
- **Multi-level TLD**: Supported (e.g., `example.co.uk`)
- **Wildcard**: Supported (e.g., `*.example.com`)

### Valid Domain Examples

#### Standard Domains
```yaml
examples:
  - example.com
  - api.example.com
  - www.example.com
  - subdomain.example.org
  - test.example.co.uk
```

#### Internationalized Domains
```yaml
examples:
  - 例え.jp
  - пример.рф
  - münchen.de
  - café.fr
  - 测试.cn
```

#### Special Cases
```yaml
examples:
  - localhost
  - *.example.com
  - example.com.
  - _tcp.example.com
```

### Invalid Domain Examples

| Invalid Domain | Error Reason |
|----------------|--------------|
| `example` | Missing TLD (no dot) |
| `.example.com` | Starts with dot |
| `example.com.` | Ends with dot |
| `example..com` | Consecutive dots |
| `example com` | Contains space |
| `example@com` | Contains invalid character |
| `-example.com` | Starts with hyphen |
| `example-.com` | Ends with hyphen |
| `example.com/` | Contains slash |
| `example.com:8080` | Contains port |

### Parameter Processing

#### 1. Input Validation
```rust
// Domain parameter validation
if domain.is_empty() {
    return ErrorResponse::from_api_error(&ApiError::Validation(
        ValidationError::MissingField("domain".to_string())
    ));
}
```

#### 2. Normalization
- **Case Conversion**: Convert to lowercase
- **Whitespace**: Trim leading/trailing whitespace
- **Encoding**: Handle internationalized domain names

#### 3. Storage
- **Format**: Stored as normalized lowercase string
- **Indexing**: Used as primary key for certificate lookup
- **Retrieval**: Case-insensitive lookup

### Error Responses

#### 400 Bad Request - Invalid Domain
```json
{
  "status_code": 400,
  "error": "Validation error: Missing field 'domain'",
  "details": "ValidationError::MissingField(\"domain\")"
}
```

#### 400 Bad Request - Invalid Format
```json
{
  "status_code": 400,
  "error": "Validation error: Invalid domain format",
  "details": "ValidationError::InvalidInput { field: \"domain\", message: \"Invalid domain format\" }"
}
```

### Usage in API Endpoints

#### GET /v1/certificates/{domain}
```yaml
get:
  summary: Get SSL certificate
  description: Retrieves SSL certificate information for a specific domain
  parameters:
    - name: domain
      in: path
      required: true
      description: The domain name for the certificate
      schema:
        type: string
        example: example.com
```

#### POST /v1/certificates/{domain}
```yaml
post:
  summary: Create SSL certificate
  description: Creates a new SSL certificate entry for the specified domain
  parameters:
    - name: domain
      in: path
      required: true
      description: The domain name for the certificate
      schema:
        type: string
        example: example.com
```

#### PUT /v1/certificates/{domain}
```yaml
put:
  summary: Update SSL certificate
  description: Updates the SSL certificate information for the specified domain
  parameters:
    - name: domain
      in: path
      required: true
      description: The domain name for the certificate
      schema:
        type: string
        example: example.com
```

#### DELETE /v1/certificates/{domain}
```yaml
delete:
  summary: Delete SSL certificate
  description: Deletes the SSL certificate information for the specified domain
  parameters:
    - name: domain
      in: path
      required: true
      description: The domain name for the certificate
      schema:
        type: string
        example: example.com
```

### Testing Domain Parameters

#### Valid Test Cases
```bash
# Standard domain
curl -X GET "https://api.example.com/v1/certificates/example.com"

# Subdomain
curl -X GET "https://api.example.com/v1/certificates/api.example.com"

# Internationalized domain
curl -X GET "https://api.example.com/v1/certificates/例え.jp"

# Case-insensitive (will be normalized)
curl -X GET "https://api.example.com/v1/certificates/EXAMPLE.COM"
```

#### Invalid Test Cases
```bash
# Missing domain (should return 404)
curl -X GET "https://api.example.com/v1/certificates/"

# Invalid format (should return 400)
curl -X GET "https://api.example.com/v1/certificates/example"

# Special characters (should return 400)
curl -X GET "https://api.example.com/v1/certificates/example@com"
```

### Security Considerations

#### 1. Input Sanitization
- **Validation**: Strict domain format validation
- **Normalization**: Case and whitespace normalization
- **Encoding**: Proper handling of internationalized domains

#### 2. Access Control
- **Authentication**: JWT token required
- **Authorization**: Permission-based access control
- **Rate Limiting**: Applied to all endpoints

#### 3. Data Protection
- **Storage**: Domain names stored securely
- **Logging**: Domain names may be logged for audit purposes
- **Privacy**: No sensitive data in domain parameter

### Implementation Notes

#### 1. Parameter Extraction
```rust
let domain = req.param::<String>("domain").unwrap_or_default();
```

#### 2. Validation Logic
```rust
if domain.is_empty() {
    // Return validation error
}
```

#### 3. Normalization
```rust
let normalized_domain = domain.to_lowercase().trim().to_string();
```

#### 4. Storage Integration
```rust
match app_state.crypto_store.get_certificate(&domain).await {
    // Handle certificate operations
}
```

### Best Practices

#### 1. Domain Format
- Use standard domain formats when possible
- Avoid special characters and spaces
- Keep domain names reasonably short

#### 2. Internationalization
- Use proper encoding for international domains
- Test with various character sets
- Consider normalization requirements

#### 3. Error Handling
- Provide clear error messages for invalid domains
- Include validation details in error responses
- Log validation failures for monitoring

#### 4. Performance
- Cache domain validation results when possible
- Use efficient string operations for normalization
- Consider domain length limits for performance

This comprehensive parameter documentation ensures that developers understand the domain parameter requirements, validation rules, and usage patterns for the crypto controller endpoints.
