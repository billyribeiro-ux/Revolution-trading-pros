# Admin Endpoints Audit - 404/401 Errors
**ICT 7 Principal Engineer - Frontend/Backend Endpoint Mismatch Analysis**

## Errors Reported

### 404 Errors (Endpoint Not Implemented)
1. ❌ `GET /api/admin/connections/status` - 404
2. ❌ `GET /api/admin/subscriptions/plans/stats` - 404  
3. ❌ `GET /api/admin/posts/stats` - 404

### 401 Errors (Authentication/Authorization Issue)
1. ⚠️ `GET /api/admin/products/stats` - 401
2. ⚠️ `GET /api/admin/coupons` - 401
3. ⚠️ `GET /api/admin/members/stats` - 401
4. ⚠️ `GET /api/admin/analytics/dashboard?period=30d` - 401

---

## Backend Routes Analysis

### ✅ Implemented Admin Routes
From `backend-rust/src/routes/mod.rs:230-299`:

```rust
// Users
GET    /api/admin/users
POST   /api/admin/users
GET    /api/admin/users/{id}
PUT    /api/admin/users/{id}
DELETE /api/admin/users/{id}

// Members
GET    /api/admin/members
GET    /api/admin/members/{id}

// Subscriptions
GET    /api/admin/subscriptions
GET    /api/admin/subscriptions/{id}
PUT    /api/admin/subscriptions/{id}
POST   /api/admin/subscriptions/{id}/cancel

// Products
GET    /api/admin/products
POST   /api/admin/products
GET    /api/admin/products/{id}
PUT    /api/admin/products/{id}
DELETE /api/admin/products/{id}

// Coupons
GET    /api/admin/coupons ✅
POST   /api/admin/coupons
GET    /api/admin/coupons/{id}
PUT    /api/admin/coupons/{id}
DELETE /api/admin/coupons/{id}
GET    /api/admin/coupons/user/available

// Connections
GET    /api/admin/connections
POST   /api/admin/connections/{service_key}/connect
POST   /api/admin/connections/{service_key}/disconnect
POST   /api/admin/connections/{service_key}/test

// Analytics
GET    /api/admin/analytics/dashboard ✅

// Settings
GET    /api/admin/settings
PUT    /api/admin/settings/general
PUT    /api/admin/settings/notifications
```

---

## Missing Endpoints (404s)

### 1. `/api/admin/connections/status` ❌
**Frontend expects:** `GET /api/admin/connections/status`  
**Backend has:** `GET /api/admin/connections` (no `/status` subpath)

**Fix:** Frontend should call `/api/admin/connections` instead

### 2. `/api/admin/subscriptions/plans/stats` ❌
**Frontend expects:** `GET /api/admin/subscriptions/plans/stats`  
**Backend has:** No stats endpoint for subscription plans

**Fix:** Need to implement stats endpoint or frontend should use different endpoint

### 3. `/api/admin/posts/stats` ❌
**Frontend expects:** `GET /api/admin/posts/stats`  
**Backend has:** No admin posts routes at all

**Fix:** Need to implement admin posts management routes

---

## 401 Errors Analysis

### Likely Causes:
1. **Token not being sent** - Frontend may not be attaching Bearer token
2. **Token expired** - Access token has 15min expiry
3. **Role check failing** - User role may not be "admin" or "super-admin"
4. **Auth middleware issue** - Middleware may not be recognizing token

### Endpoints Returning 401:
- `/api/admin/products/stats` - Endpoint doesn't exist (should be 404)
- `/api/admin/coupons` - **This endpoint EXISTS** - auth issue
- `/api/admin/members/stats` - Endpoint doesn't exist (should be 404)
- `/api/admin/analytics/dashboard` - **This endpoint EXISTS** - auth issue

**Note:** Some 401s should actually be 404s. Backend may be returning 401 before checking if route exists.

---

## Action Items

### Immediate Fixes (Frontend)
1. Change `/api/admin/connections/status` → `/api/admin/connections`
2. Remove calls to non-existent stats endpoints
3. Verify Bearer token is being sent with admin requests

### Backend Fixes Needed
1. Implement missing stats endpoints:
   - `/api/admin/subscriptions/plans/stats`
   - `/api/admin/products/stats`
   - `/api/admin/members/stats`
   - `/api/admin/posts/stats`

2. Add admin posts management routes
3. Verify auth middleware is checking routes before auth (404 should come before 401)

### Auth Investigation
1. Check if user role is "developer" vs "admin" (role mismatch)
2. Verify JWT token contains correct role claim
3. Check admin middleware role validation logic

---

## User Role Check

Your user: `welberribeirodrums@gmail.com`
- **Role:** `developer` (from login response)
- **Expected:** `admin` or `super-admin` for admin routes

**CRITICAL:** Backend admin middleware may be checking for `role == "admin"` but your role is `"developer"`. This would cause 401 errors on all admin routes.

---

## Next Steps

1. ✅ Check your user's role in database
2. ✅ Verify admin middleware accepts "developer" role
3. ✅ Fix frontend endpoint URLs for connections
4. ⚠️ Implement missing stats endpoints in backend
5. ⚠️ Add admin posts management routes
