#!/bin/bash

# Image Service v3.0 - End-to-End Live Testing

BASE_URL="http://localhost:3001"
PASSED=0
FAILED=0

test_get() {
  local name="$1"
  local endpoint="$2"

  status=$(curl -s -o /dev/null -w "%{http_code}" -X GET "$BASE_URL$endpoint" 2>/dev/null)

  if [ "$status" = "200" ]; then
    echo "✓ $name (HTTP $status)"
    PASSED=$((PASSED + 1))
  else
    echo "✗ $name (HTTP $status)"
    FAILED=$((FAILED + 1))
  fi
}

test_post() {
  local name="$1"
  local endpoint="$2"
  local body="$3"

  status=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL$endpoint" \
    -H "Content-Type: application/json" \
    -d "$body" 2>/dev/null)

  if [ "$status" = "200" ] || [ "$status" = "201" ]; then
    echo "✓ $name (HTTP $status)"
    PASSED=$((PASSED + 1))
  else
    echo "✗ $name (HTTP $status)"
    FAILED=$((FAILED + 1))
  fi
}

echo "═══════════════════════════════════════════════════════════════"
echo "  Image Service v3.0 - End-to-End Live Testing"
echo "═══════════════════════════════════════════════════════════════"
echo ""

echo "▶ CORE SERVICE TESTS"
echo "───────────────────────────────────────────────────────────────"
test_get "Health Check" "/health"
test_get "Service Stats" "/stats"
test_get "Capabilities" "/capabilities"
echo ""

echo "▶ MAINTENANCE SERVICE TESTS"
echo "───────────────────────────────────────────────────────────────"
test_get "Maintenance Status" "/maintenance/status"
test_post "Run Maintenance" "/maintenance/run" "{}"
test_post "Cleanup Temp Task" "/maintenance/task/cleanup-temp" "{}"
test_post "Cleanup Orphans Task" "/maintenance/task/cleanup-orphans" "{}"
test_post "Validate Storage Task" "/maintenance/task/validate-storage" "{}"
echo ""

echo "▶ FORMAT DETECTION TESTS"
echo "───────────────────────────────────────────────────────────────"
test_post "Detect AVIF Support" "/format/detect" '{"acceptHeader":"image/avif,image/webp,*/*"}'
test_post "Detect WebP Support" "/format/detect" '{"acceptHeader":"image/webp,*/*"}'
test_post "Detect by User-Agent" "/format/detect" '{"userAgent":"Mozilla/5.0 Chrome/120.0.0.0"}'
test_post "Picture Sources" "/format/picture-sources" '{"url":"/test.jpg","sizes":[320,640,1024]}'
test_get "Detect Format (GET)" "/format/detect"
echo ""

echo "▶ CDN RULES TESTS"
echo "───────────────────────────────────────────────────────────────"
test_get "List Profiles" "/cdn/profiles"
test_get "Web Profile" "/cdn/profiles/web"
test_get "Social Profile" "/cdn/profiles/social"
test_get "Thumbnail Profile" "/cdn/profiles/thumbnail"
test_get "Ecommerce Profile" "/cdn/profiles/ecommerce"
test_get "Avatar Profile" "/cdn/profiles/avatar"
test_get "Banner Profile" "/cdn/profiles/banner"
test_post "Transform URL" "/cdn/transform-url" '{"url":"/img/test.jpg","width":800,"format":"webp"}'
test_post "Generate Srcset" "/cdn/srcset" '{"url":"/img/test.jpg","profile":"web"}'
test_get "Social Presets" "/cdn/social-presets"
test_post "Social URL (OG)" "/cdn/social-url" '{"url":"/img/test.jpg","platform":"og-image"}'
test_post "Social URL (Twitter)" "/cdn/social-url" '{"url":"/img/test.jpg","platform":"twitter-card"}'
test_post "Social URL (Instagram)" "/cdn/social-url" '{"url":"/img/test.jpg","platform":"instagram-square"}'
echo ""

echo "▶ BULK IMPORT TESTS"
echo "───────────────────────────────────────────────────────────────"
test_post "Create Import Job" "/import/urls" '{"urls":["https://placekitten.com/100/100"]}'
test_post "Parse URL Text" "/import/urls-text" '{"text":"https://placekitten.com/200/200"}'
test_get "List Jobs" "/import/jobs"
echo ""

echo "▶ AI SERVICE TESTS"
echo "───────────────────────────────────────────────────────────────"
test_get "AI Service Status" "/ai/status"
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "  TEST RESULTS: $PASSED passed, $FAILED failed"
echo "═══════════════════════════════════════════════════════════════"

if [ $FAILED -eq 0 ]; then
  echo "✓ All tests passed!"
  exit 0
else
  echo "✗ Some tests failed"
  exit 1
fi
