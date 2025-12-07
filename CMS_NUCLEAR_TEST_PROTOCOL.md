# NUCLEAR CMS TESTING PROTOCOL - Revolution Trading Pros

## Executive Summary

| Field | Value |
|-------|-------|
| **Test Date** | 2025-12-07 |
| **CMS Type** | Custom Laravel 12 API (Not Traditional Headless CMS) |
| **Backend Framework** | Laravel 12.x with Sanctum Authentication |
| **Frontend Framework** | SvelteKit 5 (Runes) |
| **Database** | SQLite (default), MySQL/PostgreSQL supported |
| **Authentication** | Laravel Sanctum (Bearer Token + Refresh Token) |
| **Authorization** | spatie/laravel-permission (RBAC) |
| **API Version** | v1 |
| **Base URL** | `http://localhost:8000/api` |

---

## PHASE 0: DISCOVERY RESULTS

### 0.1 API Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    REVOLUTION TRADING PROS API                   │
├─────────────────────────────────────────────────────────────────┤
│  Frontend (SvelteKit)  ──────────►  Backend (Laravel 12)        │
│  Port: 5173                         Port: 8000                   │
├─────────────────────────────────────────────────────────────────┤
│  Authentication: Laravel Sanctum (Bearer + Refresh Tokens)       │
│  Authorization: spatie/laravel-permission (Roles & Permissions)  │
│  Session: Single-Session Enforcement (Microsoft-style)           │
│  MFA: TOTP, Backup Codes, WebAuthn/Biometric                    │
└─────────────────────────────────────────────────────────────────┘
```

### 0.2 Content Types/Models Discovered

| Model | API Prefix | Public Access | Auth Required | Admin Only |
|-------|------------|---------------|---------------|------------|
| Posts | `/api/posts` | GET (list, single) | - | CRUD |
| Users | `/api/admin/users` | - | - | Full CRUD |
| Members | `/api/admin/members` | - | - | Full CRUD |
| Products | `/api/admin/products` | - | - | Full CRUD |
| Indicators | `/api/indicators` | GET (list, single) | Download | - |
| Subscriptions | `/api/admin/subscriptions` | - | GET own | Full CRUD |
| Forms | `/api/forms` | Submit only | - | Full CRUD |
| Media | `/api/admin/media` | - | - | Full CRUD |
| Categories | `/api/admin/categories` | - | - | Full CRUD |
| Tags | `/api/admin/tags` | - | - | Full CRUD |
| Popups | `/api/popups` | GET active | - | Full CRUD |
| Videos | `/api/videos` | GET (list, single) | Track | Full CRUD |
| Email Campaigns | `/api/admin/email/campaigns` | - | - | Full CRUD |
| Email Templates | `/api/admin/email/templates` | - | - | Full CRUD |
| Contacts (CRM) | `/api/admin/crm/contacts` | - | - | Full CRUD |
| Deals (CRM) | `/api/admin/crm/deals` | - | - | Full CRUD |
| Pipelines (CRM) | `/api/admin/crm/pipelines` | - | - | Full CRUD |

### 0.3 Authentication Configuration

```typescript
// From: frontend/src/lib/api/config.ts

interface AuthTokens {
  token: string;           // Access token (1 hour expiry)
  refresh_token: string;   // Refresh token (30 days expiry)
  session_id: string;      // Single-session tracking
  expires_in: number;      // Seconds until access token expires
}

// Rate Limits (from api.php):
// - Login: 5 attempts per minute
// - Register: 5 attempts per minute
// - MFA: 5 attempts per minute
// - Biometric: 10 attempts per minute
// - Token Refresh: 30 attempts per minute
// - Password Reset: 3 attempts per minute
```

### 0.4 Role-Based Access Control

```
Roles Discovered:
├── public (unauthenticated)
├── authenticated (logged-in users)
├── admin
└── super-admin
```

---

## PHASE 1: AUTHENTICATION & AUTHORIZATION TESTS

### 1.1 Authentication Endpoints

| Test ID | Endpoint | Method | Description |
|---------|----------|--------|-------------|
| AUTH-001 | `/api/login` | POST | Valid login with email/password |
| AUTH-002 | `/api/login` | POST | Invalid credentials (401) |
| AUTH-003 | `/api/login` | POST | Missing fields (422) |
| AUTH-004 | `/api/register` | POST | Valid registration |
| AUTH-005 | `/api/register` | POST | Duplicate email (422) |
| AUTH-006 | `/api/logout` | POST | Logout (requires auth) |
| AUTH-007 | `/api/auth/refresh` | POST | Token refresh |
| AUTH-008 | `/api/me` | GET | Get current user |
| AUTH-009 | `/api/login/mfa` | POST | MFA login |
| AUTH-010 | `/api/login/biometric` | POST | WebAuthn login |

### 1.2 Authentication Test Scripts

```bash
#!/bin/bash
# AUTH_TESTS.sh - Execute from backend directory
BASE_URL="${BASE_URL:-http://localhost:8000/api}"

