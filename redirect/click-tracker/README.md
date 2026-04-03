# click-tracker

Stream consumer that reads raw click events from Fluvio and enriches them with contextual metadata.

## Enrichment Pipeline

Events pass through these modules in sequence:

1. **Init** — event initialization and validation
2. **Location** — geographic enrichment via MaxMind GeoIP database
3. **Session** — session identification using Redis
4. **User-Agent** — browser, OS, and device detection via UA Parser
5. **Aggregate** — batches events for downstream processing

Enriched events are published to the `click-aggs-main` Fluvio topic.

## Event Flow

```
Fluvio (hit-stream-main) → click-tracker → Fluvio (click-aggs-main)
```

## Configuration

- Configurable channel-based parallelism
- Supports both Fluvio (primary) and Kafka (fallback) as event sources

## Logging & Monitoring

Warning and error logs are sent to Grafana Loki for centralized log aggregation.

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `LOKI_URL` | Loki push endpoint | `http://shortas-loki:3100` |
| `RUST_LOG` | Log level filter | `warn` |

View logs in Grafana:
```logql
{service="click-tracker"}
```

## Dependencies

- Fluvio — event consumption and publishing
- Redis — session tracking
- MaxMind GeoIP database — geographic data
- UA Parser data — user-agent parsing

## Build

```bash
# From the repository root
make build-click-tracker

# Or directly
cargo build --manifest-path redirect/Cargo.toml -p click-tracker
```
