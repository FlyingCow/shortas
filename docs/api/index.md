---
layout: vector-theme
title: API Reference
permalink: /api/
---

<div class="hero-section">
  <h1>API Reference</h1>
  <p class="lead">This section provides comprehensive documentation for all Shortas APIs, including authentication, endpoints, data models, and integration examples. All APIs follow RESTful principles and support JSON payloads.</p>
</div>

## 🔗 API Overview

Shortas provides two main APIs for managing routes and accessing analytics:

<div class="feature-grid">
  <div class="feature-card">
    <div class="feature-icon">🔧</div>
    <h3>Click Router API</h3>
    <p>High-performance, secure API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and support for multiple database backends.</p>
    <p class="feature-meta"><strong>Base URL:</strong> <code>http://localhost:8081</code></p>
    <p class="feature-meta"><strong>Features:</strong> Route Management, SSL Certificates, User Settings</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-router-api" class="btn" target="_blank">Learn More</a>
  </div>
  
  <div class="feature-card">
    <div class="feature-icon">📊</div>
    <h3>Click Aggregator API</h3>
    <p>High-performance, secure click aggregation API with JWT authentication via Keycloak, comprehensive OpenAPI documentation, and ClickHouse integration for analytics.</p>
    <p class="feature-meta"><strong>Base URL:</strong> <code>http://localhost:8082</code></p>
    <p class="feature-meta"><strong>Features:</strong> Analytics, Reporting, Click Stream Data</p>
    <a href="https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator-api" class="btn" target="_blank">Learn More</a>
  </div>
</div>

## 🔐 Authentication

All protected endpoints require JWT authentication via Keycloak:

<div class="card">
  <div class="card-header">Authentication Methods</div>
  
  <h4>JWT Bearer Token</h4>
  <p>Include the JWT token in the Authorization header:</p>
  <pre><code>Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...</code></pre>
  
  <h4>RPT Token (Fine-grained Authorization)</h4>
  <p>For fine-grained authorization with UMA:</p>
  <pre><code>Authorization: Bearer rpt_token_for_uma_authorization</code></pre>
</div>

### Required Permissions

<div class="card">
  <div class="card-header">Permission Matrix</div>
  <table>
    <tr>
      <th>Resource</th>
      <th>Permission</th>
      <th>Description</th>
    </tr>
    <tr>
      <td>Routes</td>
      <td><code>read:routes</code></td>
      <td>Read route information</td>
    </tr>
    <tr>
      <td>Routes</td>
      <td><code>write:routes</code></td>
      <td>Create/update routes</td>
    </tr>
    <tr>
      <td>Routes</td>
      <td><code>delete:routes</code></td>
      <td>Delete routes</td>
    </tr>
    <tr>
      <td>Certificates</td>
      <td><code>read:certificates</code></td>
      <td>Read certificate information</td>
    </tr>
    <tr>
      <td>Certificates</td>
      <td><code>write:certificates</code></td>
      <td>Create/update certificates</td>
    </tr>
    <tr>
      <td>Certificates</td>
      <td><code>delete:certificates</code></td>
      <td>Delete certificates</td>
    </tr>
    <tr>
      <td>User Settings</td>
      <td><code>read:user_settings</code></td>
      <td>Read user settings</td>
    </tr>
    <tr>
      <td>User Settings</td>
      <td><code>write:user_settings</code></td>
      <td>Update user settings</td>
    </tr>
  </table>
</div>

## 📊 Data Models

Understand the core data structures used across the Shortas APIs:

<div class="card">
  <div class="card-header">RouteDto</div>
  <p>Represents a URL route configuration:</p>
  <pre><code>{
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
}</code></pre>
</div>

<div class="card">
  <div class="card-header">KeycertDto</div>
  <p>Represents SSL certificate data:</p>
  <pre><code>{
  "key": "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----",
  "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
  "ocsp_resp": "-----BEGIN OCSP RESPONSE-----\n...\n-----END OCSP RESPONSE-----"
}</code></pre>
</div>

<div class="card">
  <div class="card-header">UserSettingsDto</div>
  <p>Represents user-specific settings:</p>
  <pre><code>{
  "email": "user@example.com",
  "status": "active",
  "debug": false,
  "overflow": true,
  "skip_tracking": ["tracking", "analytics"],
  "allowed_request_params": ["utm_source", "utm_medium"],
  "allowed_destination_params": ["redirect", "target"]
}</code></pre>
</div>

