# Shortas Management API

ASP.NET Core 9 REST API for managing workspaces, routes, domains, certificates, and user settings. Serves as the backend for the Dashboard UI and proxies analytics requests to the Rust services.

## Tech Stack

- ASP.NET Core 9 / C# 12
- Entity Framework Core 8 with PostgreSQL
- Keycloak JWT authentication
- FluentValidation
- Serilog logging
- Polly resilience policies
- Swagger/OpenAPI

## Endpoints

| Controller | Base Path | Description |
|-----------|-----------|-------------|
| Routes | `/api/routes` | Short link CRUD |
| Workspaces | `/api/workspaces` | Multi-tenant workspace management |
| Domains | `/api/domains` | Custom domain configuration |
| Certificates | `/api/certificates` | TLS certificate management |
| ClickStream | `/api/clickstream` | Analytics proxy to Click Aggregator API |
| User | `/api/user` | User profile and settings |
| Health | `/api/health` | Health check |

## Project Structure

```
api/
├── Presentation/       HTTP controllers
├── Application/        Business logic, DTOs, services
├── Domain/             Entities and interfaces
├── Infrastructure/     EF Core, repositories, security, HTTP clients
├── Persistence/        Database context and configurations
├── Program.cs          Startup and dependency injection
├── appsettings.json    Configuration
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

Starts the API (port 8090) and PostgreSQL (port 5433).

## Configuration

Key settings in `appsettings.json`:

| Setting | Description |
|---------|-------------|
| `ConnectionStrings:DefaultConnection` | PostgreSQL connection |
| `Keycloak:Authority` | Keycloak realm URL |
| `Keycloak:Audience` | JWT audience |
| `ApiSettings:ClickRouterApi:BaseUrl` | Click Router API URL |
| `ApiSettings:ClickAggregatorApi:BaseUrl` | Click Aggregator API URL |

## Database Migrations

Uses Entity Framework Core migrations:

```bash
dotnet ef migrations add <MigrationName>
dotnet ef database update
```
