---
layout: professional-theme
title: Development
permalink: /development/
---

# Development

## Repository Setup

```bash
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas
```

Git submodules:
- `salvo/` — custom fork of the Salvo web framework
- `redirect/data/ua-parser/` — UA parser regex database

There are **two Cargo workspaces** in this repo:
- The root `Cargo.toml` — members `management-api` and `redirect/shortas-common`
- `redirect/Cargo.toml` — a separate workspace for the redirect microservices: `click-router`, `click-router-api`, `click-tracker`, `click-aggregator`, `click-aggregator-api`, `cert-bot`, `domain-verifier`, `route-verifier`, `route-icon-worker`

Root `make build`/`make test`/etc. only operate on the `redirect/` workspace. Use plain `cargo` commands from the repo root for `management-api`.

## Build

Unless noted, `make` targets run from the repository root and act on the `redirect/` workspace.

### Redirect Services

```bash
# Debug build (all services)
make build

# Release build
make release

# Build a specific service
make build-click-router
make build-click-tracker
```

### Management API

```bash
# From the repo root (its own workspace, not covered by the Makefile)
cargo build -p management-api
cargo run -p management-api
```

### Legacy C# API

The ASP.NET Core project at `api/` is superseded by `management-api/` and kept only for reference. If you still need it:

```bash
make build-api
```

### Dashboard

```bash
make build-ui
```

## Test

```bash
# Run all redirect/ Rust tests
make test

# Test a specific redirect/ service
make test-click-router

# Management API tests
cargo test -p management-api

# Legacy C# API tests
make test-api

# Dashboard tests
make test-ui
```

## Lint & Format

```bash
# Clippy (redirect/ workspace)
make clippy

# Rustfmt (redirect/ workspace)
make fmt

# Cargo check (redirect/ workspace)
make check
```

Run the equivalent `cargo fmt` / `cargo clippy` directly against the root workspace to cover `management-api`.

## Infrastructure

```bash
# Start redirect/ services (infrastructure + applications)
make up

# Stop
make down

# View logs
make logs

# Service status
make ps
```

For the full stack (Keycloak, Vault, RabbitMQ, MinIO, Management API, UI), use `infra/custom/docker-compose.yml` instead — see [Deployment](/deployment/).

## Project Layout

```
shortas/
├── Cargo.toml              Root workspace: management-api, redirect/shortas-common
├── management-api/         Management API (Rust/Salvo)
│   ├── src/
│   │   ├── presentation/   Controllers, middleware
│   │   ├── application/    DTOs, services
│   │   ├── domain/         Entities, traits
│   │   └── infrastructure/ Repositories, HTTP clients, messaging, search, storage
│   └── migrations/         SQLx migrations (PostgreSQL)
├── api/                     Legacy ASP.NET Core API (reference only)
├── redirect/                Separate Cargo workspace
│   ├── Cargo.toml           Workspace root with shared dependencies
│   ├── click-router/        URL redirect service (incl. conditional routing, QR, ACME)
│   ├── click-router-api/    Route/certificate/user-settings/challenge API
│   ├── click-tracker/       Event enrichment pipeline
│   ├── click-aggregator/    ClickHouse ingestion
│   ├── click-aggregator-api/ Analytics & conversions query API
│   ├── route-verifier/      Safe Browsing verification worker
│   ├── route-icon-worker/   Favicon scraping worker
│   ├── domain-verifier/     Domain ownership verification
│   ├── cert-bot/            Let's Encrypt certificate automation
│   ├── infra/
│   │   ├── pages/           Static branded not-found/blocked pages (nginx)
│   │   └── aws/              LocalStack helpers
│   ├── monitoring/          Prometheus, Grafana, Loki stack
│   ├── data/                UA parser data (submodule)
│   ├── scripts/             Setup and utility scripts
│   └── docker-compose.yml   Redirect services + monitoring
├── infra/custom/            Top-level compose orchestrator (Keycloak, Vault, RabbitMQ, MinIO)
├── infra/aws/terraform/     AWS Terraform modules and environments
└── ui/
    ├── dashboard/           React 18 admin dashboard
    └── landing/             React 19 landing page
```

## Logging

All Rust services, including `management-api`, use `tracing` with `tracing-subscriber`. Set the log level via the `RUST_LOG` environment variable:

```bash
RUST_LOG=info cargo run -p click-router
RUST_LOG=debug cargo run -p management-api
```
