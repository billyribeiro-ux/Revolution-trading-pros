# AUTH_FIX_RESULT.md — Revolution Trading Pros

**Author:** Claude Sonnet 4.6  
**Date:** 2026-04-27 (updated same day)  
**Branch:** `main` (working tree dirty, not committed)  
**Scope:** Fixes for findings C-1, H-1, H-2, H-4, H-5, H-6, H-7

---

## 1. Summary Table

| ID | Title | Status |
|----|-------|--------|
| C-1 | Banned user auth bypass (login + middleware) | **FIXED** |
| H-1 | JWT access token TTL default 24h | **FIXED** |
| H-2 | Login rate limiter fails open on Redis outage | **FIXED** |
| H-4 | Admin `create_user` skips password validation | **FIXED** |
| H-5 | `resend_verification` leaks verified status | **FIXED** |
| H-6 | Ban does not invalidate active sessions | **FIXED** |
| H-7 | Any admin can grant developer/super_admin role | **FIXED** |

**Labeling correction:** The prior revision of this document incorrectly called H-5 "M-2". M-2 in AUTH_AUDIT.md refers to the CSP `unsafe-inline` issue, which is out of scope for this pass. The resend-verification enumeration finding is H-5.

---

## 2. Per-Fix Details

### Fix H-5 — resend_verification email enumeration

**File:** `api/src/routes/auth.rs`  
**Lines changed:** ~508–521 (before) → same range (after)

**Before:**
```rust
if user.email_verified_at.is_some() {
    return Ok(Json(MessageResponse {
        message: "Your email is already verified. You can log in.".to_string(),
        success: Some(true),
    }));
}
```

**After:**
```rust
// FIX-2026-04-27 (M-2): Return identical generic message for already-verified users
// to prevent email enumeration. Do NOT send another email — it is wasted work.
if user.email_verified_at.is_some() {
    tracing::info!(
        target: "security_audit",
        event = "resend_verification_already_verified",
        ...
        "Verification resend suppressed - email already verified"
    );
    return Ok(Json(MessageResponse {
        message: "If your email is registered, you will receive a verification link shortly."
            .to_string(),
        success: Some(true),
    }));
}
```

**Verification criteria:**
- Unknown email → `"If your email is registered, you will receive a verification link shortly."`
- Unverified existing user → same generic message (email IS sent)
- Verified existing user → **same generic message** (email NOT sent; log shows `resend_verification_already_verified`)

All three branches now return the identical body. The log distinguishes the cases for operator visibility without leaking anything to the caller.

---

### Fix C-1 — Banned user auth bypass

Three changes across two files.

#### 2a. Add `is_active` to `User` struct

**File:** `api/src/models/user.rs:22`

**Before:**
```rust
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    #[sqlx(default)]
    pub role: Option<String>,
    #[sqlx(default)]
    pub email_verified_at: Option<NaiveDateTime>,
    ...
}
```

**After (added field):**
```rust
    #[sqlx(default)]
    pub role: Option<String>,
    #[sqlx(default)]
    pub is_active: Option<bool>,   // ← added
    #[sqlx(default)]
    pub email_verified_at: Option<NaiveDateTime>,
```

Typed as `Option<bool>` with `#[sqlx(default)]` so legacy rows where the column is NULL are treated as active (`None` ≠ `Some(false)`).

#### 2b. Login handler: add `is_active` to SELECT + check before JWT issuance

**File:** `api/src/routes/auth.rs`

SELECT query now includes `is_active`:
```rust
"SELECT id, email, password_hash, name, role, is_active, email_verified_at, ..."
```

Check inserted after password verification succeeds, before JWT issuance:
```rust
// FIX-2026-04-27 (C-1): Block banned/deactivated accounts at login.
if user.is_active == Some(false) {
    return Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid credentials"})),
    ));
}
```

Returns generic `"Invalid credentials"` — does not disclose ban status to the caller.

#### 2c. Auth middleware: add `is_active` to SELECT + per-request enforcement

**File:** `api/src/middleware/auth.rs`

SELECT query updated:
```rust
r#"SELECT id, email, password_hash, name, role, is_active, email_verified_at,
          avatar_url, mfa_enabled, created_at, updated_at
   FROM users WHERE id = $1"#,
```

Check added after DB fetch (before Redis cache write):
```rust
// FIX-2026-04-27 (C-1): Reject banned/deactivated users on every request.
if user.is_active == Some(false) {
    return Err((StatusCode::UNAUTHORIZED, "Account is disabled"));
}
```

