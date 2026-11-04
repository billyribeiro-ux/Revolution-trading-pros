//! Session handlers — GET /me + POST /logout + POST /logout-all.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Deserialize;
use serde_json::json;

use super::helpers::hash_token_for_blacklist;
use crate::{
    AppState,
    models::{MessageResponse, User, UserResponse},
    utils::verify_jwt,
};

/// Get current user (requires auth)
/// GET /api/auth/me
pub(super) async fn me(State(state): State<AppState>, user: User) -> Json<UserResponse> {
    Json(UserResponse::from_authenticated(user, &state.config))
}

fn revocation_error(error: anyhow::Error) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(%error, "Session revocation failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"Session revocation failed"})),
    )
}

/// Logout request with session ID and optional token
#[derive(Debug, Deserialize)]
pub(super) struct LogoutRequest {
    session_id: Option<String>,
    #[serde(default)]
    token: Option<String>,
    #[serde(default)]
    refresh_token: Option<String>,
}

/// Logout user - ICT L11+ Security: Proper session and token invalidation
/// POST /api/auth/logout
pub(super) async fn logout(
    State(state): State<AppState>,
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    user: User,
    Json(input): Json<LogoutRequest>,
) -> Result<Json<MessageResponse>, (StatusCode, Json<serde_json::Value>)> {
    let redis = state.services.redis.as_ref().ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error":"Session revocation is unavailable"})),
        )
    })?;
    let refresh = input
        .refresh_token
        .as_deref()
        .map(|token| {
            verify_jwt(token, &state.config.jwt_secret, "refresh")
                .map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error":"Invalid refresh token"})),
                    )
                })
                .and_then(|claims| {
                    if claims.sub == user.id {
                        Ok((token, claims.exp))
                    } else {
                        Err((
                            StatusCode::FORBIDDEN,
                            Json(json!({"error":"Refresh token belongs to another user"})),
                        ))
                    }
                })
        })
        .transpose()?;
    let access =
        verify_jwt(auth_header.token(), &state.config.jwt_secret, "access").map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Invalid access token"})),
            )
        })?;
    // Use each token's actual remaining validity; never report revocation
    // success after a failed write to the authoritative blacklist.
    let now = chrono::Utc::now().timestamp();
    redis
        .blacklist_token(
            &hash_token_for_blacklist(auth_header.token()),
            (access.exp - now).max(1) as u64,
        )
        .await
        .map_err(revocation_error)?;
    if let Some((token, expiry)) = refresh {
        redis
            .blacklist_token(
                &hash_token_for_blacklist(token),
                (expiry - now).max(1) as u64,
            )
            .await
            .map_err(revocation_error)?;
    }
    if let Some(session_id) = input.session_id
        && let Some(session) = redis
            .get_session(&session_id)
            .await
            .map_err(revocation_error)?
        && session.user_id == user.id
    {
        redis
            .invalidate_session(&session_id)
            .await
            .map_err(revocation_error)?;
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
