#!/bin/bash

# Keycloak Connectivity Test Script

echo "🔍 Testing Keycloak connectivity..."
echo "=================================="

KEYCLOAK_URL="http://localhost:8080"
REALM="shortas-dev"

echo "1. Testing Keycloak server..."
if curl -s -f "$KEYCLOAK_URL" > /dev/null; then
    echo "   ✅ Keycloak server is running at $KEYCLOAK_URL"
else
    echo "   ❌ Keycloak server is NOT running at $KEYCLOAK_URL"
    echo "   💡 Solution: Start Keycloak server or use mock data mode"
    echo "   💡 Mock mode: Set REACT_APP_USE_MOCK_DATA=true in .env.local"
    exit 1
fi

echo "2. Testing realm configuration..."
if curl -s -f "$KEYCLOAK_URL/realms/$REALM/.well-known/openid_configuration" > /dev/null; then
    echo "   ✅ Realm '$REALM' exists and is configured"
else
    echo "   ❌ Realm '$REALM' does NOT exist"
    echo "   💡 Solution: Create realm in Keycloak admin console or use mock data mode"
    exit 1
fi

echo "3. Testing admin console..."
if curl -s -f "$KEYCLOAK_URL/admin" > /dev/null; then
    echo "   ✅ Admin console is accessible at $KEYCLOAK_URL/admin"
else
    echo "   ❌ Admin console is NOT accessible"
fi

echo ""
echo "🎉 Keycloak is properly configured!"
echo "📋 Next steps:"
echo "   1. Create client 'shortas-dashboard' in the admin console"
echo "   2. Set REACT_APP_USE_MOCK_DATA=false in .env.local"
echo "   3. Start your React app: npm start"
echo ""
echo "🔗 Admin Console: $KEYCLOAK_URL/admin"
echo "🔗 Realm Config: $KEYCLOAK_URL/realms/$REALM/.well-known/openid_configuration"

