---
title: Architecture
permalink: /architecture/
---

# Architecture

Shortas splits into two halves that share little at runtime:

- The **click path** answers short-link requests as fast as possible and records each click.
- The **management path** is what users interact with through the dashboard: creating links, adding domains, reading analytics.

Every backend service is Rust (Salvo / Tokio). The two frontends are React.

```
                 visitor                                   user
                    |                                        |
                    v                                        v
  +-----------------------------+              +-------------------------+
  |        Click Router         |              |     Dashboard (React)   |
  |  redirect / QR / TLS / ACME |              +------------+------------+
  +--------------+--------------+                           |
                 | click event                              v
                 v  (Fluvio)                   +-------------------------+
  +-----------------------------+              |      Management API     |
  |        Click Tracker        |              |  Postgres, Elasticsearch|
  |   geo, user agent, session  |              +-----+-------------+-----+
  +--------------+--------------+                    |             |
                 v  (Fluvio)                         v             v
  +-----------------------------+          +----------------+ +---------------------+
  |      Click Aggregator       |--------->|  ClickHouse    | | Click Router API    |
  +-----------------------------+          |  (analytics)   | | (routes in MongoDB) |
                                           +--------^-------+ +---------------------+
                                                    |
                                           Click Aggregator API
```

## Click path

**Click Router** is the public entry point. For each request it:

1. Looks up the route by domain and path. Routes live in MongoDB, with an in-memory cache in front.
2. Evaluates any conditional rules (country, device, browser, OS, language) to pick a destination.
3. Responds with a redirect or a QR code. If the link is paused, blocked or missing, it shows a branded page from the small `pages` nginx service.
4. Publishes a raw click event to the Fluvio topic `hit-stream-main`.

**Click Tracker** consumes those events and enriches them with geolocation (MaxMind), parsed user agent, and a Redis-backed session. It publishes the results to `click-aggs-main`.

**Click Aggregator** batch-inserts the enriched events into ClickHouse.

When a route changes, Click Router API publishes a cache-invalidation message on RabbitMQ, and every Click Router instance drops its cached copy.

## Management path

**Management API** is the only backend the dashboard talks to. It owns workspaces, members and invitations, domains, and the user's view of their routes (PostgreSQL, with Elasticsearch for route search). It forwards route writes to **Click Router API**, which stores them in MongoDB, and analytics queries to **Click Aggregator API**, which reads ClickHouse. Authentication is via OIDC JWTs from Keycloak locally, or Cognito on AWS.

The two internal APIs aren't authenticated and must stay on the private network.

## Background workers

| Worker | Does |
|---|---|
| Route Verifier | Re-checks destinations against a local Google Safe Browsing mirror and blocks unsafe routes. |
| Route Icon Worker | Fetches destination favicons into object storage for the dashboard. |
| Domain Verifier | Confirms ownership of custom domains through a DNS TXT record. |
| Cert Bot | Issues and renews Let's Encrypt certificates for verified domains. |
| Notification Worker | Renders and delivers in-app and email notifications. |

Workers talk to each other through RabbitMQ events such as `domain.state.changed` and `route.status.changed`, never by direct calls.

## Custom domains and TLS

```
 add domain --> Domain Verifier --(TXT ok)--> domain.state.changed
                                                      |
                                                      v
                                    Cert Bot --ACME HTTP-01--> Let's Encrypt
                                       |   (challenge answered by Click Router)
                                       v
                          certificate stored in MongoDB
                                       |
                                       v
                Click Router loads it on the TLS handshake (cached)
```

Click Router terminates TLS itself. It reads certificates from MongoDB by server name and keeps them in a cache, so a handshake never needs a network call.

## Notifications

When an action needs to notify someone (currently a workspace invitation), the Management API writes an event to its outbox in the same transaction. The outbox publishes it to RabbitMQ. Notification Worker then renders Handlebars templates from object storage and delivers the result in-app (PostgreSQL), by email, or both. Failed deliveries are retried with a delay and then dead-lettered.

Templates live in the repo under `workers/notification-templates/` and are synced to object storage on deploy. See [Development](/development/#notification-templates).

## Data stores

| Store | Holds |
|---|---|
| MongoDB | Routes, user settings, TLS certificates |
| PostgreSQL | Workspaces, users, domains, notifications |
| ClickHouse | Click events and analytics |
| Elasticsearch | Route search index |
| Redis | Visitor sessions |
| S3 / MinIO | Favicons, QR assets, notification templates, ClickHouse cold data |
| Fluvio | Click event stream |
| RabbitMQ | Cache invalidation, state-change events, notifications |

## Observability

The `redirect/` services ship warning and error logs to Loki. Click Router and Click Aggregator also expose Prometheus metrics. Grafana sits on top with provisioned dashboards. See `redirect/monitoring/` for the local setup.

## Tech stack

| Area | Technology |
|---|---|
| Services | Rust, Salvo, Tokio, SQLx |
| Frontends | React, TypeScript, Bootstrap |
| Streaming / messaging | Fluvio, RabbitMQ |
| Auth | Keycloak (local), Cognito (AWS) |
| Certificates | Let's Encrypt via `instant-acme` |
| Monitoring | Prometheus, Grafana, Loki |
