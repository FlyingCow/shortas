# Click Router Metrics Endpoints - Usage Guide

This guide shows you how to use the newly enabled metrics endpoints in your Click Router application.

## 🚀 Quick Start

### Enable Metrics Endpoints

The metrics endpoints are now integrated into the main application. You can enable them using command line arguments or environment variables:

#### Command Line
```bash
# Enable metrics with default settings
cargo run -- --enable-metrics

# Custom addresses
cargo run -- --listen-addr "0.0.0.0:8080" --metrics-addr "0.0.0.0:9090" --enable-metrics

# Help
cargo run -- --help
```

#### Environment Variables
```bash
export APP_ENABLE_METRICS=true
export APP_LISTEN_ADDR="0.0.0.0:5800"
export APP_METRICS_ADDR="0.0.0.0:9090"
cargo run
```

#### Production Deployment
```bash
# Production with metrics enabled
./click-router --run-mode production --enable-metrics
```

### Default Endpoints

When metrics are enabled, the following endpoints are available:

| Endpoint | Port | Description |
|----------|------|-------------|
| Main Application | 5800 (HTTPS) | Your click router application |
| Metrics Server | 9090 (HTTP) | Monitoring endpoints |

## 📊 Available Metrics Endpoints

### 1. Health Check - `/health`

**Purpose:** Quick health status and basic metrics
**Method:** GET
**Response:** JSON

```bash
curl http://localhost:9090/health
```

**Response Example:**
```json
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

### 2. Prometheus Metrics - `/metrics`

**Purpose:** Prometheus-compatible metrics export
**Method:** GET
**Response:** Prometheus text format

```bash
curl http://localhost:9090/metrics
```

**Response Example:**
```
# HELP flow_router_requests_total Total number of requests processed
# TYPE flow_router_requests_total counter
flow_router_requests_total 1000

# HELP flow_router_request_duration_seconds Request processing time
# TYPE flow_router_request_duration_seconds histogram
flow_router_request_duration_seconds_bucket{le="0.001"} 100
flow_router_request_duration_seconds_bucket{le="0.005"} 500
flow_router_request_duration_seconds_bucket{le="0.01"} 800
flow_router_request_duration_seconds_bucket{le="+Inf"} 1000
flow_router_request_duration_seconds_sum 5.5
flow_router_request_duration_seconds_count 1000
```

### 3. Detailed Metrics Info - `/metrics/info`

**Purpose:** Comprehensive metrics analysis with calculated rates
**Method:** GET
**Response:** JSON with detailed analysis

```bash
curl http://localhost:9090/metrics/info
```

**Response Example:**
```json
{
  "performance_metrics": {
    "total_requests": 1000,
    "successful_requests": 995,
    "error_requests": 5,
    "active_requests": 0,
    "success_rate_percent": 99.5
  },
  "cache_performance": {
    "route_cache_hits": 800,
    "route_cache_misses": 200,
    "route_cache_hit_rate_percent": 80.0,
    "user_settings_cache_hits": 150,
    "user_settings_cache_misses": 50,
    "user_settings_cache_hit_rate_percent": 75.0
  },
  "optimization_impact": {
    "iterative_flow_usage": 1000,
    "recursive_flow_usage": 0,
    "optimization_usage_percent": 100.0,
    "description": "Higher optimization_usage_percent indicates better performance (should be close to 100%)"
  },
  "database_operations": {
    "route_db_queries": 200,
    "hits_registered": 995
  },
  "timing_histograms": {
    "request_duration": {
      "sample_count": 1000,
      "sample_sum": 5.5
    },
    "flow_processing_duration": {
      "sample_count": 1000,
      "sample_sum": 1.2
    }
  }
}
```

## 🔧 Configuration Options

### Command Line Arguments

```bash
click-router --help
```

| Argument | Default | Environment Variable | Description |
|----------|---------|---------------------|-------------|
| `--listen-addr` | `0.0.0.0:5800` | `APP_LISTEN_ADDR` | Main application address |
| `--metrics-addr` | `0.0.0.0:9090` | `APP_METRICS_ADDR` | Metrics server address |
| `--enable-metrics` | `false` | `APP_ENABLE_METRICS` | Enable metrics endpoints |
| `--run-mode` | `production` | `APP_RUN_MODE` | Application run mode |
| `--config-path` | `./config` | `APP_CONFIG_PATH` | Configuration directory |

### Example Configurations

#### Development
```bash
cargo run -- \
  --run-mode development \
  --listen-addr "127.0.0.1:5800" \
  --metrics-addr "127.0.0.1:9090" \
  --enable-metrics
```

#### Production
```bash
./click-router \
  --run-mode production \
  --listen-addr "0.0.0.0:443" \
  --metrics-addr "127.0.0.1:9090" \
  --enable-metrics
```

#### Docker
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/click-router .
COPY --from=builder /app/config ./config
COPY --from=builder /app/certs ./certs

EXPOSE 5800 9090
CMD ["./click-router", "--enable-metrics"]
```

## 📈 Monitoring Integration

### Prometheus Configuration

Create `prometheus.yml`:
```yaml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'click-router'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
    scrape_interval: 5s
```

