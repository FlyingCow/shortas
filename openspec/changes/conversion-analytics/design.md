# Design

## Context

See proposal.md for motivation. `conversion_events` and the trackable-clicks view come from `conversion-tracking`. The dashboard uses recharts and an existing `WorldMap`. Existing stats endpoints go through the Management API, which checks workspace membership, to click-aggregator-api, which is private.

## Goals / Non-Goals

**Goals:** one definition of each figure, shared by all views.

**Non-Goals:** cohorts, multi-touch, goals, currency conversion, exports, referrer.

## Confirmed with the user

Revenue counts the final step only. The chart list is proposed and has not been confirmed separately; task 1.1 settles it.

## Decisions

### D1. Query events directly
Read `conversion_events FINAL`, filtered by workspace and conversion. Materialized views were rejected: they process inserts, so replaced rows would be counted twice, and averages and distinct counts do not sum.

### D2. Cohort by click date
Rates, funnel and time to convert use the click's date. Conversion and revenue totals use the event date. Labels state which. The denominator is the trackable-clicks view filtered to covered routes.

### D3. Funnel
Compute the furthest current step per click, then count clicks at or beyond each step. Observed counts come from the events themselves.

### D4. Charts

| Figure | Form |
|---|---|
| Key figures | Tiles with change |
| Conversions and rate | Two stacked line charts, shared dates |
| Revenue | Area, one currency at a time |
| Funnel | Horizontal bars, observed and inferred in the tooltip |
| Time to convert | Histogram: <1h, 1-24h, 1-3d, 3-7d, 7d+ |
| Routes | Sortable table with inline bars |
| Countries | Existing world map |
| Devices, browsers | Ranked bars |
| Source | Stacked bar |

### D5. Endpoints
Under `/conversions/{id}/stats/`: `summary`, `timeseries`, `funnel`, `time-to-convert`, `routes`, `geographic`, `devices`, `browsers`, `sources`; plus a route summary and a workspace summary. Parameters are bound or escaped as in the existing store, and ranges are limited to 366 days.

## Risks / Trade-offs

- [`FINAL` is slow at volume] → Measure in 2.5; add a scheduled rollup if needed.
- [Recent rates look low] → Marked incomplete.
- [Coverage changed during the period] → The trackable flag is recorded per click; a narrowed route filter applies to the whole period, and the UI says so.
- [Conflicts with `mobile-routes-page`] → Route work is last.

## Migration Plan

No data migration. Deploy click-aggregator-api, the Management API, then the dashboard. Rollback: redeploy the previous dashboard.
