# Proposal

## Why

Users cannot tell which short links lead to signups or sales. The existing conversion code does not work end to end: events are accepted without authentication, their data is lost in the pipeline, the read API is not compiled, and nothing tells the destination site which click a visitor came from.

## What Changes

- Add workspace-level conversions with ordered steps, values, an attribution window and lifecycle status. Any route in the workspace can drive them.
- Propagate definitions to MongoDB/DynamoDB for Click Router to read.
- Add a click ID to destination URLs on routes covered by an active conversion, with a per-route opt-out.
- Accept events from a generated script per conversion and from servers with a secret key, under `/.well-known/shortas/`.
- Attribute events to clicks and store them in ClickHouse.
- Add management APIs and dashboard screens: list, editor, install snippets, event log.
- **BREAKING**: remove `/conversions/track`, `/conversions/funnel`, the unused SDK, and the unused conversion code, tables and views.
- **BREAKING**: click IDs change from `timestamp_counter` to ULIDs.

Reporting and charts are in `conversion-analytics`. Users should not activate conversions on custom domains until `custom-domain-tls` is released.

## Capabilities

### New Capabilities

- `conversion-definitions`: creating, configuring, activating and installing conversions.
- `conversion-click-identification`: passing a click ID to destinations.
- `conversion-ingestion`: receiving events from browsers and servers.
- `conversion-attribution`: matching events to clicks, deduplication, values and the event log.

### Modified Capabilities

_None._

## Impact

- `api`: Postgres tables, endpoints, outbox messages, route fields.
- `redirect/click-router`, `click-router-api`: new store, cache, exchange, endpoints and redirect behavior.
- `redirect/click-tracker`: passes the trackable flag; old conversion code removed.
- `redirect/click-aggregator`, `click-aggregator-api`: new stream consumer, reconciler, event log endpoint.
- ClickHouse, MongoDB, DynamoDB, a Fluvio topic and a RabbitMQ exchange.
- `ui/dashboard`: new Conversions pages and a route setting.
