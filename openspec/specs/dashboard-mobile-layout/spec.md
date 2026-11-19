# dashboard-mobile-layout Specification

## Purpose

Defines how the dashboard's app shell (navigation, topbar, content area) and the Dashboard page's analytics visualisations adapt to narrow, touch-only viewports so the product is fully usable on phones.

## Requirements

### Requirement: Navigation reachable on narrow viewports
At viewport widths of 900px and below, the system SHALL show a menu button in the topbar that opens the main navigation as an off-canvas drawer, so navigation is reachable without hover.

#### Scenario: Open navigation on a phone
- **GIVEN** a viewport 375px wide on a touch device
- **WHEN** the user taps the menu button in the topbar
- **THEN** a drawer slides in containing Dashboard, Routes, Domains, Workspaces, Clickstream, Settings, and the theme toggle

#### Scenario: Navigation hidden by default
- **GIVEN** a viewport 900px wide or narrower
- **WHEN** any page loads
- **THEN** the navigation drawer is closed and page content uses the full viewport width

#### Scenario: Desktop unchanged
- **GIVEN** a viewport wider than 900px
- **WHEN** any page loads
- **THEN** the sidebar is permanently visible and no menu button is shown

### Requirement: Navigation drawer dismissal
The navigation drawer SHALL close when the user taps outside it, selects a navigation destination, or presses Escape.

#### Scenario: Tap backdrop
- **GIVEN** the navigation drawer is open
- **WHEN** the user taps the area outside the drawer
- **THEN** the drawer closes

#### Scenario: Navigate to a page
- **GIVEN** the navigation drawer is open
- **WHEN** the user taps "Routes"
- **THEN** the Routes page is shown and the drawer is closed

#### Scenario: Press Escape
- **GIVEN** the navigation drawer is open
- **WHEN** the user presses Escape
- **THEN** the drawer closes

### Requirement: Background does not scroll behind open drawer
While the navigation drawer is open, the page content behind it SHALL NOT scroll.

#### Scenario: Swipe on open drawer
- **GIVEN** the navigation drawer is open on a long page
- **WHEN** the user swipes vertically
- **THEN** the underlying page keeps its scroll position

#### Scenario: Scroll restored on close
- **GIVEN** the navigation drawer was open
- **WHEN** the drawer closes
- **THEN** the page scrolls normally again

### Requirement: Compact topbar on narrow viewports
At viewport widths of 900px and below, the topbar SHALL fit within the viewport width without overflow, with the user menu button showing only its icon.

#### Scenario: Topbar on a phone
- **GIVEN** a viewport 375px wide and a signed-in user with a long email address
- **WHEN** any page loads
- **THEN** the menu button, workspace switcher, notifications, and user menu button are all visible within the viewport and the email is not shown in the button

#### Scenario: User menu still accessible
- **GIVEN** a viewport 375px wide
- **WHEN** the user taps the user menu button
- **THEN** the user menu opens showing username, email, Settings, and Logout, fully within the viewport

### Requirement: Keyboard shortcuts entry hidden on narrow viewports
The navigation SHALL NOT show the keyboard shortcuts entry at viewport widths of 900px and below.

#### Scenario: Drawer contents on a phone
- **GIVEN** a viewport 375px wide
- **WHEN** the user opens the navigation drawer
- **THEN** no "Shortcuts" entry is present

### Requirement: No horizontal page overflow on Dashboard
The Dashboard page SHALL render without horizontal page scrolling at viewport widths down to 360px.

#### Scenario: Dashboard at phone width
- **GIVEN** a viewport 360px wide and a workspace with analytics data in every section
- **WHEN** the Dashboard page loads
- **THEN** the document width equals the viewport width and no content extends beyond the right edge

#### Scenario: Resizing narrower
- **GIVEN** the Dashboard is open in a 1280px wide window
- **WHEN** the window is resized to 375px
- **THEN** all charts and the world map shrink to fit their cards without horizontal page scrolling

### Requirement: World map fits its card
The world map SHALL scale to the width of its card while preserving its aspect ratio, and SHALL NOT render outside its card.

#### Scenario: Map on a phone
- **GIVEN** a viewport 375px wide
- **WHEN** the Geographic Distribution card renders
- **THEN** the whole map is visible within the card, without large empty bands above or below it

#### Scenario: Country details within bounds
- **GIVEN** a viewport 375px wide
- **WHEN** the user taps or hovers a country with click data
- **THEN** the country details tooltip is displayed entirely within the map card

### Requirement: Dashboard charts legible on narrow viewports
Dashboard charts SHALL fit within their cards at viewport widths down to 360px, with labels and legends not clipped.

#### Scenario: Distribution charts on a phone
- **GIVEN** a viewport 360px wide
- **WHEN** the Devices, Browsers, and Traffic Type cards render
- **THEN** each chart, its slice labels, and its legend are fully visible inside its card

#### Scenario: Dashboard header controls on a phone
- **GIVEN** a viewport 360px wide
- **WHEN** the Dashboard page loads
- **THEN** the page header, period controls, and stat tiles are fully visible without overlapping or overflowing
