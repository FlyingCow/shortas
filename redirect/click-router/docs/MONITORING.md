# Click Router Monitoring Guide

This guide covers the comprehensive monitoring capabilities added to track the performance impact of the async recursion optimization and overall system health.

## 📊 Overview

The monitoring system provides detailed metrics to track:
- **Performance optimization impact** (async recursion → iterative flow)
- **Request processing performance**
- **Cache efficiency**
- **Database operation timing**
- **Memory allocation patterns**
- **Error rates and system health**

## 🚀 Quick Start

### 1. Basic Metrics Collection

```rust
use click_router::core::metrics::METRICS;

// Metrics are automatically collected when using FlowRouter
let flow_router = FlowRouter::default(/* ... */);
let result = flow_router.handle(&request, &response).await;

// Access global metrics
println!("Total requests: {}", METRICS.requests_total.get());
```

### 2. Prometheus Endpoint

```rust
use click_router::core::metrics_endpoint::create_metrics_router;
use salvo::{prelude::*, Server, TcpListener};

#[tokio::main]
async fn main() {
    // Create metrics router
    let metrics_router = create_metrics_router();
    let service = Service::new(metrics_router);
    
    // Start metrics server
    let server = Server::new(TcpListener::new("0.0.0.0:9090").bind().await);
    server.serve(service).await;
}
```

### 3. Health Check Integration

```rust
// GET /health returns JSON with current metrics
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "metrics": {
    "requests_total": 1000,
    "requests_success": 995,
    "requests_error": 5,
    "active_requests": 3,
    "route_cache_hits": 800,
    "route_cache_misses": 200,
    "iterative_flow_usage": 1000,
    "recursive_flow_usage": 0
  }
}
```

## 📈 Available Metrics

### Core Performance Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `flow_router_requests_total` | Counter | Total number of requests processed |
| `flow_router_requests_success_total` | Counter | Number of successful requests |
| `flow_router_requests_error_total` | Counter | Number of failed requests |
| `flow_router_active_requests` | Gauge | Current number of active requests |
| `flow_router_request_duration_seconds` | Histogram | End-to-end request processing time |

### Optimization Impact Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `flow_router_iterative_flow_usage_total` | Counter | Times optimized iterative flow was used |
| `flow_router_recursive_flow_usage_total` | Counter | Times legacy recursive flow was used |
| `flow_router_flow_processing_duration_seconds` | Histogram | Time spent in flow processing |
| `flow_router_memory_allocations_per_request` | Histogram | Estimated memory allocations per request |

### Cache Performance Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `flow_router_route_cache_hits_total` | Counter | Route cache hits |
| `flow_router_route_cache_misses_total` | Counter | Route cache misses |
| `flow_router_user_settings_cache_hits_total` | Counter | User settings cache hits |
| `flow_router_user_settings_cache_misses_total` | Counter | User settings cache misses |
| `flow_router_cache_lookup_duration_seconds` | Histogram | Cache lookup timing |

### Database Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `flow_router_route_db_queries_total` | Counter | Number of database queries for routes |
| `flow_router_db_query_duration_seconds` | Histogram | Database query timing |
| `flow_router_hits_registered_total` | Counter | Number of hits registered |

## 🔍 Monitoring Endpoints

### `/metrics` - Prometheus Format
Returns metrics in Prometheus text format for scraping:

```
# HELP flow_router_requests_total Total number of requests processed
# TYPE flow_router_requests_total counter
flow_router_requests_total 1000

# HELP flow_router_request_duration_seconds Request processing time
# TYPE flow_router_request_duration_seconds histogram
flow_router_request_duration_seconds_bucket{le="0.001"} 100
flow_router_request_duration_seconds_bucket{le="0.005"} 500
...
```

### `/health` - Health Check
Returns JSON with basic health and metrics:

```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "metrics": {
    "requests_total": 1000,
    "requests_success": 995,
    "active_requests": 3
  }
}
```

### `/metrics/info` - Detailed Information
Returns comprehensive metrics analysis:

```json
{
  "performance_metrics": {
    "total_requests": 1000,
    "success_rate_percent": 99.5
  },
  "cache_performance": {
    "route_cache_hit_rate_percent": 80.0
  },
  "optimization_impact": {
    "optimization_usage_percent": 100.0,
    "description": "Higher percentage indicates better performance"
  }
}
```

## 📊 Prometheus Configuration

### Basic Scraping Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'click-router'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
    scrape_interval: 5s
```

### Alerting Rules

```yaml
# click_router_alerts.yml
groups:
  - name: click_router
    rules:
      - alert: HighErrorRate
        expr: rate(flow_router_requests_error_total[5m]) / rate(flow_router_requests_total[5m]) > 0.05
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "High error rate in click router"
          
      - alert: LowCacheHitRate
        expr: |
          rate(flow_router_route_cache_hits_total[5m]) / 
          (rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) < 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Low cache hit rate - consider cache tuning"
          
      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(flow_router_request_duration_seconds_bucket[5m])) > 0.1
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "High request latency detected"
          
      - alert: OptimizationNotUsed
        expr: |
          rate(flow_router_recursive_flow_usage_total[5m]) / 
          (rate(flow_router_iterative_flow_usage_total[5m]) + rate(flow_router_recursive_flow_usage_total[5m])) > 0.01
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "Legacy recursive flow still being used"
```

## 📈 Grafana Dashboard

### Key Queries for Dashboards

```promql
# Request Rate
rate(flow_router_requests_total[5m])

