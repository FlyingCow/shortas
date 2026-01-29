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

### Rust Services

```bash
cd redirect

# Debug build
make build

# Release build
make build-release

# Build a specific service
make build-click-router
make build-click-tracker
```

### .NET API

```bash
cd api
dotnet build
```

### Dashboard

```bash
cd ui/dashboard
npm install
npm run build
```

### Landing Page

```bash
cd ui/landing
npm install
npm run build
```

## Test

```bash
cd redirect

# Run all tests
make test

# Watch mode
make test-watch

# Coverage (generates HTML report in coverage/)
make test-coverage
```

## Lint & Format

```bash
cd redirect

# Clippy
make lint

# Rustfmt
make format

# Full check (lint + test + build)
make check
```

## Infrastructure

Start the backing services locally:

```bash
cd redirect

# Custom stack (MongoDB, ClickHouse, Redis, MinIO, Fluvio)
make infra-start-custom

# Or AWS LocalStack (DynamoDB, Kinesis)
make infra-start-aws

# Stop
make infra-stop

# Clean reset (removes volumes)
make infra-reset
```

## Validate Environment

```bash
cd redirect
make validate
```

Checks that Rust, Cargo, Docker, and Docker Compose are installed.

## Project Layout

The Rust workspace in `redirect/` is organized as:

```
redirect/
├── Cargo.toml           Workspace root with shared dependencies
├── click-router/        URL redirect service
├── click-router-api/    Route management API
├── click-tracker/       Event enrichment pipeline
├── click-aggregator/    ClickHouse ingestion
├── click-aggregator-api/ Analytics query API
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
