# Routes API Implementation Summary

## ✅ Completed Features

### 1. **Outbox Pattern for Eventual Consistency**

Implemented the Outbox pattern to ensure reliable propagation of changes from the local PostgreSQL database to the Rust click-router-api.

**New Files:**
- `Domain/Entities/OutboxMessage.cs` - Outbox message entity
- `Domain/Interfaces/IOutboxRepository.cs` - Repository interface
- `Infrastructure/Repositories/OutboxRepository.cs` - Repository implementation
- `Infrastructure/BackgroundServices/OutboxProcessorService.cs` - Background service that processes outbox messages

**How it works:**
1. When a route is created/updated/deleted, the change is saved to local DB in a transaction
2. An outbox message is created in the same transaction
3. The background service polls the outbox table every 5 seconds
4. Messages are sent to click-router-api via HTTP
5. Failed messages are retried with exponential backoff (1min, 2min, 4min, 8min, 16min)
6. After 5 failed attempts, messages are marked as permanently failed

### 2. **EF Core-based Route Service**

Replaced the HTTP proxy service with a local EF Core service that:
- Stores routes in PostgreSQL
- Creates outbox events for all changes
- Uses database transactions for consistency
- Includes full validation

**Updated Files:**
- `Infrastructure/Services/EfRouteService.cs` - Complete rewrite with outbox support
- `Domain/Interfaces/IRouteService.cs` - Added `ListRoutesAsync` method

**Features:**
- CRUD operations on local database
- Bulk operations support
- Transaction management
- Comprehensive error handling
- Validation before persistence

### 3. **List/Query Routes Endpoint**

Added a new endpoint for listing routes with pagination and filtering.

**Endpoint:**
```
GET /api/v1/routes
Query parameters:
  - page: Page number (default: 1)
  - pageSize: Items per page (default: 20)
  - search: Search in link, dest, or switch fields
  - status: Filter by status
  - ownerId: Filter by owner ID
```

**Response:**
```json
{
  "data": [
    { ...route objects... }
  ],
  "pagination": {
    "page": 1,
    "pageSize": 20,
    "totalCount": 150,
    "totalPages": 8
  }
}
```

**Updated Files:**
- `Presentation/Controllers/RoutesController.cs` - Added `ListRoutes` method

### 4. **Bug Fixes**

Fixed the bulk delete bug where the HTTP DELETE request wasn't sending the request body.

**Fixed Files:**
- `Application/Services/RouteService.cs` - Line 352-358, now uses `HttpRequestMessage` for DELETE with body

### 5. **Database Schema**

Created complete EF Core data model with:
- Routes table
- RouteProperties table (with JSON columns)
- Certificates table
- UserSettings table
- OutboxMessages table (new)

**Updated Files:**
- `Infrastructure/Data/ApplicationDbContext.cs` - Added OutboxMessages DbSet and configuration

### 6. **Dependency Injection Setup**

Wired up all new services in DI container.

**Updated Files:**
- `Infrastructure/Extensions/ServiceCollectionExtensions.cs`

**Registered services:**
- `IOutboxRepository` → `OutboxRepository` (Scoped)
- `IRouteService` → `EfRouteService` (Scoped)
- `OutboxProcessorService` (Hosted Background Service)
- HTTP client for background service with Polly policies

---

## 🚀 Getting Started

### Prerequisites

1. PostgreSQL database
2. .NET 8.0 SDK

### Database Setup

**Option 1: Docker (Recommended)**
```bash
docker run --name shortas-postgres \
  -e POSTGRES_DB=shortas_dev_db \
  -e POSTGRES_USER=shortas_user \
  -e POSTGRES_PASSWORD=shortas_password \
  -p 5432:5432 \
  -d postgres:15
```

**Option 2: Local PostgreSQL**
Update connection string in `appsettings.Development.json`

### Create Database Migrations

```bash
cd /home/max/dev/shortas/api

# Create initial migration
dotnet ef migrations add InitialCreate --output-dir Infrastructure/Data/Migrations

# Apply migration to database
dotnet ef database update
```

### Configuration

Update `appsettings.Development.json`:

```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=shortas_dev_db;Username=shortas_user;Password=shortas_password;Port=5432"
  },
  "ApiSettings": {
    "ClickRouterApi": {
      "BaseUrl": "http://localhost:8081",
      "Timeout": 30
    }
  }
}
```

### Run the API

```bash
cd /home/max/dev/shortas/api
dotnet run
```

The API will start on `http://localhost:5050`

---

## 📋 API Endpoints

### Routes

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/routes` | List all routes (paginated) |
| GET | `/api/v1/routes/{domain}/{path}` | Get specific route |
| POST | `/api/v1/routes` | Create new route |
| PUT | `/api/v1/routes/{domain}/{path}` | Update route |
| DELETE | `/api/v1/routes/{domain}/{path}` | Delete route |
| POST | `/api/v1/routes/bulk` | Bulk create routes |
| PUT | `/api/v1/routes/bulk` | Bulk update routes |
| DELETE | `/api/v1/routes/bulk` | Bulk delete routes |

### Example: List Routes with Filtering

```bash
curl -X GET "http://localhost:5050/api/v1/routes?page=1&pageSize=20&status=active&search=example" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### Example: Create Route

```bash
curl -X POST "http://localhost:5050/api/v1/routes" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "switch": "default",
    "link": "example.com/test",
    "dest": "https://target.com",
    "destFormat": "direct",
    "code": 302,
    "ttl": 3600,
    "status": "active",
    "terminal": "true",
    "properties": {
      "routeId": "route-123",
      "domainId": "domain-456",
      "ownerId": "user-789",
      "scripts": [],
      "tags": ["test"],
      "custom": {},
      "opengraph": true,
      "allowDebug": false
    }
  }'
```

