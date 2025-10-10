# Database Setup Guide

This guide explains how to set up PostgreSQL with Entity Framework for the Shortas API.

## Prerequisites

- .NET 8.0 SDK
- PostgreSQL 15+ (or Docker)
- Entity Framework Core tools

## Quick Start with Docker

1. Start PostgreSQL using Docker Compose:
   ```bash
   docker-compose -f docker-compose.postgres.yml up -d
   ```

2. Run the database setup script:
   ```bash
   ./scripts/setup-database.sh
   ```

3. Start the API:
   ```bash
   dotnet run
   ```

## Manual Setup

### 1. Install PostgreSQL

#### Ubuntu/Debian:
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
```

#### macOS (with Homebrew):
```bash
brew install postgresql
brew services start postgresql
```

#### Windows:
Download and install from [PostgreSQL official website](https://www.postgresql.org/download/windows/)

### 2. Create Database and User

```sql
-- Connect to PostgreSQL as superuser
sudo -u postgres psql

-- Create user
CREATE USER shortas_user WITH PASSWORD 'shortas_password';

-- Create database
CREATE DATABASE shortas_dev_db OWNER shortas_user;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE shortas_dev_db TO shortas_user;

-- Exit
\q
```

### 3. Install Entity Framework Tools

```bash
dotnet tool install --global dotnet-ef
```

### 4. Run Migrations

```bash
# Add initial migration (if not exists)
dotnet ef migrations add InitialCreate

# Update database
dotnet ef database update
```

## Configuration

### Connection Strings

The application uses the following connection strings:

**Development:**
```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=shortas_dev_db;Username=shortas_user;Password=shortas_password;Port=5432"
  }
}
```

**Production:**
```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=shortas_db;Username=shortas_user;Password=shortas_password;Port=5432"
  }
}
```

### Environment Variables

You can override connection strings using environment variables:

```bash
export ConnectionStrings__DefaultConnection="Host=localhost;Database=shortas_db;Username=shortas_user;Password=your_password;Port=5432"
```

## Database Schema

The application includes the following entities:

- **Certificates**: SSL certificate storage
- **Routes**: URL routing configuration
- **RouteProperties**: Additional route metadata
- **UserSettings**: User preferences and configuration

**Note**: ClickStream data is handled via HTTP proxy to the Click Aggregator API and is not stored in the local database.

## Entity Framework Commands

### Add Migration
```bash
dotnet ef migrations add MigrationName
```

### Update Database
```bash
dotnet ef database update
```

### Remove Last Migration
```bash
dotnet ef migrations remove
```

### Generate SQL Script
```bash
dotnet ef migrations script
```

### Database Drop and Recreate
```bash
dotnet ef database drop
dotnet ef database update
```

## Troubleshooting

### Connection Issues

1. **Check PostgreSQL is running:**
   ```bash
   pg_isready -h localhost -p 5432
   ```

2. **Verify connection string:**
   ```bash
   psql -h localhost -U shortas_user -d shortas_dev_db
   ```

3. **Check firewall settings:**
   ```bash
   sudo ufw status
   ```

### Migration Issues

1. **Reset migrations:**
   ```bash
   rm -rf Migrations/
   dotnet ef migrations add InitialCreate
   dotnet ef database update
   ```

2. **Check migration status:**
   ```bash
   dotnet ef migrations list
   ```

### Performance Optimization

1. **Enable connection pooling:**
   ```csharp
   services.AddDbContext<ApplicationDbContext>(options =>
       options.UseNpgsql(connectionString, npgsqlOptions =>
           npgsqlOptions.EnableRetryOnFailure()));
   ```

2. **Add database indexes:**
   The DbContext includes optimized indexes for common queries.

## Security Considerations

1. **Use strong passwords** in production
2. **Enable SSL** for production connections
3. **Restrict database access** to application servers only
4. **Regular backups** of production data
5. **Monitor database logs** for suspicious activity

## Backup and Restore

### Backup
```bash
pg_dump -h localhost -U shortas_user -d shortas_dev_db > backup.sql
```

### Restore
```bash
psql -h localhost -U shortas_user -d shortas_dev_db < backup.sql
```

