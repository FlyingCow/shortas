---
layout: vector-theme
title: Home
permalink: /
---

# Shortas Documentation

Shortas is a high-performance URL shortener and analytics platform built with Rust microservices, a .NET management API, and a React dashboard.

## Core Capabilities

- **URL shortening** with custom domains and routing policies
- **Conditional routing** based on geography, device, browser, or OS
- **Real-time click tracking** through a Fluvio-based event pipeline
- **Geographic and device analytics** powered by ClickHouse
- **Multi-tenant workspaces** with Keycloak authentication
- **QR code generation** with customizable design options
- **Route safety verification** via Google Safe Browsing integration
- **Automatic favicon extraction** for destination URLs
- **Domain ownership verification** for custom domains

## System Components

| Component | Stack | Role |
|-----------|-------|------|
| Click Router | Rust / Salvo | Handles redirects (incl. conditional) and emits click events |
| Click Tracker | Rust / Tokio | Enriches events with geo, UA, session data |
| Click Aggregator | Rust / Tokio | Stores enriched events in ClickHouse |
| Route Verifier | Rust / Tokio | Checks routes against Safe Browsing |
| Route Icon Worker | Rust / Tokio | Scrapes favicons from destination URLs |
| Domain Verifier | Rust / Tokio | Verifies custom domain ownership via DNS |
| Management API | C# / ASP.NET Core 9 | Workspace, route, and user management |
| Dashboard | React 18 / TypeScript | Admin interface with analytics charts |

## Quick Links

- [Getting Started](/getting-started/) — set up a local development environment
- [Architecture](/architecture/) — system design and data flow
- [API Reference](/api/) — REST endpoint documentation
- [Deployment](/deployment/) — Docker Compose and production setup
- [Development](/development/) — build, test, and contribute
