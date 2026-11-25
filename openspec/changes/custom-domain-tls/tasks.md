# Tasks

## 1. Baseline

- [ ] 1.1 Find out which certificate production serves for a custom domain and whether the committed key is deployed; verify by recording `openssl s_client -servername` output and fingerprints in the PR
- [ ] 1.2 If the committed key is deployed, agree on rotation with the owner; verify the new fingerprint differs
- [x] 1.3 Make the fallback configurable, generate one when unset, and document it; verify tests for configured, generated and unreadable

## 2. Certificate cache

- [x] 2.1 Propagate store errors instead of unwrapping, without caching them; verify with a unit test using a failing store
- [x] 2.2 Fix the invalidation key and normalize names to lowercase; verify with a store, replace, invalidate, read test
- [x] 2.3 Cache parsed configs and short-lived misses with size limits (D2); verify tests for malformed PEM, miss expiry and eviction

## 3. SNI selection

- [x] 3.1 Implement the async accept path behind a setting (D1); verify an integration test serves different certificates for two names
- [x] 3.2 Add fallback, lookup timeout and concurrency cap; verify tests for unknown name, no name and slow store
- [x] 3.3 Preserve HTTP/2, HTTP/1.1 and client address capture; verify integration tests for each
- [x] 3.4 Add metrics for hits, misses, errors and fallbacks; verify they appear at `/metrics`

## 4. Refresh

- [x] 4.1 Publish certificate changes from click-router-api on store and delete (D3); verify a test asserts the payload has no key material
- [x] 4.2 Consume them in click-router; verify a replaced certificate is served within 60 seconds without restart
- [x] 4.3 Correct TLS in `docs/architecture/index.md`, including the AWS limitation; verify the text matches the behavior
- [x] 4.4 Upsert certificates in MongoDB, keep the owner, read latest expiry and delete all rows (D6); verify against a real MongoDB
- [x] 4.5 List expiring certificates by each domain's current certificate, for date and text expiries (D7); verify against a real MongoDB

## 5. Integration

- [x] 5.1 Run the issue, serve, renew and delete flow locally with a test CA; verify each step with `openssl s_client`
