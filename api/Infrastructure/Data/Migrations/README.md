# Database Migrations

## Creating Migrations

To create a new migration, run:

```bash
dotnet ef migrations add <MigrationName> --project api/ShortasProxyApi.csproj --output-dir Infrastructure/Data/Migrations
```

## Applying Migrations

To apply migrations to the database:

```bash
dotnet ef database update --project api/ShortasProxyApi.csproj
```

## Initial Setup

The following tables will be created:

1. **Routes** - Stores route information
2. **RouteProperties** - Stores route properties (JSON fields)
3. **Certificates** - SSL certificates
4. **UserSettings** - User settings
5. **OutboxMessages** - Outbox pattern for eventual consistency with click-router-api

## Commands to Run

```bash
# Navigate to the API directory
cd api

# Add the initial migration
dotnet ef migrations add InitialCreate --output-dir Infrastructure/Data/Migrations

# Apply the migration to create the database
dotnet ef database update

# (Optional) Generate SQL script without applying
dotnet ef migrations script --output migrations.sql
```

## Connection String

Update the connection string in `appsettings.json` or `appsettings.Development.json`:

```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=shortas_dev_db;Username=shortas_user;Password=shortas_password;Port=5432"
  }
}
```

## PostgreSQL Setup

If you don't have PostgreSQL set up, you can use Docker:

```bash
docker run --name shortas-postgres \
  -e POSTGRES_DB=shortas_dev_db \
  -e POSTGRES_USER=shortas_user \
  -e POSTGRES_PASSWORD=shortas_password \
  -p 5432:5432 \
  -d postgres:15
```
