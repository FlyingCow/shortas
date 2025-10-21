#!/bin/bash

# Reset ClickHouse - Drop all tables and views
# This script removes all click_stream tables and materialized views

set -e

# Configuration
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-localhost}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
CLICKHOUSE_USER="${CLICKHOUSE_USER:-default}"
CLICKHOUSE_PASSWORD="${CLICKHOUSE_PASSWORD:-}"
CLICKHOUSE_DATABASE="${CLICKHOUSE_DATABASE:-shortas}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

echo "========================================"
echo "ClickHouse Reset Script"
echo "========================================"
echo "Host: ${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}"
echo "Database: ${CLICKHOUSE_DATABASE}"
echo "Time: $(date)"
echo "========================================"
echo ""

echo -e "${YELLOW}WARNING: This will drop ALL click_stream tables and views!${NC}"
echo ""
read -p "Are you sure you want to continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Reset cancelled"
    exit 0
fi
echo ""

# Drop all materialized views
echo -e "${BLUE}Step 1: Dropping materialized views...${NC}"
views=(
    "click_stream_hourly_mv"
    "click_stream_daily_mv"
    "click_stream_geographic_mv"
    "click_stream_device_mv"
    "click_stream_browser_mv"
    "click_stream_route_performance_mv"
    "click_stream_owner_workspace_mv"
    "click_stream_realtime_mv"
    "click_stream_top_destinations_mv"
    "click_stream_traffic_type_mv"
    "click_stream_session_mv"
    "click_stream_recent_activity_mv"
)

for view in "${views[@]}"; do
    echo "  Dropping ${view}..."
    execute_query "DROP VIEW IF EXISTS ${view}" > /dev/null 2>&1 || echo "    (view doesn't exist)"
done
echo -e "${GREEN}✓ Views dropped${NC}"
echo ""

# Drop backup table if exists
echo -e "${BLUE}Step 2: Dropping backup tables...${NC}"
execute_query "DROP TABLE IF EXISTS click_stream_old" > /dev/null 2>&1
execute_query "DROP TABLE IF EXISTS click_stream_new" > /dev/null 2>&1
echo -e "${GREEN}✓ Backup tables dropped${NC}"
echo ""

# Drop main table
echo -e "${BLUE}Step 3: Dropping main click_stream table...${NC}"
execute_query "DROP TABLE IF EXISTS click_stream" > /dev/null 2>&1
echo -e "${GREEN}✓ Main table dropped${NC}"
echo ""

# Verify cleanup
echo -e "${BLUE}Step 4: Verifying cleanup...${NC}"
tables=$(execute_query "SELECT name FROM system.tables WHERE database = '${CLICKHOUSE_DATABASE}' AND name LIKE 'click_stream%' FORMAT TabSeparated")
if [ -z "$tables" ]; then
    echo -e "${GREEN}✓ All click_stream tables and views removed${NC}"
else
    echo -e "${YELLOW}Warning: Some tables still exist:${NC}"
    echo "$tables"
fi
echo ""

echo "========================================"
echo -e "${GREEN}Reset Complete!${NC}"
echo "========================================"
echo ""
echo "Next steps:"
echo "  1. Create fresh migrations"
echo "  2. Apply initial schema"
echo "  3. Create materialized views"
echo ""
