# Login Flow Documentation

## Overview

The Shortas dashboard now supports a custom login UI that redirects to Keycloak for authentication. This provides a better user experience with branded login screens while maintaining secure authentication.

## Authentication Flow

### 1. Initial Access
```
User visits http://localhost:3000
↓
App checks authentication status
↓
If not authenticated → Redirect to /login
If authenticated → Show dashboard
```

### 2. Login Page
```
User sees custom login UI at /login
↓
User clicks "Continue with Keycloak SSO"
↓
Redirect to Keycloak login page
↓
User authenticates with Keycloak
↓
Redirect back to dashboard
```

### 3. Logout Flow
```
User clicks logout in dashboard
↓
Keycloak logout
↓
Redirect back to /login
```

## URL Structure

- `/` - Root (redirects to `/dashboard` if authenticated, `/login` if not)
- `/login` - Custom login UI (redirects to `/dashboard` if already authenticated)
- `/dashboard` - Main dashboard (protected, requires authentication)
- `/routes` - Routes management (protected)
- `/analytics` - Analytics page (protected)
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
- **Valid post logout redirect URIs**: `http://localhost:3000/login`
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

✅ **Professional Branding**: Custom login UI matches your brand
✅ **Better UX**: Users see your interface before authentication
✅ **Flexible Flow**: Can add features like "Remember me", forgot password, etc.
✅ **Security**: Still uses Keycloak for actual authentication
✅ **SEO Friendly**: Login page can be indexed and customized

## Development

### Testing the Flow

1. **Start the app**: `npm start`
2. **Visit**: `http://localhost:3000`
3. **Should redirect to**: `http://localhost:3000/login`
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

You can customize the login page by modifying:
- `src/components/Login.tsx` - Login component logic
- `src/components/Login.css` - Login page styling
- `src/App.tsx` - Routing configuration

## Production Deployment

For production, update the environment variables:

```env
REACT_APP_KEYCLOAK_URL=https://your-keycloak-server.com
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-dashboard
REACT_APP_API_BASE_URL=https://your-api-server.com
```

And update Keycloak client settings:
- **Valid redirect URIs**: `https://your-domain.com/*`
- **Valid post logout redirect URIs**: `https://your-domain.com/login`
- **Web origins**: `https://your-domain.com`
