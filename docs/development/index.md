---
title: Development
permalink: /development/
---

# Development

## Repository layout

```
api/                    Rust: the dashboard's backend (Management API)
workers/                Rust: background workers
  notification-worker/    renders and delivers notifications
  notification-templates/ Handlebars templates, one directory per notification
  route-verifier/         Safe Browsing checks
  route-icon-worker/      favicon fetcher
  domain-verifier/        DNS ownership checks
  cert-bot/               Let's Encrypt automation
redirect/               Rust workspace: the click path
  click-router/           public redirect server
  click-router-api/       internal route/cert API (MongoDB)
  click-tracker/          click enrichment
  click-aggregator/       ClickHouse writer
  click-aggregator-api/   analytics query API
  shortas-common/         types shared with the API
  monitoring/             Prometheus, Grafana, Loki
ui/dashboard/           React admin app
ui/landing/             React marketing site
infra/                  local compose stack and deployment targets
docs/                   this site
```

There are two Cargo workspaces. `redirect/Cargo.toml` holds the click-path services. The root `Cargo.toml` holds `api`, everything under `workers/`, and `shortas-common`.

## Build, test, lint

Both workspaces, through the root `Makefile`:

```bash
make build                 # every Rust crate in both workspaces (debug)
make build-click-router    # one crate, from whichever workspace owns it;
make test-api              # same pattern for build-, test-, release-, bench-
make test
make clippy                # warnings are errors
make fmt
```

Or with cargo directly (root workspace):

```bash
cargo build -p api
cargo test  -p api
cargo run   -p notification-worker
```

Dashboard and landing page:

```bash
cd ui/dashboard
npm install
npm start          # http://localhost:3000; npm test, npm run build
```

## Running a service from source

Start the full stack (see [Getting Started](/getting-started/)), stop the container you want to work on, then run that service with cargo. Each service reads its configuration in this order, with later sources overriding earlier ones:

1. `config/default.toml`
2. `config/<APP_RUN_MODE>.toml`, where the run mode defaults to `development`
3. `config/local.toml` (optional, not committed)
4. Environment variables such as `APP__DATABASE__HOST=localhost`. Cert Bot uses the `CERTBOT__` prefix instead.

`make up` / `make down` / `make logs` start and stop only the redirect stack (`redirect/docker-compose.yml`).

## Notification templates

Each notification type is a directory under `workers/notification-templates/<id>/`:

| File | Purpose |
|---|---|
| `metadata.json` | `channels` (`email`, `in_app`) and the `required_context` keys |
| `email.subject.hbs`, `email.body.hbs` | Email subject and HTML body |
| `in_app.hbs` | Text shown in the dashboard's notification bell |

Templates are synced to object storage when the stack starts or deploys. Notification Worker loads them at runtime, so changing a template's wording needs no code change. A new notification type also needs the Management API to publish an event with that id.
