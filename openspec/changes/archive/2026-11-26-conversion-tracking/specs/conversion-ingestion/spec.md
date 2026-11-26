# Spec Delta

## Purpose

Receives conversion events from customers' websites and servers without affecting short links or redirect availability.

## ADDED Requirements

### Requirement: Isolated endpoints
Conversion endpoints SHALL be under `/.well-known/shortas/` on every router host, and routes SHALL NOT be creatable under that path.

#### Scenario: Similar short link
- **WHEN** a customer has a short link `/conversions/track`
- **THEN** the link works normally

#### Scenario: Reserved path
- **WHEN** a user creates a route under `/.well-known/shortas/`
- **THEN** the system rejects it

### Requirement: Generated script
The system SHALL serve a script for each active conversion at `/.well-known/shortas/c/{publicKey}.js`.

#### Scenario: Active conversion
- **WHEN** a page requests the script
- **THEN** it receives cacheable JavaScript configured for that conversion

#### Scenario: Unknown or inactive conversion
- **WHEN** the key is unknown or the conversion is not active
- **THEN** the response is not found

#### Scenario: Several scripts on one page
- **WHEN** a page loads scripts for two conversions
- **THEN** each step call reaches its intended conversion

### Requirement: Browser click ID storage
The script SHALL keep the click ID from the landing URL in a first-party cookie for at most the attribution window.

#### Scenario: Later page
- **WHEN** a visitor lands with a click ID and converts on another page of the site
- **THEN** the event carries that click ID

#### Scenario: Consent required
- **WHEN** the site enables consent mode and consent has not been given
- **THEN** the script writes no cookie and sends no events

### Requirement: Browser events
The script SHALL send step events with a public key, without credentials and without requiring a CORS preflight.

#### Scenario: Page closing
- **WHEN** a step fires as the page unloads
- **THEN** the script attempts delivery in a way that survives unload

#### Scenario: Origin not allowed
- **WHEN** allowed origins are configured and the request's origin is not listed
- **THEN** the event is rejected, unless it is a test event, which is accepted and not counted

#### Scenario: Server-only values
- **WHEN** the conversion accepts values only from servers and a browser event includes a value
- **THEN** the browser value is ignored

### Requirement: Server events
The system SHALL accept server events authenticated by the conversion's secret key as a bearer token.

#### Scenario: Valid event
- **WHEN** a server sends a valid step event with a click ID
- **THEN** the system responds 202 with an event ID

#### Scenario: Bad key
- **WHEN** the secret key is missing or wrong
- **THEN** the system responds 401 and nothing is stored

#### Scenario: Occurrence time
- **WHEN** a server supplies an occurrence time in the future or more than 90 days old
- **THEN** the event is rejected

### Requirement: Event validation
The system SHALL reject events with an unknown step, a negative or non-finite value, an invalid currency, or more than 16 KB of data.

#### Scenario: Unknown step
- **WHEN** an event names a step that is not active
- **THEN** the response identifies the invalid field

### Requirement: Durable acceptance
The system SHALL acknowledge an event only after the event stream has accepted it.

#### Scenario: Stream unavailable
- **WHEN** the event cannot be published
- **THEN** the response is 503 and redirects continue to work

### Requirement: Abuse limits
The system SHALL limit events per key and per client address, and cap concurrent conversion requests. Requests with a valid secret key SHALL be exempt from the address limit.

#### Scenario: Limit exceeded
- **WHEN** a key exceeds its limit
- **THEN** the response is 429 with a retry delay and redirects are unaffected

#### Scenario: Busy server
- **WHEN** a server with a valid secret key exceeds the address limit but not the key limit
- **THEN** its events are accepted

#### Scenario: Forged forwarding header
- **WHEN** a client that is not a trusted proxy sends a forwarded-address header
- **THEN** limits use the connection's address
