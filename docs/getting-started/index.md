---
layout: professional-theme
title: Getting Started
permalink: /getting-started/
---

# Getting Started

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | stable | Redirect services and Management API |
| Node.js | >= 18 | Dashboard and landing page |
| Docker | latest | Infrastructure services |
| Docker Compose | v2 | Service orchestration |

The `.NET 9 SDK` is only needed if you want to build the legacy `api/` project (superseded by `management-api/`, kept for reference).

## Clone the Repository

```bash
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas
```

The `--recurse-submodules` flag pulls the Salvo web framework fork and the UA parser data.

## Option 1: Docker Compose (Recommended)

Start the entire stack with a single command, from the top-level orchestrator:

```bash
cd infra/custom
docker compose up -d
```

This file `include`s the Rust redirect stack, the Management API stack, and the UI stack, and directly starts a few shared pieces of infrastructure. In total it brings up:

- **Shared infra**: Keycloak (+ its Postgres DB), Vault (dev mode), RabbitMQ, MinIO/RustFS (+ bucket setup + nginx CORS proxy)
- **Redirect infra**: MongoDB, ClickHouse, Redis, Fluvio (SC + SPU), gglsbl-rest (Safe Browsing), Prometheus/Grafana/Loki
- **Redirect services**: click-router, click-router-api, click-tracker, click-aggregator, click-aggregator-api, route-verifier, route-icon-worker, domain-verifier, cert-bot, pages
- **Management API infra + service**: its own Postgres, Elasticsearch, and the `management-api` container
- **UI**: dashboard, landing

ClickHouse and topic/bucket setup run automatically as init containers.

## Option 2: Local Development

### 1. Start Infrastructure

```bash
make up
```

This runs the redirect workspace's infrastructure and application services in Docker containers (see [Deployment](/deployment/) for what's not covered here, like Keycloak/Vault/Management API — start those via `infra/custom` instead if you need them).

### 2. Build Rust Services

```bash
make build
```

### 3. Run the Management API

```bash
cargo run -p management-api
```

The API starts on port 8080 by default (`config/default.toml`); the Docker Compose setup maps it to host port 8090.

### 4. Build & Run the Dashboard

```bash
make build-ui
cd ui/dashboard && npm start
```

The dashboard opens at `http://localhost:3000`.

## Service Ports

| Service | Port |
|---------|------|
| Click Router | 5800 (HTTP), 4433 (TLS) |
| Click Router API | 5810 |
| Click Aggregator API | 5820 |
| Domain Verifier | 5830 |
| Route Verifier | 5831 |
| Pages (static branded pages) | 5801 |
| Management API | 8090 (host) / 8080 (container) |
| Dashboard | 3000 |
| Landing Page | 3002 |
| Keycloak | 8080 |
| Vault | 8200 |
| MongoDB | 27017 |
| PostgreSQL (Management API) | 5433 |
| Elasticsearch | 9200 |
| ClickHouse (HTTP / native) | 8123 / 9000 |
| Redis | 6379 |
| MinIO / RustFS (API) | 9002 |
| MinIO / RustFS (Console) | 9001 |
| MinIO nginx proxy (CORS) | 9003 |
| RabbitMQ | 5672 |
| RabbitMQ (Management) | 15672 |

## Next Steps

- Read the [Architecture](/architecture/) guide to understand the data flow
- See the [API Reference](/api/) for endpoint details
- Check [Development](/development/) for build and test workflows
