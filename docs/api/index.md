---
layout: vector-theme
title: API Reference
permalink: /api/
---

<div class="hero-section">
  <h1>API Reference</h1>
  <p class="lead">This section provides comprehensive documentation for all Shortas APIs, including authentication, endpoints, data models, and integration examples.</p>
</div>

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h3>Click Router API</h3>
    <p>Route and settings management with comprehensive CRUD operations.</p>
    <a href="click-router/" class="btn">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h3>Click Aggregator API</h3>
    <p>Analytics and reporting with detailed insights and metrics.</p>
    <a href="click-aggregator/" class="btn">Learn More</a>
  </div>
</div>

## 🔐 Authentication

All protected endpoints require JWT authentication via Keycloak:

```http
Authorization: Bearer <jwt_token>
```

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

### Route Model

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

### Click Stream Model

```json
{
  "id": "string",
  "owner_id": "string",
  "creator_id": "string", 
  "route_id": "string",
  "workspace_id": "string",
  "created": "2024-01-01T12:00:00Z",
  "dest": "string",
  "ip": "string",
  "continent": "string",
  "country": "string", 
  "location": "string",
  "os_family": "string",
  "os_version": "string",
  "user_agent_family": "string",
  "user_agent_version": "string",
  "device_brand": "string",
  "device_family": "string",
  "device_model": "string",
  "session_first": "2024-01-01T12:00:00Z",
  "session_clicks": 1,
  "is_unique": true,
  "is_bot": false
}
```

### User Settings Model

```json
{
  "user_id": "string",
  "user_email": "string",
  "api_key": "string",
  "active_status": "Active|Blocked",
  "debug": false,
  "overflow": false,
  "skip": ["tracking"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["ref", "campaign"]
}
```

### Certificate Model

```json
{
  "key": "base64_encoded_private_key",
  "cert": "base64_encoded_certificate", 
  "ocsp_resp": "base64_encoded_ocsp_response"
}
```

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

### Error Types

- **ValidationError**: Input validation failures
- **AuthenticationError**: Authentication failures
- **AuthorizationError**: Permission denied
- **NotFoundError**: Resource not found
- **ConflictError**: Resource conflicts
- **DatabaseError**: Database operation failures
- **NetworkError**: Network communication failures

## 🔧 Rate Limiting

### Default Limits

- **Default**: 100 requests per minute per IP
- **Configurable**: Per-endpoint rate limits
- **Burst Protection**: Temporary blocking for excessive requests

### Rate Limit Headers

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640995200
```

## 📚 API Documentation

### Click Router API

The Click Router API provides comprehensive route management, SSL certificate handling, and user settings management.

#### Base URL
- **Development**: `http://localhost:8081`
- **Production**: `https://api.yourdomain.com`

#### Endpoints

**Routes Management:**
- `GET /v1/routes/{domain}/{path}` - Get route information
- `GET /v1/routes/{domain}/{path}/{switch}` - Get specific route switch
- `POST /v1/routes` - Create new route
- `PUT /v1/routes/{domain}/{path}` - Update route
- `DELETE /v1/routes/{domain}/{path}` - Delete route

**SSL Certificate Management:**
- `GET /v1/certificates/{domain}` - Get certificate
- `POST /v1/certificates/{domain}` - Create certificate
- `PUT /v1/certificates/{domain}` - Update certificate
- `DELETE /v1/certificates/{domain}` - Delete certificate

**User Settings:**
- `GET /v1/user-settings/{user_id}` - Get user settings
- `POST /v1/user-settings/{user_id}` - Create user settings
- `PUT /v1/user-settings/{user_id}` - Update user settings
- `DELETE /v1/user-settings/{user_id}` - Delete user settings

**Public Endpoints:**
- `GET /public/health` - Health check
- `GET /public/metrics` - Service metrics
- `GET /swagger-ui` - Interactive API documentation
- `GET /api-doc/openapi.json` - OpenAPI specification

### Click Aggregator API

The Click Aggregator API provides analytics and click stream data access.

#### Base URL
- **Development**: `http://localhost:8082`
- **Production**: `https://analytics.yourdomain.com`

#### Endpoints

**Click Stream Analytics:**
- `GET /v1/clickstream` - Get click stream data
- `GET /v1/clickstream/{route_id}` - Get route-specific analytics
- `GET /v1/clickstream/stats` - Get aggregated statistics

**Public Endpoints:**
- `GET /public/health` - Health check
- `GET /public/metrics` - Service metrics

## 🧪 Testing APIs

### Interactive Documentation

- **Swagger UI**: `http://localhost:8081/swagger-ui`
- **OpenAPI Spec**: `http://localhost:8081/api-doc/openapi.json`

### Authentication in Documentation

The Swagger UI supports JWT authentication:

1. Click "Authorize" button
2. Enter JWT token: `Bearer your_jwt_token`
3. All authenticated endpoints will use the token

## 📖 Additional Documentation

### Core Documentation
- [Click Router API](click-router/) - Complete Click Router API documentation
- [Click Aggregator API](click-aggregator/) - Complete Click Aggregator API documentation
- [Authentication](authentication/) - Authentication implementation details
- [Data Models](data-models/) - Detailed data model documentation

### API Guides
- [Getting Started](getting-started/) - Quick start with APIs
- [Integration Examples](integration/) - Real-world integration examples
- [SDK Documentation](sdk/) - Client SDK documentation
- [Webhooks](webhooks/) - Webhook configuration and usage

## 🔗 Quick Links

- [Click Router API Documentation](click-router/)
- [Click Aggregator API Documentation](click-aggregator/)
- [Authentication Guide](authentication/)
- [Data Models Reference](data-models/)
- [Integration Examples](integration/)

---

**Need help with the APIs?** Check out our [integration examples](integration/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
