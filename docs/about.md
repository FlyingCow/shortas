---
title: About
permalink: /about/
---

# About

Shortas is an open-source URL shortener built to explore what a fast, event-driven link platform looks like end to end. It covers the redirect hot path, streaming analytics, custom domains with automatic TLS, and multi-tenant management.

## Design goals

- **Fast redirects**: a small Rust service with cached lookups and no synchronous writes on the hot path.
- **Analytics without slowing clicks**: every click is streamed, enriched and aggregated asynchronously.
- **Self-hostable**: the whole platform runs from a single Compose file and scales out to more hosts without changes.

## Author and license

Built by Max Cechina. MIT licensed.

- Source: [github.com/FlyingCow/shortas](https://github.com/FlyingCow/shortas)
- Issues: [github.com/FlyingCow/shortas/issues](https://github.com/FlyingCow/shortas/issues)
