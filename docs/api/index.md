---
layout: professional-theme
title: API Reference
permalink: /api/
---

# API Reference

Shortas exposes three REST APIs. All three are built with Salvo; the two internal APIs document themselves with OpenAPI.

## Management API (Rust / Salvo)

Base URL: `http://localhost:8090` (Docker) or `http://localhost:8080` (local `cargo run`)

Authentication: JWT bearer tokens issued by Keycloak (local) / AWS Cognito (production). All `/api/v1/*` routes require a valid JWT and are rate-limited, except where noted.

### Health

| Method | Path | Description |
|--------|------|--------------|
| GET | `/health` | Health check (public) |
| GET | `/health/ready` | Readiness check (public) |
| GET | `/health/live` | Liveness check (public) |

### Routes

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/routes` | List routes (paginated, filterable) |
| POST | `/api/v1/routes` | Create a route (supports conditional routing) |
| GET | `/api/v1/routes/{id}` | Get a route by ID |
| PUT | `/api/v1/routes/{id}` | Update a route |
| DELETE | `/api/v1/routes/{id}` | Delete a route |
| POST | `/api/v1/routes/{id}/unblock` | Unblock a route blocked by Safe Browsing |
| GET | `/api/v1/routes/suggest-link` | Generate a unique short link path |
| POST | `/api/v1/routes/bulk` | Bulk create routes |
| PUT | `/api/v1/routes/bulk` | Bulk update routes |
| DELETE | `/api/v1/routes/bulk` | Bulk delete routes |

### QR Codes

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/routes/{id}/qr/settings` | Get QR code design settings |
| PUT | `/api/v1/routes/{id}/qr/settings` | Update QR code design settings |
| POST | `/api/v1/routes/{id}/qr/upload-url` | Get presigned URL for QR code SVG upload |
| POST | `/api/v1/routes/{id}/qr/logo-upload-url` | Get presigned URL for QR code logo upload |

### Route Search (Elasticsearch)

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/routes/search?q={query}` | Full-text search by link, domain, or destination |
| POST | `/api/v1/routes/search/reindex` | Rebuild the Elasticsearch index |

### Domains

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/domains` | List configured domains |
| POST | `/api/v1/domains` | Add a domain |
| GET | `/api/v1/domains/shared` | List shared domains (public, no auth) |
| GET | `/api/v1/domains/dns-config` | DNS configuration needed for a new domain |
| GET | `/api/v1/domains/{id}` | Get a domain |
| PUT | `/api/v1/domains/{id}` | Update a domain |
| DELETE | `/api/v1/domains/{id}` | Remove a domain |
| GET | `/api/v1/domains/{id}/dns-config` | DNS config for an existing domain |
| POST | `/api/v1/domains/{id}/verify` | Trigger domain ownership verification |

### Workspaces

Every member has a role: **Owner** (the creator, exactly one per workspace, never reassigned or removed), **Admin**, or **Member**. Actions below are self- or Owner-only except where noted.

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/workspaces` | List user workspaces |
| POST | `/api/v1/workspaces` | Create a workspace (creator becomes Owner) |
| GET | `/api/v1/workspaces/{id}` | Get a workspace |
| PUT | `/api/v1/workspaces/{id}` | Update a workspace (Admin+) |
| DELETE | `/api/v1/workspaces/{id}` | Delete a workspace (Owner only) |
| GET | `/api/v1/workspaces/{id}/members` | List workspace members (non-admins only see the owner and themselves) |
| POST | `/api/v1/workspaces/{id}/members` | Add a member directly by user ID (Admin+) |
| PUT | `/api/v1/workspaces/{id}/members/{user_id}` | Update a member's role (Owner only) |
| DELETE | `/api/v1/workspaces/{id}/members/{user_id}` | Remove a member (Admin+) |

#### Invitations

Invite-by-email flow for adding members without knowing their user ID up front. An Admin/Owner invites by email; a single-use token (only its hash is persisted, 7-day TTL) is emailed via the transactional outbox, and if the invitee already has an account, an in-app notification is created too. The invitee accepts either from the emailed link (`by-token`, works pre-login) or from the in-app notification center.

| Method | Path | Description | Auth |
|--------|------|--------------|------|
| GET | `/api/v1/workspaces/{id}/invitations` | List pending invitations for a workspace | Admin+ |
| POST | `/api/v1/workspaces/{id}/invitations` | Invite a user by email | Admin+ |
| DELETE | `/api/v1/workspaces/{id}/invitations/{invitation_id}` | Revoke a pending invitation | Admin+ |
| GET | `/api/v1/invitations/by-token/{token}` | Preview an invitation (workspace, inviter, role, status) by its raw token | Public |
| POST | `/api/v1/invitations/by-token/{token}/accept` | Accept an invitation via the emailed link | Authenticated |
| POST | `/api/v1/invitations/{id}/accept` | Accept an invitation (e.g. from the notification center) | Authenticated |
| POST | `/api/v1/invitations/{id}/decline` | Decline an invitation | Authenticated |

### Notifications

In-app notification center; currently used for workspace invite alerts.

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/notifications` | List the current user's notifications, newest first (up to 50) |
| POST | `/api/v1/notifications/{id}/read` | Mark a notification as read |

