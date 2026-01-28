#!/bin/bash

# Apply ClickHouse migrations in order

set -e

# Configuration
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
CLICKHOUSE_USER="${CLICKHOUSE_USER:-default}"
CLICKHOUSE_PASSWORD="${CLICKHOUSE_PASSWORD:-clickhouse}"
CLICKHOUSE_DATABASE="${CLICKHOUSE_DATABASE:-shortas}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
MIGRATION_DIR="${SCRIPT_DIR}/../migrations"

# Function to execute query
execute_query() {
    local query="$1"
    if [ -n "$CLICKHOUSE_PASSWORD" ]; then
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}&password=${CLICKHOUSE_PASSWORD}" \
            --data-binary "$query"
    else
        curl -sS "${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}/?database=${CLICKHOUSE_DATABASE}&user=${CLICKHOUSE_USER}" \
            --data-binary "$query"
    fi
}

# Function to execute file with multiple statements
execute_file_statements() {
    local file="$1"
    local description="$2"

    # Read the file and split by CREATE statements
    local current_statement=""
    local statement_count=0

    while IFS= read -r line; do
        # Skip empty lines and comment-only lines at the start
        if [ -z "$current_statement" ]; then
            if [[ "$line" =~ ^[[:space:]]*$ ]] || [[ "$line" =~ ^[[:space:]]*--.*$ ]]; then
                continue
            fi
        fi

        current_statement="${current_statement}${line}"$'\n'

        # Check if this line ends with a semicolon (end of statement)
        if [[ "$line" =~ \;[[:space:]]*$ ]]; then
            # Execute the statement
            if [ -n "$current_statement" ]; then
                statement_count=$((statement_count + 1))
                echo "  Executing statement ${statement_count}..."
                if ! execute_query "$current_statement"; then
                    echo -e "${RED}✗ Failed to execute statement ${statement_count}${NC}"
                    return 1
                fi
            fi
            current_statement=""
        fi
    done < "$file"

    # Execute any remaining statement
    if [ -n "$current_statement" ]; then
        statement_count=$((statement_count + 1))
        echo "  Executing statement ${statement_count}..."
        if ! execute_query "$current_statement"; then
            echo -e "${RED}✗ Failed to execute statement ${statement_count}${NC}"
            return 1
        fi
    fi

    echo -e "${GREEN}✓ Executed ${statement_count} statements${NC}"
}

echo "========================================"
echo "Applying ClickHouse Migrations"
echo "========================================"
echo "Host: ${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}"
echo "Database: ${CLICKHOUSE_DATABASE}"
echo "Time: $(date)"
echo "========================================"
echo ""

# Apply migration 001
echo -e "${BLUE}Applying migration 001: Create click_stream table...${NC}"
if execute_file_statements "${MIGRATION_DIR}/001_create_click_stream_table.sql" "Create click_stream table"; then
    echo -e "${GREEN}✓ Migration 001 applied${NC}"
else
    echo -e "${RED}✗ Migration 001 failed${NC}"
    exit 1
fi
echo ""

# Apply migration 002
echo -e "${BLUE}Applying migration 002: Create materialized views...${NC}"
if execute_file_statements "${MIGRATION_DIR}/002_create_materialized_views.sql" "Create materialized views"; then
    echo -e "${GREEN}✓ Migration 002 applied${NC}"
else
    echo -e "${RED}✗ Migration 002 failed${NC}"
    exit 1
fi
echo ""

# Apply migration 003
echo -e "${BLUE}Applying migration 003: Create conversions tables...${NC}"
if execute_file_statements "${MIGRATION_DIR}/003_create_conversions_tables.sql" "Create conversions tables"; then
    echo -e "${GREEN}✓ Migration 003 applied${NC}"
else
    echo -e "${RED}✗ Migration 003 failed${NC}"
    exit 1
fi
echo ""

# Apply migration 004
echo -e "${BLUE}Applying migration 004: Create conversion materialized views...${NC}"
if execute_file_statements "${MIGRATION_DIR}/004_create_conversion_materialized_views.sql" "Create conversion materialized views"; then
    echo -e "${GREEN}✓ Migration 004 applied${NC}"
else
    echo -e "${RED}✗ Migration 004 failed${NC}"
    exit 1
fi
echo ""

echo "========================================"
echo -e "${GREEN}All migrations applied successfully!${NC}"
echo "========================================"
echo ""