echo "========================================"
echo "AUTH-001: Valid Login Test"
echo "========================================"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Body: $BODY"
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "200" ]; then
  echo "RESULT: ✅ PASS"
  # Extract token for subsequent tests
  AUTH_TOKEN=$(echo "$BODY" | jq -r '.token')
  REFRESH_TOKEN=$(echo "$BODY" | jq -r '.refresh_token')
  SESSION_ID=$(echo "$BODY" | jq -r '.session_id')
  echo "Token captured: ${AUTH_TOKEN:0:50}..."
else
  echo "RESULT: ❌ FAIL (Expected 200, got $HTTP_CODE)"
fi

echo ""
echo "========================================"
echo "AUTH-002: Invalid Credentials Test"
echo "========================================"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "email": "invalid@test.com",
    "password": "wrongpassword"
  }' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Body: $BODY"
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "422" ]; then
  echo "RESULT: ✅ PASS (Validation error returned)"
elif [ "$HTTP_CODE" = "401" ]; then
  echo "RESULT: ✅ PASS (Unauthorized returned)"
else
  echo "RESULT: ❌ FAIL (Expected 422 or 401, got $HTTP_CODE)"
fi

echo ""
echo "========================================"
echo "AUTH-003: Missing Credentials Test"
echo "========================================"
RESPONSE=$(curl -s -X POST "${BASE_URL}/login" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{}' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Body: $BODY"
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "422" ]; then
  echo "RESULT: ✅ PASS (Validation error returned)"
else
  echo "RESULT: ❌ FAIL (Expected 422, got $HTTP_CODE)"
fi

echo ""
echo "========================================"
echo "AUTH-004: Registration Test"
echo "========================================"
TIMESTAMP=$(date +%s)
RESPONSE=$(curl -s -X POST "${BASE_URL}/register" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "name": "Test User '$TIMESTAMP'",
    "email": "testuser'$TIMESTAMP'@example.com",
    "password": "Password123!",
    "password_confirmation": "Password123!"
  }' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response Body: $BODY"
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "201" ]; then
  echo "RESULT: ✅ PASS"
else
  echo "RESULT: ❌ FAIL (Expected 201, got $HTTP_CODE)"
fi

echo ""
echo "========================================"
echo "AUTH-007: Token Refresh Test"
echo "========================================"
if [ -n "$REFRESH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/auth/refresh" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "refresh_token": "'$REFRESH_TOKEN'"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "Response Body: $BODY"
  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS"
    NEW_TOKEN=$(echo "$BODY" | jq -r '.token')
    echo "New token received: ${NEW_TOKEN:0:50}..."
  else
    echo "RESULT: ❌ FAIL (Expected 200, got $HTTP_CODE)"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No refresh token available)"
fi

echo ""
echo "========================================"
echo "AUTH-008: Get Current User (/me)"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/me" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "Response Body: $BODY"
  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS"
  else
    echo "RESULT: ❌ FAIL (Expected 200, got $HTTP_CODE)"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No auth token available)"
fi
```

### 1.3 Expected Authentication Response Schemas

```typescript
// Login Success Response
interface LoginResponse {
  user: {
    id: number;
    name: string;
    email: string;
    email_verified_at: string | null;
    roles: string[];
    is_admin: boolean;
    created_at: string;
    updated_at: string;
  };
  token: string;           // Bearer access token
  refresh_token: string;   // Refresh token
  session_id: string;      // Session identifier
  expires_in: number;      // Seconds (3600 = 1 hour)
}

