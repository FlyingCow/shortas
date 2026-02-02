# click-router

High-performance URL redirect service. Handles incoming HTTP requests for short URLs, resolves them against MongoDB, performs redirects, and publishes click events to Fluvio.

## How It Works

Each request passes through a processing pipeline:

1. **URL extraction** — parses the short code from the request path
2. **Route lookup** — queries MongoDB with an in-memory Moka cache layer
3. **Event registration** — publishes a raw click event to the `hit-stream-main` Fluvio topic
4. **Result building** — returns the appropriate response

## Response Types

- HTTP redirect (301/302)
- JSON response
- QR code image
- Proxied content
- Retargeting HTML

## TLS

Supports dynamic TLS certificate resolution through the Domains service (port 5801) for custom domain support.

## Ports

| Port | Purpose |
|------|---------|
| 5800 | HTTP server |
| 9090 | Prometheus metrics |

## Dependencies

- MongoDB — route storage
- Redis — caching
- ClickHouse — analytics queries
- Fluvio — event publishing
- Domains service — certificate resolution

## Build

```bash
# From the repository root
make build-click-router

# Or directly
cargo build --manifest-path redirect/Cargo.toml -p click-router
```
