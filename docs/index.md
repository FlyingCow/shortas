---
title: Home
permalink: /
---

# Shortas

Shortas is a self-hostable URL shortener with custom domains and real-time click analytics. The redirect path is a set of small Rust services built for low latency. The dashboard and its API manage everything else.

## What it does

- **Short links on your own domains**: ownership is verified through DNS, and Let's Encrypt certificates are issued automatically.
- **Conditional routing**: send visitors to different destinations by country, device, browser, OS or language.
- **Click analytics**: geography, devices, browsers, traffic sources and conversions, stored in ClickHouse and visible within seconds.
- **QR codes** for every link, with custom styling.
- **Link safety**: destinations are checked against Google Safe Browsing and unsafe ones are blocked.
- **Workspaces**: invite teammates with roles; invitations arrive in-app and by email.

## How it fits together

```
 visitor --> Click Router --> Click Tracker --> Click Aggregator --> ClickHouse
                 ^                                                      |
                 | routes                                     analytics |
                 |                                                      v
            Click Router API <---- Management API <---- Dashboard <--- user
```

The [Architecture](/architecture/) page covers each piece.

## Where to go next

- [Getting Started](/getting-started/): run the whole stack on your machine.
- [Deployment](/deployment/): run it on Hetzner or AWS.
- [Development](/development/): build, test, and find your way around the repo.
- [API](/api/): the Management API the dashboard uses.
