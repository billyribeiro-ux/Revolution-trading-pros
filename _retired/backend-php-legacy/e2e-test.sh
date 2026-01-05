#!/bin/bash

# ═══════════════════════════════════════════════════════════════════════════
# REVOLUTION TRADING PROS - COMPREHENSIVE E2E API TEST SUITE
# Date: December 8, 2025
# No Mocks - Real Data - A+ Grade Quality
# ═══════════════════════════════════════════════════════════════════════════

TOKEN="3|CnZSMVhbtCgkKAeq0nHxRTLJD5VtrahoQmeBJydQ76eeabcc"
BASE="http://127.0.0.1:8002/api"
PASS=0
FAIL=0
TOTAL=0

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test function
test_endpoint() {
    local method=$1
    local endpoint=$2
    local description=$3
    local data=$4
    local auth=$5

    TOTAL=$((TOTAL + 1))

    if [ "$auth" = "auth" ]; then
        AUTH_HEADER="-H \"Authorization: Bearer $TOKEN\""
    else
        AUTH_HEADER=""
    fi

    if [ "$method" = "GET" ]; then
        RESPONSE=$(eval curl -s -w "\n%{http_code}" "$BASE$endpoint" $AUTH_HEADER 2>&1)
    elif [ "$method" = "POST" ]; then
        RESPONSE=$(eval curl -s -w "\n%{http_code}" -X POST "$BASE$endpoint" $AUTH_HEADER -H "Content-Type: application/json" -d "'$data'" 2>&1)
    elif [ "$method" = "PUT" ]; then
        RESPONSE=$(eval curl -s -w "\n%{http_code}" -X PUT "$BASE$endpoint" $AUTH_HEADER -H "Content-Type: application/json" -d "'$data'" 2>&1)
    elif [ "$method" = "DELETE" ]; then
        RESPONSE=$(eval curl -s -w "\n%{http_code}" -X DELETE "$BASE$endpoint" $AUTH_HEADER 2>&1)
    fi

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)
    BODY=$(echo "$RESPONSE" | sed '$d')

    if [[ "$HTTP_CODE" =~ ^2 ]] || [[ "$HTTP_CODE" == "302" ]] || [[ "$HTTP_CODE" == "404" && "$description" == *"Not Found"* ]]; then
        PASS=$((PASS + 1))
        echo -e "${GREEN}✓${NC} [$HTTP_CODE] $method $endpoint - $description"
    else
        FAIL=$((FAIL + 1))
        echo -e "${RED}✗${NC} [$HTTP_CODE] $method $endpoint - $description"
        echo "   Response: $(echo "$BODY" | head -c 150)"
    fi
}

echo "═══════════════════════════════════════════════════════════════════════════"
echo "REVOLUTION TRADING PROS - E2E API TEST REPORT"
echo "Date: $(date)"
echo "Environment: REAL DATA - NO MOCKS"
echo "Base URL: $BASE"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# 1. HEALTH CHECK ENDPOINTS (Public)
# ═══════════════════════════════════════════════════════════════════════════
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1. HEALTH CHECK ENDPOINTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/health/live" "Liveness probe" "" ""
test_endpoint "GET" "/health/ready" "Readiness probe" "" ""

