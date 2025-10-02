# Click Router API Documentation

This document describes the API endpoints, request/response formats, and integration patterns for Click Router.

## 🌐 Base URL

```
https://your-domain.com
```

## 📡 HTTP Endpoints

### Core Redirection

#### GET `/{short_code}`

Redirects to the destination URL based on the short code.

**Parameters:**
- `short_code` (path): The shortened URL identifier

**Query Parameters:**
- `debug` (optional): Enable debug mode for the request
- `format` (optional): Response format (json, html)

**Headers:**
- `User-Agent`: Browser/device identification
- `Accept-Language`: Language preferences
- `X-Forwarded-For`: Client IP (for load balancers)

**Response Types:**

**302 Temporary Redirect (Default)**
```http
HTTP/1.1 302 Found
Location: https://destination.com
Cache-Control: no-cache
```

**301 Permanent Redirect**
```http
HTTP/1.1 301 Moved Permanently
Location: https://destination.com
Cache-Control: public, max-age=31536000
```

**404 Not Found**
```http
HTTP/1.1 404 Not Found
Content-Type: text/html
Location: https://your-domain.com/404/{host}
```

**Debug Response (when debug=true)**
```json
{
  "request_id": "01HZ...",
  "timestamp": "2024-01-01T00:00:00Z",
  "route": {
    "switch": "main",
    "link": "example",
    "dest": "https://example.com",
    "policy": "conditional"
  },
  "context": {
    "user_agent": "Mozilla/5.0...",
    "ip_address": "192.168.1.1",
    "country": "US",
    "device": "desktop",
    "os": "Windows"
  },
  "redirect_type": "temporary"
}
```

### Root Path Handling

#### GET `/`

Handles requests to the root domain.

**Response:**
- Proxies to the configured index URL
- Returns the index page content

## 🔧 Configuration API

### Route Management

#### Route Structure

```json
{
  "switch": "main",
  "link": "example",
  "dest": "https://example.com",
  "dest_format": "Http",
  "code": 302,
  "ttl": 3600,
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
  },
  "properties": {
    "route_id": "route_123",
    "domain_id": "domain_456",
    "owner_id": "user_789",
    "allow_debug": true,
    "opengraph": false
  }
}
```

#### Routing Policies

**Basic Routing**
```json
{
  "policy": {
    "type": "Basic"
  }
}
```

**Conditional Routing**
```json
{
  "policy": {
    "type": "Conditional",
    "conditions": [
      {
        "key": "mobile_route",
        "condition": {
          "device": {
            "type": "mobile"
          },
          "os": {
            "name": "iOS"
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

**Challenge Routing**
```json
{
  "policy": {
    "type": "Challenge",
    "challenge": {
      "key": "captcha",
      "source": "recaptcha",
      "challenge_type": "image"
    }
  }
}
```

**File Routing**
```json
{
  "policy": {
    "type": "File",
    "file": {
      "content_type": "text/html"
    }
  }
}
```

### Expression Language

#### Device Conditions

```json
{
  "device": {
    "type": "mobile" | "desktop" | "tablet"
  }
}
```

#### Operating System Conditions

```json
{
  "os": {
    "name": "Windows" | "macOS" | "Linux" | "iOS" | "Android",
    "version": "10.0"
  }
}
```

#### User Agent Conditions

```json
{
  "ua": {
    "browser": "Chrome" | "Firefox" | "Safari" | "Edge",
    "version": "120.0"
  }
}
```

#### Geographic Conditions

```json
{
  "country": {
    "code": "US" | "GB" | "DE" | "FR",
    "name": "United States"
  }
}
```

#### Time-based Conditions

```json
{
  "time": {
    "hour": 9,
    "day_of_week": "Monday",
    "day_of_month": 15
  }
}
```

#### Complex Expressions

```json
{
  "and": [
    {
      "device": {
        "type": "mobile"
      }
    },
    {
      "country": {
        "code": "US"
      }
    }
  ],
  "or": [
    {
      "os": {
        "name": "iOS"
      }
    },
    {
      "os": {
        "name": "Android"
      }
    }
  ]
}
```

## 📊 Analytics API

### Hit Tracking

Every request generates a hit record with the following structure:

```json
{
  "id": "01HZ...",
  "timestamp": "2024-01-01T00:00:00Z",
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
  "ip_address": "192.168.1.1",
  "route": {
    "switch": "main",
    "link": "example"
  },
  "click": {
    "destination": "https://example.com"
  },
  "context": {
    "country": "US",
    "device": "desktop",
    "os": "Windows",
    "browser": "Chrome"
  }
}
```

### Analytics Streams

**Kafka Topic: `hit-stream-main`**
```json
{
  "topic": "hit-stream-main",
  "partition": 0,
  "offset": 12345,
  "timestamp": 1704067200000,
  "key": "main/example",
  "value": {
    "id": "01HZ...",
    "timestamp": "2024-01-01T00:00:00Z",
    "user_agent": "Mozilla/5.0...",
    "ip_address": "192.168.1.1",
    "route": {
      "switch": "main",
      "link": "example"
    },
    "click": {
      "destination": "https://example.com"
    }
  }
}
```

**Fluvio Topic: `hit-stream-main`**
```json
{
  "topic": "hit-stream-main",
  "partition": 0,
  "offset": 12345,
  "timestamp": 1704067200000,
  "key": "main/example",
  "value": {
    "id": "01HZ...",
    "timestamp": "2024-01-01T00:00:00Z",
    "user_agent": "Mozilla/5.0...",
    "ip_address": "192.168.1.1",
    "route": {
      "switch": "main", 
      "link": "example"
    },
    "click": {
      "destination": "https://example.com"
    }
  }
}
```

## 🔧 Configuration API

### Environment Configuration

#### Development Environment
```toml
[server]
threads = 4
listen_os_signals = true
exit = true

