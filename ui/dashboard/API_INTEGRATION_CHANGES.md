# Dashboard API Integration Updates

## Overview

The dashboard routes views have been updated to integrate with the new C# Proxy API endpoints with the following improvements:

- ✅ JWT authentication with Keycloak
- ✅ Pagination support (server-side)
- ✅ Server-side search and filtering
- ✅ Updated field naming (camelCase)
- ✅ Bulk operations support
- ✅ User-based route filtering

## API Endpoint Changes

### Routes API Endpoints

#### **List Routes** (with Pagination)
```
GET /api/v1/routes?page=1&pageSize=20&search=&status=
```

**Response:**
```json
{
  "data": [
    {
      "switch": "main",
      "link": "example.com/short",
      "dest": "https://example.com/destination",
      "destFormat": "Http",
      "code": 301,
      "ttl": 3600,
      "status": "Active",
      "terminal": "External",
      "properties": {
        "routeId": "route-123",
        "domainId": "example.com",
        "ownerId": "user-abc",
        "scripts": [],
        "tags": ["demo"],
        "custom": {},
        "opengraph": true,
        "allowDebug": false
      }
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "totalCount": 45,
    "totalPages": 3
  }
}
```

#### **Get Single Route**
```
GET /api/v1/routes/{domain}/{path}
```

#### **Create Route**
```
POST /api/v1/routes
```

**Request Body:**
```json
{
  "switch": "main",
  "link": "example.com/mylink",
  "dest": "https://destination.com",
  "destFormat": "Http",
  "code": 301,
  "ttl": 3600,
  "status": "Active",
  "terminal": "External",
  "properties": {
    "routeId": "route-id",
    "domainId": "example.com",
    "scripts": [],
    "tags": [],
    "custom": {},
    "opengraph": false,
    "allowDebug": false
  }
}
```

**Note:** `ownerId` is automatically set by the API from JWT token.

#### **Update Route**
```
PUT /api/v1/routes/{domain}/{path}
```

#### **Delete Route**
```
DELETE /api/v1/routes/{domain}/{path}
```

#### **Bulk Operations**
```
POST /api/v1/routes/bulk
PUT /api/v1/routes/bulk
DELETE /api/v1/routes/bulk
```

## Field Name Changes

### Old (snake_case) → New (camelCase)

| Old Field Name | New Field Name |
|----------------|----------------|
| `dest_format` | `destFormat` |
| `properties.route_id` | `properties.routeId` |
| `properties.domain_id` | `properties.domainId` |
| `properties.owner_id` | `properties.ownerId` |
| `properties.allow_debug` | `properties.allowDebug` |

## Component Updates

### **1. API Service (`src/services/api.ts`)**

**Updated Type Definitions:**
```typescript
export interface RouteDto {
  switch: string;
  link: string;
  dest: string;
  destFormat: string;  // Changed from dest_format
  code: number;
  ttl: number;
  status: string;
  terminal: string;
  properties?: {
    routeId: string;    // Changed from route_id
    domainId: string;   // Changed from domain_id
    ownerId: string;    // Changed from owner_id
    scripts: string[];
    tags: string[];
    custom: Record<string, any>;
    opengraph: boolean;
    allowDebug: boolean; // Changed from allow_debug
  };
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    totalCount: number;
    totalPages: number;
  };
}
```

**Updated API Methods:**
```typescript
routes: {
  // Now returns paginated response
  list: async (params?: {
    page?: number;
    pageSize?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<RouteDto>>

  // Removed 'switch' parameter
  get: async (domain: string, path: string)
  update: async (domain: string, path: string, route: Partial<RouteDto>)
  delete: async (domain: string, path: string)

  // New bulk operations
  bulkCreate: async (routes: Partial<RouteDto>[])
  bulkUpdate: async (routes: Partial<RouteDto>[])
  bulkDelete: async (routeIds: string[])
}
```

### **2. Routes Component (`src/components/RoutesUnified.tsx`)**

**New Features:**

1. **Pagination Controls**
   - Page navigation with Previous/Next buttons
   - Shows current page and total pages
   - Displays record count

2. **Server-Side Search**
   - Search button triggers API call
   - Enter key support
   - Search resets to page 1

3. **Server-Side Filtering**
   - Status filter dropdown
   - Filtering triggers immediate API call

4. **Helper Function**
   ```typescript
   // Parses domain and path from link field
   const parseLinkParts = (link: string): { domain: string; path: string } => {
     const parts = link.split('/');
     return {
       domain: parts[0] || '',
       path: parts.slice(1).join('/') || ''
     };
   };
   ```

### **3. Mock Data (`src/config/development.ts`)**

Updated to use camelCase field names and proper link format:

