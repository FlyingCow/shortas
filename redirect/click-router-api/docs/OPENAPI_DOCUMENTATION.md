# Click Router API - OpenAPI Documentation

This document provides comprehensive documentation for the Click Router API's OpenAPI/Swagger integration.

## Overview

The Click Router API provides comprehensive OpenAPI 3.0 documentation with JWT authentication support. The API is built using the Salvo web framework with integrated OpenAPI schema generation and Swagger UI.

## API Documentation Endpoints

### Swagger UI
- **URL**: `/swagger-ui`
- **Description**: Interactive API documentation interface
- **Authentication**: None required

### OpenAPI Specification
- **URL**: `/api-doc/openapi.json`
- **Description**: Raw OpenAPI 3.0 specification in JSON format
- **Authentication**: None required

## API Information

- **Title**: Click Router API
- **Version**: 0.1.0
- **Description**: A high-performance click aggregation API with JWT authentication via Keycloak
- **License**: MIT
- **Contact**: API Support (support@example.com)

## Authentication

The API uses JWT (JSON Web Token) authentication with two main schemes:

### Bearer Token (Standard JWT)
- **Type**: HTTP Bearer Token
- **Header**: `Authorization: Bearer <jwt_token>`
- **Usage**: Standard JWT authentication for most endpoints
- **Token Source**: Keycloak authentication server

### RPT Token (Requesting Party Token)
- **Type**: HTTP Bearer Token
- **Header**: `Authorization: Bearer <rpt_token>`
- **Usage**: Fine-grained authorization with UMA (User Managed Access)
- **Token Source**: Keycloak token introspection

## API Endpoints

### Public Endpoints

These endpoints do not require authentication:

#### Health Check
- **Path**: `/public/health`
- **Method**: `GET`
- **Description**: Returns the current health status of the API service
- **Response**: 
  ```json
  {
    "status": "healthy",
    "timestamp": "2024-01-01T12:00:00Z",
    "version": "0.1.0"
  }
  ```

#### Metrics
- **Path**: `/public/metrics`
- **Method**: `GET`
- **Description**: Returns basic metrics about the API service
- **Response**: 
  ```json
  {
    "requests_total": 0,
    "errors_total": 0,
    "uptime_seconds": 0
  }
  ```

### Protected Endpoints (JWT Authentication Required)

These endpoints require valid JWT authentication:

#### Routes

##### Get Route Information
- **Path**: `/v1/routes`
- **Method**: `GET`
- **Description**: Retrieves routing information for a specific switch, domain, and path combination
- **Parameters**: 
  - `domain` (query): Domain name
  - `path` (query): Path component
  - `switch` (query): Switch identifier
- **Authentication**: Bearer JWT token required
- **Responses**:
  - `200`: Route found successfully
  - `404`: Route not found
  - `401`: Unauthorized - Invalid or missing JWT token
  - `403`: Forbidden - Insufficient permissions
  - `500`: Internal server error

##### Get Main Route Information
- **Path**: `/v1/routes/{domain}/{path}/{switch}`
- **Method**: `GET`
- **Description**: Retrieves the main route information for a specific domain and path combination
- **Parameters**: 
  - `domain` (path): Domain name
  - `path` (path): Path component
  - `switch` (path): Switch identifier (automatically set to "main")
- **Authentication**: Bearer JWT token required
- **Responses**:
  - `200`: Main route found successfully
  - `404`: Main route not found
  - `401`: Unauthorized - Invalid or missing JWT token
  - `403`: Forbidden - Insufficient permissions
  - `500`: Internal server error

#### Certificates

##### Get SSL Certificate
- **Path**: `/v1/certificates`
- **Method**: `GET`
- **Description**: Retrieves the SSL certificate information for a specific domain
- **Parameters**: 
  - `domain` (query): Domain name
- **Authentication**: Bearer JWT token required
- **Responses**:
  - `200`: Certificate found successfully
  - `404`: Certificate not found
  - `401`: Unauthorized - Invalid or missing JWT token
  - `403`: Forbidden - Insufficient permissions
  - `500`: Internal server error

#### User Settings

