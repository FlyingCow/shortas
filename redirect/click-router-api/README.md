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

Runs on port 8080 (mapped to 8081 in Docker Compose).

## Dependencies

- MongoDB — route storage
- Redis — caching

## Build

```bash
# From the redirect/ directory
make build-click-router-api

# Or directly
cargo build -p click-router-api
```