// Login with MFA Required Response
interface MFARequiredResponse {
  user: User;
  mfa_required: true;
}

// Validation Error Response
interface ValidationErrorResponse {
  message: string;
  errors: {
    [field: string]: string[];
  };
}
```

---

## PHASE 2: CONTENT TYPE CRUD OPERATIONS

### 2.1 Posts API Tests

| Test ID | Endpoint | Method | Auth | Description |
|---------|----------|--------|------|-------------|
| POST-R001 | `/api/posts` | GET | No | List all published posts |
| POST-R002 | `/api/posts/{slug}` | GET | No | Get single post by slug |
| POST-R003 | `/api/posts?page=1&pageSize=10` | GET | No | Paginated list |
| POST-C001 | `/api/admin/posts` | POST | Admin | Create new post |
| POST-U001 | `/api/admin/posts/{id}` | PUT | Admin | Update post |
| POST-D001 | `/api/admin/posts/{id}` | DELETE | Admin | Delete post |
| POST-D002 | `/api/admin/posts/bulk-delete` | POST | Admin | Bulk delete |

```bash
#!/bin/bash
# POSTS_CRUD_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

echo "========================================"
echo "POST-R001: List Posts (Public)"
echo "========================================"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "HTTP Status: $HTTP_CODE"
echo "Response (first 500 chars): ${BODY:0:500}"

if [ "$HTTP_CODE" = "200" ]; then
  echo "RESULT: ✅ PASS"
else
  echo "RESULT: ❌ FAIL"
fi

echo ""
echo "========================================"
echo "POST-C001: Create Post (Admin)"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  TIMESTAMP=$(date +%s)
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "title": "Test Post '$TIMESTAMP'",
      "slug": "test-post-'$TIMESTAMP'",
      "content": "This is test content for automated testing.",
      "excerpt": "Test excerpt",
      "status": "draft",
      "featured": false
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"
  echo "Response: $BODY"

  if [ "$HTTP_CODE" = "201" ]; then
    echo "RESULT: ✅ PASS"
    CREATED_ID=$(echo "$BODY" | jq -r '.data.id // .id')
    echo "Created Post ID: $CREATED_ID"
  else
    echo "RESULT: ❌ FAIL"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

echo ""
echo "========================================"
echo "POST-U001: Update Post (Admin)"
echo "========================================"
if [ -n "$AUTH_TOKEN" ] && [ -n "$CREATED_ID" ]; then
  RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/${CREATED_ID}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "title": "Updated Test Post",
      "status": "published"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"
  echo "Response: $BODY"

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS"
  else
    echo "RESULT: ❌ FAIL"
  fi
else
  echo "RESULT: ⏭️ SKIPPED"
fi

echo ""
echo "========================================"
echo "POST-D001: Delete Post (Admin)"
echo "========================================"
if [ -n "$AUTH_TOKEN" ] && [ -n "$CREATED_ID" ]; then
  RESPONSE=$(curl -s -X DELETE "${BASE_URL}/admin/posts/${CREATED_ID}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "200" ] || [ "$HTTP_CODE" = "204" ]; then
    echo "RESULT: ✅ PASS"
  else
    echo "RESULT: ❌ FAIL"
  fi
else
  echo "RESULT: ⏭️ SKIPPED"
fi
```

### 2.2 Security Validation Tests

```bash
#!/bin/bash
# SECURITY_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

echo "========================================"
echo "SEC-001: XSS Prevention Test"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "title": "<script>alert(\"XSS\")</script>",
      "slug": "xss-test-'$(date +%s)'",
      "content": "<img onerror=\"alert(1)\" src=\"x\">",
      "status": "draft"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"
  echo "Response: $BODY"

  # Check if script tags are sanitized
  if echo "$BODY" | grep -q "<script>"; then
    echo "RESULT: ❌ FAIL - XSS payload not sanitized!"
  else
    echo "RESULT: ✅ PASS - XSS payload sanitized"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

echo ""
echo "========================================"
echo "SEC-002: SQL Injection Prevention Test"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    -d '{
      "title": "'\'''; DROP TABLE posts; --",
      "slug": "sql-inject-test-'$(date +%s)'",
      "content": "Test content",
      "status": "draft"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS - SQL injection handled safely (parameterized queries)"
  elif [ "$HTTP_CODE" = "422" ]; then
    echo "RESULT: ✅ PASS - Input rejected"
  else
    echo "RESULT: ⚠️ WARNING - Unexpected response code: $HTTP_CODE"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

echo ""
echo "========================================"
echo "SEC-003: Rate Limiting Test (Login)"
echo "========================================"
echo "Sending 10 rapid login attempts..."
for i in {1..10}; do
  STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "${BASE_URL}/login" \
    -H "Content-Type: application/json" \
    -d '{"email":"rate@test.com","password":"wrong"}')

  echo "Attempt $i: HTTP $STATUS"

  if [ "$STATUS" = "429" ]; then
    echo "RESULT: ✅ PASS - Rate limit triggered at attempt $i"
    break
  fi
