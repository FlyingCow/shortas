# Conversions Functionality Documentation

## Overview

The conversions functionality extends the Shortas URL shortening system with comprehensive conversion tracking, attribution analysis, and ROI metrics. This system allows you to track user actions beyond just clicks - purchases, signups, downloads, form submissions, and custom events.

## 🎯 Key Features

### Conversion Tracking
- **Multiple Conversion Types**: Purchase, signup, download, form submission, custom events
- **Attribution Models**: Direct, session-based, time-based, and multi-touch attribution
- **Real-time Processing**: Conversions are tracked and processed in real-time
- **Flexible Configuration**: Customizable attribution windows and conversion goals

### Analytics & Reporting
- **Conversion Rates**: Track conversion rates by route, time period, and user segments
- **Revenue Analytics**: Monitor revenue, average order value, and ROI metrics
- **Attribution Analysis**: Understand which clicks lead to conversions
- **Funnel Analysis**: Track multi-step conversion processes
- **Geographic Analysis**: Conversion performance by location
- **Device Analysis**: Conversion rates by device type and browser

### Advanced Features
- **Conversion Goals**: Set and track performance against conversion targets
- **Cohort Analysis**: Track conversion behavior over time for user cohorts
- **Multi-touch Attribution**: Credit multiple touchpoints in the conversion path
- **Custom Events**: Track any user-defined conversion action

## 🏗️ Architecture

### Data Flow
```
User Action → Conversion Event → Click Tracker → Click Aggregator → ClickHouse
     ↓              ↓              ↓              ↓
Conversion    Attribution    Processing    Storage &
Tracking      Logic         & Enrichment   Analytics
```

### Database Schema

#### Core Tables
- **`conversions`** - Individual conversion events
- **`conversion_attribution`** - Links conversions to clicks
- **`conversion_funnels`** - Multi-step conversion tracking
- **`conversion_goals`** - Conversion targets and goals

#### Materialized Views
- **`conversion_rates_mv`** - Conversion rates by route and time
- **`conversion_attribution_mv`** - Attribution analysis
- **`conversion_funnels_mv`** - Funnel performance metrics
- **`revenue_analytics_mv`** - Revenue and ROI calculations
- **`geographic_conversion_mv`** - Geographic conversion analysis
- **`device_conversion_mv`** - Device-based conversion analysis

## 📊 API Endpoints

### Core Conversion Endpoints

#### Get Conversions
```
GET /v1/conversions
```
Retrieve conversion data with optional filtering.

**Query Parameters:**
- `owner_id` - Filter by owner ID
- `route_id` - Filter by route ID
- `conversion_type` - Filter by conversion type
- `created_from` - Start date (ISO 8601)
- `created_to` - End date (ISO 8601)
- `limit` - Maximum results (default: 100)
- `offset` - Results to skip (default: 0)

#### Create Conversion
```
POST /v1/conversions
```
Record a new conversion event.

**Request Body:**
```json
{
  "route_id": "route_123",
  "conversion_type": "purchase",
  "conversion_name": "Product Purchase",
  "conversion_value": 99.99,
  "attributed_click_id": "click_456",
  "attribution_type": "direct",
  "user_id": "user_789",
  "session_id": "session_101",
  "metadata": {
    "product_id": "prod_123",
    "category": "electronics"
  }
}
```

### Analytics Endpoints

#### Conversion Rates
```
GET /v1/conversions/rates
```
Get conversion rates by route and time period.

#### Revenue Analytics
```
GET /v1/conversions/revenue
```
Get revenue analytics including total revenue, AOV, and ROI.

#### Attribution Analysis
```
GET /v1/conversions/attribution
```
Analyze which clicks lead to conversions.

#### Funnel Performance
```
GET /v1/conversions/funnels
```
Track conversion funnel performance and drop-off points.

#### Conversion Summary
```
GET /v1/conversions/summary
```
Get high-level conversion metrics for dashboard display.

## 🔧 Implementation Guide

### 1. Database Setup

Run the migration scripts to create the conversion tables:

```bash
# Run ClickHouse migrations
clickhouse-client --query "$(cat migrations/003_create_conversions_tables.sql)"
clickhouse-client --query "$(cat migrations/004_create_conversion_materialized_views.sql)"
```

### 2. Conversion Tracking

#### JavaScript Integration
```javascript
// Track a conversion
fetch('/v1/conversions', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer ' + token
  },
  body: JSON.stringify({
    route_id: 'route_123',
    conversion_type: 'purchase',
    conversion_name: 'Product Purchase',
    conversion_value: 99.99,
    attributed_click_id: getClickIdFromCookie(), // From click tracking
    user_id: getUserId(),
    session_id: getSessionId(),
    metadata: {
      product_id: 'prod_123',
      category: 'electronics'
    }
  })
});
```

