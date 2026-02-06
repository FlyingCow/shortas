---
layout: vector-theme
title: API Reference
permalink: /api/
---

# API Reference

Shortas exposes three REST APIs.

## Management API (ASP.NET Core)

Base URL: `http://localhost:5050`
Swagger UI: `http://localhost:5050/swagger`

Authentication: JWT bearer tokens issued by Keycloak.

### Endpoints

#### Routes

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/routes` | List routes (paginated, filterable by status/workspace) |
| GET | `/api/v1/routes/{id}` | Get a route by ID |
| POST | `/api/v1/routes` | Create a new route |
| PUT | `/api/v1/routes/{id}` | Update a route |
| DELETE | `/api/v1/routes/{id}` | Delete a route |
| POST | `/api/v1/routes/bulk` | Bulk create routes |
| PUT | `/api/v1/routes/bulk` | Bulk update routes |
| DELETE | `/api/v1/routes/bulk` | Bulk delete routes |

#### Route Search (Elasticsearch)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/routes/search?q={query}` | Full-text search by link, domain, or destination |
| POST | `/api/v1/routes/search/reindex` | Rebuild the Elasticsearch index from PostgreSQL |

The search endpoint supports `page`, `pageSize`, and `workspaceId` query parameters. The index is kept in sync automatically via the outbox pattern — see the [Architecture](/architecture/#search-index-sync) page for details.

#### Workspaces

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/workspaces` | List user workspaces |
| POST | `/api/workspaces` | Create a workspace |
| PUT | `/api/workspaces/{id}` | Update a workspace |
| DELETE | `/api/workspaces/{id}` | Delete a workspace |

#### Domains

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/domains` | List configured domains |
| POST | `/api/domains` | Add a domain |
| DELETE | `/api/domains/{id}` | Remove a domain |

#### Certificates

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/certificates` | List certificates |
| POST | `/api/certificates` | Upload a certificate |
| DELETE | `/api/certificates/{id}` | Remove a certificate |

#### Click Stream

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/clickstream` | Query click analytics |

#### User

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/user` | Get current user profile |
| PUT | `/api/user/settings` | Update user settings |

#### Health

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/health` | Health check |

## Click Router API (Rust)

Base URL: `http://localhost:8081`

Manages short link routes stored in MongoDB. Includes OpenAPI documentation.

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/routes` | List routes |
| GET | `/routes/{id}` | Get route by ID |
| POST | `/routes` | Create a route |
| PUT | `/routes/{id}` | Update a route |
| DELETE | `/routes/{id}` | Delete a route |

## Click Aggregator API (Rust)

Base URL: `http://localhost:8082`

Queries analytics data from ClickHouse.

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/analytics` | Query click analytics with filters |
| GET | `/analytics/summary` | Aggregated summary statistics |
| GET | `/health` | Health check |
