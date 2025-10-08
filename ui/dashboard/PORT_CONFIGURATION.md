# Port Configuration

## Updated Configuration

Both Keycloak and API services are now configured to use port **8080**.

### Current Setup

```env
# Both services on same port with different paths
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_API_BASE_URL=http://localhost:8080
```

### Service Endpoints

#### Keycloak
- **Base URL**: `http://localhost:8080`
- **Realm**: `http://localhost:8080/realms/shortas-dev`
- **Auth**: `http://localhost:8080/realms/shortas-dev/protocol/openid-connect/auth`
- **Token**: `http://localhost:8080/realms/shortas-dev/protocol/openid-connect/token`

#### API Services
- **Router API**: `http://localhost:8080/api/v1`
- **Aggregator API**: `http://localhost:8080/api/aggregator/v1`

### Path Structure

```
http://localhost:8080/
├── realms/shortas-dev/          # Keycloak endpoints
├── api/v1/                      # Click Router API
└── api/aggregator/v1/           # Click Aggregator API
```

### Keycloak Client Configuration

Update your Keycloak client settings:

**Valid redirect URIs**:
- `http://localhost:3000/*`

**Valid post logout redirect URIs**:
- `http://localhost:3000/login`

**Web origins**:
- `http://localhost:3000`

### Testing Endpoints

#### Test Keycloak
```bash
curl http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration
```

#### Test Router API
```bash
curl http://localhost:8080/api/v1/health
```

#### Test Aggregator API
```bash
curl http://localhost:8080/api/aggregator/v1/health
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
REACT_APP_KEYCLOAK_URL=https://your-domain.com
REACT_APP_API_BASE_URL=https://your-domain.com
```

### Troubleshooting

If you encounter issues:

1. **Verify port 8080 is available**
2. **Check if services are running on correct paths**
3. **Use mock data mode for UI development**
4. **Check browser console for CORS errors**

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