#### Server-side Integration
```rust
// Track conversion in your application
let conversion = Conversion {
    id: generate_id(),
    route_id: "route_123".to_string(),
    conversion_type: "purchase".to_string(),
    conversion_name: "Product Purchase".to_string(),
    conversion_value: Decimal::from_f64(99.99).unwrap(),
    attributed_click_id: click_id,
    attribution_type: "direct".to_string(),
    user_id: user_id,
    session_id: session_id,
    // ... other fields
};

conversion_store.store_conversion(conversion).await?;
```

### 3. Conversion Goals Setup

```rust
// Create a conversion goal
let goal = ConversionGoal {
    id: generate_id(),
    route_id: "route_123".to_string(),
    goal_name: "Daily Sales Target".to_string(),
    goal_type: "revenue".to_string(),
    target_value: Decimal::from_f64(1000.0).unwrap(),
    target_period: "daily".to_string(),
    attribution_window_hours: 24,
    is_active: 1,
    // ... other fields
};

conversion_store.store_conversion_goal(goal).await?;
```

### 4. Funnel Tracking

```rust
// Track funnel steps
let funnel_step = ConversionFunnel {
    id: generate_id(),
    funnel_name: "E-commerce Funnel".to_string(),
    funnel_steps: vec![
        "view_product".to_string(),
        "add_to_cart".to_string(),
        "checkout".to_string(),
        "purchase".to_string()
    ],
    step_name: "add_to_cart".to_string(),
    step_position: 2,
    step_completed: 1,
    step_value: Decimal::from_f64(99.99).unwrap(),
    // ... other fields
};

conversion_store.store_conversion_funnel(funnel_step).await?;
```

## 📈 Analytics Examples

### Conversion Rate Analysis
```sql
-- Get conversion rates by route
SELECT 
    route_id,
    conversion_type,
    total_conversions,
    total_conversion_value,
    avg_conversion_value,
    unique_converting_users
FROM conversion_rates_mv
WHERE owner_id = 'user_123'
  AND date >= '2024-01-01'
ORDER BY total_conversions DESC;
```

### Revenue Analysis
```sql
-- Get revenue metrics
SELECT 
    route_id,
    total_conversions,
    total_revenue,
    avg_order_value,
    unique_customers,
    revenue_per_click
FROM revenue_analytics_mv
WHERE owner_id = 'user_123'
  AND date >= '2024-01-01';
```

### Attribution Analysis
```sql
-- Analyze attribution patterns
SELECT 
    attribution_type,
    attribution_position,
    attribution_count,
    avg_time_to_conversion,
    unique_conversions
FROM conversion_attribution_mv
WHERE owner_id = 'user_123'
  AND date >= '2024-01-01';
```

## 🎯 Use Cases

### E-commerce
- Track product purchases with revenue attribution
- Monitor conversion funnels from product view to purchase
- Analyze which marketing campaigns drive the most revenue
- Set revenue goals and track performance

### Lead Generation
- Track form submissions and signups
- Monitor conversion rates from landing pages
- Analyze which traffic sources generate the most leads
- Set lead generation goals

### Content Marketing
- Track downloads of whitepapers and resources
- Monitor engagement with content
- Analyze which content pieces drive conversions
- Track content performance metrics

### SaaS Applications
- Track free trial signups
- Monitor conversion from trial to paid
- Analyze user onboarding funnel
- Track feature adoption and usage

## 🔍 Best Practices

### Conversion Tracking
1. **Consistent Naming**: Use consistent conversion names across your application
2. **Attribution Windows**: Set appropriate attribution windows based on your business model
3. **Data Quality**: Ensure accurate user identification and session tracking
4. **Privacy Compliance**: Follow data privacy regulations when tracking user behavior

### Analytics
1. **Regular Monitoring**: Set up regular reports to monitor conversion performance
2. **Goal Setting**: Define clear, measurable conversion goals
3. **Segmentation**: Analyze conversions by different user segments
4. **A/B Testing**: Use conversion data to measure A/B test results

### Performance
1. **Batch Processing**: Use batch processing for high-volume conversion events
2. **Caching**: Cache frequently accessed conversion metrics
3. **Indexing**: Ensure proper database indexing for fast queries
4. **Monitoring**: Monitor system performance and conversion processing times

## 🚀 Future Enhancements

### Planned Features
- **Machine Learning**: Predictive conversion modeling
- **Real-time Alerts**: Conversion goal achievement notifications
- **Advanced Attribution**: Machine learning-based attribution models
- **Cross-device Tracking**: Track conversions across multiple devices
- **Integration APIs**: Third-party integration capabilities

### Extensibility
The conversion system is designed to be easily extensible:
- Add new conversion types
- Implement custom attribution models
- Create custom analytics views
- Integrate with external systems

## 📚 Additional Resources

- [API Reference Documentation](api-reference.md)
- [Database Schema Documentation](database-schema.md)
- [Integration Examples](integration-examples.md)
- [Troubleshooting Guide](troubleshooting.md)
- [Performance Optimization](performance-optimization.md)
