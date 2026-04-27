# AUTH_AUDIT.md — Revolution Trading Pros

**Auditor:** Claude Sonnet 4.6 (adversarial, read-only pass)  
**Date:** 2026-04-27  
**Scope:** Authentication, authorization, secrets management, CSRF, CSP, session handling, Stripe webhook  
**Branch:** `main` (HEAD at audit time)

---

## ⚠️ CRITICAL FINDING — READ BEFORE ANYTHING ELSE

**R2 cloud storage credentials are present in plaintext in `api/.env`.**

```
R2_ACCESS_KEY_ID=c59b71db89704deb941119ac5324a12a
R2_SECRET_ACCESS_KEY=1b9b305088fc88c42321e84b91125d9302cc65fd98d6d6c0520d390a7848e03f
```

Confirmed in `api/.env` (local, not committed to git — `.gitignore` correctly excludes it).  
The `.env` file is **not in the git index** (`git ls-files` returned exit 1), so it has not been committed to history. **However**, these are real credential values, not placeholders, which means they are credentials for the live Cloudflare R2 account. Any developer who has ever cloned this repo and run `git status` could see them on-screen. They should be rotated **now**, regardless of git status.

**Status of other secrets in `api/.env`:** JWT_SECRET contains a placeholder (`replace-me-with-a-long-random-secret-at-least-32-characters-long`), not a real value. Stripe, Postmark, and OAuth secrets are blank. Only R2 contains real values.

**Action required immediately:** Rotate `R2_ACCESS_KEY_ID` and `R2_SECRET_ACCESS_KEY` in Cloudflare R2 console. Do not wait for rest of audit remediation.

---

## 1. Executive Summary

**Ready for paying customers? NO.**

The codebase shows substantial recent hardening work (Argon2id, per-email rate limiting, token-type segregation, refresh token reuse detection, webhook signature verification). This is not a neglected codebase — multiple security passes have clearly occurred. However, several significant issues remain that are disqualifying for a site handling PII and payments:

### Blocking Issues (must fix before any production traffic)

1. **Banned/deactivated users can still authenticate.** `is_active = false` and `status = 'deleted'`/`'deactivated'` are **not checked** in the login handler or the JWT auth middleware. An admin who bans a user via `POST /admin/users/:id/ban` has no effect on existing JWTs or future logins by that user.

2. **JWT access token TTL is 24 hours (default).** OWASP and industry guidance is 15–60 minutes for sites with PII/payments. A stolen access token gives an attacker a 24-hour exploitation window. No in-request blacklist check at non-logout time (see item 8 below — blacklist only runs at request time, not inside the JWT itself).

3. **OAuth tokens passed in URL query string.** After Google/Apple sign-in, the JWT access token and refresh token are embedded in the redirect URL (`/auth/callback?token=...&refresh_token=...`). These tokens appear in server access logs, Cloudflare logs, browser history, Referer headers, and analytics tools. The code contains an acknowledged comment about this trade-off but has no mitigating mechanism (no immediate URL clearing, no token TTL shortened for URL-delivered tokens).

4. **Impersonation endpoint returns a non-functional placeholder token.** `POST /admin/users/:id/impersonate` returns `impersonate_{id}_{timestamp}` — a non-JWT string that would pass no middleware check, so it is harmless in isolation. But the endpoint is live, guarded only by `require_super_admin`, and the comment says "In a real implementation, you would generate a JWT." If this is ever wired up to actually issue a JWT, it will need full audit.

5. **`resend_verification` endpoint leaks verified status.** The endpoint returns `"Your email is already verified. You can log in."` when called for a verified address — a different response than for an unknown address — enabling email enumeration to determine both existence AND verification state.

6. **CSP contains `'unsafe-inline'` for script-src.** `script-src 'self' 'unsafe-inline'` on the API's CSP header eliminates the XSS protection that CSP is meant to provide. The Rust API serves JSON, not HTML, so this is lower risk for the API itself, but the SvelteKit frontend's hooks.server.ts does not set a CSP header at all (only non-CSP security headers are set).

### High-Severity Issues (must fix before public launch)

7. **Login rate limiter fails open on Redis outage.** The per-email rate limiter at `api/src/routes/auth.rs:697` falls back to `allowed: true` when Redis is unavailable. The per-IP rate limiter on login also explicitly fails open. This means a Redis outage (or the common case: no Redis configured at all) disables all brute-force protection on the login endpoint.

8. **Blacklist check fails open in auth middleware.** `api/src/middleware/auth.rs:71–80`: when Redis blacklist check fails (`Err(e)`), the request is allowed through. This is documented as intentional, but means a Redis failure at logout time leaves active JWTs unrevocable for up to 24 hours.

9. **Password requirements violate NIST SP 800-63B.** The validator at `api/src/utils/mod.rs:72–127` requires uppercase, lowercase, digit, and special character — character-class requirements explicitly deprecated by NIST 800-63B (2017) and not in the 2024 SP 800-63B-4 revision. More importantly, the `validate_password()` function is **not called** in the admin user-creation path (`api/src/routes/admin.rs:210–247`: `create_user` handler calls only `hash_password`, not `validate_password`), allowing an admin to create users with passwords like "a".

