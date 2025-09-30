# RouteDto Documentation

This document provides comprehensive information about the `RouteDto` used in the routes controller.

## Overview

The `RouteDto` is a Data Transfer Object (DTO) that provides a clean, simplified API interface for routes. It simplifies complex nested structures and provides better API usability while maintaining all essential route information.

## DTO Structure

### Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `switch` | `String` | Route switch identifier | `"main"` |
| `link` | `String` | Route link/URL | `"https://example.com"` |
| `dest` | `Option<String>` | Destination URL (optional) | `"https://target.com"` |
| `dest_format` | `String` | Destination format | `"Http"` or `"Native"` |
| `code` | `Option<u16>` | HTTP status code (optional) | `301` or `302` |
| `ttl` | `Option<u128>` | Time to live in seconds (optional) | `3600` |
| `status` | `String` | Route status | `"Active"` or `"Blocked: reason"` |
| `terminal` | `String` | Routing terminal type | `"External"`, `"Internal"`, or `"Middleware"` |
| `properties` | `RoutePropertiesDto` | Route properties | See RoutePropertiesDto section |

### API Response Example

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
    "creator_id": "user-789",
    "workspace_id": "workspace-101",
    "scripts": ["script1.js", "script2.js"],
    "tags": ["api", "v1"],
    "custom": {"key": "value"},
    "native": {"feature": "enabled"},
    "bundling": {"enabled": true},
    "opengraph": true,
    "allow_debug": false
  }
}
```

## RoutePropertiesDto

### Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `route_id` | `Option<String>` | Route ID (optional) | `"route-123"` |
| `domain_id` | `Option<String>` | Domain ID (optional) | `"domain-456"` |
| `owner_id` | `Option<String>` | Owner ID (optional) | `"user-789"` |
| `creator_id` | `Option<String>` | Creator ID (optional) | `"user-789"` |
| `workspace_id` | `Option<String>` | Workspace ID (optional) | `"workspace-101"` |
| `scripts` | `Option<Vec<String>>` | Scripts (optional) | `["script1.js", "script2.js"]` |
| `tags` | `Option<Vec<String>>` | Tags (optional) | `["api", "v1"]` |
| `custom` | `Option<Value>` | Custom properties (optional) | `{"key": "value"}` |
| `native` | `Option<Value>` | Native properties (optional) | `{"feature": "enabled"}` |
| `bundling` | `Option<Value>` | Bundling properties (optional) | `{"enabled": true}` |
| `opengraph` | `bool` | OpenGraph enabled | `true` or `false` |
| `allow_debug` | `bool` | Debug allowed | `true` or `false` |

## API Endpoints

### GET /v1/routes/{switch}/{domain}/{path}

#### Description
Retrieves routing information for a specific switch, domain, and path combination.

#### Parameters
- **switch** (path, required): The switch identifier
  - Type: `String`
  - Example: `"main"`
- **domain** (path, required): The domain name
  - Type: `String`
  - Example: `"example.com"`
- **path** (path, required): The path
  - Type: `String`
  - Example: `"/api/v1"`

#### Response Example (200 OK)
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
    "creator_id": "user-789",
    "workspace_id": "workspace-101",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "native": {"feature": "enabled"},
    "bundling": {"enabled": true},
    "opengraph": true,
    "allow_debug": false
  }
}
```

### POST /v1/routes/{switch}/{domain}/{path}

#### Description
Creates a new routing entry with the provided configuration.

#### Parameters
- **switch** (path, required): The switch identifier
  - Type: `String`
  - Example: `"main"`
- **domain** (path, required): The domain name
  - Type: `String`
  - Example: `"example.com"`
- **path** (path, required): The path
  - Type: `String`
  - Example: `"/api/v1"`

