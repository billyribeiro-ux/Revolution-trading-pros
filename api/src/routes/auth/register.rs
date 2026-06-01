//! Registration handler — POST /api/auth/register.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;

use super::helpers::{client_ip, enforce_ip_rate_limit_strict};
use crate::{
    // FIX-2026-04-26 (Priority 2): pull in ValidatedJson extractor for hot-path handlers.
    middleware::validated_json::ValidatedJson,
    models::{CreateUser, User},
    utils::{generate_verification_token, hash_password, validate_password},
    AppState,
};

/// Registration response (no tokens - must verify email first)
#[derive(Debug, serde::Serialize)]
pub(super) struct RegisterResponse {
    message: String,
    email: String,
    requires_verification: bool,
}

/// Register a new user
/// POST /api/auth/register
// FIX-2026-04-26 (Priority 2): migrated Json<CreateUser> -> ValidatedJson<CreateUser>.
// FIX-2026-04-26 (Priority 4): added HeaderMap + per-IP rate limit (5 / 15min).
// Original signature: async fn register(State(state): State<AppState>, Json(input): Json<CreateUser>)
pub(super) async fn register(
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
        r"
        INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
        VALUES ($1, $2, $3, 'user', NULL, NOW(), NOW())
        RETURNING id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at
        ",
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
        r"
        INSERT INTO email_verification_tokens (user_id, token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '24 hours')
        ",
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
