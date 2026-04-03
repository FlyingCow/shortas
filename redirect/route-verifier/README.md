# route-verifier

Background worker service that periodically checks route destinations against Google Safe Browsing data. When unsafe URLs are detected, routes are automatically blocked to protect users from malware and phishing.

## How It Works

The service runs on a configurable interval (default: 5 minutes) and processes routes in batches:

1. **Route selection** — queries MongoDB for routes due for verification (`next_safety_check <= now`)
2. **Safe Browsing check** — validates each route's destinations against the local gglsbl-rest API
3. **Status update** — blocks routes with unsafe destinations, updates local timestamps
4. **Event publishing** — publishes status changes to RabbitMQ for downstream consumers

## Architecture

```
MongoDB (routes_to_verify)
        │
        ▼
  route-verifier ────▶ gglsbl-rest (Safe Browsing)
        │
        ▼
  RabbitMQ (route.status.changed)
   ╱           ╲
  ▼             ▼
Management    click-router
API           (cache update)
```

## Route Status

Routes have two possible statuses:

- `Active` — safe, redirects work normally
- `Blocked` — unsafe URL detected, redirects are disabled

When a route is blocked, the reason is stored (e.g., "Safe Browsing: MALWARE").

## Verification Schedule

| Route Status | Recheck Interval |
|--------------|------------------|
| Active       | 24 hours         |
| Blocked      | 1 hour           |

Blocked routes are rechecked more frequently to allow automatic unblocking if the threat is removed from Safe Browsing lists (manual review currently required).

## Ports

| Port | Purpose |
|------|---------|
| 5831 | HTTP health endpoint |

## Logging & Monitoring

Warning and error logs are sent to Grafana Loki for centralized log aggregation.

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `LOKI_URL` | Loki push endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |

View logs in Grafana:
```logql
{service="route-verifier"}
```

## Configuration

Key settings in `config/default.toml`:

```toml
[worker]
check_interval_seconds = 300    # Batch processing interval
batch_size = 100                # Routes per batch
recheck_interval_hours = 24     # Safe route recheck
blocked_recheck_interval_hours = 1  # Blocked route recheck

[safe_browsing]
base_url = "http://safe-browsing:5000"
timeout_seconds = 10

[mongodb]
connection_string = "mongodb://root:example@mongo:27017/"
database_name = "shortas"
collection = "routes_to_verify"
```

## Dependencies

- MongoDB — routes to verify (synced from Management API)
- gglsbl-rest — local Google Safe Browsing API mirror
- RabbitMQ — status change event publishing

## RabbitMQ Events

Publishes to fanout exchange `route.status.changed`:

```json
{
  "route_id": "uuid",
  "link": "example.com/abc",
  "owner_id": "user-uuid",
  "workspace_id": "workspace-uuid",
  "previous_status": "Active",
  "new_status": "Blocked",
  "blocked_reason": "Safe Browsing: MALWARE",
  "threat_type": "MALWARE",
  "threat_url": "https://malicious-site.com",
  "checked_at": 1709913600000,
  "next_check_at": 1709917200000
}
```

## Build

```bash
# From the repository root
cargo build --manifest-path redirect/Cargo.toml -p route-verifier

# Release build
cargo build --manifest-path redirect/Cargo.toml -p route-verifier --release
```