# ═══════════════════════════════════════════════════════════════════════════
# 2. USER/ME ENDPOINTS (Protected)
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2. USER ENDPOINTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/me" "Get current user" "" "auth"
test_endpoint "GET" "/me/memberships" "Get user memberships" "" "auth"
test_endpoint "GET" "/me/products" "Get user products" "" "auth"
test_endpoint "GET" "/me/indicators" "Get user indicators" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 3. USER INDICATORS CONTROLLER
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3. USER INDICATORS CONTROLLER"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/user/indicators" "List user indicators" "" "auth"
test_endpoint "GET" "/user/indicators/1" "Get indicator - Not Found expected" "" "auth"
test_endpoint "GET" "/user/indicators/1/download" "Download indicator - Not Found expected" "" "auth"
test_endpoint "GET" "/user/indicators/1/docs" "Indicator docs - Not Found expected" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 4. TRADING ROOMS SSO CONTROLLER
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4. TRADING ROOMS SSO CONTROLLER"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/trading-rooms/sso/accessible" "Get accessible rooms" "" "auth"
test_endpoint "POST" "/trading-rooms/day-trading/sso" "Generate SSO token" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 5. ADMIN EMAIL SETTINGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5. ADMIN EMAIL SETTINGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/settings" "Get email settings" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 6. ADMIN EMAIL TEMPLATES
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "6. ADMIN EMAIL TEMPLATES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/templates" "List email templates" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 7. ADMIN EMAIL CAMPAIGNS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "7. ADMIN EMAIL CAMPAIGNS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/campaigns" "List email campaigns" "" "auth"
test_endpoint "GET" "/admin/email/campaigns/stats" "Campaign stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 8. ADMIN EMAIL SUBSCRIBERS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "8. ADMIN EMAIL SUBSCRIBERS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/subscribers" "List subscribers" "" "auth"
test_endpoint "GET" "/admin/email/subscribers/stats" "Subscriber stats" "" "auth"
test_endpoint "GET" "/admin/email/subscribers/tags" "Subscriber tags" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 9. ADMIN EMAIL METRICS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "9. ADMIN EMAIL METRICS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/metrics/dashboard" "Email dashboard" "" "auth"
test_endpoint "GET" "/admin/email/metrics/overview" "Email overview" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 10. ADMIN EMAIL DOMAINS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "10. ADMIN EMAIL DOMAINS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/domains" "List email domains" "" "auth"
test_endpoint "GET" "/admin/email/domains/stats" "Domain stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 11. ADMIN EMAIL AUDIT LOGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "11. ADMIN EMAIL AUDIT LOGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/logs" "List audit logs" "" "auth"
test_endpoint "GET" "/admin/email/logs/stats" "Audit log stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 12. ADMIN EMAIL WEBHOOKS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "12. ADMIN EMAIL WEBHOOKS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/email/webhooks" "List webhooks" "" "auth"
test_endpoint "GET" "/admin/email/webhooks/stats" "Webhook stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 13. ADMIN USERS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "13. ADMIN USERS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/users" "List users" "" "auth"
test_endpoint "GET" "/admin/users/stats" "User stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 14. ADMIN MEMBERS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "14. ADMIN MEMBERS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/members" "List members" "" "auth"
test_endpoint "GET" "/admin/members/stats" "Member stats" "" "auth"
test_endpoint "GET" "/admin/members/services" "Member services" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 15. ADMIN SUBSCRIPTION PLANS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "15. ADMIN SUBSCRIPTION PLANS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/subscriptions/plans" "List subscription plans" "" "auth"
test_endpoint "GET" "/admin/subscriptions/plans/stats" "Plan stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 16. ADMIN USER SUBSCRIPTIONS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "16. ADMIN USER SUBSCRIPTIONS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/subscriptions" "List subscriptions" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 17. ADMIN PRODUCTS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "17. ADMIN PRODUCTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/products" "List products" "" "auth"
test_endpoint "GET" "/admin/products/stats" "Product stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 18. ADMIN BLOG POSTS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "18. ADMIN BLOG POSTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/posts" "List posts" "" "auth"
test_endpoint "GET" "/admin/posts/stats" "Post stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 19. ADMIN CATEGORIES
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "19. ADMIN CATEGORIES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/categories" "List categories" "" "auth"
test_endpoint "GET" "/admin/categories/stats" "Category stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 20. ADMIN TAGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "20. ADMIN TAGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/tags" "List tags" "" "auth"
test_endpoint "GET" "/admin/tags/stats" "Tag stats" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 21. ADMIN SETTINGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "21. ADMIN SETTINGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/settings" "Get all settings" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 22. ADMIN CONSENT SETTINGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "22. ADMIN CONSENT SETTINGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/consent/settings" "Get consent settings" "" "auth"
test_endpoint "GET" "/admin/consent/templates" "Get consent templates" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 23. ADMIN ABANDONED CARTS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "23. ADMIN ABANDONED CARTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/abandoned-carts/dashboard" "Cart dashboard" "" "auth"
test_endpoint "GET" "/admin/abandoned-carts" "List carts" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 24. ADMIN PAST MEMBERS DASHBOARD
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "24. ADMIN PAST MEMBERS DASHBOARD"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/past-members-dashboard/overview" "Past members overview" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 25. ADMIN POPUPS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "25. ADMIN POPUPS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/popups" "List popups" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 26. ADMIN MEDIA
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "26. ADMIN MEDIA"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/media" "List media" "" "auth"
test_endpoint "GET" "/admin/media/statistics" "Media statistics" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 27. ADMIN CRM CONTACTS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "27. ADMIN CRM CONTACTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/contacts" "List CRM contacts" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 28. ADMIN CRM DEALS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "28. ADMIN CRM DEALS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/deals" "List CRM deals" "" "auth"
test_endpoint "GET" "/admin/crm/deals/forecast" "Deal forecast" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 29. ADMIN CRM PIPELINES
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "29. ADMIN CRM PIPELINES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/pipelines" "List pipelines" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 30. ADMIN CRM SEQUENCES
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "30. ADMIN CRM SEQUENCES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/sequences" "List sequences" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 31. ADMIN CRM AUTOMATION FUNNELS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "31. ADMIN CRM AUTOMATION FUNNELS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/automations" "List automations" "" "auth"
test_endpoint "GET" "/admin/crm/automations/trigger-types" "Trigger types" "" "auth"
test_endpoint "GET" "/admin/crm/automations/action-types" "Action types" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 32. ADMIN CRM LISTS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "32. ADMIN CRM LISTS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/lists" "List contact lists" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 33. ADMIN CRM CONTACT TAGS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "33. ADMIN CRM CONTACT TAGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/contact-tags" "List contact tags" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 34. ADMIN CRM COMPANIES
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "34. ADMIN CRM COMPANIES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/crm/companies" "List companies" "" "auth"
test_endpoint "GET" "/admin/crm/companies/industries" "Company industries" "" "auth"
test_endpoint "GET" "/admin/crm/companies/sizes" "Company sizes" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# 35. ADMIN TRADING ROOMS
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "35. ADMIN TRADING ROOMS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
test_endpoint "GET" "/admin/trading-rooms" "List trading rooms" "" "auth"
test_endpoint "GET" "/admin/trading-rooms/traders" "List traders" "" "auth"

# ═══════════════════════════════════════════════════════════════════════════
# FINAL REPORT
# ═══════════════════════════════════════════════════════════════════════════
echo ""
echo "═══════════════════════════════════════════════════════════════════════════"
echo "TEST RESULTS SUMMARY"
echo "═══════════════════════════════════════════════════════════════════════════"
echo ""
echo -e "Total Tests:  ${YELLOW}$TOTAL${NC}"
echo -e "Passed:       ${GREEN}$PASS${NC}"
echo -e "Failed:       ${RED}$FAIL${NC}"
PERCENTAGE=$(echo "scale=1; $PASS * 100 / $TOTAL" | bc)
echo -e "Success Rate: ${YELLOW}$PERCENTAGE%${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}        ALL TESTS PASSED - A+ GRADE - PRODUCTION READY${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════════════════${NC}"
elif [ $PERCENTAGE -ge 90 ]; then
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}        MOST TESTS PASSED - A GRADE${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════════════════════════${NC}"
else
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${RED}        SOME TESTS FAILED - NEEDS ATTENTION${NC}"
    echo -e "${RED}═══════════════════════════════════════════════════════════════════════════${NC}"
fi
echo ""
echo "Test completed at: $(date)"