10. **`developer` role bypasses email verification and has full admin access.** A user with `role = "developer"` in the DB bypasses email verification (`auth.rs:781`) and passes the admin check (`admin.rs:30`). This role is not created by the seed script but could be assigned by any admin via `PUT /admin/users/:id` with `{"role": "developer"}`. No `require_super_admin` guard prevents a regular admin from promoting themselves or others to `developer`.

---

## 2. Findings by Severity

### CRITICAL

#### C-1: Banned/Deactivated Users Can Authenticate
**File:** `api/src/routes/auth.rs:704–710`, `api/src/middleware/auth.rs:103–118`  
**Evidence:** The login query selects by email only:
```sql
SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at 
FROM users WHERE email = $1
```
`is_active`, `status`, and `deleted_at` are never read from the database during login or JWT validation. A user banned via `POST /admin/users/:id/ban` (which sets `is_active = false`) can still log in. A user "deleted" via `POST /api/user/deactivate` (which sets `status = 'deleted'` and replaces the email with `deleted_{id}@deactivated.local`) would be blocked only because the email no longer matches — but their existing JWT remains valid for 24 hours.  
**Severity:** CRITICAL  
**Passing would require:** Add `AND is_active = true` to the login user lookup query; add `is_active` to the `User` struct; check it in `User::from_request_parts()` after DB fetch.

#### C-2: R2 Credentials in Local .env (Not in Git, But Real)
**File:** `api/.env` (local, untracked)  
**Evidence:** Lines contain real Cloudflare R2 access key ID and secret access key, not placeholder values.  
**Severity:** CRITICAL (requires immediate credential rotation regardless of git status)

---

### HIGH

#### H-1: JWT Access Token TTL = 24 Hours
**File:** `api/src/config/mod.rs:205–208`, `api/.env.example:33`  
**Evidence:**
```rust
jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
    .unwrap_or_else(|_| "24".to_string())
    .parse()
    .unwrap_or(24),
```
Default is 24 hours. `.env.example` documents this default. OWASP recommends 15–60 minutes for access tokens on financial/PII applications.  
**Severity:** HIGH  
**Passing would require:** `JWT_EXPIRES_IN=1` (or max 1 hour) in prod configuration; document this requirement in `ENV_VARS.md`.

#### H-2: Login Rate Limiter Fails Open on Redis Unavailability
**File:** `api/src/routes/auth.rs:664–701`  
**Evidence:**
```rust
let rate_limit_result = if let Some(redis) = &state.services.redis {
    redis.check_login_rate_limit(&input.email).await
} else {
    Ok(crate::services::redis::RateLimitResult {
        allowed: true,
        remaining: 999,
        ...
    })
};
...
Err(e) => {
    tracing::warn!("Rate limit check failed (Redis unavailable): {} - continuing without rate limiting", e);
}
```
If Redis is not configured or fails, the per-email check returns `allowed: true`. The in-memory fallback in `api/src/services/rate_limit.rs` is defined but the code in `auth.rs` uses `state.services.redis` directly, bypassing the `RateLimitService` fallback entirely.  
**Severity:** HIGH  
**Passing would require:** Use the `RateLimitService` with its multi-tier fallback (Redis → memory → database) instead of directly calling `state.services.redis`.

#### H-3: OAuth JWT Tokens in Redirect URL
**File:** `api/src/routes/oauth.rs:843–854` (Google), `api/src/routes/oauth.rs:1073–1083` (Apple)  
**Evidence:**
```rust
let callback_url = format!(
    "{}/auth/callback?provider=google&token={}&refresh_token={}&session_id={}&expires_in={}",
    state.config.app_url,
    urlencoding::encode(&auth_response.token),
    urlencoding::encode(&auth_response.refresh_token),
    ...
```
Full JWT access token and refresh token are in the URL query string. Code contains a `SECURITY NOTE` comment acknowledging this. Tokens will appear in Cloudflare access logs, any CDN, browser history, Referer headers.  
**Severity:** HIGH  
**Passing would require:** Use server-side session cookie to pass the callback token, or exchange the URL token for a one-time code that the frontend redeems immediately via a server-side POST.

#### H-4: Admin `create_user` Bypasses Password Validation
**File:** `api/src/routes/admin.rs:210–248`  
**Evidence:**
```rust
async fn create_user(...) {
    require_admin(&user)?;
    let password_hash = crate::utils::hash_password(&input.password).map_err(|e| {
```
`validate_password()` is never called. An admin can create a user with password `"a"` (1 character). The regular `register` endpoint calls `validate_password`; this path does not.  
**Severity:** HIGH  
**Passing would require:** Call `validate_password(&input.password)?` before `hash_password`.

#### H-5: `resend_verification` Enables Email Verification Status Enumeration
**File:** `api/src/routes/auth.rs:508–521`  
**Evidence:**
```rust
if user.email_verified_at.is_some() {
    return Ok(Json(MessageResponse {
        message: "Your email is already verified. You can log in.".to_string(),
```
vs. unknown email returns:
```rust
return Ok(Json(MessageResponse {
    message: "If your email is registered, you will receive a verification link shortly.".to_string(),
```
An attacker can distinguish: (a) email unknown → generic message; (b) email known + verified → distinct message. This reveals both email existence and verification state.  
**Severity:** HIGH  
**Passing would require:** Return the same generic message for all three cases (unknown, known unverified, known verified).

