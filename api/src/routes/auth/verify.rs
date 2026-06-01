//! Email verification handlers — GET /verify-email + POST /resend-verification.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::{MessageResponse, User},
    utils::{generate_verification_token, hash_token},
    AppState,
};

/// Query params for email verification
#[derive(Debug, Deserialize)]
pub(super) struct VerifyEmailQuery {
    token: String,
}

/// Request to resend verification email
#[derive(Debug, Deserialize)]
pub(super) struct ResendVerificationRequest {
    email: String,
}

/// Verify email with token
/// GET /api/auth/verify-email?token=xxx
pub(super) async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    let hashed_token = hash_token(&query.token);

    // Find the verification token
    let token_record: Option<(i64, i64)> = sqlx::query_as(
        r"
        SELECT id, user_id FROM email_verification_tokens
        WHERE token = $1 AND expires_at > NOW()
        ",
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
pub(super) async fn resend_verification(
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
