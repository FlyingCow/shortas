# Spec Delta

## Purpose

Connects conversion events to the clicks that caused them and stores trustworthy counts and values.

## ADDED Requirements

### Requirement: Click matching
An event SHALL be attributed to the click named by its click ID when the click is trackable, in the same workspace, on a covered route and inside the attribution window. Other clicks SHALL NOT be inferred.

#### Scenario: Match
- **WHEN** an event names an eligible click
- **THEN** it takes its route, country, device, operating system and browser from that click

#### Scenario: No click ID
- **WHEN** an event has no click ID
- **THEN** it is stored as unattributed and is not matched by IP address

#### Scenario: Another workspace's click
- **WHEN** the click belongs to another workspace
- **THEN** the event is rejected and reveals nothing about that click

#### Scenario: Outside the window
- **WHEN** the event occurred before the click or after the window
- **THEN** it is stored as unattributed with that reason

### Requirement: Late clicks
The system SHALL retry attribution when an event arrives before its click is stored, for up to 15 minutes.

#### Scenario: Event first
- **WHEN** the click is stored within 15 minutes of the event
- **THEN** the event becomes attributed

#### Scenario: Restart while waiting
- **WHEN** the aggregator restarts before the click arrives
- **THEN** the event is still retried

### Requirement: Deduplication
Repeated delivery SHALL NOT change totals.

#### Scenario: Once per click
- **WHEN** counting is once per click and a click fires a step twice
- **THEN** it counts once, using the earliest accepted event

#### Scenario: Every conversion
- **WHEN** counting is every conversion and two events have different external IDs
- **THEN** both count

#### Scenario: Same external ID
- **WHEN** counting is every conversion and an external ID is repeated for a step, from a browser or a server
- **THEN** it counts once

#### Scenario: Client retry
- **WHEN** an event without an external ID is retried with the same event ID
- **THEN** it counts once

### Requirement: Value and revenue
An event's value SHALL be its submitted value, or the step's default. Revenue SHALL be the sum of final-step values, per currency.

#### Scenario: Default value
- **WHEN** an event has no value and its step defaults to 5
- **THEN** its value is 5

#### Scenario: Earlier step with a value
- **WHEN** a cart step has value 49 and the final purchase has value 49
- **THEN** revenue is 49

#### Scenario: Appending a step later
- **WHEN** a step is added after the final step
- **THEN** conversions already recorded remain conversions

### Requirement: Refunds
The system SHALL accept refunds from servers only, referencing the original external ID in the same currency.

#### Scenario: Partial refund
- **WHEN** 20 is refunded from a 49 conversion
- **THEN** net revenue falls by 20 and the conversion count is unchanged

#### Scenario: Excess refund
- **WHEN** refunds would exceed the original value
- **THEN** the excess does not reduce revenue and the log shows the refund as invalid

#### Scenario: Repeated refund
- **WHEN** a refund ID is repeated
- **THEN** it applies once

### Requirement: Event log
Workspace members SHALL be able to see recent events with step, source, value, status and reason, including rejected events that identified a known conversion.

#### Scenario: Debugging an install
- **WHEN** an event is rejected for an origin that is not allowed
- **THEN** the log shows it with that reason

### Requirement: Click statistics unaffected
Conversion events SHALL NOT be counted as clicks.

#### Scenario: Event received
- **WHEN** a conversion event is stored
- **THEN** click totals are unchanged
