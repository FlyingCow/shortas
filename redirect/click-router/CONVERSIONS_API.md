# Click Router Conversion Tracking API

## Overview

The Click Router provides HTTP API endpoints for tracking conversion events and funnel steps. These endpoints accept conversion data, create `Hit` objects, and send them through the Fluvio message queue to the Click Tracker for processing.

## Base URL

```
https://your-domain.com:5800
```

**Note:** The Click Router uses HTTPS by default on port 5800.

## API Endpoints

### POST /conversions/track

Track a conversion event (purchase, signup, download, etc.)

#### Request

**Headers:**
- `Content-Type: application/json`
- `User-Agent`: Automatically extracted for user agent enrichment
- `X-Forwarded-For`: Client IP (if behind proxy)
- `Referer`: Referrer URL (automatically extracted)

**Request Body:**
```json
{
  "route_id": "route_123",
  "conversion_type": "purchase",
  "conversion_name": "Product Purchase",
  "conversion_value": 99.99,
  "attributed_click_id": "click_456",
  "attribution_type": "direct",
  "attribution_window_hours": 24,
  "user_id": "user_789",
  "session_id": "session_101",
  "metadata": {
    "product_id": "prod_123",
    "category": "electronics"
  }
}
```

**Required Fields:**
- `route_id` (string) - The route/short link ID
- `conversion_type` (string) - Type of conversion (e.g., "purchase", "signup", "download")
- `conversion_name` (string) - Name/description of the conversion

**Optional Fields:**
- `conversion_value` (number) - Monetary value of the conversion
- `attributed_click_id` (string) - ID of the click that led to this conversion
- `attribution_type` (string) - Attribution model: "direct", "session", "time-based", "multi-touch" (default: "direct")
- `attribution_window_hours` (number) - Attribution window in hours (default: 24)
- `user_id` (string) - User identifier
- `session_id` (string) - Session identifier
- `metadata` (object) - Additional custom metadata

#### Response

**Success (201 Created):**
```json
{
  "success": true,
  "conversion_id": "01HZ9ABC123XYZ",
  "message": "Conversion tracked successfully"
}
```

**Error (400 Bad Request):**
```json
{
  "error": "Invalid JSON data"
}
```

**Error (500 Internal Server Error):**
```json
{
  "error": "Failed to track conversion"
}
```

### POST /conversions/funnel

Track a funnel step event

#### Request

**Request Body:**
```json
{
  "route_id": "route_123",
  "funnel_name": "E-commerce Purchase Funnel",
  "funnel_steps": ["view", "add_to_cart", "checkout", "purchase"],
  "step_name": "add_to_cart",
  "step_position": 2,
  "step_value": 99.99,
  "user_id": "user_789",
  "session_id": "session_101",
  "metadata": {
    "product_id": "prod_123"
  }
}
```

**Required Fields:**
- `route_id` (string) - The route/short link ID
- `funnel_name` (string) - Name of the funnel
- `step_name` (string) - Name of the current step
- `step_position` (number) - Position of the step in the funnel (1-indexed)

**Optional Fields:**
- `funnel_steps` (array of strings) - Complete list of all funnel steps
- `step_value` (number) - Value associated with this step
- `user_id` (string) - User identifier
- `session_id` (string) - Session identifier
- `metadata` (object) - Additional custom metadata

#### Response

**Success (201 Created):**
```json
{
  "success": true,
  "funnel_step_id": "01HZ9ABC123XYZ",
  "message": "Funnel step tracked successfully"
}
```

**Error (400 Bad Request):**
```json
{
  "error": "Invalid JSON data"
}
```

**Error (500 Internal Server Error):**
```json
{
  "error": "Failed to track funnel step"
}
```

## Example Usage

### JavaScript

```javascript
// Track a purchase conversion
async function trackPurchase(productData) {
  const response = await fetch('https://your-domain.com:5800/conversions/track', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      route_id: productData.routeId,
      conversion_type: 'purchase',
      conversion_name: `${productData.name} Purchase`,
      conversion_value: productData.price,
      attributed_click_id: getClickIdFromCookie(),
      attribution_type: 'direct',
      user_id: getUserId(),
      session_id: getSessionId(),
      metadata: {
        product_id: productData.id,
        category: productData.category
      }
    })
  });
  
  const result = await response.json();
  if (result.success) {
    console.log('Conversion tracked:', result.conversion_id);
  }
}

// Track a funnel step
async function trackFunnelStep(stepData) {
  const response = await fetch('https://your-domain.com:5800/conversions/funnel', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      route_id: stepData.routeId,
      funnel_name: 'E-commerce Purchase Funnel',
      funnel_steps: ['view', 'add_to_cart', 'checkout', 'purchase'],
      step_name: stepData.stepName,
      step_position: stepData.position,
      step_value: stepData.value,
      user_id: stepData.userId,
      session_id: stepData.sessionId
    })
  });
  
  const result = await response.json();
  if (result.success) {
    console.log('Funnel step tracked:', result.funnel_step_id);
  }
}
```

