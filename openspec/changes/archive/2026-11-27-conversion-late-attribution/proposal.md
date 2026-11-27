# Proposal

## Why

An event whose click is not stored within 15 minutes is recorded as unattributed (`click_not_found`) and never checked again. Clicks pass through the hit stream, click-tracker and the aggregator. A click spike can delay that path by more than 15 minutes, so valid conversions lose their attribution for good, during the traffic that matters most.

Every event with a click ID also goes through `conversion_pending` and waits for the next reconciler run, even when its click is already stored. That is the usual case, because a visitor converts after landing.

## What Changes

- After the 15-minute wait, events counted as `click_not_found` keep being re-checked for 72 hours. When the click is stored, the event becomes attributed in place of its unattributed record. Totals do not change; only attribution and breakdowns do.
- Only `click_not_found` is re-checked. Verdicts made with the click present (not trackable, route not covered, before the click, outside the window, another workspace) stay final.
- The aggregator attributes events when it reads them from the stream if their click is already stored. Only events without a stored click wait in `conversion_pending`.
- A metric counts events attributed late, so delays in the click pipeline become visible.
- Migration `007` changes the lifetime of `conversion_pending` rows to fit the new states.

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

- `conversion-attribution`: the "Late clicks" requirement extends retries past 15 minutes with re-checks for 72 hours, and says what changes when a late click is found.

## Impact

- `redirect/click-aggregator`: conversions consumer, reconciler, attribution, store and their tests.
- `redirect/click-aggregator-api/migrations`: new `007` for `conversion_pending`.
- `docs/api/conversion-tracking.md`: when attribution can still change.
- `conversion-analytics` (in progress): attributed figures and breakdowns for recent events can change for up to 72 hours after an event. Its specs need no change, because they already read the current status of each event.