#### Request Body
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
    "creator_id": "user-789",
    "workspace_id": "workspace-101",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "native": {"feature": "enabled"},
    "bundling": {"enabled": true},
    "opengraph": true,
    "allow_debug": false
  }
}
```

#### Response Example (201 Created)
```json
{
  "message": "Route created successfully",
  "route": {
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
      "creator_id": "user-789",
      "workspace_id": "workspace-101",
      "scripts": ["script1.js"],
      "tags": ["api"],
      "custom": {"key": "value"},
      "native": {"feature": "enabled"},
      "bundling": {"enabled": true},
      "opengraph": true,
      "allow_debug": false
    }
  }
}
```

### PUT /v1/routes/{switch}/{domain}/{path}

#### Description
Updates an existing routing entry with the provided configuration.

#### Parameters
- **switch** (path, required): The switch identifier
  - Type: `String`
  - Example: `"main"`
- **domain** (path, required): The domain name
  - Type: `String`
  - Example: `"example.com"`
- **path** (path, required): The path
  - Type: `String`
  - Example: `"/api/v1"`

#### Request Body
```json
{
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
    "creator_id": "user-789",
    "workspace_id": "workspace-101",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "native": {"feature": "enabled"},
    "bundling": {"enabled": true},
    "opengraph": true,
    "allow_debug": false
  }
}
```

#### Response Example (200 OK)
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
    "creator_id": "user-789",
    "workspace_id": "workspace-101",
    "scripts": ["script1.js"],
    "tags": ["api"],
    "custom": {"key": "value"},
    "native": {"feature": "enabled"},
    "bundling": {"enabled": true},
    "opengraph": true,
    "allow_debug": false
  }
}
```

### DELETE /v1/routes/{switch}/{domain}/{path}

#### Description
Deletes an existing routing entry.

#### Parameters
- **switch** (path, required): The switch identifier
  - Type: `String`
  - Example: `"main"`
- **domain** (path, required): The domain name
  - Type: `String`
  - Example: `"example.com"`
- **path** (path, required): The path
  - Type: `String`
  - Example: `"/api/v1"`

#### Response Example (200 OK)
```json
{
  "message": "Route deleted successfully",
  "switch": "main",
  "domain": "example.com",
  "path": "/api/v1"
}
```

### POST /v1/routes/bulk

#### Description
Creates multiple routing entries in a single request.

#### Request Body
```json
[
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
      "creator_id": "user-789",
      "workspace_id": "workspace-101",
      "scripts": ["script1.js"],
      "tags": ["api"],
      "custom": {"key": "value"},
      "native": {"feature": "enabled"},
      "bundling": {"enabled": true},
      "opengraph": true,
      "allow_debug": false
    }
  },
  {
    "switch": "admin",
    "link": "https://admin.example.com",
    "dest": "https://admin.target.com",
    "dest_format": "Http",
    "code": 302,
    "ttl": 7200,
    "status": "Active",
    "terminal": "Internal",
    "properties": {
      "route_id": "route-456",
      "domain_id": "domain-789",
      "owner_id": "user-101",
      "creator_id": "user-101",
      "workspace_id": "workspace-202",
      "scripts": ["admin.js"],
      "tags": ["admin", "v2"],
      "custom": {"admin": true},
      "native": {"admin_feature": "enabled"},
      "bundling": {"enabled": false},
      "opengraph": false,
      "allow_debug": true
    }
  }
]
```

#### Response Example (201 Created)
```json
{
  "message": "Routes created successfully",
  "count": 2,
  "routes": [
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
        "creator_id": "user-789",
        "workspace_id": "workspace-101",
        "scripts": ["script1.js"],
        "tags": ["api"],
        "custom": {"key": "value"},
        "native": {"feature": "enabled"},
        "bundling": {"enabled": true},
        "opengraph": true,
        "allow_debug": false
      }
    },
    {
      "switch": "admin",
      "link": "https://admin.example.com",
      "dest": "https://admin.target.com",
      "dest_format": "Http",
      "code": 302,
      "ttl": 7200,
      "status": "Active",
      "terminal": "Internal",
      "properties": {
        "route_id": "route-456",
        "domain_id": "domain-789",
        "owner_id": "user-101",
        "creator_id": "user-101",
        "workspace_id": "workspace-202",
        "scripts": ["admin.js"],
        "tags": ["admin", "v2"],
        "custom": {"admin": true},
        "native": {"admin_feature": "enabled"},
        "bundling": {"enabled": false},
        "opengraph": false,
        "allow_debug": true
      }
    }
  ]
}
```

### PUT /v1/routes/bulk

#### Description
Updates multiple routing entries in a single request.

