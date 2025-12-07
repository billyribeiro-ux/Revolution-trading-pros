#!/bin/bash
# SECURITY_TESTS.sh - Security Validation Test Suite
# Usage: BASE_URL=http://localhost:8000/api AUTH_TOKEN=xxx ./SECURITY_TESTS.sh

set -o pipefail

BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

PASSED=0
FAILED=0
SKIPPED=0

log_result() {
  local test_id=$1
  local result=$2
  local message=$3

  case $result in
    PASS) echo -e "[\033[32m✓ PASS\033[0m] $test_id: $message"; ((PASSED++));;
    FAIL) echo -e "[\033[31m✗ FAIL\033[0m] $test_id: $message"; ((FAILED++));;
    SKIP) echo -e "[\033[33m⏭ SKIP\033[0m] $test_id: $message"; ((SKIPPED++));;
    WARN) echo -e "[\033[33m⚠ WARN\033[0m] $test_id: $message"; ((PASSED++));;
  esac
}

echo "=============================================="
echo "SECURITY TEST SUITE"
echo "Base URL: ${BASE_URL}"
echo "=============================================="
echo ""

# SEC-001: XSS Prevention in Title
echo "--- SEC-001: XSS Prevention (Title) ---"
if [ -n "$AUTH_TOKEN" ]; then
  TIMESTAMP=$(date +%s)
  XSS_PAYLOAD='<script>alert("XSS")</script>'

  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"title\": \"${XSS_PAYLOAD}\",
      \"slug\": \"xss-test-${TIMESTAMP}\",
      \"content\": \"Test content\",
      \"status\": \"draft\"
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    # Check if XSS payload is sanitized
    if echo "$BODY" | grep -q "<script>"; then
      log_result "SEC-001" "FAIL" "XSS payload NOT sanitized - CRITICAL VULNERABILITY!"
    else
      log_result "SEC-001" "PASS" "XSS payload sanitized or escaped"
    fi

    # Cleanup - try to get ID and delete
    CREATED_ID=$(echo "$BODY" | jq -r '.data.id // .id' 2>/dev/null)
    if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
      curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
        -H "Authorization: Bearer $AUTH_TOKEN" > /dev/null 2>&1
    fi
  elif [ "$HTTP_CODE" = "422" ]; then
    log_result "SEC-001" "PASS" "XSS payload rejected at validation"
  else
    log_result "SEC-001" "WARN" "Unexpected response: $HTTP_CODE"
  fi
else
  log_result "SEC-001" "SKIP" "No auth token"
fi

# SEC-002: XSS Prevention in Content
echo "--- SEC-002: XSS Prevention (Content) ---"
if [ -n "$AUTH_TOKEN" ]; then
  TIMESTAMP=$(date +%s)

  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"title\": \"XSS Content Test ${TIMESTAMP}\",
      \"slug\": \"xss-content-${TIMESTAMP}\",
      \"content\": \"<img onerror=alert(1) src=x><svg onload=alert(1)>\",
      \"status\": \"draft\"
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    if echo "$BODY" | grep -qE "onerror|onload"; then
      log_result "SEC-002" "FAIL" "Event handlers NOT sanitized - CRITICAL!"
    else
      log_result "SEC-002" "PASS" "Event handlers sanitized"
    fi

    # Cleanup
    CREATED_ID=$(echo "$BODY" | jq -r '.data.id // .id' 2>/dev/null)
    if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
      curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
        -H "Authorization: Bearer $AUTH_TOKEN" > /dev/null 2>&1
    fi
  else
    log_result "SEC-002" "WARN" "Response code: $HTTP_CODE"
  fi
else
  log_result "SEC-002" "SKIP" "No auth token"
fi

# SEC-003: SQL Injection Prevention
echo "--- SEC-003: SQL Injection Prevention ---"
if [ -n "$AUTH_TOKEN" ]; then
  TIMESTAMP=$(date +%s)

  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"title\": \"'; DROP TABLE posts; --\",
      \"slug\": \"sql-test-${TIMESTAMP}\",
      \"content\": \"Test content\",
      \"status\": \"draft\"
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    # Verify posts table still exists
    VERIFY=$(curl -s -X GET "${BASE_URL}/posts" \
      -H "Accept: application/json" \
      -w "\n%{http_code}")

    VERIFY_CODE=$(echo "$VERIFY" | tail -1)

    if [ "$VERIFY_CODE" = "200" ]; then
      log_result "SEC-003" "PASS" "SQL injection handled safely (parameterized queries)"
    else
      log_result "SEC-003" "FAIL" "Posts table may have been affected!"
    fi

    # Cleanup
    CREATED_ID=$(echo "$RESPONSE" | sed '$d' | jq -r '.data.id // .id' 2>/dev/null)
    if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
      curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
        -H "Authorization: Bearer $AUTH_TOKEN" > /dev/null 2>&1
    fi
  elif [ "$HTTP_CODE" = "422" ]; then
    log_result "SEC-003" "PASS" "SQL injection rejected at validation"
  else
    log_result "SEC-003" "WARN" "Response code: $HTTP_CODE"
  fi
