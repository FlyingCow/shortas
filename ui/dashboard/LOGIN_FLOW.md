# Login Flow Documentation

## Overview

The Shortas dashboard uses Keycloak's native login flow for authentication. This provides a secure, enterprise-grade authentication experience with single sign-on (SSO) capabilities.

## Authentication Flow

### 1. Initial Access
```
User visits http://localhost:3000
↓
App checks authentication status
↓
If not authenticated → Show logged out page
If authenticated → Show dashboard
```

### 2. Logged Out Page
```
User sees logged out page
↓
User can choose to:
- Sign in to dashboard (redirects to Keycloak)
- Visit landing page (external site)
↓
After Keycloak authentication → Redirect back to dashboard
```

### 3. Logout Flow
```
User clicks logout in dashboard
↓
Keycloak logout
↓
Redirect back to Keycloak login
```

## URL Structure

- `/` - Root (redirects to `/dashboard` if authenticated, `/logged-out` if not)
- `/logged-out` - Logged out page (public, shows login options)
- `/dashboard` - Main dashboard (protected, requires authentication)
- `/routes` - Routes management (protected)
- `/analytics` - Analytics page (protected)
- `/clickstream` - Clickstream analytics (protected)
- `/settings` - Settings page (protected)

## Configuration

### Keycloak Settings

The app now uses `check-sso` instead of `login-required`:

```typescript
// Before (forced immediate login)
onLoad: 'login-required'

// After (check if logged in, show custom UI if not)
onLoad: 'check-sso'
```

### Redirect URIs

Make sure your Keycloak client is configured with:
- **Valid redirect URIs**: `http://localhost:3000/*`
- **Valid post logout redirect URIs**: `http://localhost:3000/logged-out`
- **Web origins**: `http://localhost:3000`

## User Experience

### First Visit
1. User visits the app
2. Sees professional login screen with branding
3. Can interact with demo form (UI only)
4. Clicks "Continue with Keycloak SSO"
5. Redirected to Keycloak
6. After authentication, returns to dashboard

### Returning Users
1. User visits the app
2. If session is still valid, goes directly to dashboard
3. If session expired, redirected to login page

### Logout
1. User clicks logout
2. Keycloak session is terminated
3. Redirected to login page

## Benefits

✅ **User-Friendly Experience**: Clear logged out page with options to sign in or visit landing page
✅ **Enterprise Security**: Keycloak's native authentication with SSO support
✅ **Flexible Navigation**: Users can choose to authenticate or visit the main site
✅ **Professional UI**: Branded logged out page with clear call-to-actions
✅ **Standard Flow**: Follows OAuth2/OpenID Connect best practices

## Development

### Testing the Flow

1. **Start the app**: `npm start`
2. **Visit**: `http://localhost:3000`
3. **Should redirect to**: `http://localhost:3000/`
4. **Click "Continue with Keycloak SSO"**
5. **Should redirect to**: Keycloak login page
6. **After login**: Should return to dashboard

### Debugging

If you're having issues:

1. **Check Keycloak client configuration**
2. **Verify redirect URIs match exactly**
3. **Check browser console for errors**
4. **Ensure Keycloak server is running**

### Customization

You can customize the authentication flow by modifying:
- `src/config/keycloak.ts` - Keycloak configuration and initialization
- `src/App.tsx` - Authentication logic and routing
- `src/components/LoggedOut.tsx` - Logged out page UI and behavior
- Keycloak admin console - Login page themes and branding

### Environment Variables

- `REACT_APP_LANDING_URL` - URL for the landing page (default: https://shortas.com)
- `REACT_APP_USE_MOCK_DATA` - Enable mock data mode for development

## Production Deployment

For production, update the environment variables:

```env
REACT_APP_KEYCLOAK_URL=https://your-keycloak-server.com
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-dashboard
REACT_APP_PROXY_API_URL=https://your-api-server.com
REACT_APP_LANDING_URL=https://your-landing-page.com
```

And update Keycloak client settings:
- **Valid redirect URIs**: `https://your-domain.com/*`
- **Valid post logout redirect URIs**: `https://your-domain.com/logged-out`
- **Web origins**: `https://your-domain.com`


