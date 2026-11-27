# Tasks

Integration tests in `redirect/click-aggregator/src/conversions/store_tests.rs` need ClickHouse with migrations applied and run with `cargo test -p click-aggregator -- --ignored`. Unit tests run with `cargo test -p click-aggregator`.

## 1. Pending states and migration

- [x] 1.1 Add `redirect/click-aggregator-api/migrations/007_conversion_late.sql` with `ALTER TABLE conversion_pending MODIFY TTL` (done: 1 day, late: 4 days, both after `inserted_at`), and a comment listing the values of `resolved` (D1, D6). Verify by applying the migrations twice to a local ClickHouse without errors, then checking that `SHOW CREATE TABLE conversion_pending` shows both TTL rules.
- [x] 1.2 In `store.rs`, name the three `resolved` values as constants. Make `Batch::resolve` write `done`, and add a way to write a late row that keeps the payload. Make `waiting()` take the state to read. Verify that `cargo test -p click-aggregator` passes and that existing integration tests still pass.

## 2. Attribution in the consumer (D3)

- [x] 2.1 In `consumer::store_batch`, look up the clicks of the batch's click IDs once and apply `decide()`. Write verdicts directly and only `Wait` to pending. Make `Batch::add` take the click lookup result, so the reconciler and the consumer build the same rows and versions. Verify with an integration test: an event whose click is already stored is attributed after `store` alone, with no reconcile run and no row in `conversion_pending`.
- [x] 2.2 Verify with integration tests that an event whose click is stored later still waits and is attributed by the reconciler (existing `late_clicks_are_found_also_after_a_restart`), and that the workspace, window and filter verdicts are the same when decided in the consumer (extend `other_workspaces_clicks_are_rejected_and_reveal_nothing` and `window_and_filter_are_applied` to store the click first).

## 3. Late re-checks (D1, D2, D4)

- [x] 3.1 In `attribution.rs`, add `LATE_HORIZON` (72 h) and a function that says whether a click ID can still name a click: a ULID whose time is within the event's window before `received_at`, allowing 1 minute of clock difference after it. Verify with unit tests for a non-ULID, a ULID from after the event, a ULID older than the window, and a valid one.
- [x] 3.2 In the waiting pass, write a late row instead of done when the verdict is `click_not_found` and the click ID passes 3.1. The unattributed event row is written as today. Verify with an integration test: after 15 minutes without a click, the event is unattributed `click_not_found` and its pending row is late, while a non-ULID click ID ends done.
- [x] 3.3 Add `reconciler::recheck_late`: page through late rows, look clicks up per page, and apply D2 (attributed row + done; other verdict → done; past `LATE_HORIZON` from `received_at` → done; unreadable payload → done; otherwise nothing). Verify with integration tests for each scenario of the "Late clicks" requirement: a click stored 2 hours late gives an attributed event with unchanged count and revenue; no click after 72 hours stays `click_not_found` and becomes done; another workspace's click becomes done without changing the event; a re-check after recreating the store (restart) still finds the click.
- [x] 3.4 Add `late_recheck_millis` (default 60 000) to `conversions/settings.rs` and `config/*.toml`, and start the late pass as its own task in `conversions::start`. Verify that `cargo test -p click-aggregator` passes and that, locally, the aggregator logs no errors from either pass with conversions enabled.

## 4. Metrics and documentation

- [x] 4.1 Count `late_attributed` in `conversion_records_total`, and set a `conversion_events_late` gauge after each late pass (D5). Verify in the 3.3 test that the counter rises for the late attribution, and check `/metrics` locally for the gauge.
- [x] 4.2 In `docs/api/conversion-tracking.md`, state that an event whose click is delayed is counted as unattributed and can become attributed for up to 72 hours, without changing totals. Verify the text matches the "Late clicks" requirement in the delta spec.

## 5. Integration check

- [x] 5.1 Run the full aggregator integration suite against a ClickHouse with migrations 001–007 applied, then run `openspec validate conversion-late-attribution --strict`. Both pass.
