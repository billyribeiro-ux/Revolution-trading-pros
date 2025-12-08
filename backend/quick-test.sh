#!/bin/bash

TOKEN="4|5rt30ed7SgtYZmcUQelDMJwFhDLP2ntEHbNu7iNtf39a0e1a"
BASE_URL="http://127.0.0.1:8002"

echo "=== Testing Previously Failing Endpoints ==="
echo ""

test_endpoint() {
    local name="$1"
    local endpoint="$2"
    local status

    status=$(curl -s -o /dev/null -w "%{http_code}" -H "Authorization: Bearer $TOKEN" -H "Accept: application/json" "$BASE_URL$endpoint")

    if [ "$status" -ge 200 ] && [ "$status" -lt 300 ]; then
        echo "✅ PASS ($status): $name"
    elif [ "$status" == "404" ]; then
        echo "⚠️ 404: $name (endpoint or resource not found)"
    else
        echo "❌ FAIL ($status): $name"
        curl -s -H "Authorization: Bearer $TOKEN" -H "Accept: application/json" "$BASE_URL$endpoint" | head -c 200
        echo ""
    fi
}

# Previously failing endpoints
test_endpoint "User Indicators" "/api/user/indicators"
test_endpoint "Trading Rooms SSO Accessible" "/api/trading-rooms/sso/accessible"
test_endpoint "Admin Users Stats" "/api/admin/users/stats"
test_endpoint "Admin Members" "/api/admin/members"
test_endpoint "Admin Members Stats" "/api/admin/members/stats"
test_endpoint "Admin Subscriptions" "/api/admin/subscriptions"
test_endpoint "Admin Categories" "/api/admin/categories"
test_endpoint "Admin Products" "/api/admin/products"
test_endpoint "Admin Popups" "/api/admin/popups"
test_endpoint "Abandoned Carts Dashboard" "/api/admin/abandoned-carts/dashboard"

echo ""
echo "=== All Previously Passing Endpoints ==="
echo ""

# Health & Auth
test_endpoint "Health Live" "/api/health/live"
test_endpoint "Health Ready" "/api/health/ready"
test_endpoint "API Me" "/api/me"

# Email System
test_endpoint "Email Settings" "/api/admin/email/settings"
test_endpoint "Email Templates" "/api/admin/email/templates"
test_endpoint "Email Campaigns" "/api/admin/email/campaigns"
test_endpoint "Email Campaigns Stats" "/api/admin/email/campaigns/stats"
test_endpoint "Email Subscribers" "/api/admin/email/subscribers"
test_endpoint "Email Subscribers Stats" "/api/admin/email/subscribers/stats"
test_endpoint "Email Metrics Dashboard" "/api/admin/email/metrics/dashboard"
test_endpoint "Email Domains" "/api/admin/email/domains"
test_endpoint "Email Domains Stats" "/api/admin/email/domains/stats"
test_endpoint "Email Logs" "/api/admin/email/logs"
test_endpoint "Email Webhooks" "/api/admin/email/webhooks"
test_endpoint "Email Webhooks Stats" "/api/admin/email/webhooks/stats"

# Admin Users
test_endpoint "Admin Users" "/api/admin/users"

# Subscriptions
test_endpoint "Subscription Plans" "/api/admin/subscriptions/plans"
test_endpoint "Subscription Plans Stats" "/api/admin/subscriptions/plans/stats"

# Content Management
test_endpoint "Admin Posts" "/api/admin/posts"
test_endpoint "Admin Posts Stats" "/api/admin/posts/stats"
test_endpoint "Admin Tags" "/api/admin/tags"
test_endpoint "Admin Tags Stats" "/api/admin/tags/stats"
test_endpoint "Admin Media" "/api/admin/media"

# Settings
test_endpoint "Admin Settings" "/api/admin/settings"
test_endpoint "Consent Settings" "/api/admin/consent/settings"
test_endpoint "Consent Templates" "/api/admin/consent/templates"

# CRM
test_endpoint "CRM Contacts" "/api/admin/crm/contacts"
test_endpoint "CRM Deals" "/api/admin/crm/deals"
test_endpoint "CRM Pipelines" "/api/admin/crm/pipelines"
test_endpoint "CRM Sequences" "/api/admin/crm/sequences"
test_endpoint "CRM Automations" "/api/admin/crm/automations"
test_endpoint "CRM Automation Trigger Types" "/api/admin/crm/automations/trigger-types"
test_endpoint "CRM Automation Action Types" "/api/admin/crm/automations/action-types"
test_endpoint "CRM Lists" "/api/admin/crm/lists"
test_endpoint "CRM Contact Tags" "/api/admin/crm/contact-tags"
test_endpoint "CRM Companies" "/api/admin/crm/companies"
test_endpoint "CRM Companies Industries" "/api/admin/crm/companies/industries"
test_endpoint "CRM Companies Sizes" "/api/admin/crm/companies/sizes"

# Trading Rooms
test_endpoint "Admin Trading Rooms" "/api/admin/trading-rooms"
test_endpoint "Admin Trading Rooms Traders" "/api/admin/trading-rooms/traders"

# Abandoned Carts
test_endpoint "Abandoned Carts" "/api/admin/abandoned-carts"

echo ""
echo "=== Test Complete ==="
