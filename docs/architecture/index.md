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
                              │    │
                              │    └──────────────┐
                   ┌──────────┴──────────┐        ▼
                   ▼                     ▼  ┌────────────────┐
          ┌────────────────┐    ┌────────┐  │ Elasticsearch  │
          │ Click Router   │    │ Click  │  │ (route search) │
          │ API (Rust)     │    │Agg. API│  └────────────────┘
          └────────────────┘    └────────┘
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

```
Cache invalidation flow:

  Management API ──▶ Click Router API ──▶ MongoDB write
                                │
                                ▼
                          RabbitMQ (fanout)
                         ╱      │      ╲
                        ▼       ▼       ▼
                   Click     Click     Click
                   Router    Router    Router
                   (Moka)    (Moka)    (Moka)
```

## Services

### Click Router

The entry point for all short URL requests. Built on Salvo (a custom fork) running on Tokio, it resolves the short code, performs the redirect, and publishes a raw click event to Fluvio.

**Processing pipeline:**

1. **URL extraction** — parse the short code from the request path
2. **Route lookup** — query MongoDB (with Moka in-memory cache)
3. **Conditional evaluation** — if the route has conditional routing, evaluate expressions against request context (geo, device, browser, OS)
4. **Registration** — emit a click event to `hit-stream-main`
5. **Result building** — return a redirect response, QR code, proxy, or retarget HTML

**Conditional routing** allows routes to redirect to different destinations based on visitor attributes like country, device type, browser, or operating system. Conditions are evaluated using an expression engine that supports comparison operators and logical combinations.

Each instance runs a RabbitMQ consumer that listens for cache invalidation messages and evicts stale entries from the Moka cache. If RabbitMQ is unavailable, caches degrade gracefully via TTL expiry.

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

REST API for route management — creating, updating, and deleting short links. Supports lookup by composite key (`switch/domain/path`) and by route ID. Uses MongoDB for storage and Redis for caching. After successful writes, publishes cache invalidation messages to RabbitMQ so all Click Router instances can evict stale Moka cache entries. Includes OpenAPI documentation.

### Click Aggregator API

REST API for querying analytics data from ClickHouse. Supports time-range queries, geographic breakdowns, and device/browser distributions.

### Route Verifier

Background worker that protects users from malicious destinations by checking routes against Google Safe Browsing data. Runs on a configurable interval (default: 5 minutes), processing routes in batches.

**Verification flow:**

1. **Route selection** — queries MongoDB for routes due for verification
2. **Safe Browsing check** — validates destinations against local gglsbl-rest API
3. **Status update** — blocks routes with unsafe URLs
4. **Event publishing** — notifies downstream services via RabbitMQ

Routes have two statuses: `Active` (safe) or `Blocked` (unsafe URL detected). Blocked routes include the reason (e.g., "Safe Browsing: MALWARE").

**Recheck intervals:**

- Active routes: every 24 hours
- Blocked routes: every 1 hour (for faster recovery when threats are removed)

### Route Icon Worker

Background worker that automatically extracts favicons from route destination URLs. Listens to RabbitMQ for route creation/update events and stores icons in MinIO/S3 for display in the dashboard.

**Processing flow:**

1. **Event consumption** — receives route events from RabbitMQ
2. **Favicon scraping** — fetches the destination URL and extracts favicon links
3. **Image processing** — downloads and processes the icon
4. **Storage** — uploads the icon to MinIO/S3

### Domain Verifier

Background worker that verifies custom domain ownership through DNS record validation. Users add a TXT record to prove domain ownership before the domain can be used for short links.

**Verification flow:**

1. **Domain registration** — user adds a custom domain in the dashboard
2. **Challenge generation** — system generates a unique TXT record value
3. **DNS lookup** — worker periodically checks for the TXT record
4. **Verification** — domain is marked as verified once the record is found

### Management API

ASP.NET Core 9 service providing workspace management, user settings, domain configuration, and certificate handling. Uses PostgreSQL via Entity Framework Core. Proxies route and analytics requests to the Rust APIs.

