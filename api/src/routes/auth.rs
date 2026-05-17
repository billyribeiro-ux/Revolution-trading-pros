//! Authentication routes - December 2025 ICT11+ Principal Engineer Grade
//!
//! Implements complete authentication contract for frontend:
//! - POST /register - Create new user account (requires email verification)
//! - POST /login - Authenticate user (requires verified email)
//! - POST /refresh - Refresh access token
//! - POST /logout - Invalidate session
//! - GET /me - Get current user
//! - POST /forgot-password - Request password reset
//! - POST /reset-password - Reset password with token
//! - GET /verify-email - Verify email with token
//! - POST /resend-verification - Resend verification email

use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, Query, State},
    // FIX-2026-04-26 (Priority 4): pull in HeaderMap to extract client IP for rate-limiting.
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json,
    Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    // P1-3 (FULL_REPO_AUDIT_2026-05-17): trusted-proxy CIDR type for the
    // spoof-resistant client-IP resolver.
    config::IpCidr,
    // FIX-2026-04-26 (Priority 2): pull in ValidatedJson extractor for hot-path handlers.
    middleware::validated_json::ValidatedJson,
    models::{
        AuthResponse, CreateUser, ForgotPasswordRequest, LoginUser, MessageResponse,
        RefreshTokenRequest, RefreshTokenResponse, ResetPasswordRequest, User, UserResponse,
    },
    utils::{
        create_jwt_versioned, create_refresh_token_versioned, generate_session_id,
        generate_verification_token, hash_dummy_password, hash_password, hash_token,
        validate_password, verify_jwt, verify_password,
    },
    AppState,
};

/// P1-3 (FULL_REPO_AUDIT_2026-05-17): spoof-resistant client-IP resolver.
///
/// Replaces the old `client_ip(headers)` helper, which blindly trusted the
/// first hop of client-supplied `X-Forwarded-For` (then `X-Real-IP`). Because
/// every per-IP rate-limit bucket is keyed by the returned string, an attacker
/// could mint a brand-new bucket on every request just by rotating the XFF
/// header — defeating the register (5/15m), per-IP login (10/15m),
/// forgot-password (3/h) and reset-password (5/h) limiters.
///
/// Root cause: there was no notion of *which* proxy is allowed to assert a
/// forwarded client IP. The fix introduces an explicit trusted-proxy CIDR
/// allowlist (`Config::trusted_proxy_cidrs`, env `TRUSTED_PROXY_CIDRS`) and the
/// standard, well-understood algorithm:
///
/// 1. If the immediate TCP peer is **not** within a trusted-proxy CIDR, the
///    request did not arrive through our infrastructure: `X-Forwarded-For` /
///    `X-Real-IP` are attacker-controlled noise and are **ignored entirely**.
///    The real TCP peer is the client.
/// 2. If the peer **is** trusted, walk `X-Forwarded-For` **right-to-left**
///    (proxies *append* the address they saw, so the rightmost entries are the
///    ones closest to us and the most trustworthy) and return the rightmost
///    address that is **not** itself a trusted proxy. That is the first hop we
///    cannot vouch for — i.e. the real external client. This correctly handles
///    a chain of N trusted proxies.
/// 3. If every XFF entry is a trusted proxy (or XFF is absent/empty/malformed),
///    fall back to the trusted peer's own socket address.
///
/// With an empty allowlist (the default — `TRUSTED_PROXY_CIDRS` unset) NO peer
/// is ever trusted, so forwarded headers are always ignored and the socket
/// peer is authoritative. The legitimate single-proxy and direct-connection
/// cases are byte-for-byte unchanged; only header spoofing is closed.
pub fn resolve_client_ip(peer: IpAddr, headers: &HeaderMap, trusted: &[IpCidr]) -> IpAddr {
    // Step 1: is the immediate peer a proxy we explicitly trust?
    let peer_trusted = trusted.iter().any(|cidr| cidr.contains(peer));
    if !peer_trusted {
        // Untrusted (or no allowlist configured): forwarded headers are
        // adversary-controlled. Ignore them completely.
        return peer;
    }

    // Step 2: the peer is a trusted proxy. Reconstruct the forwarded chain
    // and find the rightmost non-trusted (i.e. real client) hop.
    //
    // RFC 7239 / de-facto XFF semantics: a proxy appends the address it
    // received the connection from. So the list reads, left→right,
    // [original client, proxy1, proxy2, ...]. Walking right→left and
    // skipping trusted proxies yields the genuine client even with several
    // chained load balancers.
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        for hop in xff.rsplit(',') {
            let hop = hop.trim();
            if hop.is_empty() {
                continue;
            }
            // XFF entries can carry a `:port` (and IPv6 may be bracketed).
            let Some(parsed) = parse_forwarded_hop(hop) else {
                // Malformed entry — cannot trust or attribute it. Skip and
                // keep walking; do NOT abort to the peer, since a later
                // (more leftward) entry might still be a valid client and a
                // garbage rightmost hop would otherwise mask it.
                continue;
            };
            if trusted.iter().any(|cidr| cidr.contains(parsed)) {
                // Another trusted proxy in the chain — keep walking left.
                continue;
            }
            // First non-trusted hop from the right: the real client.
            return parsed;
        }
    }

    // Step 3: XFF absent / empty / all-trusted / all-malformed. Fall back
    // to the trusted proxy's socket address. (We intentionally do NOT
    // consult `X-Real-IP` here: a trusted proxy that wants to assert the
    // client IP must do so via XFF, which is what every reverse proxy in
    // this stack emits. Honoring a second, independently-spoofable header
    // would re-open a parallel bypass.)
    peer
}

/// Parse one `X-Forwarded-For` element into an [`IpAddr`].
///
/// Tolerates the common real-world shapes:
///
/// - bare IPv4 / IPv6 (`203.0.113.7` / `2001:db8::1`)
/// - IPv4 with port (`203.0.113.7:54321`)
/// - bracketed IPv6, optionally with port (`[2001:db8::1]` / `[2001:db8::1]:443`)
///
/// Anything else returns `None` (the caller skips it).
fn parse_forwarded_hop(hop: &str) -> Option<IpAddr> {
    // Fast path: already a valid bare IP (covers unbracketed IPv6).
    if let Ok(ip) = hop.parse::<IpAddr>() {
        return Some(ip);
    }
    // Bracketed IPv6, optionally followed by `:port`.
    if let Some(rest) = hop.strip_prefix('[') {
        if let Some(end) = rest.find(']') {
            return rest[..end].parse::<IpAddr>().ok();
        }
        return None;
    }
    // `host:port` — only meaningful for IPv4 (an unbracketed IPv6 with a
    // port is ambiguous and non-conformant; reject it).
    if let Some((host, _port)) = hop.rsplit_once(':') {
        if let Ok(v4) = host.parse::<std::net::Ipv4Addr>() {
            return Some(IpAddr::V4(v4));
        }
    }
    None
}

