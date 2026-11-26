# custom-domain-tls Specification

## Purpose

Lets visitors reach short links and other router endpoints over HTTPS on custom domains, using the certificate issued for each domain.

## Requirements

### Requirement: Certificate selection by server name
The router SHALL present the stored certificate that matches the server name requested in the TLS handshake. Matching SHALL be case-insensitive.

#### Scenario: Domain with a certificate
- **WHEN** a client connects over HTTPS to a domain that has a stored, valid certificate
- **THEN** the router presents that domain's certificate

#### Scenario: Mixed-case server name
- **WHEN** a client requests `Go.Example.com` and a certificate is stored for `go.example.com`
- **THEN** the router presents the certificate for `go.example.com`

### Requirement: Fallback certificate
The router SHALL present a fallback certificate when no usable certificate matches. Without a configured fallback it SHALL generate a self-signed one at startup.

#### Scenario: Unknown domain
- **WHEN** a client connects to a domain with no stored certificate
- **THEN** the router presents the fallback certificate

#### Scenario: Nothing configured
- **WHEN** the router starts without a configured fallback certificate
- **THEN** HTTPS starts with a generated certificate

#### Scenario: Configured fallback unusable
- **WHEN** the configured fallback cannot be read or parsed
- **THEN** startup fails with an error naming the file

#### Scenario: No server name
- **WHEN** a client connects without a server name
- **THEN** the router presents the fallback certificate

#### Scenario: Malformed stored certificate
- **WHEN** the stored certificate or key for a domain cannot be parsed
- **THEN** the router presents the fallback certificate and records an error naming the domain

### Requirement: Certificate changes apply without a restart
The router SHALL start using an issued, renewed or deleted certificate within 60 seconds of a delivered change notification, and within the configured cache lifetime if the notification is lost.

#### Scenario: First certificate issued
- **WHEN** a certificate is stored for a domain that previously received the fallback
- **THEN** new handshakes for that domain receive the new certificate within 60 seconds

#### Scenario: Certificate renewed
- **WHEN** a certificate is replaced
- **THEN** new handshakes receive the replacement within 60 seconds

#### Scenario: Notification lost
- **WHEN** a certificate is replaced and the notification is not delivered
- **THEN** new handshakes receive the replacement once the cache lifetime passes

### Requirement: One current certificate per domain
Storing a certificate for a domain SHALL replace the previous one and keep its owner. If several exist, the one that expires last SHALL be used.

#### Scenario: Renewal
- **WHEN** a certificate is stored for a domain that already has one
- **THEN** lookups return the new certificate and the owner is unchanged

#### Scenario: Existing duplicates
- **WHEN** a domain already has two stored certificates
- **THEN** lookups return the one that expires last

#### Scenario: Deletion with duplicates
- **WHEN** a domain's certificate is deleted
- **THEN** no certificate remains for that domain

### Requirement: Certificates due for renewal are found
The system SHALL list a domain as expiring when its current certificate expires before the requested time.

#### Scenario: Expiring soon
- **WHEN** a domain's current certificate expires before the requested time
- **THEN** the domain is listed, including when its expiry was stored by an earlier version

#### Scenario: Already renewed
- **WHEN** a domain has an older expiring row and a current certificate that expires later
- **THEN** the domain is not listed

### Requirement: Certificate store failure
The router SHALL continue to accept HTTPS connections when the certificate store is unavailable.

#### Scenario: Store down with a cached certificate
- **WHEN** the store is unavailable and the domain's certificate is cached and unexpired
- **THEN** the router presents the cached certificate

#### Scenario: Store down without a cached certificate
- **WHEN** the store is unavailable and the domain's certificate is not cached
- **THEN** the router presents the fallback certificate and other connections are unaffected

### Requirement: Bounded certificate lookups
The router SHALL limit the cost of handshakes for names it does not serve.

#### Scenario: Flood of unknown names
- **WHEN** many handshakes request distinct unknown names
- **THEN** cache size and concurrent store lookups stay within configured limits