#### H-6: Banned User Status Not Propagated to Active Sessions
**File:** `api/src/routes/admin.rs:348–368` (ban), `api/src/middleware/auth.rs`  
**Evidence:** `ban_user` sets `is_active = false` but does not call `redis.invalidate_all_user_sessions()`. The JWT middleware does not check `is_active`. A banned user with a valid JWT continues to have API access for up to 24 hours (see H-1).  
**Severity:** HIGH  
**Passing would require:** `ban_user` must call `redis.invalidate_all_user_sessions(id)` and the auth middleware must check `is_active` after DB fetch.

#### H-7: `developer` Role Privilege Escalation by Any Admin
**File:** `api/src/routes/admin.rs:253–320` (update_user), `api/src/middleware/admin.rs:27–30`  
**Evidence:** `update_user` accepts `{"role": "developer"}` and passes `require_admin` (which allows `admin` role). Any admin can promote a user to `developer`, which bypasses email verification and has full admin access. No `require_super_admin` guard on the role-update code path. No audit log is written for role changes.  
**Severity:** HIGH  
**Passing would require:** Validate that the `role` field in `UpdateUserRequest` is not `developer`, `super_admin`, or `super-admin` unless the requestor is themselves a `super_admin`; add an audit log INSERT on role changes.

---

### MEDIUM

#### M-1: Frontend JWT Decoded Client-Side on API Failure (No Signature Verification)
**File:** `frontend/src/hooks.server.ts:267–284`  
**Evidence:**
```typescript
const payload = JSON.parse(Buffer.from(parts[1], 'base64').toString());
event.locals.user = {
    id: Number(payload.sub || ...),
    email: payload.email || 'unknown@temp.local',
    role: payload.role || 'user'
};
```
On transient API failure, the server hook decodes the JWT payload without verifying the signature. An attacker who can cause a transient failure (flood the API, kill Redis) and also inject a crafted JWT into the `rtp_access_token` cookie could land on admin pages during the failure window. Note: the backend still enforces auth on API calls, so this is a frontend gate bypass only — the admin panel would show, but subsequent API calls would fail validation.  
**Severity:** MEDIUM  
**Passing would require:** Remove the JWT decode-without-verify path; instead fail closed and redirect to login on API failure for protected routes.

#### M-2: CSP Contains `'unsafe-inline'` for `script-src`
**File:** `api/src/main.rs:190`  
**Evidence:**
```
script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'
```
`'unsafe-inline'` in `script-src` negates XSS protection from CSP. The frontend `hooks.server.ts` does not set a CSP header (only sets `X-Frame-Options`, `X-XSS-Protection`, HSTS, etc.).  
**Severity:** MEDIUM  
**Passing would require:** Remove `'unsafe-inline'` from `script-src`; use nonces or hashes. Add CSP to SvelteKit security headers handler.

#### M-3: `frame-ancestors` Not in CSP
**File:** `api/src/main.rs:190`  
**Evidence:** The CSP string does not include `frame-ancestors`. `X-Frame-Options: DENY` is set separately, but `frame-ancestors` in CSP supersedes `X-Frame-Options` in modern browsers and is the recommended mechanism.  
**Severity:** MEDIUM  
**Passing would require:** Add `frame-ancestors 'none'` (or `'self'` if framing is needed) to the CSP header.

#### M-4: Per-IP Login Rate Limit Bypassed by Rotating IPs; IP Trust
**File:** `api/src/routes/auth.rs:48–64`  
**Evidence:** The `client_ip` function trusts `x-forwarded-for` without validation:
```rust
if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
    if let Some(first) = xff.split(',').next() {
```
When deployed behind Cloudflare (the evident deployment), Cloudflare sets `CF-Connecting-IP`, not `X-Forwarded-For`. Using `X-Forwarded-For` allows an attacker to spoof the IP by setting the header, bypassing per-IP rate limits. Additionally, the per-IP check on login only fails open on Redis error.  
**Severity:** MEDIUM  
**Passing would require:** Use `CF-Connecting-IP` when behind Cloudflare (or configure Fly.io to inject real IP); fail the per-IP check closed for login.

#### M-5: Refresh Token Blacklist Write Failure Leaves Old Token Valid
**File:** `api/src/routes/auth.rs:1019–1031`  
**Evidence:**
```rust
if let Err(e) = redis.blacklist_token(&token_hash, ttl).await {
    tracing::warn!(..., "Failed to blacklist old refresh token");
}
```
If the Redis write fails after the new refresh token has been issued, the old refresh token is still usable. An attacker in possession of the old token can now create two concurrent valid sessions. This is documented as a known trade-off but is worth flagging as a medium risk.  
**Severity:** MEDIUM

#### M-6: `impersonate_user` Returns Non-JWT Placeholder Token (Unfinished Feature Live)
**File:** `api/src/routes/admin.rs:1614–1668`  
**Evidence:**
```rust
let token = format!("impersonate_{}_{}", target_user.id, chrono::Utc::now().timestamp());
```
The endpoint is live at `POST /admin/users/:id/impersonate`. The "token" it returns is not a JWT. If the frontend ever tries to use it as a Bearer token, the middleware rejects it immediately. But the endpoint exists, its access control is checked only against `require_super_admin` (via a string role check, not the `SuperAdminUser` extractor), and there is no rate limit or audit log specific to impersonation attempts.  
**Severity:** MEDIUM

