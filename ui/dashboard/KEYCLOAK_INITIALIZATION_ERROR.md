# Keycloak Initialization Error Fix

## Error Message
```
Keycloak initialization failed: Error: A 'Keycloak' instance can only be initialized once.
```

## Root Cause
This error occurs when the Keycloak instance is being initialized multiple times, typically due to:

1. **React StrictMode**: In development, React StrictMode intentionally double-invokes functions to detect side effects
2. **Component Re-renders**: Multiple useEffect calls during component lifecycle
3. **Hot Reloading**: Development server reloads causing re-initialization

## Fixes Applied

### 1. Singleton Pattern
```typescript
// Before: Direct instance creation
const keycloak = new Keycloak(keycloakConfig);

// After: Singleton pattern with guard
let keycloakInstance: Keycloak | null = null;
const getKeycloakInstance = (): Keycloak => {
  if (!keycloakInstance) {
    keycloakInstance = new Keycloak(keycloakConfig);
  }
  return keycloakInstance;
};
```

### 2. Initialization Guard
```typescript
let initializationPromise: Promise<boolean> | null = null;
let isKeycloakInitialized = false;

export const initializeKeycloak = async (options: any): Promise<boolean> => {
  // Return cached result if already initialized
  if (isKeycloakInitialized) {
    return keycloak.authenticated || false;
  }
  
  // Wait for ongoing initialization
  if (initializationPromise) {
    return initializationPromise;
  }
  
  // Start new initialization
  initializationPromise = keycloak.init(options);
  return initializationPromise;
};
```

### 3. Mock Data Mode
```typescript
// Skip Keycloak entirely in mock mode
if (useMockData) {
  setState({
    keycloakInitialized: true,
    authenticated: true, // Mock authenticated state
    loading: false,
    error: null,
  });
  return;
}
```

### 4. Removed React StrictMode
```typescript
// Before: StrictMode causes double initialization
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// After: Direct rendering
root.render(<App />);
```

## Testing the Fix

### Option 1: Use Mock Data (Recommended)
```env
# In .env.local
REACT_APP_USE_MOCK_DATA=true
```
This completely bypasses Keycloak initialization.

### Option 2: Real Keycloak
```env
# In .env.local
REACT_APP_USE_MOCK_DATA=false
REACT_APP_KEYCLOAK_URL=http://localhost:8080
```
Uses the protected initialization with guards.

## Debug Information

The fix includes debug logging to help track initialization:

```javascript
[Shortas Debug] Creating new Keycloak instance
[Shortas Debug] Starting Keycloak initialization
[Shortas Debug] Keycloak initialization successful
```

## Verification

After applying the fix, you should see:
- ✅ No "can only be initialized once" error
- ✅ Single Keycloak instance creation
- ✅ Proper initialization flow
- ✅ Debug logs showing controlled initialization

## Alternative Solutions

If you still encounter issues:

1. **Clear browser cache and localStorage**
2. **Restart the development server**
3. **Check for multiple App component mounts**
4. **Use mock data mode for development**

## Prevention

To prevent this error in the future:
- Always use the `initializeKeycloak()` function instead of `keycloak.init()` directly
- Don't create multiple Keycloak instances
- Be careful with React StrictMode in development
- Use mock data mode when Keycloak server isn't available








