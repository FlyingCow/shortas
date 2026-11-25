# Design

## Context

See proposal.md for motivation. Relevant current behavior:

- Click Router holds routes and settings in Moka caches over MongoDB/DynamoDB, invalidated by RabbitMQ fanout.
- `click_stream` is `ORDER BY id`, so lookup by click ID is efficient.
- click-tracker turns conversions into clicks with an encoded `dest`; `ConversionProcessingModule` is not registered in either pipeline.
- click-aggregator-api does not declare its conversion modules. Migrations 003/004 create tables that nothing in the repo writes to.
- The redirect parses destinations as `http::Uri`, which drops fragments.
- `FluvioHitRegistrar` uses `.expect` on send, and the settings cache unwraps store errors.
- Domains belong to an owner and may be shared; they have no workspace column.

## Goals / Non-Goals

**Goals:** working tracking from redirect to ClickHouse; ingestion failures that cannot break redirects.

**Non-Goals:** charts (see `conversion-analytics`), multi-touch or IP attribution, goals/cohorts, currency conversion, AWS TLS.

## Assumptions (unconfirmed)

The user confirmed workspace scope, the flexible-entry ordered funnel, click ID on covered routes, ingestion in click-router, propagation like routes, and own-domain snippets. The user approved writing these artifacts with the following recorded as assumptions, not as confirmed decisions:

1. Revenue counts the final step only; refunds are server-only.
2. Last click by click ID, no IP fallback.
3. Optional route filter, domain eligibility rules, deduplication rules, limits and time windows.
4. AWS gets the DynamoDB store, while custom-domain TLS on AWS is out of scope.
5. Tables and views from migrations 003/004 may be dropped once each deployed database is checked for rows.

Tasks 1.1 to 1.3 settle these before dependent work.

## Decisions

### D1. Ownership of data
Postgres is the source of truth (`conversions`, `conversion_steps`, `conversion_routes`). The router gets a read-optimized document keyed by public key, in its own MongoDB collection/DynamoDB table. Reusing the routes collection was rejected: it needs dummy destinations and puts key hashes next to data that route APIs return.

### D2. Propagation
Saves write an outbox message in the same transaction. The outbox calls click-router-api, which stores and publishes to a new fanout exchange. Documents carry a version, and older versions are ignored, because outbox retries can arrive out of order. Router cache lifetime is 60 s and misses are cached briefly in a bounded cache, so revocation works even if a message is lost.

### D3. Route coverage flag
The Management API computes `track_conversions` on routes; `conversion_opt_out` is user-set. Redirects read the flag and never look up conversions. Activation queues a resumable batch over covered routes. Route create, update and move recompute the flag server-side and ignore any client value. Alternative: look up the workspace on every redirect, which adds work to the hot path.

### D4. Redirect
Use a URL type that keeps fragments. Append to the final destination after conditional routing, and keep the recorded `dest` unchanged. Any failure falls back to the unchanged URL. Click IDs are ULIDs. The hit carries `trackable`, with a serde default so services can be deployed independently.

### D5. Ingestion

    GET  /.well-known/shortas/c/{publicKey}.js
    POST /.well-known/shortas/v1/events        public key in body
    POST /.well-known/shortas/v1/conversions   bearer secret key
    POST /.well-known/shortas/v1/refunds       bearer secret key

Browser requests use `text/plain` JSON with `sendBeacon` or keepalive fetch, which avoids preflight. Secrets have high entropy and a non-secret lookup prefix, are stored as SHA-256 and compared in constant time. Forwarded addresses count only from configured proxies. Public keys cannot prove that an event is genuine: origin checks deter casual misuse only, so `source` is stored and values can be limited to servers.

### D6. Own stream
Publish to `conversion-stream-main` with step order, `is_final`, window and route filter captured at ingest. click-aggregator consumes it directly. click-tracker is skipped because geo and device come from the click, and server requests carry the customer's server address. Publishing awaits acknowledgement and returns errors as 503.

### D7. Storage and reconciliation
`conversion_events` is a `ReplacingMergeTree` ordered by `(workspace_id, conversion_id, dedupe_key)`, with **no time partition**, because replacement does not work across partitions. Reads use `FINAL`.

Dedupe key: step + click ID for once-per-click; otherwise step + external ID, or step + event ID. An event with no click ID always uses the external or event ID.

Rows are inserted as `pending` and a reconciler in the aggregator re-inserts them as attributed, unattributed or rejected. The version ranks status first (pending lowest), then earlier receipt, so repeated or concurrent reconciliation converges. This was chosen over an in-memory delay queue, which loses events on restart.

Values are `Decimal(18,4)`. Refunds are separate rows with their own dedupe key. Rejected events go to a table with a 30-day lifetime.

### D8. Trackable clicks
Add `is_trackable` to `click_stream`, and an aggregate view of trackable clicks by workspace, route, date, country, device, OS and browser. It ships here so the data exists from the first activation.

### D9. Removal
Delete the old router endpoints, conversion types, modules, SDK, uncompiled API code and test script. Drop migration 003/004 objects with a new migration, after the check in task 1.3.

## Risks / Trade-offs

- [Ingestion shares the redirect process] → Body, rate and concurrency limits, and separate metrics.
- [Browser events can be forged] → Labelled source, server-only values, origin allowlist.
- [Signed or strict destination URLs break] → Route opt-out and an activation warning.
- [Safari limits script-set cookies, to 7 days or less] → Documented; recommend passing the click ID to the server and using server events.
- [Ad blockers] → Own domain and server events.
- [`FINAL` cost grows] → Ordering by workspace and conversion; revisit with measurements.
- [Skip-tracking path: in the current code a `Break` in Register appears to end the flow without a result] → Regression test in 4.4 before changing this path.
- [Consent law] → Consent mode, plus documentation that the site owner is responsible.

## Migration Plan

1. ClickHouse additions, DynamoDB table, MongoDB index, topic and exchange.
2. Aggregator, aggregator API, tracker, click-router-api.
3. Router, with redirect tagging and ingestion behind separate settings.
4. Management API, then dashboard.
5. Remove old code; drop old tables last.

Rollback: turn off the settings. Stored events remain.