/// Resolve the client IP for rate-limiting and return it as the `String`
/// bucket key the limiter expects. Thin adapter over [`resolve_client_ip`]
/// so the handler call sites stay terse.
///
/// `peer` is `Option` because the `ConnectInfo` extension is only present
/// when the server is launched via `into_make_service_with_connect_info`
/// (production — see `main.rs`). Tower's `oneshot`-driven integration test
/// harness does not install it; in that case we fall back to the
/// unspecified address `0.0.0.0`, which is **never** inside any
/// trusted-proxy CIDR — so the spoof-resistant branch still ignores
/// forwarded headers. Fail-safe: a missing peer can only make the limiter
/// *more* conservative (single shared bucket), never bypassable.
fn client_ip(peer: Option<SocketAddr>, headers: &HeaderMap, trusted: &[IpCidr]) -> String {
    let peer_ip = peer
        .map(|p| p.ip())
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));
    resolve_client_ip(peer_ip, headers, trusted).to_string()
}

/// FIX-2026-04-26 (Priority 4): per-IP rate limit gate.
/// FAIL-CLOSED on Redis outage for register/forgot/reset (sensitive endpoints).
/// Returns Err if rate limit exceeded OR if Redis is unreachable.
async fn enforce_ip_rate_limit_strict(
    state: &AppState,
    ip: &str,
    bucket_key: &str,
    max_requests: i64,
    window_seconds: u64,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let Some(redis) = state.services.redis.as_ref() else {
        // No Redis configured at all — fail closed for these sensitive endpoints.
        tracing::error!(
            target: "security",
            event = "rate_limit_redis_unavailable_fail_closed",
            bucket = %bucket_key,
            ip = %ip,
            "Redis service not configured; rejecting sensitive request"
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Service temporarily unavailable, please try again"})),
        ));
    };

    // Scope key by bucket + IP so register/forgot/reset are independently throttled.
    let key = format!("{}:{}", bucket_key, ip);
    match redis
        .check_ip_rate_limit(&key, max_requests, window_seconds)
        .await
    {
        Ok(result) => {
            if !result.allowed {
                tracing::warn!(
                    target: "security",
                    event = "ip_rate_limit_exceeded",
                    bucket = %bucket_key,
                    ip = %ip,
                    "IP rate limit exceeded for sensitive endpoint"
                );
                return Err((
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(json!({
                        "error": "Too many requests, please try again later",
                        "retry_after": result.retry_after,
                    })),
                ));
            }
            Ok(())
        }
        Err(e) => {
            // FAIL-CLOSED: Redis transient failure must not become a free pass for abuse.
            tracing::error!(
                target: "security",
                event = "rate_limit_check_failed_fail_closed",
                bucket = %bucket_key,
                ip = %ip,
                error = %e,
                "Rate limit Redis check failed; rejecting request (fail-closed)"
            );
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            ))
        }
    }
}

/// Query params for email verification
#[derive(Debug, Deserialize)]
struct VerifyEmailQuery {
    token: String,
}

/// Request to resend verification email
#[derive(Debug, Deserialize)]
struct ResendVerificationRequest {
    email: String,
}

/// Registration response (no tokens - must verify email first)
#[derive(Debug, serde::Serialize)]
struct RegisterResponse {
    message: String,
    email: String,
    requires_verification: bool,
}

/// Register a new user
/// POST /api/auth/register
// FIX-2026-04-26 (Priority 2): migrated Json<CreateUser> -> ValidatedJson<CreateUser>.
// FIX-2026-04-26 (Priority 4): added HeaderMap + per-IP rate limit (5 / 15min).
// Original signature: async fn register(State(state): State<AppState>, Json(input): Json<CreateUser>)
async fn register(
    State(state): State<AppState>,
    peer: Option<ConnectInfo<SocketAddr>>,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<CreateUser>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit BEFORE Argon2 hashing
    // (otherwise an attacker can DOS by burning CPU on hash attempts).
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware).
    let ip = client_ip(
        peer.map(|ci| ci.0),
        &headers,
        &state.config.trusted_proxy_cidrs,
    );
    enforce_ip_rate_limit_strict(&state, &ip, "register", 5, 900).await?;

    // Validate email format
    if !input.email.contains('@') || !input.email.contains('.') {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "email": ["Invalid email format"]
                }
            })),
        ));
    }

    // ICT L11+ Security: Validate password strength with hardened rules
    if let Err(password_error) = validate_password(&input.password) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password": [password_error]
                }
            })),
        ));
    }

    // Check if email exists
    let existing: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "email": ["Email already registered"]
                }
            })),
        ));
    }

    // Hash password
    let password_hash = hash_password(&input.password).map_err(|e| {
        tracing::error!("Password hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Password hashing failed"})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): wrap user-insert + verification-token-insert in a
    // single Postgres transaction. Previously a crash between the two writes (or a
    // unique-violation on retry) yielded an account that could not self-verify.
    // Reference pattern: routes/user.rs::update_profile.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Create user with email_verified_at = NULL (unverified)
    // Original pool reference: .fetch_one(&state.db.pool)
    let user: User = sqlx::query_as(
        r#"
        INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
        VALUES ($1, $2, $3, 'user', NULL, NOW(), NOW())
        RETURNING id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
        "#,
    )
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&input.name)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("User creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create user"})),
        )
    })?;

    // Generate verification token
    let (raw_token, hashed_token) = generate_verification_token();

    // FIX-L-2 (2026-04-29): defensively delete any pre-existing verification
    // tokens for this user before inserting the new one. Matches the
    // resend-verification handler's pattern (auth.rs:537). The
    // email-already-registered check at line 196 makes a real collision
    // unreachable today, but this protects against a future code path that
    // creates a user via a different route and then issues a verification
    // email. Idempotent — DELETE of zero rows is a no-op.
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id = $1")
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .ok();

    // Store verification token (expires in 24 hours)
    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (user_id, token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '24 hours')
        "#,
    )
    .bind(user.id)
    .bind(&hashed_token)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create verification token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create verification token"})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): commit BEFORE any external HTTP calls (email send).
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit registration transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Batch 6: send welcome + verification via send_transactional.
    // Best-effort — don't fail registration if email is down; the user
    // can request a resend. The email_logs row provides retry visibility.
    let verification_link = format!(
        "{}/verify-email?token={}",
        state.config.app_url.trim_end_matches('/'),
        raw_token
    );
    let _ = state
        .services
        .email
        .send_transactional(
            &state.db.pool,
            &user.email,
            "welcome",
            json!({
                "name": user.name,
                "app_url": state.config.app_url.clone(),
            }),
        )
        .await;
    let _ = state
        .services
        .email
        .send_transactional(
            &state.db.pool,
            &user.email,
            "email-verification",
            json!({
                "name": user.name,
                "verification_link": verification_link,
                "expires_in_hours": 24,
            }),
        )
        .await;

    tracing::info!(
        target: "security_audit",
        event = "user_registered",
        user_id = %user.id,
        verification_required = true,
        "ICT 11+ AUDIT: User registered - pending email verification"
    );

    // Return success without tokens - user must verify email first
    Ok(Json(RegisterResponse {
        message: "Registration successful! Please check your email to verify your account."
            .to_string(),
        email: user.email,
        requires_verification: true,
    }))
}

