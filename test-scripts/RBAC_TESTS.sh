#!/bin/bash
# RBAC_TESTS.sh - Role-Based Access Control Test Suite
# Usage:
#   BASE_URL=http://localhost:8000/api \
#   PUBLIC_TOKEN="" \
#   USER_TOKEN="xxx" \
#   ADMIN_TOKEN="xxx" \
#   ./RBAC_TESTS.sh

set -o pipefail

BASE_URL="${BASE_URL:-http://localhost:8000/api}"

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
echo "RBAC (Role-Based Access Control) TEST SUITE"
echo "Base URL: ${BASE_URL}"
echo "=============================================="
echo ""

# ============ PUBLIC (No Authentication) ============

echo "=== TESTING PUBLIC ACCESS ==="

# RBAC-P001: Public - List Posts
echo "--- RBAC-P001: Public can list posts ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "200" ]; then
  log_result "RBAC-P001" "PASS" "Public can access posts list"
else
  log_result "RBAC-P001" "FAIL" "Expected 200, got $HTTP_CODE"
fi

# RBAC-P002: Public - Cannot Access Admin Posts
echo "--- RBAC-P002: Public blocked from admin posts ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "RBAC-P002" "PASS" "Public blocked from admin posts (401)"
else
  log_result "RBAC-P002" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# RBAC-P003: Public - Cannot Create Posts
echo "--- RBAC-P003: Public cannot create posts ---"
RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{"title":"Unauthorized","slug":"unauth","content":"test"}' \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "RBAC-P003" "PASS" "Public cannot create posts (401)"
else
  log_result "RBAC-P003" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# RBAC-P004: Public - Cannot Access Users
echo "--- RBAC-P004: Public blocked from users ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/users" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "RBAC-P004" "PASS" "Public blocked from users (401)"
else
  log_result "RBAC-P004" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# RBAC-P005: Public - Cannot Access Settings
echo "--- RBAC-P005: Public blocked from settings ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/settings" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "401" ]; then
  log_result "RBAC-P005" "PASS" "Public blocked from settings (401)"
else
  log_result "RBAC-P005" "FAIL" "Expected 401, got $HTTP_CODE"
fi

# ============ AUTHENTICATED USER ============

echo ""
echo "=== TESTING AUTHENTICATED (NON-ADMIN) USER ==="

if [ -z "$USER_TOKEN" ]; then
  echo "Skipping user tests - Set USER_TOKEN for authenticated non-admin user"
  log_result "RBAC-U001-U005" "SKIP" "No USER_TOKEN provided"
else
  # RBAC-U001: User - Can Access /me
  echo "--- RBAC-U001: User can access profile ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-U001" "PASS" "User can access profile"
  else
    log_result "RBAC-U001" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-U002: User - Cannot Access Admin Posts
  echo "--- RBAC-U002: User blocked from admin posts ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "403" ]; then
    log_result "RBAC-U002" "PASS" "User blocked from admin posts (403)"
  elif [ "$HTTP_CODE" = "401" ]; then
    log_result "RBAC-U002" "PASS" "User blocked from admin posts (401 - token issue)"
  else
    log_result "RBAC-U002" "FAIL" "Expected 403, got $HTTP_CODE"
  fi

  # RBAC-U003: User - Cannot Create Posts
  echo "--- RBAC-U003: User cannot create admin posts ---"
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{"title":"User Created","slug":"user-post","content":"test"}' \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "403" ]; then
    log_result "RBAC-U003" "PASS" "User cannot create admin posts (403)"
  else
    log_result "RBAC-U003" "FAIL" "Expected 403, got $HTTP_CODE"
  fi

  # RBAC-U004: User - Can Access Own Subscriptions
  echo "--- RBAC-U004: User can access own subscriptions ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/my/subscriptions" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-U004" "PASS" "User can access own subscriptions"
  else
    log_result "RBAC-U004" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-U005: User - Cannot Access Admin Users
  echo "--- RBAC-U005: User blocked from admin users ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/users" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "403" ]; then
    log_result "RBAC-U005" "PASS" "User blocked from admin users (403)"
  else
    log_result "RBAC-U005" "FAIL" "Expected 403, got $HTTP_CODE"
  fi