```typescript
export const mockRoutes = [
  {
    switch: 'main',
    link: 'example.com/example1',  // Format: domain/path
    dest: 'https://example.com/page1',
    destFormat: 'Http',  // camelCase
    code: 301,
    ttl: 3600,
    status: 'Active',
    terminal: 'External',
    properties: {
      routeId: 'route-1',      // camelCase
      domainId: 'example.com', // camelCase
      ownerId: 'user-1',       // camelCase
      scripts: [],
      tags: ['demo'],
      custom: {},
      opengraph: true,
      allowDebug: false,       // camelCase
    },
  },
  // ...
];
```

## Authentication Flow

1. **User logs in via Keycloak**
   - Dashboard redirects to Keycloak login page
   - User authenticates with credentials

2. **Keycloak issues JWT token**
   - Token includes `audience: "shortas-api"`
   - Token includes user claims (`sub`, `preferred_username`, `email`)

3. **API requests include JWT token**
   - Interceptor adds `Authorization: Bearer <token>` header
   - Token is auto-refreshed before expiration

4. **API validates token**
   - Validates signature with Keycloak
   - Validates audience matches "shortas-api"
   - Extracts `userId` from claims

5. **API filters routes by user**
   - Automatically applies `ownerId` filter
   - Users can only see/modify their own routes

## Environment Variables

### Dashboard `.env` file:
```bash
# API Configuration
REACT_APP_PROXY_API_URL=http://localhost:5050

# Mock Data (for development without backend)
REACT_APP_USE_MOCK_DATA=false

# Keycloak Configuration
REACT_APP_KEYCLOAK_URL=http://keycloak:8080
REACT_APP_KEYCLOAK_REALM=shortas-dev
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-dashboard
```

### API `appsettings.json`:
```json
{
  "Keycloak": {
    "Authority": "http://keycloak:8080/realms/shortas-dev",
    "Audience": "shortas-api",
    "ClientId": "shortas-api",
    "RequireHttpsMetadata": false
  },
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=shortas_dev_db;Username=shortas_user;Password=shortas_password;Port=5433"
  }
}
```

## Testing the Integration

### **1. With Mock Data (No Backend Required)**
```bash
cd ui/dashboard
echo "REACT_APP_USE_MOCK_DATA=true" > .env.local
npm start
```

### **2. With Real API**
```bash
# Start Keycloak
docker compose up keycloak

# Start PostgreSQL
docker compose up postgres

# Start API
cd api
dotnet run

# Start Dashboard
cd ui/dashboard
echo "REACT_APP_USE_MOCK_DATA=false" > .env.local
echo "REACT_APP_PROXY_API_URL=http://localhost:5050" >> .env.local
npm start
```

### **3. Test Flow**
1. Navigate to `http://localhost:3000`
2. Click "Login" - redirects to Keycloak
3. Enter credentials
4. Redirected back to dashboard
5. Navigate to "Routes" page
6. See paginated list of routes (filtered by current user)
7. Use search box to filter routes
8. Use status dropdown to filter by status
9. Click "Create Route" to add new route
10. Edit/Delete routes as needed

## API Response Examples

### **Success Response (List)**
```json
{
  "data": [
    {
      "switch": "main",
      "link": "example.com/summer-sale",
      "dest": "https://shop.example.com/sale",
      "destFormat": "Http",
      "code": 302,
      "ttl": 1800,
      "status": "Active",
      "terminal": "External",
      "properties": {
        "routeId": "abc-123",
        "domainId": "example.com",
        "ownerId": "user-xyz",
        "scripts": [],
        "tags": ["promo", "summer"],
        "custom": {},
        "opengraph": true,
        "allowDebug": false
      }
    }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "totalCount": 1,
    "totalPages": 1
  }
}
```

### **Error Response**
```json
{
  "error": "FORBIDDEN",
  "message": "You do not have permission to access this resource"
}
```

### **Validation Error**
```json
{
  "error": "VALIDATION_ERROR",
  "message": "Route validation failed: Link: Required field missing"
}
```

## Troubleshooting

### **401 Unauthorized**
- Check that JWT token is valid and not expired
- Verify Keycloak is running
- Check that token includes `audience: "shortas-api"`
- Verify API `Keycloak:Authority` matches Keycloak realm URL

### **403 Forbidden**
- User is trying to access/modify routes they don't own
- Check that route's `properties.ownerId` matches user's ID from JWT

### **Empty Routes List**
- User has no routes created yet
- Check that routes in DB have correct `ownerId`
- Verify JWT token contains user ID claim

### **CORS Errors**
- Check API `Security:AllowedOrigins` includes dashboard URL
- Verify dashboard URL is `http://localhost:3000`
- Check browser console for specific CORS error

## Migration Notes

If you have existing data with old field names:

1. **Database Migration** - No changes needed (DB uses same schema)
2. **API Layer** - Uses camelCase in JSON serialization
3. **Dashboard** - Updated to use camelCase
4. **Old Dashboard Code** - Will need to be updated or discarded

## Next Steps

- [ ] Implement route form/edit modal components
- [ ] Add toast notifications for success/error feedback
- [ ] Implement bulk operations UI
- [ ] Add route analytics integration
- [ ] Add certificate management views
- [ ] Add user settings management
