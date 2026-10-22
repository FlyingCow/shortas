# Shortas

A high-performance URL shortener and analytics platform with real-time click tracking, geographic enrichment, conditional routing, QR code generation, and multi-tenant workspace support.

## Architecture

Shortas is composed of multiple subsystems working together:

```
                         ┌──────────────────┐
                         │   Dashboard UI   │ React 18 / TypeScript
                         │    (port 3000)   │
                         └────────┬─────────┘
                                  │
                         ┌────────▼─────────┐
                         │  Management API  │ Rust / Salvo
                         │    (port 8090)   │ PostgreSQL, Elasticsearch
                         └────────┬─────────┘
                                  │
      ┌───────────────────────────┼───────────────────────────┐
      │                           │                           │
┌─────▼──────┐  ┌────────────────▼────────────────┐  ┌───────▼────────┐
│Click Router│  │         Background Workers       │  │Click Aggregator│
│ (port 5800)│  │  ┌─────────────┬─────────────┐  │  │     API        │
│Rust / Salvo│  │  │Route        │Route Icon   │  │  │ Rust / Salvo   │
└─────┬──────┘  │  │Verifier     │Worker       │  │  └───────┬────────┘
      │         │  │(Safe Browse)│(Favicons)   │  │          │
      │         │  ├─────────────┼─────────────┤  │          │
      │         │  │Domain       │Cert Bot     │  │          │
      │         │  │Verifier(DNS)│(Let's Encr.)│  │          │
      │         │  └──────┬──────┴──────┬──────┘  │          │
      │         └─────────┼─────────────┼─────────┘          │
      │                   │             │                    │
      └───────┬───────────┼─────────────┼────────────────────┘
              │           │             │
              │ Fluvio    │ RabbitMQ    │
              │           │             │
    ┌─────────┼───────────┼─────────────┼─────────┐
    │         │           │             │         │
 MongoDB   Redis      RabbitMQ      MinIO    ClickHouse
 (routes,  (sessions)  (events)   (images,   (analytics)
  certs)                          CH storage)

                    Monitoring Stack
    ┌─────────────────────────────────────────────┐
    │                                             │
    │  Prometheus ◄─── metrics ───┐               │
    │  (port 9091)                │               │
    │       │                All Services         │
    │       ▼                     │               │
    │   Grafana ◄─── logs ── Loki ◄───────────────┤
    │  (port 3001)         (port 3100)            │
    │                                             │
    └─────────────────────────────────────────────┘
```

**Click Router** handles incoming short URL requests, performs redirects (including conditional routing based on geo, device, or browser), serves QR codes and ACME challenge responses, and emits click events to Fluvio. **Click Tracker** consumes raw events, enriches them with geolocation (MaxMind), user-agent parsing, and session tracking (Redis), then publishes aggregated events. **Click Aggregator** consumes aggregated events and stores them in ClickHouse for analytics. Additional workers handle route safety verification (Safe Browsing), favicon scraping, domain ownership verification, and TLS certificate issuance. Each Rust service has a companion REST API for management and querying, and the **Management API** (also Rust/Salvo) is the single backend the Dashboard talks to, proxying route and analytics calls to the internal Click Router API and Click Aggregator API.

