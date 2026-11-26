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

## Confirmed with the user

Workspace scope, flexible-entry ordered funnel, click ID on covered routes, ingestion in click-router, propagation to MongoDB and DynamoDB like routes, and own-domain snippets. In addition:

1. Revenue counts the final step only; refunds are server-only.
2. Last click by click ID with no IP fallback, the optional route filter, domain eligibility, deduplication, limits and time windows are accepted **for now** and may be revisited.
3. Nothing is deployed on any platform. Existing conversion tables may be changed or deleted.

4. The DynamoDB store stays. Custom-domain TLS on AWS is out of scope.
5. Moving a route into a workspace requires Owner or Admin there, because its clicks would count in that workspace.
6. Shared domains are registered with domain-verifier at startup, so Cert Bot issues and renews their certificates. A failed check does not remove a shared domain's verified status. `shortas.space` is verified unconditionally; other shared domains need the TXT record.
7. When an owner's settings cannot be read, visitors are redirected and the click is not recorded.
8. Servers with a valid secret key are exempt from the address limit. Failed authentications are limited by address.
9. Bulk route update is left alone and fixed separately.

## Decisions

### D1. Ownership of data
Postgres is the source of truth (`conversions`, `conversion_steps`, `conversion_routes`). The router gets a read-optimized document keyed by public key, in its own MongoDB collection/DynamoDB table. Reusing the routes collection was rejected: it needs dummy destinations and puts key hashes next to data that route APIs return.

### D2. Propagation
Saves write an outbox message in the same transaction. The outbox calls click-router-api, which stores and publishes to a new fanout exchange. Documents carry a version, and older versions are ignored, because outbox retries can arrive out of order. Router cache lifetime is 60 s and misses are cached briefly in a bounded cache, so revocation works even if a message is lost. Conversion messages retry until delivered, and deleting a workspace stores an inactive definition that later messages cannot replace.

### D3. Route coverage flag
The Management API computes `track_conversions` on routes; `conversion_opt_out` is user-set. Redirects read the flag and never look up conversions. Activation queues a resumable batch over covered routes. The queue is the routes whose wanted flag differs from the flag last sent, so an interrupted run continues where it stopped; a conversion becomes active when its workspace has none left. Writes that change coverage wake the sender, which otherwise checks once a minute for retries and other instances' changes; a partial index keeps that check to the pending routes. Route create, update and move recompute the flag server-side and ignore any client value. Alternative: look up the workspace on every redirect, which adds work to the hot path.

### D4. Redirect
Use a URL type that keeps fragments. Append to the final destination after conditional routing, and keep the recorded `dest` unchanged. Any failure falls back to the unchanged URL. Click IDs are ULIDs. The hit carries `trackable`, with a serde default so services can be deployed independently.

### D5. Ingestion

    GET  /.well-known/shortas/c/{publicKey}.js
    POST /.well-known/shortas/v1/events        public key in body
    POST /.well-known/shortas/v1/conversions   bearer secret key
    POST /.well-known/shortas/v1/refunds       bearer secret key

Browser requests use `text/plain` JSON with `sendBeacon` or keepalive fetch, which avoids preflight. Secrets have high entropy and contain the public key's ID as a non-secret lookup prefix (no second index), are stored as SHA-256 and compared in constant time. Forwarded addresses count only from configured proxies. Public keys cannot prove that an event is genuine: origin checks deter casual misuse only and do not apply to test events, which are never counted, so the dashboard can send them, so `source` is stored and values can be limited to servers.

### D6. Own stream
Publish to `conversion-stream-main` with step order, `is_final`, window and route filter captured at ingest. click-aggregator consumes it directly. click-tracker is skipped because geo and device come from the click, and server requests carry the customer's server address. Publishing awaits acknowledgement and returns errors as 503.

### D7. Storage and reconciliation
`conversion_events` is a `ReplacingMergeTree` ordered by `(workspace_id, conversion_id, dedupe_key)`, with **no time partition**, because replacement does not work across partitions. Reads use `FINAL`.

Dedupe key: step + click ID for once-per-click; otherwise step + external ID, or step + event ID. An event with no click ID always uses the external or event ID.

Events with a click ID are first stored in `conversion_pending`, and a reconciler in the aggregator moves them to `conversion_events` as attributed or unattributed, or to the rejected table. Everything else is stored directly. The version ranks earlier receipt first, then attributed over unattributed for the same event, so repeated or concurrent reconciliation converges. This was chosen over an in-memory delay queue, which loses events on restart, and over pending rows in `conversion_events`, where an earlier pending row would replace a later one with the same key and, if then rejected, lose a valid event.

Offsets are committed only after the records are stored, and directly after a record is received: cancelling a pending read in the Fluvio client keeps its lock, and a commit would wait forever. A restart may re-read about the last second of traffic, which deduplicates.

Values are `Decimal(18,4)`. Refunds are separate rows with their own dedupe key; a view, `conversion_refunds`, works out on reading how much of each reduces revenue, so arrival order does not matter. Rejected events go to a table with a 30-day lifetime.

### D8. Trackable clicks
Add `is_trackable` to `click_stream`, and an aggregate view of trackable clicks by workspace, route, date, country, device, OS and browser. It ships here so the data exists from the first activation.

### D9. Removal
Delete the old router endpoints, conversion types, modules, SDK, uncompiled API code and test script. Nothing is deployed, so migrations 003/004 are deleted and replaced rather than followed by a drop migration. The new 003 starts by dropping the old objects if they exist, so local databases upgrade without a reset.

## Risks / Trade-offs

- [Ingestion shares the redirect process] → Body, rate and concurrency limits, and separate metrics. Budget: event traffic adds at most 2 ms to redirect p99. Measured locally (release router, 50 redirect clients, limits raised so every event was accepted): p99 5–6 ms alone, 6 ms with 2,400 events/s, 7 ms with 8,600 events/s.
- [Browser events can be forged] → Labelled source, server-only values, origin allowlist.
- [Signed or strict destination URLs break] → Route opt-out and an activation warning.
- [Safari limits script-set cookies, to 7 days or less] → Documented; recommend passing the click ID to the server and using server events.
- [Ad blockers] → Own domain and server events.
- [`FINAL` cost grows] → Ordering by workspace and conversion; revisit with measurements.
- [Skip-tracking path: in the current code a `Break` in Register appears to end the flow without a result] → Regression test in 4.4 before changing this path.
- [Consent law] → Consent mode, plus documentation that the site owner is responsible.

## Migration Plan

Nothing is live, so there is no data to migrate. Roll out in this order once deployment starts: stores, topic and exchange; aggregator, aggregator API, tracker and click-router-api; router; Management API; dashboard. Redirect tagging and ingestion stay behind separate settings.

Rollback: turn off the settings. Stored events remain.
