# Conversions Pipeline Integration

## Overview

Conversions now flow through your existing Shortas pipeline: **Click Router → Fluvio → Click Tracker → Click Aggregator → ClickHouse**. This document explains how the conversion tracking integrates with your current architecture.

## 🔄 Data Flow

```
Conversion Event → Click Router → Fluvio → Click Tracker → Click Aggregator → ClickHouse
      ↓              ↓           ↓          ↓              ↓              ↓
   JavaScript    API Endpoint  Message   Processing    Processing    Storage &
   Tracking      /conversions  Queue     Pipeline      Pipeline      Analytics
```

## 📊 Integration Points

### 1. **Click Router** - Entry Point
- **New Endpoints**: `/conversions/track` and `/conversions/funnel`
- **Data Flow**: Accepts conversion events via HTTP POST
- **Processing**: Creates `Hit` objects with `HitData::Conversion` or `HitData::FunnelStep`
- **Output**: Sends to Fluvio message queue

### 2. **Click Tracker** - Processing Pipeline
- **New Module**: `ConversionProcessingModule`
- **Data Flow**: Receives conversion events from Fluvio
- **Processing**: Enriches conversion data with user agent, geographic, and device information
- **Output**: Converts to `ClickStreamItem` and sends to Click Aggregator

### 3. **Click Aggregator** - Storage Pipeline
- **New Module**: `ConversionProcessingModule`
- **Data Flow**: Receives enriched conversion data
- **Processing**: Identifies conversion events and stores in ClickHouse
- **Output**: Stores in conversion tables and materialized views

## 🛠️ Implementation Details

### Click Router Integration

#### New Conversion Endpoints
```rust
// Track a conversion
POST /conversions/track
{
  "route_id": "route_123",
  "conversion_type": "purchase",
  "conversion_name": "Product Purchase",
  "conversion_value": 99.99,
  "attributed_click_id": "click_456",
  "user_id": "user_789",
  "session_id": "session_101"
}

// Track a funnel step
POST /conversions/funnel
{
  "route_id": "route_123",
  "funnel_name": "E-commerce Funnel",
  "step_name": "add_to_cart",
  "step_position": 2,
  "step_value": 99.99
}
```

#### HitData Extension
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HitData {
    Click(Click),
    Event(Event),
    Conversion(ConversionEvent),      // NEW
    FunnelStep(ConversionFunnelStep), // NEW
}
```

### Click Tracker Integration

#### Conversion Processing Module
```rust
#[derive(Clone)]
pub struct ConversionProcessingModule;

#[async_trait::async_trait]
impl TrackingModule for ConversionProcessingModule {
    async fn execute(&mut self, context: &mut TrackingPipeContext) -> Result<()> {
        if let HitData::Conversion(conversion) = &context.hit.data {
            // Enrich conversion with user agent, geographic, device data
            // Convert to ClickStreamItem for aggregator
        }
        Ok(())
    }
}
```

#### Pipeline Integration
```rust
#[derive(Clone)]
pub enum ClickModules {
    Init(InitModule),
    Aggregate(AggregateModule),
    Location(EnrichLocationModule),
    Session(EnrichSessionModule),
    UserAgent(EnrichUserAgentModule),
    Conversion(ConversionProcessingModule), // NEW
}
```

### Click Aggregator Integration

#### Conversion Processing Module
```rust
#[derive(Clone)]
pub struct ConversionProcessingModule;

#[async_trait::async_trait]
impl AggsModule for ConversionProcessingModule {
    async fn execute(&mut self, context: &mut AggsPipeContext) -> Result<()> {
        // Check if ClickStreamItem is a conversion event
        if let Some(dest) = &context.click.dest {
            if dest.starts_with("conversion:") {
                // Process conversion event
                // Store in ClickHouse conversion tables
            }
        }
        Ok(())
    }
}
```

## 🔧 Usage Examples

### JavaScript Integration
```javascript
// Base URL for Click Router (default: https://your-domain.com:5800)
const CLICK_ROUTER_URL = 'https://your-domain.com:5800';

