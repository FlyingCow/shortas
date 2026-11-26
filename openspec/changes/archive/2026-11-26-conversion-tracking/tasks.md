# Tasks

## 1. Settle assumptions

- [x] 1.1 Confirm the revenue and refund rules with the user; verify design.md and specs record the answer before groups 5 and 6
- [x] 1.2 Confirm whether AWS is live, plus the other listed assumptions; verify design.md is updated before groups 2 and 4
- [x] 1.3 Find out whether old conversion tables hold data that must be kept; settled by the user: nothing is live and they may be deleted (recorded in design.md)

## 2. Definitions and API

- [x] 2.1 Add Postgres tables, entities and repositories; verify migration and repository tests
- [x] 2.2 Add CRUD, lifecycle and key endpoints with role checks; verify tests for Owner, Admin, Member and non-member
- [x] 2.3 Add validation and domain eligibility; verify tests for each invalid case in the spec
- [x] 2.4 Document endpoints in `docs/api/index.md`; verify the examples run

## 3. Propagation

- [x] 3.1 Add the conversion store to click-router-api for MongoDB and DynamoDB with version checks; verify stale-version tests
- [x] 3.2 Add the DynamoDB table and IAM in Terraform; verify `terraform validate`
- [x] 3.3 Add the fanout exchange and outbox message type; verify delivery after a simulated outage
- [x] 3.4 Add the router store and cache without panics; verify invalidation, miss-expiry and failing-store tests
- [x] 3.5 Add route flags to the API, both stores and DTOs; verify round-trip tests and that client-set `track_conversions` is ignored
- [x] 3.6 Add the resumable coverage batch and recompute on route changes; verify create, move, opt-out, pause and interrupted-batch tests
- [x] 3.7 Require Owner or Admin in the target workspace to move a route; verify over HTTP that an outsider and a Member are refused
- [x] 3.8 Register shared domains with domain-verifier at startup; verify with a stub, including when it is unavailable

## 4. Click identification

- [x] 4.1 Switch click IDs to ULIDs; verify uniqueness test and redirect benchmark
- [x] 4.2 Add `trackable` through router, tracker and aggregator with defaults; verify old payloads still parse
- [x] 4.3 Add the ClickHouse column and trackable-clicks view; verify on a fresh and an existing database
- [x] 4.4 Add a regression test for skip-tracking, overflow and HEAD requests on existing routes; verify they return a redirect
- [x] 4.5 Append the click ID behind a setting; verify tests for every scenario in the spec, including fragments and conditional routes
- [x] 4.6 Return route and user-settings lookup failures as errors instead of panicking; verify with a failing store that recovers
- [x] 4.7 Save links in lowercase, as the router looks them up; verify over HTTP that variants conflict

## 5. Ingestion

- [x] 5.1 Add isolated endpoints and reserve the path in the Management API; verify `/conversions/track` works as a short link
- [x] 5.2 Add server and browser event validation and authentication; verify tests per scenario
- [x] 5.3 Add acknowledged publishing; verify 503 and working redirects with Fluvio stopped
- [x] 5.4 Add body, rate and concurrency limits, trusted proxies and metrics; verify limit and forged-header tests
- [x] 5.5 Build the script with cookie, consent, unload delivery and several conversions per page; verify browser tests on a cross-site page
- [x] 5.6 Document script, server events, consent and cookie limits; verify examples run

## 6. Attribution and storage

- [x] 6.1 Create the topic in all compose files and the ClickHouse tables; verify migrations run twice without error
- [x] 6.2 Consume the stream and insert pending rows; verify click totals do not change
- [x] 6.3 Add the reconciler; verify late-click, restart, other-workspace, window and filter tests
- [x] 6.4 Add deduplication and refunds; verify tests per scenario, with two reconcilers running
- [x] 6.5 Add event log endpoints with workspace checks; verify cross-workspace access is refused

## 7. Dashboard

- [x] 7.1 Add Conversions navigation, list and lifecycle actions; verify component tests, including Member view
- [x] 7.2 Add the editor with ordered steps, values, route filter and domain; verify validation tests
- [x] 7.3 Add install view with one-time secret, snippets and test event; verify no browser snippet contains the secret
- [x] 7.4 Add the event log; verify attributed, unattributed, rejected and test rows
- [x] 7.5 Add route opt-out and the no-workspace explanation; verify saving and display
- [x] 7.6 Add a domain removal warning; verify it lists affected conversions

## 8. Removal

- [x] 8.1 Remove old endpoints, types, modules, SDK and test script; verify the workspaces build and tests pass
- [x] 8.2 Delete migrations 003/004 and update `apply_migrations.sh`, with the new 003 dropping old objects if present (D9); verify on a fresh database and on a local one holding the old objects

## 9. Integration

- [x] 9.1 Run click to browser step to server purchase to refund locally; verify one attributed conversion and correct net revenue
- [x] 9.2 Load-test ingestion alongside redirects; verify event traffic adds at most 2 ms to redirect p99
