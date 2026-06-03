//! Password-flow handlers — POST /forgot-password + POST /reset-password.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;

use super::helpers::{client_ip, enforce_ip_rate_limit_strict, ClientAddr};
use crate::{
    // FIX-2026-04-26 (Priority 2): pull in ValidatedJson extractor for hot-path handlers.
    middleware::validated_json::ValidatedJson,
    models::{ForgotPasswordRequest, MessageResponse, ResetPasswordRequest, User},
    utils::{generate_verification_token, hash_password, hash_token, validate_password},
    AppState,
};

/// Request password reset
/// POST /api/auth/forgot-password
// FIX-2026-04-26 (Priority 4): added HeaderMap + per-IP rate limit (3 / hour).
// Original signature: async fn forgot_password(State(state): State<AppState>, Json(input): Json<ForgotPasswordRequest>)
pub(super) async fn forgot_password(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (3/hour) — fail-closed.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware).
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
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
            r"
            INSERT INTO password_resets (email, token, expires_at, created_at)
            VALUES ($1, $2, NOW() + INTERVAL '1 hour', NOW())
            ",
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
pub(super) async fn reset_password(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    // FIX-2026-04-26 (Priority 4): per-IP rate limit (5/hour) — fail-closed.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware).
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
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
        r"
        SELECT id, email FROM password_resets
        WHERE token = $1 AND email = $2 AND expires_at > NOW()
        ",
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
            r"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
               VALUES ($1, 'password_reset', 'authentication', 'high', '{}'::jsonb)",
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
