#!/bin/bash

# Apply ClickStream Materialized Views Migration
# This script applies the materialized views to your ClickHouse database

set -e

# Configuration
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
CLICKHOUSE_USER="${CLICKHOUSE_USER:-default}"
CLICKHOUSE_PASSWORD="${CLICKHOUSE_PASSWORD:-}"
CLICKHOUSE_DATABASE="${CLICKHOUSE_DATABASE:-shortas}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to execute ClickHouse query
execute_query() {
    local query="$1"
    local description="$2"

    echo -e "${YELLOW}Executing: ${description}${NC}"

    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary "$query"
    else
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary "$query"
    fi

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Success${NC}"
    else
        echo -e "${RED}✗ Failed${NC}"
        return 1
    fi
    echo ""
}

# Function to apply migration file
apply_migration() {
    local migration_file="$1"

    if [ ! -f "$migration_file" ]; then
        echo -e "${RED}Migration file not found: ${migration_file}${NC}"
        exit 1
    fi

    echo -e "${GREEN}Applying migration: ${migration_file}${NC}"
    echo "========================================"

    # Read and execute SQL file
    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary @"$migration_file"
    else
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary @"$migration_file"
    fi

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Migration applied successfully${NC}"
    else
        echo -e "${RED}✗ Migration failed${NC}"
        exit 1
    fi
}

# Function to check if base table exists
check_base_table() {
    echo "Checking if click_stream table exists..."

    local result
    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        result=$(curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary "EXISTS TABLE click_stream")
    else
        result=$(curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary "EXISTS TABLE click_stream")
    fi

    if [ "$result" = "1" ]; then
        echo -e "${GREEN}✓ Base table exists${NC}"
        return 0
    else
        echo -e "${RED}✗ Base table 'click_stream' not found${NC}"
        echo "Please create the click_stream table first"
        exit 1
    fi
}

# Function to list existing materialized views
list_existing_views() {
    echo "Checking existing materialized views..."

    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary "SELECT name, engine FROM system.tables WHERE database = '${CLICKHOUSE_DATABASE}' AND name LIKE '%_mv' ORDER BY name FORMAT Pretty"
    else
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary "SELECT name, engine FROM system.tables WHERE database = '${CLICKHOUSE_DATABASE}' AND name LIKE '%_mv' ORDER BY name FORMAT Pretty"
    fi
    echo ""
}

# Main execution
main() {
    echo "========================================"
    echo "ClickStream Materialized Views Setup"
    echo "========================================"
    echo "Host: ${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}"
    echo "Database: ${CLICKHOUSE_DATABASE}"
    echo "User: ${CLICKHOUSE_USER}"
    echo "========================================"
    echo ""

    # Check prerequisites
    check_base_table
    echo ""

    # List existing views
    list_existing_views
    echo ""

    # Apply migration
    SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
    MIGRATION_FILE="${SCRIPT_DIR}/../migrations/002_create_materialized_views.sql"

    apply_migration "$MIGRATION_FILE"
    echo ""

    # List views after migration
    echo "========================================"
    echo "Materialized views after migration:"
    echo "========================================"
    list_existing_views

    # Show view statistics
    echo "========================================"
    echo "View statistics:"
    echo "========================================"
    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary "
                SELECT
                    name,
                    formatReadableSize(total_bytes) AS size,
                    formatReadableQuantity(total_rows) AS rows
                FROM system.tables
                WHERE database = '${CLICKHOUSE_DATABASE}'
                  AND name LIKE '%_mv'
                ORDER BY total_rows DESC
                FORMAT Pretty"
    else
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary "
                SELECT
                    name,
                    formatReadableSize(total_bytes) AS size,
                    formatReadableQuantity(total_rows) AS rows
                FROM system.tables
                WHERE database = '${CLICKHOUSE_DATABASE}'
                  AND name LIKE '%_mv'
                ORDER BY total_rows DESC
                FORMAT Pretty"
    fi

    echo ""
    echo -e "${GREEN}✓ All materialized views created successfully!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Views will automatically populate as new data arrives"
    echo "  2. Check docs/MATERIALIZED_VIEWS.md for query examples"
    echo "  3. Monitor view performance with: ./scripts/check_view_stats.sh"
}

# Run main function
main
