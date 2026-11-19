# click-aggregator-api

An internal analytics API over ClickHouse. It serves clickstream statistics (daily and hourly, geography, devices, browsers, traffic types, top routes and destinations) and conversions (rates, revenue, attribution, funnels). The Management API proxies it for the dashboard, and it has no authentication of its own.

## Run

```bash
make build-click-aggregator-api
make test-click-aggregator-api
```

It listens on 5820 (the `production` profile; `server.port`). Endpoints are under `/v1`, documented at `/swagger-ui`.

## Migrations

The ClickHouse schema lives in `migrations/`. Compose applies it automatically. To apply it by hand:

```bash
bash scripts/apply_migrations.sh      # CLICKHOUSE_HOST etc. override the defaults
```
