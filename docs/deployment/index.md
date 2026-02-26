---
layout: vector-theme
title: Deployment
permalink: /deployment/
---

# Deployment

## Docker Compose

The recommended deployment method runs all services as Docker containers.

### Full Stack

```bash
cd redirect
docker compose up -d
```

This starts:
- **Infrastructure**: MongoDB 7, ClickHouse, Redis 7, MinIO, Fluvio (SC + SPU), RabbitMQ, gglsbl-rest (Safe Browsing)
- **Init containers**: MinIO bucket setup, Fluvio topic creation, ClickHouse migrations
- **Application services**: click-router, click-router-api, click-tracker, click-aggregator, click-aggregator-api, route-verifier, route-icon-worker, domain-verifier, domains

### Management API

```bash
cd api
docker compose up -d
```

Starts the .NET API and its PostgreSQL database.

### UI

```bash
cd ui
docker compose up -d
```

Starts the dashboard and landing page. This compose file includes the redirect compose file via the `include` directive.

## Service Dependencies

```
click-router         → mongo, redis, clickhouse, fluvio (topics), rabbitmq, domains
click-router-api     → mongo, redis, rabbitmq
click-tracker        → redis, fluvio (topics)
click-aggregator     → clickhouse, fluvio (topics)
click-aggregator-api → clickhouse (after migrations)
route-verifier       → mongo, rabbitmq, gglsbl-rest
route-icon-worker    → rabbitmq, minio
domain-verifier      → mongo, rabbitmq
domains              → (standalone)
management-api       → postgresql, elasticsearch, click-router-api, click-aggregator-api
dashboard            → management-api
```

## Environment Configuration

### Rust Services

Configuration is loaded from TOML files in each service's `config/` directory. The `APP_RUN_MODE` environment variable selects the profile:

- `development` — local defaults with debug logging
- `production` — production-optimized settings

Docker containers default to `APP_RUN_MODE=production`.

### Management API

Configured via `appsettings.json` and environment variables:

| Variable | Description |
|----------|-------------|
| `ConnectionStrings__DefaultConnection` | PostgreSQL connection string |
| `Keycloak__Authority` | Keycloak realm URL |
| `Keycloak__Audience` | JWT audience |
| `ApiSettings__ClickRouterApi__BaseUrl` | Click Router API URL |
| `ApiSettings__ClickAggregatorApi__BaseUrl` | Click Aggregator API URL |

## Docker Images

Multi-stage builds minimize image sizes:

- **Rust services**: Built from the workspace Dockerfile, producing a Debian slim image with a non-root user
- **.NET API**: Three-stage build (base, build/publish, runtime) targeting `mcr.microsoft.com/dotnet/aspnet:9.0`
- **UI apps**: Node-based builds outputting static assets

## Health Checks

All services expose health endpoints. Docker Compose healthchecks monitor service status automatically:

```bash
make ps    # Show service status and health
make logs  # Tail service logs
```

## Networking

All services share the `shortas-net` Docker bridge network. Inter-service communication uses container hostnames (e.g., `mongo`, `clickhouse`, `click-router-api`).
