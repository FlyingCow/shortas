# click-router-api

REST API for managing short link routes. Built on Salvo with OpenAPI support.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/routes` | List routes |
| GET | `/routes/{id}` | Get route by ID |
| POST | `/routes` | Create a route |
| PUT | `/routes/{id}` | Update a route |
| DELETE | `/routes/{id}` | Delete a route |

## Features

- CRUD operations on routes
- Short URL generation
- Route configuration and routing policies
- OpenAPI/Swagger documentation
- JWT authentication

## Port

Runs on port 5810.

## Logging & Monitoring

Warning and error logs are sent to Grafana Loki for centralized log aggregation.

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `LOKI_URL` | Loki push endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |

View logs in Grafana:
```logql
{service="click-router-api"}
```

## Dependencies

- MongoDB — route storage
- Redis — caching

## Build

```bash
# From the repository root
make build-click-router-api

# Or directly
cargo build --manifest-path redirect/Cargo.toml -p click-router-api
```