> **Note:** `api/` is the previous ASP.NET Core / C# implementation of the Management API. It has been superseded by the Rust service in `management-api/` and is kept only for reference — it is no longer part of the active stack or CI.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Management API | Rust, Salvo, SQLx, PostgreSQL, Elasticsearch |
| Redirect services | Rust, Salvo, Tokio |
| Event streaming | Fluvio |
| Messaging | RabbitMQ |
| Document store | MongoDB 7 (routes, user settings, TLS certificates) |
| Cache / sessions | Redis 7 |
| Analytics store | ClickHouse (MinIO-backed) |
| Object storage | MinIO (RustFS) |
| Route safety | gglsbl-rest (Google Safe Browsing) |
| Dashboard | React 18, TypeScript, Bootstrap 5 |
| Landing page | React 19, TypeScript |
| Auth | Keycloak (local) / AWS Cognito (AWS) |
| Secrets | HashiCorp Vault (dev mode, provisioned for future use) |
| Monitoring | Prometheus, Grafana, Loki |

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) >= 18
- [Docker](https://docs.docker.com/get-started/) & Docker Compose

`.NET 9 SDK` is only needed if you want to build the legacy `api/` project — it is not required for the current stack.

## Quick Start

### Docker Compose (full stack)

```bash
cd infra/custom
docker compose up -d
```

This is the top-level compose file: it pulls in the Rust redirect services (`redirect/docker-compose.yml`, which itself includes the monitoring stack), the Management API (`management-api/docker-compose.yml`), and the UI (`ui/docker-compose.yml`), and additionally starts Keycloak, Vault, RabbitMQ, and MinIO/RustFS directly.

### Local Development

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas

# Start infrastructure + all Rust services
make up

# Run the Management API locally
cargo run -p management-api

# In a separate terminal, start the dashboard
cd ui/dashboard
npm install && npm start
```

### AWS Deployment

For production deployments, Shortas can be deployed to AWS using Terraform. This replaces local services with AWS managed equivalents:

| Local | AWS |
|-------|-----|
| MongoDB | DynamoDB |
| PostgreSQL | RDS Aurora |
| Redis | ElastiCache |
| MinIO | S3 |
| RabbitMQ | Amazon MQ |
| Keycloak | AWS Cognito |

```bash
# Deploy to AWS (dev environment)
cd infra/aws/terraform/environments/dev
terraform init
terraform apply

# Build and push Docker images
cd ../scripts
./build-push-images.sh dev

# Deploy services
./deploy-services.sh dev
```

See [infra/aws/terraform/README.md](infra/aws/terraform/README.md) for detailed AWS deployment instructions.

## Project Structure

```
shortas/
├── management-api/         Management API (Rust/Salvo) - workspaces, routes, domains, certs
├── api/                     Legacy ASP.NET Core management API (superseded, kept for reference)
├── redirect/                Rust microservices workspace
│   ├── click-router/        URL redirect handler
│   ├── click-router-api/    Route management REST API
│   ├── click-tracker/       Click event enrichment pipeline
│   ├── click-aggregator/    ClickHouse ingestion consumer
│   ├── click-aggregator-api/ Analytics & conversions query API
│   ├── route-verifier/      Safe Browsing verification worker
│   ├── route-icon-worker/   Favicon scraping worker
│   ├── domain-verifier/     Domain ownership verification
│   ├── cert-bot/            Let's Encrypt certificate automation
│   ├── infra/pages/         Static branded error/blocked pages (nginx)
│   ├── monitoring/          Prometheus, Grafana, Loki stack
│   └── docker-compose.yml   Rust services + monitoring
├── infra/
│   ├── custom/              Top-level local compose orchestrator (Keycloak, Vault, RabbitMQ, MinIO)
│   └── aws/terraform/       AWS Terraform modules and environments
├── ui/
│   ├── dashboard/           React admin dashboard
│   ├── landing/             React landing page
│   └── docker-compose.yml   Dashboard + landing page
├── salvo/                   Salvo web framework fork (git submodule)
├── docs/                    Jekyll documentation site
├── Cargo.toml                Root workspace (management-api, redirect/shortas-common)
└── Makefile                  Root build orchestration
```

## Services

| Service | Port | Description |
|---------|------|--------------|
| click-router | 4433 (TLS), 5800 (HTTP) | Redirect handler, QR codes, ACME challenges |
| click-router-api | 5810 | Route CRUD, certificates, user settings (internal) |
| click-tracker | - | Event enrichment consumer |
| click-aggregator | 9090 | ClickHouse ingestion consumer (metrics) |
| click-aggregator-api | 5820 | Analytics & conversions query API |
| route-verifier | 5831 | Safe Browsing verification worker |
| route-icon-worker | - | Favicon scraping worker |
| domain-verifier | 5830 | Domain ownership verification |
| cert-bot | - | Let's Encrypt certificate automation |
| pages | 5801 | Static branded not-found/blocked/index pages |
| management-api | 8090 | Workspace, route, domain & user management |
| dashboard | 3000 | Admin UI |
| landing | 3002 | Marketing site |
| keycloak | 8080 | Local authentication |
| vault | 8200 | Secrets store (dev mode) |
| rabbitmq | 5672, 15672 (mgmt) | Messaging |
| minio (RustFS) | 9002 (API), 9001 (console), 9003 (nginx/CORS) | Object storage |
| postgres (management-api) | 5433 | Management API database |
| elasticsearch | 9200 | Route search index |
| mongo | 27017 | Route & certificate storage |
| clickhouse | 8123 (HTTP), 9000 (native) | Analytics storage |
| redis | 6379 | Cache / sessions |
| prometheus | 9091 | Metrics collection |
| grafana | 3001 | Monitoring dashboards |
| loki | 3100 | Log aggregation |

## Make Targets

The root `Makefile` covers the Rust redirect workspace, the legacy C# API, and the UI. There is no `make help`; key targets:

```
make build            Build all redirect/ services (debug)
make build-%          Build one redirect/ service, e.g. make build-click-router
make release          Build all redirect/ services (release)
make test             Run all redirect/ tests
make clippy           Run cargo clippy
make fmt              Run cargo fmt
make up / down        Start/stop redirect/ docker compose stack
make logs / ps        Tail logs / show service status
```

`management-api` is not yet wired into the Makefile — build/test/run it directly from the repo root:

```bash
cargo build -p management-api
cargo test -p management-api
cargo run -p management-api
```

## Documentation

Full documentation is hosted at [shortas.work](https://shortas.work/) and built with Jekyll from the `docs/` directory.

## License

MIT
