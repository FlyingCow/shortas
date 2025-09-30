# JWT Authentication with Keycloak

This document outlines the JWT-based authentication system implemented in the Click Router API using Keycloak.

## Overview

The API now uses JWT (JSON Web Token) authentication with Keycloak integration instead of API keys. This provides:

- **Standard OAuth 2.0 / OpenID Connect** authentication
- **JWT token validation** with Keycloak JWKS
- **RPT (Requesting Party Token)** support for fine-grained authorization
- **Role-based access control** (RBAC)
- **Scope-based permissions**
- **Token introspection** for opaque tokens

## Keycloak Configuration

### Environment Variables

```bash
# Keycloak server configuration
KEYCLOAK_BASE_URL=http://keycloak:8080
KEYCLOAK_REALM=master
KEYCLOAK_CLIENT_ID=click-router-api
KEYCLOAK_CLIENT_SECRET=your-client-secret
KEYCLOAK_AUDIENCE=click-router-api

# Optional: Custom realm and client settings
KEYCLOAK_REALM=your-realm
KEYCLOAK_CLIENT_ID=your-client-id
```

### Keycloak Setup

1. **Create a new realm** (or use existing)
2. **Create a client** with the following settings:
   - Client ID: `click-router-api`
   - Client Protocol: `openid-connect`
   - Access Type: `confidential`
   - Standard Flow Enabled: `true`
   - Direct Access Grants Enabled: `true`
   - Service Accounts Enabled: `true`

3. **Configure client roles**:
   - `admin` - Full access to all resources
   - `user` - Read access to routes, certificates, user settings
   - `api-user` - Read/write access to routes and certificates

4. **Configure OAuth scopes**:
   - `routes:read` - Read route information
   - `routes:write` - Create/update routes
   - `routes:delete` - Delete routes
   - `certificates:read` - Read certificate information
   - `certificates:write` - Create/update certificates
   - `certificates:delete` - Delete certificates
   - `user-settings:read` - Read user settings
   - `user-settings:write` - Update user settings

## Authentication Flow

### 1. JWT Access Token Authentication

