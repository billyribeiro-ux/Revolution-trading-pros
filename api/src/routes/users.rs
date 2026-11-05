//! User routes - ICT Level 7 Security Fix
//! CRITICAL: All routes now require AdminUser authentication
//! Previously these endpoints were PUBLIC - exposing all user data

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde_json::json;

use crate::{AppState, middleware::admin::AdminUser, models::UserResponse};

/// Get user by ID - REQUIRES ADMIN AUTH
/// ICT Level 7 Security: Protected endpoint
async fn get_user(
    State(state): State<AppState>,
    _admin: AdminUser, // ICT 7 FIX: Require admin authentication
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let value:serde_json::Value=sqlx::query_scalar(
        "SELECT to_jsonb(record) FROM(SELECT id,email,name,role,is_active,email_verified_at,avatar_url,mfa_enabled,created_at,updated_at FROM users WHERE id=$1 AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted') record"
    )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "User not found"}))))?;

    let user = crate::services::account_authority::checked_user(value)
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    Ok(Json(user.into()))
}

/// List all users - REQUIRES ADMIN AUTH
/// ICT Level 7 Security: Protected endpoint
async fn list_users(
    State(state): State<AppState>,
    _admin: AdminUser, // ICT 7 FIX: Require admin authentication
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 FIX: Use explicit column list to avoid SQLx FromRow deserialization errors
    let values:Vec<serde_json::Value>=sqlx::query_scalar(
        "SELECT to_jsonb(record) FROM(SELECT id,email,name,role,is_active,email_verified_at,avatar_url,mfa_enabled,created_at,updated_at FROM users WHERE deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' ORDER BY created_at DESC NULLS LAST,id DESC LIMIT 100) record",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let users = values
        .into_iter()
        .map(crate::services::account_authority::checked_user)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user))
}
