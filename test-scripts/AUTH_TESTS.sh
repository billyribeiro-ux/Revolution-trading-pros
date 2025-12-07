#!/bin/bash
# AUTH_TESTS.sh - Authentication Test Suite
# Usage: BASE_URL=http://localhost:8000/api ./AUTH_TESTS.sh

set -o pipefail

BASE_URL="${BASE_URL:-http://localhost:8000/api}"
TEST_EMAIL="testuser_$(date +%s)@example.com"
TEST_PASSWORD="Password123!"

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
  esac
}

echo "=============================================="
echo "AUTHENTICATION TEST SUITE"
echo "Base URL: ${BASE_URL}"
echo "=============================================="
echo ""

# AUTH-001: Valid Registration
echo "--- AUTH-001: Valid Registration ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/register" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d "{
    \"name\": \"Test User\",
    \"email\": \"${TEST_EMAIL}\",
    \"password\": \"${TEST_PASSWORD}\",
    \"password_confirmation\": \"${TEST_PASSWORD}\"
  }" -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" = "201" ]; then
  log_result "AUTH-001" "PASS" "Registration successful"
  AUTH_TOKEN=$(echo "$BODY" | jq -r '.token // empty')
  REFRESH_TOKEN=$(echo "$BODY" | jq -r '.refresh_token // empty')
else
  log_result "AUTH-001" "FAIL" "Expected 201, got $HTTP_CODE"
fi

# AUTH-002: Duplicate Registration
echo "--- AUTH-002: Duplicate Registration ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/register" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d "{
    \"name\": \"Test User\",
    \"email\": \"${TEST_EMAIL}\",
    \"password\": \"${TEST_PASSWORD}\",
    \"password_confirmation\": \"${TEST_PASSWORD}\"
  }" -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "422" ]; then
  log_result "AUTH-002" "PASS" "Duplicate email rejected"
else
  log_result "AUTH-002" "FAIL" "Expected 422, got $HTTP_CODE"
fi

# AUTH-003: Valid Login
echo "--- AUTH-003: Valid Login ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d "{
    \"email\": \"${TEST_EMAIL}\",
    \"password\": \"${TEST_PASSWORD}\"
  }" -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
  log_result "AUTH-003" "PASS" "Login successful"
  AUTH_TOKEN=$(echo "$BODY" | jq -r '.token // empty')
  REFRESH_TOKEN=$(echo "$BODY" | jq -r '.refresh_token // empty')
  SESSION_ID=$(echo "$BODY" | jq -r '.session_id // empty')
else
  log_result "AUTH-003" "FAIL" "Expected 200, got $HTTP_CODE"
fi

# AUTH-004: Invalid Credentials
echo "--- AUTH-004: Invalid Credentials ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "email": "invalid@example.com",
    "password": "wrongpassword"
  }' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "422" ] || [ "$HTTP_CODE" = "401" ]; then
  log_result "AUTH-004" "PASS" "Invalid credentials rejected"
else
  log_result "AUTH-004" "FAIL" "Expected 422/401, got $HTTP_CODE"
fi

# AUTH-005: Missing Credentials
echo "--- AUTH-005: Missing Credentials ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{}' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "422" ]; then
  log_result "AUTH-005" "PASS" "Missing credentials rejected"
else
  log_result "AUTH-005" "FAIL" "Expected 422, got $HTTP_CODE"
fi

# AUTH-006: Get Current User
echo "--- AUTH-006: Get Current User (/me) ---"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "AUTH-006" "PASS" "User profile retrieved"
  else
    log_result "AUTH-006" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
else
  log_result "AUTH-006" "SKIP" "No auth token available"
fi

# AUTH-007: Token Refresh
echo "--- AUTH-007: Token Refresh ---"
if [ -n "$REFRESH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/refresh" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"refresh_token\": \"${REFRESH_TOKEN}\"
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "AUTH-007" "PASS" "Token refreshed"
    AUTH_TOKEN=$(echo "$BODY" | jq -r '.token // empty')
  else
    log_result "AUTH-007" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
else
  log_result "AUTH-007" "SKIP" "No refresh token available"
fi

# AUTH-008: Auth Check
echo "--- AUTH-008: Auth Check ---"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/auth/check" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "AUTH-008" "PASS" "Auth check passed"
  else
    log_result "AUTH-008" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
else
  log_result "AUTH-008" "SKIP" "No auth token available"
fi

# AUTH-009: Logout
echo "--- AUTH-009: Logout ---"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/logout" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "AUTH-009" "PASS" "Logout successful"
  else
    log_result "AUTH-009" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
else
  log_result "AUTH-009" "SKIP" "No auth token available"
fi

# AUTH-010: Access After Logout (should fail)
echo "--- AUTH-010: Access After Logout ---"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "401" ]; then
    log_result "AUTH-010" "PASS" "Token invalidated after logout"
  else
    log_result "AUTH-010" "FAIL" "Expected 401, got $HTTP_CODE"
  fi
else
  log_result "AUTH-010" "SKIP" "No auth token available"
fi

echo ""
echo "=============================================="
echo "AUTHENTICATION TESTS COMPLETE"
echo "=============================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Skipped: $SKIPPED"
echo ""

exit $FAILED
