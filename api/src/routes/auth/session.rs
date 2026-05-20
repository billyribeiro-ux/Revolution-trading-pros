//! Session handlers — GET /me + POST /logout + POST /logout-all.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::Deserialize;
use serde_json::json;

use super::helpers::hash_token_for_blacklist;
use crate::{
    models::{MessageResponse, User, UserResponse},
    AppState,
};

/// Get current user (requires auth)
/// GET /api/auth/me
pub(super) async fn me(user: User) -> Json<UserResponse> {
    Json(user.into())
}

/// Logout request with session ID and optional token
#[derive(Debug, Deserialize)]
pub(super) struct LogoutRequest {
    session_id: Option<String>,
    #[serde(default)]
    token: Option<String>,
}

/// Logout user - ICT L11+ Security: Proper session and token invalidation
/// POST /api/auth/logout
pub(super) async fn logout(
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
pub(super) async fn logout_all(
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
