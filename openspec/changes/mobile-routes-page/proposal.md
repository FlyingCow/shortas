# Proposal

## Why

The Routes page is unusable on phones. It is a side-by-side list and detail layout with a fixed 380px list and no narrow-viewport handling, so at phone widths the detail pane gets no width. As a result, the create/edit form, route analytics, and QR designer are invisible. Route card actions are hover-only (`opacity: 0`), so on touch devices they are invisible but still tappable. The QR designer has no other entry point.

## What Changes

- Give each Routes view its own URL under `/routes/*`: `/routes` (list), `/routes/new`, `/routes/:id` (analytics), `/routes/:id/edit`, `/routes/:id/qr`. The URL replaces the component state for selection, editing, and the QR designer. Browser and gesture back navigation then work, and routes can be deep-linked.
- At viewport widths of 900px and below, show one pane at a time: the list at `/routes`, otherwise the detail pane at full width with a "back to Routes" control. Desktop layout is unchanged.
- Make route analytics fit at 360px, using the same treatments the Dashboard page uses: percentage donut radii, the Traffic Type legend instead of outside labels, fewer X-axis ticks on the daily clicks chart, and a wrapping analytics header.
- Make the QR designer reachable from the route detail header and verify that it fits at 360px.
- Show route card actions permanently on devices without hover.
- After saving an edit, go to `/routes/:id`. After creating, go to the new route's `/routes/:id`.
- Link Dashboard "needs attention" rows to the specific route (`/routes/:id`) instead of `/routes`.
- Cleanup: delete the unused `RoutesWithSidebar.css` (never imported, with stale selectors) and the unused `RouteFormModal` component and its test.

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

- `dashboard-mobile-layout`: extends narrow-viewport coverage from the app shell and Dashboard page to the Routes page. Adds addressable Routes views, one-pane-at-a-time behaviour on narrow viewports, analytics chart fit, QR designer access, touch-visible card actions, and deep links from the Dashboard attention list.

## Impact

- `ui/dashboard/src/App.tsx`: `/routes` becomes `/routes/*`.
- `ui/dashboard/src/components/RoutesWithSidebar.tsx`: state-to-URL refactor, narrow-viewport layout, chart props, header actions, embedded styles.
- `ui/dashboard/src/components/DashboardUnified.tsx`: attention-row links.
- `ui/dashboard/src/components/QRCodeDesigner.css`: narrow-width fixes if needed.
- Removed: `RoutesWithSidebar.css`, `RouteFormModal.tsx`, `RouteFormModal.test.tsx`.
- No API or backend changes. The existing `GET /routes/:id` and the create response (the created `RouteDto`) are sufficient.
