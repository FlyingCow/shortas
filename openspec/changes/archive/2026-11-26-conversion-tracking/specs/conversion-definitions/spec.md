# Spec Delta

## Purpose

Lets workspace members define what counts as a conversion, control when it is tracked, and get what they need to install it.

## ADDED Requirements

### Requirement: Conversion definition
The system SHALL let a workspace Owner or Admin create a conversion with a name, steps, attribution window, currency, counting mode and short domain.

#### Scenario: Create a conversion
- **WHEN** an Admin submits a valid definition
- **THEN** the conversion is created as a draft with a public key and a secret key

#### Scenario: Invalid definition
- **WHEN** the definition has no steps, duplicate step keys, an unknown currency or a window outside 1 to 90 days
- **THEN** the system rejects it and names the invalid fields

### Requirement: Access control
Workspace members SHALL be able to view conversions, and only Owners and Admins SHALL be able to change them or their keys.

#### Scenario: Member tries to edit
- **WHEN** a workspace Member submits a change
- **THEN** the system responds with forbidden

#### Scenario: Non-member
- **WHEN** a user outside the workspace requests a conversion
- **THEN** the system does not reveal it

### Requirement: Ordered steps
A conversion SHALL have 1 to 20 ordered steps, each with an immutable key, a name and an optional default value. The final step SHALL be the conversion.

#### Scenario: Single step
- **WHEN** a conversion has one step
- **THEN** that step is the conversion

#### Scenario: Step with history removed
- **WHEN** an Admin removes a step that has events
- **THEN** the step is archived, its events are kept, and new events for it are rejected

### Requirement: Route coverage
A conversion SHALL cover every route in its workspace unless it has a route filter.

#### Scenario: New route
- **WHEN** a route is created in a workspace with an active unfiltered conversion
- **THEN** the route is covered without further setup

#### Scenario: Filtered conversion
- **WHEN** a conversion lists specific routes
- **THEN** only those routes are covered

#### Scenario: Route without a workspace
- **WHEN** a route belongs to no workspace
- **THEN** the dashboard explains that it cannot drive conversions

### Requirement: Lifecycle
A conversion SHALL be draft, activating, active, paused or archived. Only active conversions SHALL accept live events.

#### Scenario: Activation
- **WHEN** an Admin activates a conversion
- **THEN** it shows as activating until covered routes are updated, then active

#### Scenario: Pause
- **WHEN** an Admin pauses a conversion
- **THEN** new events are rejected within the cache lifetime and stored events remain

### Requirement: Domain required for activation
Activation SHALL require a verified short domain with a live certificate that the workspace owner owns or that is shared.

#### Scenario: No eligible domain
- **WHEN** an Admin activates a conversion without an eligible domain
- **THEN** activation is refused and the missing condition is stated

#### Scenario: Domain in use removed
- **WHEN** a user tries to remove a domain used by active conversions
- **THEN** the system warns and lists those conversions

### Requirement: Secret keys
The secret key SHALL be shown only when created or rotated, and SHALL be stored only as a hash.

#### Scenario: Rotation
- **WHEN** an Admin rotates the secret key
- **THEN** the new key is shown once and the previous key stops working within the cache lifetime

### Requirement: Reliable propagation
Saved changes SHALL reach the redirect service even if it is unavailable when they are saved.

#### Scenario: Redirect service API unavailable
- **WHEN** a change is saved while that API is down
- **THEN** the save succeeds and the change is delivered after it recovers

### Requirement: Install instructions
The dashboard SHALL show a script tag, step calls, a server request example and a test action for each conversion.

#### Scenario: Copy the script
- **WHEN** a user opens the install view
- **THEN** snippets use the conversion's domain and public key, and no browser snippet contains the secret key

#### Scenario: Test event
- **WHEN** a user sends a test event
- **THEN** it appears in the event log marked as a test and is excluded from totals
