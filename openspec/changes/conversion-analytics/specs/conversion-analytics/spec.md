# Spec Delta

## Purpose

Shows workspace members how well their links convert, what they earn and where visitors drop off, using consistently defined figures.

## ADDED Requirements

### Requirement: Access to statistics
Conversion statistics SHALL be available only to members of the conversion's workspace.

#### Scenario: Non-member
- **WHEN** a user outside the workspace requests statistics
- **THEN** the request is refused

### Requirement: Key figures
The system SHALL report conversions, conversion rate, net revenue, average value and revenue per trackable click, each compared with the preceding period of equal length.

#### Scenario: Period with data
- **WHEN** a member selects the last 30 days
- **THEN** each figure is shown with its change from the 30 days before

#### Scenario: No earlier data
- **WHEN** the preceding period has no data
- **THEN** no percentage change is shown

### Requirement: Conversion rate
The rate SHALL be distinct converting clicks divided by trackable clicks made in the selected period on covered routes, assigning conversions to the date of the click.

#### Scenario: Late conversion
- **WHEN** a click on Monday converts on Friday
- **THEN** it raises Monday's rate

#### Scenario: Repeat purchases
- **WHEN** one click converts three times
- **THEN** it counts once in the rate and three times in conversions when every conversion counts

#### Scenario: No trackable clicks
- **WHEN** there are none in the period
- **THEN** the rate is shown as unavailable, not zero

#### Scenario: Window still open
- **WHEN** recent clicks can still convert
- **THEN** their dates are marked as incomplete

### Requirement: Funnel
The funnel SHALL count a click at a step when it fired that step or any later one, counting each click once per step.

#### Scenario: Skipped step
- **WHEN** a click fires the final step but not the first
- **THEN** it is counted at both, and shown as inferred at the first

#### Scenario: Drop-off
- **WHEN** 100 clicks reach step 1 and 40 reach step 2
- **THEN** the step 2 completion is 40% and drop-off 60%

### Requirement: Currencies
Revenue SHALL NOT be added across currencies.

#### Scenario: Two currencies
- **WHEN** a period has revenue in EUR and USD
- **THEN** each is reported separately

### Requirement: Attributed figures
Rate, funnel and breakdowns SHALL include attributed events only, and the number of unattributed events SHALL be shown separately. Test and rejected events SHALL be excluded.

#### Scenario: Unattributed conversion
- **WHEN** a conversion has no matching click
- **THEN** it is not credited to any route and appears in the unattributed count

### Requirement: Trends
The view SHALL show conversions, rate and revenue over time without plotting different units on one pair of axes.

#### Scenario: Day without data
- **WHEN** a day has no conversions
- **THEN** it is shown as zero conversions

### Requirement: Breakdowns
The view SHALL show time to convert, top routes, countries, devices, browsers and browser versus server source.

#### Scenario: Sort routes
- **WHEN** a member sorts routes by revenue
- **THEN** routes are ordered by revenue in the selected currency

### Requirement: Route conversions
Route analytics SHALL show conversions driven by that route, per conversion.

#### Scenario: Covered route without conversions
- **WHEN** a covered route has no conversions
- **THEN** an empty state links to setup

### Requirement: Dashboard tile
The main dashboard SHALL show conversions for the current workspace and period.

#### Scenario: No conversions defined
- **WHEN** the workspace has none
- **THEN** the tile offers to create one

### Requirement: Readable on phones
Conversion views SHALL fit a 360px viewport without horizontal page scrolling, and charts SHALL convey values without relying on color alone.

#### Scenario: Phone
- **WHEN** the analytics view opens at 360px
- **THEN** all charts and controls fit the page width
