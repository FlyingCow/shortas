# Shortas Architecture

## Overview

Shortas is a multi-tenant URL shortener platform with:
- Real-time click analytics
- Conditional routing (geo, device, browser, time-based)
- QR code generation
- Custom domain support with automatic TLS
- Multi-backend storage (MongoDB, DynamoDB)

## System Architecture

```
                                    ┌─────────────────────┐
                                    │   Dashboard (React) │
                                    │     Port 3000       │
                                    └──────────┬──────────┘
                                               │
┌──────────────────────────────────────────────┼──────────────────────────────────────────────┐
│                                              ▼                                              │
│  ┌─────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                        Management API (ASP.NET Core 9)                               │   │
│  │                               Port 5050                                              │   │
│  │   PostgreSQL ◄──► Entity Framework    Elasticsearch ◄──► NEST                       │   │
│  └─────────────────────────────────────────────────────────────────────────────────────┘   │
│                                              │                                              │
│                              ┌───────────────┼───────────────┐                              │
│                              ▼               ▼               ▼                              │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐            │
│  │ click-router   │  │click-router-api│  │click-aggregator│  │ route-verifier │            │
│  │   Port 5800    │  │   Port 5810    │  │   -api: 5820   │  │   Port 5831    │            │
│  │  URL Redirect  │  │  Route CRUD    │  │ Analytics API  │  │ Safe Browsing  │            │
│  └───────┬────────┘  └────────────────┘  └────────────────┘  └────────────────┘            │
│          │                                                                                  │
│          │ Events (Fluvio/Kafka)                                                           │
│          ▼                                                                                  │
│  ┌────────────────┐                       ┌────────────────┐  ┌────────────────┐            │
│  │ click-tracker  │  ──RabbitMQ──►       │click-aggregator│  │domain-verifier │            │
│  │ Event Enricher │                       │ClickHouse Sink │  │   Port 5830    │            │
│  └────────────────┘                       └────────────────┘  └────────────────┘            │
│                                                                                             │
│  Background Workers:                                                                        │
│  ┌────────────────┐  ┌────────────────┐                                                    │
│  │    cert-bot    │  │route-icon-worker│                                                   │
│  │  TLS Certs     │  │ Favicon Scraper │                                                   │
│  └────────────────┘  └────────────────┘                                                    │
│                                                                                             │
└─────────────────────────────────────────────────────────────────────────────────────────────┘

Data Stores:
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   MongoDB/   │  │  PostgreSQL  │  │  ClickHouse  │  │    Redis     │  │   MinIO/S3   │
│   DynamoDB   │  │              │  │              │  │              │  │              │
│   (Routes)   │  │ (Management) │  │ (Analytics)  │  │  (Sessions)  │  │   (Images)   │
└──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘
```

## Services

### Core Redirect Path

| Service | Port | Purpose | Technology |
|---------|------|---------|------------|
| click-router | 5800 | URL redirect handler | Rust/Salvo |
| click-tracker | - | Event enrichment | Rust/Tokio consumer |
| click-aggregator | - | ClickHouse ingestion | Rust/Tokio consumer |
| click-aggregator-api | 5820 | Analytics queries | Rust/Salvo |

### Management

| Service | Port | Purpose | Technology |
|---------|------|---------|------------|
| management-api | 5050 | Admin API | ASP.NET Core 9 |
| click-router-api | 5810 | Route CRUD | Rust/Salvo |
| dashboard | 3000 | Admin UI | React 18 |

### Background Workers

| Service | Port | Purpose | Technology |
|---------|------|---------|------------|
| route-verifier | 5831 | Safe Browsing checks | Rust/Salvo |
| domain-verifier | 5830 | DNS verification | Rust/Salvo |
| cert-bot | - | TLS automation | Rust/instant-acme |
| route-icon-worker | - | Favicon scraping | Rust consumer |

## Rust Services (`/redirect/`)

All Rust services are in a Cargo workspace with shared dependencies.

### Workspace Structure

```
redirect/
├── Cargo.toml              # Workspace manifest
├── docker-compose.yml      # Local infrastructure
├── click-router/           # URL redirect handler
├── click-router-api/       # Route CRUD API
├── click-tracker/          # Event enrichment
├── click-aggregator/       # ClickHouse sink
├── click-aggregator-api/   # Analytics API
├── cert-bot/               # TLS automation
├── domain-verifier/        # DNS verification
├── route-verifier/         # Safe Browsing
└── route-icon-worker/      # Favicon scraper
```

### Adapter Pattern

Rust services use enum-based dispatch for storage backends:

```rust
// Enum wraps concrete implementations
pub enum RoutesStoreType {
    Dynamo(DynamoRoutesStore),
    Mongodb(MongodbRoutesStore),
    InMemory(InMemoryRoutesStore),
}

// Trait implementation delegates to inner type
impl RoutesStore for RoutesStoreType {
    async fn get_route(&self, switch: &str, path: &str) -> Result<Option<Route>> {
        match self {
            Self::Dynamo(s) => s.get_route(switch, path).await,
            Self::Mongodb(s) => s.get_route(switch, path).await,
            Self::InMemory(s) => s.get_route(switch, path).await,
        }
    }
}
```

**Why enums over trait objects?**
- Clone without Arc wrappers
- No vtable overhead
- Compile-time exhaustiveness checking

### Configuration

Three-tier hierarchy:
1. `config/default.toml` - Base configuration
2. `config/{run_mode}.toml` - Environment overrides
3. Environment variables (`APP_*` prefix)