# Error Rate Percentage
rate(flow_router_requests_error_total[5m]) / rate(flow_router_requests_total[5m]) * 100

# Cache Hit Rate Percentage
rate(flow_router_route_cache_hits_total[5m]) / 
(rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) * 100

# P95 Latency
histogram_quantile(0.95, rate(flow_router_request_duration_seconds_bucket[5m]))

# Optimization Usage Percentage
rate(flow_router_iterative_flow_usage_total[5m]) / 
(rate(flow_router_iterative_flow_usage_total[5m]) + rate(flow_router_recursive_flow_usage_total[5m])) * 100

# Average Memory Allocations
rate(flow_router_memory_allocations_per_request_sum[5m]) / 
rate(flow_router_memory_allocations_per_request_count[5m])
```

### Dashboard Panels

1. **Request Overview**
   - Total requests/sec
   - Success rate %
   - Error rate %
   - Active requests

2. **Performance Optimization**
   - Iterative vs Recursive flow usage
   - Flow processing time P95/P99
   - Memory allocations trend

3. **Cache Performance**
   - Route cache hit rate %
   - User settings cache hit rate %
   - Cache lookup times

4. **Database Performance**
   - Query rate
   - Query duration P95/P99
   - Hits registration rate

## 🔧 Custom Metrics

### Adding Custom Metrics

```rust
use click_router::core::metrics::FlowRouterMetrics;
use prometheus::{IntCounter, register_int_counter};

// Create custom metrics
let custom_counter = register_int_counter!(
    "my_custom_metric_total",
    "Description of my custom metric"
)?;

// Use in your code
custom_counter.inc();
```

### Using Custom Registry

```rust
use prometheus::Registry;
use click_router::core::metrics::FlowRouterMetrics;

let registry = Registry::new();
let metrics = FlowRouterMetrics::with_registry(&registry)?;

let flow_router = FlowRouter::with_metrics(
    routes_cache,
    user_settings_cache,
    user_agent_detector,
    location_detector,
    hit_registrar,
    modules,
    metrics,
);
```

## 🎯 Performance Monitoring Best Practices

### 1. Key Metrics to Monitor

**Optimization Impact:**
- `optimization_usage_percent` should be close to 100%
- `flow_processing_duration` should show improvement over time
- `memory_allocations_per_request` should be stable/decreasing

**System Health:**
- `success_rate_percent` should be > 99%
- `cache_hit_rate_percent` should be > 80%
- `p95_latency` should be < 100ms

### 2. Alerting Thresholds

```yaml
# Recommended alert thresholds
- Error rate > 5% for 2 minutes
- Cache hit rate < 80% for 5 minutes  
- P95 latency > 100ms for 2 minutes
- Recursive flow usage > 1% for 1 minute
```

### 3. Dashboard Organization

**Executive Dashboard:**
- Request rate, error rate, latency
- Cache performance summary
- System health status

**Technical Dashboard:**
- Detailed timing histograms
- Memory and optimization metrics
- Database performance details

### 4. Capacity Planning

Monitor these trends for capacity planning:
- Request rate growth
- Memory allocation trends
- Database query patterns
- Cache efficiency over time

## 🚀 Integration Examples

### Docker Compose with Monitoring Stack

```yaml
version: '3.8'
services:
  click-router:
    build: .
    ports:
      - "5800:5800"  # Main app
      - "9090:9090"  # Metrics
    
  prometheus:
    image: prom/prometheus
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      
  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
```

### Kubernetes Monitoring

```yaml
apiVersion: v1
kind: Service
metadata:
  name: click-router-metrics
  labels:
    app: click-router
spec:
  ports:
  - port: 9090
    name: metrics
  selector:
    app: click-router
---
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: click-router
spec:
  selector:
    matchLabels:
      app: click-router
  endpoints:
  - port: metrics
    path: /metrics
```

## 🔍 Troubleshooting

### Common Issues

1. **Metrics not appearing**
   - Check if metrics endpoint is accessible
   - Verify Prometheus scraping configuration
   - Ensure metrics are being incremented in code

2. **High memory allocations**
   - Check for memory leaks in custom code
   - Monitor garbage collection patterns
   - Consider object pooling for hot paths

3. **Low cache hit rates**
   - Review cache configuration (TTL, capacity)
   - Analyze request patterns
   - Consider cache warming strategies

4. **Legacy flow still being used**
   - Check for deprecated method usage
   - Update modules to use new iterative flow
   - Review error handling paths

### Debug Mode

Enable debug logging for detailed metrics information:

```rust
RUST_LOG=click_router::core::metrics=debug cargo run
```

This comprehensive monitoring system provides full visibility into the performance optimization impact and overall system health, enabling data-driven decisions for further improvements.