Route changes are propagated to Elasticsearch in real time through an outbox pattern (see [Search Index Sync](#search-index-sync) below), enabling full-text search across route links, domain names, and destination URLs.

### Domains Service

Resolves custom domains and serves TLS certificates for the Click Router.

## Data Stores

| Store | Purpose | Data |
|-------|---------|------|
| PostgreSQL | Management state | Workspaces, users, settings, domains, certificates |
| MongoDB | Route data | Short URL mappings, routing policies, link metadata, routes to verify |
| ClickHouse | Analytics | Click events, aggregated metrics, time-series data |
| Elasticsearch | Route search | Full-text index of route links, domains, destinations |
| Redis | Ephemeral state | Sessions, cache entries |
| RabbitMQ | Messaging | Cache invalidation, route status changes |
| MinIO | Object storage | ClickHouse data files, route icons |
| gglsbl-rest | Safe Browsing | Local mirror of Google Safe Browsing threat lists |

## Event Streaming

Fluvio provides the event bus between services:

- **`hit-stream-main`** — raw click events from Click Router to Click Tracker
- **`click-aggs-main`** — enriched events from Click Tracker to Click Aggregator

## Cache Invalidation

RabbitMQ broadcasts cache invalidation events from Click Router API to all Click Router instances using fanout exchanges:

- **`cache.invalidation.routes`** — route create/update/delete events
- **`cache.invalidation.user_settings`** — user settings change events

Each Click Router instance binds an exclusive auto-delete queue to each exchange. Messages are non-persistent (delivery mode 1) since cache invalidation is ephemeral — if RabbitMQ restarts, the Moka cache TTL provides eventual consistency.

## Route Safety Verification

The Route Verifier service protects users from malicious destinations using Google Safe Browsing data:

```
Management API ──▶ MongoDB (routes_to_verify)
                          │
                          ▼
                   route-verifier
                          │
                          ├──▶ gglsbl-rest (Safe Browsing lookup)
                          │
                          ▼
                   RabbitMQ (route.status.changed)
                    ╱                    ╲
                   ▼                      ▼
            Management API          click-router
            (PostgreSQL sync)       (cache update)
```

**Event flow:**

1. Management API writes route with destinations to `routes_to_verify` collection
2. Route Verifier periodically checks destinations against Safe Browsing
3. If unsafe, publishes `RouteStatusChanged` event to RabbitMQ
4. Management API consumes event, updates PostgreSQL, syncs to click-router via outbox
5. Click Router receives status update and blocks redirects for unsafe routes

**RabbitMQ exchange:** `route.status.changed` (fanout)

## Search Index Sync

Elasticsearch provides full-text search over routes. The Management API keeps the index in sync using the transactional outbox pattern:

```
Route CRUD operation
        │
        ▼
┌──────────────────────────┐
│  PostgreSQL transaction   │
│  1. Write route data      │
│  2. Insert outbox message │
└──────────────────────────┘
        │
        ▼
  OutboxProcessorService
  (background worker)
        │
        ▼
┌──────────────────────────┐
│  Elasticsearch            │
│  Index / Delete document  │
└──────────────────────────┘
```

**Outbox event types:**

| Event | Trigger |
|-------|---------|
| `RouteSearchIndex` | Route created or updated |
| `RouteSearchDelete` | Route deleted |
| `RouteSearchBulkIndex` | Bulk route create/update |
| `RouteSearchBulkDelete` | Bulk route delete |

The outbox message is written in the same database transaction as the route change, guaranteeing at-least-once delivery to Elasticsearch. The `OutboxProcessorService` polls for pending messages and applies them. If Elasticsearch is temporarily unreachable, messages remain in the outbox and are retried on the next polling cycle.

A manual reindex endpoint (`POST /api/v1/routes/search/reindex`) is available for initial index population or recovery after index corruption.

## Authentication

Keycloak handles user authentication. The Management API validates JWTs issued by Keycloak, and the Dashboard integrates via `keycloak-js`.