Start Prometheus:
```bash
prometheus --config.file=prometheus.yml
```

### Grafana Dashboard

1. **Add Prometheus Data Source:**
   - URL: `http://localhost:9091`

2. **Import Dashboard:**
   Use these queries for panels:

```promql
# Request Rate
rate(flow_router_requests_total[5m])

# Error Rate %
rate(flow_router_requests_error_total[5m]) / rate(flow_router_requests_total[5m]) * 100

# Cache Hit Rate %
rate(flow_router_route_cache_hits_total[5m]) / 
(rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) * 100

# P95 Latency
histogram_quantile(0.95, rate(flow_router_request_duration_seconds_bucket[5m]))

# Optimization Usage %
rate(flow_router_iterative_flow_usage_total[5m]) / 
(rate(flow_router_iterative_flow_usage_total[5m]) + rate(flow_router_recursive_flow_usage_total[5m])) * 100
```

### Docker Compose Setup

```yaml
version: '3.8'
services:
  click-router:
    build: .
    ports:
      - "5800:5800"  # Main app
      - "9090:9090"  # Metrics
    environment:
      - APP_ENABLE_METRICS=true
    
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9091:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      
  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    volumes:
      - grafana-storage:/var/lib/grafana

volumes:
  grafana-storage:
```

## 🎯 Key Metrics to Monitor

### Performance Optimization Impact

Monitor these metrics to verify the async recursion optimization is working:

1. **Optimization Usage:** Should be 100%
   ```bash
   curl -s http://localhost:9090/metrics/info | jq '.optimization_impact.optimization_usage_percent'
   ```

2. **Recursive Flow Usage:** Should be 0
   ```bash
   curl -s http://localhost:9090/metrics | grep "flow_router_recursive_flow_usage_total"
   ```

3. **Flow Processing Time:** Should show improvement
   ```bash
   curl -s http://localhost:9090/metrics | grep "flow_router_flow_processing_duration"
   ```

### System Health

1. **Success Rate:** Should be > 99%
   ```bash
   curl -s http://localhost:9090/metrics/info | jq '.performance_metrics.success_rate_percent'
   ```

2. **Cache Hit Rate:** Should be > 80%
   ```bash
   curl -s http://localhost:9090/metrics/info | jq '.cache_performance.route_cache_hit_rate_percent'
   ```

3. **Active Requests:** Monitor for spikes
   ```bash
   curl -s http://localhost:9090/metrics | grep "flow_router_active_requests"
   ```

## 🔍 Troubleshooting

### Common Issues

1. **Metrics endpoints not accessible**
   ```bash
   # Check if metrics are enabled
   curl http://localhost:9090/health
   
   # If connection refused, check if --enable-metrics flag is used
   ps aux | grep click-router
   ```

2. **No metrics data**
   ```bash
   # Generate some traffic first
   curl -k https://localhost:5800/test
   
   # Then check metrics
   curl http://localhost:9090/metrics/info
   ```

3. **Port conflicts**
   ```bash
   # Use different ports
   cargo run -- --metrics-addr "0.0.0.0:9091" --enable-metrics
   ```

### Debug Mode

Enable debug logging for metrics:
```bash
RUST_LOG=click_router::core::metrics=debug cargo run -- --enable-metrics
```

## 🚀 Production Best Practices

### Security

1. **Restrict metrics access:**
   ```bash
   # Bind to localhost only in production
   ./click-router --metrics-addr "127.0.0.1:9090" --enable-metrics
   ```

2. **Use reverse proxy:**
   ```nginx
   # nginx.conf
   location /metrics {
       proxy_pass http://127.0.0.1:9090;
       allow 10.0.0.0/8;  # Only allow internal networks
       deny all;
   }
   ```

### Performance

1. **Monitor resource usage:**
   ```bash
   # Check memory usage
   ps aux | grep click-router
   
   # Check metrics overhead
   curl -s http://localhost:9090/metrics/info | jq '.timing_histograms'
   ```

2. **Tune scrape intervals:**
   ```yaml
   # prometheus.yml - reduce frequency if needed
   scrape_configs:
     - job_name: 'click-router'
       scrape_interval: 30s  # Instead of 5s
   ```

### Alerting

Set up alerts for key metrics:
```yaml
# alerts.yml
groups:
  - name: click_router
    rules:
      - alert: HighErrorRate
        expr: rate(flow_router_requests_error_total[5m]) / rate(flow_router_requests_total[5m]) > 0.05
        for: 2m
        
      - alert: LowCacheHitRate
        expr: rate(flow_router_route_cache_hits_total[5m]) / (rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) < 0.8
        for: 5m
        
      - alert: OptimizationNotWorking
        expr: rate(flow_router_recursive_flow_usage_total[5m]) > 0
        for: 1m
```

## ✅ Verification Checklist

- [ ] Metrics endpoints are accessible
- [ ] Prometheus can scrape metrics
- [ ] Grafana dashboards are working
- [ ] Optimization usage is 100%
- [ ] Cache hit rates are healthy
- [ ] Alerts are configured
- [ ] Security restrictions are in place

The metrics endpoints are now fully integrated and ready for production monitoring!
