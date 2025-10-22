# User Initialization Flow

## Overview

This document describes the user initialization flow that automatically sets up new users with a default workspace and user settings when they first authenticate.

## Architecture

### Backend Components

#### 1. UserController (`api/Presentation/Controllers/UserController.cs`)

**Endpoint:** `POST /api/v1/user/initialize`

**Authentication:** Required (JWT Bearer token)

**Description:** Initializes a new user by creating:
- A default workspace named "My Workspace"
- Default user settings with "Active" status

**Behavior:**
- **Idempotent**: Can be called multiple times safely
- If workspace exists: Returns existing workspace
- If settings exist: Returns existing settings
- If neither exist: Creates both
- If only one exists: Creates the missing one

**Response:**
```json
{
  "workspace": {
    "id": "guid",
    "name": "My Workspace",
    "description": "Default workspace for organizing your routes",
    "type": "User",
    "createdAt": "2025-10-21T...",
    "updatedAt": "2025-10-21T...",
    "userRole": "Owner"
  },
  "userSettings": {
    "email": "user@example.com",
    "status": "Active",
    "debug": false,
    "overflow": false,
    "skipTracking": [],
    "allowedRequestParams": [],
    "allowedDestinationParams": []
  },
  "message": "User initialization completed successfully"
}
```

#### 2. Services Used

- **IWorkspaceService**: Manages workspace creation and access
- **IUserSettingsService**: Manages user settings in PostgreSQL and syncs to click-router-api via outbox pattern

### Frontend Integration

#### 1. API Service (`ui/dashboard/src/services/api.ts`)

Added `user.initialize()` method:

```typescript
user: {
  initialize: async (): Promise<InitializationResponse> => {
    const response = await routerApi.post('/user/initialize');
    return response.data;
  },
}
```

#### 2. App Component (`ui/dashboard/src/App.tsx`)

Calls initialization automatically after successful Keycloak authentication:

```typescript
if (authenticated) {
  // Set up token refresh
  setInterval(() => {
    keycloak.updateToken(70).catch(() => {
      console.error('Failed to refresh token');
    });
  }, 60000);

  // Initialize user (create default workspace and settings if needed)
  try {
    console.log('Initializing user account...');
    const initResponse = await apiService.user.initialize();
    console.log('User initialization completed:', initResponse.message);
  } catch (error) {
    // Don't block the app if initialization fails
    console.error('User initialization failed (non-critical):', error);
  }
}
```

## User Flow

### First-Time User Registration

```
1. User registers in Keycloak
   ↓
2. User logs into dashboard
   ↓
3. Keycloak authentication succeeds
   ↓
4. Dashboard calls /api/v1/user/initialize
   ↓
5. Backend creates:
   - Default workspace (with user as Owner)
   - Default user settings (Active status)
   ↓
6. Backend returns initialization response
   ↓
7. Dashboard is ready to use
```

### Returning User Login

```
1. User logs into dashboard
   ↓
2. Keycloak authentication succeeds
   ↓
3. Dashboard calls /api/v1/user/initialize
   ↓
4. Backend detects existing workspace & settings
   ↓
5. Backend returns existing data
   ↓
6. Dashboard is ready to use
```

## Key Features

### 1. Idempotent Design
- Can be called multiple times without side effects
- Safe to call on every login
- Returns existing resources if already created

### 2. Non-Blocking
- Initialization runs asynchronously after authentication
- Errors don't prevent dashboard from loading
- Logged for monitoring but not critical to app startup

### 3. Automatic
- No manual user action required
- Happens transparently after authentication
- Ensures all users have required resources

### 4. Workspace Management
- Default workspace created automatically
- User is assigned as "Owner" role
- Workspace is associated with user via UserWorkspace table
- Enables multi-workspace support in the future

### 5. User Settings
- Default settings created in PostgreSQL
- Synced to click-router-api via outbox pattern
- Ensures consistency across services

## Database Schema

### Workspaces Table
```sql
CREATE TABLE "Workspaces" (
    "Id" uuid PRIMARY KEY,
    "Name" varchar(255) NOT NULL,
    "Description" varchar(1000) DEFAULT '',
    "Type" varchar(50) DEFAULT 'User',
    "CreatedAt" timestamp NOT NULL,
    "UpdatedAt" timestamp NOT NULL
);
```

### UserWorkspaces Table (Join Table)
```sql
CREATE TABLE "UserWorkspaces" (
    "Id" uuid PRIMARY KEY,
    "UserId" varchar(255) NOT NULL,  -- Keycloak user ID
    "WorkspaceId" uuid NOT NULL,
    "Role" varchar(50) DEFAULT 'Member',  -- Owner, Admin, Member
    "JoinedAt" timestamp NOT NULL,
    FOREIGN KEY ("WorkspaceId") REFERENCES "Workspaces"("Id") ON DELETE CASCADE
);
```