// Track a conversion
fetch(`${CLICK_ROUTER_URL}/conversions/track`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    route_id: 'product-page-123',
    conversion_type: 'purchase',
    conversion_name: 'Product Purchase',
    conversion_value: 199.99,
    attributed_click_id: getClickIdFromCookie(), // Optional: from click tracking
    attribution_type: 'direct', // Optional: 'direct', 'session', 'time-based', 'multi-touch'
    attribution_window_hours: 24, // Optional: default 24
    user_id: getUserId(), // Optional
    session_id: getSessionId(), // Optional
    metadata: {
      product_id: 'prod_123',
      category: 'electronics'
    }
  })
})
.then(response => response.json())
.then(data => {
  console.log('Conversion tracked:', data.conversion_id);
})
.catch(error => {
  console.error('Error tracking conversion:', error);
});

// Track a funnel step
fetch(`${CLICK_ROUTER_URL}/conversions/funnel`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    route_id: 'product-page-123',
    funnel_name: 'E-commerce Purchase Funnel',
    funnel_steps: ['view', 'add_to_cart', 'checkout', 'purchase'], // Optional
    step_name: 'add_to_cart',
    step_position: 2,
    step_value: 199.99, // Optional
    user_id: getUserId(), // Optional
    session_id: getSessionId(), // Optional
    metadata: {
      product_id: 'prod_123'
    }
  })
});
```

### Server-side Integration
```rust
// Option 1: Send to Click Router API (Recommended - flows through pipeline)
use reqwest::Client;

let client = Client::new();
let response = client
    .post("https://your-click-router.com:5800/conversions/track")
    .json(&serde_json::json!({
        "route_id": "product-page-123",
        "conversion_type": "purchase",
        "conversion_name": "Product Purchase",
        "conversion_value": 199.99,
        "attributed_click_id": click_id,
        "attribution_type": "direct",
        "attribution_window_hours": 24,
        "user_id": "user_456",
        "session_id": "session_789",
        "metadata": {
            "product_id": "prod_123",
            "category": "electronics"
        }
    }))
    .send()
    .await?;

let result: serde_json::Value = response.json().await?;
println!("Conversion tracked: {}", result["conversion_id"]);

// Option 2: Direct integration (if you have access to click-router internals)
// This requires direct access to the HitRegistrar
let conversion_event = ConversionEvent {
    id: ulid::Ulid::new().to_string(),
    route_id: Some("product-page-123".to_string()),
    conversion_type: "purchase".to_string(),
    conversion_name: "Product Purchase".to_string(),
    conversion_value: Some(199.99),
    user_id: Some("user_456".to_string()),
    session_id: Some("session_789".to_string()),
    created: Utc::now(),
    ..Default::default()
};

let hit = Hit::conversion(
    &conversion_event.id,
    Utc::now(),
    None, // user_agent
    None, // ip (will be extracted)
    conversion_event,
    Some(HitRoute {
        id: Some("product-page-123".to_string()),
        ..Default::default()
    }),
);

hit_registrar.register(&hit).await?;
```

## 📈 Data Processing Flow

### 1. **Conversion Event Creation**
- User performs action (purchase, signup, etc.)
- JavaScript or server creates conversion event
- Event sent to Click Router `/conversions/track` or `/conversions/funnel` endpoint
- Click Router runs on `https://your-domain.com:5800` (default port)

### 2. **Click Router Processing**
- Receives HTTP POST request at `/conversions/track` or `/conversions/funnel`
- Validates conversion data and creates `ConversionEvent` or `ConversionFunnelStep`
- Creates `Hit` with `HitData::Conversion` or `HitData::FunnelStep`
- Extracts IP address, user agent, referrer from request
- Registers hit with HitRegistrar (Fluvio)
- Returns HTTP 201 response with conversion ID

