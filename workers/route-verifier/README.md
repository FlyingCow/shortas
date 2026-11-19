# route-verifier

Periodically checks route destinations against a local Google Safe Browsing mirror (`gglsbl-rest`) and blocks routes that point somewhere unsafe. See [Architecture → Background workers](https://shortas.work/architecture/#background-workers).

## How it works

Every `check_interval_seconds` the worker takes a batch of routes that are due for a check from MongoDB, checks their destinations, and updates their status. Active routes are re-checked every 24 hours and blocked routes every hour. A blocked route that later comes back clean stays blocked until someone unblocks it by hand.

Each status change is published to the `route.status.changed` fanout exchange:

```json
{
  "route_id": "uuid", "link": "example.com/abc",
  "owner_id": "uuid", "workspace_id": "uuid",
  "previous_status": "Active", "new_status": "Blocked",
  "blocked_reason": "Safe Browsing: MALWARE",
  "threat_type": "MALWARE", "threat_url": "https://…",
  "checked_at": 1709913600000, "next_check_at": 1709917200000
}
```

## Run

```bash
cp .env.example .env     # set GSB_API_KEY, also used by the safe-browsing container
make build-route-verifier
make test-route-verifier
```

Its health endpoint is on port 5831.

## Config

| Section | Purpose |
|---|---|
| `worker` | `check_interval_seconds`, `batch_size`, `recheck_interval_hours`, `blocked_recheck_interval_hours` |
| `safe_browsing` | `gglsbl-rest` URL and timeout |
| `mongodb` / `aws.dynamo` | Routes to verify |
| `rabbitmq` | Status-change exchange |
