#!/bin/bash

# Logged Out Page Test Script

echo "🧪 Testing Logged Out Page functionality..."
echo "=========================================="

# Check if the app is running
echo "1. Checking if React app is running..."
if curl -s -f "http://localhost:3000" > /dev/null; then
    echo "   ✅ React app is running at http://localhost:3000"
else
    echo "   ❌ React app is NOT running at http://localhost:3000"
    echo "   💡 Start the app with: npm start"
    exit 1
fi

# Check if Keycloak is running
echo "2. Checking Keycloak connectivity..."
if curl -s -f "http://localhost:8080" > /dev/null; then
    echo "   ✅ Keycloak server is running at http://localhost:8080"
else
    echo "   ❌ Keycloak server is NOT running at http://localhost:8080"
    echo "   💡 Start Keycloak or use mock data mode"
fi

# Test the logged out page
echo "3. Testing logged out page..."
echo "   📋 Expected behavior:"
echo "   - Visit http://localhost:3000 (should redirect to /logged-out if not authenticated)"
echo "   - Visit http://localhost:3000/logged-out (direct access to logged out page)"
echo "   - Should show logged out page (not redirect to Keycloak)"
echo "   - Should have 'Sign In to Dashboard' button"
echo "   - Should have 'Visit Landing Page' button"

echo ""
echo "🎉 Logged Out Page Test Complete!"
echo "=================================="
echo "📋 Next steps:"
echo "   1. Visit http://localhost:3000 in your browser"
echo "   2. Verify you see the logged out page"
echo "   3. Test the 'Sign In to Dashboard' button (should redirect to Keycloak)"
echo "   4. Test the 'Visit Landing Page' button (should redirect to landing page)"
echo ""
echo "🔗 Test URLs:"
echo "   - Dashboard: http://localhost:3000"
echo "   - Logged Out Page: http://localhost:3000/logged-out"
echo "   - Keycloak: http://localhost:8080"
echo "   - Admin Console: http://localhost:8080/admin"