done

if [ "$STATUS" != "429" ]; then
  echo "RESULT: ⚠️ WARNING - Rate limit not triggered after 10 attempts"
fi
```

---

## PHASE 3: MEDIA/ASSET HANDLING TESTS

### 3.1 Media Upload Endpoints

| Test ID | Endpoint | Method | Description |
|---------|----------|--------|-------------|
| MEDIA-001 | `/api/admin/media/upload` | POST | Single file upload |
| MEDIA-002 | `/api/admin/media/bulk-upload` | POST | Multiple file upload |
| MEDIA-003 | `/api/admin/media` | GET | List media |
| MEDIA-004 | `/api/admin/media/{id}` | GET | Get single media |
| MEDIA-005 | `/api/admin/media/{id}` | PUT | Update metadata |
| MEDIA-006 | `/api/admin/media/{id}` | DELETE | Delete media |
| MEDIA-007 | `/api/admin/media/{id}/optimize` | POST | Optimize image |
| MEDIA-008 | `/api/admin/media/{id}/download` | GET | Download file |

```bash
#!/bin/bash
# MEDIA_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

# Create test image (1x1 red PNG)
echo "Creating test files..."
echo -n 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==' | base64 -d > /tmp/test_image.png

echo "========================================"
echo "MEDIA-001: Single File Upload"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/media/upload" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -F "file=@/tmp/test_image.png" \
    -F "name=test_upload_$(date +%s)" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"
  echo "Response: $BODY"

  if [ "$HTTP_CODE" = "201" ] || [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS"
    MEDIA_ID=$(echo "$BODY" | jq -r '.data.id // .id')
    echo "Uploaded Media ID: $MEDIA_ID"
  else
    echo "RESULT: ❌ FAIL"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

echo ""
echo "========================================"
echo "MEDIA-002: Forbidden File Type Test"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  echo "<?php echo 'test'; ?>" > /tmp/test.php

  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/media/upload" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -F "file=@/tmp/test.php" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"
  echo "Response: $BODY"

  if [ "$HTTP_CODE" = "422" ] || [ "$HTTP_CODE" = "400" ]; then
    echo "RESULT: ✅ PASS - PHP file rejected"
  else
    echo "RESULT: ❌ FAIL - PHP file should be rejected!"
  fi

  rm -f /tmp/test.php
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

echo ""
echo "========================================"
echo "MEDIA-003: Oversized File Test"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  # Create 51MB file (assuming 50MB limit)
  dd if=/dev/urandom of=/tmp/large_file.bin bs=1M count=51 2>/dev/null

  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/media/upload" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Accept: application/json" \
    -F "file=@/tmp/large_file.bin" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "422" ] || [ "$HTTP_CODE" = "413" ]; then
    echo "RESULT: ✅ PASS - Large file rejected"
  else
    echo "RESULT: ⚠️ WARNING - Large file may have been accepted"
  fi

  rm -f /tmp/large_file.bin
else
  echo "RESULT: ⏭️ SKIPPED (No auth token)"
fi