## 🔗 API Endpoints

### Click Router API Endpoints

<div class="card">
  <div class="card-header">Routes Management</div>
  <table>
    <tr>
      <th>Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/routes/{switch}/{domain}/{path}</code></td>
      <td>Get route information</td>
    </tr>
    <tr>
      <td><code>POST</code></td>
      <td><code>/v1/routes/{switch}/{domain}/{path}</code></td>
      <td>Create new route</td>
    </tr>
    <tr>
      <td><code>PUT</code></td>
      <td><code>/v1/routes/{switch}/{domain}/{path}</code></td>
      <td>Update existing route</td>
    </tr>
    <tr>
      <td><code>DELETE</code></td>
      <td><code>/v1/routes/{switch}/{domain}/{path}</code></td>
      <td>Delete route</td>
    </tr>
  </table>
</div>

<div class="card">
  <div class="card-header">Bulk Operations</div>
  <table>
    <tr>
      <th>Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>POST</code></td>
      <td><code>/v1/routes/bulk</code></td>
      <td>Create multiple routes</td>
    </tr>
    <tr>
      <td><code>PUT</code></td>
      <td><code>/v1/routes/bulk</code></td>
      <td>Update multiple routes</td>
    </tr>
    <tr>
      <td><code>DELETE</code></td>
      <td><code>/v1/routes/bulk</code></td>
      <td>Delete multiple routes</td>
    </tr>
  </table>
</div>

<div class="card">
  <div class="card-header">SSL Certificate Management</div>
  <table>
    <tr>
      <th>Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/certificates/{domain}</code></td>
      <td>Get certificate information</td>
    </tr>
    <tr>
      <td><code>POST</code></td>
      <td><code>/v1/certificates/{domain}</code></td>
      <td>Create new certificate</td>
    </tr>
    <tr>
      <td><code>PUT</code></td>
      <td><code>/v1/certificates/{domain}</code></td>
      <td>Update existing certificate</td>
    </tr>
    <tr>
      <td><code>DELETE</code></td>
      <td><code>/v1/certificates/{domain}</code></td>
      <td>Delete certificate</td>
    </tr>
  </table>
</div>

<div class="card">
  <div class="card-header">User Settings Management</div>
  <table>
    <tr>
      <th>Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/user-settings/{user_id}</code></td>
      <td>Get user settings</td>
    </tr>
    <tr>
      <td><code>POST</code></td>
      <td><code>/v1/user-settings/{user_id}</code></td>
      <td>Create user settings</td>
    </tr>
    <tr>
      <td><code>PUT</code></td>
      <td><code>/v1/user-settings/{user_id}</code></td>
      <td>Update user settings</td>
    </tr>
    <tr>
      <td><code>DELETE</code></td>
      <td><code>/v1/user-settings/{user_id}</code></td>
      <td>Delete user settings</td>
    </tr>
  </table>
</div>

### Click Aggregator API Endpoints

<div class="card">
  <div class="card-header">Analytics Endpoints</div>
  <p>The Click Aggregator API provides analytics and reporting endpoints for click stream data stored in ClickHouse:</p>
  <table>
    <tr>
      <th>Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/daily-stats</code></td>
      <td>Get daily statistics</td>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/geographic-stats</code></td>
      <td>Get geographic statistics</td>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/device-stats</code></td>
      <td>Get device statistics</td>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/browser-stats</code></td>
      <td>Get browser statistics</td>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/traffic-type-stats</code></td>
      <td>Get traffic type statistics</td>
    </tr>
    <tr>
      <td><code>GET</code></td>
      <td><code>/v1/clickstream/route-performance</code></td>
      <td>Get route performance metrics</td>
    </tr>
  </table>
</div>

### Public Endpoints

<div class="card">
  <div class="card-header">Public Endpoints</div>
  <ul>
    <li><code>GET /health</code> - Health check</li>
    <li><code>GET /swagger-ui</code> - Interactive API documentation</li>
    <li><code>GET /api-doc/openapi.json</code> - OpenAPI specification</li>
  </ul>
</div>

## 📚 OpenAPI Documentation

