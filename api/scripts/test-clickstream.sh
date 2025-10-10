#!/bin/bash

# ClickStream API Test Script
# This script tests the ClickStream API endpoints

echo "📊 ClickStream API Test"
echo "====================="

# Configuration
API_URL="http://localhost:5050"
KEYCLOAK_URL="http://localhost:8080"
REALM="shortas-dev"
CLIENT_ID="shortas-api"
CLIENT_SECRET=""
USERNAME="testuser"
PASSWORD="testpassword"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Step 1: Check if API is running...${NC}"
if curl -s "$API_URL/api/health" > /dev/null; then
    echo -e "${GREEN}✅ API is running at $API_URL${NC}"
else
    echo -e "${RED}❌ API is not running at $API_URL${NC}"
    echo "Please start the API first:"
    echo "cd /home/max/dev/shortas/api && dotnet run"
    exit 1
fi

echo -e "${YELLOW}Step 2: Check if Keycloak is running...${NC}"
if curl -s "$KEYCLOAK_URL/realms/$REALM" > /dev/null; then
    echo -e "${GREEN}✅ Keycloak is running at $KEYCLOAK_URL${NC}"
else
    echo -e "${RED}❌ Keycloak is not running at $KEYCLOAK_URL${NC}"
    echo "Please start Keycloak first:"
    echo "./setup-keycloak.sh"
    exit 1
fi

echo -e "${YELLOW}Step 3: Get JWT Token...${NC}"

# Check if client secret is provided
if [ -z "$CLIENT_SECRET" ]; then
    echo -e "${RED}❌ Client secret is not set${NC}"
    echo "Please update the CLIENT_SECRET variable in this script"
    echo "You can find it in Keycloak Admin Console > shortas-dev realm > Clients > shortas-api > Credentials"
    exit 1
fi

# Get access token
TOKEN_RESPONSE=$(curl -s -X POST "$KEYCLOAK_URL/auth/realms/$REALM/protocol/openid-connect/token" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=$CLIENT_ID" \
  -d "client_secret=$CLIENT_SECRET" \
  -d "username=$USERNAME" \
  -d "password=$PASSWORD")

# Extract access token
ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r '.access_token // empty')

if [ -z "$ACCESS_TOKEN" ] || [ "$ACCESS_TOKEN" = "null" ]; then
    echo -e "${RED}❌ Failed to get access token${NC}"
    echo "Response: $TOKEN_RESPONSE"
    exit 1
fi

echo -e "${GREEN}✅ Successfully obtained access token${NC}"

echo -e "${YELLOW}Step 4: Test ClickStream API endpoints...${NC}"

# Test 1: Get all clickstream data
echo "Testing GET /api/v1/clickstream..."
CLICKSTREAM_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" "$API_URL/api/v1/clickstream")
HTTP_CODE="${CLICKSTREAM_RESPONSE: -3}"
CLICKSTREAM_BODY="${CLICKSTREAM_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ ClickStream endpoint working (HTTP $HTTP_CODE)${NC}"
    echo "Response preview:"
    echo "$CLICKSTREAM_BODY" | jq '.[0:2]' 2>/dev/null || echo "$CLICKSTREAM_BODY" | head -c 200
    echo ""
elif [ "$HTTP_CODE" = "401" ]; then
    echo -e "${RED}❌ ClickStream endpoint returned 401 Unauthorized${NC}"
    echo "This indicates authentication issues. Check:"
    echo "1. Token is valid and not expired"
    echo "2. User has required roles (read:clickstream)"
    echo "3. Keycloak configuration is correct"
else
    echo -e "${YELLOW}⚠️  ClickStream endpoint returned HTTP $HTTP_CODE${NC}"
    echo "Response: $CLICKSTREAM_BODY"
fi

# Test 2: Get clickstream statistics
echo "Testing GET /api/v1/clickstream/stats..."
STATS_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" "$API_URL/api/v1/clickstream/stats")
HTTP_CODE="${STATS_RESPONSE: -3}"
STATS_BODY="${STATS_RESPONSE%???}"

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ ClickStream stats endpoint working (HTTP $HTTP_CODE)${NC}"
    echo "Response preview:"
    echo "$STATS_BODY" | jq '.' 2>/dev/null || echo "$STATS_BODY" | head -c 200
    echo ""
elif [ "$HTTP_CODE" = "401" ]; then
    echo -e "${RED}❌ ClickStream stats endpoint returned 401 Unauthorized${NC}"
else
    echo -e "${YELLOW}⚠️  ClickStream stats endpoint returned HTTP $HTTP_CODE${NC}"
    echo "Response: $STATS_BODY"
fi

# Test 3: Test with query parameters
echo "Testing GET /api/v1/clickstream with date filters..."
DATE_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" \
  "$API_URL/api/v1/clickstream?startDate=2024-01-01&endDate=2024-12-31")
HTTP_CODE="${DATE_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ ClickStream with date filters working (HTTP $HTTP_CODE)${NC}"
else
    echo -e "${YELLOW}⚠️  ClickStream with date filters returned HTTP $HTTP_CODE${NC}"
fi

# Test 4: Test specific route (if you have a route ID)
echo "Testing GET /api/v1/clickstream/{routeId}..."
ROUTE_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" \
  "$API_URL/api/v1/clickstream/test-route-123")
HTTP_CODE="${ROUTE_RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "404" ]; then
    echo -e "${GREEN}✅ ClickStream by route endpoint accessible (HTTP $HTTP_CODE)${NC}"
else
    echo -e "${YELLOW}⚠️  ClickStream by route endpoint returned HTTP $HTTP_CODE${NC}"
fi

echo ""
echo -e "${GREEN}🎉 ClickStream API test completed!${NC}"
echo ""
echo "📋 Summary:"
echo "- Access Token: ${ACCESS_TOKEN:0:20}..."
echo "- API Base URL: $API_URL"
echo "- Swagger UI: $API_URL/swagger/index.html"
echo ""
echo "🔧 Available endpoints:"
echo "- GET /api/v1/clickstream - Get all clickstream data"
echo "- GET /api/v1/clickstream/{routeId} - Get clickstream for specific route"
echo "- GET /api/v1/clickstream/stats - Get clickstream statistics"
echo ""
echo "📊 Example usage in your dashboard:"
echo "```javascript"
echo "const response = await fetch('$API_URL/api/v1/clickstream', {"
echo "  headers: { 'Authorization': 'Bearer $ACCESS_TOKEN' }"
echo "});"
echo "const data = await response.json();"
echo "```"
echo ""
echo "🌐 Test in Swagger UI:"
echo "1. Open: $API_URL/swagger/index.html"
echo "2. Click 'Authorize' button"
echo "3. Enter: Bearer $ACCESS_TOKEN"
echo "4. Test the /api/v1/clickstream endpoints"