#### M-7: No Audit Log for Role Changes or Admin Actions
**File:** `api/src/routes/admin.rs:253–320`, `api/src/routes/security.rs`  
**Evidence:** `update_user`, `ban_user`, `grant_membership` do not write to any audit log table. The `security_events` table exists (read by `GET /api/security/events`) but no admin write path inserts into it. Role escalation events are entirely invisible post-hoc.  
**Severity:** MEDIUM  
**Passing would require:** INSERT into `security_events` (or an audit table) on: role change, user ban, membership grant.

#### M-8: `reset_password` Does Not Invalidate Existing Sessions
**File:** `api/src/routes/auth.rs:1327–1376`  
**Evidence:** After a successful password reset, the handler only updates `password_hash` and deletes the reset token. It does not call `redis.invalidate_all_user_sessions(user_id)`. An attacker who obtained the user's session (via XSS, network intercept, etc.) retains access even after the user resets their password.  
**Severity:** MEDIUM  
**Passing would require:** Add `redis.invalidate_all_user_sessions(user_id).await` after the `UPDATE users SET password_hash` succeeds.

---

### LOW

#### L-1: Password Complexity Requirements Violate NIST SP 800-63B
**File:** `api/src/utils/mod.rs:72–127`  
**Evidence:** Rules require uppercase, lowercase, digit, and special character. NIST 800-63B (2017) §5.1.1.2 states verifiers SHOULD NOT impose character composition rules. The intent is to allow passphrases. The minimum length (12) is correct.  
**Severity:** LOW  
**Passing would require:** Remove character-class requirements; optionally add HIBP breach checking.

#### L-2: `generate_session_id` Uses `rand::thread_rng()`
**File:** `api/src/utils/mod.rs:295–300`  
**Evidence:**
```rust
pub fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
```
`rand::thread_rng()` is cryptographically secure on platforms that seed from the OS, but the preferred Rust idiom is `rand::rngs::OsRng`. The same applies to `generate_verification_token` and `generate_password_reset_token`. Functionally equivalent on Linux/macOS but more explicit about intent.  
**Severity:** LOW

#### L-3: `validate_coupon` Endpoint Is Public
**File:** `api/src/routes/admin.rs:721–773`  
**Evidence:** `validate_coupon` accepts `GET /admin/coupons/validate/:code` but does NOT call `require_admin(&user)?`. Any unauthenticated caller can check whether a coupon code is valid. This reveals coupon existence, validity, discount details, expiry, and usage counts.  
**Severity:** LOW (coupon enumeration is not PII, but leaks business data)  
**Passing would require:** Add `require_admin(&user)?` or move to a gated checkout flow.

#### L-4: `logout` Requires a JSON Body
**File:** `api/src/routes/auth.rs:1066–1121`  
**Evidence:**
```rust
async fn logout(
    ...
    Json(input): Json<LogoutRequest>,
```
If the client sends logout without a body, Axum will return 422. This is a usability issue but also means clients that do not send `Content-Type: application/json` will get a 415 error instead of being logged out, potentially leaving tokens active.  
**Severity:** LOW

#### L-5: SvelteKit Hook Does Not Set Content-Security-Policy
**File:** `frontend/src/hooks.server.ts:322–444`  
**Evidence:** The `securityHeaders` handler sets `X-XSS-Protection`, `X-Content-Type-Options`, `X-Frame-Options`, HSTS, `Permissions-Policy`, and COOP/CORP but does **not** set `Content-Security-Policy`. The CSP the user receives for the SvelteKit frontend comes only from the Rust API if the frontend is behind it; but if the frontend (Cloudflare Pages) is separate from the API (Fly.io), the frontend has no CSP at all.  
**Severity:** LOW (blocks some XSS attack classes; important for a financial site)

#### L-6: `admin.rs` Uses Dynamic SQL String Building With Column Names
**File:** `api/src/routes/admin.rs:261–293` (`update_user`), `api/src/routes/admin.rs:1175–1213` (`update_user_membership`)  
**Evidence:** Column names like `"name = $2"`, `"email = $3"`, etc. are assembled via `format!` and `Vec::push_str` — not via user-supplied data, but via hard-coded Rust strings. The values are still parameterized. This is not a SQL injection risk (the column names come from `if input.name.is_some()` branches), but the pattern is worth noting as something that should stay that way.  
**Severity:** LOW (informational — currently safe, but fragile pattern)

---

## 3. Layer 1 Checklist Results

### Authentication

**1. Password storage — Argon2id**  
**Status:** PASS  
**Evidence:** `api/src/utils/mod.rs:151–169`:
```rust
let params = Params::new(65536, 3, 4, Some(32))...;
let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
```
Parameters: 64 MiB memory, 3 iterations, 4 lanes, 32-byte output. This meets and exceeds OWASP 2024 minimums (19456 KiB, t=2, p=1). Salt is per-user via `SaltString::generate(&mut OsRng)`. bcrypt legacy support is present for migration compatibility.

---