fi

# ============ ADMIN USER ============

echo ""
echo "=== TESTING ADMIN USER ==="

if [ -z "$ADMIN_TOKEN" ]; then
  echo "Skipping admin tests - Set ADMIN_TOKEN for admin user"
  log_result "RBAC-A001-A010" "SKIP" "No ADMIN_TOKEN provided"
else
  # RBAC-A001: Admin - Can Access Admin Posts
  echo "--- RBAC-A001: Admin can access admin posts ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A001" "PASS" "Admin can access admin posts"
  else
    log_result "RBAC-A001" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A002: Admin - Can Create Posts
  echo "--- RBAC-A002: Admin can create posts ---"
  TIMESTAMP=$(date +%s)
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"title\": \"Admin Test Post $TIMESTAMP\",
      \"slug\": \"admin-test-$TIMESTAMP\",
      \"content\": \"Admin created content\",
      \"status\": \"draft\"
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  if [ "$HTTP_CODE" = "201" ]; then
    log_result "RBAC-A002" "PASS" "Admin can create posts"
    CREATED_ID=$(echo "$BODY" | jq -r '.data.id // .id' 2>/dev/null)
  else
    log_result "RBAC-A002" "FAIL" "Expected 201, got $HTTP_CODE"
  fi

  # RBAC-A003: Admin - Can Update Posts
  echo "--- RBAC-A003: Admin can update posts ---"
  if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
    RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/${CREATED_ID}" \
      -H "Authorization: Bearer $ADMIN_TOKEN" \
      -H "Content-Type: application/json" \
      -H "Accept: application/json" \
      -d '{"title":"Updated by Admin"}' \
      -w "\n%{http_code}")

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)

    if [ "$HTTP_CODE" = "200" ]; then
      log_result "RBAC-A003" "PASS" "Admin can update posts"
    else
      log_result "RBAC-A003" "FAIL" "Expected 200, got $HTTP_CODE"
    fi
  else
    log_result "RBAC-A003" "SKIP" "No post to update"
  fi

  # RBAC-A004: Admin - Can Delete Posts
  echo "--- RBAC-A004: Admin can delete posts ---"
  if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
    RESPONSE=$(curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
      -H "Authorization: Bearer $ADMIN_TOKEN" \
      -H "Accept: application/json" \
      -w "\n%{http_code}")

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)

    if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "204" ]; then
      log_result "RBAC-A004" "PASS" "Admin can delete posts"
    else
      log_result "RBAC-A004" "FAIL" "Expected 200/204, got $HTTP_CODE"
    fi
  else
    log_result "RBAC-A004" "SKIP" "No post to delete"
  fi

  # RBAC-A005: Admin - Can Access Users
  echo "--- RBAC-A005: Admin can access users ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/users" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A005" "PASS" "Admin can access users"
  else
    log_result "RBAC-A005" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A006: Admin - Can Access Settings
  echo "--- RBAC-A006: Admin can access settings ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/settings" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A006" "PASS" "Admin can access settings"
  else
    log_result "RBAC-A006" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A007: Admin - Can Access Media
  echo "--- RBAC-A007: Admin can access media ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/media" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A007" "PASS" "Admin can access media"
  else
    log_result "RBAC-A007" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A008: Admin - Can Access Email Settings
  echo "--- RBAC-A008: Admin can access email settings ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/email/settings" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A008" "PASS" "Admin can access email settings"
  else
    log_result "RBAC-A008" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A009: Admin - Can Access CRM
  echo "--- RBAC-A009: Admin can access CRM ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/crm/contacts" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A009" "PASS" "Admin can access CRM"
  else
    log_result "RBAC-A009" "FAIL" "Expected 200, got $HTTP_CODE"
  fi

  # RBAC-A010: Admin - Can Access Analytics
  echo "--- RBAC-A010: Admin can access analytics ---"
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/analytics/dashboard" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "RBAC-A010" "PASS" "Admin can access analytics"
  else
    log_result "RBAC-A010" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
fi

echo ""
echo "=============================================="
echo "RBAC TESTS COMPLETE"
echo "=============================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Skipped: $SKIPPED"
echo ""

exit $FAILED
