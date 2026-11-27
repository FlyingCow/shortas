# Spec Delta

## MODIFIED Requirements

### Requirement: Late clicks
The system SHALL retry attribution when an event arrives before its click is stored, for up to 15 minutes. After that, an event whose click was not found SHALL be stored as unattributed and re-checked for 72 hours from its receipt. When its click is stored in that time, the event SHALL become attributed in place of the unattributed record.

#### Scenario: Event first
- **WHEN** the click is stored within 15 minutes of the event
- **THEN** the event becomes attributed

#### Scenario: Restart while waiting
- **WHEN** the aggregator restarts before the click arrives
- **THEN** the event is still retried

#### Scenario: Click delayed by a backlog
- **WHEN** the click is stored 2 hours after the event
- **THEN** the event is unattributed with reason `click_not_found` until then, and attributed afterwards
- **AND** conversion counts and revenue are the same before and after

#### Scenario: Click never stored
- **WHEN** the click is not stored within 72 hours of the event
- **THEN** the event stays unattributed with reason `click_not_found`

#### Scenario: Restart while re-checking
- **WHEN** the aggregator restarts during the 72 hours
- **THEN** the event is still re-checked

#### Scenario: Verdict made with the click
- **WHEN** the click was found and the event was unattributed or rejected for another reason
- **THEN** the event is not re-checked
