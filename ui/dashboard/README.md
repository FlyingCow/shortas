# Shortas Dashboard

React admin interface for managing short links and viewing click analytics.

## Features

- Link management (create, edit, delete short URLs)
- Conditional routing configuration (geo, device, browser, OS targeting)
- QR code designer with customizable colors, logos, and patterns
- Click analytics with interactive charts (Recharts)
- Geographic click distribution map (React Simple Maps)
- Route blocking/unblocking for Safe Browsing violations
- Automatic favicon display for destination URLs
- Workspace management
- Keyboard shortcuts for common actions
- Keycloak-based authentication

## Tech Stack

- React 18 with TypeScript
- Bootstrap 5 / React-Bootstrap
- Recharts for data visualization
- Axios for API calls
- Keycloak JS for authentication
- React Router for navigation

## Development

```bash
npm install
npm start
```

Runs on `http://localhost:3000`. API requests are proxied to `http://localhost:8080` (see `proxy` in `package.json`).

## Build

```bash
npm run build
```

## Docker

```bash
docker compose -f ../docker-compose.yml up dashboard
```

Serves the built app on port 80 (mapped to 3000).