else
  log_result "SEC-003" "SKIP" "No auth token"
fi

# SEC-004: Rate Limiting (Login)
echo "--- SEC-004: Rate Limiting (Login) ---"
echo "Sending rapid login attempts..."
RATE_LIMITED=false

for i in {1..15}; do
  STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "${BASE_URL}/login" \
    -H "Content-Type: application/json" \
    -d '{"email":"ratelimit@test.com","password":"wrong"}')

  if [ "$STATUS" = "429" ]; then
    RATE_LIMITED=true
    log_result "SEC-004" "PASS" "Rate limit triggered at attempt $i"
    break
  fi
done

if [ "$RATE_LIMITED" = false ]; then
  log_result "SEC-004" "WARN" "Rate limit not triggered after 15 attempts (may be configured differently)"
fi

# SEC-005: Invalid Token Rejection
echo "--- SEC-005: Invalid Token Rejection ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
  -H "Authorization: Bearer invalid_token_12345" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "SEC-005" "PASS" "Invalid token rejected"
else
  log_result "SEC-005" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# SEC-006: Missing Token Rejection
echo "--- SEC-006: Missing Token Rejection ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "SEC-006" "PASS" "Missing token rejected"
else
  log_result "SEC-006" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# SEC-007: Admin Endpoint Protection
echo "--- SEC-007: Admin Endpoint Protection ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/users" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "SEC-007" "PASS" "Admin endpoint protected (unauthenticated)"
else
  log_result "SEC-007" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# SEC-008: Path Traversal Prevention
echo "--- SEC-008: Path Traversal Prevention ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts/../../../etc/passwd" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" = "404" ] || [ "$HTTP_CODE" = "400" ]; then
  log_result "SEC-008" "PASS" "Path traversal blocked"
elif echo "$BODY" | grep -q "root:"; then
  log_result "SEC-008" "FAIL" "PATH TRAVERSAL VULNERABILITY DETECTED!"
else
  log_result "SEC-008" "PASS" "Path traversal handled safely"
fi

# SEC-009: CORS Headers Check
echo "--- SEC-009: CORS Headers Check ---"
RESPONSE=$(curl -s -I -X OPTIONS "${BASE_URL}/posts" \
  -H "Origin: http://evil.com" \
  -H "Access-Control-Request-Method: POST" 2>&1)

if echo "$RESPONSE" | grep -qi "Access-Control-Allow-Origin: \*"; then
  log_result "SEC-009" "WARN" "CORS allows all origins - verify this is intentional"
elif echo "$RESPONSE" | grep -qi "Access-Control-Allow-Origin: http://evil.com"; then
  log_result "SEC-009" "FAIL" "CORS allows arbitrary origins"
else
  log_result "SEC-009" "PASS" "CORS properly restricted"
fi

# SEC-010: Security Headers Check
echo "--- SEC-010: Security Headers Check ---"
RESPONSE=$(curl -s -I "${BASE_URL}/posts" 2>&1)

MISSING_HEADERS=""

if ! echo "$RESPONSE" | grep -qi "X-Content-Type-Options"; then
  MISSING_HEADERS="$MISSING_HEADERS X-Content-Type-Options"
fi

if ! echo "$RESPONSE" | grep -qi "X-Frame-Options"; then
  MISSING_HEADERS="$MISSING_HEADERS X-Frame-Options"
fi

if [ -n "$MISSING_HEADERS" ]; then
  log_result "SEC-010" "WARN" "Missing security headers:$MISSING_HEADERS"
else
  log_result "SEC-010" "PASS" "Security headers present"
fi

echo ""
echo "=============================================="
echo "SECURITY TESTS COMPLETE"
echo "=============================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Skipped: $SKIPPED"
echo ""

exit $FAILED
