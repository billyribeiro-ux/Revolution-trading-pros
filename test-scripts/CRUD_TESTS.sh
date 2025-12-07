#!/bin/bash
# CRUD_TESTS.sh - Content CRUD Operations Test Suite
# Usage: BASE_URL=http://localhost:8000/api AUTH_TOKEN=xxx ./CRUD_TESTS.sh

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
  esac
}

echo "=============================================="
echo "CRUD OPERATIONS TEST SUITE"
echo "Base URL: ${BASE_URL}"
echo "=============================================="
echo ""

# ============ PUBLIC READ OPERATIONS ============

# CRUD-R001: List Posts (Public)
echo "--- CRUD-R001: List Posts (Public) ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
  log_result "CRUD-R001" "PASS" "Posts list retrieved"
else
  log_result "CRUD-R001" "FAIL" "Expected 200, got $HTTP_CODE"
fi

# CRUD-R002: Get Single Post (Public)
echo "--- CRUD-R002: Get Single Post ---"
# Extract first post slug from list
FIRST_SLUG=$(echo "$BODY" | jq -r '.data[0].slug // .data[0].attributes.slug // empty' 2>/dev/null)

if [ -n "$FIRST_SLUG" ] && [ "$FIRST_SLUG" != "null" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/posts/${FIRST_SLUG}" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    log_result "CRUD-R002" "PASS" "Single post retrieved (slug: $FIRST_SLUG)"
  else
    log_result "CRUD-R002" "FAIL" "Expected 200, got $HTTP_CODE"
  fi
else
  log_result "CRUD-R002" "SKIP" "No posts available to test"
fi

# CRUD-R003: Non-existent Post (404)
echo "--- CRUD-R003: Non-existent Post ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts/non-existent-post-$(date +%s)" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)

if [ "$HTTP_CODE" = "404" ]; then
  log_result "CRUD-R003" "PASS" "404 returned for non-existent post"
else
  log_result "CRUD-R003" "FAIL" "Expected 404, got $HTTP_CODE"
fi

# CRUD-R004: Pagination
echo "--- CRUD-R004: Pagination ---"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts?page=1&pageSize=5" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

if [ "$HTTP_CODE" = "200" ]; then
  # Check if pagination metadata exists
  HAS_META=$(echo "$BODY" | jq 'has("meta") or has("pagination")' 2>/dev/null)
  if [ "$HAS_META" = "true" ]; then
    log_result "CRUD-R004" "PASS" "Pagination working with metadata"
  else
    log_result "CRUD-R004" "PASS" "Pagination request successful (metadata may vary)"
  fi
else
  log_result "CRUD-R004" "FAIL" "Expected 200, got $HTTP_CODE"
fi

# ============ ADMIN OPERATIONS (Require Auth) ============

if [ -z "$AUTH_TOKEN" ]; then
  echo ""
  echo "Skipping admin tests - No AUTH_TOKEN provided"
  log_result "CRUD-C001-C010" "SKIP" "No auth token"
  log_result "CRUD-U001-U008" "SKIP" "No auth token"
  log_result "CRUD-D001-D006" "SKIP" "No auth token"
else
  # CRUD-C001: Create Post
  echo "--- CRUD-C001: Create Post ---"
  TIMESTAMP=$(date +%s)
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d "{
      \"title\": \"Test Post $TIMESTAMP\",
      \"slug\": \"test-post-$TIMESTAMP\",
      \"content\": \"This is automated test content.\",
      \"excerpt\": \"Test excerpt\",
      \"status\": \"draft\",
      \"featured\": false
    }" -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  if [ "$HTTP_CODE" = "201" ]; then
    log_result "CRUD-C001" "PASS" "Post created"
    CREATED_ID=$(echo "$BODY" | jq -r '.data.id // .id' 2>/dev/null)
    echo "  Created ID: $CREATED_ID"
  elif [ "$HTTP_CODE" = "403" ]; then
    log_result "CRUD-C001" "FAIL" "Forbidden - Token may not have admin role"
  else
    log_result "CRUD-C001" "FAIL" "Expected 201, got $HTTP_CODE"
  fi

  # CRUD-C002: Create with Missing Required Field
  echo "--- CRUD-C002: Create with Missing Required Field ---"
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "content": "Content without title"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "422" ]; then
    log_result "CRUD-C002" "PASS" "Validation error for missing required field"
  else
    log_result "CRUD-C002" "FAIL" "Expected 422, got $HTTP_CODE"
  fi

  # CRUD-U001: Update Post
  echo "--- CRUD-U001: Update Post ---"
  if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
    RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/${CREATED_ID}" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Content-Type: application/json" \
      -H "Accept: application/json" \
      -d '{
        "title": "Updated Test Post",
        "status": "published"
      }' -w "\n%{http_code}")

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)

    if [ "$HTTP_CODE" = "200" ]; then
      log_result "CRUD-U001" "PASS" "Post updated"
    else
      log_result "CRUD-U001" "FAIL" "Expected 200, got $HTTP_CODE"
    fi
  else
    log_result "CRUD-U001" "SKIP" "No post created to update"
  fi

  # CRUD-U002: Update Non-existent
  echo "--- CRUD-U002: Update Non-existent Post ---"
  RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/99999999" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "title": "Update Non-existent"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "404" ]; then
    log_result "CRUD-U002" "PASS" "404 returned for non-existent post update"
  else
    log_result "CRUD-U002" "FAIL" "Expected 404, got $HTTP_CODE"
  fi

  # CRUD-D001: Delete Post
  echo "--- CRUD-D001: Delete Post ---"
  if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
    RESPONSE=$(curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Accept: application/json" \
      -w "\n%{http_code}")

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)

    if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "204" ]; then
      log_result "CRUD-D001" "PASS" "Post deleted"
    else
      log_result "CRUD-D001" "FAIL" "Expected 200/204, got $HTTP_CODE"
    fi
  else
    log_result "CRUD-D001" "SKIP" "No post created to delete"
  fi

  # CRUD-D002: Delete Non-existent
  echo "--- CRUD-D002: Delete Non-existent Post ---"
  RESPONSE=$(curl -s -X DELETE "${BASE_URL}/admin/posts/99999999" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "404" ]; then
    log_result "CRUD-D002" "PASS" "404 returned for non-existent delete"
  else
    log_result "CRUD-D002" "FAIL" "Expected 404, got $HTTP_CODE"
  fi

  # CRUD-D003: Verify Deletion
  echo "--- CRUD-D003: Verify Post Deletion ---"
  if [ -n "$CREATED_ID" ] && [ "$CREATED_ID" != "null" ]; then
    RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts/${CREATED_ID}" \
      -H "Authorization: Bearer $AUTH_TOKEN" \
      -H "Accept: application/json" \
      -w "\n%{http_code}")

    HTTP_CODE=$(echo "$RESPONSE" | tail -1)

    if [ "$HTTP_CODE" = "404" ]; then
      log_result "CRUD-D003" "PASS" "Deleted post returns 404"
    else
      log_result "CRUD-D003" "FAIL" "Expected 404, got $HTTP_CODE - Post may still exist"
    fi
  else
    log_result "CRUD-D003" "SKIP" "No deletion to verify"
  fi
fi

echo ""
echo "=============================================="
echo "CRUD TESTS COMPLETE"
echo "=============================================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Skipped: $SKIPPED"
echo ""

exit $FAILED