### 3. **Click Tracker Processing**
- Receives conversion event from Fluvio
- Enriches with geographic data (country, location)
- Enriches with device data (OS, browser, device)
- Enriches with session data
- Converts to `ClickStreamItem` format
- Sends to Click Aggregator

### 4. **Click Aggregator Processing**
- Receives enriched conversion data
- Identifies conversion events by `dest` field pattern
- Processes conversion-specific logic
- Stores in ClickHouse conversion tables
- Updates materialized views for analytics

## 🗄️ ClickHouse Storage

### Conversion Events Storage
Conversions are stored in the `conversions` table with enriched data:
- User agent information
- Geographic location
- Device information
- Session data
- Attribution data

### Materialized Views
Pre-aggregated views for fast analytics:
- `conversion_rates_mv` - Conversion rates by route/time
- `conversion_attribution_mv` - Attribution analysis
- `conversion_funnels_mv` - Funnel performance
- `revenue_analytics_mv` - Revenue metrics

## 🔍 Monitoring & Debugging

### Logging
Each service logs conversion processing:
```rust
// Click Router
info!("Conversion received: {} - {}", conversion_type, conversion_name);
info!("Conversion stream item: {}", serde_json::json!(stream_item));

// Click Tracker
info!("Processing conversion event: {} - {}", conversion_type, conversion_name);
info!("Conversion stream item: {}", serde_json::json!(stream_item));

// Click Aggregator
info!("Processing conversion event: {}", dest);
info!("Conversion processed: {} - {}", conversion_type, conversion_name);
```

### Debugging Conversion Flow
1. **Check Click Router logs** - Verify conversion received and sent to Fluvio
   ```bash
   # Look for messages like:
   # "Conversion received: purchase - Product Purchase"
   # "Conversion tracked successfully"
   ```

2. **Check Fluvio** - Verify message queued in the hit stream
   ```bash
   # Check Fluvio topic for conversion messages
   fluvio consume hits
   ```

3. **Check Click Tracker logs** - Verify processing and enrichment
   ```bash
   # Look for messages like:
   # "Processing conversion event: purchase - Product Purchase"
   # "Conversion stream item: {...}"
   ```

4. **Check Click Aggregator logs** - Verify storage
   ```bash
   # Look for messages like:
   # "Processing conversion event: conversion:purchase:Product Purchase"
   # "Conversion processed: purchase - Product Purchase"
   ```

5. **Check ClickHouse** - Verify data stored
   ```sql
   SELECT * FROM conversions 
   WHERE route_id = 'your-route-id' 
   ORDER BY created DESC 
   LIMIT 10;
   ```

## 🚀 Benefits

### Seamless Integration
- Uses existing pipeline infrastructure
- No additional message queues or services
- Leverages existing enrichment capabilities

### Real-time Processing
- Conversions processed in real-time
- Immediate availability in analytics
- Consistent with click tracking performance

### Unified Analytics
- Conversions and clicks in same system
- Cross-referencing capabilities
- Unified attribution analysis

### Scalability
- Inherits pipeline scalability
- Handles high-volume conversion events
- Efficient ClickHouse storage

## 🔧 Configuration

### Fluvio Topics
- Uses existing hit tracking topic
- No additional topic configuration needed
- Same partitioning and replication

### ClickHouse Tables
- New conversion tables added
- Materialized views for performance
- TTL policies for data retention

### Service Configuration
- No additional service configuration
- Uses existing pipeline modules
- Same monitoring and alerting

## 📊 Analytics Integration

### API Endpoints
All existing analytics endpoints work with conversion data:
- `/v1/conversions` - Conversion data
- `/v1/conversions/rates` - Conversion rates
- `/v1/conversions/revenue` - Revenue analytics
- `/v1/conversions/attribution` - Attribution analysis

### Dashboard Integration
- Conversion metrics in existing dashboards
- Real-time conversion tracking
- Cross-reference with click data

This integration provides a complete conversion tracking solution that seamlessly fits into your existing Shortas architecture while maintaining high performance and scalability.
