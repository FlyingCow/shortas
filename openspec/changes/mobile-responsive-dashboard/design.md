# Design

## Context

See proposal.md for motivation. Relevant current state:

- The app shell lives in `LayoutUnified.tsx` with styles in `DesignSystem.css` (`.app-layout`, `.app-sidebar`, `.app-main-wrapper`, `.app-topbar`). The sidebar is `position: fixed`. Above 900px the main wrapper is offset by `--sidebar-width` (224px). At 900px and below, `--sidebar-width-collapsed: 0px` plus a `:hover`/`:focus-within` expand rule makes the sidebar unreachable on touch devices.
- `.app-layout` is a flex row. `.app-main-wrapper` (`flex: 1`) has no `min-width: 0`, so its minimum width is its content's intrinsic width.
- Dashboard sections use `.section-row.two-cols` / `.three-cols` (CSS grid, `repeat(n, 1fr)`, collapsing to `1fr` at 1100px). A `1fr` track resolves to `minmax(auto, 1fr)`, so it also refuses to shrink below its content's minimum width.
- Charts use Recharts `ResponsiveContainer width="100%"`. It measures its parent, so once an ancestor has grown, the chart holds it at that size.
- `WorldMap` renders an 800x400 viewBox SVG inside a container with a fixed pixel `height` prop (300) and `overflow: visible`. Its tooltip has `minWidth: 180px` and is positioned relative to the container.
- A global rule around `DesignSystem.css:2627` forces `overflow: visible !important` on `.card`, `.card-body`, `.flex`, `.container` (added so dropdowns aren't clipped).
- A rule in the 768px media block sets `.btn { width: 100% }`. Any button placed in the topbar (hamburger, user button) would stretch to full width unless excluded.
- Dead selectors `.dashboard-row-geo` / `.dashboard-row-distribution` exist in the CSS but are not used in markup.

## Goals / Non-Goals

**Goals:**
- One breakpoint (900px) for the shell switch, matching the existing sidebar breakpoint.
- Fix overflow at the root (shrinkable containers) rather than with per-component `overflow-x: hidden` patches.
- Keep the desktop layout pixel-identical above 900px.

**Non-Goals:**
- Swipe gestures to open or close the drawer.
- A collapsed icon rail.
- Refactoring the inline styles in `LayoutUnified.tsx` beyond what the drawer needs.
- Mobile layouts for Routes, Domains, Workspaces, and the modals (follow-up change).

## Decisions

### D1: Off-canvas drawer reusing the existing `<aside>`
At 900px and below, `.app-sidebar` keeps its full `--sidebar-width` but is translated off-screen (`transform: translateX(-100%)`). A `.app-sidebar.is-open` class translates it back. A sibling backdrop element (rendered only while open) covers the page and closes the drawer on click. The `:hover`/`:focus-within` expand rule and the `--sidebar-width-collapsed` mechanism are removed.

*Alternatives:* bottom tab bar (needs a separate home for Settings, Theme and Shortcuts, and duplicates the navigation markup); 56px icon rail (takes too much width on a 375px screen). A drawer is the smallest change and keeps one source of navigation markup.

### D2: Drawer state in `Layout`, closed on route change
`isNavOpen` lives in `Layout` alongside `isUserMenuOpen`. A `useEffect` on `location.pathname` sets it to false, which covers nav links, the logo link, and programmatic navigation. Esc handling joins the existing global `keydown` handler with priority: shortcut help, then drawer, then user menu.

The hamburger button is rendered always but hidden by CSS above 900px, so no JS media-query listener is needed. It gets `aria-label="Open navigation"`, `aria-expanded`, and `aria-controls` pointing at the sidebar id.

*Alternative:* closing via `onClick` on each link. Rejected because it misses programmatic navigation and the logo link.

### D3: Scroll lock via a body class
While the drawer is open, add a class to `document.body` that sets `overflow: hidden`, and remove it on close and on unmount. This is simple and sufficient for the drawer's short content. iOS Safari rubber-banding may still move the backdrop slightly; that's acceptable.

### D4: Make the shell shrinkable
- `.app-main-wrapper { min-width: 0; }`
- `.section-row.two-cols` → `repeat(2, minmax(0, 1fr))`, `.three-cols` → `repeat(3, minmax(0, 1fr))`, and the 1100px collapse → `minmax(0, 1fr)`.
- `.chart-card { min-width: 0; }` as a belt-and-braces measure for grid children.

This removes the feedback loop where the chart holds its container wide. It also benefits `RoutesWithSidebar`, which uses the same classes, without changing that page's layout.

*Alternative:* `overflow-x: hidden` on `.app-main`. Rejected because it hides the symptom and clips content that should reflow.

### D5: World map sized by aspect ratio
`WorldMap` drops the fixed-height container in favour of `aspect-ratio: 2 / 1` (matching the 800x400 viewBox) with `width: 100%` and an optional `max-height` taken from the existing `height` prop, so the desktop size is unchanged. The container uses `overflow: hidden`. Callers in `DashboardUnified` and `RoutesWithSidebar` stop wrapping the map in a fixed `height: 300px` chart body.

Tooltip: compute left/right/top/bottom placement from the container's actual `clientHeight` (it currently uses the `height` prop), and cap its width with `max-width: calc(100% - 20px)` so it can't exceed a narrow map.

### D6: Chart sizing for narrow cards
- Pie charts: replace fixed `outerRadius={70}` / `innerRadius={45}` with percentage radii (for example `"70%"` / `"45%"`), which Recharts supports, so donuts scale with the card.
- Traffic Type: move the outside slice labels into the shared `PieLegend` used by Devices and Browsers, so nothing renders outside the pie's bounding box.
- Area chart: reduce X-axis tick density on narrow widths (`interval="preserveStartEnd"` and `minTickGap`) so date labels don't collide.
- Header and stat tiles: verify the existing 768px/640px rules at 360px and adjust only if overflow is observed.

### D7: Topbar compaction and `.btn` width override
At 900px and below, the user button's email `<span>` is hidden (it gets a class, and keeps an `aria-label` with the email). Topbar buttons are excluded from the 768px `.btn { width: 100% }` rule by scoping an override to `.app-topbar .btn { width: auto; }`. The workspace dropdown gets `min-width: 0` and text truncation so long workspace names shrink instead of overflowing.

The Shortcuts entry in the sidebar footer gets a class that is `display: none` at 900px and below.

### D8: Remove dead selectors
Delete the `.dashboard-row-geo` and `.dashboard-row-distribution` rules and their media-query blocks. Confirm with grep across `src/` before deleting.

## Risks / Trade-offs

- [The global `overflow: visible !important` override may still let some child spill out] → After D4, test at 360px. If a specific container still overflows, scope a fix to that container rather than touching the global rule, which exists for dropdowns.
- [The `aspect-ratio` change alters the map's height on the Routes analytics page too] → Pass the existing `height` as `max-height` so desktop rendering is unchanged, and check the Routes analytics panel visually once.
- [Percentage pie radii change the desktop donut size slightly] → Choose percentages that reproduce roughly 70/45px at the current desktop card size.
- [Body scroll lock is left on if `Layout` unmounts while the drawer is open] → Remove the class in the effect cleanup.
- [Tests that query the sidebar by role or visibility may change] → Update `LayoutUnified.test.tsx` and add drawer tests (open, close on backdrop, Esc, route change).

## Migration Plan

Frontend-only and CSS/React-only; it ships with the normal dashboard build. Roll back by reverting the commit. No data or API changes.