**Note on Redis cache coherence:** The cache stores the full `User` JSON. After a ban, the cached object will have `is_active: null` (the old value) until it expires or is explicitly invalidated. Fix H-6 calls `invalidate_user_cache` at ban time, which clears the stale entry so the next request hits the DB and sees `is_active = false`. Without Fix H-6 the cache TTL would delay enforcement.

---

### Fix H-6 — Ban does not invalidate active sessions

**File:** `api/src/routes/admin.rs:348–390` (`ban_user` handler)

**Before:** Handler just ran the UPDATE and returned.

**After:** After the UPDATE, calls both `invalidate_all_user_sessions` and `invalidate_user_cache`, tolerating Redis failure with a warning log:

```rust
if let Some(ref redis) = state.services.redis {
    if let Err(e) = redis.invalidate_all_user_sessions(id).await {
        tracing::warn!(
            target: "security_audit",
            event = "ban_session_invalidation_failed",
            user_id = %id, error = %e,
            "Could not invalidate sessions for banned user (Redis unavailable) - DB ban still effective"
        );
    }
    if let Err(e) = redis.invalidate_user_cache(id).await {
        tracing::warn!(...);
    }
}
```

The method names `invalidate_all_user_sessions(i64)` and `invalidate_user_cache(i64)` were confirmed to exist in `api/src/services/redis.rs` lines 231 and 457 respectively.

**Verification path:**
1. User logs in → gets JWT and Redis session
2. User calls `GET /api/user/profile` → 200
3. Admin calls `POST /admin/users/:id/ban` → 200; Redis sessions cleared, user cache evicted
4. User calls `GET /api/user/profile` with same JWT → middleware fetches from DB, sees `is_active = false` → **401 "Account is disabled"**

---

### Fix H-7 — Developer/super_admin privilege escalation by any admin

**File:** `api/src/routes/admin.rs:253–320` (`update_user` handler)

**Before:** `require_admin(&user)?` was the only guard — any admin could `PUT /admin/users/:id` with `{"role": "developer"}`.

**After (guard added at top of handler):**
```rust
if let Some(ref new_role) = input.role {
    let privileged = matches!(
        new_role.as_str(),
        "developer" | "super_admin" | "super-admin"
    );
    if privileged {
        let actor_role = user.role.as_deref().unwrap_or("user");
        let actor_is_super = actor_role == "super_admin" || actor_role == "super-admin";
        if !actor_is_super {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Only super_admin can grant developer or super_admin roles"})),
            ));
        }
    }
}
```

**Audit log added after every successful role change:**
```rust
if let Some(ref new_role) = input.role {
    let details = serde_json::json!({
        "old_role": prev_role,
        "new_role": new_role,
        "actor_id": user.id,
        "target_user_id": id
    });
    sqlx::query(
        r#"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
           VALUES ($1, 'role_change', 'access_control', 'high', $2)"#,
    )
    .bind(id)
    .bind(&details)
    .execute(&state.db.pool)
    .await
    ...
}
```

Uses the existing `security_events` table (created in `migrations/015_consolidated_schema.sql:1024`, extended in `migrations/035_ict7_member_system_complete.sql`). No new migration needed — `event_category`, `severity`, and `details` (JSONB) columns already exist.

**Verification path:**
1. Regular admin → `PUT /admin/users/:id {"role": "developer"}` → **403 "Only super_admin can grant developer or super_admin roles"**
2. super_admin → same request → 200, role updated, `security_events` row inserted
3. Regular admin → `PUT /admin/users/:id {"role": "member"}` → 200, `security_events` row inserted
4. `SELECT * FROM security_events WHERE event_type = 'role_change'` → rows visible for cases 2 and 3

---

### Fix H-1 — JWT access token TTL default 24h → 1h

**Files:**
- `api/src/config/mod.rs` (default changed)
- `api/.env.example` (documented)

**Before (`config/mod.rs`):**
```rust
jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
    .unwrap_or_else(|_| "24".to_string())
    .parse()
    .unwrap_or(24),
```

**After:**
```rust
// FIX-2026-04-27 (H-1): Default TTL reduced from 24h to 1h.
jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
    .unwrap_or_else(|_| "1".to_string())
    .parse()
    .unwrap_or(1),
```

**Before (`.env.example`):**
```
# OPTIONAL — default 24 (hours). Access-token TTL.
JWT_EXPIRES_IN=24
```

