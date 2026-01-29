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

## Dependencies

- Fluvio — event consumption and publishing
- Redis — session tracking
- MaxMind GeoIP database — geographic data
- UA Parser data — user-agent parsing

## Build

```bash
# From the redirect/ directory
make build-click-tracker

# Or directly
cargo build -p click-tracker
```