#### Request Body
```json
[
  {
    "switch": "main",
    "link": "https://updated.example.com",
    "dest": "https://updated.target.com",
    "dest_format": "Http",
    "code": 301,
    "ttl": 3600,
    "status": "Active",
    "terminal": "External",
    "properties": {
      "route_id": "route-123",
      "domain_id": "domain-456",
      "owner_id": "user-789",
      "creator_id": "user-789",
      "workspace_id": "workspace-101",
      "scripts": ["script1.js", "script2.js"],
      "tags": ["api", "updated"],
      "custom": {"key": "updated_value"},
      "native": {"feature": "enabled"},
      "bundling": {"enabled": true},
      "opengraph": true,
      "allow_debug": false
    }
  }
]
```

#### Response Example (200 OK)
```json
{
  "message": "Routes updated successfully",
  "count": 1,
  "routes": [
    {
      "switch": "main",
      "link": "https://updated.example.com",
      "dest": "https://updated.target.com",
      "dest_format": "Http",
      "code": 301,
      "ttl": 3600,
      "status": "Active",
      "terminal": "External",
      "properties": {
        "route_id": "route-123",
        "domain_id": "domain-456",
        "owner_id": "user-789",
        "creator_id": "user-789",
        "workspace_id": "workspace-101",
        "scripts": ["script1.js", "script2.js"],
        "tags": ["api", "updated"],
        "custom": {"key": "updated_value"},
        "native": {"feature": "enabled"},
        "bundling": {"enabled": true},
        "opengraph": true,
        "allow_debug": false
      }
    }
  ]
}
```

### DELETE /v1/routes/bulk

#### Description
Deletes multiple routing entries in a single request.

#### Request Body
```json
[
  {
    "switch": "main",
    "domain": "example.com",
    "path": "/api/v1"
  },
  {
    "switch": "admin",
    "domain": "admin.example.com",
    "path": "/dashboard"
  }
]
```

#### Response Example (200 OK)
```json
{
  "message": "Routes deleted successfully",
  "count": 2,
  "routes": [
    {
      "switch": "main",
      "domain": "example.com",
      "path": "/api/v1"
    },
    {
      "switch": "admin",
      "domain": "admin.example.com",
      "path": "/dashboard"
    }
  ]
}
```

## Field Descriptions

### Core Route Fields

#### switch
- **Type**: `String`
- **Description**: Route switch identifier
- **Required**: Yes
- **Example**: `"main"`
- **Validation**: Must not be empty

#### link
- **Type**: `String`
- **Description**: Route link/URL
- **Required**: Yes
- **Example**: `"https://example.com"`
- **Validation**: Must not be empty, should be valid URL

#### dest
- **Type**: `Option<String>`
- **Description**: Destination URL (optional)
- **Required**: No
- **Example**: `"https://target.com"`
- **Validation**: Should be valid URL if provided

#### dest_format
- **Type**: `String`
- **Description**: Destination format
- **Required**: Yes
- **Values**: `"Http"` or `"Native"`
- **Default**: `"Http"`

#### code
- **Type**: `Option<u16>`
- **Description**: HTTP status code (optional)
- **Required**: No
- **Example**: `301`, `302`, `200`
- **Validation**: Should be valid HTTP status code if provided

#### ttl
- **Type**: `Option<u128>`
- **Description**: Time to live in seconds (optional)
- **Required**: No
- **Example**: `3600` (1 hour)
- **Validation**: Should be positive number if provided

#### status
- **Type**: `String`
- **Description**: Route status
- **Required**: Yes
- **Values**: `"Active"` or `"Blocked: reason"`
- **Default**: `"Active"`

#### terminal
- **Type**: `String`
- **Description**: Routing terminal type
- **Required**: Yes
- **Values**: `"External"`, `"Internal"`, or `"Middleware"`
- **Default**: `"External"`

### Status Values

#### Active Status
- **"Active"**: Route is active and functional
- **"Blocked: reason"**: Route is blocked with specific reason

#### Status Examples
```json
{
  "status": "Active"
}
```

```json
{
  "status": "Blocked: Maintenance"
}
```

```json
{
  "status": "Blocked: Unknown"
}
```

### Terminal Types

#### External
- **Description**: Route terminates externally
- **Use Case**: External redirects, API calls
- **Example**: Redirect to external website

#### Internal
- **Description**: Route terminates internally
- **Use Case**: Internal routing, microservices
- **Example**: Route to internal service

#### Middleware
- **Description**: Route uses middleware
- **Use Case**: Processing, transformation
- **Example**: Authentication, logging

### Destination Formats