##### Get User Settings
- **Path**: `/v1/user-settings`
- **Method**: `GET`
- **Description**: Retrieves user settings for a specific user ID. If no user ID is provided, uses the user ID from the JWT token context
- **Parameters**: 
  - `user_id` (query, optional): User identifier
- **Authentication**: Bearer JWT token required
- **Responses**:
  - `200`: User settings found successfully
  - `404`: User not found
  - `401`: Unauthorized - Invalid or missing JWT token
  - `403`: Forbidden - Insufficient permissions
  - `500`: Internal server error

### RPT Token Endpoints (Fine-grained Authorization)

These endpoints use RPT tokens for fine-grained authorization:

- **Base Path**: `/rpt`
- **Endpoints**: Same as protected endpoints but with RPT token authentication
- **Authentication**: RPT Bearer token required
- **Usage**: For resources requiring fine-grained permissions and UMA-compliant authorization

## Data Models

### Route
```json
{
  "switch": "string",
  "link": "string",
  "dest": "string",
  "dest_format": "Http|Native",
  "code": 200,
  "ttl": 3600000,
  "status": "Active|Blocked",
  "terminal": "External|Internal|Middleware",
  "policy": "Basic|Conditional|Challenge|File|Mirroring|Unknown",
  "properties": {
    "route_id": "string",
    "domain_id": "string",
    "owner_id": "string",
    "creator_id": "string",
    "workspace_id": "string",
    "scripts": ["string"],
    "tags": ["string"],
    "custom": {},
    "native": {},
    "bundling": {},
    "opengraph": false,
    "allow_debug": false
  }
}
```

### Certificate (Keycert)
```json
{
  "key": "base64_encoded_private_key",
  "cert": "base64_encoded_certificate",
  "ocsp_resp": "base64_encoded_ocsp_response"
}
```

### User Settings
```json
{
  "user_id": "string",
  "user_email": "string",
  "api_key": "string",
  "active_status": "Active|Blocked",
  "debug": false,
  "overflow": false,
  "skip": ["string"],
  "allowed_request_params": ["string"],
  "allowed_destination_params": ["string"]
}
```

### Error Response
```json
{
  "status_code": 404,
  "error": "Error message",
  "details": "Additional error details"
}
```

## Security Features

### Rate Limiting
- **Implementation**: In-memory rate limiting per client
- **Default Limits**: Configurable per endpoint
- **Headers**: Rate limit information included in response headers

### Input Validation
- **Parameter Validation**: Length limits and format validation
- **Injection Protection**: SQL injection and XSS prevention
- **Content Length Limits**: Request size limitations

### Security Headers
- **X-Content-Type-Options**: nosniff
- **X-Frame-Options**: DENY
- **X-XSS-Protection**: 1; mode=block
- **Strict-Transport-Security**: max-age=31536000; includeSubDomains
- **Content-Security-Policy**: default-src 'self'

## JWT Token Structure

### Standard JWT Claims
```json
{
  "sub": "user_id",
  "iss": "keycloak_issuer",
  "aud": "client_id",
  "exp": 1640995200,
  "iat": 1640991600,
  "realm_access": {
    "roles": ["user", "admin"]
  },
  "resource_access": {
    "client_id": {
      "roles": ["read:routes", "write:routes"]
    }
  },
  "preferred_username": "username",
  "email": "user@example.com",
  "name": "Full Name",
  "scope": "openid profile email"
}
```

### RPT Token
- **Type**: Opaque token
- **Validation**: Token introspection with Keycloak
- **Permissions**: Fine-grained resource permissions
- **UMA Compliance**: User Managed Access 2.0 compliant

## Configuration

### Environment Variables

