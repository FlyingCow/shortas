# Tasks

## 1. Baseline

- [x] 1.1 Capture "before" screenshots of the Dashboard at 375px and 1280px widths (with analytics data loaded), and record the measured `document.documentElement.scrollWidth` at 375px. Verify: screenshots saved and the overflow width noted for comparison.

## 2. Mobile navigation drawer

- [x] 2.1 Replace the `<=900px` sidebar rules in `DesignSystem.css` with off-canvas styles (`translateX(-100%)`, `.is-open`, backdrop, transition) and remove the hover/focus-within expand rule and `--sidebar-width-collapsed` (design D1). Verify: at 375px the sidebar is not visible and content is full-width; at 1280px the layout is unchanged.
- [x] 2.2 Add `isNavOpen` state, a hamburger button in `.topbar-left` (hidden above 900px, with `aria-label`, `aria-expanded`, `aria-controls`), and a backdrop element to `LayoutUnified.tsx` (D1, D2). Verify: a test in `LayoutUnified.test.tsx` asserts that clicking the button sets `aria-expanded="true"` and applies `is-open`.
- [x] 2.3 Close the drawer on backdrop click, on `location.pathname` change, and on Esc, with Esc priority shortcut help → drawer → user menu (D2). Verify: tests for all three close paths pass.
- [x] 2.4 Add a body scroll-lock class while the drawer is open, removed on close and on unmount (D3). Verify: a test asserts the class is added on open and removed on close and on unmount.
- [x] 2.5 Hide the Shortcuts sidebar entry at `<=900px` (D7). Verify: at 375px the opened drawer has no "Shortcuts" entry; at 1280px it is present.

## 3. Compact topbar

- [x] 3.1 Hide the user button's email text at `<=900px` while keeping an accessible label, and add `.app-topbar .btn { width: auto; }` to override the 768px full-width button rule (D7). Verify: at 375px with a long email, all topbar controls fit on one row; a test checks the button's accessible name contains the email.
- [x] 3.2 Let the workspace dropdown shrink and truncate long names in the topbar (D7). Verify: at 360px with a 40-character workspace name, the topbar does not overflow and the user menu opens fully inside the viewport.

## 4. Shrinkable app shell

- [x] 4.1 Add `min-width: 0` to `.app-main-wrapper` and `.chart-card`, and change the `.section-row` two/three-column and 1100px collapsed tracks to `minmax(0, 1fr)` (D4). Verify: resizing the Dashboard from 1280px to 375px shrinks all charts with no horizontal scrollbar; `RoutesWithSidebar` analytics still look the same at 1280px.
- [x] 4.2 Remove the dead `.dashboard-row-geo` / `.dashboard-row-distribution` rules after confirming with grep that `src/` has no usages (D8). Verify: grep returns no matches and the build succeeds.

## 5. World map

- [x] 5.1 Switch `WorldMap` to aspect-ratio sizing (`2 / 1`, `width: 100%`, `max-height` from the `height` prop, `overflow: hidden`) and drop the fixed `height: 300px` chart-body wrappers in `DashboardUnified.tsx` and `RoutesWithSidebar.tsx` (D5). Verify: at 375px the whole map is visible in its card with no large empty bands; at 1280px the map height matches the baseline.
- [x] 5.2 Base the tooltip's placement on the container's measured size and cap its width to the container (D5). Verify: `WorldMap.test.tsx` covers placement with a narrow container; manually, tapping a country at 375px shows the tooltip fully inside the card.

## 6. Dashboard charts

- [x] 6.1 Change the Devices, Browsers, and Traffic Type pie radii to percentage values that match the current desktop size (D6). Verify: at 1280px the donuts look like the baseline; at 360px they fit their cards.
- [x] 6.2 Replace the Traffic Type outside slice labels with `PieLegend` (D6). Verify: at 360px the Traffic Type labels and legend are fully visible; existing Dashboard tests still pass.
- [x] 6.3 Add `minTickGap` / `interval="preserveStartEnd"` to the Clicks Over Time X-axis (D6). Verify: at 360px with a 30-day range, date labels don't overlap.
- [x] 6.4 Check the Dashboard header, period controls, and stat tiles at 360px and fix any overflow found (D6). Verify: a screenshot at 360px shows no overlap or clipping.

## 7. Integration check

- [x] 7.1 Run `npm test` and `npm run build` in `ui/dashboard`. Verify: both succeed.
- [x] 7.2 Capture "after" screenshots at 360px, 375px, 768px, and 1280px and compare them with the baseline. Verify: at 360px and 375px, `scrollWidth` equals the viewport width; at 1280px there are no visual differences apart from the intended pie-radius tweak; the drawer opens and closes via the button, the backdrop, a nav link, and Esc.
