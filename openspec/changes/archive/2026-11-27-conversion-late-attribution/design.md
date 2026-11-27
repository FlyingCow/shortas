# Design

## Context

See proposal.md for motivation. Current behavior in `redirect/click-aggregator/src/conversions`:

- The consumer stores events with a click ID in `conversion_pending` (`resolved = 0`, payload = the record). It stores everything else directly.
- Every 5 s the reconciler reads `conversion_pending FINAL WHERE resolved = 0`, looks the clicks up in `click_stream` and calls `decide()`. A missing click means `Wait` for 15 minutes (`CLICK_WAIT`), then `Unattributed("click_not_found")`. Each verdict also writes a pending row with `resolved = 1` and an empty payload.
- `conversion_pending` is a `ReplacingMergeTree(resolved)` ordered by `id`, so the highest `resolved` wins. Its TTL deletes `resolved = 1` rows after a day. The version column of a table cannot be changed later.
- `conversion_events` keeps, for each dedupe key, the row with the highest `version`. For the same event, `version(true)` is one more than `version(false)`, so an attributed row replaces the unattributed one.
- Click IDs are ULIDs made by the router. Ingestion accepts any string of up to 64 characters, and browser events are rate limited per key (200/s by default).
- Conversions are now enabled by default, so `conversion_pending` may already hold data. Changes to it go in a new migration.

## Goals / Non-Goals

**Goals:** attribution that survives click-pipeline delays of up to 72 hours. Fewer pending writes and reconciler work in the common case.

**Non-Goals:** recovering clicks the router dropped because its queue was full. Same-partition ordering or a click watermark (discussed and set aside: they couple the redirect, the tracker and the aggregator). Correcting verdicts other than `click_not_found`. Moving tables off the S3 disk.

## Decisions

### D1. Pending states in the existing version column

`resolved` keeps its name and type, and gains a third value. The highest value wins:

```
  0  waiting   re-checked every 5 s                      payload kept
  1  late      re-checked every minute, up to 72 h       payload kept
  2  done      never re-checked                          payload empty
```

```
  stream --> consumer --click stored--> conversion_events   (D3)
                |
                +--no click--> [0 waiting] --15 min--> unattributed row
                                                       + [1 late]
                                [1 late] --click--> attributed row + [2 done]
                                         --other verdict or 72 h--> [2 done]
```

Existing `resolved = 1` rows have empty payloads. The late pass treats an unreadable payload as done, so those rows become done without any migration of data.

*Alternatives:* a new table for late events, which means a second scheme to keep consistent. A new version column, which means recreating the table, because ClickHouse cannot change the engine's version column.

### D2. The late pass

There is a separate repeating task with its own interval (`late_recheck_millis`, default 60 000). It pages through `resolved = 1` the same way the waiting pass does, and looks the clicks up in one query per page.

- Click found and `decide()` returns `Attributed`: write the attributed event row (`version(true)`) and a done row.
- Click found, any other verdict: write only a done row. The stored record stays `click_not_found`. Rewriting it would have to remove events from `conversion_events` for rejections, and the reason changes no figure.
- No click, and `now - received_at >= 72 h`: done row.
- No click otherwise: nothing.

The horizon is counted from `received_at` (receipt at the router), as the spec says. A constant, `LATE_HORIZON`, sits next to `CLICK_WAIT`.

*Alternative:* letting the TTL end late rows. That would mean querying with `inserted_at`, which measures from when the row became late, not from receipt.

### D3. Attribute in the consumer

`store_batch` gathers the click IDs of the batch and makes one `clicks()` lookup. For each record with a click ID, it calls `decide(record, click, now, now)`:

- A verdict: write it directly with the same rows and version the reconciler would write. No pending row.
- `Wait` (click not stored): pending, as today.

Waiting and late rows then hold only real misses. If the lookup fails, the batch fails and is read again, the same as a failed insert, so nothing is decided without the click data.

*Alternative:* falling back to pending when the lookup fails. That keeps ingestion going while `click_stream` cannot be read, but adds a second path for the same outcome. Both queries go to the same server, so the gain is small.

### D4. Which events become late

An event moves from waiting to late only if its click ID could still name a stored click. That means it is a ULID whose timestamp lies between `received_at` minus the attribution window and `received_at`. Anything else ends as `click_not_found` with a done row, exactly as today.

Without this check, forged browser events with random click IDs (up to 200/s per key) would each be re-checked every minute for 72 hours.

### D5. Metrics

- `conversion_records_total{outcome="late_attributed"}` when the late pass attributes an event. If it moves, the click pipeline was late.
- `conversion_events_late` (gauge): late events after the last pass, next to `conversion_events_waiting`.

### D6. Migration 007

`ALTER TABLE conversion_pending MODIFY TTL` with two rules:

- Delete `resolved = 2` rows a day after `inserted_at`.
- Delete `resolved = 1` rows 4 days after `inserted_at`. This is a safety net in case the late pass stops; it normally ends late rows at 72 h.

The statement can be repeated safely. 006 is left alone, because it may already have been applied.

## Risks / Trade-offs

- [Attributed figures for recent events can change for up to 72 h] → Totals and revenue do not change. Document it in the conversion tracking docs. `conversion-analytics` reads the current status of each event, so it needs no change.
- [After a spike, many late rows] → They are bounded by the spike's misses. Each page is one `FINAL` read and one `IN` lookup. The gauge shows the size.
- [Forged ULID-shaped click IDs pass D4] → They are bounded by the per-key rate limit and the window. The gauge shows the effect. A stricter limit can follow if it is ever seen.
- [The consumer now reads `click_stream` on the S3 disk about once a second] → One primary-key lookup per batch, which replaces a pending write, a `FINAL` read and a resolved write per event.
- [Integration tests use click IDs that are not ULIDs] → Tests that expect late re-checks use ULID click IDs.
- [Equal-version rows] → Not produced. The late pass writes only an attributed row, which has the higher version.

## Migration Plan

1. Apply migration 007. It is harmless to the current code: the new TTL deletes `resolved = 1` rows after 4 days instead of 1.
2. Deploy click-aggregator. Existing `resolved = 1` rows become done on the first late pass (D1).

Rollback: deploy the previous aggregator. It treats any non-zero `resolved` as resolved, so late rows are simply no longer re-checked. Attributions already made remain.
