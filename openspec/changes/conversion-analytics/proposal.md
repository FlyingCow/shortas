# Proposal

## Why

`conversion-tracking` records conversions but shows only an event log. Users need to see which links convert, how much they earn, and where visitors drop off.

## What Changes

- Add conversion statistics endpoints to click-aggregator-api and the Management API.
- Add an analytics view per conversion: key figures, trends, funnel, time to convert, top routes and breakdowns.
- Add a conversions section to route analytics.
- Add a conversions tile to the main dashboard.

Depends on `conversion-tracking`. Route analytics work starts after `mobile-routes-page`, which edits the same file. Referrer breakdown is excluded because clicks do not record a referrer.

## Capabilities

### New Capabilities

- `conversion-analytics`: conversion statistics, their definitions, and how they are shown.

### Modified Capabilities

_None._

## Impact

- `redirect/click-aggregator-api`: statistics queries and endpoints.
- `api`: authorized proxy endpoints.
- `ui/dashboard`: Conversions, Routes and Dashboard pages.
