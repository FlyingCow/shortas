# Proposal

## Why

The dashboard UI is unusable on phones. Below 900px the sidebar collapses to 0px and only expands on hover, so touch users have no way to reach navigation. On the Dashboard page the world map and Recharts charts overflow the viewport horizontally because the layout containers cannot shrink below their content's intrinsic width.

## What Changes

- Add a mobile navigation drawer: below 900px a hamburger button in the topbar opens the existing sidebar as an off-canvas drawer over a backdrop. It closes on backdrop tap, route change, or Esc, and locks body scroll while open. The hover-to-expand behaviour is removed.
- Compact the topbar on small screens: the user menu button shows only its icon (no email), and the Shortcuts entry is hidden in the mobile drawer.
- Make the app shell shrinkable so content never forces horizontal page scroll (flex/grid min-width fixes on the main wrapper and section rows).
- Make the world map size by aspect ratio instead of a fixed pixel height, clip it to its card, and keep its hover tooltip within the map bounds.
- Make Dashboard charts fit narrow cards (pie radius and Traffic Type labels, header controls, stat tiles at ~375px width).
- Remove dead responsive CSS rules (`.dashboard-row-geo`, `.dashboard-row-distribution`) that target classes no longer used.
- Desktop layout (>900px) is unchanged.

Out of scope (follow-up change): Routes page list/analytics split, Domains and Workspaces tables, route form and QR designer modals on mobile.

## Capabilities

### New Capabilities
- `dashboard-mobile-layout`: How the dashboard app shell (navigation, topbar, content area) and the Dashboard page's analytics visualisations adapt to narrow, touch-only viewports.

### Modified Capabilities
<!-- none: no existing specs -->

## Impact

- `ui/dashboard/src/components/LayoutUnified.tsx`: drawer open/close state, hamburger button, backdrop, route-change/Esc handling, compact user button.
- `ui/dashboard/src/components/DesignSystem.css`: sidebar/app-shell responsive rules, `.section-row` column sizing, world map container, Dashboard responsive rules, removal of dead selectors.
- `ui/dashboard/src/components/WorldMap.tsx`: aspect-ratio sizing and tooltip positioning.
- `ui/dashboard/src/components/DashboardUnified.tsx`: chart sizing adjustments for narrow cards.
- `ui/dashboard/src/components/LayoutUnified.test.tsx`, `WorldMap.test.tsx`: test updates for new behaviour.
- `RoutesWithSidebar` shares `.section-row` and `WorldMap`, so it may benefit incidentally; no dedicated work there.
- No API, dependency, or backend changes.
