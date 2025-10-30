# Conversions System Architecture

## Data Flow Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   User Action   │───▶│  Conversion API  │───▶│ Click Tracker   │
│ (Purchase/Signup│    │   (JavaScript)   │    │   Service       │
│ /Download/etc)  │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                                         │
                                                         ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   ClickHouse    │◀───│ Click Aggregator │◀───│ Conversion      │
│   Analytics     │    │     Service      │    │ Processing      │
│   Storage       │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Materialized    │    │ Attribution      │    │ Conversion      │
│ Views           │    │ Analysis         │    │ Goals           │
│ (Pre-aggregated)│    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Analytics API   │    │ Dashboard        │    │ Reporting       │
│ Endpoints       │    │ Widgets           │    │ System          │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Conversion Types & Attribution Models

### Conversion Types
- **Purchase**: E-commerce transactions with monetary value
- **Signup**: User registrations and account creations
- **Download**: File downloads and resource access
- **Form Submission**: Contact forms, surveys, lead generation
- **Custom Events**: User-defined conversion actions

### Attribution Models
- **Direct Attribution**: Immediate conversion after click
- **Session Attribution**: Conversion within same user session
- **Time-based Attribution**: Conversion within configurable time window
- **Multi-touch Attribution**: Credit multiple touchpoints in conversion path

## Database Schema Overview

### Core Tables
1. **conversions** - Individual conversion events
2. **conversion_attribution** - Links conversions to clicks
3. **conversion_funnels** - Multi-step conversion tracking
4. **conversion_goals** - Conversion targets and goals

### Materialized Views
1. **conversion_rates_mv** - Conversion rates by route/time
2. **conversion_attribution_mv** - Attribution analysis
3. **conversion_funnels_mv** - Funnel performance
4. **revenue_analytics_mv** - Revenue and ROI metrics
5. **geographic_conversion_mv** - Geographic analysis
6. **device_conversion_mv** - Device-based analysis
7. **hourly_conversion_mv** - Real-time tracking
8. **conversion_goals_performance_mv** - Goal tracking
9. **multi_touch_attribution_mv** - Multi-touch analysis
10. **conversion_cohort_mv** - Cohort analysis

## API Endpoints Structure

```
/v1/conversions/
├── GET /                    # Get conversions with filters
├── POST /                   # Create new conversion
├── GET /rates               # Conversion rates analytics
├── GET /revenue             # Revenue analytics
├── GET /attribution         # Attribution analysis
├── GET /funnels             # Funnel performance
├── GET /summary             # Dashboard summary
├── GET /geographic          # Geographic analysis
├── GET /devices             # Device analysis
├── GET /hourly              # Hourly tracking
├── GET /goals               # Conversion goals
├── POST /goals              # Create conversion goal
├── PUT /goals/{id}          # Update conversion goal
├── DELETE /goals/{id}       # Delete conversion goal
├── GET /cohorts             # Cohort analysis
└── GET /multi-touch         # Multi-touch attribution
```

## Integration Points

### Frontend Integration
- JavaScript SDK for conversion tracking
- Cookie-based click ID tracking
- Session management
- User identification

### Backend Integration
- REST API endpoints
- Webhook support for real-time events
- Batch processing capabilities
- Third-party integrations

### Analytics Integration
- Dashboard widgets
- Custom reporting
- Data export capabilities
- Real-time monitoring

## Performance Considerations

### Database Optimization
- Partitioned tables by date
- Materialized views for fast queries
- Proper indexing strategies
- TTL policies for data retention

### Processing Optimization
- Async processing pipeline
- Batch operations for high volume
- Caching for frequently accessed data
- Connection pooling

### Scalability
- Horizontal scaling support
- Load balancing capabilities
- Microservices architecture
- Event-driven processing
