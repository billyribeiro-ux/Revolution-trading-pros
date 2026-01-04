#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
# End-to-End API Tests - Apple ICT 11+ Principal Engineer
# Evidence-based testing with documented results
# ═══════════════════════════════════════════════════════════════════════════

set -e

# Configuration
API_BASE="${API_BASE:-https://revolution-trading-pros-api.fly.dev}"
OUTPUT_DIR="./test_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${OUTPUT_DIR}/e2e_results_${TIMESTAMP}.md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
TOTAL=0

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Initialize results file
cat > "$RESULTS_FILE" << EOF
# End-to-End Test Results
**API Base:** $API_BASE  
**Timestamp:** $(date)  
**Test Suite:** ICT 11+ Principal Engineer Evidence-Based Testing

---

## Test Results

EOF

# ═══════════════════════════════════════════════════════════════════════════
# Test Functions
# ═══════════════════════════════════════════════════════════════════════════

test_endpoint() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local expected_status="$4"
    local data="$5"
    
    TOTAL=$((TOTAL + 1))
    
    echo -n "Testing: $name... "
    
    # Build curl command
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "\n%{http_code}" -X GET "${API_BASE}${endpoint}" \
            -H "Accept: application/json" \
            -H "Content-Type: application/json" 2>&1)
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "${API_BASE}${endpoint}" \
            -H "Accept: application/json" \
            -H "Content-Type: application/json" \
            -d "$data" 2>&1)
    fi
    
    # Extract status code (last line)
    status_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    
    # Check result
    if [ "$status_code" = "$expected_status" ]; then
        echo -e "${GREEN}PASSED${NC} (HTTP $status_code)"
        PASSED=$((PASSED + 1))
        result="✅ PASSED"
    else
        echo -e "${RED}FAILED${NC} (Expected $expected_status, got $status_code)"
        FAILED=$((FAILED + 1))
        result="❌ FAILED"
    fi
    
    # Log to results file
    cat >> "$RESULTS_FILE" << EOF
### $name
- **Endpoint:** \`$method $endpoint\`
- **Expected Status:** $expected_status
- **Actual Status:** $status_code
- **Result:** $result

<details>
<summary>Response Body</summary>

\`\`\`json
$body
\`\`\`
</details>

---

EOF
}

# ═══════════════════════════════════════════════════════════════════════════
# Health Check Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  ICT 11+ End-to-End API Tests"
echo "  API: $API_BASE"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

echo "--- Health Check ---"
test_endpoint "Health Check" "GET" "/health" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Robots.txt Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Robots.txt (P0 Conversion) ---"
test_endpoint "Robots.txt" "GET" "/robots.txt" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Sitemap Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Sitemap (P0 Conversion) ---"
test_endpoint "Sitemap Index" "GET" "/sitemap" "200"
test_endpoint "Sitemap Categories" "GET" "/sitemap/categories" "200"
test_endpoint "Sitemap Tags" "GET" "/sitemap/tags" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Categories Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Categories (P0 Conversion) ---"
test_endpoint "List Categories" "GET" "/admin/categories" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Tags Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Tags (P0 Conversion) ---"
test_endpoint "List Tags" "GET" "/admin/tags" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Redirects Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Redirects (P0 Conversion) ---"
test_endpoint "List Redirects" "GET" "/redirects" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Settings Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Settings (P1 Conversion) ---"
test_endpoint "List Settings" "GET" "/admin/settings" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Existing Rust API Tests
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Existing Rust Endpoints ---"
test_endpoint "List Posts" "GET" "/posts" "200"
test_endpoint "List Products" "GET" "/products" "200"
test_endpoint "List Indicators" "GET" "/indicators" "200"
test_endpoint "Search" "GET" "/search?q=trading" "200"

# ═══════════════════════════════════════════════════════════════════════════
# Auth Endpoints (Existing)
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "--- Auth Endpoints ---"
test_endpoint "Auth Me (Unauthenticated)" "GET" "/auth/me" "401"

# ═══════════════════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "  Test Summary"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo -e "  Total:  $TOTAL"
echo -e "  ${GREEN}Passed: $PASSED${NC}"
echo -e "  ${RED}Failed: $FAILED${NC}"
echo ""

# Add summary to results file
cat >> "$RESULTS_FILE" << EOF

## Summary

| Metric | Count |
|--------|-------|
| Total Tests | $TOTAL |
| Passed | $PASSED |
| Failed | $FAILED |
| Pass Rate | $(echo "scale=1; $PASSED * 100 / $TOTAL" | bc)% |

---

**Test completed at:** $(date)
EOF

echo "Results saved to: $RESULTS_FILE"

# Exit with error if any tests failed
if [ $FAILED -gt 0 ]; then
    exit 1
fi
