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

use axum::{
    extract::{Query, State},
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
    // FIX-2026-04-26 (Priority 2): pull in ValidatedJson extractor for hot-path handlers.
    middleware::validated_json::ValidatedJson,
    models::{
        AuthResponse, CreateUser, ForgotPasswordRequest, LoginUser, MessageResponse,
        RefreshTokenRequest, RefreshTokenResponse, ResetPasswordRequest, User, UserResponse,
    },
    utils::{
        create_jwt, create_refresh_token, generate_session_id, generate_verification_token,
        hash_dummy_password, hash_password, hash_token, validate_password, verify_jwt,
        verify_password,
    },
    AppState,
};

/// FIX-2026-04-26 (Priority 4): client IP extraction helper.
/// Reads `x-forwarded-for` (first hop), then `x-real-ip`, then falls back to `"unknown"`.
/// Used to scope per-IP rate limits on register/forgot-password/reset-password and to
/// add per-IP scope to login (in addition to the existing per-email scope).
fn client_ip(headers: &HeaderMap) -> String {
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = xff.split(',').next() {
            let ip = first.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = real_ip.trim();
        if !ip.is_empty() {
            return ip.to_string();
        }
    }
    "unknown".to_string()
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
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<CreateUser>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit BEFORE Argon2 hashing
    // (otherwise an attacker can DOS by burning CPU on hash attempts).
    let ip = client_ip(&headers);
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

    // Store verification token (expires in 24 hours)
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

    // FIX-2026-04-26 (Priority 5): commit BEFORE any external HTTP calls (email send).
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit registration transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // ICT 11+ EMAIL VERIFICATION: Send verification email with comprehensive logging
    if let Some(ref email_service) = state.services.email {
        match email_service
            .send_verification_email(&user.email, &user.name, &raw_token)
            .await
        {
            Ok(_) => {
                tracing::info!(
                    target: "security_audit",
                    event = "verification_email_sent",
                    user_id = %user.id,
                    email = %user.email,
                    token_expires = "24_hours",
                    "ICT 11+ AUDIT: Verification email sent successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "verification_email_failed",
                    user_id = %user.id,
                    email = %user.email,
                    error = %e,
                    "ICT 11+ ALERT: Failed to send verification email - user can resend"
                );
                // Don't fail registration if email fails - user can resend
            }
        }
    } else {
        tracing::warn!(
            target: "security_audit",
            event = "email_service_not_configured",
            user_id = %user.id,
            email = %user.email,
            "ICT 11+ WARNING: Email service not configured - verification email not sent"
        );
    }

    tracing::info!(
        target: "security_audit",
        event = "user_registered",
        user_id = %user.id,
        email = %user.email,
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
        email = %user.email,
        verified_at = %chrono::Utc::now().to_rfc3339(),
        "ICT 11+ AUDIT: Email successfully verified - user can now login"
    );

    // ICT 11+ UX: Send welcome email with comprehensive logging
    if let Some(ref email_service) = state.services.email {
        match email_service
            .send_welcome_email(&user.email, &user.name)
            .await
        {
            Ok(_) => {
                tracing::info!(
                    target: "security_audit",
                    event = "welcome_email_sent",
                    user_id = %user.id,
                    email = %user.email,
                    "ICT 11+ AUDIT: Welcome email sent successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "welcome_email_failed",
                    user_id = %user.id,
                    email = %user.email,
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
            email = %input.email,
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
            email = %user.email,
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

    // ICT 11+ EMAIL: Send verification email with comprehensive logging
    if let Some(ref email_service) = state.services.email {
        match email_service
            .send_verification_email(&user.email, &user.name, &raw_token)
            .await
        {
            Ok(_) => {
                tracing::info!(
                    target: "security_audit",
                    event = "verification_email_resent",
                    user_id = %user.id,
                    email = %user.email,
                    token_expires = "24_hours",
                    "ICT 11+ AUDIT: Verification email resent successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    target: "security_audit",
                    event = "verification_email_resend_failed",
                    user_id = %user.id,
                    email = %user.email,
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
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<LoginUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Security audit logging
    tracing::info!(
        target: "security",
        event = "login_attempt",
        email = %input.email,
        "Login attempt initiated"
    );

    // FIX-2026-04-27 (H-2): Both per-IP and per-email checks now FAIL CLOSED.
    // A Redis outage must not become a free pass for brute-force attacks.
    let ip = client_ip(&headers);
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
                email = %input.email,
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
            email = %user.email,
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

    // FIX-2026-04-27 (C-1): Block banned/deactivated accounts at login.
    // is_active == Some(false) means an admin explicitly banned this account.
    // NULL (legacy rows) and Some(true) are both treated as active.
    if user.is_active == Some(false) {
        tracing::warn!(
            target: "security_audit",
            event = "login_blocked_banned",
            user_id = %user.id,
            email = %user.email,
            "Login blocked - account is banned/deactivated"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // ICT 11+ ENTERPRISE SECURITY: Email Verification Enforcement
    // Apple Principal Engineer Grade - Zero Trust Security Model
    // Only developers and superadmins bypass verification for operational access
    let is_developer =
        state.config.is_developer_email(&user.email) || user.role.as_deref() == Some("developer");

    let is_superadmin = state.config.is_superadmin_email(&user.email)
        || user.role.as_deref() == Some("super_admin")
        || user.role.as_deref() == Some("super-admin");

    // ICT 11+ SECURITY: Strict verification enforcement
    // Only privileged roles bypass - all other users MUST verify email
    let bypass_verification = is_developer || is_superadmin;

    // ICT 11+ AUDIT: Log all verification bypass attempts
    if bypass_verification && user.email_verified_at.is_none() {
        let role_type = if is_developer {
            "developer"
        } else {
            "superadmin"
        };
        tracing::warn!(
            target: "security_audit",
            event = "email_verification_bypassed",
            user_id = %user.id,
            email = %user.email,
            role = %role_type,
            reason = "privileged_role_access",
            "ICT 11+ AUDIT: Email verification bypassed for privileged user"
        );
    }

    // ICT 11+ SECURITY GATE: Block unverified users
    if user.email_verified_at.is_none() && !bypass_verification {
        tracing::warn!(
            target: "security",
            event = "login_blocked_unverified",
            user_id = %user.id,
            email = %user.email,
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

    // ICT 11+ AUDIT: Log successful verification bypass
    if bypass_verification && user.email_verified_at.is_none() {
        let role_type = if is_developer {
            "developer"
        } else {
            "superadmin"
        };
        tracing::info!(
            target: "security_audit",
            event = "privileged_verification_bypass",
            role = role_type,
            user_id = %user.id,
            email = %user.email,
            "Privileged user bypassing email verification requirement"
        );
    }

    // Create access token
    let token = create_jwt(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Token creation failed"})),
        )
    })?;

    // Create refresh token
    let refresh_token = create_refresh_token(user.id, &state.config.jwt_secret).map_err(|e| {
        tracing::error!("Refresh token creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Refresh token creation failed"})),
        )
    })?;

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
        email = %user.email,
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

    // Create new access token
    let token = create_jwt(
        claims.sub,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
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
        create_refresh_token(claims.sub, &state.config.jwt_secret).map_err(|e| {
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
        email = %user.email,
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
        email = %user.email,
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
    headers: HeaderMap,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (3/hour) — fail-closed.
    let ip = client_ip(&headers);
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

        // Store reset token (expires in 1 hour)
        // Original pool reference: .execute(&state.db.pool)
        sqlx::query(
            r#"
            INSERT INTO password_resets (id, email, token, expires_at, created_at)
            VALUES (gen_random_uuid(), $1, $2, NOW() + INTERVAL '1 hour', NOW())
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

        // Send password reset email
        if let Some(ref email_service) = state.services.email {
            if let Err(e) = email_service
                .send_password_reset(&user.email, &user.name, &raw_token)
                .await
            {
                tracing::error!("Failed to send password reset email: {}", e);
            } else {
                tracing::info!("Password reset email sent to: {}", user.email);
            }
        }
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
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (5/hour) — fail-closed.
    let ip = client_ip(&headers);
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

    tracing::info!("Password reset completed for: {}", input.email);

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
