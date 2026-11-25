# Spec Delta

## ADDED Requirements

### Requirement: Routes views are addressable by URL
Each Routes page view SHALL have its own URL: `/routes` for the list, `/routes/new` for creating a route, `/routes/:id` for a route's analytics, `/routes/:id/edit` for editing it, and `/routes/:id/qr` for its QR designer. Browser back and forward navigation SHALL move between these views.

#### Scenario: Open create form updates the URL
- **GIVEN** the user is on `/routes`
- **WHEN** the user taps "Create Route"
- **THEN** the URL becomes `/routes/new` and the create form is shown

#### Scenario: Back returns to the previous view
- **GIVEN** the user opened `/routes/:id` from the list
- **WHEN** the user uses the browser or gesture back navigation
- **THEN** the URL becomes `/routes` and the list is shown

#### Scenario: Deep link to a route not in the loaded list
- **GIVEN** a route exists but is not among the routes currently loaded in the list
- **WHEN** the user opens `/routes/:id` for that route
- **THEN** that route's analytics are shown

#### Scenario: Unknown route id
- **GIVEN** no route exists with the given id
- **WHEN** the user opens `/routes/:id`
- **THEN** the user is redirected to `/routes`

#### Scenario: Workspace switch returns to the list
- **GIVEN** the user is viewing `/routes/:id`
- **WHEN** the user switches the current workspace
- **THEN** the URL becomes `/routes`

### Requirement: Navigation after saving a route
After a route is saved, the system SHALL show that route's analytics view.

#### Scenario: Save an edit
- **GIVEN** the user is on `/routes/:id/edit`
- **WHEN** the user saves the form successfully
- **THEN** the URL becomes `/routes/:id`

#### Scenario: Create a route
- **GIVEN** the user is on `/routes/new`
- **WHEN** the user creates a route successfully
- **THEN** the URL becomes `/routes/<new route id>`

### Requirement: Routes page shows one pane at a time on narrow viewports
At viewport widths of 900px and below, the Routes page SHALL show only the route list at `/routes`, and only the detail pane at full viewport width on any other Routes URL. Above 900px, the list and detail pane SHALL remain side by side.

#### Scenario: Create route on a phone
- **GIVEN** a viewport 375px wide on `/routes`
- **WHEN** the user taps "Create Route"
- **THEN** the create form fills the viewport width and the route list is not shown

#### Scenario: List shown by default on a phone
- **GIVEN** a viewport 375px wide
- **WHEN** the user opens `/routes`
- **THEN** the route list fills the viewport width

#### Scenario: Desktop unchanged
- **GIVEN** a viewport wider than 900px
- **WHEN** the user opens `/routes/new`
- **THEN** the route list and the create form are shown side by side

### Requirement: Back to Routes control on narrow viewports
At viewport widths of 900px and below, every Routes detail view SHALL show a control that returns to the route list.

#### Scenario: Return from analytics
- **GIVEN** a viewport 375px wide on `/routes/:id`
- **WHEN** the user taps the back-to-Routes control
- **THEN** the URL becomes `/routes` and the list is shown

### Requirement: Route analytics fit narrow viewports
The route analytics view SHALL render without horizontal page scrolling at viewport widths down to 360px. Its charts and world map SHALL fit within their cards, with labels and legends not clipped.

#### Scenario: Analytics on a small phone
- **GIVEN** a viewport 360px wide
- **WHEN** the user opens `/routes/:id` for a route with click data
- **THEN** the page does not scroll horizontally, and every chart, legend, and the world map stay inside their cards

#### Scenario: Analytics header controls wrap
- **GIVEN** a viewport 360px wide on `/routes/:id`
- **WHEN** the analytics header renders
- **THEN** the Edit, QR Code, time range, and refresh controls are all visible within the viewport width

### Requirement: QR designer reachable from route detail
The route analytics view SHALL include a QR Code action that opens that route's QR designer.

#### Scenario: Open QR designer from analytics
- **GIVEN** the user is on `/routes/:id`
- **WHEN** the user taps "QR Code" in the header
- **THEN** the URL becomes `/routes/:id/qr` and the QR designer is shown

### Requirement: QR designer fits narrow viewports
The QR designer SHALL render without horizontal page scrolling at viewport widths down to 360px.

#### Scenario: QR designer on a small phone
- **GIVEN** a viewport 360px wide
- **WHEN** the user opens `/routes/:id/qr`
- **THEN** the preview and all controls are within the viewport width

### Requirement: Route card actions visible on touch devices
On devices without hover capability, route card actions SHALL be visible without any prior interaction.

#### Scenario: Touch device shows card actions
- **GIVEN** a touch-only device on `/routes`
- **WHEN** the route list renders
- **THEN** each route card shows its QR Code, Copy URL, Edit, and Delete actions

#### Scenario: Pointer device keeps hover reveal
- **GIVEN** a device with hover capability
- **WHEN** the route list renders
- **THEN** card actions appear only when hovering over a card

### Requirement: Dashboard attention items link to the route
Each route listed in the Dashboard's "needs attention" section SHALL link to that route's analytics view.

#### Scenario: Open an attention item
- **GIVEN** the Dashboard lists a route needing attention
- **WHEN** the user taps it
- **THEN** the URL becomes `/routes/:id` for that route
