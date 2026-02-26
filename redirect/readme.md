# Shortas Redirect System

Rust workspace containing the high-performance URL redirect and click analytics pipeline.

## Workspace Members

| Crate | Type | Description |
|-------|------|-------------|
| `click-router` | HTTP server | Handles short URL redirects (incl. conditional) and emits click events |
| `click-router-api` | REST API | CRUD operations for routes and short links |
| `click-tracker` | Consumer | Enriches click events with geo, UA, and session data |
| `click-aggregator` | Consumer | Ingests aggregated events into ClickHouse |
| `click-aggregator-api` | REST API | Analytics queries over ClickHouse |
| `route-verifier` | Worker | Checks routes against Google Safe Browsing |
| `route-icon-worker` | Worker | Scrapes and stores favicons from destination URLs |
| `domain-verifier` | Worker | Verifies custom domain ownership via DNS |
| `infra/domains` | Service | Domain resolution and certificate management |

## Data Flow

```
HTTP request → click-router → Fluvio (hit-stream-main)
                                    ↓
                              click-tracker (enrich)
                                    ↓
                              Fluvio (click-aggs-main)
                                    ↓
                              click-aggregator → ClickHouse
```

## Infrastructure

The `docker-compose.yml` in this directory brings up the full stack:

- **MongoDB 7** — route storage (port 27017)
- **ClickHouse** — analytics warehouse (port 8123)
- **Redis 7** — session tracking and cache (port 6379)
- **MinIO** — object storage for ClickHouse data and route icons (port 9002)
- **Fluvio** — event streaming (SC port 9103, SPU ports 9110-9111)
- **RabbitMQ** — cache invalidation and route events (port 5672, management UI port 15672)
- **gglsbl-rest** — local Google Safe Browsing mirror for route verification

## Build

All `make` targets run from the repository root (`../`).

```bash
# Debug build
make build

# Release build
make release

# Run tests
make test

# Clippy
make clippy

# Format
make fmt
```

## Docker

```bash
docker compose up -d
```

This starts all infrastructure and application services. ClickHouse migrations run automatically via the `clickhouse-migrations` init container.

## Configuration

Services are configured via TOML files in each crate's `config/` directory with environment-specific overrides (`development.toml`, `production.toml`). The `APP_RUN_MODE` environment variable selects the active config profile.

## Shared Dependencies

The workspace `Cargo.toml` centralizes dependency versions. Key shared crates:

- `tokio` 1.47 — async runtime
- `salvo` 0.84 — web framework (custom fork in `../salvo/`)
- `mongodb` 3.3 — document store driver
- `clickhouse` 0.13 — analytics driver
- `redis` 0.32 — cache driver
- `fluvio` 0.50 — event streaming client
- `moka` 0.12 — in-memory cache
- `prometheus` 0.13 — metrics export
