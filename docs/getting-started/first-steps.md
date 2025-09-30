---
layout: page
title: First Steps
permalink: /getting-started/first-steps/
---

# First Steps with Shortas

This guide will walk you through creating your first shortened URL and understanding the basic concepts of Shortas.

## 🚀 Quick Start

### 1. Start the Services

```bash
# Start all services
make dev-start

# Check that services are running
make health-check
```

### 2. Create Your First Route

```bash
# Create a simple route using the API
curl -X POST http://localhost:8081/v1/routes/main/example.com/test \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_jwt_token" \
  -d '{
    "switch": "main",
    "link": "test",
    "dest": "https://example.com",
    "dest_format": "Http",
    "code": 302,
    "status": "Active",
    "terminal": "External",
    "policy": {
      "type": "Basic"
    },
    "properties": {
      "route_id": "route_123",
      "domain_id": "domain_456",
      "owner_id": "user_789"
    }
  }'
```

### 3. Test the Redirect

```bash
# Test the redirect
curl -I http://localhost:8080/test

# You should see a 302 redirect response
```

## 📝 Understanding Routes

### Route Structure

A route in Shortas consists of several key components:

```json
{
  "switch": "main",           // Route identifier
  "link": "test",            // URL path
  "dest": "https://example.com", // Destination URL
  "dest_format": "Http",     // Format type
  "code": 302,              // HTTP status code
  "status": "Active",       // Route status
  "terminal": "External",    // Routing terminal
  "policy": {               // Routing policy
    "type": "Basic"
  },
  "properties": {           // Metadata
    "route_id": "route_123",
    "domain_id": "domain_456",
    "owner_id": "user_789"
  }
}
```

### Route Components Explained

- **switch**: Identifies the route group (e.g., "main", "mobile", "desktop")
- **link**: The shortened URL path (e.g., "test" for `/test`)
- **dest**: The destination URL to redirect to
- **dest_format**: Format type ("Http", "Native")
- **code**: HTTP status code (301, 302, etc.)
- **status**: Route status ("Active", "Blocked")
- **terminal**: Where the route terminates ("External", "Internal", "Middleware")
- **policy**: Routing rules and conditions
- **properties**: Additional metadata and settings

## 🔧 Basic Route Types

### Simple Redirect

```json
{
  "switch": "main",
  "link": "simple",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Basic"
  }
}
```

### Permanent Redirect

```json
{
  "switch": "main",
  "link": "permanent",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 301,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Basic"
  }
}
```

### Conditional Routing

```json
{
  "switch": "main",
  "link": "conditional",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Conditional",
    "conditions": [
      {
        "key": "mobile",
        "condition": {
          "device": {
            "type": "mobile"
          }
        }
      }
    ]
  }
}
```

## 🎯 Advanced Routing Examples

### Device-Based Routing

```json
{
  "switch": "main",
  "link": "smart",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Conditional",
    "conditions": [
      {
        "key": "mobile_route",
        "condition": {
          "device": {
            "type": "mobile"
          }
        }
      },
      {
        "key": "desktop_route",
        "condition": {
          "device": {
            "type": "desktop"
          }
        }
      }
    ]
  }
}
```

### Geographic Routing

```json
{
  "switch": "main",
  "link": "geo",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Conditional",
    "conditions": [
      {
        "key": "us_route",
        "condition": {
          "country": {
            "code": "US"
          }
        }
      },
      {
        "key": "eu_route",
        "condition": {
          "country": {
            "code": "DE"
          }
        }
      }
    ]
  }
}
```

### Time-Based Routing

```json
{
  "switch": "main",
  "link": "time",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "status": "Active",
  "terminal": "External",
  "policy": {
    "type": "Conditional",
    "conditions": [
      {
        "key": "business_hours",
        "condition": {
          "time": {
            "hour": 9
          }
        }
      }
    ]
  }
}
```

## 📊 Analytics and Tracking

### View Click Analytics

```bash
# Check analytics for a route
curl http://localhost:8082/v1/clickstream/route_123

# Get aggregated statistics
curl http://localhost:8082/v1/clickstream/stats
```

### Analytics Data Structure

```json
{
  "id": "01HZ...",
  "owner_id": "user_789",
  "creator_id": "user_789",
  "route_id": "route_123",
  "workspace_id": "workspace_456",
  "created": "2024-01-01T12:00:00Z",
  "dest": "https://example.com",
  "ip": "192.168.1.1",
  "continent": "North America",
  "country": "United States",
  "location": "New York",
  "os_family": "Windows",
  "os_version": "10.0",
  "user_agent_family": "Chrome",
  "user_agent_version": "120.0",
  "device_brand": "Dell",
  "device_family": "Desktop",
  "device_model": "OptiPlex",
  "session_first": "2024-01-01T12:00:00Z",
  "session_clicks": 1,
  "is_unique": true,
  "is_bot": false
}
```