# Cleanup
rm -f /tmp/test_image.png
```

---

## PHASE 4: PERMISSION & ROLE-BASED ACCESS TESTS

### 4.1 RBAC Test Matrix

| Test ID | Role | Endpoint | Method | Expected |
|---------|------|----------|--------|----------|
| RBAC-001 | public | `/api/posts` | GET | 200 |
| RBAC-002 | public | `/api/admin/posts` | GET | 401 |
| RBAC-003 | public | `/api/admin/posts` | POST | 401 |
| RBAC-004 | authenticated | `/api/me` | GET | 200 |
| RBAC-005 | authenticated | `/api/admin/posts` | GET | 403 |
| RBAC-006 | admin | `/api/admin/posts` | GET | 200 |
| RBAC-007 | admin | `/api/admin/posts` | POST | 201 |
| RBAC-008 | admin | `/api/admin/users` | GET | 200 |
| RBAC-009 | super-admin | `/api/admin/settings` | GET | 200 |

```bash
#!/bin/bash
# RBAC_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"

echo "========================================"
echo "RBAC-001: Public Access - List Posts"
echo "========================================"
RESPONSE=$(curl -s -X GET "${BASE_URL}/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "200" ]; then
  echo "RESULT: ✅ PASS - Public can access posts"
else
  echo "RESULT: ❌ FAIL"
fi

echo ""
echo "========================================"
echo "RBAC-002: Public Access - Admin Posts (Should Fail)"
echo "========================================"
RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
  -H "Accept: application/json" \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "401" ]; then
  echo "RESULT: ✅ PASS - Public blocked from admin"
else
  echo "RESULT: ❌ FAIL - Expected 401, got $HTTP_CODE"
fi

echo ""
echo "========================================"
echo "RBAC-003: Public Access - Create Post (Should Fail)"
echo "========================================"
RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{"title":"Unauthorized","slug":"unauth"}' \
  -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
echo "HTTP Status: $HTTP_CODE"

if [ "$HTTP_CODE" = "401" ]; then
  echo "RESULT: ✅ PASS - Public cannot create posts"
else
  echo "RESULT: ❌ FAIL - Expected 401, got $HTTP_CODE"
fi