/// Verify email with token
/// GET /api/auth/verify-email?token=xxx
async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    let hashed_token = hash_token(&query.token);

    // Find the verification token
    let token_record: Option<(i64, i64)> = sqlx::query_as(
        r#"
        SELECT id, user_id FROM email_verification_tokens 
        WHERE token = $1 AND expires_at > NOW()
        "#,
    )
    .bind(&hashed_token)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let (token_id, user_id) = token_record.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid or expired verification link"})),
        )
    })?;

    // Get user for welcome email
    let user: User = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE id = $1"
    )
        .bind(user_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // ICT 11+ SECURITY: Update user's email_verified_at with audit trail
    sqlx::query("UPDATE users SET email_verified_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(user_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "security_audit",
                event = "email_verification_failed",
                user_id = %user_id,
                error = %e,
                "ICT 11+ ALERT: Failed to update email verification status"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to verify email"})),
            )
        })?;

    // ICT 11+ SECURITY: Delete the used verification token (one-time use)
    sqlx::query("DELETE FROM email_verification_tokens WHERE id = $1")
        .bind(token_id)
        .execute(&state.db.pool)
        .await
        .ok(); // Ignore errors on cleanup

    // ICT 11+ AUDIT: Log successful email verification
    tracing::info!(
        target: "security_audit",
        event = "email_verified_success",
        user_id = %user.id,
        verified_at = %chrono::Utc::now().to_rfc3339(),
        "ICT 11+ AUDIT: Email successfully verified - user can now login"
    );

    // Batch 6: welcome email is sent at register, not at verify. We
    // KEEP the audit-event structure here for compatibility with any
    // logging dashboards that watch for `welcome_email_sent`, but no
    // outgoing call. The match arm signature stays Result<_, _> so the
    // surrounding code (which logs based on Ok/Err) remains intact.
    {
        let _email_service = &state.services.email;
        let result: anyhow::Result<()> = Ok(());
        match result {
            Ok(_) => {
                tracing::debug!(
                    target: "security_audit",
                    event = "welcome_email_skipped_already_sent_at_register",
                    user_id = %user.id,
                    "Welcome email was sent at register; not re-sending on verify"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "welcome_email_failed",
                    user_id = %user.id,
                    error = %e,
                    "ICT 11+ ALERT: Failed to send welcome email - non-critical"
                );
                // Don't fail verification if welcome email fails
            }
        }
    }

    Ok(Json(MessageResponse {
        message: "Email verified successfully! You can now log in.".to_string(),
        success: Some(true),
    }))
}

