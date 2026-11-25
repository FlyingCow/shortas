# Tasks

## 1. Confirm

- [ ] 1.1 Confirm the chart list and that revenue rules match `conversion-tracking`; verify design.md records the answer

## 2. Statistics queries

- [ ] 2.1 Add summary and time series with previous period; verify tests for empty periods, currencies and refunds
- [ ] 2.2 Add funnel and time to convert; verify tests for skipped steps and repeat events
- [ ] 2.3 Add route, geographic, device, browser and source breakdowns; verify unattributed, test and rejected rows are excluded
- [ ] 2.4 Add route and workspace summaries; verify a late conversion counts on its click date
- [ ] 2.5 Measure queries on a seeded data set; verify timings are recorded and rollups are decided on

## 3. Management API

- [ ] 3.1 Add authorized endpoints; verify member and non-member tests
- [ ] 3.2 Document endpoints and figure definitions in `docs/api/index.md`; verify examples run

## 4. Conversion analytics view

- [ ] 4.1 Add key figures, period and currency selection; verify tests for no data and unavailable rate
- [ ] 4.2 Add trend charts with incomplete dates marked; verify tests and no dual axis
- [ ] 4.3 Add the funnel; verify the tooltip separates observed and inferred
- [ ] 4.4 Add the remaining breakdowns; verify sorting and empty states
- [ ] 4.5 Check at 360px and in light and dark themes; verify no horizontal scroll and that every chart has text labels

## 5. Dashboard and routes

- [ ] 5.1 Add the dashboard tile; verify with and without conversions
- [ ] 5.2 After `mobile-routes-page` lands, add the route analytics section; verify with and without conversions, and at 360px

## 6. Integration

- [ ] 6.1 Generate clicks and events through the real pipeline; verify each figure against a hand-computed result
