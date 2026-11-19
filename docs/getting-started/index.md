---
title: Getting Started
permalink: /getting-started/
---

# Getting Started

This runs the full stack on one machine with Docker Compose: every service, database and the monitoring stack.

## Prerequisites

- Docker with Compose v2
- About 10 GB of free RAM. Mongo, ClickHouse, Elasticsearch and Keycloak are the heavy parts.
- Python 3, only for the optional seed script

## Start the stack

```bash
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas
cp redirect/route-verifier/.env.example redirect/route-verifier/.env
cp redirect/cert-bot/.env.example redirect/cert-bot/.env
cd infra/custom
docker compose up -d
```

Put your own [Google Safe Browsing API key](https://developers.google.com/safe-browsing/v4/get-started) in `GSB_API_KEY` in `redirect/route-verifier/.env`.

The first start builds every Rust service image, so expect several minutes. One-off setup containers (buckets, Fluvio topics, ClickHouse migrations, notification templates) run on their own and exit.

## Sign in

Open the dashboard at <http://localhost:3000> and sign in with the Keycloak dev user:

- user: `testuser`
- password: `testpassword`

## Add sample data

The seed script creates a workspace and a few routes through the real APIs, then generates click history so the charts have something to show:

```bash
python3 infra/custom/seed-dashboard-data.py            # last 21 days
python3 infra/custom/seed-dashboard-data.py --days 60
```

It is safe to re-run, because it replaces this user's previous sample clicks. Run it with `--help` for connection options.

## Local URLs

| What | URL |
|---|---|
| Dashboard | <http://localhost:3000> |
| Landing page | <http://localhost:3002> |
| Management API (Swagger UI) | <http://localhost:8090/swagger-ui> |
| Short links (Click Router) | <http://localhost:5800>, <https://localhost:4433> |
| Keycloak admin | <http://localhost:8080> |
| Grafana | <http://localhost:3001> |
| Prometheus | <http://localhost:9091> |
| RabbitMQ admin | <http://localhost:15672> |
| MailHog (outgoing email) | <http://localhost:8025> |
| MinIO console | <http://localhost:9001> |
| Mongo Express | <http://localhost:8081> |

Databases are also published on their default ports: Mongo 27017, ClickHouse 8123/9000, Redis 6379, Elasticsearch 9200, and Management API Postgres 5433.

## Stop

```bash
docker compose down        # keep data
docker compose down -v     # also delete volumes
```

Keycloak's database lives in `infra/custom/fs/` (a bind mount), so `-v` does not reset it.

To run services from source instead of containers, see [Development](/development/).