**2. Password requirements**  
**Status:** PARTIAL  
**Evidence:** Minimum 12 characters (correct), maximum 128 (correct). However, character-class requirements (uppercase, lowercase, digit, special) are present (`utils/mod.rs:72–127`) in violation of NIST SP 800-63B. HIBP check is absent. Admin create-user path bypasses validation entirely (see H-4).

---

**3. Login throttling**  
**Status:** PARTIAL  
**Evidence:**  
- Per-email rate limit: implemented via Redis `check_login_rate_limit` at `auth.rs:666–701`. 10 attempts per 15-minute window. Multi-tier fallback exists in `rate_limit.rs` but is not used here.  
- Per-IP rate limit: implemented at `auth.rs:631–662`, but **fails open** on Redis error or absence.  
- Both limits fail open when Redis is unavailable (see H-2).

---

**4. Account enumeration prevention**  
**Status:** PARTIAL  
**Evidence:**  
- Login: same `"Invalid credentials"` response for wrong password and unknown user; hash_dummy_password() called for timing. PASS for login.  
- Registration: `"Email already registered"` response leaks email existence. This is common/acceptable for registration forms but worth noting.  
- `resend_verification`: returns different message for verified vs. unverified vs. unknown (see H-5). FAIL.  
- `forgot_password`: correctly returns generic message. PASS.

---

**5. Session management — cookie flags**  
**Status:** PASS  
**Evidence:** `frontend/src/routes/api/auth/login/+server.ts:60–78`:
```typescript
cookies.set('rtp_access_token', data.token, {
    path: '/',
    httpOnly: true,
    secure: isSecure,
    sameSite: 'lax',
    maxAge: data.expires_in || 3600
});
```
`httpOnly: true`, `secure: isSecure` (true in production), `sameSite: 'lax'`. These are correct. TTL concern is noted under H-1 (24h default). Refresh token is set at 30 days (`maxAge: 60 * 60 * 24 * 30`), which is at the outer boundary of acceptable but not disqualifying.

---

**6. JWT correctness**  
**Status:** PASS  
**Evidence:** `api/src/utils/mod.rs:273–291`:
```rust
let validation = Validation::new(JwtAlgorithm::HS256);
```
Algorithm pinned to HS256. `token_type` claim enforced — access tokens rejected when expected_type is "refresh" and vice versa. `sub`, `exp`, `iat` claims present. Secret from env var (required, not defaulted). Expiry checked by `jsonwebtoken` library.

---

**7. Refresh token rotation**  
**Status:** PASS with caveat  
**Evidence:** `api/src/routes/auth.rs:907–1042`: on each `/refresh` call, the old token is blacklisted in Redis, a new token is issued. Reuse detection is implemented — if a blacklisted token is presented, all sessions are invalidated. Caveat: blacklist write failure leaves old token valid (see M-5).

---

**8. Logout — server-side invalidation**  
**Status:** PASS with caveat  
**Evidence:** `api/src/routes/auth.rs:1066–1121`: logout blacklists the JWT hash in Redis and invalidates the session. Caveat: blacklist check in middleware fails open on Redis error (see finding H-2). There is no persistent DB token store (refresh tokens are not in a DB table), so revocation entirely depends on Redis availability.

---

**9. Email verification**  
**Status:** PASS  
**Evidence:** Registration issues a 24h token (`NOW() + INTERVAL '24 hours'`). Token is hashed (SHA-256) before storage. Raw token sent to user. Token deleted on use (`DELETE FROM email_verification_tokens WHERE id = $1`). Login blocked for unverified users (with documented developer/superadmin bypass).

---

**10. Password reset**  
**Status:** PARTIAL  
**Evidence:**  
- 1-hour expiry: `NOW() + INTERVAL '1 hour'`. PASS.  
- Single-use: token deleted after use. PASS.  
- Token entropy: 64-character alphanumeric token generated via `rand::thread_rng()`. ~380 bits. PASS.  
- Email enumeration: generic response. PASS.  
- **FAIL:** Does not invalidate existing sessions on reset (see M-8).

---

**11. OAuth (Google + Apple)**  
**Status:** PARTIAL  
**Evidence:**  
- State parameter for CSRF: stored in DB, validated on callback, marked used. PASS.  
- PKCE for Google: implemented (code_verifier stored in DB, code_challenge sent). PASS.  
- Apple nonce validation: implemented with SHA-256 hash comparison. PASS.  
- Redirect URI: built from `state.config.app_url` (env var). No hardcoded values. PASS.  
- **FAIL:** Tokens passed in callback URL (see H-3).

---

### Authorization (RBAC)

**12. Default deny**  
**Status:** PASS  
**Evidence:** The `User` extractor (`middleware/auth.rs`) rejects unauthenticated requests. Every admin handler calls `require_admin(&user)?`. Public routes (health, register, login, public post/coupon reads) are explicit exceptions. No globally-unguarded handler was found for sensitive operations.

---

**13. Server-side enforcement**  
**Status:** PASS  
**Evidence:** All authorization checks (`require_admin`, `AdminUser` extractor, `SuperAdminUser` extractor) are in the Rust API. The SvelteKit frontend's `hooks.server.ts` only guards navigation; it does not substitute for API-level auth.

---