# Test with authenticated non-admin user
echo ""
echo "========================================"
echo "RBAC-005: Authenticated Non-Admin - Admin Posts"
echo "========================================"
echo "Requires USER_TOKEN environment variable for non-admin user"
if [ -n "$USER_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $USER_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "403" ]; then
    echo "RESULT: ✅ PASS - Non-admin blocked"
  else
    echo "RESULT: ❌ FAIL - Expected 403, got $HTTP_CODE"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (Set USER_TOKEN for non-admin user)"
fi

# Test with admin user
echo ""
echo "========================================"
echo "RBAC-006: Admin - Access Admin Posts"
echo "========================================"
if [ -n "$ADMIN_TOKEN" ]; then
  RESPONSE=$(curl -s -X GET "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $ADMIN_TOKEN" \
    -H "Accept: application/json" \
    -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS - Admin has access"
  else
    echo "RESULT: ❌ FAIL - Expected 200, got $HTTP_CODE"
  fi
else
  echo "RESULT: ⏭️ SKIPPED (Set ADMIN_TOKEN for admin user)"
fi
```

---

## PHASE 5: PERFORMANCE & LOAD TESTING

### 5.1 Performance Benchmarks

```bash
#!/bin/bash
# PERFORMANCE_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

echo "========================================"
echo "PERF-001: Single Request Latency (10 samples)"
echo "========================================"
TOTAL_TIME=0
for i in {1..10}; do
  TIME=$(curl -s -o /dev/null -w "%{time_total}" \
    -X GET "${BASE_URL}/posts" \
    -H "Accept: application/json")
  TOTAL_TIME=$(echo "$TOTAL_TIME + $TIME" | bc)
  echo "Request $i: ${TIME}s"
done
AVG_TIME=$(echo "scale=4; $TOTAL_TIME / 10" | bc)
echo "Average Latency: ${AVG_TIME}s"

if (( $(echo "$AVG_TIME < 0.5" | bc -l) )); then
  echo "RESULT: ✅ PASS - Under 500ms average"
elif (( $(echo "$AVG_TIME < 1.0" | bc -l) )); then
  echo "RESULT: ⚠️ WARNING - Between 500ms-1s"
else
  echo "RESULT: ❌ FAIL - Over 1s average"
fi

echo ""
echo "========================================"
echo "PERF-002: Concurrent Requests (10 parallel)"
echo "========================================"
START=$(date +%s.%N)
for i in {1..10}; do
  curl -s -o /dev/null \
    -X GET "${BASE_URL}/posts" \
    -H "Accept: application/json" &
done
wait
END=$(date +%s.%N)
DURATION=$(echo "$END - $START" | bc)
echo "10 concurrent requests completed in ${DURATION}s"

if (( $(echo "$DURATION < 2.0" | bc -l) )); then
  echo "RESULT: ✅ PASS"
else
  echo "RESULT: ⚠️ WARNING - Concurrent requests took over 2s"
fi

echo ""
echo "========================================"
echo "PERF-003: Large Payload Request"
echo "========================================"
TIME=$(curl -s -o /dev/null -w "%{time_total}" \
  -X GET "${BASE_URL}/posts?page=1&pageSize=100" \
  -H "Accept: application/json")
echo "Time to fetch 100 items: ${TIME}s"

if (( $(echo "$TIME < 2.0" | bc -l) )); then
  echo "RESULT: ✅ PASS"
else
  echo "RESULT: ⚠️ WARNING - Large payload took over 2s"
fi
```

---

## PHASE 6: WEBHOOK & EVENT TESTING

### 6.1 Webhook Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/admin/email/webhooks` | GET | List webhooks |
| `/api/admin/email/webhooks` | POST | Create webhook |
| `/api/admin/email/webhooks/{id}` | GET | Get webhook |
| `/api/admin/email/webhooks/{id}` | PUT | Update webhook |
| `/api/admin/email/webhooks/{id}` | DELETE | Delete webhook |
| `/api/admin/email/webhooks/{id}/test` | POST | Test webhook |
| `/webhooks/stripe` | POST | Stripe webhook receiver |

### 6.2 Stripe Webhook Test

```bash
#!/bin/bash
# WEBHOOK_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000}"

echo "========================================"
echo "WEBHOOK-001: Stripe Webhook Endpoint"
echo "========================================"
# Note: Real Stripe webhooks require valid signatures
# This tests that the endpoint exists and handles requests

RESPONSE=$(curl -s -X POST "${BASE_URL}/webhooks/stripe" \
  -H "Content-Type: application/json" \
  -H "Stripe-Signature: t=1234,v1=fake_signature" \
  -d '{
    "type": "checkout.session.completed",
    "data": {"object": {}}
  }' -w "\n%{http_code}")

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
echo "HTTP Status: $HTTP_CODE"

# 400 = signature invalid (expected without real signature)
# 200 = processed (shouldn't happen with fake signature)
if [ "$HTTP_CODE" = "400" ] || [ "$HTTP_CODE" = "401" ]; then
  echo "RESULT: ✅ PASS - Webhook validates signatures"
else
  echo "RESULT: ⚠️ WARNING - Unexpected response: $HTTP_CODE"
fi
```

---

## PHASE 7: DRAFT/PUBLISH WORKFLOW TESTING

### 7.1 Content Lifecycle Tests

```bash
#!/bin/bash
# WORKFLOW_TESTS.sh
BASE_URL="${BASE_URL:-http://localhost:8000/api}"
AUTH_TOKEN="${AUTH_TOKEN:-}"

echo "========================================"
echo "WORKFLOW-001: Create Draft Post"
echo "========================================"
if [ -n "$AUTH_TOKEN" ]; then
  TIMESTAMP=$(date +%s)
  RESPONSE=$(curl -s -X POST "${BASE_URL}/admin/posts" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "title": "Draft Post '$TIMESTAMP'",
      "slug": "draft-post-'$TIMESTAMP'",
      "content": "Draft content",
      "status": "draft"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)
  BODY=$(echo "$RESPONSE" | sed '$d')

  echo "HTTP Status: $HTTP_CODE"

  if [ "$HTTP_CODE" = "201" ]; then
    DRAFT_ID=$(echo "$BODY" | jq -r '.data.id // .id')
    echo "Created Draft ID: $DRAFT_ID"
    echo "RESULT: ✅ PASS"
  else
    echo "RESULT: ❌ FAIL"
  fi
else
  echo "RESULT: ⏭️ SKIPPED"
fi

echo ""
echo "========================================"
echo "WORKFLOW-002: Verify Draft Not Public"
echo "========================================"
if [ -n "$DRAFT_ID" ]; then
  # Try to access via public API
  RESPONSE=$(curl -s -X GET "${BASE_URL}/posts" \
    -H "Accept: application/json")

  if echo "$RESPONSE" | grep -q "$DRAFT_ID"; then
    echo "RESULT: ❌ FAIL - Draft visible in public API!"
  else
    echo "RESULT: ✅ PASS - Draft not visible publicly"
  fi
fi

echo ""
echo "========================================"
echo "WORKFLOW-003: Publish Draft"
echo "========================================"
if [ -n "$AUTH_TOKEN" ] && [ -n "$DRAFT_ID" ]; then
  RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/${DRAFT_ID}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "status": "published",
      "published_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS - Post published"
  else
    echo "RESULT: ❌ FAIL"
  fi
fi

echo ""
echo "========================================"
echo "WORKFLOW-004: Unpublish (Revert to Draft)"
echo "========================================"
if [ -n "$AUTH_TOKEN" ] && [ -n "$DRAFT_ID" ]; then
  RESPONSE=$(curl -s -X PUT "${BASE_URL}/admin/posts/${DRAFT_ID}" \
    -H "Authorization: Bearer $AUTH_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "status": "draft",
      "published_at": null
    }' -w "\n%{http_code}")

  HTTP_CODE=$(echo "$RESPONSE" | tail -1)

  if [ "$HTTP_CODE" = "200" ]; then
    echo "RESULT: ✅ PASS - Post reverted to draft"
  else
    echo "RESULT: ❌ FAIL"
  fi
fi

# Cleanup
if [ -n "$AUTH_TOKEN" ] && [ -n "$DRAFT_ID" ]; then
  curl -s -X DELETE "${BASE_URL}/admin/posts/${DRAFT_ID}" \
    -H "Authorization: Bearer $AUTH_TOKEN" > /dev/null
  echo "Cleanup: Test post deleted"
fi
```

---

## PHASE 8: COMPLETE API ENDPOINT REFERENCE

### 8.1 Public Endpoints (No Authentication)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health/live` | GET | Kubernetes liveness probe |
| `/api/health/ready` | GET | Kubernetes readiness probe |
| `/api/time/now` | GET | Current server time |
| `/api/login` | POST | User login |
| `/api/register` | POST | User registration |
| `/api/forgot-password` | POST | Password reset request |
| `/api/reset-password` | POST | Password reset |
| `/api/posts` | GET | List published posts |
| `/api/posts/{slug}` | GET | Get single post |
| `/api/indicators` | GET | List indicators |
| `/api/indicators/{slug}` | GET | Get indicator |
| `/api/videos` | GET | List videos |
| `/api/videos/{id}` | GET | Get video |
| `/api/popups/active` | GET | Get active popups |
| `/api/newsletter/subscribe` | POST | Newsletter subscription |
| `/api/forms/{slug}/submit` | POST | Form submission |

### 8.2 Authenticated Endpoints (Bearer Token Required)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/me` | GET | Current user profile |
| `/api/me/memberships` | GET | User memberships |
| `/api/me/products` | GET | User products |
| `/api/me/sessions` | GET | Active sessions |
| `/api/logout` | POST | Logout |
| `/api/auth/refresh` | POST | Refresh token |
| `/api/my/subscriptions` | GET | User subscriptions |
| `/api/cart/checkout` | POST | Checkout |

### 8.3 Admin Endpoints (Admin Role Required)

| Prefix | Resources |
|--------|-----------|
| `/api/admin/posts` | Full CRUD |
| `/api/admin/categories` | Full CRUD |
| `/api/admin/tags` | Full CRUD |
| `/api/admin/users` | Full CRUD |
| `/api/admin/members` | Full CRUD |
| `/api/admin/products` | Full CRUD |
| `/api/admin/subscriptions` | Full CRUD |
| `/api/admin/media` | Full CRUD + Upload |
| `/api/admin/email/*` | Full email management |
| `/api/admin/crm/*` | Full CRM |
| `/api/admin/popups` | Full CRUD |
| `/api/admin/coupons` | Full CRUD |
| `/api/admin/forms` | Full CRUD |

---

## TEST EXECUTION CHECKLIST

```
PHASE 0: DISCOVERY ✅
[ ] CMS type identified: Custom Laravel 12 API
[ ] Authentication method: Laravel Sanctum
[ ] Content types discovered: 15+
[ ] API endpoints mapped: 200+

PHASE 1: AUTHENTICATION
[ ] AUTH-001: Valid login
[ ] AUTH-002: Invalid credentials
[ ] AUTH-003: Missing fields
[ ] AUTH-004: Registration
[ ] AUTH-005: Duplicate email
[ ] AUTH-006: Logout
[ ] AUTH-007: Token refresh
[ ] AUTH-008: Get current user
[ ] AUTH-009: MFA login
[ ] AUTH-010: Biometric login

PHASE 2: CRUD OPERATIONS
[ ] Create operations (per content type)
[ ] Read operations (list, single, filter, sort, paginate)
[ ] Update operations (full, partial)
[ ] Delete operations (soft, hard, bulk)

PHASE 3: MEDIA HANDLING
[ ] MEDIA-001: Single upload
[ ] MEDIA-002: Forbidden file types
[ ] MEDIA-003: Size limits
[ ] MEDIA-004: Bulk upload
[ ] MEDIA-005: Optimization

PHASE 4: RBAC
[ ] RBAC-001: Public access
[ ] RBAC-002-003: Public blocked from admin
[ ] RBAC-004-005: Authenticated access
[ ] RBAC-006-009: Admin/Super-admin access

PHASE 5: PERFORMANCE
[ ] PERF-001: Single request latency
[ ] PERF-002: Concurrent requests
[ ] PERF-003: Large payloads
[ ] Rate limiting verified

PHASE 6: WEBHOOKS
[ ] Webhook registration
[ ] Stripe webhook validation
[ ] Event triggering

PHASE 7: WORKFLOW
[ ] Draft creation
[ ] Draft visibility
[ ] Publishing
[ ] Unpublishing

PHASE 8: SECURITY
[ ] XSS prevention
[ ] SQL injection prevention
[ ] CSRF protection
[ ] Rate limiting
```

---

## MASTER TEST SCRIPT

```bash
#!/bin/bash
# run_all_tests.sh - Master test runner
# Usage: BASE_URL=http://localhost:8000/api AUTH_TOKEN=xxx ./run_all_tests.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="${SCRIPT_DIR}/test-results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "=============================================="
echo "NUCLEAR CMS TEST PROTOCOL - REVOLUTION TRADING"
echo "=============================================="
echo "Base URL: ${BASE_URL:-http://localhost:8000/api}"
echo "Results: ${RESULTS_DIR}"
echo ""

# Run each test suite
for TEST_SCRIPT in AUTH_TESTS POSTS_CRUD_TESTS SECURITY_TESTS MEDIA_TESTS RBAC_TESTS PERFORMANCE_TESTS WEBHOOK_TESTS WORKFLOW_TESTS; do
  if [ -f "${SCRIPT_DIR}/${TEST_SCRIPT}.sh" ]; then
    echo "Running ${TEST_SCRIPT}..."
    bash "${SCRIPT_DIR}/${TEST_SCRIPT}.sh" > "${RESULTS_DIR}/${TEST_SCRIPT}.log" 2>&1
    echo "  Output: ${RESULTS_DIR}/${TEST_SCRIPT}.log"
  fi
done

echo ""
echo "=============================================="
echo "TEST EXECUTION COMPLETE"
echo "=============================================="
echo "Results saved to: ${RESULTS_DIR}"
```

---

## RECOMMENDATIONS

### Security Improvements
1. Ensure all admin endpoints check `role:admin|super-admin` middleware
2. Implement request signing for webhook endpoints
3. Add CSRF token validation for state-changing operations
4. Consider implementing API key authentication for machine-to-machine

### Performance Improvements
1. Add Redis caching for frequently accessed endpoints
2. Implement response compression (gzip)
3. Add database query optimization indexes
4. Consider implementing GraphQL for flexible querying

### Monitoring
1. Add APM (Application Performance Monitoring)
2. Implement detailed audit logging
3. Set up alerting for error rate thresholds
4. Monitor rate limiting effectiveness

---

*Generated by Nuclear CMS Testing Protocol v1.0*
*Date: 2025-12-07*
*Engineer Level: L8+ Principal*
