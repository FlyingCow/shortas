# Tasks

## 1. Settle assumptions

- [ ] 1.1 Confirm the revenue and refund rules with the user; verify design.md and specs record the answer before groups 5 and 6
- [ ] 1.2 Confirm whether AWS is live, plus the other listed assumptions; verify design.md is updated before groups 2 and 4
- [ ] 1.3 Check row counts of migration 003/004 tables in every deployed database; verify counts are recorded in the PR

## 2. Definitions and API

- [ ] 2.1 Add Postgres tables, entities and repositories; verify migration and repository tests
- [ ] 2.2 Add CRUD, lifecycle and key endpoints with role checks; verify tests for Owner, Admin, Member and non-member
- [ ] 2.3 Add validation and domain eligibility; verify tests for each invalid case in the spec
- [ ] 2.4 Document endpoints in `docs/api/index.md`; verify the examples run

## 3. Propagation

- [ ] 3.1 Add the conversion store to click-router-api for MongoDB and DynamoDB with version checks; verify stale-version tests
- [ ] 3.2 Add the DynamoDB table and IAM in Terraform; verify `terraform validate`
- [ ] 3.3 Add the fanout exchange and outbox message type; verify delivery after a simulated outage
- [ ] 3.4 Add the router store and cache without panics; verify invalidation, miss-expiry and failing-store tests
- [ ] 3.5 Add route flags to the API, both stores and DTOs; verify round-trip tests and that client-set `track_conversions` is ignored
- [ ] 3.6 Add the resumable coverage batch and recompute on route changes; verify create, move, opt-out, pause and interrupted-batch tests

## 4. Click identification

- [ ] 4.1 Switch click IDs to ULIDs; verify uniqueness test and redirect benchmark
- [ ] 4.2 Add `trackable` through router, tracker and aggregator with defaults; verify old payloads still parse
- [ ] 4.3 Add the ClickHouse column and trackable-clicks view; verify on a fresh and an existing database
- [ ] 4.4 Add a regression test for skip-tracking, overflow and HEAD requests on existing routes; verify they return a redirect
- [ ] 4.5 Append the click ID behind a setting; verify tests for every scenario in the spec, including fragments and conditional routes

## 5. Ingestion

- [ ] 5.1 Add isolated endpoints and reserve the path in the Management API; verify `/conversions/track` works as a short link
- [ ] 5.2 Add server and browser event validation and authentication; verify tests per scenario
- [ ] 5.3 Add acknowledged publishing; verify 503 and working redirects with Fluvio stopped
- [ ] 5.4 Add body, rate and concurrency limits, trusted proxies and metrics; verify limit and forged-header tests
- [ ] 5.5 Build the script with cookie, consent, unload delivery and several conversions per page; verify browser tests on a cross-site page
- [ ] 5.6 Document script, server events, consent and cookie limits; verify examples run

## 6. Attribution and storage

- [ ] 6.1 Create the topic in all compose files and the ClickHouse tables; verify migrations run twice without error
- [ ] 6.2 Consume the stream and insert pending rows; verify click totals do not change
- [ ] 6.3 Add the reconciler; verify late-click, restart, other-workspace, window and filter tests
- [ ] 6.4 Add deduplication and refunds; verify tests per scenario, with two reconcilers running
- [ ] 6.5 Add event log endpoints with workspace checks; verify cross-workspace access is refused

## 7. Dashboard

- [ ] 7.1 Add Conversions navigation, list and lifecycle actions; verify component tests, including Member view
- [ ] 7.2 Add the editor with ordered steps, values, route filter and domain; verify validation tests
- [ ] 7.3 Add install view with one-time secret, snippets and test event; verify no browser snippet contains the secret
- [ ] 7.4 Add the event log; verify attributed, unattributed, rejected and test rows
- [ ] 7.5 Add route opt-out and the no-workspace explanation; verify saving and display
- [ ] 7.6 Add a domain removal warning; verify it lists affected conversions

## 8. Removal

- [ ] 8.1 Remove old endpoints, types, modules, SDK and test script; verify the workspaces build and tests pass
- [ ] 8.2 Drop old tables and views only after 1.3 shows this is safe; verify in a copy of a deployed schema

## 9. Integration

- [ ] 9.1 Run click to browser step to server purchase to refund locally; verify one attributed conversion and correct net revenue
- [ ] 9.2 Load-test ingestion alongside redirects; verify redirect p99 stays within the budget agreed in the PR