**14. Object-level authorization (BOLA/IDOR)**  
**Status:** PASS for user-owned resources  
**Evidence:**  
- `cancel_membership` (`user.rs:201–264`): `WHERE id = $1 AND user_id = $2` — binds both membership_id and user.id. PASS.  
- `get_membership_details` (`user.rs:274–283`): same dual-bind pattern. PASS.  
- `delete_payment_method` (`user.rs:1190–1208`): verifies `payment_method.customer == customer_id` derived from user's own subscriptions. PASS.  
- Admin endpoints (`admin.rs`) operate on any ID by design and are guarded by `require_admin`.

---

**15. Privilege escalation prevention**  
**Status:** FAIL  
**Evidence:**  
- A user cannot update their own role via `PUT /api/user/profile` (the struct only has name, email, password fields). PASS for self-promotion via profile update.  
- However, any admin can promote any user to `developer` via `PUT /admin/users/:id` with `{"role": "developer"}`, which grants full admin access and verification bypass (see H-7).  
- No audit log for role changes.

---

**16. Permission revocation propagation**  
**Status:** FAIL  
**Evidence:** `ban_user` (`admin.rs:348–368`) sets `is_active = false` but does not invalidate sessions or blacklist JWTs. With a 24h access token TTL and no `is_active` check in the middleware, a banned user has up to 24 hours of continued access (see C-1, H-1, H-6).

---

### CRUD Endpoints (Posts/Users Sample Audit)

**17. Users CRUD**  
- **Create:** Admin only (PASS), but password validation skipped (see H-4, FAIL).  
- **Read (list):** Admin only, `password_hash` not in `AdminUserRow` response struct (PASS). Password hash is in `UserResponse` but marked `#[serde(skip_serializing)]` (PASS).  
- **Read (detail):** Admin only with `require_admin` (PASS). No password hash in response.  
- **Update:** Admin only (PASS). Accepts `{"role": "developer"}` without super_admin guard (FAIL, see H-7). No immutable-field protection for `id`, `created_at` (both absent from `UpdateUserRequest` — PASS by omission).  
- **Delete:** Admin only (PASS). Hard delete with no soft-delete option.

---

### Seeding

**18. Default admin password**  
**Status:** PASS  
**Evidence:** `api/scripts/seed-local-admin.sh` requires password as CLI argument. `DEVELOPER_BOOTSTRAP_PASSWORD_HASH` is read from env var (an Argon2id hash must be supplied). No hardcoded default password exists in committed code.

---

**19. Production safety of seeds**  
**Status:** PASS  
**Evidence:** `seed-local-admin.sh` is a shell script that runs against the local Docker stack only (hardcoded to `rtp-db` container). `Config::from_env()` includes a production-check guard (`api/src/config/mod.rs:143–158`): panics if `ENVIRONMENT=development` but `APP_URL` contains a known production hostname. The bootstrap-from-env path (`bootstrap_developer`) is idempotent.

---

**20. Bootstrap admin uniqueness**  
**Status:** PASS  
**Evidence:** `seed-local-admin.sh` uses `ON CONFLICT (email) DO UPDATE` — idempotent. The Rust bootstrap path (if implemented) would need to be verified separately; `db.bootstrap_developer` is called at startup but its implementation was not examined.

---

### Audit Logging

**21. Privileged action logging**  
**Status:** PARTIAL  
**Evidence:** Login and logout are logged to `tracing` with `target: "security_audit"`. OAuth actions log to `oauth_audit_log` table. Role changes, membership grants, and bans are **not** logged to any audit table (see M-7).

---

**22. Log integrity**  
**Status:** NOT APPLICABLE (not implemented)  
**Evidence:** No append-only constraint, separate database role, or external log sink is implemented. Tracing logs go to stdout. No log integrity mechanism exists. For a pre-launch site this is acceptable but must be addressed before scale.

---

**23. Retention policy**  
**Status:** NOT APPLICABLE  
**Evidence:** No log retention policy is defined or enforced.

---

### Defense in Depth

**24. HTTPS enforcement**  
**Status:** PASS  
**Evidence:** HSTS header is set: `Strict-Transport-Security: max-age=31536000; includeSubDomains; preload` in both `api/src/main.rs:194–196` and `frontend/src/hooks.server.ts:352`. Fly.io and Cloudflare Pages both enforce HTTPS at the edge. Secure cookie flag is conditional on `isSecure` (set to true in production).

---

**25. CSP header**  
**Status:** FAIL  
**Evidence:** API CSP: `default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'...` — `'unsafe-inline'` degrades protection (see M-2). Frontend (SvelteKit): no CSP header set (see L-5). `frame-ancestors` absent from both.

---

**26. CSRF protection**  
**Status:** PASS  
**Evidence:** Cookies use `sameSite: 'lax'`, which prevents cross-site form-submission CSRF. The SvelteKit server routes also act as a proxy, and cross-origin requests cannot set cookies. No additional CSRF token mechanism is needed given `sameSite: lax`.

---

**27. Input validation**  
**Status:** PARTIAL  
**Evidence:** `ValidatedJson<T>` extractor with `validator::Validate` annotations on `CreateUser`, `LoginUser`, `ResetPasswordRequest` (`models/user.rs`). `ForgotPasswordRequest` and `ResendVerificationRequest` do not use `ValidatedJson`. Admin `CreateUserRequest` and `UpdateUserRequest` do not use `ValidatedJson` and have no length limits.