/// Resend verification email
/// POST /api/auth/resend-verification
async fn resend_verification(
    State(state): State<AppState>,
    Json(input): Json<ResendVerificationRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Find user by email
    let user: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // ICT 11+ SECURITY: Always return success to prevent user enumeration
    let Some(user) = user else {
        tracing::info!(
            target: "security_audit",
            event = "resend_verification_unknown_email",
            "ICT 11+ AUDIT: Verification resend requested for unknown email"
        );
        return Ok(Json(MessageResponse {
            message: "If your email is registered, you will receive a verification link shortly."
                .to_string(),
            success: Some(true),
        }));
    };

    // FIX-2026-04-27 (M-2): Return identical generic message for already-verified users
    // to prevent email enumeration. Do NOT send another email — it is wasted work.
    if user.email_verified_at.is_some() {
        tracing::info!(
            target: "security_audit",
            event = "resend_verification_already_verified",
            user_id = %user.id,
            "Verification resend suppressed - email already verified"
        );
        return Ok(Json(MessageResponse {
            message: "If your email is registered, you will receive a verification link shortly."
                .to_string(),
            success: Some(true),
        }));
    }

    // FIX-2026-04-26 (Priority 5): wrap delete-old + insert-new verification token in tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // ICT 11+ SECURITY: Delete any existing verification tokens (prevent token accumulation)
    // Original pool reference: .execute(&state.db.pool)
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id = $1")
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .ok();

    // ICT 11+ SECURITY: Generate new verification token
    let (raw_token, hashed_token) = generate_verification_token();

    // ICT 11+ SECURITY: Store verification token with 24-hour expiry
    // Original pool reference: .execute(&state.db.pool)
    sqlx::query(
        r#"
        INSERT INTO email_verification_tokens (user_id, token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '24 hours')
        "#,
    )
    .bind(user.id)
    .bind(&hashed_token)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create verification token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create verification token"})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): commit BEFORE external email HTTP call.
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit resend-verification transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Batch 6: resend verification via send_transactional (same template alias
    // as the original send at register time).
    let verification_link = format!(
        "{}/verify-email?token={}",
        state.config.app_url.trim_end_matches('/'),
        raw_token
    );
    {
        let email_service = &state.services.email;
        match email_service
            .send_transactional(
                &state.db.pool,
                &user.email,
                "email-verification",
                json!({
                    "name": user.name,
                    "verification_link": verification_link,
                    "expires_in_hours": 24,
                }),
            )
            .await
        {
            Ok(_) => {
                tracing::info!(
                    target: "security_audit",
                    event = "verification_email_resent",
                    user_id = %user.id,
                    token_expires = "24_hours",
                    "ICT 11+ AUDIT: Verification email resent successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "verification_email_resend_failed",
                    user_id = %user.id,
                    error = %e,
                    "ICT 11+ ALERT: Failed to resend verification email"
                );
            }
        }
    }

    Ok(Json(MessageResponse {
        message: "If your email is registered, you will receive a verification link shortly."
            .to_string(),
        success: Some(true),
    }))
}

/// Login user
/// POST /api/auth/login
/// ICT L11+ Security: Rate limiting, timing attack prevention, session management
// FIX-2026-04-26 (Priority 2): migrated Json<LoginUser> -> ValidatedJson<LoginUser>.
// FIX-2026-04-26 (Priority 4): added per-IP rate limit (10 / 15min) ALONGSIDE existing
// per-email rate limit. Per-email alone is bypassable by an attacker rotating emails;
// per-IP catches that pattern. Login keeps fail-OPEN behavior (per audit) — only the
// per-IP additional check uses fail-OPEN here for availability of the auth surface.
// Original signature: async fn login(State(state): State<AppState>, Json(input): Json<LoginUser>)
async fn login(
    State(state): State<AppState>,
    peer: Option<ConnectInfo<SocketAddr>>,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<LoginUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Security audit logging
    tracing::info!(
        target: "security",
        event = "login_attempt",
        "Login attempt initiated"
    );

    // FIX-2026-04-27 (H-2): Both per-IP and per-email checks now FAIL CLOSED.
    // A Redis outage must not become a free pass for brute-force attacks.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware), so
    // the per-IP bucket can no longer be sidestepped by rotating XFF.
    let ip = client_ip(
        peer.map(|ci| ci.0),
        &headers,
        &state.config.trusted_proxy_cidrs,
    );
    enforce_ip_rate_limit_strict(&state, &ip, "login_ip", 10, 900).await?;

    let Some(ref redis) = state.services.redis else {
        tracing::error!(
            target: "security",
            event = "login_rate_limit_redis_unavailable",
            "Redis unavailable for login rate limit check - rejecting request (fail-closed)"
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Service temporarily unavailable, please try again"})),
        ));
    };

    match redis.check_login_rate_limit(&input.email).await {
        Ok(rate_limit) if !rate_limit.allowed => {
            let error_msg = if rate_limit.locked {
                "Account temporarily locked due to too many failed attempts"
            } else {
                "Too many login attempts. Please wait before trying again"
            };
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": error_msg,
                    "retry_after": rate_limit.retry_after,
                    "locked": rate_limit.locked
                })),
            ));
        }
        Ok(_) => {}
        Err(e) => {
            tracing::error!(
                target: "security",
                event = "login_email_rate_limit_failed_closed",
                error = %e,
                "Per-email rate limit Redis check failed - rejecting request (fail-closed)"
            );
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            ));
        }
    }

    // Find user
    let user_result: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, is_active, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error during login: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // ICT L11+ Security: Timing attack prevention
    // Always perform password hashing even if user doesn't exist
    // This prevents timing-based user enumeration
    let user = match user_result {
        Some(u) => u,
        None => {
            // Hash dummy password to match timing of real verification
            hash_dummy_password();
            tracing::info!(
                target: "security",
                event = "login_failed",
                reason = "user_not_found",
                "Login failed - user not found (timing protected)"
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            ));
        }
    };

    // ICT 7 SECURITY: Password verification - NO password data logged
    // Apple Principal Engineer Grade: Never log credentials, hashes, or password metadata

    // Verify password (this happens regardless of user existence due to above)
    let password_valid = verify_password(&input.password, &user.password_hash).map_err(|e| {
        tracing::error!("Password verification error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Authentication error"})),
        )
    })?;

    if !password_valid {
        // Record failed attempt for rate limiting
        if let Some(redis) = &state.services.redis {
            let _ = redis.record_failed_login(&input.email).await;
        }

        tracing::info!(
            target: "security",
            event = "login_failed",
            reason = "invalid_password",
            user_id = %user.id,
            "Login failed - invalid password"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // Clear failed login attempts on successful authentication
    if let Some(redis) = &state.services.redis {
        let _ = redis.clear_login_attempts(&input.email).await;
    }

    // FIX-M-4 (2026-04-29): silently re-hash bcrypt (Laravel legacy) into
    // argon2id on every successful login. Bcrypt at default cost is still
    // acceptable, but argon2id is the project standard (utils/mod.rs:151)
    // and we should not maintain two hash schemes indefinitely.
    //
    // verify_password() (utils/mod.rs:180) recognizes hashes by prefix:
    //   $2y$ / $2b$ / $2a$  -> bcrypt
    //   $argon2             -> argon2id
    // If the stored hash is bcrypt, we already verified it — re-hash the
    // plaintext we have in `input.password` and overwrite the column.
    // Best-effort: a failure here logs but does not fail the login; the
    // user keeps a bcrypt hash and we'll try again on their next login.
    if user.password_hash.starts_with("$2") {
        match hash_password(&input.password) {
            Ok(new_hash) => {
                if let Err(e) = sqlx::query(
                    "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                )
                .bind(&new_hash)
                .bind(user.id)
                .execute(&state.db.pool)
                .await
                {
                    tracing::error!(
                        target: "security_audit",
                        event = "bcrypt_rehash_write_failed",
                        user_id = %user.id,
                        error = %e,
                        "Could not migrate bcrypt hash to argon2id"
                    );
                } else {
                    tracing::info!(
                        target: "security_audit",
                        event = "bcrypt_rehashed_to_argon2id",
                        user_id = %user.id,
                        "Migrated user from bcrypt to argon2id on successful login"
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "bcrypt_rehash_compute_failed",
                    user_id = %user.id,
                    error = %e,
                    "argon2 hash computation failed during bcrypt migration"
                );
            }
        }
    }

    // FIX-2026-04-27 (C-1): Block banned/deactivated accounts at login.
    // is_active == Some(false) means an admin explicitly banned this account.
    // NULL (legacy rows) and Some(true) are both treated as active.
    //
    // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2) — COMPANION NOW IMPLEMENTED.
    // The ban *state transition* lives in `routes/admin.rs::ban_user`; the
    // required `redis.bump_token_version(id)` call has been added there,
    // immediately next to the existing `redis.invalidate_all_user_sessions(id)`
    // call, using the same best-effort/log-on-Err pattern as `logout_all`
    // and `reset_password` below. That bump strands every prior
    // access/refresh JWT for a just-banned user the instant the extractor
    // re-reads the epoch — the C-1 `is_active` guard below + the extractor
    // only catch the *next DB-backed* fetch, not a token served from the
    // Redis user-cache hot path. This guard itself is a login gate (no live
    // token exists at this point), so no bump belongs here.
    if user.is_active == Some(false) {
        tracing::warn!(
            target: "security_audit",
            event = "login_blocked_banned",
            user_id = %user.id,
            "Login blocked - account is banned/deactivated"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // ICT 11+ ENTERPRISE SECURITY: Email Verification Enforcement
    // Apple Principal Engineer Grade - Zero Trust Security Model
    //
    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): close the
    // email-verification bypass AND the email-list-as-identity elevation
    // hole. ROOT CAUSE: the previous code OR'd `is_developer_email` /
    // `is_superadmin_email` (a configured env email-list string) straight
    // into `bypass_verification`. That meant ANYONE who controlled (or could
    // get their address onto) the configured `DEVELOPER_EMAILS` /
    // `SUPERADMIN_EMAILS` list could log in WITHOUT ever verifying that email
    // and be treated as a privileged role — an env list string was being
    // accepted as proof of identity.
    //
    // FIX: email-list membership is at most a *secondary hint* layered on
    // top of a verified, DB-role'd account. The privileged bypass now
    // requires BOTH (a) the user row has `email_verified_at` set, AND (b)
    // the user carries a real DB privileged role (developer / super_admin /
    // super-admin / admin) — the env list can only *confirm* an elevation
    // the database already grants, never *create* one or skip verification.
    // `is_*_email_strict` enforces (verified ∧ DB-role ∧ on-list); the
    // explicit DB-role checks below cover a verified DB-role'd account that
    // is NOT on any env list (legitimate verified-admin login preserved).
    let email_verified = user.email_verified_at.is_some();
    let db_role = user.role.as_deref();
    let has_db_developer_role = db_role == Some("developer");
    let has_db_superadmin_role = db_role == Some("super_admin") || db_role == Some("super-admin");

    let is_developer = email_verified
        && (has_db_developer_role
            || state
                .config
                .is_developer_email_strict(&user.email, db_role, email_verified));

    let is_superadmin = email_verified
        && (has_db_superadmin_role
            || state
                .config
                .is_superadmin_email_strict(&user.email, db_role, email_verified));

    // ICT 11+ SECURITY: Strict verification enforcement.
    // A privileged bypass now requires a VERIFIED account with a real DB
    // privileged role — controlling a configured email is never sufficient
    // on its own. All other users MUST verify email.
    let bypass_verification = is_developer || is_superadmin;

    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): the old code logged an
    // "email_verification_bypassed" warning here for `bypass_verification &&
    // email_verified_at.is_none()`. Under the hardened derivation above,
    // `bypass_verification` IMPLIES `email_verified` (a privileged bypass now
    // requires a verified, DB-role'd account), so an unverified privileged
    // bypass is no longer reachable — that warning would be dead code that
    // misrepresents the (now-closed) invariant. We instead audit the real,
    // legitimate event: a verified privileged account authenticating.
    if bypass_verification {
        let role_type = if is_developer {
            "developer"
        } else {
            "superadmin"
        };
        tracing::info!(
            target: "security_audit",
            event = "privileged_login",
            user_id = %user.id,
            role = %role_type,
            email_verified = true,
            "ICT 11+ AUDIT: Verified privileged account login (email-list is a secondary hint only, never a standalone gate)"
        );
    }

    // ICT 11+ SECURITY GATE: Block unverified users
    if user.email_verified_at.is_none() && !bypass_verification {
        tracing::warn!(
            target: "security",
            event = "login_blocked_unverified",
            user_id = %user.id,
            attempt_timestamp = %chrono::Utc::now().to_rfc3339(),
            "ICT 11+ SECURITY: Login blocked - email not verified"
        );
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Please verify your email before logging in. Check your inbox for the verification link.",
                "code": "EMAIL_NOT_VERIFIED",
                "email": user.email,
                "help": "Didn't receive the email? Contact support or check your spam folder.",
                "security_level": "ICT_11_ENFORCED"
            })),
        ));
    }

    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): the second
    // "privileged_verification_bypass" log (previously here, also gated on
    // `bypass_verification && email_verified_at.is_none()`) is likewise
    // unreachable under the hardened invariant and has been removed. The
    // single `privileged_login` audit above is the accurate, reachable event.

    // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): mint both tokens with the
    // user's CURRENT token epoch so they survive the extractor's stale-token
    // check until the next logout-all/password-reset/ban bump. If Redis is
    // configured but errors here we fail closed (do not mint a token we
    // cannot later validate); if Redis is absent the epoch is 0 (matches the
    // extractor's "Redis absent -> skip the epoch check" branch).
    let token_version = current_token_version(&state, user.id).await?;

    // Create access token
    let token = create_jwt_versioned(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        token_version,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Token creation failed"})),
        )
    })?;

    // Create refresh token
    let refresh_token =
        create_refresh_token_versioned(user.id, &state.config.jwt_secret, token_version).map_err(
            |e| {
                tracing::error!("Refresh token creation error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Refresh token creation failed"})),
                )
            },
        )?;

    // Generate session ID and store in Redis
    let session_id = generate_session_id();

    // ICT L11+ Security: Create server-side session in Redis
    if let Some(redis) = &state.services.redis {
        if let Err(e) = redis
            .create_session(&session_id, user.id, &user.email, None, None)
            .await
        {
            tracing::warn!("Failed to create session in Redis: {}", e);
        }
    }

    // Security audit: successful login
    tracing::info!(
        target: "security",
        event = "login_success",
        user_id = %user.id,
        session_id = %session_id,
        "Login successful"
    );

    Ok(Json(AuthResponse {
        token: token.clone(),
        access_token: token, // Frontend prefers access_token
        refresh_token,
        session_id,
        user: user.into(),
        expires_in: state.config.jwt_expires_in * 3600, // Convert hours to seconds
    }))
}

