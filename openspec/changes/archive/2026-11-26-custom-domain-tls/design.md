# Design

## Context

See proposal.md for motivation. Constraints found in the code:

- `create_default_tls_config` in `click-router/src/main.rs` loads `certs/cert.pem` and has a TODO for SNI. `ResolvesServerCert` is imported but not implemented.
- `MokaCryptoCache` reads through to Mongo or DynamoDB, but unwraps store errors inside the cache loader.
- Entries are keyed `crypto_<domain>`, while `invalidate` removes `<domain>`. No caller of it was found.
- rustls `ResolvesServerCert::resolve` is synchronous; the cache is async.
- On Hetzner the router publishes 443 directly. On AWS the ALB terminates TLS.

This is based on reading code. Production behavior has not been tested.

## Goals / Non-Goals

**Goals:** per-domain certificates on Hetzner, refresh without restart, no panics in handshakes.

**Non-Goals:** AWS TLS, changes to Cert Bot issuance, OCSP stapling.

## Decisions

### D1. Resolve asynchronously in the accept step
Use rustls `LazyConfigAcceptor` to read the ClientHello, await the cache, then finish the handshake with a config for that certificate. Alternative: preload all certificates into a snapshot for a synchronous resolver. That makes handshakes simpler, but memory grows with all domains and new certificates depend on refresh. Blocking on async inside `resolve` is rejected because it can stall runtime threads.

### D2. Cache parsed server configs
Cache the parsed config beside the stored certificate it came from, and parse again only when that certificate changes, so one invalidation refreshes both. Cache misses separately with a short lifetime (30 s), so a new domain recovers quickly even if a notification is lost. Both caches are bounded, and store lookups have a timeout and a concurrency cap.

### D3. Certificate change exchange
click-router-api publishes to a new fanout exchange after storing or deleting a certificate. The router consumes it as it does for routes and invalidates positive and negative entries. Messages contain the domain only, never key material.

### D4. Errors are not cached
A store error returns the fallback for that handshake and is not stored. Unexpired entries survive an outage until their lifetime ends.

### D5. Fallback certificate
Use the configured file if set; otherwise generate a self-signed certificate at startup (confirmed by the user). Requiring a mounted file was rejected as extra setup, and shipping the committed key was rejected because it is public.

### D6. Certificate replacement
On MongoDB, storing upserts by hostname and preserves the owner; reads sort by latest expiry; deletion removes every row. Existing duplicates are not cleaned up and no unique index is added here.

### D7. Expiry listing
Expiry was stored as text and compared with a date, so nothing was ever due for renewal. Store dates, match existing text rows too, and judge each domain by its current row so duplicates do not cause extra renewals.

## Risks / Trade-offs

- [Handshake latency on a cold cache] → Lookup timeout and parsed-config cache.
- [Replacing the HTTPS accept loop breaks HTTP/2 or client address capture] → Integration tests for ALPN and `ConnectInfo`.
- [Committed private key used in production] → Task 1.1 checks this first; rotate if so.
- [Workspaces on AWS still lack custom-domain TLS] → Documented limitation.

## Migration Plan

1. Deploy click-router-api (the exchange has no consumer yet, which is harmless).
2. Deploy click-router with a setting that enables SNI selection.
3. Verify a known custom domain with `openssl s_client`.

Rollback: disable the setting to return to the static certificate.