**After:**
```
# OPTIONAL — default 1 (hour). Access-token TTL.
# Kept short per OWASP guidance for sites handling PII/payments.
# Frontend must use the refresh-token flow to obtain a new access token.
JWT_EXPIRES_IN=1
```

**Verification path:** After restarting the container, POST `/api/auth/login`, base64-decode the JWT payload, confirm `exp - iat ≈ 3600` (1 hour in seconds). The existing refresh-token rotation flow (already in place) lets the frontend maintain sessions transparently.

**Note:** Any deployment that was relying on the 24h default will now issue 1h tokens. If an existing `JWT_EXPIRES_IN=24` is set explicitly in fly secrets, that value takes precedence and is unchanged. Only operators using the default are affected.

---

### Fix H-2 — Login rate limiter fails open on Redis outage

**File:** `api/src/routes/auth.rs` (login handler, lines ~632–703 before fix)

The login handler had two fail-open rate limit blocks:
1. **Per-IP:** Wrapped in `if let Some(redis) = ...` — silently skipped if Redis was `None`.
2. **Per-email:** `else` branch explicitly returned `allowed: true` when Redis was `None`.

Both now fail closed, matching the behavior already in place for `/register`, `/forgot-password`, and `/reset-password` (which used the existing `enforce_ip_rate_limit_strict` helper).

**Before (per-IP — fail open):**
```rust
if let Some(redis) = state.services.redis.as_ref() {
    let key = format!("login_ip:{}", ip);
    match redis.check_ip_rate_limit(&key, 10, 900).await {
        ...
        Err(e) => {
            tracing::warn!("Per-IP rate limit check failed (fail-open ...)");
            // ← no return, request continues
        }
    }
} // ← no Redis = no check at all
```

**Before (per-email — fail open):**
```rust
let rate_limit_result = if let Some(redis) = &state.services.redis {
    redis.check_login_rate_limit(&input.email).await
} else {
    Ok(RateLimitResult { allowed: true, remaining: 999, ... }) // ← explicit pass
};
...
Err(e) => {
    tracing::warn!("Rate limit check failed ... continuing without rate limiting");
    // ← no return, request continues
}
```

**After (both fail closed):**
```rust
// Per-IP: delegates to enforce_ip_rate_limit_strict (already fail-closed)
enforce_ip_rate_limit_strict(&state, &ip, "login_ip", 10, 900).await?;

// Per-email: fail closed if Redis is None
let Some(ref redis) = state.services.redis else {
    return Err((SERVICE_UNAVAILABLE, json!({"error": "Service temporarily unavailable..."})));
};

match redis.check_login_rate_limit(&input.email).await {
    Ok(rate_limit) if !rate_limit.allowed => { /* 429 */ }
    Ok(_) => {}
    Err(e) => {
        // FAIL CLOSED
        return Err((SERVICE_UNAVAILABLE, json!({"error": "..."})));
    }
}
```

**Verification path:** Stop the Redis container. Attempt to POST `/api/auth/login`. Must receive HTTP 503, not 200/401. (Without this fix it would fall through to password verification.)

**Note on `RateLimitService`:** `api/src/services/rate_limit.rs` contains a standalone multi-tier (Redis → memory → DB) `RateLimitService` that is not wired into `AppState`. Wiring it would require changes to `Services`, `AppState`, and `main.rs` — exceeding the 30-line budget. The existing `enforce_ip_rate_limit_strict` + Redis-direct path with fail-closed behavior satisfies the security requirement. Wiring the full multi-tier service is a separate refactor (noted in §6 as N-5).

---

### Fix H-4 — Admin create_user skips password validation

**File:** `api/src/routes/admin.rs` (`create_user` handler, ~line 217)

**Before:**
```rust
require_admin(&user)?;

let password_hash = crate::utils::hash_password(&input.password).map_err(...)?;
// validate_password never called — "a" accepted, hashed, stored
```

**After:**
```rust
require_admin(&user)?;

// FIX-2026-04-27 (H-4): Validate password strength before hashing.
crate::utils::validate_password(&input.password).map_err(|e| {
    (StatusCode::BAD_REQUEST, Json(json!({"error": e})))
})?;

let password_hash = crate::utils::hash_password(&input.password).map_err(...)?;
```

**Verification path:** As admin, POST `/api/admin/users` with `{"password": "a", ...}`. Must receive HTTP 400 with `{"error": "Password must be at least 12 characters"}`. Previously returned 201.

---

## 3. Cumulative Line Count

