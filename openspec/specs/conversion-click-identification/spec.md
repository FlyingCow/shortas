# conversion-click-identification Specification

## Purpose

Tells destination sites which short-link click brought a visitor, without changing links that do not need it.

## Requirements

### Requirement: Click ID on covered routes
The redirect SHALL add `shortas_cid` to HTTP destinations only for human clicks on tracked routes covered by an active conversion and not opted out.

#### Scenario: Covered route
- **WHEN** a visitor opens a covered link
- **THEN** the redirect location contains `shortas_cid` with the recorded click's ID

#### Scenario: Uncovered route
- **WHEN** a visitor opens a link not covered by an active conversion
- **THEN** the destination is unchanged

#### Scenario: Opted-out route
- **WHEN** a covered route has opted out
- **THEN** the destination is unchanged

#### Scenario: No click recorded
- **WHEN** tracking is skipped for the owner, the owner's settings cannot be read, the request is a detected bot or HEAD, or the destination is native
- **THEN** no click ID is added and the redirect otherwise behaves as for an uncovered route

### Requirement: Destination preservation
Adding the click ID SHALL preserve the rest of the final destination, including for conditional routes.

#### Scenario: Query and fragment
- **WHEN** the destination is `https://a.com/p?x=1&x=2#top`
- **THEN** both `x` values and `#top` are preserved

#### Scenario: Existing click ID
- **WHEN** the destination already has `shortas_cid`
- **THEN** it is replaced by the current click's ID

#### Scenario: Unparseable destination
- **WHEN** the click ID cannot be added safely
- **THEN** the visitor is redirected to the unchanged destination

### Requirement: Click ID uniqueness
Click IDs SHALL be unique across router instances and restarts, and impractical to guess.

#### Scenario: Multiple instances
- **WHEN** several instances record clicks in the same second
- **THEN** all IDs differ

### Requirement: Destination analytics unaffected
Recorded click destinations SHALL not contain the click ID.

#### Scenario: Top destinations
- **WHEN** two visitors open the same covered link
- **THEN** both clicks have the same recorded destination

### Requirement: Trackable clicks
Each recorded click SHALL state whether it was given a click ID.

#### Scenario: Covered and uncovered clicks
- **WHEN** one click is given an ID and another is not
- **THEN** only the first is recorded as trackable

### Requirement: Coverage follows changes
Route coverage SHALL be updated when conversions change or routes are created, opted out or moved between workspaces.

#### Scenario: Last conversion paused
- **WHEN** the only conversion covering a route is paused
- **THEN** its redirects stop adding a click ID after propagation
