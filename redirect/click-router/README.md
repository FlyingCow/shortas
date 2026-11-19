# click-router

The public redirect server. It resolves a short link, applies conditional rules, redirects the visitor, and publishes the click to Fluvio. It also serves QR codes, answers ACME HTTP-01 challenges, and terminates TLS for custom domains using certificates from MongoDB. See [Architecture → Click path](https://shortas.work/architecture/#click-path).

## Run

```bash
make build-click-router
make test-click-router
```

| Port | Purpose |
|---|---|
| 5800 | HTTP (`APP_LISTEN_ADDR`) |
| 4433 | HTTPS |
| 9090 | Prometheus metrics (`APP_METRICS_ADDR`; 9092 in compose) |

## Config

| Section | Purpose |
|---|---|
| `mongodb` / `aws.dynamo` | Route, settings and certificate store |
| `fluvio.hit_stream` | Topic for raw clicks (`kafka.*` is an alternative) |
| `moka.*` | In-memory caches for routes, settings, certificates and challenges |
| `geo_ip`, `uaparser` | Lookups used by conditional routing |
| `redirect` | URLs of the `pages` service and the QR image base URL |
| `rabbitmq` | Cache-invalidation exchanges |
