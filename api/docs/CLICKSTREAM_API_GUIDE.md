# ClickStream API Endpoint Guide

This guide explains how to use the ClickStream API endpoint in your dashboard application.

## 🔗 API Endpoints

### Base URL
```
http://localhost:5050/api/v1/clickstream
```

### Authentication
All endpoints require JWT authentication. Include the Bearer token in the Authorization header:
```
Authorization: Bearer YOUR_JWT_TOKEN
```

## 📊 Available Endpoints

### 1. Get All Click Stream Data
```http
GET /api/v1/clickstream
```

**Query Parameters:**
- `routeId` (optional): Filter by specific route ID
- `startDate` (optional): Filter from date (ISO 8601 format)
- `endDate` (optional): Filter to date (ISO 8601 format)

**Example:**
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "http://localhost:5050/api/v1/clickstream?startDate=2024-01-01&endDate=2024-01-31"
```

### 2. Get Click Stream Data for Specific Route
```http
GET /api/v1/clickstream/{routeId}
```

**Path Parameters:**
- `routeId`: The route ID to filter by

**Query Parameters:**
- `startDate` (optional): Filter from date
- `endDate` (optional): Filter to date

**Example:**
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "http://localhost:5050/api/v1/clickstream/route-123?startDate=2024-01-01"
```

### 3. Get Click Stream Statistics
```http
GET /api/v1/clickstream/stats
```

**Query Parameters:**
- `routeId` (optional): Filter by specific route ID
- `startDate` (optional): Filter from date
- `endDate` (optional): Filter to date

**Example:**
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "http://localhost:5050/api/v1/clickstream/stats?routeId=route-123"
```

## 📋 Response Data Structure

### ClickStream Data Response
```json
[
  {
    "id": "click-123",
    "ownerId": "user-456",
    "creatorId": "user-456",
    "routeId": "route-789",
    "workspaceId": "workspace-101",
    "created": "2024-01-15T10:30:00Z",
    "dest": "https://example.com/target",
    "ip": "192.168.1.100",
    "continent": "North America",
    "country": "United States",
    "location": "New York, NY",
    "osFamily": "Windows",
    "osVersion": "10",
    "userAgentFamily": "Chrome",
    "userAgentVersion": "120.0.0",
    "deviceBrand": "Dell",
    "deviceFamily": "Desktop",
    "deviceModel": "OptiPlex",
    "sessionFirst": "2024-01-15T10:00:00Z",
    "sessionClicks": 5,
    "isUnique": true,
    "isBot": false
  }
]
```

### Statistics Response
```json
{
  "totalClicks": 1250,
  "uniqueClicks": 980,
  "botClicks": 45,
  "topCountries": [
    { "country": "United States", "count": 450 },
    { "country": "Canada", "count": 200 }
  ],
  "topDevices": [
    { "device": "Desktop", "count": 600 },
    { "device": "Mobile", "count": 400 }
  ],
  "clickTrends": [
    { "date": "2024-01-01", "clicks": 50 },
    { "date": "2024-01-02", "clicks": 75 }
  ]
}
```

## 🔧 Dashboard Integration Examples

### JavaScript/Fetch API
```javascript
// Get clickstream data
async function getClickStreamData(token, routeId = null, startDate = null, endDate = null) {
  const params = new URLSearchParams();
  if (routeId) params.append('routeId', routeId);
  if (startDate) params.append('startDate', startDate);
  if (endDate) params.append('endDate', endDate);
  
  const response = await fetch(`http://localhost:5050/api/v1/clickstream?${params}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}

// Get statistics
async function getClickStreamStats(token, routeId = null) {
  const params = new URLSearchParams();
  if (routeId) params.append('routeId', routeId);
  
  const response = await fetch(`http://localhost:5050/api/v1/clickstream/stats?${params}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json'
    }
  });
  
  return await response.json();
}

// Usage example
const token = 'your-jwt-token';
const clickData = await getClickStreamData(token, 'route-123', '2024-01-01', '2024-01-31');
const stats = await getClickStreamStats(token, 'route-123');
```

### React Component Example
```jsx
import React, { useState, useEffect } from 'react';

