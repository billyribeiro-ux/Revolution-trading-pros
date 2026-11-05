//! Refresh rotates exactly once using the database authority, regardless of Redis state.
use crate::{
    AppState,
    models::{RefreshTokenRequest, RefreshTokenResponse},
    services::account_authority,
    utils::{create_jwt_authenticated, create_refresh_token_authenticated, verify_jwt},
};
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
pub(super) async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, (StatusCode, Json<serde_json::Value>)> {
    let claims =
        verify_jwt(&input.refresh_token, &state.config.jwt_secret, "refresh").map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Invalid or expired refresh token"})),
            )
        })?;
    let account = account_authority::consume_refresh(&state.db.pool, &claims)
        .await
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    let token = create_jwt_authenticated(
        account.user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        account.version,
        claims.mfa_assured,
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Token creation failed"})),
        )
    })?;
    let refresh_token = create_refresh_token_authenticated(
        account.user.id,
        &state.config.jwt_secret,
        account.version,
        claims.mfa_assured,
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Token creation failed"})),
        )
    })?;
    Ok(Json(RefreshTokenResponse {
        token: token.clone(),
        access_token: token,
        refresh_token,
        expires_in: state.config.jwt_expires_in * 3600,
    }))
}