#### Keycloak Configuration
- `KEYCLOAK_BASE_URL`: Keycloak server base URL (default: http://keycloak:8080)
- `KEYCLOAK_REALM`: Keycloak realm name (default: master)
- `KEYCLOAK_CLIENT_ID`: OAuth client ID
- `KEYCLOAK_CLIENT_SECRET`: OAuth client secret (optional)
- `KEYCLOAK_AUDIENCE`: Token audience (optional)

#### Token Validation
- `JWT_VALIDATE_ISSUER`: Validate token issuer (default: true)
- `JWT_VALIDATE_AUDIENCE`: Validate token audience (default: true)
- `JWT_VALIDATE_EXPIRATION`: Validate token expiration (default: true)
- `JWT_CLOCK_SKEW_SECONDS`: Clock skew tolerance (default: 60)

#### RPT Configuration
- `RPT_ENABLED`: Enable RPT token support (default: false)
- `RPT_INTROSPECTION_TIMEOUT`: Introspection timeout in seconds (default: 10)
- `RPT_CACHE_TTL`: Token cache TTL in seconds (default: 300)

## Error Handling

### HTTP Status Codes
- `200`: Success
- `400`: Bad Request - Invalid input or parameters
- `401`: Unauthorized - Missing or invalid authentication
- `403`: Forbidden - Insufficient permissions
- `404`: Not Found - Resource not found
- `429`: Too Many Requests - Rate limit exceeded
- `500`: Internal Server Error - Server-side error
- `502`: Bad Gateway - External service error
- `503`: Service Unavailable - Service temporarily unavailable

### Error Categories
- **Authentication Errors**: Invalid tokens, expired tokens, missing authentication
- **Authorization Errors**: Insufficient permissions, blocked accounts
- **Validation Errors**: Invalid input, missing fields, format errors
- **Database Errors**: Connection failures, query errors, timeouts
- **External Service Errors**: AWS errors, MongoDB errors, service unavailable
- **Internal Errors**: Serialization errors, unknown errors

## Usage Examples

### Using curl

#### Get Health Status
```bash
curl -X GET "http://localhost:8080/public/health"
```

#### Get Route with JWT Authentication
```bash
curl -X GET "http://localhost:8080/v1/routes?domain=example.com&path=/api&switch=main" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

#### Get Certificate with JWT Authentication
```bash
curl -X GET "http://localhost:8080/v1/certificates?domain=example.com" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Using JavaScript/fetch

```javascript
// Get health status
const healthResponse = await fetch('/public/health');
const health = await healthResponse.json();

// Get route with authentication
const routeResponse = await fetch('/v1/routes?domain=example.com&path=/api&switch=main', {
  headers: {
    'Authorization': `Bearer ${jwtToken}`
  }
});
const route = await routeResponse.json();
```

## Testing

### Swagger UI Testing
1. Navigate to `/swagger-ui` in your browser
2. Authenticate using the "Authorize" button
3. Enter your JWT token in the format: `Bearer YOUR_TOKEN`
4. Test endpoints directly from the interface

### Automated Testing
```bash
# Run API tests
cargo test

# Check OpenAPI specification validity
cargo check
```

## Best Practices

### Authentication
- Always include the `Authorization` header with valid JWT tokens
- Handle token expiration gracefully with refresh logic
- Use HTTPS in production environments

### Error Handling
- Check HTTP status codes for error conditions
- Parse error response bodies for detailed error information
- Implement retry logic for transient errors (5xx status codes)

### Rate Limiting
- Respect rate limit headers in responses
- Implement exponential backoff for rate-limited requests
- Monitor rate limit usage to avoid hitting limits

### Security
- Validate all input parameters
- Use parameterized queries to prevent injection attacks
- Implement proper CORS policies for web applications
- Keep JWT tokens secure and avoid logging them

## Troubleshooting

### Common Issues

#### 401 Unauthorized
- Check JWT token validity and expiration
- Verify token format: `Bearer <token>`
- Ensure Keycloak is accessible and configured correctly

#### 403 Forbidden
- Verify user has required roles/permissions
- Check resource access configuration in Keycloak
- Ensure client has proper scope assignments

#### 404 Not Found
- Verify endpoint paths and parameters
- Check if resources exist in the database
- Ensure API version is correct

#### 500 Internal Server Error
- Check server logs for detailed error information
- Verify database connectivity
- Ensure all required environment variables are set

### Debug Mode
Enable debug logging by setting environment variables:
```bash
RUST_LOG=debug
```

This will provide detailed logging for troubleshooting authentication and authorization issues.

## Support

For API support and questions:
- **Email**: support@example.com
- **Documentation**: This document and Swagger UI
- **Source Code**: Available in the project repository
- **Issues**: Report bugs and feature requests through the project's issue tracker