```http
GET /v1/routes
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Token Structure:**
```json
{
  "sub": "user-123",
  "iss": "http://keycloak:8080/realms/master",
  "aud": "click-router-api",
  "exp": 1640995200,
  "iat": 1640991600,
  "realm_access": {
    "roles": ["user", "api-user"]
  },
  "resource_access": {
    "click-router-api": {
      "roles": ["read", "write"]
    }
  },
  "preferred_username": "john.doe",
  "email": "john.doe@example.com",
  "name": "John Doe",
  "scope": "routes:read routes:write certificates:read"
}
```

### 2. RPT Token Authentication

```http
GET /rpt/routes
Authorization: Bearer rpt-token-here...
```

RPT tokens are opaque tokens that require introspection with Keycloak.

## API Endpoints

### Public Endpoints (No Authentication)
```http
GET /public/health          # Health check
GET /public/metrics         # Basic metrics
GET /api-doc/openapi.json   # API documentation
GET /swagger-ui/*           # Swagger UI
```

### JWT Protected Endpoints
```http
GET    /v1/routes/*          # Route management
GET    /v1/certificates/*    # Certificate management
GET    /v1/user-settings/*   # User settings
```

### RPT Protected Endpoints
```http
GET    /rpt/routes/*         # Route management with RPT
GET    /rpt/certificates/*   # Certificate management with RPT
GET    /rpt/user-settings/*  # User settings with RPT
```

## Permission System

### Role-Based Permissions

| Role | Permissions |
|------|-------------|
| `admin` | All permissions |
| `user` | `routes:read`, `certificates:read`, `user-settings:read` |
| `api-user` | `routes:read`, `routes:write`, `certificates:read` |

### Scope-Based Permissions

| Scope | Permission |
|-------|------------|
| `routes:read` | Read route information |
| `routes:write` | Create/update routes |
| `routes:delete` | Delete routes |
| `certificates:read` | Read certificate information |
| `certificates:write` | Create/update certificates |
| `certificates:delete` | Delete certificates |
| `user-settings:read` | Read user settings |
| `user-settings:write` | Update user settings |

## Token Validation

### JWT Token Validation

1. **Signature Verification**: Uses Keycloak JWKS endpoint
2. **Expiration Check**: Validates `exp` claim
3. **Issuer Validation**: Checks `iss` claim matches Keycloak
4. **Audience Validation**: Validates `aud` claim
5. **Algorithm Validation**: Supports RS256, RS384, RS512

### RPT Token Validation

1. **Token Introspection**: Calls Keycloak introspection endpoint
2. **Active Status**: Checks if token is active
3. **Permission Extraction**: Extracts roles and scopes from introspection response

## Error Responses

### Authentication Errors

```json
{
  "code": 401,
  "error": "Invalid token",
  "message": "Unauthorized",
  "details": "AuthenticationError::InvalidApiKey"
}
```

### Authorization Errors

```json
{
  "code": 403,
  "error": "Insufficient permissions",
  "message": "Forbidden",
  "details": "AuthenticationError::InsufficientPermissions"
}
```

### Token Expired

```json
{
  "code": 401,
  "error": "Token expired",
  "message": "Unauthorized",
  "details": "AuthenticationError::ExpiredToken"
}
```

## Security Features

### 1. JWT Validation
- **JWKS Caching**: Caches Keycloak public keys
- **Signature Verification**: RSA signature validation
- **Token Expiration**: Automatic expiration checking
- **Clock Skew**: Configurable clock skew tolerance

### 2. RPT Token Support
- **Token Introspection**: Real-time token validation
- **Permission Extraction**: Dynamic permission checking
- **Caching**: Introspection result caching
- **Timeout Handling**: Configurable introspection timeouts

### 3. Security Headers
```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
```

### 4. Rate Limiting
- **Per-user rate limiting** based on JWT subject
- **Configurable limits** per user/role
- **Token-based identification**

## Configuration

### JWT Configuration

```rust
let jwt_config = JwtConfig::from_env();
```

### Token Validation Configuration

```rust
let validation_config = TokenValidationConfig {
    validate_issuer: true,
    validate_audience: true,
    validate_expiration: true,
    clock_skew_seconds: 60,
    require_scope: false,
    allowed_algorithms: vec![Algorithm::RS256],
};
```

### RPT Configuration

```rust
let rpt_config = RptConfig {
    enabled: true,
    introspection_timeout_seconds: 30,
    cache_ttl_seconds: 300,
    require_uma_scope: true,
};
```

## Usage Examples

### 1. Get Access Token

```bash
curl -X POST http://keycloak:8080/realms/master/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=click-router-api" \
  -d "client_secret=your-client-secret" \
  -d "username=john.doe" \
  -d "password=password"
```

### 2. Use JWT Token

```bash
curl -X GET http://localhost:8080/v1/routes \
  -H "Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### 3. Use RPT Token

```bash
curl -X GET http://localhost:8080/rpt/routes \
  -H "Authorization: Bearer rpt-token-here..."
```

## Monitoring and Logging

### Security Events

- **Authentication failures**
- **Token validation errors**
- **Permission denials**
- **RPT introspection failures**
- **JWKS fetch errors**

### Log Format

```json
{
  "timestamp": "2024-01-01T00:00:00Z",
  "level": "WARN",
  "event": "jwt_validation_failed",
  "user_id": "user-123",
  "token_type": "access_token",
  "error": "Invalid signature",
  "endpoint": "/v1/routes"
}
```

## Production Considerations

### 1. Keycloak High Availability
- **Load balancer** configuration
- **Database clustering** for Keycloak
- **Session replication** across nodes

### 2. Token Security
- **Short token lifetimes** (15-30 minutes)
- **Refresh token rotation**
- **Token revocation** on logout
- **Secure token storage**

### 3. Performance
- **JWKS caching** with TTL
- **Token introspection caching**
- **Connection pooling** for Keycloak
- **Rate limiting** per user

### 4. Monitoring
- **Token validation metrics**
- **Authentication success/failure rates**
- **Permission check performance**
- **Keycloak connectivity monitoring**

## Migration from API Keys

### 1. Remove API Key Endpoints
- Remove `/api-keys/*` endpoints
- Remove API key management functionality
- Update documentation

### 2. Update Client Applications
- Implement OAuth 2.0 flow
- Store JWT tokens securely
- Handle token refresh
- Update error handling

### 3. Configure Keycloak
- Set up realm and client
- Configure roles and scopes
- Test authentication flow
- Monitor token validation

## Troubleshooting

### Common Issues

1. **JWKS Fetch Failures**
   - Check Keycloak connectivity
   - Verify JWKS endpoint URL
   - Check network configuration

2. **Token Validation Errors**
   - Verify token format
   - Check expiration time
   - Validate issuer and audience

3. **Permission Denied**
   - Check user roles in Keycloak
   - Verify scope assignments
   - Review permission mappings

4. **RPT Introspection Failures**
   - Check Keycloak introspection endpoint
   - Verify client credentials
   - Check token format

### Debug Configuration

```rust
// Enable debug logging
RUST_LOG=debug cargo run

// Check JWT configuration
curl http://localhost:8080/public/health
```
