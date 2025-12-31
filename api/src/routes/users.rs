//! User routes
//! ICT 11+ Fix: Changed Path parameter from Uuid to i64 to match database schema

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{models::UserResponse, AppState};

/// Get user by ID
/// ICT 11+ Fix: User ID is i64 (BIGINT) in database, not UUID
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user: crate::models::User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
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

/// List all users (admin only)
async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, Json<serde_json::Value>)> {
    let users: Vec<crate::models::User> = sqlx::query_as(
        "SELECT * FROM users ORDER BY created_at DESC LIMIT 100"
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
