# click-aggregator-api

REST API for querying click analytics stored in ClickHouse. Built on Salvo.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/analytics` | Query click data with time range and dimension filters |
| GET | `/analytics/summary` | Aggregated summary statistics |
| GET | `/health` | Health check |

## Features

- Time-range queries over click event data
- Geographic distribution breakdowns
- Device and browser analytics
- Aggregated summary statistics

## Port

Runs on port 5820.

## Logging & Monitoring

Warning and error logs are sent to Grafana Loki for centralized log aggregation.

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `LOKI_URL` | Loki push endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |

View logs in Grafana:
```logql
{service="click-aggregator-api"}
```

## Dependencies

- ClickHouse — analytics data store

## Migrations

ClickHouse schema migrations are in the `migrations/` directory and applied automatically by the `clickhouse-migrations` init container in Docker Compose. To run manually:

```bash
cd scripts
bash apply_migrations.sh
```

## Build

```bash
# From the repository root
make build-click-aggregator-api

# Or directly
cargo build --manifest-path redirect/Cargo.toml -p click-aggregator-api
```
