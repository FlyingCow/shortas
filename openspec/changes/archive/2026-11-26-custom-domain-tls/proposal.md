# Proposal

## Why

Click Router loads one static certificate for its HTTPS listener, and the Docker image does not contain that file, so from the code the HTTPS task fails at startup and custom domains are not served over HTTPS at all. The certificate store, cache and Cert Bot already exist, but nothing connects them to the TLS handshake. Conversion snippets served from a workspace's own domain (change `conversion-tracking`) depend on this working.

## What Changes

- Select the certificate for each TLS handshake by server name, from the existing certificate store.
- Fall back to a configured certificate, or one generated at startup when none is configured.
- Replace stored certificates on renewal instead of adding a second row (MongoDB), and read the one that expires last.
- Pick up issued, renewed and deleted certificates without a restart.
- Keep serving when the certificate store fails, without panicking.
- Fix the MongoDB query that finds certificates due for renewal.
- Fix cache invalidation, which uses a different key from the one entries are stored under.
- Correct the architecture docs, and replace any deployed use of the private key committed at `click-router/certs/key.pem`.

Out of scope: TLS on AWS, where the load balancer terminates HTTPS with one configured certificate.

## Capabilities

### New Capabilities

- `custom-domain-tls`: certificate selection, fallback and refresh for HTTPS requests to short-link domains.

### Modified Capabilities

_None._

## Impact

- `redirect/click-router`: HTTPS listener, certificate cache, RabbitMQ consumer.
- `redirect/click-router-api`: publishes certificate changes.
- RabbitMQ: one new fanout exchange.
- `docs/architecture/index.md`.