/// Refresh access token
/// POST /api/auth/refresh
///
/// FIX-2026-04-26 (Priority 6): Refresh-token reuse detection.
///   1. Hash the presented refresh token; check Redis blacklist FIRST.
///      If already blacklisted -> a stolen-and-replayed token. Per RFC 6749 §10.4 and
///      OAuth 2.0 BCP, invalidate ALL of the user's sessions/refresh tokens (chain
///      invalidation) and return 401.
///   2. After successful verification + rotation, blacklist the OLD refresh token in
///      Redis with TTL = remaining validity (max 7 days).
async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Verify the refresh token
    // FIX-2026-04-26 (Priority 3): pass "refresh" expected_type so an access token
    // cannot be used as a refresh token.
    // Original: let claims = verify_jwt(&input.refresh_token, &state.config.jwt_secret).map_err(|e| {
    let claims =
        verify_jwt(&input.refresh_token, &state.config.jwt_secret, "refresh").map_err(|e| {
            tracing::warn!("Invalid refresh token: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid or expired refresh token"})),
            )
        })?;

    // FIX-2026-04-26 (Priority 6): hash the presented refresh token for blacklist lookup.
    let token_hash = hash_token_for_blacklist(&input.refresh_token);

    if let Some(redis) = state.services.redis.as_ref() {
        match redis.is_token_blacklisted(&token_hash).await {
            Ok(true) => {
                // REUSE DETECTED — chain invalidation per RFC 6749 §10.4.
                tracing::warn!(
                    target: "security_audit",
                    event = "refresh_token_reuse_detected",
                    user_id = %claims.sub,
                    "Refresh-token reuse detected — invalidating all sessions for user"
                );
                // Best-effort: invalidate every active session for the subject. This forces
                // re-auth across all devices and breaks the attacker's chain.
                let _ = redis.invalidate_all_user_sessions(claims.sub).await;
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "Refresh token reuse detected; all sessions invalidated. Please log in again.",
                        "code": "REFRESH_TOKEN_REUSE",
                    })),
                ));
            }
            Ok(false) => {
                // Not previously seen — proceed with rotation.
            }
            Err(e) => {
                // Redis transient failure — log but continue (fail-open) to preserve
                // refresh availability. The audit explicitly accepts this trade-off
                // for non-write-side rate-limit checks.
                tracing::warn!(
                    target: "security",
                    event = "refresh_blacklist_check_failed",
                    user_id = %claims.sub,
                    error = %e,
                    "Refresh-token blacklist check failed; continuing"
                );
            }
        }
    }

    // Verify user still exists
    let user_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if user_exists.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "User not found"})),
        ));
    }

    // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): the presented refresh
    // token's own epoch must not be stale. `verify_jwt` is pure (sig+exp+
    // type), so enforce the epoch here explicitly — this is the refresh-side
    // mirror of the extractor's access-token check. A refresh token minted
    // before a logout-all/password-reset/ban is rejected instead of being
    // honored "until rotation" (the exact gap the audit flagged). Fail-closed
    // on Redis error; epoch 0 when Redis absent.
    let token_version = current_token_version(&state, claims.sub).await?;
    if claims.token_version < token_version {
        tracing::warn!(
            target: "security_audit",
            event = "stale_refresh_token_epoch_rejected",
            user_id = %claims.sub,
            token_version = %claims.token_version,
            current_version = %token_version,
            "Refresh rejected: token minted before a logout-all/password-reset/ban"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Refresh token has been revoked. Please log in again."})),
        ));
    }

    // Create new access token
    let token = create_jwt_versioned(
        claims.sub,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        token_version,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Token creation failed"})),
        )
    })?;

    // Create new refresh token (token rotation for security)
    let new_refresh_token =
        create_refresh_token_versioned(claims.sub, &state.config.jwt_secret, token_version)
            .map_err(|e| {
                tracing::error!("Refresh token creation error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Refresh token creation failed"})),
                )
            })?;

    // FIX-2026-04-26 (Priority 6): blacklist OLD refresh token in Redis so a re-presentation
    // is detected. TTL = remaining validity window (clamp to refresh-token lifetime = 7d).
    if let Some(redis) = state.services.redis.as_ref() {
        let now_ts = chrono::Utc::now().timestamp();
        let remaining = (claims.exp - now_ts).max(0) as u64;
        let ttl = remaining.min(7 * 24 * 3600);
        if let Err(e) = redis.blacklist_token(&token_hash, ttl).await {
            tracing::warn!(
                target: "security",
                event = "refresh_blacklist_write_failed",
                user_id = %claims.sub,
                error = %e,
                "Failed to blacklist old refresh token"
            );
        }
    }

    tracing::debug!("Token refreshed for user: {}", claims.sub);

    Ok(Json(RefreshTokenResponse {
        access_token: token.clone(),
        token,
        refresh_token: new_refresh_token,
        expires_in: state.config.jwt_expires_in * 3600,
    }))
}

