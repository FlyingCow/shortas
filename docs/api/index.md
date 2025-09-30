---
layout: professional-theme
title: API Reference
permalink: /api/
---

<div class="hero-section">
  <h1>API Reference</h1>
  <p class="lead">This section provides comprehensive documentation for all Shortas APIs, including authentication, endpoints, data models, and integration examples.</p>
</div>

## 🔗 API Overview

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h3>Click Router API</h3>
    <p>High-performance, secure click aggregation API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and support for multiple database backends.</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-router-api" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h3>Click Aggregator API</h3>
    <p>High-performance, secure click aggregation API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and ClickHouse integration for analytics.</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator-api" class="btn" target="_blank">Learn More</a>
  </div>
</div>

## 🔐 Authentication

All protected endpoints require JWT authentication via Keycloak:

### Authentication Methods

#### JWT Bearer Token
```http
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### RPT Token (Fine-grained Authorization)
```http
Authorization: Bearer rpt_token_for_uma_authorization
```

### Required Permissions

| Resource | Permission | Description |
|----------|------------|-------------|
| Routes | `read:routes` | Read route information |
| Routes | `write:routes` | Create/update routes |
| Routes | `delete:routes` | Delete routes |
| Certificates | `read:certificates` | Read certificate information |
| Certificates | `write:certificates` | Create/update certificates |
| Certificates | `delete:certificates` | Delete certificates |
| User Settings | `read:user_settings` | Read user settings |
| User Settings | `write:user_settings` | Update user settings |

## 📊 Data Models

Understand the core data structures used across the Shortas APIs.

### RouteDto
```json
{
  "switch": "main",
  "link": "https://example.com",
  "dest": "https://target.com",
  "dest_format": "Http",
  "code": 301,
  "ttl": 3600,
  "status": "Active",
  "terminal": "External",
  "properties": {
    "route_id": "route-123",
    "domain_id": "domain-456",
    "owner_id": "user-789",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "opengraph": true,
    "allow_debug": false
  }
}
```

### KeycertDto
```json
{
  "key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
}
```

### UserSettingsDto
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

## 🔗 API Endpoints

### Click Router API Endpoints

#### Routes Management
- **GET** `/v1/routes/{switch}/{domain}/{path}` - Get route information
- **POST** `/v1/routes/{switch}/{domain}/{path}` - Create new route
- **PUT** `/v1/routes/{switch}/{domain}/{path}` - Update existing route
- **DELETE** `/v1/routes/{switch}/{domain}/{path}` - Delete route

#### Bulk Operations
- **POST** `/v1/routes/bulk` - Create multiple routes
- **PUT** `/v1/routes/bulk` - Update multiple routes
- **DELETE** `/v1/routes/bulk` - Delete multiple routes

#### SSL Certificate Management
- **GET** `/v1/certificates/{domain}` - Get certificate information
- **POST** `/v1/certificates/{domain}` - Create new certificate
- **PUT** `/v1/certificates/{domain}` - Update existing certificate
- **DELETE** `/v1/certificates/{domain}` - Delete certificate

#### User Settings Management
- **GET** `/v1/user-settings/{user_id}` - Get user settings
- **POST** `/v1/user-settings/{user_id}` - Create user settings
- **PUT** `/v1/user-settings/{user_id}` - Update user settings
- **DELETE** `/v1/user-settings/{user_id}` - Delete user settings

### Click Aggregator API Endpoints

The Click Aggregator API provides analytics and reporting endpoints for click stream data stored in ClickHouse.

#### Public Endpoints
- **GET** `/health` - Health check
- **GET** `/swagger-ui` - Interactive API documentation
- **GET** `/api-doc/openapi.json` - OpenAPI specification

## 📚 OpenAPI Documentation

### Interactive Documentation

- **Swagger UI**: `http://localhost:8080/swagger-ui`
- **OpenAPI Spec**: `http://localhost:8080/api-doc/openapi.json`

### API Information

- **Title**: Shortas APIs
- **Version**: 0.1.0
- **Description**: High-performance click aggregation and routing APIs
- **License**: MIT
- **Contact**: API Support (support@shortas.com)

### Authentication in Documentation

The Swagger UI supports JWT authentication:

1. Click "Authorize" button
2. Enter JWT token: `Bearer your_jwt_token`
3. All authenticated endpoints will use the token

## 🚨 Error Handling

### Error Response Format

```json
{
  "status_code": 400,
  "error": "Validation Error",
  "details": "Invalid input data: field 'email' is required"
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 409 | Conflict |
| 422 | Unprocessable Entity |
| 429 | Too Many Requests |
| 500 | Internal Server Error |

## 🔒 Security Features

- **JWT Authentication**: Secure token-based authentication
- **Role-Based Access Control**: Fine-grained permissions
- **Rate Limiting**: Protection against abuse
- **Input Validation**: Comprehensive request validation
- **Security Headers**: Automatic security header injection
- **CORS Support**: Configurable cross-origin resource sharing

### Security Headers

The APIs automatically include security headers:

```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
```

---

**Next Steps**: Explore the detailed documentation for the [Click Router API](https://github.com/FlyingCow/shortas/tree/main/redirect/click-router-api) or the [Click Aggregator API](https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator-api).