#### Http
- **Description**: HTTP-based destination
- **Use Case**: Web redirects, API calls
- **Example**: `"https://api.example.com"`

#### Native
- **Description**: Native application destination
- **Use Case**: Mobile apps, desktop applications
- **Example**: `"myapp://action"`

## DTO Methods

### Constructor Methods
```rust
// Create with all parameters
let dto = RouteDto::new(
    "main".to_string(),
    "https://example.com".to_string(),
    Some("https://target.com".to_string()),
    "Http".to_string(),
    Some(301),
    Some(3600),
    "Active".to_string(),
    "External".to_string(),
    RoutePropertiesDto::default()
);

// Create with default values
let dto = RouteDto::default();
```

### Builder Pattern
```rust
let dto = RouteDto::default()
    .switch("main".to_string())
    .link("https://example.com".to_string())
    .dest(Some("https://target.com".to_string()))
    .dest_format("Http".to_string())
    .code(Some(301))
    .ttl(Some(3600))
    .status("Active".to_string())
    .terminal("External".to_string())
    .properties(RoutePropertiesDto::default());
```

### Validation Methods
```rust
// Check if DTO is valid
if dto.is_valid() {
    println!("DTO is valid");
}

// Get switch value
let switch = dto.get_switch(); // "main"

// Get link value
let link = dto.get_link(); // "https://example.com"

// Check if destination is set
if dto.has_destination() {
    println!("Destination: {:?}", dto.get_destination());
}

// Check if status is active
if dto.is_active() {
    println!("Route is active");
}

// Check if status is blocked
if dto.is_blocked() {
    println!("Route is blocked");
}

// Get terminal type
let terminal = dto.get_terminal(); // "External"

// Check terminal type
if dto.is_external() {
    println!("External route");
} else if dto.is_internal() {
    println!("Internal route");
} else if dto.is_middleware() {
    println!("Middleware route");
}
```

## Conversion Methods

### From Route to RouteDto
```rust
use crate::dto::RouteDto;
use crate::model::route::Route;

// Convert from Route
let route = Route::new(/* ... */);
let dto = RouteDto::from(route);

// Convert from &Route
let dto = RouteDto::from(&route);
```

### From RouteDto to Route
```rust
use crate::dto::RouteDto;

let dto = RouteDto::new(/* ... */);
let route: Route = dto.into();
```

## Error Handling

### Validation Errors
```rust
// Check if DTO is valid before processing
if !dto.is_valid() {
    return Err("Invalid route DTO");
}
```

### Conversion Errors
```rust
// Handle conversion errors gracefully
match RouteDto::from(route) {
    Ok(dto) => Ok(dto),
    Err(e) => Err(format!("Conversion failed: {}", e)),
}
```

## Best Practices

### 1. Data Validation
- Always validate DTO before processing
- Check required fields (switch, link)
- Validate URL formats when applicable
- Ensure status values are valid

### 2. Conversion
- Use references when possible (`&Route`)
- Handle conversion errors gracefully
- Implement efficient conversion methods
- Cache frequently accessed routes

### 3. API Design
- Use consistent field naming
- Provide comprehensive documentation
- Include examples in API responses
- Implement proper error handling

### 4. Performance
- Use efficient string operations
- Implement proper caching strategies
- Consider route complexity for performance
- Monitor conversion overhead

## Integration with OpenAPI

The `RouteDto` is fully integrated with OpenAPI documentation:

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
        let dto = RouteDto::new(
            "main".to_string(),
            "https://example.com".to_string(),
            Some("https://target.com".to_string()),
            "Http".to_string(),
            Some(301),
            Some(3600),
            "Active".to_string(),
            "External".to_string(),
            RoutePropertiesDto::default()
        );
        
        assert!(dto.is_valid());
        assert_eq!(dto.switch, "main");
        assert_eq!(dto.link, "https://example.com");
    }

    #[test]
    fn test_conversion() {
        let route = Route::new(/* ... */);
        let dto = RouteDto::from(&route);
        assert!(dto.is_valid());
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_get_route() {
    let response = get_route(&mut request, &mut depot, &mut response).await;
    assert_eq!(response.status_code, 200);
    
    let body: RouteDto = response.json().await.unwrap();
    assert!(body.is_valid());
}
```

This comprehensive DTO implementation provides a clean, efficient, and well-documented interface for route management in the API.
