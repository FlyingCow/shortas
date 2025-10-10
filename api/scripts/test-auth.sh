#!/bin/bash

# Shortas Proxy API Authentication Test Script
# This script helps you test the Keycloak authentication with the API

echo "🔐 Shortas Proxy API Authentication Test"
echo "======================================"

# Configuration
KEYCLOAK_URL="http://localhost:8080"
REALM="shortas-dev"
CLIENT_ID="shortas-api"
CLIENT_SECRET=""
USERNAME="testuser"
PASSWORD="testpassword"
API_URL="http://localhost:5050"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Step 1: Check if Keycloak is running...${NC}"
if curl -s "$KEYCLOAK_URL/realms/$REALM" > /dev/null; then
    echo -e "${GREEN}✅ Keycloak is running at $KEYCLOAK_URL${NC}"
else
    echo -e "${RED}❌ Keycloak is not running at $KEYCLOAK_URL${NC}"
    echo "Please start Keycloak first:"
    echo "docker run -d --name keycloak -p 8080:8080 -e KEYCLOAK_ADMIN=admin -e KEYCLOAK_ADMIN_PASSWORD=admin quay.io/keycloak/keycloak:latest start-dev"
    exit 1
fi

echo -e "${YELLOW}Step 2: Check if API is running...${NC}"
if curl -s "$API_URL/api/health" > /dev/null; then
    echo -e "${GREEN}✅ API is running at $API_URL${NC}"
else
    echo -e "${RED}❌ API is not running at $API_URL${NC}"
    echo "Please start the API first:"
    echo "cd /home/max/dev/shortas/api && dotnet run"
    exit 1
fi

echo -e "${YELLOW}Step 3: Get JWT Token from Keycloak...${NC}"

# Check if client secret is provided
if [ -z "$CLIENT_SECRET" ]; then
    echo -e "${RED}❌ Client secret is not set${NC}"
    echo "Please update the CLIENT_SECRET variable in this script with your Keycloak client secret"
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
    echo ""
    echo "Please check:"
    echo "1. Keycloak realm 'shortas-dev' exists"
    echo "2. Client 'shortas-api' exists and is configured correctly"
    echo "3. User '$USERNAME' exists with password '$PASSWORD'"
    echo "4. Client secret is correct"
    exit 1
fi

echo -e "${GREEN}✅ Successfully obtained access token${NC}"

echo -e "${YELLOW}Step 4: Test API endpoints with authentication...${NC}"

# Test health endpoint (should work without auth)
echo "Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "%{http_code}" "$API_URL/api/health")
HTTP_CODE="${HEALTH_RESPONSE: -3}"
if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ Health endpoint working${NC}"
else
    echo -e "${RED}❌ Health endpoint failed (HTTP $HTTP_CODE)${NC}"
fi

# Test protected endpoint
echo "Testing protected clickstream endpoint..."
CLICKSTREAM_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" "$API_URL/api/v1/clickstream")
HTTP_CODE="${CLICKSTREAM_RESPONSE: -3}"
if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ Clickstream endpoint working with authentication${NC}"
elif [ "$HTTP_CODE" = "401" ]; then
    echo -e "${RED}❌ Clickstream endpoint returned 401 Unauthorized${NC}"
    echo "This might indicate:"
    echo "1. Token is invalid or expired"
    echo "2. User doesn't have required roles"
    echo "3. Keycloak configuration issue"
else
    echo -e "${YELLOW}⚠️  Clickstream endpoint returned HTTP $HTTP_CODE${NC}"
fi

# Test routes endpoint
echo "Testing routes endpoint..."
ROUTES_RESPONSE=$(curl -s -w "%{http_code}" -H "Authorization: Bearer $ACCESS_TOKEN" "$API_URL/api/v1/routes/example.com/test")
HTTP_CODE="${ROUTES_RESPONSE: -3}"
if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "404" ]; then
    echo -e "${GREEN}✅ Routes endpoint accessible (HTTP $HTTP_CODE)${NC}"
elif [ "$HTTP_CODE" = "401" ]; then
    echo -e "${RED}❌ Routes endpoint returned 401 Unauthorized${NC}"
else
    echo -e "${YELLOW}⚠️  Routes endpoint returned HTTP $HTTP_CODE${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Authentication test completed!${NC}"
echo ""
echo "📋 Summary:"
echo "- Access Token: ${ACCESS_TOKEN:0:20}..."
echo "- Token expires in: $(echo "$TOKEN_RESPONSE" | jq -r '.expires_in // "unknown"') seconds"
echo "- API Base URL: $API_URL"
echo "- Swagger UI: $API_URL/swagger/index.html"
echo ""
echo "🔧 To use the token in your applications:"
echo "curl -H \"Authorization: Bearer $ACCESS_TOKEN\" $API_URL/api/v1/clickstream"
echo ""
echo "🌐 To test in Swagger UI:"
echo "1. Open: $API_URL/swagger/index.html"
echo "2. Click 'Authorize' button"
echo "3. Enter: Bearer $ACCESS_TOKEN"
echo "4. Click 'Authorize' and test endpoints"
