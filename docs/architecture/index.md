---
layout: vector-theme
title: Architecture
permalink: /architecture/
---

# Architecture

## High-Level Overview

Shortas follows an event-driven microservices architecture. Three subsystems collaborate to deliver URL shortening, real-time analytics, and management:

```
┌────────────────┐     ┌──────────────────┐     ┌────────────────┐
│  Dashboard UI  │────▶│  Management API   │────▶│   PostgreSQL   │
│  React 18      │     │  ASP.NET Core 9   │     │                │
└────────────────┘     └──────────────────┘     └────────────────┘
                              │
                   ┌──────────┴──────────┐
                   ▼                     ▼
          ┌────────────────┐    ┌────────────────┐
          │ Click Router   │    │ Click Agg. API │
          │ API (Rust)     │    │ (Rust)         │
          └────────────────┘    └────────────────┘
```

```
Short URL click flow:

  Browser ──▶ Click Router ──▶ HTTP Redirect
                    │
                    ▼
              Fluvio topic: hit-stream-main
                    │
                    ▼
              Click Tracker
              (geo, UA, session enrichment)
                    │
                    ▼
              Fluvio topic: click-aggs-main
                    │
                    ▼
              Click Aggregator ──▶ ClickHouse
```

## Services

### Click Router

The entry point for all short URL requests. Built on Salvo (a custom fork) running on Tokio, it resolves the short code, performs the redirect, and publishes a raw click event to Fluvio.

**Processing pipeline:**

1. **URL extraction** — parse the short code from the request path
2. **Route lookup** — query MongoDB (with Moka in-memory cache)
3. **Registration** — emit a click event to `hit-stream-main`
4. **Result building** — return a redirect response, QR code, proxy, or retarget HTML

Supports TLS with dynamic certificate resolution via the Domains service.

### Click Tracker

A Tokio-based stream consumer that reads raw click events and enriches them:

- **Geolocation** — MaxMind GeoIP database lookup
- **User-Agent parsing** — browser, OS, and device extraction
- **Session tracking** — Redis-backed session identification
- **Aggregation** — groups events for batch processing

Publishes enriched events to `click-aggs-main`.

### Click Aggregator

Consumes from `click-aggs-main` and batch-inserts events into ClickHouse. ClickHouse uses MinIO as its object storage backend for cost-efficient data retention.

### Click Router API

REST API for route management — creating, updating, and deleting short links. Uses MongoDB for storage and Redis for caching. Includes OpenAPI documentation.

### Click Aggregator API

REST API for querying analytics data from ClickHouse. Supports time-range queries, geographic breakdowns, and device/browser distributions.

### Management API

ASP.NET Core 9 service providing workspace management, user settings, domain configuration, and certificate handling. Uses PostgreSQL via Entity Framework Core. Proxies route and analytics requests to the Rust APIs.

### Domains Service

Resolves custom domains and serves TLS certificates for the Click Router.

## Data Stores

| Store | Purpose | Data |
|-------|---------|------|
| PostgreSQL | Management state | Workspaces, users, settings, domains, certificates |
| MongoDB | Route data | Short URL mappings, routing policies, link metadata |
| ClickHouse | Analytics | Click events, aggregated metrics, time-series data |
| Redis | Ephemeral state | Sessions, cache entries |
| MinIO | Object storage | ClickHouse data files |

## Event Streaming

Fluvio provides the event bus between services:

- **`hit-stream-main`** — raw click events from Click Router to Click Tracker
- **`click-aggs-main`** — enriched events from Click Tracker to Click Aggregator

## Authentication

Keycloak handles user authentication. The Management API validates JWTs issued by Keycloak, and the Dashboard integrates via `keycloak-js`.
