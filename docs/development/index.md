---
layout: vector-theme
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

## Build

All `make` targets run from the repository root.

### Rust Services

```bash
# Debug build (all services)
make build

# Release build
make release

# Build a specific service
make build-click-router
make build-click-tracker
```

### .NET API

```bash
make build-api
```

### Dashboard

```bash
make build-ui
```

## Test

```bash
# Run all Rust tests
make test

# Test a specific service
make test-click-router

# .NET API tests
make test-api

# Dashboard tests
make test-ui
```

## Lint & Format

```bash
# Clippy
make clippy

# Rustfmt
make fmt

# Cargo check
make check
```

## Infrastructure

```bash
# Start all services (infrastructure + applications)
make up

# Stop
make down

# View logs
make logs

# Service status
make ps
```

## Project Layout

The Rust workspace in `redirect/` is organized as:

```
redirect/
├── Cargo.toml           Workspace root with shared dependencies
├── click-router/        URL redirect service (incl. conditional routing)
├── click-router-api/    Route management API
├── click-tracker/       Event enrichment pipeline
├── click-aggregator/    ClickHouse ingestion
├── click-aggregator-api/ Analytics query API
├── route-verifier/      Safe Browsing verification worker
├── route-icon-worker/   Favicon scraping worker
├── domain-verifier/     Domain ownership verification
├── infra/
│   ├── domains/         Domain resolver service
│   ├── custom/          Docker Compose for local infra
│   └── aws/             LocalStack + Terraform
├── clickhouse/          ClickHouse config (MinIO storage)
├── data/                UA parser data (submodule)
├── scripts/             Setup and utility scripts
└── docker-compose.yml   Full stack compose
```

## Logging

Rust services use `tracing` with `tracing-subscriber`. Set the log level via the `RUST_LOG` environment variable:

```bash
RUST_LOG=info cargo run -p click-router
RUST_LOG=debug cargo run -p click-tracker
```

The .NET API uses Serilog with console and file sinks.
