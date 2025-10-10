# Port Configuration

## Updated Configuration

Both Keycloak and API services are now configured to use port **8080**.

### Current Setup

```env
# Both services on same port with different paths
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_PROXY_API_URL=http://localhost:5050
```

### Service Endpoints

#### Keycloak
- **Base URL**: `http://localhost:8080`
- **Realm**: `http://localhost:8080/realms/shortas-dev`
- **Auth**: `http://localhost:8080/realms/shortas-dev/protocol/openid-connect/auth`
- **Token**: `http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/token`

#### API Services
- **Proxy API**: `http://localhost:5050`
- **Swagger UI**: `http://localhost:5050/swagger/index.html`
- **API Documentation**: `http://localhost:5050/swagger/v1/swagger.json`

### Path Structure

```
http://localhost:8080/           # Keycloak Server
├── realms/shortas-dev/          # Keycloak endpoints
└── admin/                       # Keycloak Admin Console

http://localhost:5050/           # Proxy API Server
├── api/v1/                      # Routes API
├── api/aggregator/v1/           # ClickStream API
├── swagger/index.html           # Swagger UI
└── swagger/v1/swagger.json      # API Documentation
```

### Keycloak Client Configuration

Update your Keycloak client settings:

**Valid redirect URIs**:
- `http://localhost:3000/*`

**Valid post logout redirect URIs**:
- `http://localhost:3000/logged-out`

**Web origins**:
- `http://localhost:3000`

### Testing Endpoints

#### Test Keycloak
```bash
curl http://localhost:8080/realms/shortas-dev/.well-known/openid-configuration
```

#### Test Proxy API
```bash
curl http://localhost:5050/api/health
```

#### Test Swagger UI
```bash
curl http://localhost:5050/swagger/index.html
```

#### Test API Documentation
```bash
curl http://localhost:5050/swagger/v1/swagger.json
```

### Development Mode

For development without running services, use mock data:

```env
REACT_APP_USE_MOCK_DATA=true
```

This bypasses all network calls and uses local mock data.

### Production Configuration

For production, update to your actual domain:

```env
REACT_APP_KEYCLOAK_URL=https://your-keycloak-domain.com
REACT_APP_PROXY_API_URL=https://your-api-domain.com
```

### CORS Configuration

The proxy API is configured with CORS support for the dashboard:

```json
{
  "Security": {
    "AllowedOrigins": [
      "http://localhost:3000",
      "https://localhost:3000",
      "http://127.0.0.1:3000",
      "https://127.0.0.1:3000"
    ]
  }
}
```

### Troubleshooting

If you encounter issues:

1. **Verify port 8080 is available** (Keycloak)
2. **Verify port 5050 is available** (Proxy API)
3. **Check if services are running on correct paths**
4. **Use mock data mode for UI development**
5. **Check browser console for CORS errors**
6. **Verify CORS configuration in appsettings.json**

### Alternative Configuration

If you need separate ports:

```env
# Separate ports
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_API_BASE_URL=http://localhost:8081
```

Then update the API paths back to:
- Router API: `http://localhost:8081/v1`
- Aggregator API: `http://localhost:8081/aggregator/v1`


