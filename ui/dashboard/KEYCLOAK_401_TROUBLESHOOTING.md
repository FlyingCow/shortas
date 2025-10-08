# Keycloak 401 Token Error Troubleshooting

## Error Description
Browser shows 401 error when accessing `/token` URL, typically:
```
GET http://localhost:8080/realms/shortas-dev/protocol/openid-connect/token
Response: 401 Unauthorized
```

## Root Causes & Solutions

### 1. Keycloak Server Not Running
**Check if Keycloak is running:**
```bash
curl http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration
```

**Expected Response:** JSON with Keycloak configuration
**If 404/Connection Error:** Keycloak server is not running

**Solution:**
- Start your Keycloak server on port 8080
- Or use mock data mode: `REACT_APP_USE_MOCK_DATA=true`

### 2. Realm 'shortas-dev' Doesn't Exist
**Check available realms:**
```bash
curl http://localhost:8080/realms/
```

**Solution:**
- Create the `shortas-dev` realm in Keycloak admin console
- Or update the realm name in configuration
- Or use mock data mode

### 3. Client 'shortas-dashboard' Not Configured
**Check if client exists in Keycloak admin console:**
1. Go to `http://localhost:8080/admin`
2. Login to admin console
3. Select `shortas-dev` realm
4. Go to Clients → Check for `shortas-dashboard`

**Solution:**
- Create the client in Keycloak
- Or use mock data mode

### 4. Incorrect Client Configuration
**Required client settings:**
- **Client ID:** `shortas-dashboard`
- **Client Type:** `OpenID Connect`
- **Client authentication:** `Off` (public client)
- **Valid redirect URIs:** `http://localhost:3000/*`
- **Valid post logout redirect URIs:** `http://localhost:3000/login`
- **Web origins:** `http://localhost:3000`

## Quick Fix: Use Mock Data Mode

The fastest solution is to enable mock data mode, which bypasses Keycloak entirely:

### Step 1: Create .env.local
```bash
cd ui/dashboard
cp env.example .env.local
```

### Step 2: Enable Mock Data
Edit `.env.local`:
```env
# Keycloak Configuration (not used in mock mode)
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-dashboard

# API Configuration (not used in mock mode)
REACT_APP_API_BASE_URL=http://localhost:8080

# Development Options - ENABLE THIS
REACT_APP_USE_MOCK_DATA=true

# Environment
NODE_ENV=development
```

### Step 3: Restart App
```bash
npm start
```

**Result:** App will work without any Keycloak server, using mock data for everything.

## Full Keycloak Setup (If You Want Real Auth)

### Step 1: Install & Start Keycloak
```bash
# Download Keycloak
wget https://github.com/keycloak/keycloak/releases/download/23.0.3/keycloak-23.0.3.zip
unzip keycloak-23.0.3.zip
cd keycloak-23.0.3

# Start Keycloak on port 8080
bin/kc.sh start-dev --http-port=8080
```

### Step 2: Create Admin User
First time setup will prompt for admin credentials.

### Step 3: Create Realm
1. Go to `http://localhost:8080/admin`
2. Login with admin credentials
3. Click "Create Realm"
4. Name: `shortas-dev`
5. Click "Create"

### Step 4: Create Client
1. In `shortas-dev` realm, go to "Clients"
2. Click "Create client"
3. **Client ID:** `shortas-dashboard`
4. **Client Type:** `OpenID Connect`
5. Click "Next"
6. **Client authentication:** `Off`
7. Click "Next"
8. **Valid redirect URIs:** `http://localhost:3000/*`
9. **Valid post logout redirect URIs:** `http://localhost:3000/login`
10. **Web origins:** `http://localhost:3000`
11. Click "Save"

### Step 5: Test Configuration
```bash
# Test realm endpoint
curl http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration

# Should return JSON with endpoints including:
# "token_endpoint": "http://localhost:8080/realms/shortas-dev/protocol/openid-connect/token"
```

### Step 6: Update App Configuration
Edit `.env.local`:
```env
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-dashboard
REACT_APP_API_BASE_URL=http://localhost:8080
REACT_APP_USE_MOCK_DATA=false  # Use real Keycloak
```

## Debugging Steps

### 1. Check Browser Network Tab
1. Open Developer Tools (F12)
2. Go to Network tab
3. Refresh the page
4. Look for failed requests to Keycloak URLs

### 2. Check Console Logs
Look for these debug messages:
```
[Shortas Debug] Creating new Keycloak instance
[Shortas Debug] Starting Keycloak initialization
[Shortas Error] Keycloak initialization failed
```

### 3. Test Keycloak Endpoints Manually
```bash
# Test if Keycloak is running
curl -I http://localhost:8080

# Test realm configuration
curl http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration

# Test admin console access
curl -I http://localhost:8080/admin
```

## Common Error Messages & Solutions

### "Failed to fetch"
- **Cause:** Keycloak server not running
- **Solution:** Start Keycloak or use mock data mode

### "Realm does not exist"
- **Cause:** `shortas-dev` realm not created
- **Solution:** Create realm or use mock data mode

### "Client not found"
- **Cause:** `shortas-dashboard` client not configured
- **Solution:** Create client or use mock data mode

### "Invalid redirect URI"
- **Cause:** Client redirect URIs not configured correctly
- **Solution:** Add `http://localhost:3000/*` to valid redirect URIs

## Recommended Development Workflow

### For UI Development
```env
REACT_APP_USE_MOCK_DATA=true
```
- ✅ No external dependencies
- ✅ Fast development
- ✅ Full UI functionality
- ✅ No 401 errors

### For Integration Testing
```env
REACT_APP_USE_MOCK_DATA=false
```
- ✅ Real authentication flow
- ✅ Actual Keycloak integration
- ❌ Requires Keycloak server setup
- ❌ More complex troubleshooting

## Quick Status Check

Run this command to check your setup:
```bash
echo "=== Keycloak Status Check ==="
echo "1. Testing Keycloak server..."
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080 && echo " - Server: OK" || echo " - Server: FAILED"

echo "2. Testing realm..."
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/realms/shortas-dev/.well-known/openid_configuration && echo " - Realm: OK" || echo " - Realm: FAILED"

echo "3. Current configuration:"
echo "   REACT_APP_USE_MOCK_DATA=${REACT_APP_USE_MOCK_DATA:-not set}"
```

If any tests fail, use mock data mode for development.

