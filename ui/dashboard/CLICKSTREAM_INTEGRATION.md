# Clickstream API Integration

## Overview

The clickstream view has been updated to integrate with the C# Proxy API clickstream endpoints instead of using mock data.

## API Changes

### **New API Service Methods** (`src/services/api.ts`)

Added three new clickstream methods:

```typescript
clickstream: {
  // Get all clickstream events with optional filters
  getAll: async (params?: {
    routeId?: string;
    startDate?: string;
    endDate?: string
  }): Promise<ClickStreamEvent[]>

  // Get clickstream events for a specific route
  getByRoute: async (
    routeId: string,
    params?: { startDate?: string; endDate?: string }
  ): Promise<ClickStreamEvent[]>

  // Get clickstream statistics
  getStats: async (params?: {
    routeId?: string;
    startDate?: string;
    endDate?: string
  }): Promise<ClickStreamStats>
}
```

### **New TypeScript Interfaces**

```typescript
export interface ClickStreamEvent {
  id: string;
  ownerId: string;
  creatorId: string;
  routeId: string;
  workspaceId: string;
  created: string;
  dest: string;
  ip: string;
  continent: string;
  country: string;
  location: string;
  osFamily: string;
  osVersion: string;
  userAgentFamily: string;
  userAgentVersion: string;
  deviceBrand: string;
  deviceFamily: string;
  deviceModel: string;
  sessionFirst: string;
  sessionClicks: number;
  isUnique: boolean;
  isBot: boolean;
}

export interface ClickStreamStats {
  totalClicks: number;
  uniqueClicks: number;
  botClicks: number;
  topCountries: Array<{ country: string; count: number }>;
  topDevices: Array<{ device: string; count: number }>;
  clickTrends: Array<{ date: string; clicks: number }>;
}
```

## Component Updates (`src/components/ClickstreamUnified.tsx`)

### **Key Changes:**

1. **Removed Mock Data Generator**
   - Replaced generateMockEvent() with real API calls
   - Added mapToClickEvent() to convert API responses to component format

2. **API Data Mapping**
   - Maps API ClickStreamEvent to component ClickEvent format
   - Parses city from location field
   - Combines OS family and version
   - Determines user type from isUnique flag

3. **Date Range Filtering**
   - Added date range selector: Last Hour, Last 24 Hours, Last 7 Days, Last 30 Days
   - Automatically calculates ISO 8601 date ranges for API calls
   - Refreshes data when date range changes

4. **Real-Time Updates**
   - Live mode refreshes data every 5 seconds from API
   - Replaced mock event generation with actual API polling
   - Pause/Resume functionality maintained

5. **Updated Stats**
   - Changed from uniqueUsers, avgResponseTime, errorRate
   - To: uniqueClicks, botClicks, filteredEvents
   - Stats now fetched from API /clickstream/stats endpoint

6. **Updated Table Columns**
   - Time (timestamp)
   - Route ID
   - Destination URL
   - Location (city, country)
   - Device
   - Browser / OS
   - IP Address
   - Type (New/Returning + Bot indicator)

7. **Filter Updates**
   - Removed "Status" filter
   - Added "Route ID" text input filter
   - Kept "Device" and "Search" filters
   - Search now includes routeId, url, city, and country

8. **Bot Detection**
   - Bot clicks are visually distinguished with CSS class bot-event
   - Bot badge shown in the "Type" column
   - Bot clicks counted in statistics

## API Endpoints Used

### **GET /api/v1/clickstream**
Fetches all clickstream events with optional filters.

**Query Parameters:**
- routeId (optional): Filter by route ID
- startDate (optional): ISO 8601 start date
- endDate (optional): ISO 8601 end date

**Response:** Array of ClickStreamEvent objects

### **GET /api/v1/clickstream/{routeId}**
Fetches clickstream events for a specific route.

### **GET /api/v1/clickstream/stats**
Fetches clickstream statistics.

## Authentication

All clickstream endpoints require JWT authentication via Authorization header.
The API service automatically includes the token from Keycloak.

## Testing

### **With Real API**
```bash
# Start services
docker compose up keycloak postgres
cd click-aggregator-api && cargo run
cd api && dotnet run
cd ui/dashboard && npm start
```

Navigate to http://localhost:3000/clickstream

## Performance Considerations

1. **Polling Interval**: Live mode refreshes every 5 seconds
2. **Parallel Requests**: Events and stats fetched in parallel using Promise.all()
3. **Client-Side Filtering**: Device and search filters applied client-side
4. **Date Range Filtering**: Server-side filtering reduces data transfer