### UserSettings Table
```sql
CREATE TABLE "UserSettings" (
    "Id" uuid PRIMARY KEY,
    "Email" varchar(255) UNIQUE NOT NULL,
    "Status" text NOT NULL,
    "Debug" boolean NOT NULL,
    "Overflow" boolean NOT NULL,
    "SkipTrackingJson" jsonb DEFAULT '[]',
    "AllowedRequestParamsJson" jsonb DEFAULT '[]',
    "AllowedDestinationParamsJson" jsonb DEFAULT '[]'
);
```

## Error Handling

### Backend Errors
- **Missing userId**: Returns 400 Bad Request
- **Database errors**: Returns 500 Internal Server Error
- **Service failures**: Logged and returned as 500

### Frontend Errors
- **Network errors**: Caught and logged
- **API errors**: Caught and logged
- **App continues**: Dashboard loads even if initialization fails

## Configuration

### Environment Variables

**Backend (API)**
- Configured via `appsettings.json`
- Uses existing database connection
- No additional configuration needed

**Frontend (Dashboard)**
- Uses existing Keycloak configuration
- Uses existing API URL from `REACT_APP_PROXY_API_URL`
- No additional configuration needed

## Testing

### Manual Testing

1. **New User Test**
   ```bash
   # Register new user in Keycloak
   # Login to dashboard
   # Check browser console for:
   # "Initializing user account..."
   # "User initialization completed: User initialization completed successfully"
   
   # Verify in database:
   SELECT * FROM "Workspaces" WHERE "Type" = 'User';
   SELECT * FROM "UserWorkspaces";
   SELECT * FROM "UserSettings";
   ```

2. **Existing User Test**
   ```bash
   # Login with existing user
   # Check browser console for:
   # "Initializing user account..."
   # "User initialization completed: User initialization completed successfully"
   
   # Verify existing data is returned (not duplicated)
   ```

3. **Error Recovery Test**
   ```bash
   # Stop API server
   # Login to dashboard
   # Check browser console for error message
   # Verify dashboard still loads and is functional
   ```

### API Testing

```bash
# Test with cURL
curl -X POST http://localhost:5050/api/v1/user/initialize \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"

# Expected Response (200 OK)
{
  "workspace": { ... },
  "userSettings": { ... },
  "message": "User initialization completed successfully"
}
```

## Monitoring

### Logs to Monitor

**Backend:**
- `"Initializing user {UserId}"` - User initialization started
- `"User {UserId} already has workspace {WorkspaceId}"` - Existing workspace found
- `"Created default workspace {WorkspaceId} for user {UserId}"` - New workspace created
- `"User {UserId} already has settings"` - Existing settings found
- `"Created default settings for user {UserId}"` - New settings created
- `"Failed to create workspace for user {UserId}: {Error}"` - Workspace creation failed
- `"Failed to create user settings for user {UserId}: {Error}"` - Settings creation failed

**Frontend:**
- `"Initializing user account..."` - Initialization started
- `"User initialization completed: ..."` - Initialization succeeded
- `"User initialization failed (non-critical): ..."` - Initialization failed (app continues)

## Future Enhancements

1. **Custom Workspace Names**: Allow users to customize workspace name during onboarding
2. **Workspace Templates**: Provide pre-configured workspace templates
3. **Onboarding Flow**: Add guided tour after first initialization
4. **Email Notifications**: Send welcome email after successful initialization
5. **Analytics**: Track initialization success rates and timing
6. **Multi-Workspace**: Allow users to create additional workspaces
7. **Team Workspaces**: Support shared workspaces with multiple users

## Security Considerations

1. **Authentication Required**: Endpoint requires valid JWT token
2. **User Isolation**: Users can only initialize their own account
3. **No Data Exposure**: Returns only user's own data
4. **Idempotent**: Safe to retry without security implications
5. **Rate Limiting**: Consider adding rate limiting to prevent abuse

## Troubleshooting

### User Has No Workspace

**Symptom:** User can't create routes, workspace dropdown is empty

**Solution:**
1. Call `/api/v1/user/initialize` manually via API
2. Check backend logs for initialization errors
3. Verify database connectivity
4. Check user ID in JWT token matches database records

### User Has No Settings

**Symptom:** Settings page shows error, click tracking doesn't work

**Solution:**
1. Call `/api/v1/user/initialize` manually via API
2. Check backend logs for settings creation errors
3. Verify outbox messages are being processed
4. Check click-router-api synchronization

### Initialization Called Multiple Times

**Symptom:** Multiple initialization logs on every page refresh

**Solution:**
- This is expected behavior
- Initialization is idempotent and safe to call multiple times
- No duplicate data will be created

## Related Documentation

- [WORKSPACE_API.md](./WORKSPACE_API.md) - Workspace management API
- [KEYCLOAK_SETUP.md](./KEYCLOAK_SETUP.md) - Authentication configuration
- [DATABASE_SETUP.md](./DATABASE_SETUP.md) - Database schema details

