# click-router-api

An internal API over the route store (MongoDB or DynamoDB). It manages routes, TLS certificates, ACME challenges, user settings and custom pages. After every write it publishes a cache-invalidation message so all Click Router instances drop stale entries.

It has **no authentication**. Only the Management API, Cert Bot and Route Verifier call it, and it must not be exposed publicly.

## Run

```bash
make build-click-router-api
make test-click-router-api
```

It listens on 5810 (the `production` profile; `server.port`). Endpoints are under `/v1`, documented at `/swagger-ui`.

## Config

| Section | Purpose |
|---|---|
| `mongodb` / `aws.dynamo` | Route store |
| `rabbitmq` | Cache-invalidation exchanges |
| `server` | Port |
