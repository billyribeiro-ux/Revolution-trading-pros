//! User routes - ICT Level 7 Security Fix
//! CRITICAL: All routes now require AdminUser authentication
//! Previously these endpoints were PUBLIC - exposing all user data

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{middleware::admin::AdminUser, models::UserResponse, AppState};

/// Get user by ID - REQUIRES ADMIN AUTH
/// ICT Level 7 Security: Protected endpoint
async fn get_user(
    State(state): State<AppState>,
    _admin: AdminUser, // ICT 7 FIX: Require admin authentication
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user: crate::models::User = sqlx::query_as(
        "SELECT id, email, password_hash, name, role, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE id = $1"
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

    Ok(Json(user.into()))
}

/// List all users - REQUIRES ADMIN AUTH
/// ICT Level 7 Security: Protected endpoint
async fn list_users(
    State(state): State<AppState>,
    _admin: AdminUser, // ICT 7 FIX: Require admin authentication
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 FIX: Use explicit column list to avoid SQLx FromRow deserialization errors
    let users: Vec<crate::models::User> =
        sqlx::query_as(
            r#"SELECT id, email, password_hash, name, role, email_verified_at, 
                      avatar_url, mfa_enabled, created_at, updated_at 
               FROM users ORDER BY created_at DESC LIMIT 100"#
        )
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(users.into_iter().map(|u| u.into()).collect()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
}
