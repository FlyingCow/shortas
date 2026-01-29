# Shortas

A high-performance URL shortener and analytics platform with real-time click tracking, geographic enrichment, and multi-tenant workspace support.

## Architecture

Shortas is composed of three main subsystems:

```
                    ┌──────────────────┐
                    │   Dashboard UI   │ React 18 / TypeScript
                    │    (port 3000)   │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │  Management API  │ ASP.NET Core 9 / C#
                    │    (port 5050)   │ PostgreSQL
                    └────────┬─────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
┌────────▼─────────┐ ┌──────▼───────┐ ┌────────▼─────────┐
│   Click Router   │ │Click Tracker │ │ Click Aggregator │
│   (port 5800)    │ │  (consumer)  │ │    (consumer)    │
│   Rust / Salvo   │ │ Rust / Tokio │ │  Rust / Tokio    │
└──────────────────┘ └──────────────┘ └──────────────────┘
         │                   │                   │
         └───────┬───────────┘                   │
                 │ Fluvio streams                 │
                 │ hit-stream-main                │
                 │ click-aggs-main ───────────────┘
                 │
    ┌────────────┼────────────┐
    │            │            │
 MongoDB      Redis      ClickHouse
 (routes)   (sessions)  (analytics)
```

**Click Router** handles incoming short URL requests, performs redirects, and emits click events to Fluvio. **Click Tracker** consumes raw events, enriches them with geolocation (MaxMind), user-agent parsing, and session tracking (Redis), then publishes aggregated events. **Click Aggregator** consumes aggregated events and stores them in ClickHouse for analytics. Each component has a companion REST API for management and querying.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Management API | ASP.NET Core 9, Entity Framework Core, PostgreSQL |
| Redirect services | Rust, Salvo, Tokio |
| Event streaming | Fluvio |
| Document store | MongoDB 7 |
| Cache / sessions | Redis 7 |
| Analytics store | ClickHouse (MinIO-backed) |
| Object storage | MinIO |
| Dashboard | React 18, TypeScript, Bootstrap 5 |
| Auth | Keycloak (JWT) |

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

This starts all infrastructure (MongoDB, ClickHouse, Redis, MinIO, Fluvio) and application services.

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
│   ├── infra/              Infrastructure (domains service, AWS/custom)
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
| click-router-api | 8081 | Route CRUD API |
| click-tracker | - | Event enrichment consumer |
| click-aggregator | - | ClickHouse ingestion consumer |
| click-aggregator-api | 8082 | Analytics query API |
| domains | 5801 | Domain & certificate management |
| management API | 5050 | Workspace, route & user management |
| dashboard | 3000 | Admin UI |
| landing | 3001 | Marketing site |

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

Full documentation is hosted at [shortas.tech](https://shortas.tech/) and built with Jekyll from the `docs/` directory.

## License

MIT
