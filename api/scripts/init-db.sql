-- Initialize PostgreSQL database for Shortas API
-- This script sets up the database with proper extensions and configurations

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Create indexes for better performance (will be created by EF migrations, but good to have as backup)
-- These will be created by Entity Framework migrations, but keeping for reference

-- Grant necessary permissions
GRANT ALL PRIVILEGES ON DATABASE shortas_dev_db TO shortas_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO shortas_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO shortas_user;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO shortas_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO shortas_user;

