# Routes API Integration Guide

This guide explains how the Routes API has been integrated into the dashboard.

## 🔧 Configuration

### Environment Variables

The Routes API integration uses the same configuration as the ClickStream API:

```bash
# API Configuration
REACT_APP_PROXY_API_URL=http://localhost:5050

# Development Settings
REACT_APP_USE_MOCK_DATA=true

# Keycloak Configuration
REACT_APP_KEYCLOAK_URL=http://localhost:8080
REACT_APP_KEYCLOAK_REALM=shortas-dev
REACT_APP_KEYCLOAK_CLIENT_ID=shortas-api
```

### API Endpoints

The dashboard now connects to the following Routes API endpoints:

- **Get All Routes**: `GET /api/v1/routes`
- **Get Route**: `GET /api/v1/routes/{domain}/{path}`
- **Create Route**: `POST /api/v1/routes`
- **Update Route**: `PUT /api/v1/routes/{domain}/{path}`
- **Delete Route**: `DELETE /api/v1/routes/{domain}/{path}`

## 📊 Features

### Route Management

The updated Routes component now provides:

1. **Route Listing**: View all routes with filtering and search
2. **Route Creation**: Create new routes with comprehensive form
3. **Route Editing**: Edit existing routes with full form
4. **Route Deletion**: Delete routes with confirmation
5. **Route Actions**: Copy URLs, open destinations, etc.

### Data Structure

The component works with the following data structure:

```typescript
interface RouteDto {
  switch: string;
  link: string;
  dest: string;
  dest_format: string;
  code: number;
  ttl: number;
  status: string;
  terminal: string;
  properties: {
    route_id: string;
    domain_id: string;
    owner_id: string;
    scripts: string[];
    tags: string[];
    custom: Record<string, any>;
    opengraph: boolean;
    allow_debug: boolean;
  };
}
```

### Route Form Fields

The route creation/editing form includes:

- **Switch**: Route switch (main, secondary, backup)
- **Short URL Path**: The path part of the short URL
- **Destination URL**: The target URL to redirect to
- **Redirect Code**: HTTP status code (301, 302, 307, 308)
- **TTL**: Time to live in seconds
- **Status**: Route status (Active, Inactive, Paused)
- **Destination Format**: Protocol (HTTP, HTTPS, SFTP, FTP)
- **Terminal**: Route type (External, Internal, API)
- **Domain ID**: Associated domain
- **Tags**: Comma-separated tags
- **OpenGraph**: Enable OpenGraph metadata
- **Debug**: Allow debug mode

## 🔄 API Integration

### Service Layer

The `apiService.routes` provides:

```typescript
// Get all routes
const routes = await apiService.routes.list();

// Get specific route
const route = await apiService.routes.get('domain', 'path', 'switch');

// Create new route
const newRoute = await apiService.routes.create(routeData);

// Update existing route
const updatedRoute = await apiService.routes.update('domain', 'path', routeData, 'switch');

// Delete route
await apiService.routes.delete('domain', 'path', 'switch');
```

### Authentication

The API service automatically handles:

- JWT token management
- Token refresh
- Authentication headers
- Error handling for 401 responses

### Mock Data

When `REACT_APP_USE_MOCK_DATA=true`, the service provides:

- 2 mock routes with realistic data
- Simulated API delays
- Full CRUD operations for testing

## 🚀 Usage

### Starting the Dashboard

1. **Install dependencies**:
   ```bash
   cd /home/max/dev/shortas/ui/dashboard
   npm install
   ```

2. **Configure environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start the dashboard**:
   ```bash
   npm start
   ```

### Testing with Mock Data

1. Set `REACT_APP_USE_MOCK_DATA=true` in your `.env` file
2. The dashboard will use mock data instead of real API calls
3. You'll see sample routes for testing

### Testing with Real API

1. Set `REACT_APP_USE_MOCK_DATA=false` in your `.env` file
2. Ensure the Routes API is running on port 5050
3. Ensure Keycloak is configured with the `shortas-dev` realm
4. The dashboard will make real API calls

## 🔧 Development

### Adding New Features

To add new Routes features:

1. **Update the API service** in `src/services/api.ts`
2. **Add new types** to the `RouteDto` interface
3. **Update the component** to display new data
4. **Add new form fields** in `RouteEditModal.tsx`

### Customizing the Display

The Routes component can be customized by:

1. **Modifying the table columns** in the JSX
2. **Adding new filter options** in the filters section
3. **Updating the form fields** in `RouteEditModal.tsx`
4. **Changing the route actions** in the table

## 🐛 Troubleshooting

### Common Issues

1. **401 Unauthorized**: Check Keycloak configuration and token
2. **No routes displayed**: Verify API is running and accessible
3. **Mock data not working**: Check `REACT_APP_USE_MOCK_DATA` setting
4. **Form validation errors**: Check required fields and data types
5. **CORS errors**: Ensure API allows requests from dashboard origin

### Debug Mode

Enable debug logging by setting:

```bash
REACT_APP_DEBUG=true
```

This will log API calls and responses to the console.

## 📈 Performance

### Optimization Tips

1. **Pagination**: Consider implementing pagination for large route lists
2. **Caching**: Implement client-side caching for frequently accessed routes
3. **Debouncing**: Add debouncing to search inputs
4. **Lazy Loading**: Load route details on demand

### Monitoring

The component includes:

- Loading states during API calls
- Error handling with retry options
- Form validation
- Success/error feedback

## 🔗 Related Files

- `src/components/Routes.tsx` - Main routes component
- `src/components/RouteEditModal.tsx` - Route creation/editing modal
- `src/services/api.ts` - API service layer
- `src/config/development.ts` - Development configuration
- `src/config/keycloak.ts` - Keycloak configuration

## 📚 API Documentation

For complete API documentation, see:
- [Routes API Guide](../api/README.md)
- [Keycloak Setup Guide](../api/KEYCLOAK_SETUP.md)
- [API Testing Script](../api/test-auth.sh)

## 🎯 Features Overview

### Route Management
- ✅ List all routes with search and filtering
- ✅ Create new routes with comprehensive form
- ✅ Edit existing routes
- ✅ Delete routes with confirmation
- ✅ Copy URLs to clipboard
- ✅ Open destination URLs

### Form Features
- ✅ Switch selection (main, secondary, backup)
- ✅ URL path validation
- ✅ Destination URL validation
- ✅ HTTP status code selection
- ✅ TTL configuration
- ✅ Status management
- ✅ Protocol selection
- ✅ Terminal type selection
- ✅ Domain association
- ✅ Tag management
- ✅ Feature toggles (OpenGraph, Debug)

### UI Features
- ✅ Responsive table layout
- ✅ Search functionality
- ✅ Status filtering
- ✅ Action buttons
- ✅ Loading states
- ✅ Error handling
- ✅ Empty state messages
- ✅ Modal forms
- ✅ Form validation
- ✅ Success feedback
