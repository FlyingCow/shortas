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
                         │  Management API  │ ASP.NET Core 9 / C#
                         │    (port 5050)   │ PostgreSQL, Elasticsearch
                         └────────┬─────────┘
                                  │
      ┌───────────────────────────┼───────────────────────────┐
      │                           │                           │
┌─────▼──────┐  ┌────────────────▼────────────────┐  ┌───────▼────────┐
│Click Router│  │         Background Workers       │  │Click Aggregator│
│ (port 5800)│  │  ┌─────────────┬─────────────┐  │  │   (consumer)   │
│Rust / Salvo│  │  │Route        │Route Icon   │  │  │ Rust / Tokio   │
└─────┬──────┘  │  │Verifier     │Worker       │  │  └───────┬────────┘
      │         │  │(Safe Browse)│(Favicons)   │  │          │
      │         │  ├─────────────┼─────────────┤  │          │
      │         │  │Domain       │Click Tracker│  │          │
      │         │  │Verifier     │(Enrichment) │  │          │
      │         │  │(DNS)        │             │  │          │
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
 (routes) (sessions)  (events)     (icons)  (analytics)

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

**Click Router** handles incoming short URL requests, performs redirects (including conditional routing based on geo, device, or browser), and emits click events to Fluvio. **Click Tracker** consumes raw events, enriches them with geolocation (MaxMind), user-agent parsing, and session tracking (Redis), then publishes aggregated events. **Click Aggregator** consumes aggregated events and stores them in ClickHouse for analytics. Additional workers handle route safety verification (Safe Browsing), favicon scraping, and domain ownership verification. Each component has a companion REST API for management and querying.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Management API | ASP.NET Core 9, Entity Framework Core, PostgreSQL, Elasticsearch |
| Redirect services | Rust, Salvo, Tokio |
| Event streaming | Fluvio |
| Messaging | RabbitMQ |
| Document store | MongoDB 7 |
| Cache / sessions | Redis 7 |
| Analytics store | ClickHouse (MinIO-backed) |
| Object storage | MinIO |
| Route safety | gglsbl-rest (Google Safe Browsing) |
| Dashboard | React 18, TypeScript, Bootstrap 5 |
| Auth | Keycloak (local) / AWS Cognito (AWS) |
| Monitoring | Prometheus, Grafana, Loki |

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [.NET 9 SDK](https://dotnet.microsoft.com/download)
- [Node.js](https://nodejs.org/) >= 18
- [Docker](https://docs.docker.com/get-started/) & Docker Compose

## Quick Start

### Docker Compose (full stack)

```bash
cd redirect
docker compose up -d
```

This starts all infrastructure (MongoDB, ClickHouse, Redis, MinIO, Fluvio, RabbitMQ, gglsbl-rest) and application services.

### Local Development

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas

# Start infrastructure
cd redirect
make infra-start

# Build all Rust services
make build

# In a separate terminal, start the .NET API
cd api
dotnet run

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
├── api/                    ASP.NET Core management API
├── redirect/               Rust microservices workspace
│   ├── click-router/       URL redirect handler
│   ├── click-router-api/   Route management REST API
│   ├── click-tracker/      Click event enrichment pipeline
│   ├── click-aggregator/   ClickHouse ingestion consumer
│   ├── click-aggregator-api/ Analytics query API
│   ├── route-verifier/     Safe Browsing verification worker
│   ├── route-icon-worker/  Favicon scraping worker
│   ├── domain-verifier/    Domain ownership verification
│   ├── cert-bot/           Let's Encrypt certificate automation
│   ├── monitoring/         Prometheus, Grafana, Loki stack
│   ├── infra/              Infrastructure configs
├── infra/
│   └── aws/terraform/      AWS Terraform modules and environments
│   ├── clickhouse/         ClickHouse configuration
│   └── docker-compose.yml  Full stack compose file
├── ui/
│   ├── dashboard/          React admin dashboard
│   └── landing/            React landing page
├── salvo/                  Salvo web framework (git submodule)
├── docs/                   Jekyll documentation site
└── makefile                Root build orchestration
```

## Services

| Service | Port | Description |
|---------|------|-------------|
| click-router | 5800 | HTTP redirect & click capture |
| click-router-api | 5810 | Route CRUD API |
| click-tracker | - | Event enrichment consumer |
| click-aggregator | 9090 | ClickHouse ingestion consumer (metrics) |
| click-aggregator-api | 5820 | Analytics query API |
| route-verifier | 5831 | Safe Browsing verification worker |
| route-icon-worker | - | Favicon scraping worker |
| domain-verifier | 5830 | Domain ownership verification |
| cert-bot | - | Let's Encrypt certificate automation |
| management API | 5050 | Workspace, route & user management |
| dashboard | 3000 | Admin UI |
| prometheus | 9091 | Metrics collection |
| grafana | 3001 | Monitoring dashboards |
| loki | 3100 | Log aggregation |

## Make Targets

Run `make help` from the `redirect/` directory for the full list. Key targets:

```
make build            Build all services (debug)
make build-release    Build all services (release)
make test             Run all tests
make dev-setup        Full development environment setup
make infra-start      Start infrastructure services
make deploy-docker    Deploy with Docker Compose
make health-check     Check all service health
make lint             Run cargo clippy
make format           Run cargo fmt
```

## Documentation

Full documentation is hosted at [shortas.work](https://shortas.work/) and built with Jekyll from the `docs/` directory.

## License

MIT