### Users

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/users/search?q={query}` | Search known users by email prefix/substring (workspace invite typeahead); requires 2+ characters |

### Certificates

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/certificates` | List certificates |
| POST | `/api/v1/certificates` | Register a certificate |
| GET | `/api/v1/certificates/{id}` | Get a certificate |
| PUT | `/api/v1/certificates/{id}` | Update a certificate |
| DELETE | `/api/v1/certificates/{id}` | Remove a certificate |

### Clickstream (Analytics Proxy)

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/clickstream` | Query click events |
| GET | `/api/v1/clickstream/routes/{route_id}` | Clicks for a specific route (+ `/timeseries`, `/geo`, `/devices`, `/browsers`) |
| GET | `/api/v1/clickstream/workspaces/{workspace_id}` | Clicks for a workspace (+ `/top-routes`) |
| GET | `/api/v1/clickstream/stats` | Aggregated stats (+ `/daily`, `/hourly`, `/geographic`, `/devices`, `/browsers`, `/route-performance`, `/top-destinations`, `/traffic-types`) |

### User

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/v1/user/me` | Get current user profile |
| POST | `/api/v1/user/initialize` | Initialize a new user (workspace bootstrap) |
| GET | `/api/v1/user/initialization-status` | Check onboarding/initialization status |
| POST | `/api/v1/user/onboarding/complete` | Mark onboarding as complete |

## Click Router API (Rust / Salvo)

Base URL: `http://localhost:5810`
OpenAPI: `/api-doc/openapi.json` · Swagger UI: `/swagger-ui`

Internal API backing route storage in MongoDB, certificate storage, user settings, and ACME challenges. **Not intended for public exposure** — it currently has no JWT/validation middleware enabled, since it's only meant to be called by Management API, Route Verifier, and Cert Bot within the private network.

| Method | Path | Description |
|--------|------|--------------|
| GET | `/public/health` | Health check |
| GET | `/public/metrics` | Prometheus metrics |
| GET | `/v1/routes` | List routes |
| GET | `/v1/routes/{route_id}` | Get route by ID |
| GET | `/v1/routes/{switch}/{domain}/{path}` | Get route by composite key |
| POST | `/v1/routes` | Create a route |
| PUT | `/v1/routes/{route_id}` | Update a route |
| DELETE | `/v1/routes/{route_id}` | Delete a route |
| POST/PUT/DELETE | `/v1/routes/bulk` | Bulk operations |
| GET | `/v1/certificates?expires_before=` | List certificates expiring before a date |
| GET/POST/PUT/DELETE | `/v1/certificates/{domain}` | Certificate CRUD (used by Cert Bot) |
| GET/PUT | `/v1/user-settings/{user_id}` | User settings |
| GET/POST | `/v1/challenges/{domain}/{token}` | ACME HTTP-01 challenge storage (used by Cert Bot, served by Click Router) |

## Click Aggregator API (Rust / Salvo)

Base URL: `http://localhost:5820`
OpenAPI: `/api-doc/openapi.json` · Swagger UI: `/swagger-ui`

Queries analytics data from ClickHouse.

| Method | Path | Description |
|--------|------|--------------|
| GET | `/public/health` | Health check |
| GET | `/public/metrics` | Prometheus metrics |
| GET | `/v1/clickstream` | Raw click event queries |
| GET | `/v1/stats` | Aggregated stats (+ `/daily`, `/hourly`, `/geographic`, `/devices`, `/browsers`, `/route-performance`, `/top-destinations`, `/traffic-types`) |
| GET | `/v1/conversions` | Conversion queries (+ `/rates`, `/revenue`, `/attribution`, `/funnels`, `/summary`) |
