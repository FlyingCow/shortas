#!/bin/bash

# Shortas Proxy API - Keycloak Setup Script
# This script helps you set up Keycloak for the Shortas Proxy API

echo "🔐 Shortas Proxy API - Keycloak Setup"
echo "====================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}This script will help you set up Keycloak for the Shortas Proxy API${NC}"
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}❌ Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

echo -e "${YELLOW}Step 1: Starting Keycloak with Docker...${NC}"

# Stop existing Keycloak container if running
docker stop shortas-keycloak 2>/dev/null || true
docker rm shortas-keycloak 2>/dev/null || true

# Start Keycloak
docker run -d --name shortas-keycloak \
  -p 8080:8080 \
  -e KEYCLOAK_ADMIN=admin \
  -e KEYCLOAK_ADMIN_PASSWORD=admin \
  -e KC_DB=h2-file \
  -e KC_HOSTNAME_STRICT=false \
  -e KC_HOSTNAME_STRICT_HTTPS=false \
  -e KC_HTTP_ENABLED=true \
  quay.io/keycloak/keycloak:latest start-dev

echo -e "${GREEN}✅ Keycloak container started${NC}"

echo -e "${YELLOW}Step 2: Waiting for Keycloak to be ready...${NC}"

# Wait for Keycloak to be ready
for i in {1..30}; do
    if curl -s http://localhost:8080/realms/master > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Keycloak is ready!${NC}"
        break
    fi
    echo "Waiting for Keycloak... ($i/30)"
    sleep 2
done

if ! curl -s http://localhost:8080/realms/master > /dev/null 2>&1; then
    echo -e "${RED}❌ Keycloak failed to start properly${NC}"
    echo "Check the container logs: docker logs shortas-keycloak"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 Keycloak is now running!${NC}"
echo ""
echo "📋 Next Steps:"
echo "1. Open Keycloak Admin Console: http://localhost:8080/admin"
echo "2. Login with: admin / admin"
echo "3. Follow the setup guide in KEYCLOAK_SETUP.md"
echo ""
echo "🔧 Quick Setup Commands:"
echo ""
echo "1. Create realm 'shortas-dev':"
echo "   - Go to http://localhost:8080/admin"
echo "   - Click 'Add realm' → Name: 'shortas-dev' → Create"
echo ""
echo "2. Create client 'shortas-api':"
echo "   - Go to Clients → Create"
echo "   - Client ID: 'shortas-api'"
echo "   - Client Protocol: 'openid-connect'"
echo "   - Access Type: 'confidential'"
echo "   - Standard Flow Enabled: ON"
echo "   - Service Accounts Enabled: ON"
echo "   - Valid Redirect URIs: 'http://localhost:5050/*'"
echo "   - Web Origins: 'http://localhost:5050'"
echo ""
echo "3. Create test user:"
echo "   - Go to Users → Add user"
echo "   - Username: 'testuser'"
echo "   - Email: 'test@shortas.com'"
echo "   - Set password in Credentials tab"
echo ""
echo "4. Update appsettings.json:"
echo "   - Replace 'YOUR_CLIENT_SECRET_HERE' with the actual client secret"
echo "   - You can find it in Clients → shortas-api → Credentials"
echo ""
echo "5. Test authentication:"
echo "   - Run: ./test-auth.sh"
echo ""
echo "🌐 Access URLs:"
echo "- Keycloak Admin: http://localhost:8080/admin"
echo "- API Swagger: http://localhost:5050/swagger/index.html"
echo "- API Health: http://localhost:5050/api/health"
echo ""
echo -e "${BLUE}For detailed instructions, see KEYCLOAK_SETUP.md${NC}"