/// Get current user (requires auth)
/// GET /api/auth/me
async fn me(user: User) -> Json<UserResponse> {
    Json(user.into())
}

/// Logout request with session ID and optional token
#[derive(Debug, Deserialize)]
struct LogoutRequest {
    session_id: Option<String>,
    #[serde(default)]
    token: Option<String>,
}

/// Hash a JWT token for blacklist storage
fn hash_token_for_blacklist(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): resolve the user's current
/// token epoch for embedding into a freshly minted access/refresh token.
///
/// Fail-closed contract (mirrors the `User` extractor and the existing
/// fail-closed Redis paths in this file): if Redis is configured but the
/// lookup errors we return 503 rather than minting a token at an unknown
/// epoch — a token minted at the wrong (stale) version would either be
/// instantly rejected by the extractor (annoying) or, worse, a guessed-too-
/// low value could mask a real revocation. If Redis is NOT configured the
/// epoch is 0, which is exactly what the extractor's "Redis absent -> skip
/// the epoch check" branch expects, so the two stay consistent.
async fn current_token_version(
    state: &AppState,
    user_id: i64,
) -> Result<i64, (StatusCode, Json<serde_json::Value>)> {
    match state.services.redis.as_ref() {
        Some(redis) => redis.get_token_version(user_id).await.map_err(|e| {
            tracing::error!(
                target: "security_audit",
                event = "token_version_read_failed_fail_closed",
                user_id = %user_id,
                error = %e,
                "Could not read token epoch; refusing to mint token (fail-closed)"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            )
        }),
        None => Ok(0),
    }
}

