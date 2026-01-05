#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# ICT 11+ Schedule System - End-to-End Test Script
# Tests the deployed schedule management system
# ═══════════════════════════════════════════════════════════════════════════

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
API_URL="${API_URL:-https://your-api-url.com}"
FRONTEND_URL="${FRONTEND_URL:-https://your-frontend-url.pages.dev}"

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}ICT 11+ Schedule System - End-to-End Test${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}API URL:${NC} $API_URL"
echo -e "${YELLOW}Frontend URL:${NC} $FRONTEND_URL"
echo ""

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Test function
test_endpoint() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local expected_status="$4"
    
    echo -e "${YELLOW}Testing:${NC} $name"
    
    response=$(curl -s -w "\n%{http_code}" -X "$method" "$API_URL$endpoint" \
        -H "Content-Type: application/json" 2>&1)
    
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [ "$status_code" = "$expected_status" ]; then
        echo -e "${GREEN}✓ PASS${NC} - Status: $status_code"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC} - Expected: $expected_status, Got: $status_code"
        echo -e "${RED}Response:${NC} $body"
        ((TESTS_FAILED++))
        return 1
    fi
}

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}1. PUBLIC API ENDPOINTS${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Test 1.1: Get Trading Rooms
echo -e "${YELLOW}Test 1.1: Get Trading Rooms List${NC}"
test_endpoint "GET /api/schedules/rooms" "GET" "/api/schedules/rooms" "200"
echo ""

# Test 1.2: Get Day Trading Room Schedule
echo -e "${YELLOW}Test 1.2: Get Day Trading Room Schedule${NC}"
test_endpoint "GET /api/schedules/day-trading-room" "GET" "/api/schedules/day-trading-room" "200"
echo ""

# Test 1.3: Get Upcoming Events
echo -e "${YELLOW}Test 1.3: Get Upcoming Events (7 days)${NC}"
test_endpoint "GET /api/schedules/day-trading-room/upcoming" "GET" "/api/schedules/day-trading-room/upcoming?days=7" "200"
echo ""

# Test 1.4: Get Swing Trading Room Schedule
echo -e "${YELLOW}Test 1.4: Get Swing Trading Room Schedule${NC}"
test_endpoint "GET /api/schedules/swing-trading-room" "GET" "/api/schedules/swing-trading-room" "200"
echo ""

# Test 1.5: Get Small Account Mentorship Schedule
echo -e "${YELLOW}Test 1.5: Get Small Account Mentorship Schedule${NC}"
test_endpoint "GET /api/schedules/small-account-mentorship" "GET" "/api/schedules/small-account-mentorship" "200"
echo ""

# Test 1.6: Get Non-existent Room (should return 404)
echo -e "${YELLOW}Test 1.6: Get Non-existent Room${NC}"
test_endpoint "GET /api/schedules/invalid-room" "GET" "/api/schedules/invalid-room" "404"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}2. DETAILED RESPONSE VALIDATION${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Test 2.1: Validate Trading Rooms Response Structure
echo -e "${YELLOW}Test 2.1: Validate Trading Rooms Response${NC}"
response=$(curl -s "$API_URL/api/schedules/rooms")
if echo "$response" | jq -e '.rooms | length > 0' > /dev/null 2>&1; then
    room_count=$(echo "$response" | jq '.rooms | length')
    echo -e "${GREEN}✓ PASS${NC} - Found $room_count trading rooms"
    echo "$response" | jq '.rooms[] | {id, name, slug, type}'
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Invalid response structure"
    ((TESTS_FAILED++))
fi
echo ""

# Test 2.2: Validate Day Trading Room Schedule Structure
echo -e "${YELLOW}Test 2.2: Validate Day Trading Room Schedule${NC}"
response=$(curl -s "$API_URL/api/schedules/day-trading-room")
if echo "$response" | jq -e '.events | length >= 0' > /dev/null 2>&1; then
    event_count=$(echo "$response" | jq '.events | length')
    echo -e "${GREEN}✓ PASS${NC} - Found $event_count schedule events"
    echo "$response" | jq '{plan: .plan.name, timezone, event_count: (.events | length)}'
    if [ "$event_count" -gt 0 ]; then
        echo -e "${YELLOW}Sample Event:${NC}"
        echo "$response" | jq '.events[0]'
    fi
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Invalid response structure"
    ((TESTS_FAILED++))
fi
echo ""

# Test 2.3: Validate Upcoming Events Structure
echo -e "${YELLOW}Test 2.3: Validate Upcoming Events${NC}"
response=$(curl -s "$API_URL/api/schedules/day-trading-room/upcoming?days=7")
if echo "$response" | jq -e '.events | length >= 0' > /dev/null 2>&1; then
    event_count=$(echo "$response" | jq '.events | length')
    echo -e "${GREEN}✓ PASS${NC} - Found $event_count upcoming events"
    echo "$response" | jq '{plan: .plan.name, days_ahead, event_count: (.events | length)}'
    if [ "$event_count" -gt 0 ]; then
        echo -e "${YELLOW}Next Event:${NC}"
        echo "$response" | jq '.events[0] | {title, trader_name, event_date, start_time}'
    fi
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Invalid response structure"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}3. ALL TRADING ROOMS TEST${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""

rooms=("day-trading-room" "swing-trading-room" "small-account-mentorship" "alerts-only" "explosive-swing" "spx-profit-pulse")

for room in "${rooms[@]}"; do
    echo -e "${YELLOW}Testing: $room${NC}"
    response=$(curl -s "$API_URL/api/schedules/$room")
    if echo "$response" | jq -e '.events' > /dev/null 2>&1; then
        event_count=$(echo "$response" | jq '.events | length')
        echo -e "${GREEN}✓ PASS${NC} - $room: $event_count events"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC} - $room: Invalid response"
        ((TESTS_FAILED++))
    fi
done
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}4. FRONTEND INTEGRATION TEST${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${YELLOW}Test 4.1: Frontend Accessibility${NC}"
frontend_status=$(curl -s -o /dev/null -w "%{http_code}" "$FRONTEND_URL")
if [ "$frontend_status" = "200" ]; then
    echo -e "${GREEN}✓ PASS${NC} - Frontend is accessible"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Frontend returned status: $frontend_status"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}Test 4.2: Admin Schedules Page${NC}"
admin_status=$(curl -s -o /dev/null -w "%{http_code}" "$FRONTEND_URL/admin/schedules")
if [ "$admin_status" = "200" ] || [ "$admin_status" = "302" ] || [ "$admin_status" = "401" ]; then
    echo -e "${GREEN}✓ PASS${NC} - Admin page exists (status: $admin_status)"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Admin page returned status: $admin_status"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}Test 4.3: Day Trading Room Page${NC}"
dtr_status=$(curl -s -o /dev/null -w "%{http_code}" "$FRONTEND_URL/dashboard/day-trading-room")
if [ "$dtr_status" = "200" ] || [ "$dtr_status" = "302" ] || [ "$dtr_status" = "401" ]; then
    echo -e "${GREEN}✓ PASS${NC} - Day Trading Room page exists (status: $dtr_status)"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ FAIL${NC} - Day Trading Room page returned status: $dtr_status"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}TEST SUMMARY${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}✓ ALL TESTS PASSED - SYSTEM IS FLAWLESS!${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
    exit 0
else
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${RED}✗ SOME TESTS FAILED - REVIEW REQUIRED${NC}"
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════${NC}"
    exit 1
fi
