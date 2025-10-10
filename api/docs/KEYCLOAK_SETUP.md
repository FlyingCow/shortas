# Keycloak Setup for Shortas Proxy API

This guide explains how to configure Keycloak for the Shortas Proxy API authentication.

## 🔧 Keycloak Server Configuration

### 1. Keycloak Server Details
- **URL**: `http://localhost:8080`
- **Realm**: `shortas-dev`
- **Client**: `shortas-api`

### 2. Create Realm (if not exists)

1. **Access Keycloak Admin Console**
   ```
   http://localhost:8080/admin
   ```

2. **Create Realm**
   - Click "Add realm"
   - Name: `shortas-dev`
   - Enabled: `ON`
   - Click "Create"

### 3. Create Client

1. **Navigate to Clients**
   - Go to `shortas-dev` realm
   - Click "Clients" → "Create"

2. **Client Configuration**
   ```
   Client ID: shortas-api
   Client Protocol: openid-connect
   Root URL: http://localhost:5050
   ```

3. **Client Settings**
   ```
   Access Type: confidential
   Standard Flow Enabled: ON
   Direct Access Grants Enabled: ON
   Service Accounts Enabled: ON
   Authorization Enabled: ON
   ```

4. **Valid Redirect URIs**
   ```
   http://localhost:5050/*
   http://localhost:3000/*
   ```

5. **Web Origins**
   ```
   http://localhost:5050
   http://localhost:3000
   ```

6. **Save and Note the Client Secret**
   - Go to "Credentials" tab
   - Copy the "Secret" value

### 4. Create Roles

1. **Navigate to Roles**
   - Go to `shortas-dev` realm
   - Click "Roles" → "Add Role"

2. **Create the following roles:**
   ```
   - read:routes
   - write:routes
   - delete:routes
   - read:certificates
   - write:certificates
   - delete:certificates
   - read:user_settings
   - write:user_settings
   - read:clickstream
   ```

### 5. Create Test User

1. **Navigate to Users**
   - Go to `shortas-dev` realm
   - Click "Users" → "Add user"

2. **User Configuration**
   ```
   Username: testuser
   Email: test@shortas.com
   First Name: Test
   Last Name: User
   Email Verified: ON
   Enabled: ON
   ```

3. **Set Password**
   - Go to "Credentials" tab
   - Set password: `testpassword`
   - Temporary: OFF

4. **Assign Roles**
   - Go to "Role Mappings" tab
   - Assign all created roles to the user

## 🔑 API Configuration

### 1. Update appsettings.json

```json
{
  "Keycloak": {
    "Authority": "http://localhost:8080/realms/shortas-dev",
    "Audience": "shortas-api",
    "ClientId": "shortas-api",
    "ClientSecret": "YOUR_CLIENT_SECRET_HERE",
    "RequireHttpsMetadata": false
  }
}
```

### 2. Environment Variables (Alternative)

```bash
export Keycloak__Authority="http://localhost:8080/realms/shortas-dev"
export Keycloak__Audience="shortas-api"
export Keycloak__ClientId="shortas-api"
export Keycloak__ClientSecret="YOUR_CLIENT_SECRET_HERE"
export Keycloak__RequireHttpsMetadata="false"
```

## 🧪 Testing Authentication

### 1. Get Access Token

```bash
curl -X POST http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=shortas-api" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "username=testuser" \
  -d "password=testpassword"
```

### 2. Use Token in API Calls

```bash
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  http://localhost:5050/api/health
```

### 3. Test Protected Endpoints

```bash
# Test routes endpoint
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  http://localhost:5050/api/v1/routes/example.com/test

# Test clickstream endpoint
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  http://localhost:5050/api/v1/clickstream
```

## 🔐 Swagger UI Authentication

1. **Open Swagger UI**: `http://localhost:5050/swagger/index.html`

2. **Click "Authorize" button**

3. **Enter JWT Token**:
   ```
   Bearer YOUR_ACCESS_TOKEN
   ```

4. **Test endpoints** using the "Try it out" buttons

## 🚨 Troubleshooting

### Common Issues

1. **401 Unauthorized**
   - Check if Keycloak is running on port 8080
   - Verify client secret is correct
   - Ensure user has proper roles assigned

2. **Token Expired**
   - Get a new token using the curl command above
   - Check token expiration time

3. **CORS Issues**
   - Ensure Web Origins are configured in Keycloak
   - Check AllowedOrigins in appsettings.json

4. **Client Not Found**
   - Verify client ID matches in Keycloak
   - Check if client is enabled

### Debug Logging

Add to appsettings.json for detailed authentication logs:

```json
{
  "Logging": {
    "LogLevel": {
      "Microsoft.AspNetCore.Authentication": "Debug",
      "Microsoft.AspNetCore.Authorization": "Debug"
    }
  }
}
```

## 📋 Quick Setup Script

```bash
#!/bin/bash

# Start Keycloak (if not running)
docker run -d --name keycloak \
  -p 8080:8080 \
  -e KEYCLOAK_ADMIN=admin \
  -e KEYCLOAK_ADMIN_PASSWORD=admin \
  quay.io/keycloak/keycloak:latest \
  start-dev

# Wait for Keycloak to start
sleep 30

# Create realm and client (requires Keycloak Admin CLI)
# This would need to be done through the web interface or CLI
```

## 🔗 Useful Links

- **Keycloak Admin Console**: http://localhost:8080/admin
- **API Swagger UI**: http://localhost:5050/swagger/index.html
- **API Health Check**: http://localhost:5050/api/health
- **Keycloak Token Endpoint**: http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/token
