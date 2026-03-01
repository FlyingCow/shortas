---
layout: professional-theme
title: Getting Started
permalink: /getting-started/
---

# Getting Started

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | stable | Redirect services |
| .NET SDK | 9.0 | Management API |
| Node.js | >= 18 | Dashboard and landing page |
| Docker | latest | Infrastructure services |
| Docker Compose | v2 | Service orchestration |

## Clone the Repository

```bash
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas
```

The `--recurse-submodules` flag pulls the Salvo web framework fork and the UA parser data.

## Option 1: Docker Compose (Recommended)

Start the entire stack with a single command:

```bash
cd redirect
docker compose up -d
```

This brings up:
- Infrastructure: MongoDB, ClickHouse, Redis, MinIO, Fluvio, RabbitMQ, gglsbl-rest (Safe Browsing)
- Application services: click-router, click-router-api, click-tracker, click-aggregator, click-aggregator-api, route-verifier, route-icon-worker, domain-verifier, domains
- ClickHouse migrations (runs automatically)

## Option 2: Local Development

### 1. Start Infrastructure

```bash
make up
```

This runs all infrastructure and application services in Docker containers.

### 2. Build Rust Services

```bash
make build
```

### 3. Build & Run the Management API

```bash
make build-api
cd api && dotnet run
```

The API starts on port 5050 with Swagger UI at `http://localhost:5050/swagger`.

### 4. Build & Run the Dashboard

```bash
make build-ui
cd ui/dashboard && npm start
```

The dashboard opens at `http://localhost:3000`.

## Service Ports

| Service | Port |
|---------|------|
| Click Router | 5800 |
| Click Router API | 8081 |
| Click Aggregator API | 8082 |
| Domains | 5801 |
| Management API | 5050 |
| Dashboard | 3000 |
| Landing Page | 3001 |
| MongoDB | 27017 |
| ClickHouse (HTTP) | 8123 |
| Redis | 6379 |
| MinIO (API) | 9002 |
| MinIO (Console) | 9001 |
| RabbitMQ | 5672 |
| RabbitMQ (Management) | 15672 |

## Next Steps

- Read the [Architecture](/architecture/) guide to understand the data flow
- See the [API Reference](/api/) for endpoint details
- Check [Development](/development/) for build and test workflows
