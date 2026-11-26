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
| `workspaces/{id}/conversions` | Conversions: definition, lifecycle and keys |
| `invitations` | Accept or decline an invitation |
| `notifications` | In-app notification list and read state |
| `certificates` | TLS certificates issued for your domains |
| `clickstream` | Analytics: time series, geography, devices, top routes and destinations |
| `user`, `users` | Your profile and onboarding; user search for invites |

Health endpoints live outside `/api/v1`: `/health`, `/health/live`, `/health/ready`.

## Conversions

Members can read a workspace's conversions. Owners and Admins can change them. People outside the workspace get 404.

Create a draft. The response is the only time `secretKey` is shown:

```bash
curl -X POST "$API/api/v1/workspaces/$WORKSPACE/conversions" \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' -d '{
    "name": "Purchase",
    "currency": "EUR",
    "windowDays": 30,
    "counting": "OncePerClick",
    "allowedOrigins": ["https://shop.example"],
    "steps": [
      { "key": "cart", "name": "Added to cart" },
      { "key": "purchase", "name": "Purchase", "defaultValue": "5.00" }
    ]
  }'
```

- The last step is the conversion. A step's `key` cannot be renamed. Removing a step after the first activation archives it.
- `counting` is `OncePerClick` or `Every`.
- `routeIds` limits the conversion to those routes. Leave it out to cover the whole workspace.
- Values are decimal strings with up to 4 decimals.

Replace the definition. Send the `version` you read; if someone else saved first, the response is 409:

```bash
curl -X PUT "$API/api/v1/workspaces/$WORKSPACE/conversions/$CONVERSION" \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' -d '{
    "version": 1,
    "name": "Purchase",
    "currency": "EUR",
    "windowDays": 30,
    "counting": "Every",
    "steps": [{ "key": "purchase", "name": "Purchase" }]
  }'
```

Rotate the secret key:

```bash
curl -X POST "$API/api/v1/workspaces/$WORKSPACE/conversions/$CONVERSION/rotate-secret" \
  -H "Authorization: Bearer $TOKEN"
```

`POST .../activate`, `.../pause` and `.../archive` change the status. Archiving is final. Activating needs a verified short domain with a valid certificate that is shared or belongs to the workspace owner. Otherwise the response is 422 and `details` is `domain_missing`, `domain_not_owned`, `domain_unverified` or `certificate_missing`.

Invalid definitions return 400, and `details` lists the fields.

Members can read a conversion's recent events, newest first:

```bash
curl "$API/api/v1/workspaces/$WORKSPACE/conversions/$CONVERSION/events?limit=50&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

- Events are `pending` while their click is awaited (up to 15 minutes), then `attributed`, `unattributed` or `rejected`. Refunds are `refunded`, `invalid` or `rejected`. `reason` says why.
- `applied` is the part of a refund that reduces revenue.
- Repeated events are shown once. Rejected events are kept for 30 days.

`GET workspaces/{id}/conversions/domains` lists the domains Owners and Admins can choose, and for Members those in use. `GET domains/{id}/conversions` lists the running conversions that removing a domain of yours would stop.

Receiving events from sites and servers is described in [Conversion tracking](conversion-tracking).

## Internal APIs

Click Router API (routes and certificates in MongoDB) and Click Aggregator API (ClickHouse queries) are internal. They have no authentication and are reached only through the Management API. Each serves its own Swagger UI at `/swagger-ui` on the private network.