<div class="card">
  <div class="card-header">Interactive Documentation</div>
  <p>Access interactive API documentation:</p>
  <ul>
    <li><strong>Swagger UI:</strong> <code>http://localhost:8081/swagger-ui</code> (Router API)</li>
    <li><strong>Swagger UI:</strong> <code>http://localhost:8082/swagger-ui</code> (Aggregator API)</li>
    <li><strong>OpenAPI Spec:</strong> <code>http://localhost:8081/api-doc/openapi.json</code></li>
    <li><strong>OpenAPI Spec:</strong> <code>http://localhost:8082/api-doc/openapi.json</code></li>
  </ul>
</div>

### API Information

<div class="card">
  <div class="card-header">API Metadata</div>
  <ul>
    <li><strong>Title:</strong> Shortas APIs</li>
    <li><strong>Version:</strong> 0.1.0</li>
    <li><strong>Description:</strong> High-performance click aggregation and routing APIs</li>
    <li><strong>License:</strong> MIT</li>
    <li><strong>Contact:</strong> API Support (support@shortas.com)</li>
  </ul>
</div>

### Authentication in Documentation

<div class="alert alert-info">
  <strong>Using Swagger UI:</strong>
  <ol>
    <li>Click "Authorize" button</li>
    <li>Enter JWT token: <code>Bearer your_jwt_token</code></li>
    <li>All authenticated endpoints will use the token</li>
  </ol>
</div>

## 🚨 Error Handling

<div class="card">
  <div class="card-header">Error Response Format</div>
  <p>All errors follow a consistent format:</p>
  <pre><code>{
  "status_code": 400,
  "error": "Validation Error",
  "details": "Invalid input data: field 'email' is required"
}</code></pre>
</div>

### HTTP Status Codes

<div class="card">
  <div class="card-header">Status Code Reference</div>
  <table>
    <tr>
      <th>Code</th>
      <th>Description</th>
    </tr>
    <tr>
      <td><code>200</code></td>
      <td>Success</td>
    </tr>
    <tr>
      <td><code>201</code></td>
      <td>Created</td>
    </tr>
    <tr>
      <td><code>400</code></td>
      <td>Bad Request</td>
    </tr>
    <tr>
      <td><code>401</code></td>
      <td>Unauthorized</td>
    </tr>
    <tr>
      <td><code>403</code></td>
      <td>Forbidden</td>
    </tr>
    <tr>
      <td><code>404</code></td>
      <td>Not Found</td>
    </tr>
    <tr>
      <td><code>409</code></td>
      <td>Conflict</td>
    </tr>
    <tr>
      <td><code>422</code></td>
      <td>Unprocessable Entity</td>
    </tr>
    <tr>
      <td><code>429</code></td>
      <td>Too Many Requests</td>
    </tr>
    <tr>
      <td><code>500</code></td>
      <td>Internal Server Error</td>
    </tr>
  </table>
</div>

## 🔒 Security Features

<div class="feature-grid">
  <div class="card">
    <div class="card-header">JWT Authentication</div>
    <p>Secure token-based authentication using Keycloak</p>
  </div>
  
  <div class="card">
    <div class="card-header">Role-Based Access Control</div>
    <p>Fine-grained permissions for resources and actions</p>
  </div>
  
  <div class="card">
    <div class="card-header">Rate Limiting</div>
    <p>Protection against abuse and DDoS attacks</p>
  </div>
  
  <div class="card">
    <div class="card-header">Input Validation</div>
    <p>Comprehensive request validation</p>
  </div>
</div>

### Security Headers

The APIs automatically include security headers:

```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
```

## 📝 Example Requests

<div class="card">
  <div class="card-header">Create Route Example</div>
  <pre><code>curl -X POST http://localhost:8081/v1/routes/main/example.com/test \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "dest": "https://target.com",
    "code": 301,
    "ttl": 3600
  }'</code></pre>
</div>

<div class="card">
  <div class="card-header">Get Daily Stats Example</div>
  <pre><code>curl -X GET \
  "http://localhost:8082/v1/clickstream/daily-stats?fromDate=2024-01-01&toDate=2024-01-31" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"</code></pre>
</div>

---

**Next Steps**: Explore the detailed documentation for the [Click Router API](https://github.com/FlyingCow/shortas/tree/main/redirect/click-router-api) or the [Click Aggregator API](https://github.com/FlyingCow/shortas/tree/main/redirect/click-aggregator-api).
