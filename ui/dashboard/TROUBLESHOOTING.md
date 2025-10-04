# Troubleshooting Guide

## Browser Authentication Dialog Issue

If you're seeing a browser standard login dialog instead of the custom login UI, here are the solutions:

### Quick Fix: Use Mock Data

The easiest way to test the UI without needing Keycloak/API servers running:

1. **Create `.env.local` file:**
   ```bash
   cp env.example .env.local
   ```

2. **Enable mock data in `.env.local`:**
   ```env
   REACT_APP_USE_MOCK_DATA=true
   ```

3. **Restart the app:**
   ```bash
   npm start
   ```

This will use mock data instead of making API calls, so you can see the full UI working without authentication issues.

### Root Cause Analysis

The browser authentication dialog appears when:

1. **API Server Not Running**: The API endpoints return 401 with `WWW-Authenticate` header
2. **Keycloak Server Not Running**: Keycloak initialization fails
3. **CORS Issues**: Browser blocks the requests
4. **Network Authentication**: Corporate firewall or proxy requires authentication

### Solutions

#### 1. Check Keycloak Server
```bash
# Make sure Keycloak is running on the configured URL
curl http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration
```

#### 2. Check API Server
```bash
# Make sure API server is running
curl http://localhost:8080/health
```

#### 3. Update Environment Variables
Edit `.env.local`:
```env
# Update these to match your running services
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_API_BASE_URL=http://localhost:8080
REACT_APP_USE_MOCK_DATA=false
```

#### 4. Check Browser Console
Open Developer Tools (F12) and check for:
- CORS errors
- Network failures
- Keycloak initialization errors

### Development Modes

#### Mock Data Mode (Recommended for UI Development)
```env
REACT_APP_USE_MOCK_DATA=true
```
- ✅ No external dependencies
- ✅ Fast development
- ✅ Full UI functionality
- ❌ No real authentication

#### Real API Mode
```env
REACT_APP_USE_MOCK_DATA=false
```
- ✅ Real authentication
- ✅ Real data
- ❌ Requires all services running
- ❌ More complex setup

### Step-by-Step Setup

#### For UI Development (Mock Mode)
1. `cd ui/dashboard`
2. `npm install`
3. `cp env.example .env.local`
4. Edit `.env.local` and set `REACT_APP_USE_MOCK_DATA=true`
5. `npm start`

#### For Full Integration (Real Mode)
1. Start Keycloak server
2. Start API services
3. Configure Keycloak client
4. Set `REACT_APP_USE_MOCK_DATA=false`
5. `npm start`

### Common Error Messages

#### "Keycloak not initialized"
- Keycloak server is not running
- Wrong Keycloak URL in environment variables
- Network connectivity issues

#### "User not authenticated"
- Normal behavior when not logged in
- Should redirect to `/login` page

#### "No authentication token available"
- Token expired
- Keycloak session ended
- Should redirect to login

### Browser Dialog Prevention

The updated code now prevents browser auth dialogs by:
1. Checking authentication status before making API requests
2. Using mock data when configured
3. Proper error handling without triggering browser auth
4. Better request interceptor logic

### Getting Help

If you're still having issues:

1. **Check the Development Banner**: Look for the orange banner at the top showing development mode
2. **Check Browser Console**: Look for error messages
3. **Try Mock Mode First**: Get the UI working with mock data
4. **Verify Services**: Ensure Keycloak and API servers are running

### Production Deployment

For production, ensure:
- All environment variables point to production services
- Keycloak client is properly configured
- CORS is configured correctly
- SSL certificates are valid
