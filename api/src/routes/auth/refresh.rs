//! Refresh-token handler — POST /api/auth/refresh.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::helpers::{current_token_version, hash_token_for_blacklist};
use crate::{
    models::{RefreshTokenRequest, RefreshTokenResponse},
    utils::{create_jwt_versioned, create_refresh_token_versioned, verify_jwt},
    AppState,
};

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
pub(super) async fn refresh(
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