/// Logout user - ICT L11+ Security: Proper session and token invalidation
/// POST /api/auth/logout
async fn logout(
    State(state): State<AppState>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    user: User,
    Json(input): Json<LogoutRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Invalidate session in Redis if session_id provided
    if let Some(session_id) = input.session_id {
        if let Some(redis) = &state.services.redis {
            if let Err(e) = redis.invalidate_session(&session_id).await {
                tracing::warn!("Failed to invalidate session: {}", e);
            }
        }
    }

    // ICT 7 SECURITY: Blacklist the current JWT token to prevent reuse
    if let Some(redis) = &state.services.redis {
        let token = auth_header.token();
        let token_hash = hash_token_for_blacklist(token);

        // Blacklist for the remaining token lifetime (max 24 hours for safety)
        let blacklist_duration = (state.config.jwt_expires_in * 3600).min(86400) as u64;

        if let Err(e) = redis.blacklist_token(&token_hash, blacklist_duration).await {
            tracing::warn!(
                target: "security",
                event = "token_blacklist_failed",
                user_id = %user.id,
                error = %e,
                "Failed to blacklist token on logout"
            );
        } else {
            tracing::info!(
                target: "security_audit",
                event = "token_blacklisted",
                user_id = %user.id,
                "JWT token blacklisted on logout"
            );
        }
    }

    tracing::info!(
        target: "security",
        event = "logout",
        user_id = %user.id,
        "User logged out"
    );

    Ok(Json(MessageResponse {
        message: "Logged out successfully".to_string(),
        success: Some(true),
    }))
}

/// Logout from all devices - ICT L11+ Security: Force logout everywhere
/// POST /api/auth/logout-all
async fn logout_all(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let count = if let Some(redis) = &state.services.redis {
        // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): bump the token epoch
        // ALONGSIDE the session sweep. Deleting `session:*` keys alone never
        // touched the stateless access/refresh JWTs — that was the root bug:
        // "log out all devices" left a stolen access token usable for its
        // full TTL and a stolen refresh token usable until rotation. The
        // bump strands every previously-issued token for this user the
        // instant the extractor (and the refresh handler) re-checks the
        // epoch. Best-effort like the session sweep: a Redis fault here is
        // logged but does not 500 the logout — and the fail-closed read in
        // the extractor still rejects on a Redis fault at validation time.
        if let Err(e) = redis.bump_token_version(user.id).await {
            tracing::error!(
                target: "security_audit",
                event = "logout_all_token_version_bump_failed",
                user_id = %user.id,
                error = %e,
                "Could not bump token epoch on logout-all (sessions still swept; extractor fails closed on Redis error)"
            );
        }
        redis
            .invalidate_all_user_sessions(user.id)
            .await
            .unwrap_or(0)
    } else {
        0
    };

    tracing::info!(
        target: "security",
        event = "logout_all",
        user_id = %user.id,
        sessions_invalidated = %count,
        "User logged out from all devices"
    );

    Ok(Json(json!({
        "message": format!("Logged out from {} device(s)", count),
        "sessions_invalidated": count,
        "success": true
    })))
}

