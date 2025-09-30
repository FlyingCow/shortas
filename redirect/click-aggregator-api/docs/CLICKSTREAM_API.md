# ClickStream Analytics API

This document describes the ClickStream Analytics API for querying click stream data from ClickHouse.

## Overview

The ClickStream API provides access to analytics data stored in ClickHouse, allowing you to query click stream information with various filters and pagination options.

## Base URL

```
GET /v1/clickstream
```

## Authentication

All endpoints require JWT Bearer token authentication:

```
Authorization: Bearer <your-jwt-token>
```

## Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `owner_id` | string | No | Filter by owner ID |
| `creator_id` | string | No | Filter by creator ID |
| `route_id` | string | No | Filter by route ID |
| `workspace_id` | string | No | Filter by workspace ID |
| `created_from` | string | No | Filter by creation date from (ISO 8601 format) |
| `created_to` | string | No | Filter by creation date to (ISO 8601 format) |
| `limit` | integer | No | Maximum number of results (default: 100) |
| `offset` | integer | No | Number of results to skip (default: 0) |

## Response Format

### Success Response (200 OK)

```json
{
  "items": [
    {
      "id": "click_123456",
      "owner_id": "user_789",
      "creator_id": "user_456",
      "route_id": "route_123",
      "workspace_id": "workspace_456",
      "created": "2023-12-01T10:30:00Z",
      "dest": "https://example.com/destination",
      "ip": "192.168.1.100",
      "continent": "North America",
      "country": "United States",
      "location": "New York",
      "os_family": "Windows",
      "os_version": "10",
      "user_agent_family": "Chrome",
      "user_agent_version": "120.0",
      "device_brand": "Dell",
      "device_family": "Desktop",
      "device_model": "OptiPlex",
      "session_first": "2023-12-01T10:00:00Z",
      "session_clicks": 5,
      "is_unique": true,
      "is_bot": false
    }
  ],
  "total": 1500,
  "offset": 0,
  "limit": 100,
  "has_more": true
}
```

### Error Responses

#### 400 Bad Request
```json
{
  "error": "Invalid query parameters",
  "details": "Invalid created_from date format. Use ISO 8601 format (e.g., 2023-01-01T00:00:00Z)"
}
```

#### 500 Internal Server Error
```json
{
  "error": "Failed to query click stream",
  "details": "Connection to ClickHouse failed"
}
```

## Data Model

### ClickStreamItem

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier for the click stream item |
| `owner_id` | string | Owner of the route |
| `creator_id` | string | Creator of the route |
| `route_id` | string | Route identifier |
| `workspace_id` | string | Workspace identifier |
| `created` | datetime | Timestamp when the click occurred (ISO 8601) |
| `dest` | string | Destination URL |
| `ip` | string | IP address of the clicker |
| `continent` | string? | Geographic continent (optional) |
| `country` | string? | Geographic country (optional) |
| `location` | string? | Geographic location (optional) |
| `os_family` | string? | Operating system family (optional) |
| `os_version` | string? | Operating system version (optional) |
| `user_agent_family` | string? | User agent family (optional) |
| `user_agent_version` | string? | User agent version (optional) |
| `device_brand` | string? | Device brand (optional) |
| `device_family` | string? | Device family (optional) |
| `device_model` | string? | Device model (optional) |
| `session_first` | datetime? | First session timestamp (optional) |
| `session_clicks` | integer? | Number of clicks in session (optional) |
| `is_unique` | boolean | Whether this is a unique click |
| `is_bot` | boolean | Whether this click is from a bot |

## Examples

### Basic Query
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.example.com/v1/clickstream?limit=50"
```

### Filter by Owner
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.example.com/v1/clickstream?owner_id=user_123&limit=100"
```

### Date Range Filter
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.example.com/v1/clickstream?created_from=2023-12-01T00:00:00Z&created_to=2023-12-31T23:59:59Z"
```

### Pagination
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.example.com/v1/clickstream?limit=50&offset=100"
```

### Complex Filter
```bash
curl -H "Authorization: Bearer <token>" \
  "https://api.example.com/v1/clickstream?owner_id=user_123&route_id=route_456&created_from=2023-12-01T00:00:00Z&limit=25"
```

## ClickHouse Configuration

The API connects to ClickHouse with the following configuration:

- **URL**: `http://clickhouse:8123`
- **User**: `default`
- **Password**: `clickhouse`
- **Database**: `shortas`
- **Table**: `click_stream`

## Ordering

Results are always ordered by `id` in descending order (newest first).

## Rate Limiting

The API implements rate limiting to prevent abuse. See the main API documentation for rate limiting details.

## Error Handling

The API provides detailed error messages for common issues:

- **Invalid date format**: Use ISO 8601 format (e.g., `2023-12-01T10:30:00Z`)
- **Invalid parameters**: Check parameter names and types
- **Database errors**: Connection or query issues with ClickHouse
- **Authentication errors**: Invalid or missing JWT token

## Performance Considerations

- Use appropriate `limit` values (recommended: 100-1000)
- Use date range filters to limit data scope
- Consider using specific filters (`owner_id`, `route_id`) for better performance
- Large result sets may take longer to process

## Security

- All endpoints require valid JWT authentication
- User access is controlled by JWT token claims
- Sensitive data (IP addresses) should be handled according to privacy policies
- Rate limiting prevents abuse

## Monitoring

The API provides metrics for:
- Request count and response times
- Error rates by type
- Database query performance
- Authentication success/failure rates

## Support

For issues or questions about the ClickStream API:
- Check the error response details
- Verify your JWT token is valid and not expired
- Ensure your query parameters are correctly formatted
- Contact the API team for database connectivity issues

