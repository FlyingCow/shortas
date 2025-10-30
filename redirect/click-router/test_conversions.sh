#!/bin/bash

# Test script for conversion tracking endpoints
# This script tests the conversion tracking functionality in click-router

echo "🧪 Testing Conversion Tracking Endpoints"
echo "========================================"

# Configuration
CLICK_ROUTER_URL="https://localhost:5800"
CONVERSION_ENDPOINT="$CLICK_ROUTER_URL/conversions/track"
FUNNEL_ENDPOINT="$CLICK_ROUTER_URL/conversions/funnel"

# Test data
ROUTE_ID="test-route-123"
CONVERSION_TYPE="purchase"
CONVERSION_NAME="Test Purchase"
CONVERSION_VALUE=99.99
USER_ID="test-user-456"
SESSION_ID="test-session-789"

echo "📡 Testing Conversion Tracking..."
echo "Endpoint: $CONVERSION_ENDPOINT"

# Test conversion tracking
curl -k -X POST "$CONVERSION_ENDPOINT" \
  -H "Content-Type: application/json" \
  -H "User-Agent: Mozilla/5.0 (Test Browser)" \
  -d "{
    \"route_id\": \"$ROUTE_ID\",
    \"conversion_type\": \"$CONVERSION_TYPE\",
    \"conversion_name\": \"$CONVERSION_NAME\",
    \"conversion_value\": $CONVERSION_VALUE,
    \"attributed_click_id\": \"click-123\",
    \"attribution_type\": \"direct\",
    \"attribution_window_hours\": 24,
    \"user_id\": \"$USER_ID\",
    \"session_id\": \"$SESSION_ID\",
    \"metadata\": {
      \"product_id\": \"prod-123\",
      \"category\": \"electronics\",
      \"test\": true
    }
  }" \
  -w "\nHTTP Status: %{http_code}\n" \
  -s

echo ""
echo "📊 Testing Funnel Step Tracking..."
echo "Endpoint: $FUNNEL_ENDPOINT"

# Test funnel step tracking
curl -k -X POST "$FUNNEL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -H "User-Agent: Mozilla/5.0 (Test Browser)" \
  -d "{
    \"route_id\": \"$ROUTE_ID\",
    \"funnel_name\": \"E-commerce Purchase Funnel\",
    \"funnel_steps\": [\"view\", \"add_to_cart\", \"checkout\", \"purchase\"],
    \"step_name\": \"add_to_cart\",
    \"step_position\": 2,
    \"step_value\": $CONVERSION_VALUE,
    \"user_id\": \"$USER_ID\",
    \"session_id\": \"$SESSION_ID\",
    \"metadata\": {
      \"product_id\": \"prod-123\",
      \"test\": true
    }
  }" \
  -w "\nHTTP Status: %{http_code}\n" \
  -s

echo ""
echo "✅ Test completed!"
echo ""
echo "📝 Expected Results:"
echo "- Both requests should return HTTP 201 (Created)"
echo "- Response should include success: true"
echo "- Conversion/funnel step should be tracked in the pipeline"
echo ""
echo "🔍 To verify the data flow:"
echo "1. Check click-router logs for conversion processing"
echo "2. Check Fluvio for queued messages"
echo "3. Check click-tracker logs for enrichment"
echo "4. Check click-aggregator logs for storage"
echo "5. Check ClickHouse for stored conversion data"
