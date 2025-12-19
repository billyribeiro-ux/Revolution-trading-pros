//! User routes (singular) - Revolution Trading Pros
//! Routes for /api/user/* endpoints expected by frontend
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::json;

use crate::{
    models::User,
    routes::subscriptions::UserSubscriptionRow,
    AppState,
};

/// Get user's memberships/subscriptions
/// Matches frontend expectation: GET /api/user/memberships
async fn get_memberships(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Vec<UserSubscriptionRow>>, (StatusCode, Json<serde_json::Value>)> {
    let subscriptions: Vec<UserSubscriptionRow> = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(subscriptions))
}

/// Get current user profile
/// Matches frontend expectation: GET /api/user/profile
async fn get_profile(user: User) -> Json<crate::models::UserResponse> {
    Json(user.into())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/memberships", get(get_memberships))
        .route("/profile", get(get_profile))
}

