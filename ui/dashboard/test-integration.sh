#!/bin/bash

# Dashboard ClickStream Integration Test Script
# This script tests the dashboard integration with the ClickStream API

echo "🧪 Dashboard ClickStream Integration Test"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DASHBOARD_URL="http://localhost:3000"
API_URL="http://localhost:5050"
KEYCLOAK_URL="http://localhost:8080"

echo -e "${YELLOW}Step 1: Check if API is running...${NC}"
if curl -s "$API_URL/api/health" > /dev/null; then
    echo -e "${GREEN}✅ ClickStream API is running at $API_URL${NC}"
else
    echo -e "${RED}❌ ClickStream API is not running at $API_URL${NC}"
    echo "Please start the API first:"
    echo "cd /home/max/dev/shortas/api && dotnet run"
    exit 1
fi

echo -e "${YELLOW}Step 2: Check if Keycloak is running...${NC}"
if curl -s "$KEYCLOAK_URL/realms/shortas-dev" > /dev/null; then
    echo -e "${GREEN}✅ Keycloak is running at $KEYCLOAK_URL${NC}"
else
    echo -e "${RED}❌ Keycloak is not running at $KEYCLOAK_URL${NC}"
    echo "Please start Keycloak first:"
    echo "cd /home/max/dev/shortas/api && ./setup-keycloak.sh"
    exit 1
fi

echo -e "${YELLOW}Step 3: Check if Dashboard is running...${NC}"
if curl -s "$DASHBOARD_URL" > /dev/null; then
    echo -e "${GREEN}✅ Dashboard is running at $DASHBOARD_URL${NC}"
else
    echo -e "${RED}❌ Dashboard is not running at $DASHBOARD_URL${NC}"
    echo "Please start the dashboard first:"
    echo "cd /home/max/dev/shortas/ui/dashboard && npm start"
    exit 1
fi

echo -e "${YELLOW}Step 4: Test ClickStream API endpoints...${NC}"

# Test ClickStream endpoint (should return 401 without auth, which is expected)
CLICKSTREAM_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/clickstream")
HTTP_CODE="${CLICKSTREAM_RESPONSE: -3}"

if [ "$HTTP_CODE" = "401" ]; then
    echo -e "${GREEN}✅ ClickStream endpoint is accessible (HTTP 401 - requires auth)${NC}"
elif [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ ClickStream endpoint is working (HTTP 200)${NC}"
else
    echo -e "${YELLOW}⚠️  ClickStream endpoint returned HTTP $HTTP_CODE${NC}"
fi

# Test stats endpoint
STATS_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/clickstream/stats")
HTTP_CODE="${STATS_RESPONSE: -3}"

if [ "$HTTP_CODE" = "401" ]; then
    echo -e "${GREEN}✅ ClickStream stats endpoint is accessible (HTTP 401 - requires auth)${NC}"
elif [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ ClickStream stats endpoint is working (HTTP 200)${NC}"
else
    echo -e "${YELLOW}⚠️  ClickStream stats endpoint returned HTTP $HTTP_CODE${NC}"
fi

echo -e "${YELLOW}Step 5: Test Routes API endpoints...${NC}"

# Test Routes endpoint
ROUTES_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/v1/routes")
HTTP_CODE="${ROUTES_RESPONSE: -3}"

if [ "$HTTP_CODE" = "401" ]; then
    echo -e "${GREEN}✅ Routes endpoint is accessible (HTTP 401 - requires auth)${NC}"
elif [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ Routes endpoint is working (HTTP 200)${NC}"
else
    echo -e "${YELLOW}⚠️  Routes endpoint returned HTTP $HTTP_CODE${NC}"
fi

echo -e "${YELLOW}Step 6: Test Dashboard integration...${NC}"

# Check if dashboard is accessible
DASHBOARD_RESPONSE=$(curl -s -w "%{http_code}" "$DASHBOARD_URL")
HTTP_CODE="${DASHBOARD_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ Dashboard is accessible${NC}"
else
    echo -e "${RED}❌ Dashboard returned HTTP $HTTP_CODE${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Integration test completed!${NC}"
echo ""
echo "📋 Summary:"
echo "- ClickStream API: $API_URL"
echo "- Dashboard: $DASHBOARD_URL"
echo "- Keycloak: $KEYCLOAK_URL"
echo ""
echo "🔧 Next Steps:"
echo "1. Open the dashboard: $DASHBOARD_URL"
echo "2. Navigate to the ClickStream section to test analytics"
echo "3. Navigate to the Routes section to test route management"
echo "4. If using mock data, you should see sample data"
echo "5. If using real API, ensure you're authenticated with Keycloak"
echo ""
echo "🌐 Access URLs:"
echo "- Dashboard: $DASHBOARD_URL"
echo "- ClickStream API: $API_URL/api/v1/clickstream"
echo "- Routes API: $API_URL/api/v1/routes"
echo "- API Health: $API_URL/api/health"
echo "- Swagger UI: $API_URL/swagger/index.html"
echo "- Keycloak Admin: $KEYCLOAK_URL/admin"
echo ""
echo "📚 Documentation:"
echo "- ClickStream Integration: /home/max/dev/shortas/ui/dashboard/CLICKSTREAM_INTEGRATION.md"
echo "- Routes Integration: /home/max/dev/shortas/ui/dashboard/ROUTES_INTEGRATION.md"
echo "- Route Form Guide: /home/max/dev/shortas/ui/dashboard/ROUTE_FORM_GUIDE.md"
echo "- API Guide: /home/max/dev/shortas/api/CLICKSTREAM_API_GUIDE.md"
echo "- Keycloak Setup: /home/max/dev/shortas/api/KEYCLOAK_SETUP.md"