## 🔧 API Usage Examples

### Create a Route

```bash
curl -X POST http://localhost:8081/v1/routes/main/example.com/my-link \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_jwt_token" \
  -d '{
    "switch": "main",
    "link": "my-link",
    "dest": "https://my-website.com",
    "dest_format": "Http",
    "code": 302,
    "status": "Active",
    "terminal": "External",
    "policy": {
      "type": "Basic"
    },
    "properties": {
      "route_id": "my_route_123",
      "domain_id": "my_domain_456",
      "owner_id": "my_user_789"
    }
  }'
```

### Get Route Information

```bash
curl http://localhost:8081/v1/routes/main/example.com/my-link \
  -H "Authorization: Bearer your_jwt_token"
```

### Update a Route

```bash
curl -X PUT http://localhost:8081/v1/routes/main/example.com/my-link \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_jwt_token" \
  -d '{
    "switch": "main",
    "link": "my-link",
    "dest": "https://my-updated-website.com",
    "dest_format": "Http",
    "code": 301,
    "status": "Active",
    "terminal": "External",
    "policy": {
      "type": "Basic"
    },
    "properties": {
      "route_id": "my_route_123",
      "domain_id": "my_domain_456",
      "owner_id": "my_user_789"
    }
  }'
```

### Delete a Route

```bash
curl -X DELETE http://localhost:8081/v1/routes/main/example.com/my-link \
  -H "Authorization: Bearer your_jwt_token"
```

## 🧪 Testing Your Setup

### Test Basic Redirect

```bash
# Create a test route
curl -X POST http://localhost:8081/v1/routes/main/example.com/test \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_jwt_token" \
  -d '{
    "switch": "main",
    "link": "test",
    "dest": "https://httpbin.org/get",
    "dest_format": "Http",
    "code": 302,
    "status": "Active",
    "terminal": "External",
    "policy": {"type": "Basic"},
    "properties": {
      "route_id": "test_route",
      "domain_id": "test_domain",
      "owner_id": "test_user"
    }
  }'

# Test the redirect
curl -I http://localhost:8080/test
```

### Test Conditional Routing

```bash
# Create a conditional route
curl -X POST http://localhost:8081/v1/routes/main/example.com/smart \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your_jwt_token" \
  -d '{
    "switch": "main",
    "link": "smart",
    "dest": "https://mobile.example.com",
    "dest_format": "Http",
    "code": 302,
    "status": "Active",
    "terminal": "External",
    "policy": {
      "type": "Conditional",
      "conditions": [
        {
          "key": "mobile",
          "condition": {
            "device": {"type": "mobile"}
          }
        }
      ]
    },
    "properties": {
      "route_id": "smart_route",
      "domain_id": "test_domain",
      "owner_id": "test_user"
    }
  }'

# Test with mobile user agent
curl -H "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)" \
     -I http://localhost:8080/smart

# Test with desktop user agent
curl -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" \
     -I http://localhost:8080/smart
```

## 📈 Monitoring Your Routes

### Check Service Health

```bash
# Check all services
make health-check

# Check specific service
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8082/health
```

### View Logs

```bash
# View all logs
make logs

# View specific service logs
make logs-router
make logs-tracker
make logs-aggregator
```

### Check Analytics

```bash
# Get click stream data
curl http://localhost:8082/v1/clickstream

# Get route-specific analytics
curl http://localhost:8082/v1/clickstream/route_123
```

## 🚨 Troubleshooting

### Common Issues

**Route not working:**
```bash
# Check if route exists
curl http://localhost:8081/v1/routes/main/example.com/your-link

# Check service logs
make logs-router
```

**Analytics not showing:**
```bash
# Check tracker service
make logs-tracker

# Check aggregator service
make logs-aggregator
```

**API authentication issues:**
```bash
# Check JWT token
echo $JWT_TOKEN

# Test authentication
curl -H "Authorization: Bearer $JWT_TOKEN" \
     http://localhost:8081/v1/routes
```

## 📚 Next Steps

Now that you've created your first route:

1. [Learn about the architecture](../architecture/)
2. [Explore the API documentation](../api/)
3. [Set up production deployment](../deployment/)
4. [Configure advanced routing](../architecture/advanced-routing/)

## 🔗 Additional Resources

- [API Reference](../api/) - Complete API documentation
- [Architecture Overview](../architecture/) - System architecture
- [Deployment Guide](../deployment/) - Production deployment
- [Troubleshooting](../deployment/troubleshooting/) - Common issues and solutions

---

**Need help?** Check our [troubleshooting guide](../deployment/troubleshooting/) or [open an issue](https://github.com/FlyingCow/shortas/issues) on GitHub.
