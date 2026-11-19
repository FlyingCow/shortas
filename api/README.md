# api

The dashboard's backend. It owns workspaces, members, invitations, domains, notifications and the user's view of their routes (PostgreSQL), plus route search (Elasticsearch). Route writes go to `click-router-api` and analytics reads go to `click-aggregator-api`. An outbox publishes events (search indexing, notifications) to RabbitMQ in the same transaction as the change that caused them.

See [Architecture → Management path](https://shortas.work/architecture/#management-path) and the [API overview](https://shortas.work/api/).

## Run

From the repository root, with the stack's databases running:

```bash
cargo run  -p api
cargo test -p api
```

It listens on 8080 (published as 8090 in compose). Swagger UI is at `/swagger-ui`. Migrations in `migrations/` run on startup.

## Config

`config/default.toml` → `config/<APP_RUN_MODE>.toml` → `config/local.toml` → `APP__SECTION__KEY` environment variables.

| Section | Purpose |
|---|---|
| `database` | PostgreSQL |
| `jwt` | Issuer, audience and JWKS URL of the OIDC provider |
| `click_router`, `click_aggregator`, `route_verifier`, `domain_verifier` | Internal service URLs |
| `elasticsearch` | Route search index |
| `minio` | Object storage for QR logos, favicons and templates |
| `rabbitmq` | Outbox publishing, status consumers, notification queues |
| `shared_domains` | Domains every workspace can shorten under |
| `email` | SES or SMTP sender |