---

**28. SQL injection prevention**  
**Status:** PASS (with noted exceptions)  
**Evidence:** All auth-critical SQL uses `sqlx::query_as(...)` with `.bind()` parameters. Dynamic SQL building (`format!` for `WHERE` clauses) found in `admin.rs`, `security.rs`, `videos.rs`, `media.rs`, `cms_assets.rs` — all cases examined build column names from Rust code (not user input) and bind values via `.bind()`. No `format!("... WHERE email = '{}'", email)` pattern found in auth paths.

---

**29. XSS prevention**  
**Status:** PARTIAL  
**Evidence:** `password_hash` is `#[serde(skip_serializing)]` on the `User` struct (PASS). The API is JSON-only, so reflected HTML XSS in API responses is not applicable. The frontend's use of DOMPurify was not verified in this pass (frontend rendering is out of scope for the Rust API audit). CSP `'unsafe-inline'` reduces XSS defense in depth.

---

**30. Secrets in env vars**  
**Status:** PARTIAL  
**Evidence:** `JWT_SECRET`, `DATABASE_URL`, `STRIPE_SECRET`, `R2_SECRET_ACCESS_KEY`, `POSTMARK_TOKEN` are all read from env vars (PASS). `.env` is gitignored and not committed (PASS). `.env.example` contains only placeholder values (`replace-me-...` or blank) (PASS). **However**: `api/.env` (local, not tracked) contains real R2 credentials, not placeholders (see C-2). `jwt_expires_in` defaults to `24` if `JWT_SECRET` env var is omitted, config panics correctly — but `JWT_EXPIRES_IN` defaults to 24 hours silently (see H-1).

---

## 4. Adversarial Scenarios

### A. Account Takeover via Password Reset
**Exploitable? NO**  
Reset email is sent only to the address on file (`user.email` from DB, not from the request body). The request does use `input.email` in the DB query, but the token is stored against the DB-verified email. An attacker cannot redirect the reset link to a different address. Generic response prevents enumeration.  
**Caveat:** Sessions are not invalidated on reset (see M-8) — takeover window if attacker has an existing session.

### B. Privilege Escalation via Update — Self
**Exploitable? NO (for role)**  
`PUT /api/user/profile` only accepts `first_name`, `last_name`, `display_name`, `email`, `current_password`, `new_password`. No `role` field. A regular member cannot self-promote via this endpoint.

### C. Privilege Escalation — Regular Member → Admin via Admin Endpoint
**Exploitable? NO** (requires admin credentials first)  
`PUT /admin/users/:id` requires the `admin` role. A regular member cannot call this endpoint. However, once an admin is compromised, they can promote any user to `developer` without being a `super_admin` (see H-7).

### D. Token Theft Window
**Yes, 24 hours**  
If an attacker steals an access token (via XSS, network interception, or URL exposure after OAuth), the exploitation window is 24 hours — the default JWT TTL (see H-1). The blacklist is only populated on logout; there is no forced revocation path for stolen tokens. The legitimate user cannot shorten this window unless they call `/logout-all`.

### E. Brute Force
**Partially exploitable**  
Per-email: 10 attempts per 15 minutes before lockout. Per-IP: 10 attempts per 15 minutes, but **fails open** on Redis unavailability (see H-2). Without Redis, there is no rate limiting whatsoever. With Redis available, rotating emails bypasses per-email limits; per-IP is the backstop (but see M-4 for IP spoofing via X-Forwarded-For).

### F. Enumeration
**Partially exploitable**  
- Login: NOT exploitable — "Invalid credentials" for both wrong password and unknown user.  
- Registration: email existence revealed by "Email already registered" — common/acceptable.  
- Password reset: NOT exploitable — generic response.  
- `resend_verification`: EXPLOITABLE — distinct response for verified users reveals both existence and verification state (see H-5).  
- Profile lookup (`GET /api/users/:id`): requires `AdminUser` — not accessible to regular users.

### G. Mass Assignment
**Exploitable? NO**  
- `UpdateProfileRequest` (`user.rs:712`): fields limited to name, email, password. No role field.  
- `CreateUserRequest` and `UpdateUserRequest` (`admin.rs`): role field is present but these require admin auth. An admin setting `role = "developer"` is a privilege escalation issue (H-7), not mass assignment per se.

### H. Session Fixation
**Exploitable? NO**  
On login, a new session ID is generated via `generate_session_id()` (`auth.rs:872`) and new JWTs are created. Logout invalidates the old session. There is no mechanism to pre-set a session ID. Cookies use `sameSite: lax` preventing cross-site fixation.

### I. CORS Bypass
**Exploitable? NO**  
CORS origins are an explicit allowlist (`cors_origins` from `CORS_ORIGINS` env var, or the hardcoded localhost/production list). `allow_credentials(true)` is paired with a specific origin list, not `*`. No `Access-Control-Allow-Origin: *` with credentials pattern.