---

## 🏗️ Architecture Overview

### Data Flow

```
┌─────────────────┐
│   Dashboard     │
│  (React App)    │
└────────┬────────┘
         │ HTTP Requests
         ↓
┌─────────────────────────────┐
│    C# Proxy API (Port 5050) │
│  ┌──────────────────────┐   │
│  │ RoutesController     │   │
│  └──────────┬───────────┘   │
│             ↓               │
│  ┌──────────────────────┐   │
│  │  EfRouteService      │   │
│  │  (Business Logic)    │   │
│  └──────────┬───────────┘   │
│             ↓               │
│  ┌──────────────────────┐   │
│  │  PostgreSQL DB       │   │
│  │  + OutboxMessages    │   │
│  └──────────────────────┘   │
└─────────────────────────────┘
         ↑
         │ Background Service
         │ Processes outbox every 5s
         ↓
┌─────────────────────────────┐
│   Rust click-router-api     │
│   (Port 8081)               │
└─────────────────────────────┘
```

### Outbox Pattern Flow

```
1. API Request → Controller → Service
2. Service starts DB transaction
3. Save route to DB
4. Create outbox message
5. Commit transaction
6. Return response to client

   [Background Service - Every 5 seconds]
7. Query pending outbox messages
8. Send to click-router-api via HTTP
9. Mark as completed or schedule retry
10. Exponential backoff for failures
```

---

## 🧪 Testing

### Monitor Outbox Processing

Check the logs for outbox processor activity:

```bash
tail -f logs/shortas-api-*.txt | grep "Outbox"
```

You should see:
```
[INF] Outbox Processor Service started
[INF] Processing 3 outbox messages
[INF] Successfully processed outbox message {MessageId}, Event: RouteCreated
```

### Verify Database

```sql
-- Check routes
SELECT * FROM "Routes" ORDER BY "Id" DESC LIMIT 10;

-- Check outbox messages
SELECT * FROM "OutboxMessages" ORDER BY "CreatedAt" DESC LIMIT 10;

-- Check pending messages
SELECT * FROM "OutboxMessages" WHERE "Status" = 'Pending';

-- Check failed messages
SELECT * FROM "OutboxMessages" WHERE "Status" = 'Failed';
```

---

## 🔧 Troubleshooting

### Outbox Messages Not Processing

1. **Check background service is running:**
   - Look for "Outbox Processor Service started" in logs

2. **Check click-router-api connectivity:**
   ```bash
   curl http://localhost:8081/health
   ```

3. **Check for failed messages:**
   ```sql
   SELECT * FROM "OutboxMessages" WHERE "Status" = 'Failed';
   ```

4. **Manually retry failed messages:**
   ```sql
   UPDATE "OutboxMessages"
   SET "Status" = 'Pending', "RetryCount" = 0, "NextRetryAt" = NULL
   WHERE "Status" = 'Failed';
   ```

### Database Connection Issues

1. **Verify PostgreSQL is running:**
   ```bash
   docker ps | grep postgres
   ```

2. **Test connection:**
   ```bash
   psql -h localhost -U shortas_user -d shortas_dev_db
   ```

3. **Check connection string** in `appsettings.Development.json`

---

## 📊 Performance Considerations

### Outbox Processing

- **Polling Interval**: 5 seconds (configurable in `OutboxProcessorService.cs`)
- **Batch Size**: 10 messages per poll (configurable)
- **Retry Strategy**: Exponential backoff (1min → 2min → 4min → 8min → 16min)
- **Max Retries**: 5 attempts

### Database Indexes

The following indexes are created for optimal performance:
- `OutboxMessages.Status` - Fast pending message queries
- `OutboxMessages.(Status, NextRetryAt)` - Efficient retry scheduling
- `Routes.Link` - Fast route lookups
- `RouteProperties.OwnerId` - Owner-based filtering
- `RouteProperties.DomainId` - Domain-based filtering

---

## 🚧 Future Enhancements

### Recommended Improvements

1. **Monitoring & Observability**
   - Add Prometheus metrics for outbox processing
   - Track message processing latency
   - Alert on high failure rates

2. **Dead Letter Queue**
   - Move permanently failed messages to a dead letter table
   - Manual review and reprocessing interface

3. **Idempotency**
   - Add idempotency keys to prevent duplicate processing
   - Handle duplicate messages gracefully

4. **Batch Processing**
   - Send multiple routes to click-router-api in a single request
   - Reduce HTTP overhead

5. **Event Sourcing**
   - Keep history of all route changes
   - Support event replay

---

## 📝 API Documentation

Swagger documentation is available at:
```
http://localhost:5050/swagger
```

---

## ✅ Checklist for Deployment

- [ ] PostgreSQL database is running
- [ ] Run EF Core migrations: `dotnet ef database update`
- [ ] Update connection strings for production
- [ ] Configure click-router-api base URL
- [ ] Test outbox processing in staging environment
- [ ] Set up monitoring for outbox messages
- [ ] Configure log aggregation
- [ ] Set up database backups
- [ ] Review and adjust retry policies
- [ ] Load test the API endpoints

---

## 🎉 Summary

Your Routes API now has:

✅ **Local database storage** with PostgreSQL
✅ **Outbox pattern** for reliable event propagation
✅ **Background service** for async processing
✅ **Pagination & filtering** on list endpoint
✅ **Transaction management** for data consistency
✅ **Retry logic** with exponential backoff
✅ **Comprehensive error handling**
✅ **Bug fix** for bulk delete

All changes are saved locally and automatically propagated to the Rust click-router-api asynchronously!
