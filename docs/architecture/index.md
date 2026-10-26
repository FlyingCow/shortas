---
layout: professional-theme
title: Architecture
permalink: /architecture/
---

# Architecture

## High-Level Overview

Shortas follows an event-driven microservices architecture. All services — including the Management API — are written in Rust on Salvo/Tokio, except the two React frontends:

```
┌────────────────┐     ┌──────────────────┐     ┌────────────────┐
│  Dashboard UI  │────▶│  Management API   │────▶│   PostgreSQL   │
│  React 18      │     │  Rust / Salvo      │     │                │
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
5. **Result building** — return a redirect, QR code, proxy, retarget HTML, paused/blocked/not-found page, `robots.txt`, or injected Open Graph tags, depending on route state

**Conditional routing** allows routes to redirect to different destinations based on visitor attributes like country, device type, browser, or operating system. Conditions are evaluated using an expression engine that supports comparison operators and logical combinations.

Click Router also serves QR codes for routes, answers ACME HTTP-01 challenges at `/.well-known/acme-challenge/{token}` (backed by Click Router API, used by Cert Bot), and falls back to the [Pages](#pages) service for branded not-found/blocked/paused responses.

Each instance runs a RabbitMQ consumer that listens for cache invalidation messages and evicts stale entries from the Moka cache. If RabbitMQ is unavailable, caches degrade gracefully via TTL expiry.

Supports TLS with certificates read directly from MongoDB — see [TLS Certificates](#tls-certificates) below.

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

Internal REST API for route, certificate, user-settings, and ACME challenge management, backed by MongoDB. Supports lookup by composite key (`switch/domain/path`) and by route ID, plus bulk operations. After successful writes it publishes cache invalidation messages to RabbitMQ so all Click Router instances evict stale Moka cache entries. Documented with OpenAPI at `/api-doc/openapi.json` (Swagger UI at `/swagger-ui`).

This API is intended to be called only by other internal services (Management API, Cert Bot, Route Verifier) — JWT/validation middleware is not currently enabled on it, so it should not be exposed outside the private network.

### Click Aggregator API

REST API for querying analytics data from ClickHouse: time-range stats, daily/hourly breakdowns, geographic and device/browser distributions, route performance and top destinations, plus a **conversions** endpoint group (conversion rates, revenue, attribution, funnels, summary). Documented with OpenAPI/Swagger, same pattern as Click Router API.

### Route Verifier

Background worker that protects users from malicious destinations by checking routes against Google Safe Browsing data. Runs on a configurable interval (default: 5 minutes), processing routes in batches.

**Verification flow:**

1. **Route selection** — queries MongoDB for routes due for verification
2. **Safe Browsing check** — validates destinations against the local `gglsbl-rest` API
3. **Status update** — updates the route's status via Click Router API (which persists to MongoDB and triggers the normal cache-invalidation flow)
4. **Event publishing** — publishes `route.status.changed` to RabbitMQ so the Management API can sync its own view

Routes have two statuses: `Active` (safe) or `Blocked` (unsafe URL detected). Blocked routes include the reason (e.g., "Safe Browsing: MALWARE").

**Recheck intervals:**

- Active routes: every 24 hours
- Blocked routes: every 1 hour (for faster recovery when threats are removed)

### Route Icon Worker

Background worker that automatically extracts favicons from route destination URLs. Listens to RabbitMQ (`cache.invalidation.routes`) for route creation/update events and stores icons in MinIO/S3 for display in the dashboard.

**Processing flow:**

1. **Event consumption** — receives route events from RabbitMQ
2. **Favicon scraping** — fetches the destination URL and extracts favicon links
3. **Image processing** — downloads and processes the icon
4. **Storage** — uploads the icon to MinIO/S3

### Domain Verifier

Background worker that verifies custom domain ownership through DNS record validation. Users add a TXT record to prove domain ownership before the domain can be used for short links. Publishes `domain.state.changed` once a domain's verification state changes.

**Verification flow:**

1. **Domain registration** — user adds a custom domain in the dashboard
2. **Challenge generation** — system generates a unique TXT record value
3. **DNS lookup** — worker periodically checks for the TXT record via Hickory DNS
4. **Verification** — domain is marked as verified once the record is found, and `domain.state.changed` is published

### Cert Bot

Background worker that automates TLS certificate issuance and renewal via Let's Encrypt (ACME), using `instant-acme`. It consumes `domain.state.changed` so newly-verified domains get a certificate automatically, serves ACME HTTP-01 challenges through Click Router API's `/v1/challenges/{domain}/{token}` endpoint (answered by Click Router at `/.well-known/acme-challenge/{token}`), and writes issued certificates back through Click Router API's `/v1/certificates/{domain}` endpoints. It also runs a renewal check on a daily interval.

### Pages

A small nginx service (`redirect/infra/pages`) that serves static, branded HTML for `/index`, `/not-found`, and `/blocked`. Click Router forwards visitors here (via configured URLs) when a short link doesn't exist or has been blocked, instead of showing a generic error. It has no relationship to domain resolution or TLS — that is handled by Click Router itself plus Cert Bot (see below).

### Management API

Rust/Salvo service providing workspace management (including email-based member invitations), domain configuration, certificate listing, user onboarding, and analytics proxying for the Dashboard. Uses PostgreSQL via SQLx. All `/api/v1/*` routes are protected by JWT authentication (validated against Keycloak/Cognito) and rate limiting, except health checks, the shared-domains lookup, and the invitation-by-token preview.

**Endpoint groups:**

| Group | Base path | Purpose |
|-------|-----------|---------|
| Routes | `/api/v1/routes` | CRUD, bulk operations, search, QR settings/uploads, unblock |
| Domains | `/api/v1/domains` | Custom domain CRUD, DNS config, verification |
| Workspaces | `/api/v1/workspaces` | Multi-tenant workspace & membership management |
| Invitations | `/api/v1/workspaces/{id}/invitations`, `/api/v1/invitations` | Email-based member invitations: create/list/revoke (workspace-scoped, Admin+), accept/decline, and a public by-token preview |
| Notifications | `/api/v1/notifications` | In-app notification center (e.g. workspace invite alerts) |
| Users | `/api/v1/users` | Email-prefix search backing the invite-by-email typeahead |
| Certificates | `/api/v1/certificates` | Certificate listing (issued by Cert Bot) |
| Clickstream | `/api/v1/clickstream` | Analytics proxy to Click Aggregator API |
| User | `/api/v1/user` | Profile, onboarding, initialization status |

Route changes affecting search are reflected in Elasticsearch, which backs `GET /api/v1/routes/search`; `POST /api/v1/routes/search/reindex` is available to rebuild the index manually.

**Workspace roles & invitations:** every workspace member has a role of **Owner** (the creator; exactly one per workspace, never reassigned or removed), **Admin** (manages members and invitations), or **Member**. Inviting a user by email creates a `WorkspaceInvitation` with a random token (only its hash is stored, 7-day TTL); accepting it — either via the emailed link or from the in-app notification — creates the membership, marks the invitation accepted, and marks the originating notification `Actioned` so it stops reappearing in the bell. Both the invite email and the in-app notification are delivered through the generic notification pipeline (see below) via a single `PublishNotificationEvent` outbox message, rather than sent inline from the request handler.

### Notification Worker

A separate Rust/Tokio binary that renders and delivers the notifications any Management API action triggers — currently the workspace-invite flow, but built to be generic to future notification types. It has no HTTP endpoints; it only consumes from RabbitMQ, and depends on `management-api` as a library (reusing its `Notification`/`User` repositories, `EmailSender`, and S3 client) rather than duplicating domain code.

**Pipeline:**

1. **Event creation** — an action handler (e.g. `POST /api/v1/workspaces/{id}/invitations`) builds a `NotificationEvent` (`notification_id` template key, `recipient` { `user_id`, `email` }, `context` JSON) and writes it as a `PublishNotificationEvent` outbox message in the same DB transaction as the business action.
2. **Publish** — the same transactional Outbox processor that handles Elasticsearch indexing and Safe Browsing checks publishes the event onto the `notifications` topic exchange (routing key = `notification_id`).
3. **Template fetch** — Notification Worker fetches `notification-templates/<notification_id>/{metadata.json, email.subject.hbs, email.body.hbs, in_app.hbs}` from the same S3/MinIO bucket Management API already uses for QR codes/favicons, cached in-memory via Moka.
4. **Validation** — the event's `context` is checked against `metadata.json`'s `required_context`; a missing key fails fast instead of rendering a half-empty template.
5. **Render & deliver** — Handlebars renders each channel `metadata.json` declares:
   - `in_app` → writes a `Notification` row (PostgreSQL) with the rendered text, shown in the dashboard's notification bell
   - `email` → renders subject + HTML body, sent via the same `EmailSender` (SES/SMTP) Management API uses
6. **Retry / dead-letter** — a failed delivery is nacked (no requeue) and dead-lettered into a retry queue with a TTL-based delay, looping back onto the main exchange; after `notifications_max_retries` redeliveries it's routed to a dead queue instead of retrying forever.

**Notification templates** live in the repo under `notification-templates/<notification_id>/` and are synced to S3 as part of deploy (see [Deployment](/deployment/)) — so adding a notification type is a template PR, not a Rust or frontend change. `metadata.json` declares `channels` (`email`, `in_app`) and `required_context`; `email.subject.hbs`/`email.body.hbs` render the email, `in_app.hbs` renders the bell's display text.

## TLS Certificates

There is no separate "domain resolver" service. Certificates live in MongoDB and are read directly by Click Router through a cached `CryptoStore`/`CryptoCache` abstraction (Moka-backed, with MongoDB and DynamoDB adapters) keyed by server name — no HTTP round-trip is needed on the TLS handshake path.

```
Domain Verifier ──▶ domain.state.changed (RabbitMQ)
                            │
                            ▼
                         Cert Bot
                            │
          ┌─────────────────┴──────────────────┐
          ▼                                     ▼
  Click Router API                       Let's Encrypt (ACME)
  /v1/challenges/{domain}/{token}        HTTP-01 challenge
  (written by Cert Bot,                  answered by Click Router at
   served by Click Router)               /.well-known/acme-challenge/{token}
          │
          ▼
  Click Router API
  /v1/certificates/{domain}
  (issued cert written here, stored in MongoDB)
          │
          ▼
  Click Router reads the cert directly from MongoDB
  via its CryptoCache for TLS termination
```

## Data Stores

| Store | Purpose | Data |
|-------|---------|------|
| PostgreSQL | Management API state | Workspaces, users, settings, domains |
| MongoDB | Route data | Short URL mappings, routing policies, link metadata, TLS certificates |
| ClickHouse | Analytics | Click events, aggregated metrics, time-series data |
| Elasticsearch | Route search | Full-text index of route links, domains, destinations |
| Redis | Ephemeral state | Sessions, cache entries |
| RabbitMQ | Messaging | Cache invalidation, domain/route/certificate state changes, notification delivery events |
| MinIO | Object storage | ClickHouse data files, route icons, QR code assets, notification templates |
| gglsbl-rest | Safe Browsing | Local mirror of Google Safe Browsing threat lists |
| Vault | Secrets (dev mode) | Provisioned for future use; not yet wired into any service |
| Prometheus | Metrics | Time-series metrics from all services |
| Loki | Logs | Aggregated warning/error logs from all services |

## Event Streaming

Fluvio provides the event bus for the click pipeline:

- **`hit-stream-main`** — raw click events from Click Router to Click Tracker
- **`click-aggs-main`** — enriched events from Click Tracker to Click Aggregator

## RabbitMQ Topology

Two different patterns, by use case.

**Cache/state-change exchanges** are all fanout; each consuming instance binds its own exclusive, auto-delete queue, making them fire-and-forget — if RabbitMQ restarts, Moka cache TTLs provide eventual consistency, so nothing is lost that matters.

| Exchange | Published by | Consumed by |
|----------|--------------|-------------|
| `cache.invalidation.routes` | Click Router API (on route writes) | Click Router (Moka), Route Icon Worker |
| `cache.invalidation.user_settings` | Click Router API | Click Router (Moka) |
| `cache.invalidation.certificates` | Cert Bot (after issuing/renewing a cert) | Click Router (certificate cache) |
| `domain.state.changed` | Domain Verifier | Cert Bot, Management API |
| `route.status.changed` | Route Verifier | Management API |

**Notification delivery** is durable and at-least-once with explicit retry/dead-letter handling instead — a dropped notification isn't something a TTL can paper over.

```
Management API (Outbox) ──▶ "notifications" (durable topic exchange, routing key = notification_id)
                                    │
                                    ▼
                           notifications (main queue)
                                    │
                     ┌──────────────┴──────────────┐
                success (ack)                  failure (nack, no requeue)
                     │                              │
                     ▼                              ▼
             Notification Worker          notifications.retry (TTL-delayed)
            delivers in_app/email                   │
                                                      ▼
                                       back onto "notifications" exchange
                                      (after notifications_max_retries
                                       redeliveries → notifications.dead)
```

| Queue | Purpose |
|-------|---------|
| `notifications` | Main queue, consumed by Notification Worker |
| `notifications.retry` | Holds failed messages for `notifications_retry_delay_ms`, then dead-letters back onto the main exchange |
| `notifications.dead` | Terminal queue after `notifications_max_retries` redeliveries — needs manual inspection |

## Route Safety Verification

The Route Verifier service protects users from malicious destinations using Google Safe Browsing data:

```
Route Verifier ──▶ MongoDB (routes due for recheck)
       │
       ├──▶ gglsbl-rest (Safe Browsing lookup)
       │
       ▼
Click Router API (status update)
       │                      ╲
       ▼                       ▼
cache.invalidation.routes   route.status.changed
       │                              │
       ▼                              ▼
  Click Router                  Management API
 (cache update)                (PostgreSQL sync)
```

## Authentication

Keycloak handles user authentication locally (AWS Cognito in production). The Management API validates JWTs issued by Keycloak/Cognito, and the Dashboard integrates via `keycloak-js`.

## Monitoring & Observability

Shortas includes a comprehensive monitoring stack for metrics collection and log aggregation.

### Stack Components

| Component | Purpose | Port |
|-----------|---------|------|
| Prometheus | Metrics collection and storage | 9091 |
| Grafana | Visualization and dashboards | 3001 |
| Loki | Log aggregation | 3100 |

### Metrics Collection

All Rust services, including the Management API, expose Prometheus metrics endpoints. Prometheus scrapes these endpoints and stores time-series data for:

- Request counts and latencies
- Error rates
- Active connections
- Cache hit/miss ratios
- Queue depths

### Log Aggregation

Every service — including the Management API — is a Rust/Tokio process using `tracing` + `tracing-subscriber`, with `tracing-loki` shipping Warning/Error logs to Grafana Loki with service labels:

```
┌─────────────────┐
│  Rust Services   │
│ (incl. mgmt-api) │────▶ Loki (port 3100) ────▶ Grafana (port 3001)
│ tracing-loki     │
└─────────────────┘
```

### Log Levels

Services are configured to send only Warning and Error level logs to Loki to minimize noise, via `RUST_LOG=warn`.

### Querying Logs

In Grafana, use the Explore view with the Loki datasource:

```logql
# All service logs
{service=~".+"}

# Specific service
{service="click-router"}

# Filter by level
{service=~".+"} |= "ERROR"

# Search for patterns
{service=~".+"} |~ "(?i)timeout|connection"
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `LOKI_URL` | Loki push API endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |
