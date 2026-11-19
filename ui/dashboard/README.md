# dashboard

The React admin app for links, QR codes, domains, workspaces and analytics. It talks only to the Management API and signs in through Keycloak or Cognito with `keycloak-js`.

## Run

```bash
cp .env.example .env.local    # then adjust
npm install
npm start                     # http://localhost:3000
npm test
npm run build
```

In the full compose stack the dashboard is served by nginx on port 3000.

## Config

| Variable | Purpose |
|---|---|
| `REACT_APP_PROXY_API_URL` | Management API base URL (`http://localhost:8090` locally) |
| `REACT_APP_KEYCLOAK_URL`, `REACT_APP_KEYCLOAK_CLIENT_ID` | OIDC provider and client |
| `REACT_APP_ROUTE_IMAGES_BASE_URL` | Where favicons and QR assets are served (`minio-nginx`, `http://localhost:9003`) |
| `REACT_APP_LANDING_URL` | Link back to the landing page |
| `REACT_APP_USE_MOCK_DATA` | `true` uses mock data and skips sign-in |
