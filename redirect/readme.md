# redirect

The Rust workspace for the click path and background workers. See [Architecture](https://shortas.work/architecture/) for how the services fit together.

| Crate | Role |
|---|---|
| `click-router` | Public redirect server; also serves QR codes and ACME challenges, and terminates TLS |
| `click-router-api` | Internal API for routes, certificates and user settings (MongoDB) |
| `click-tracker` | Enriches click events (geo, user agent, session) |
| `click-aggregator` | Writes enriched events to ClickHouse |
| `click-aggregator-api` | Internal analytics query API (ClickHouse) |
| `route-verifier` | Safe Browsing checks |
| `route-icon-worker` | Fetches destination favicons |
| `domain-verifier` | DNS ownership checks for custom domains |
| `cert-bot` | Let's Encrypt issuance and renewal |
| `shortas-common` | Types shared with `management-api` |

`infra/pages/` is the nginx container for branded not-found and blocked pages. `monitoring/` holds the Prometheus, Grafana and Loki stack.

## Run

From the repository root:

```bash
make build            # or make build-<crate>
make test             # or make test-<crate>
make clippy fmt
make up               # start this directory's docker-compose.yml
```

`make up` starts only this workspace's services and their databases. For the full stack, use `infra/custom` (see [Getting Started](https://shortas.work/getting-started/)).

## Config

Each crate reads `config/default.toml`, then `config/<APP_RUN_MODE>.toml`, then `APP__SECTION__KEY` environment variables. Every service also honours:

| Variable | Purpose |
|---|---|
| `APP_RUN_MODE` | Config profile (`development`, `production`, …) |
| `RUST_LOG` | Log filter, `warn` in containers |
| `LOKI_URL` | Loki push endpoint; warnings and errors are shipped there |
