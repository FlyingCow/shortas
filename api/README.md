# Shortas Management API

ASP.NET Core 9 REST API for managing workspaces, routes, domains, certificates, and user settings. Serves as the backend for the Dashboard UI and proxies analytics requests to the Rust services.

## Tech Stack

- ASP.NET Core 9 / C# 12
- Entity Framework Core 8 with PostgreSQL
- Elasticsearch 7.17 (NEST client) for full-text route search
- Keycloak JWT authentication
- FluentValidation
- Serilog logging
- Polly resilience policies
- Swagger/OpenAPI

## Endpoints

| Controller | Base Path | Description |
|-----------|-----------|-------------|
| Routes | `/api/v1/routes` | Short link CRUD and search |
| Workspaces | `/api/workspaces` | Multi-tenant workspace management |
| Domains | `/api/domains` | Custom domain configuration |
| Certificates | `/api/certificates` | TLS certificate management |
| ClickStream | `/api/clickstream` | Analytics proxy to Click Aggregator API |
| User | `/api/user` | User profile and settings |
| Health | `/api/health` | Health check |

### Route Search (Elasticsearch)

Routes are indexed in Elasticsearch and searchable by link, domain name, and destination URL.

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/routes/search?q=<query>` | Full-text search across routes |
| `POST` | `/api/v1/routes/search/reindex` | Rebuild the search index from the database |

The search endpoint supports pagination (`page`, `pageSize`) and workspace filtering (`workspaceId`).

**Real-time sync:** Route changes (create, update, delete) are automatically propagated to Elasticsearch via the outbox pattern. The `OutboxProcessorService` background worker picks up `RouteSearchIndex`, `RouteSearchDelete`, `RouteSearchBulkIndex`, and `RouteSearchBulkDelete` events and applies them to the search index.

## Project Structure

```
api/
├── Presentation/       HTTP controllers
├── Application/        Business logic, DTOs, services
├── Domain/             Entities and interfaces
├── Infrastructure/     EF Core, repositories, security, HTTP clients,
│                       Elasticsearch service, outbox processor
├── Persistence/        Database context and configurations
├── Program.cs          Startup and dependency injection
├── appsettings.json    Configuration
├── docker-compose.yml  PostgreSQL + Elasticsearch + API
└── Dockerfile          Multi-stage Docker build
```

## Development

```bash
dotnet run
```

The API starts on port 5050. Swagger UI is available at `http://localhost:5050/swagger`.

## Docker

```bash
docker compose up -d
```

Starts the following services:

| Service | Container | Port |
|---------|-----------|------|
| API | `shortas-api` | 8090 |
| PostgreSQL 15 | `shortas-api-postgres` | 5433 |
| Elasticsearch 7.17 | `shortas-api-elasticsearch` | 9200 |

The API waits for both PostgreSQL and Elasticsearch health checks before starting.

## Configuration

Key settings in `appsettings.json`:

| Setting | Description |
|---------|-------------|
| `ConnectionStrings:DefaultConnection` | PostgreSQL connection |
| `Keycloak:Authority` | Keycloak realm URL |
| `Keycloak:Audience` | JWT audience |
| `ApiSettings:ClickRouterApi:BaseUrl` | Click Router API URL |
| `ApiSettings:ClickAggregatorApi:BaseUrl` | Click Aggregator API URL |
| `Elasticsearch:Url` | Elasticsearch node URL (default: `http://localhost:9200`) |
| `Elasticsearch:IndexName` | Search index name (default: `routes`) |

## Elasticsearch

The API uses Elasticsearch for full-text route search. On startup, the `routes` index is created automatically with a custom analyzer that supports partial and prefix matching.

**Index initialization:** `Program.cs` calls `IRouteSearchService.EnsureIndexAsync()` at startup. If Elasticsearch is unreachable, the API starts normally and search returns errors until the connection is restored.

**Reindexing:** If the index is out of sync (e.g. after a fresh Elasticsearch deployment), call:

```bash
curl -X POST http://localhost:8090/api/v1/routes/search/reindex \
  -H "Authorization: Bearer <token>"
```

This fetches all routes from PostgreSQL and bulk-indexes them into Elasticsearch.

## Database Migrations

Uses Entity Framework Core migrations:

```bash
dotnet ef migrations add <MigrationName>
dotnet ef database update
```