/// Request password reset
/// POST /api/auth/forgot-password
// FIX-2026-04-26 (Priority 4): added HeaderMap + per-IP rate limit (3 / hour).
// Original signature: async fn forgot_password(State(state): State<AppState>, Json(input): Json<ForgotPasswordRequest>)
async fn forgot_password(
    State(state): State<AppState>,
    peer: Option<ConnectInfo<SocketAddr>>,
    headers: HeaderMap,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (3/hour) — fail-closed.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware).
    let ip = client_ip(
        peer.map(|ci| ci.0),
        &headers,
        &state.config.trusted_proxy_cidrs,
    );
    enforce_ip_rate_limit_strict(&state, &ip, "forgot_password", 3, 3600).await?;

    // Find user (but don't reveal if they exist)
    let user: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // If user exists, create reset token and send email
    if let Some(user) = user {
        // FIX-2026-04-26 (Priority 5): wrap delete-old + insert-new reset token in tx.
        let mut tx = state.db.pool.begin().await.map_err(|e| {
            tracing::error!("Failed to begin transaction: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

        // Delete any existing reset tokens for this user
        // Original pool reference: .execute(&state.db.pool)
        sqlx::query("DELETE FROM password_resets WHERE email = $1")
            .bind(&user.email)
            .execute(&mut *tx)
            .await
            .ok();

        // Generate reset token
        let (raw_token, hashed_token) = generate_verification_token();

        // Store reset token (expires in 1 hour).
        // Batch 6 fix: the INSERT used to override `id` with
        // `gen_random_uuid()`, but `password_resets.id` is BIGSERIAL.
        // The mismatch made every forgot-password request 500 with
        // "column id is of type bigint but expression is of type uuid"
        // — pre-existing latent bug surfaced during Batch 6 verification
        // scenario E. Let the sequence default handle the id.
        sqlx::query(
            r#"
            INSERT INTO password_resets (email, token, expires_at, created_at)
            VALUES ($1, $2, NOW() + INTERVAL '1 hour', NOW())
            "#,
        )
        .bind(&user.email)
        .bind(&hashed_token)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create reset token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create reset token"})),
            )
        })?;

        // FIX-2026-04-26 (Priority 5): commit BEFORE external email HTTP call.
        tx.commit().await.map_err(|e| {
            tracing::error!("Failed to commit forgot-password transaction: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

        // Batch 6: password reset via send_transactional. Token TTL is
        // 1 hour per existing reset flow (see reset_password handler).
        let reset_link = format!(
            "{}/reset-password?token={}",
            state.config.app_url.trim_end_matches('/'),
            raw_token
        );
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                &user.email,
                "password-reset",
                json!({
                    "name": user.name,
                    "reset_link": reset_link,
                    "expires_in_minutes": 60,
                }),
            )
            .await;
    }

    // Always return success to prevent user enumeration
    Ok(Json(MessageResponse {
        message: "If your email is registered, you will receive a password reset link shortly."
            .to_string(),
        success: Some(true),
    }))
}

/// Reset password with token
/// POST /api/auth/reset-password
// FIX-2026-04-26 (Priority 2): migrated Json<ResetPasswordRequest> -> ValidatedJson<...>.
// FIX-2026-04-26 (Priority 4): added HeaderMap + per-IP rate limit (5 / hour).
// Original signature: async fn reset_password(State(state): State<AppState>, Json(input): Json<ResetPasswordRequest>)
async fn reset_password(
    State(state): State<AppState>,
    peer: Option<ConnectInfo<SocketAddr>>,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (5/hour) — fail-closed.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware).
    let ip = client_ip(
        peer.map(|ci| ci.0),
        &headers,
        &state.config.trusted_proxy_cidrs,
    );
    enforce_ip_rate_limit_strict(&state, &ip, "reset_password", 5, 3600).await?;

    // Validate passwords match
    if input.password != input.password_confirmation {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password_confirmation": ["Passwords do not match"]
                }
            })),
        ));
    }

    // ICT L11+ Security: Validate password strength with hardened rules
    if let Err(password_error) = validate_password(&input.password) {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Validation failed",
                "errors": {
                    "password": [password_error]
                }
            })),
        ));
    }

    // Hash the token and find it in the database
    let hashed_token = hash_token(&input.token);

    let reset_record: Option<(uuid::Uuid, String)> = sqlx::query_as(
        r#"
        SELECT id, email FROM password_resets 
        WHERE token = $1 AND email = $2 AND expires_at > NOW()
        "#,
    )
    .bind(&hashed_token)
    .bind(&input.email)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let (reset_id, _email) = reset_record.ok_or_else(|| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Invalid or expired reset token",
                "errors": {
                    "token": ["Invalid reset token"]
                }
            })),
        )
    })?;

    // Hash the new password
    let password_hash = hash_password(&input.password).map_err(|e| {
        tracing::error!("Password hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Password hashing failed"})),
        )
    })?;

    // Update user's password
    let result =
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE email = $2")
            .bind(&password_hash)
            .bind(&input.email)
            .execute(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
            })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Failed to reset password",
                "errors": {
                    "email": ["User not found"]
                }
            })),
        ));
    }

    // Delete the used reset token
    sqlx::query("DELETE FROM password_resets WHERE id = $1")
        .bind(reset_id)
        .execute(&state.db.pool)
        .await
        .ok();

    // FIX-H-2 (2026-04-29): a successful password reset MUST invalidate every
    // active session and bust the user cache. Without this, a thief who
    // already holds a valid access or refresh token continues to have access
    // for up to 1h (access) or 7d (refresh) — exactly the window the reset
    // was supposed to close.
    //
    // We need user_id; the handler so far has only used `email`. Look it up
    // here (best-effort: a missing row at this stage means the password
    // update we just did affected zero rows, which we already returned 422
    // for — so this fetch should succeed).
    let user_id_for_invalidate: Option<i64> =
        sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
            .bind(&input.email)
            .fetch_optional(&state.db.pool)
            .await
            .unwrap_or_else(|e| {
                tracing::error!(
                    target: "security_audit",
                    event = "password_reset_user_lookup_failed",
                    error = %e,
                    "Could not look up user_id after password reset (sessions NOT invalidated)"
                );
                None
            });

    if let Some(uid) = user_id_for_invalidate {
        if let Some(redis) = &state.services.redis {
            // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): bump the token
            // epoch on password reset. Pre-fix, the reset only deleted
            // `session:*` keys, so a thief holding a valid access (≤1h) or
            // refresh (≤7d) token kept access for exactly the window the
            // reset was meant to close. The bump revokes every prior
            // access/refresh token for this user immediately. Best-effort,
            // same rationale as the session sweep below (password is already
            // changed; the extractor fails closed on a Redis fault anyway).
            if let Err(e) = redis.bump_token_version(uid).await {
                tracing::error!(
                    target: "security_audit",
                    event = "password_reset_token_version_bump_failed",
                    user_id = %uid,
                    error = %e,
                    "Could not bump token epoch on password reset (extractor fails closed on Redis error)"
                );
            }
            // Best-effort: a Redis outage here logs but does not fail the
            // reset. The password is already changed; rolling that back
            // would be worse than leaving residual sessions.
            if let Err(e) = redis.invalidate_all_user_sessions(uid).await {
                tracing::error!(
                    target: "security_audit",
                    event = "password_reset_session_invalidation_failed",
                    user_id = %uid,
                    error = %e,
                    "Could not invalidate sessions on password reset"
                );
            }
            if let Err(e) = redis.invalidate_user_cache(uid).await {
                tracing::warn!(
                    target: "security_audit",
                    event = "password_reset_cache_invalidation_failed",
                    user_id = %uid,
                    error = %e,
                    "Could not invalidate user cache on password reset"
                );
            }
        }

        // Audit trail row in security_events. event_category/severity match
        // the schema used by H-7 role_change writes (admin.rs:332-339).
        if let Err(e) = sqlx::query(
            r#"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
               VALUES ($1, 'password_reset', 'authentication', 'high', '{}'::jsonb)"#,
        )
        .bind(uid)
        .execute(&state.db.pool)
        .await
        {
            tracing::error!(
                target: "security_audit",
                event = "password_reset_audit_write_failed",
                user_id = %uid,
                error = %e,
                "Failed to write password_reset audit event"
            );
        }
    }

    tracing::info!(
        target: "security_audit",
        event = "password_reset_completed",
        user_id = ?user_id_for_invalidate,
        "Password reset completed and sessions invalidated"
    );

    Ok(Json(MessageResponse {
        message: "Password has been reset successfully. You can now login with your new password."
            .to_string(),
        success: Some(true),
    }))
}

/// Build the auth router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .route("/logout", post(logout))
        .route("/logout-all", post(logout_all)) // ICT L11+ Security: Force logout everywhere
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/verify-email", get(verify_email))
        .route("/resend-verification", post(resend_verification))
}

/// Router for /logout at top level (frontend compatibility)
pub fn logout_router() -> Router<AppState> {
    Router::new().route("/", post(logout))
}
