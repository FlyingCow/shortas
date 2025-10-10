#!/bin/bash

# Database setup script for Shortas API
# This script helps set up PostgreSQL database and run Entity Framework migrations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up PostgreSQL database for Shortas API...${NC}"

# Check if PostgreSQL is running
if ! pg_isready -h localhost -p 5433 > /dev/null 2>&1; then
    echo -e "${RED}PostgreSQL is not running. Please start PostgreSQL first.${NC}"
    exit 1
fi

# Database configuration
DB_NAME="shortas_dev_db"
DB_USER="shortas_user"
DB_PASSWORD="shortas_password"

echo -e "${YELLOW}Creating database and user...${NC}"

# Create user if it doesn't exist
sudo -u postgres psql -c "DO \$\$ BEGIN IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = '$DB_USER') THEN CREATE ROLE $DB_USER LOGIN PASSWORD '$DB_PASSWORD'; END IF; END \$\$;"

# Create database if it doesn't exist
sudo -u postgres psql -c "SELECT 'CREATE DATABASE $DB_NAME OWNER $DB_USER' WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '$DB_NAME')\gexec"

# Grant privileges
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"

echo -e "${GREEN}Database setup completed!${NC}"

# Check if we're in the API directory
if [ ! -f "ShortasProxyApi.csproj" ]; then
    echo -e "${YELLOW}Please run this script from the API directory${NC}"
    exit 1
fi

echo -e "${YELLOW}Running Entity Framework migrations...${NC}"

# Add initial migration if it doesn't exist
if [ ! -d "Migrations" ]; then
    echo -e "${YELLOW}Creating initial migration...${NC}"
    dotnet ef migrations add InitialCreate
fi

# Update database
echo -e "${YELLOW}Updating database schema...${NC}"
dotnet ef database update

echo -e "${GREEN}Database setup completed successfully!${NC}"
echo -e "${GREEN}You can now run the API with: dotnet run${NC}"