[debug]
enabled = true
verbose = true

[mongodb]
uri = "mongodb://localhost:27017/"
database = "shortas_dev"

[moka.routes_cache]
max_capacity = 1000
time_to_live_minutes = 30
```

#### Production Environment
```toml
[server]
threads = 16
listen_os_signals = true
exit = false

[debug]
enabled = false
verbose = false

[mongodb]
uri = "mongodb://prod-cluster:27017/"
database = "shortas_prod"

[moka.routes_cache]
max_capacity = 100000
time_to_live_minutes = 60
```

### Service Configuration

#### MongoDB Configuration
```toml
[mongodb]
uri = "mongodb://username:password@host:port/"
database = "shortas"
routes_collection = "routes"
encryption_collection = "encryption"
user_settings_collection = "user_settings"
```

#### DynamoDB Configuration
```toml
[aws.dynamo]
routes_table = "routes-table"
encryption_table = "encryption-table"
user_settings_table = "user-settings-table"
```

#### Cache Configuration
```toml
[moka.routes_cache]
max_capacity = 10000
time_to_live_minutes = 60
time_to_idle_minutes = 20

[moka.crypto_cache]
max_capacity = 1000
time_to_live_minutes = 1440
time_to_idle_minutes = 60

[moka.user_settings_cache]
max_capacity = 5000
time_to_live_minutes = 30
time_to_idle_minutes = 10
```

#### Analytics Configuration
```toml
[fluvio.hit_stream]
topic = "hit-stream-main"
host = "sc:9003"
batch_size = 10000
linger = 1000

[kafka.hit_stream]
topic = "hit-stream-main"
hosts = ["localhost:9092", "localhost:9093"]
ack_timeout_secs = 60
batch_size = 100
consumers_count = 2
iteration_seconds = 1
```

#### GeoIP Configuration
```toml
[geo_ip]
mmdb = "../data/geo-ip/GeoLite2-Country.mmdb"
```

#### User Agent Parser Configuration
```toml
[uaparser]
yaml = "../data/ua-parser/user-agents.yaml"
```

## 🚀 Integration Examples

### Basic Redirect

```bash
curl -L "https://short.ly/abc123"
# Follows redirect to destination
```

### Debug Mode

```bash
curl "https://short.ly/abc123?debug=true"
# Returns debug information
```

### Custom Headers

```bash
curl -H "User-Agent: CustomBot/1.0" \
     -H "Accept-Language: en-US,en;q=0.9" \
     "https://short.ly/abc123"
```

### Conditional Routing Test

```bash
# Mobile user agent
curl -H "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)" \
     "https://short.ly/abc123"

# Desktop user agent  
curl -H "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" \
     "https://short.ly/abc123"
```

## 🔒 Security Considerations

### Input Validation

- URL sanitization and validation
- Parameter length limits
- Character encoding validation
- SQL injection prevention

### Rate Limiting

- Per-IP rate limiting
- Per-route rate limiting
- Burst protection
- DDoS mitigation

### Authentication

- API key authentication
- JWT token support
- OAuth integration
- Role-based access control

### HTTPS/TLS

- TLS 1.2+ support
- Custom certificate management
- HSTS headers
- Secure cookie handling

## 📈 Performance Optimization

### Caching Headers

```http
Cache-Control: public, max-age=3600
ETag: "abc123"
Last-Modified: Wed, 01 Jan 2024 00:00:00 GMT
```

### Compression

```http
Content-Encoding: gzip
Content-Length: 1024
```

### Connection Handling

```http
Connection: keep-alive
Keep-Alive: timeout=5, max=1000
```

## 🧪 Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_redirect() {
        let router = create_test_router().await;
        let request = create_test_request("/test");
        let response = router.handle(&request).await.unwrap();
        
        assert!(matches!(response, FlowRouterResult::Redirect(_, _)));
    }
}
```

### Integration Testing

```bash
# Test basic redirect
curl -I "https://test.short.ly/abc123"

# Test conditional routing
curl -H "User-Agent: Mobile" "https://test.short.ly/abc123"

# Test error handling
curl -I "https://test.short.ly/nonexistent"
```

### Load Testing

```bash
# Using Apache Bench
ab -n 10000 -c 100 "https://short.ly/abc123"

# Using wrk
wrk -t12 -c400 -d30s "https://short.ly/abc123"
```

This API documentation provides comprehensive information for integrating with Click Router and understanding its capabilities.


