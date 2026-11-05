//! Session handlers — GET /me + POST /logout + POST /logout-all.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use axum::{Extension, Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Deserialize;
use serde_json::json;

use crate::services::account_authority;
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

fn revocation_error(error: account_authority::Failure) -> (StatusCode, Json<serde_json::Value>) {
    (error.0, Json(json!({"error":error.1})))
}

/// Logout request with session ID and optional token
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
    if input.session_id.as_ref().is_some_and(|v| v.len() > 256)
        || input.token.as_ref().is_some_and(|v| v.len() > 8192)
        || input.refresh_token.as_ref().is_some_and(|v| v.len() > 8192)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Session credentials exceed their limits"})),
        ));
    }
    if input
        .token
        .as_ref()
        .is_some_and(|token| token != auth_header.token())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Body access token must match the authenticated token"})),
        ));
    }
    let access =
        verify_jwt(auth_header.token(), &state.config.jwt_secret, "access").map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Invalid access token"})),
            )
        })?;
    let mut tokens = vec![access];
    if let Some(raw) = input.refresh_token.as_deref() {
        let refresh = verify_jwt(raw, &state.config.jwt_secret, "refresh").map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Invalid refresh token"})),
            )
        })?;
        if refresh.sub != user.id {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error":"Refresh token belongs to another account"})),
            ));
        }
        tokens.push(refresh);
    }
    account_authority::revoke_tokens(&state.db.pool, user.id, &tokens)
        .await
        .map_err(revocation_error)?;
    // Redis sessions are optional UI bookkeeping, not revocation authority.
    if let (Some(redis), Some(session_id)) = (&state.services.redis, input.session_id)
        && let Ok(Ok(Some(session))) = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            redis.get_session(&session_id),
        )
        .await
        && session.user_id == user.id
        && !matches!(
            tokio::time::timeout(
                std::time::Duration::from_secs(2),
                redis.invalidate_session(&session_id)
            )
            .await,
            Ok(Ok(()))
        )
    {
        tracing::warn!("Durably revoked tokens; optional Redis session cleanup was not confirmed");
    }

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
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let auth_version = account_authority::revoke_all(&state.db.pool, &verified)
        .await
        .map_err(revocation_error)?;
    let count = if let Some(redis) = &state.services.redis {
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            redis.invalidate_all_user_sessions(user.id),
        )
        .await
        .ok()
        .and_then(Result::ok)
    } else {
        None
    };

    tracing::info!(
        target: "security",
        event = "logout_all",
        user_id = %user.id,
        sessions_invalidated = ?count,
        "User logged out from all devices"
    );

    Ok(Json(json!({
        "message": "All previously issued account tokens were revoked",
        "auth_version":auth_version,
        "sessions_invalidated": count,
        "success": true
    })))
}
