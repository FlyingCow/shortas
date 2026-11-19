---
title: API
permalink: /api/
---

# API

The **Management API** is the public backend: the dashboard uses it, and so can your own tools. Locally it runs at `http://localhost:8090`.

Endpoints are generated from the code and always current:

- Swagger UI: `/swagger-ui`
- OpenAPI spec: `/api-doc/openapi.json`

## Authentication

Send an OIDC access token from Keycloak (local) or Cognito (AWS) as `Authorization: Bearer <token>`. Everything under `/api/v1` requires a token and is rate-limited. The exceptions are the health checks, the list of shared domains, and the invitation preview linked from invite emails.

## Endpoint groups

All paths are under `/api/v1`.

| Group | Covers |
|---|---|
| `routes` | Create, edit, bulk-edit and search short links; QR settings; unblock |
| `domains` | Custom domains, DNS instructions, verification, shared domains |
| `workspaces` | Workspaces, members and roles, invitations |
| `invitations` | Accept or decline an invitation |
| `notifications` | In-app notification list and read state |
| `certificates` | TLS certificates issued for your domains |
| `clickstream` | Analytics: time series, geography, devices, top routes and destinations |
| `user`, `users` | Your profile and onboarding; user search for invites |

Health endpoints live outside `/api/v1`: `/health`, `/health/live`, `/health/ready`.

## Internal APIs

Click Router API (routes and certificates in MongoDB) and Click Aggregator API (ClickHouse queries) are internal. They have no authentication and are reached only through the Management API. Each serves its own Swagger UI at `/swagger-ui` on the private network.