const ClickStreamDashboard = ({ token }) => {
  const [clickData, setClickData] = useState([]);
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetchClickStreamData();
  }, [token]);

  const fetchClickStreamData = async () => {
    setLoading(true);
    try {
      const [dataResponse, statsResponse] = await Promise.all([
        fetch('http://localhost:5050/api/v1/clickstream', {
          headers: { 'Authorization': `Bearer ${token}` }
        }),
        fetch('http://localhost:5050/api/v1/clickstream/stats', {
          headers: { 'Authorization': `Bearer ${token}` }
        })
      ]);

      const data = await dataResponse.json();
      const statsData = await statsResponse.json();
      
      setClickData(data);
      setStats(statsData);
    } catch (error) {
      console.error('Error fetching clickstream data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      <h2>Click Stream Analytics</h2>
      {stats && (
        <div>
          <p>Total Clicks: {stats.totalClicks}</p>
          <p>Unique Clicks: {stats.uniqueClicks}</p>
        </div>
      )}
      <div>
        {clickData.map(click => (
          <div key={click.id}>
            <p>Route: {click.routeId}</p>
            <p>Country: {click.country}</p>
            <p>Device: {click.deviceFamily}</p>
            <p>Time: {new Date(click.created).toLocaleString()}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ClickStreamDashboard;
```

### Axios Example
```javascript
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:5050/api/v1',
  headers: {
    'Content-Type': 'application/json'
  }
});

// Add token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('jwt_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// API functions
export const clickStreamAPI = {
  getClickStream: (params = {}) => api.get('/clickstream', { params }),
  getClickStreamByRoute: (routeId, params = {}) => api.get(`/clickstream/${routeId}`, { params }),
  getStats: (params = {}) => api.get('/clickstream/stats', { params })
};

// Usage
const data = await clickStreamAPI.getClickStream({
  startDate: '2024-01-01',
  endDate: '2024-01-31'
});
```

## 🔐 Authentication Setup

### 1. Get JWT Token
```bash
# Using the test script
./test-auth.sh

# Or manually
curl -X POST http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=shortas-api" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "username=testuser" \
  -d "password=testpassword"
```

### 2. Store Token in Dashboard
```javascript
// Store token in localStorage
localStorage.setItem('jwt_token', token);

// Or use sessionStorage
sessionStorage.setItem('jwt_token', token);

// Or use a state management solution (Redux, Context, etc.)
```

## 🚨 Error Handling

### Common HTTP Status Codes
- `200 OK`: Success
- `400 Bad Request`: Invalid parameters
- `401 Unauthorized`: Missing or invalid token
- `403 Forbidden`: Insufficient permissions
- `404 Not Found`: Route not found
- `500 Internal Server Error`: Server error
- `502 Bad Gateway`: External service error
- `503 Service Unavailable`: Circuit breaker open

### Error Response Format
```json
{
  "error": "ERROR_CODE",
  "message": "Human readable error message"
}
```

### Error Handling Example
```javascript
try {
  const response = await fetch('http://localhost:5050/api/v1/clickstream', {
    headers: { 'Authorization': `Bearer ${token}` }
  });
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(`${error.error}: ${error.message}`);
  }
  
  const data = await response.json();
  return data;
} catch (error) {
  console.error('API Error:', error.message);
  // Handle error appropriately
}
```

## 📈 Dashboard Integration Tips

1. **Caching**: Implement caching for better performance
2. **Pagination**: Consider implementing pagination for large datasets
3. **Real-time Updates**: Use WebSockets or polling for real-time data
4. **Error Boundaries**: Implement error boundaries in React
5. **Loading States**: Show loading indicators during API calls
6. **Data Visualization**: Use libraries like Chart.js, D3.js, or Recharts

## 🔗 Related Documentation

- [Keycloak Setup Guide](KEYCLOAK_SETUP.md)
- [API Authentication Test](test-auth.sh)
- [Swagger UI](http://localhost:5050/swagger/index.html)