### J. Webhook Spoofing (Stripe)
**Exploitable? NO**  
`api/src/routes/payments.rs:337–411`: webhook handler requires `stripe-signature` header, calls `stripe_for_webhook.verify_webhook(body.as_bytes(), signature)`. Returns 401 on invalid signature. Only bypassed in `is_dev` mode (environment starts with "dev"). Production correctly rejects unverified webhooks. The `state.config.environment` (not `std::env::var` per request) is used to determine dev mode — correct.

---

## 5. Surprises

### S-1: Rate Limiter Architecture Mismatch
`api/src/services/rate_limit.rs` implements a sophisticated three-tier rate limiter (Redis → in-memory → database). However, `auth.rs` does **not use this service** — it calls `state.services.redis` directly. The full RateLimitService is apparently unused for the auth path, making the database and memory fallback layers dead code for login rate limiting.

### S-2: `security.rs` Dynamic Query Builder With Parameter Counting
`api/src/routes/security.rs:68–120`: builds a `WHERE` clause by string-appending and manually tracking `$1/$2/$3` parameter numbers. The values are bound via `.bind()` so SQL injection is not possible, but the manually-tracked parameter numbering is error-prone and could produce a parameter count mismatch bug if the filter logic is refactored.

### S-3: Logout Requires an Auth Header AND a JSON Body
`api/src/routes/auth.rs:1066–1073`: `logout` requires `TypedHeader<Authorization<Bearer>>` AND `Json(input)`. If a frontend sends a logout request without `Content-Type: application/json` or without a body, Axum returns 400/422 rather than 200. This means a logout attempt that fails due to missing body leaves the token active.

### S-4: `is_developer_mode()` Returns True in Development
`api/src/config/mod.rs:304–306`:
```rust
pub fn is_developer_mode(&self) -> bool {
    self.developer_mode || !self.is_production()
}
```
Any non-production environment has developer mode enabled. This flag is not used in the auth path directly, but if code is ever written that gates on `is_developer_mode()`, it would bypass checks in staging/test environments.

### S-5: `deactivate_account` with `delete_data = true` Sets `password_hash = 'DEACTIVATED'`
`api/src/routes/user.rs:619`: after anonymization, `password_hash` is set to the literal string `'DEACTIVATED'`. The `verify_password` function in `utils/mod.rs:205–213` returns `Err("Unknown password hash format")` for this value. This means if the user somehow tries to log in with the anonymized email (e.g., `deleted_123@deactivated.local`), they get a 500 Internal Server Error rather than a clean 401. Not a security issue, but a poor error surface.

---

## 6. What's NOT in Scope (Layer 2)

The following items are important for production readiness but were NOT audited in this pass:

- **MFA implementation** (`api/src/services/mfa.rs`): the service exists, was not audited
- **HIBP breach checking** on registration
- **Compliance** (SOC 2, PCI DSS for Stripe integration)  
- **Content sanitization** in blog post rendering (DOMPurify usage in SvelteKit components)
- **File upload security** beyond avatar content-type checks (media upload paths not audited)
- **Redis security** (no auth on `REDIS_URL=redis://localhost:6379` default — no TLS, no password)
- **Database row-level security** (RLS) — not implemented
- **Search/analytics** endpoint authorization completeness (many routes not audited)
- **Background job queue** security (`api/src/queue/`) — not audited
- **WebSocket auth** (`api/src/routes/websocket.rs`) — not audited
- **Rate limiting on public blog/search endpoints** — not audited
- **Monitoring endpoint** authentication (`/monitoring/metrics` — likely open)

---

## 7. Summary Table

| ID | Finding | Severity | Status |
|----|---------|----------|--------|
| C-1 | Banned users can authenticate | Critical | Open |
| C-2 | Real R2 credentials in local .env | Critical | Requires rotation |
| H-1 | JWT TTL 24h default | High | Open |
| H-2 | Login rate limiter fails open | High | Open |
| H-3 | OAuth tokens in redirect URL | High | Open |
| H-4 | Admin create_user skips password validation | High | Open |
| H-5 | resend_verification leaks verified status | High | Open |
| H-6 | ban_user doesn't invalidate sessions | High | Open |
| H-7 | `developer` role escalation by any admin | High | Open |
| M-1 | Frontend JWT decoded without signature verification | Medium | Open |
| M-2 | CSP unsafe-inline on script-src | Medium | Open |
| M-3 | frame-ancestors absent from CSP | Medium | Open |
| M-4 | X-Forwarded-For IP trust enables rate limit bypass | Medium | Open |
| M-5 | Refresh blacklist write failure leaves old token valid | Medium | Open |
| M-6 | Impersonation endpoint serves placeholder (unfinished) | Medium | Open |
| M-7 | No audit log for role changes or admin actions | Medium | Open |
| M-8 | Password reset doesn't invalidate sessions | Medium | Open |
| L-1 | Password complexity requirements vs NIST 800-63B | Low | Open |
| L-2 | `thread_rng()` instead of `OsRng` in token generation | Low | Open |
| L-3 | validate_coupon endpoint is public | Low | Open |
| L-4 | logout requires JSON body (usability/security) | Low | Open |
| L-5 | Frontend missing CSP header | Low | Open |
| L-6 | Dynamic SQL column-name building (fragile pattern) | Low | Informational |

---

*This report covers one read-only audit pass. No code was modified. All findings reference source lines verified at audit time. Fixes are a separate session.*
