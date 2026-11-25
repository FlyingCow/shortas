# Tasks

## 1. URL-addressable Routes views

- [x] 1.1 Change `/routes` to `/routes/*` in `App.tsx`, and work out `view` (`list | new | detail | edit | qr`) and `routeId` from the URL inside `RoutesWithSidebar` (D1). Verify that opening `/routes/new` directly renders the create form.
- [x] 1.2 Replace `selectedRoute`, `editingRoute`, `isEditing`, and `qrDesignerRoute` state with values derived from the URL, plus a fallback that fetches by id and redirects to `/routes` on 404 (D2). Verify that a deep link to a route outside the loaded page shows its analytics, and that a bad id redirects to `/routes`.
- [ ] 1.3 Change card select, Create, Edit, QR, Cancel, Escape, Ctrl/Alt+N, deleting the open route, and workspace switch to use `navigate()` (D2). Make sure the workspace effect doesn't fire on mount. Verify each by hand in the browser, including that back and forward move between views.
- [x] 1.4 Navigate to `/routes/:id` after an edit and to `/routes/<new id>` after a create, using the created `RouteDto` returned by the API, and narrow the `apiService.routes.create` return type (D3). Verify both flows end on the analytics of the saved route.
- [x] 1.5 Add `RoutesWithSidebar.test.tsx` covering URL to view mapping, the Cancel and Escape targets, post-save navigation, and redirect on an unknown id (mock `apiService`). Verify `npm test -- RoutesWithSidebar --watchAll=false` passes.
- [ ] 1.6 Check that desktop (>900px) looks and behaves the same as before, apart from URLs: list beside the main pane, dimmed list while editing, existing shortcuts. Verify by hand at 1280px.

## 2. Narrow-viewport layout

- [ ] 2.1 Add the `has-detail` root class and the 900px rules: stacked layout, full-width list, hide whichever pane isn't active, drop the fixed height and the editing fade (D4). Verify at 375px that Create shows the form at full width with the list hidden.
- [ ] 2.2 Add a "< Routes" back button to the top of the analytics, form, and QR views, hidden above 900px. Verify at 375px that it returns to `/routes`, and at 1280px that it isn't shown.
- [x] 2.3 Extend `RoutesWithSidebar.test.tsx` to check that the back button navigates to `/routes` and that the root has `has-detail` on non-list URLs. Verify the tests pass.

## 3. Analytics fit at 360px

- [ ] 3.1 Switch the three donut charts to percentage radii and replace the Traffic Type outside labels with `PieLegend` (D5). Verify at 360px that donuts and legends stay inside their cards.
- [ ] 3.2 Add `interval="preserveStartEnd"` and `minTickGap` to the daily clicks X axis. Verify at 360px with 30d range that date labels don't overlap.
- [ ] 3.3 Make `.route-actions-header` wrap so Edit, QR Code, time range, and Refresh fit (D5). Verify at 360px that every control is visible and the page doesn't scroll horizontally.
- [ ] 3.4 Check the world map and stat tiles on `/routes/:id` at 360px, and fix any overflow. Verify that `document.documentElement.scrollWidth <= innerWidth` at 360px.

## 4. QR designer and card actions

- [ ] 4.1 Add a "QR Code" button to the analytics header that navigates to `/routes/:id/qr` (D6). Verify by tapping it at 375px.
- [ ] 4.2 Show `.rs-route-actions` permanently under `@media (hover: none)` (D6). Verify in a touch-emulated browser at 375px that actions are visible, and with a mouse at 1280px that they still appear on hover.
- [ ] 4.3 Check the QR designer at 360px and collapse `.qr-color-row` to one column if it overflows (D6). Verify the preview and all controls fit with no horizontal scroll, and that `QRCodeDesigner.test.tsx` still passes.

## 5. Dashboard link and cleanup

- [ ] 5.1 Point the Dashboard "needs attention" rows to `/routes/${route.id}` (D8). Verify that tapping a row opens that route's analytics.
- [x] 5.2 Confirm with grep that nothing references `RoutesWithSidebar.css`, `RouteFormModal.tsx`, or `RouteFormModal.test.tsx`, then delete them (D7). Verify that `npm run build` and `npm test -- --watchAll=false` pass.

## 6. Integration check

- [ ] 6.1 Run through the whole page on a 360px and a 375px touch-emulated viewport: list, then create and save onto analytics, then edit and cancel, then QR and back, then gesture/browser back to list, then a Dashboard attention link. Then repeat the main flows at 1280px. Verify there is no horizontal scroll and every spec scenario holds.

## Workflow follow-up

- Archive the change after implementation is reviewed and merged.
