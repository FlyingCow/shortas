# Monitoring Stack

Prometheus, Grafana, and Loki monitoring stack for Shortas services.

## Components

| Service | Port | Purpose |
|---------|------|---------|
| Prometheus | 9091 | Metrics collection and storage |
| Grafana | 3001 | Visualization dashboards |
| Loki | 3100 | Log aggregation |

## Prerequisites

Loki requires MinIO for log storage. Ensure MinIO is running from the main infrastructure:

```bash
cd infra/custom
docker compose up -d minio minio-setup
```

The `minio-setup` container creates the required `loki` bucket automatically.

## Quick Start

```bash
cd redirect/monitoring
docker compose up -d
```

Access Grafana at http://localhost:3001 (admin/admin).

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      All Services                           │
│  (click-router, click-tracker, click-aggregator, etc.)     │
└──────────────┬─────────────────────────┬────────────────────┘
               │                         │
         metrics (/metrics)        logs (warn/error)
               │                         │
               ▼                         ▼
      ┌─────────────────┐       ┌─────────────────┐
      │   Prometheus    │       │      Loki       │
      │   (port 9091)   │       │   (port 3100)   │
      └────────┬────────┘       └────────┬────────┘
               │                         │
               │                         ▼
               │                ┌─────────────────┐
               │                │     MinIO       │
               │                │  (S3 storage)   │
               │                └─────────────────┘
               │                         │
               └──────────┬──────────────┘
                          │
                          ▼
                 ┌─────────────────┐
                 │     Grafana     │
                 │   (port 3001)   │
                 └─────────────────┘
```

## Prometheus

Prometheus scrapes metrics from all services. Configuration is in `prometheus/prometheus.yml`.

### Scraped Targets

- click-router: `shortas-click-router:9090`
- click-aggregator: `shortas-click-aggregator:9090`

### Example Queries

```promql
# Request rate
rate(requests_total[5m])

# Error rate
rate(requests_error_total[5m]) / rate(requests_total[5m])

# 95th percentile latency
histogram_quantile(0.95, rate(request_duration_seconds_bucket[5m]))
```

## Loki

Loki aggregates logs from all services. Services send Warning and Error level logs via:

- **Rust services**: `tracing-loki` crate
- **.NET API**: `Serilog.Sinks.Grafana.Loki`

### Storage Backend

Loki uses MinIO (S3-compatible) for log storage instead of local filesystem. This provides:

- Durable storage with MinIO's data protection
- Shared storage across Loki instances (if scaled)
- Consistent storage backend with ClickHouse

**MinIO Configuration:**

| Setting | Value |
|---------|-------|
| Endpoint | `shortas-minio:9000` |
| Bucket | `loki` |
| Access Key | `minioadmin` |
| Secret Key | `minioadmin` |

The `loki` bucket is created automatically by `minio-setup` in `infra/custom/docker-compose.yml`.

Configuration file: `loki/config.yml`

### Log Labels

Each log entry includes:

| Label | Description |
|-------|-------------|
| `service` | Service name (e.g., `click-router`, `shortas-api`) |
| `level` | Log level (`WARN`, `ERROR`) |

### Example Queries

```logql
# All logs
{service=~".+"}

# Specific service
{service="click-router"}

# Error logs only
{service=~".+"} |= "ERROR"

# Search for connection errors
{service=~".+"} |~ "(?i)connection|timeout"

# Logs from last hour with pattern
{service="click-router"} |~ "RabbitMQ" | json
```

## Grafana

### Datasources

Pre-configured datasources (provisioned automatically):

| Name | Type | URL |
|------|------|-----|
| Prometheus | prometheus | http://prometheus:9090 |
| Loki | loki | http://shortas-loki:3100 |

### Accessing Logs

1. Open Grafana at http://localhost:3001
2. Go to **Explore** (compass icon in sidebar)
3. Select **Loki** from the datasource dropdown
4. Enter a LogQL query (e.g., `{service=~".+"}`)
5. Click **Run query**

### Creating Dashboards

1. Go to **Dashboards** > **New** > **New Dashboard**
2. Add panels with Prometheus or Loki queries
3. Save the dashboard

## Configuration

### Environment Variables

Services use these environment variables for Loki:

| Variable | Description | Default |
|----------|-------------|---------|
| `LOKI_URL` | Loki push endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |

### Log Level

To change the log level for a service, update the `RUST_LOG` environment variable in `docker-compose.yml`:

```yaml
environment:
  - RUST_LOG=info  # or debug, warn, error
```

## Troubleshooting

### No logs appearing in Loki

1. Check if Loki is running: `docker logs shortas-loki`
2. Check if the service is sending logs: `docker logs <service-name> | grep -i loki`
3. Verify the LOKI_URL is correct: `http://shortas-loki:3100`
4. Check Loki labels: `curl http://localhost:3100/loki/api/v1/labels`

### Service not appearing in Loki

Services only appear after producing warn/error logs. To test:

```bash
docker restart shortas-click-router
curl http://localhost:3100/loki/api/v1/label/service/values
```

### Grafana can't connect to Loki

1. Check the datasource URL in Grafana (should be `http://shortas-loki:3100`)
2. Verify network connectivity: `docker exec shortas-grafana wget -qO- http://shortas-loki:3100/ready`

## Volumes

| Volume | Purpose |
|--------|---------|
| `prometheus_data` | Prometheus time-series database |
| `grafana_data` | Grafana dashboards and settings |
| `loki_data` | Loki working directory (compactor, WAL) |
| MinIO `loki` bucket | Loki log chunks and index storage |

## Network

All monitoring services run on the `shortas-net` Docker network, which is shared with application services.