```toml
# config/default.toml
[mongodb]
uri = "mongodb://localhost:27017"
database = "shortas"

[moka.routes_cache]
max_capacity = 10000
time_to_live = 300

[server]
host = "0.0.0.0"
port = 5800
```

## C# API (`/api/`)

### Clean Architecture

```
api/
├── Presentation/           # Controllers, Middleware
│   └── Controllers/
├── Application/            # DTOs, Services
│   ├── DTOs/
│   └── Services/
├── Domain/                 # Entities, Interfaces
│   ├── Entities/
│   ├── Interfaces/
│   └── Common/
└── Infrastructure/         # Data access, HTTP clients
    ├── Data/
    ├── HttpClients/
    └── Services/
```

### Service Interfaces

```csharp
// Write operations - implemented by EfRouteService and RouteService
public interface IRouteCommandService
{
    Task<Result<Route>> CreateRouteAsync(Route route);
    Task<Result<Route>> UpdateRouteByIdAsync(Guid id, string userId, Route route);
    Task<Result> DeleteRouteByIdAsync(Guid id, string userId);
    // ... bulk operations
}

// Read operations - implemented only by EfRouteService
public interface IRouteQueryService
{
    Task<Result<Route?>> GetRouteByIdAsync(Guid id, string userId);
    Task<Result<(List<Route>, int)>> ListRoutesAsync(...);
}

// Combined for backwards compatibility
public interface IRouteService : IRouteCommandService, IRouteQueryService { }
```

### Error Handling

Always use `Result<T>` pattern:

```csharp
// Success
return Result<Route>.Success(route);

// Failure with error factory
return Result<Route>.Failure(Error.NotFound("Route", id.ToString()));
return Result<Route>.Failure(Error.Validation("Invalid input", details));
```

Error factories in `Domain/Common/Error.cs`:
- `Error.NotFound(resource, identifier)`
- `Error.Validation(message, details?)`
- `Error.Unauthorized()`
- `Error.Forbidden()`
- `Error.Conflict(message)`
- `Error.Internal(message, details?)`

### JSON Serialization

Use centralized config:

```csharp
// Correct - use shared config
JsonSerializer.Serialize(obj, JsonConfig.Default);

// Incorrect - don't create new options
var options = new JsonSerializerOptions { ... }; // NO
```

## Frontend (`/ui/`)

### Dashboard (`/ui/dashboard/`)

React 18 + TypeScript + Bootstrap admin interface.

```
src/
├── components/     # React components
├── contexts/       # State management
├── services/       # API clients (axios)
├── config/         # Configuration
├── types/          # TypeScript interfaces
└── utils/          # Helpers
```

Key components:
- `Analytics.tsx` - Charts and metrics
- `Clickstream.tsx` - Real-time events
- `Domains.tsx` - Domain management
- `ConditionsEditor.tsx` - Conditional routing

### Landing (`/ui/landing/`)

Minimal React 19 marketing site.

## Infrastructure

### Local Development

```bash
# Start infrastructure
cd redirect && docker-compose up -d

# Start services
cargo run --package click-router
dotnet run --project api/ShortasProxyApi.csproj
cd ui/dashboard && npm start
```

### AWS Production (`/infra/aws/terraform/`)

19 Terraform modules deploy to AWS:
- ECS Fargate for all services
- RDS Aurora (PostgreSQL)
- ElastiCache (Redis)
- DynamoDB (routes)
- S3 (images)
- Amazon MQ (RabbitMQ)
- AWS Cognito (auth)

## Nullable Patterns

Match Rust `Option<T>` types:

```csharp
// Required with defaults - never null
public string Switch { get; set; } = string.Empty;
public string Status { get; set; } = "Active";

// Optional - nullable, maps to Option<T>
public string? Dest { get; set; }
public int? Code { get; set; }

// Navigation properties
public Guid? DomainId { get; set; }
public RouteDomain? Domain { get; set; }

// Child objects with defaults
public RouteProperties Properties { get; set; } = new();
```

## Data Flow

### Click Processing

1. **click-router** receives HTTP request
2. Lookup route in MongoDB/DynamoDB (via Moka cache)
3. Evaluate conditional routing rules
4. Emit click event to Fluvio/Kafka
5. Return HTTP redirect (301/302)

### Event Enrichment

1. **click-tracker** consumes from Fluvio/Kafka
2. Parse user-agent (uaparser)
3. Geolocate IP (MaxMind)
4. Resolve session (Redis)
5. Publish enriched event to RabbitMQ

### Analytics Ingestion

1. **click-aggregator** consumes from RabbitMQ
2. Batch events
3. Insert into ClickHouse
4. Expose Prometheus metrics

## Adding New Features

### New Rust Service

1. Create crate: `redirect/{service-name}/`
2. Add to workspace: `redirect/Cargo.toml`
3. Implement adapters: `src/adapters/`
4. Add config: `config/default.toml`
5. Add to docker-compose

### New API Endpoint

1. Interface: `Domain/Interfaces/`
2. Entity: `Domain/Entities/` (if needed)
3. DTO: `Application/DTOs/`
4. Service: `Infrastructure/Services/`
5. Register: `Program.cs`
6. Controller: `Presentation/Controllers/`

### New Dashboard Feature

1. Component: `components/`
2. Route: `App.tsx` (if needed)
3. Service: `services/`
4. Types: `types/`