### cURL

```bash
# Track a conversion
curl -X POST https://your-domain.com:5800/conversions/track \
  -H "Content-Type: application/json" \
  -d '{
    "route_id": "route_123",
    "conversion_type": "purchase",
    "conversion_name": "Product Purchase",
    "conversion_value": 99.99,
    "user_id": "user_456",
    "session_id": "session_789"
  }'

# Track a funnel step
curl -X POST https://your-domain.com:5800/conversions/funnel \
  -H "Content-Type: application/json" \
  -d '{
    "route_id": "route_123",
    "funnel_name": "E-commerce Purchase Funnel",
    "step_name": "add_to_cart",
    "step_position": 2,
    "step_value": 99.99
  }'
```

### Rust (reqwest)

```rust
use reqwest::Client;
use serde_json::json;

async fn track_conversion(
    route_id: &str,
    conversion_type: &str,
    conversion_name: &str,
    value: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("https://your-domain.com:5800/conversions/track")
        .json(&json!({
            "route_id": route_id,
            "conversion_type": conversion_type,
            "conversion_name": conversion_name,
            "conversion_value": value,
            "attribution_type": "direct",
            "attribution_window_hours": 24
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    println!("Conversion tracked: {}", result["conversion_id"]);
    Ok(())
}
```

## Data Flow

1. **Request Received** - Click Router receives HTTP POST request
2. **Data Validation** - Validates JSON structure and required fields
3. **Hit Creation** - Creates `Hit` object with `HitData::Conversion` or `HitData::FunnelStep`
4. **IP Extraction** - Extracts client IP from request or X-Forwarded-For header
5. **Registration** - Registers hit with HitRegistrar (sends to Fluvio)
6. **Response** - Returns HTTP 201 with conversion ID

## Data Enrichment

The conversion data is enriched as it flows through the pipeline:

1. **Click Router** - Extracts IP, user agent, referrer
2. **Click Tracker** - Enriches with geographic data (country, location), device data (OS, browser), session data
3. **Click Aggregator** - Processes and stores enriched data in ClickHouse

## Error Handling

### Invalid JSON
- Status: 400 Bad Request
- Response: `{"error": "Invalid JSON data"}`

### Registration Failure
- Status: 500 Internal Server Error
- Response: `{"error": "Failed to track conversion"}` or `{"error": "Failed to track funnel step"}`

### Network Errors
- Standard HTTP error codes apply
- Check Click Router logs for detailed error messages

## Testing

Use the provided test script:

```bash
cd redirect/click-router
./test_conversions.sh
```

Or test manually:

```bash
# Test conversion tracking
curl -k -X POST https://localhost:5800/conversions/track \
  -H "Content-Type: application/json" \
  -d '{
    "route_id": "test-route",
    "conversion_type": "purchase",
    "conversion_name": "Test Purchase",
    "conversion_value": 99.99
  }'

# Test funnel step tracking
curl -k -X POST https://localhost:5800/conversions/funnel \
  -H "Content-Type: application/json" \
  -d '{
    "route_id": "test-route",
    "funnel_name": "Test Funnel",
    "step_name": "test_step",
    "step_position": 1
  }'
```

## Integration Notes

- **HTTPS**: Click Router uses HTTPS by default (port 5800)
- **TLS Certificates**: Uses embedded test certificates in development
- **CORS**: May need to configure CORS headers for browser requests
- **Rate Limiting**: Consider implementing rate limiting for production
- **Authentication**: Currently no authentication required; consider adding for production

## Related Documentation

- [Complete Pipeline Integration](../click-aggregator-api/docs/CONVERSIONS_PIPELINE_INTEGRATION.md)
- [Implementation Examples](../click-aggregator-api/docs/CONVERSIONS_IMPLEMENTATION_EXAMPLES.md)
- [Conversions Guide](../click-aggregator-api/docs/CONVERSIONS_GUIDE.md)