| File | Lines Added | Lines Deleted |
|------|-------------|---------------|
| `api/src/models/user.rs` | 2 | 0 |
| `api/src/middleware/auth.rs` | 17 | 3 |
| `api/src/routes/auth.rs` | 37 | 42 |
| `api/src/routes/admin.rs` | 72 | 13 |
| `api/src/config/mod.rs` | 3 | 2 |
| `api/.env.example` | 3 | 2 |
| **Total** | **134** | **62** |

Net: +72 lines across 6 files.

---

## 4. Surprises / Unexpected Findings

1. **Redis cache can serve stale `is_active`:** The auth middleware's Redis cache stores the User struct JSON. If a ban happens while a cached entry is live, the cached User has no `is_active = false` until the cache expires or is evicted. This is why Fix H-6's `invalidate_user_cache` call is load-bearing for Fix C-1 to take effect immediately. Without it, the ban would only propagate after the Redis TTL expires (default 300s per `api/src/services/redis.rs`).

2. **`is_active` column was missing from `User` struct entirely.** The `AdminUserRow` struct (used only in admin list/get endpoints) did have `is_active: bool`, but the core `User` struct used by every auth path did not. This means any prior code that wanted to check `is_active` on an authenticated user had no way to do so — the field was simply never fetched.

3. **`require_super_admin` was `#[allow(dead_code)]`** at `admin.rs:56`. The function exists and is correct, but was unused. Fix H-7 does not use it (inline check is cleaner for this specific guard), but the dead_code suppression is a signal that super_admin-level enforcement was never actually applied to any handler before this fix.

4. **`deactivate_account` in `user.rs` sets `password_hash = 'DEACTIVATED'`** (noted in audit, not fixed). This causes a `verify_password` error (not a clean 401) if the user somehow gets past the `is_active` check. Fix C-1 blocks the login before password verification, so this is now less dangerous, but the deactivate handler should also set `is_active = false` for consistency. **Flagged but not fixed per the no-while-I'm-here rule.**

5. **`enforce_ip_rate_limit_strict` already existed** at `auth.rs:69` and was already used for `/register`, `/forgot-password`, `/reset-password`. The login handler was an explicit carve-out that used a different, fail-open pattern. The inconsistency was not documented anywhere.

---

## 5. Final Gate Results

```
cargo check                    ✅  Success (0 errors, 0 warnings)
cargo test --test utils_test   ✅  17/17 passed
cargo test --test stripe_test  ✅  15/15 passed
pnpm check                     ✅  0 errors, 0 warnings (5215 files)
```

`docker compose ps` not run — the local Docker stack was not started during this session. All changes are pure Rust API logic (no schema changes, no new env vars). The H-1 JWT TTL change does require a container restart to take effect (config is read at startup).

---

## 6. NOTES — Bugs Discovered But NOT Fixed

| ID | Location | Issue | Why Not Fixed |
|----|----------|-------|---------------|
| N-1 | `api/src/routes/user.rs` — `deactivate_account` | Sets `password_hash = 'DEACTIVATED'` but does NOT set `is_active = false`. Fix C-1 guards the login path, but the deactivation is semantically incomplete. | Out of scope; would also need session invalidation to be symmetric with ban. |
| N-2 | `api/src/middleware/auth.rs:86–98` | Redis cache hit path returns `User` struct deserialized from cached JSON. A user banned while a cache entry is live may have their ban take up to ~300s to propagate if Fix H-6's `invalidate_user_cache` call fails. Not a logic error in the fix, but a latency window. | Acceptable residual risk given H-6's cache eviction. Documented. |
| N-3 | `api/src/routes/admin.rs` — `require_super_admin` | Function exists at line 57 but is `#[allow(dead_code)]`. Fix H-7 added an inline check rather than calling this helper. | Refactoring, not a security fix. Out of scope. |
| N-4 | `api/src/routes/admin.rs` — `update_user` | Old role is fetched with a separate SELECT before the UPDATE. TOCTOU window: audit log could record a stale `old_role` under concurrent updates. | Fixing requires a transaction; would exceed 30-line budget. |
| N-5 | `api/src/services/rate_limit.rs` | `RateLimitService` (Redis → memory → DB fallback) exists but is never instantiated in `AppState`. Fix H-2 uses `enforce_ip_rate_limit_strict` + direct Redis, which is fail-closed but Redis-only. The memory/DB tiers are dead code. | Wiring into `AppState` + `Services` + `main.rs` exceeds 30-line budget and is a standalone refactor. |
