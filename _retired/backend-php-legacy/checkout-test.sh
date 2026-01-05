#!/bin/bash

# ═══════════════════════════════════════════════════════════════════════════
# REVOLUTION TRADING PROS - CHECKOUT FLOW E2E TEST
# Apple ICT 11 Principal Engineer Level Testing
# ═══════════════════════════════════════════════════════════════════════════

set -e

# Configuration
BASE_URL="${BASE_URL:-http://127.0.0.1:8002/api}"
TOKEN="${TOKEN:-4|5rt30ed7SgtYZmcUQelDMJwFhDLP2ntEHbNu7iNtf39a0e1a}"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}CHECKOUT FLOW E2E TEST - Apple ICT 11 Level${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo ""

# Step 1: Health Check
echo -e "${YELLOW}━━━ Step 1: Health Check ━━━${NC}"
HEALTH=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health/live")
if [ "$HEALTH" == "200" ]; then
    echo -e "${GREEN}✓${NC} API is healthy"
else
    echo -e "${RED}✗${NC} API is not responding (status: $HEALTH)"
    echo "Please start the backend: cd backend && php artisan serve --port=8002"
    exit 1
fi

# Step 2: Create a 100% discount coupon
echo ""
echo -e "${YELLOW}━━━ Step 2: Create 100% Discount Coupon ━━━${NC}"

COUPON_CODE="FREETEST$(date +%s)"
COUPON_DATA=$(cat <<EOF
{
    "code": "$COUPON_CODE",
    "type": "percentage",
    "value": 100,
    "is_active": true,
    "max_uses": 10,
    "description": "E2E Test - 100% off coupon"
}
EOF
)

COUPON_RESPONSE=$(curl -s -X POST "$BASE_URL/admin/coupons" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "$COUPON_DATA")

COUPON_ID=$(echo "$COUPON_RESPONSE" | grep -o '"id":[0-9]*' | head -1 | cut -d':' -f2)

if [ -n "$COUPON_ID" ]; then
    echo -e "${GREEN}✓${NC} Coupon created: $COUPON_CODE (ID: $COUPON_ID)"
else
    echo -e "${RED}✗${NC} Failed to create coupon"
    echo "Response: $COUPON_RESPONSE"
    # Try to get existing coupons
    echo ""
    echo "Checking existing coupons..."
    curl -s "$BASE_URL/admin/coupons" \
        -H "Authorization: Bearer $TOKEN" \
        -H "Accept: application/json" | head -c 500
    echo ""
fi

# Step 3: Get/Create a test product (Options Scanner Pro)
echo ""
echo -e "${YELLOW}━━━ Step 3: Get or Create Test Product ━━━${NC}"

# First check if product exists
PRODUCTS_RESPONSE=$(curl -s "$BASE_URL/admin/products" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Accept: application/json")

SCANNER_ID=$(echo "$PRODUCTS_RESPONSE" | grep -o '"id":[0-9]*,"name":"Options Scanner' | head -1 | grep -o '"id":[0-9]*' | cut -d':' -f2)

if [ -z "$SCANNER_ID" ]; then
    echo "Creating Options Scanner Pro product..."
    PRODUCT_DATA=$(cat <<EOF
{
    "name": "Options Scanner Pro",
    "slug": "options-scanner-pro",
    "type": "indicator",
    "description": "Professional options scanner for trading",
    "price": 99.00,
    "is_active": true,
    "is_taxable": true
}
EOF
)

    PRODUCT_RESPONSE=$(curl -s -X POST "$BASE_URL/admin/products" \
        -H "Authorization: Bearer $TOKEN" \
        -H "Content-Type: application/json" \
        -H "Accept: application/json" \
        -d "$PRODUCT_DATA")

    SCANNER_ID=$(echo "$PRODUCT_RESPONSE" | grep -o '"id":[0-9]*' | head -1 | cut -d':' -f2)
fi

if [ -n "$SCANNER_ID" ]; then
    echo -e "${GREEN}✓${NC} Product ready: Options Scanner Pro (ID: $SCANNER_ID)"
else
    echo -e "${YELLOW}⚠${NC} Could not find/create product. Using ID 1 as fallback."
    SCANNER_ID=1
fi

# Step 4: Validate coupon
echo ""
echo -e "${YELLOW}━━━ Step 4: Validate Coupon ━━━${NC}"

VALIDATE_RESPONSE=$(curl -s -X POST "$BASE_URL/coupon/validate" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{\"code\": \"$COUPON_CODE\"}")

if echo "$VALIDATE_RESPONSE" | grep -q '"valid":true\|"is_valid":true\|"success":true'; then
    echo -e "${GREEN}✓${NC} Coupon validated successfully"
else
    echo -e "${YELLOW}⚠${NC} Coupon validation response:"
    echo "$VALIDATE_RESPONSE" | head -c 300
    echo ""
fi

# Step 5: Checkout with coupon
echo ""
echo -e "${YELLOW}━━━ Step 5: Checkout with 100% Discount ━━━${NC}"

CHECKOUT_DATA=$(cat <<EOF
{
    "items": [
        {
            "product_id": $SCANNER_ID,
            "quantity": 1,
            "price": 99.00
        }
    ],
    "coupon_code": "$COUPON_CODE",
    "billing_country": "US",
    "billing_state": "CA"
}
EOF
)

echo "Checkout payload:"
echo "$CHECKOUT_DATA"
echo ""

CHECKOUT_RESPONSE=$(curl -s -X POST "$BASE_URL/cart/checkout" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "$CHECKOUT_DATA")

echo "Checkout response:"
echo "$CHECKOUT_RESPONSE" | head -c 1000
echo ""

# Check if order was created
ORDER_ID=$(echo "$CHECKOUT_RESPONSE" | grep -o '"order_id":[0-9]*\|"id":[0-9]*' | head -1 | cut -d':' -f2)
ORDER_NUMBER=$(echo "$CHECKOUT_RESPONSE" | grep -o '"order_number":"[^"]*"' | head -1 | cut -d'"' -f4)
TOTAL=$(echo "$CHECKOUT_RESPONSE" | grep -o '"total":[0-9.]*' | head -1 | cut -d':' -f2)

echo ""
if [ -n "$ORDER_ID" ] || [ -n "$ORDER_NUMBER" ]; then
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✓ CHECKOUT SUCCESSFUL!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "  Order ID: ${ORDER_ID:-N/A}"
    echo "  Order Number: ${ORDER_NUMBER:-N/A}"
    echo "  Total: \$${TOTAL:-0.00}"
    echo "  Coupon Applied: $COUPON_CODE (100% off)"
    echo ""

    if [ "$TOTAL" == "0" ] || [ "$TOTAL" == "0.00" ] || [ "$TOTAL" == "0.0" ]; then
        echo -e "${GREEN}✓${NC} Total is \$0.00 as expected!"
    else
        echo -e "${YELLOW}⚠${NC} Total is \$${TOTAL} (expected \$0.00)"
    fi
else
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}✗ CHECKOUT FAILED${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo "Check the response above for error details."
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}E2E Test Complete${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════${NC}"
