# Design

## Context

`RoutesWithSidebar.tsx` (around 1,800 lines, with styles embedded in a `<style>` string) renders a flex row: `.rs-sidebar` is a fixed 380px wide and `.main-content` is `flex: 1`. Three pieces of component state decide what the main pane shows: `isEditing`/`editingRoute`, `selectedRoute`, and `qrDesignerRoute`. Global keydown handlers use the same state: Escape closes the current view, and Ctrl/Alt+N opens the create form. A workspace switch clears all three.

The archived `mobile-responsive-dashboard` change already did several things this design reuses:
- Set the 900px breakpoint and the `.app-main-wrapper { min-width: 0 }` shell fix.
- Changed `.section-row` grids to `minmax(0, 1fr)`.
- Made `WorldMap` size itself by aspect ratio, with `height` used as a maximum height.

These already apply to route analytics. The Routes page still has the old pie chart radii, outside labels on the Traffic Type chart, and default X-axis ticks.

See proposal.md for motivation and specs/dashboard-mobile-layout/spec.md for the required behaviour.

## Goals / Non-Goals

**Goals:**
- The URL is the only source of truth for which Routes view is shown.
- The narrow layout is driven by CSS from a single class on the page root. There is no JS media-query listener.

**Non-Goals:**
- Splitting `RoutesWithSidebar.tsx` into smaller files, beyond what the refactor naturally extracts.
- Warning about unsaved changes when navigating away from the form (Cancel doesn't warn today).
- Putting the time range or search term in the URL.

## Decisions

### D1: Nested routes under `/routes/*`, read via `useParams`
`App.tsx` changes `<Route path="/routes">` to `/routes/*`. Inside the page, a small set of `<Routes>` entries (or `useMatch` patterns) work out a `view` value (`list | new | detail | edit | qr`) and a `routeId`. The page layout (list and main pane) stays one component, so the list keeps its scroll position and loaded data across views.

*Alternative:* a separate page component per URL. Rejected because the list would remount, losing scroll position and search results, and it would need a much larger split of the component.

### D2: Derive the active route from the URL
`selectedRoute`, `editingRoute`, and `qrDesignerRoute` become derived values. The page looks up `routeId` in the loaded routes (and in search results, if any). If it isn't found, it fetches the route once with `apiService.routes.get(id)` and stores it in an `activeRouteFallback` state. A 404 or other error does `navigate('/routes', { replace: true })`. Analytics fetching keys off the derived route id.

The state setters are replaced with `navigate()` calls:
- Selecting a card goes to `/routes/:id`.
- Create goes to `/routes/new`. Edit goes to `/routes/:id/edit`. QR goes to `/routes/:id/qr`.
- Cancel and Escape go to the parent URL: `/routes/:id` from edit or QR, and `/routes` from new or detail.
- Ctrl/Alt+N goes to `/routes/new`.
- A workspace switch does `navigate('/routes')`. It skips the first render so that opening a deep link isn't redirected immediately.

Deleting the route that is currently open also navigates to `/routes`.

*Alternative:* keep the state and sync it to the URL both ways. Rejected because two sources of truth drift apart.

### D3: Navigation after saving
- Edit: navigate to `/routes/:id`.
- Create: `POST /api/v1/routes` (the `api` service) responds 201 with the created `RouteDto`, so navigate to `/routes/${created.id}`. If `id` is missing, fall back to `/routes`.

The `apiService.routes.create` return type is narrowed to `Promise<RouteDto>`.

### D4: Narrow layout via a root class
The page root gets `has-detail` whenever `view !== 'list'`. At 900px and below:

```
.routes-with-sidebar           { flex-direction: column; height: auto }
.routes-with-sidebar .rs-sidebar { width: 100% }
.routes-with-sidebar.has-detail .rs-sidebar   { display: none }
.routes-with-sidebar:not(.has-detail) .main-content { display: none }
.routes-with-sidebar.editing .rs-sidebar       { opacity: 1 }   /* not visible anyway */
```

The fixed `height: calc(100vh - 60px)` is removed below 900px, so the page scrolls normally. This avoids `100vh` problems with mobile browser toolbars. A "< Routes" button (`navigate('/routes')`) goes at the top of each detail view. It is always rendered and hidden above 900px, the same approach as the topbar menu button.

*Alternative:* a JS `matchMedia` hook to render only one pane. Rejected for consistency with the dashboard change, which uses no JS breakpoints.

### D5: Chart treatments copied from the Dashboard
- The three donut charts use `outerRadius="70%"` and `innerRadius="45%"`.
- Traffic Type drops its `label` prop and uses `PieLegend`.
- The daily clicks `XAxis` gets `interval="preserveStartEnd"` and `minTickGap`.
- `.route-actions-header` gets `flex-wrap: wrap`, and the range group may take its own row at 360px.

If after implementation the Routes and Dashboard pages use identical chart props, a shared helper may be extracted, but it isn't required.

### D6: Card actions and QR in the header
Add `@media (hover: none) { .rs-route-actions { opacity: 1 } }`. Add a "QR Code" button to `.route-actions-header` next to Edit. In the QR designer, check the `.qr-color-row` `1fr 1fr` grid at 360px and collapse it to one column under 600px if it overflows.

### D7: Cleanup
Before deleting each file, confirm with grep across `src/` that it isn't referenced:
- `RoutesWithSidebar.css`: never imported, and its selectors predate the `rs-` redesign.
- `RouteFormModal.tsx` and `RouteFormModal.test.tsx`: no consumers.

Keep `RouteForm.css` (imported by `RouteForm.tsx`).

### D8: Dashboard attention links
In `DashboardUnified.tsx:717`, change `<Link to="/routes">` to `<Link to={`/routes/${route.id}`}>`.

## Risks / Trade-offs

- **Large refactor of a large component.** Replacing state with URL-derived values touches many handlers. → Do it as an early task by itself, verify that desktop behaviour is unchanged, then add the mobile CSS on top.
- **Deep link to a route in another workspace.** `GET /routes/:id` may return a route outside the current workspace filter. → Show it anyway (the API allows access). A workspace switch still resets to the list.
- **Redirect loop on first load.** The workspace-switch effect could fire on mount and wipe a deep link. → Track the previous workspace id and navigate only when it actually changes.
- **Hidden actions that can still be tapped (existing bug).** Fixed by D6 on touch devices. Hybrid devices that report hover keep the hover reveal.
- **Escape priority.** The global Layout handler (shortcut help, then drawer, then user menu) and the Routes handler could both act on Escape. → Keep the existing `stopImmediatePropagation`, and make sure the drawer, if open, closes first.
