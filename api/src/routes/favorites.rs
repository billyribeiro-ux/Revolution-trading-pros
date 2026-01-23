//! User Favorites API - Persistent bookmarking system
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 2026
//!
//! Full CRUD for user favorites with room-scoped access

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tracing::{error, info};

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct UserFavorite {
    pub id: i64,
    pub user_id: i64,
    pub room_slug: String,
    pub item_type: String,
    pub item_id: i64,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub href: Option<String>,
    pub thumbnail_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFavoriteRequest {
    pub room_slug: String,
    pub item_type: String,
    pub item_id: i64,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub href: Option<String>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListFavoritesQuery {
    pub room_slug: Option<String>,
    pub item_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CheckFavoriteQuery {
    pub item_type: String,
    pub item_id: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List user's favorites
async fn list_favorites(
    State(state): State<AppState>,
    auth: User,
    Query(query): Query<ListFavoritesQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let favorites: Vec<UserFavorite> = match (&query.room_slug, &query.item_type) {
        (Some(room), Some(item_type)) => {
            sqlx::query_as(
                r#"SELECT * FROM user_favorites 
                   WHERE user_id = $1 AND room_slug = $4 AND item_type = $5
                   ORDER BY created_at DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(auth.id)
            .bind(per_page)
            .bind(offset)
            .bind(room)
            .bind(item_type)
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(room), None) => {
            sqlx::query_as(
                r#"SELECT * FROM user_favorites 
                   WHERE user_id = $1 AND room_slug = $4
                   ORDER BY created_at DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(auth.id)
            .bind(per_page)
            .bind(offset)
            .bind(room)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, Some(item_type)) => {
            sqlx::query_as(
                r#"SELECT * FROM user_favorites 
                   WHERE user_id = $1 AND item_type = $4
                   ORDER BY created_at DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(auth.id)
            .bind(per_page)
            .bind(offset)
            .bind(item_type)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, None) => {
            sqlx::query_as(
                r#"SELECT * FROM user_favorites 
                   WHERE user_id = $1
                   ORDER BY created_at DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(auth.id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
    }
    .map_err(|e| {
        error!("Failed to fetch favorites: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM user_favorites WHERE user_id = $1")
        .bind(auth.id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "data": favorites,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

/// Add item to favorites
async fn add_favorite(
    State(state): State<AppState>,
    auth: User,
    Json(input): Json<CreateFavoriteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check if already favorited
    let existing: Option<UserFavorite> = sqlx::query_as(
        "SELECT * FROM user_favorites WHERE user_id = $1 AND item_type = $2 AND item_id = $3",
    )
    .bind(auth.id)
    .bind(&input.item_type)
    .bind(input.item_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if existing.is_some() {
        return Ok(Json(json!({
            "success": true,
            "message": "Already in favorites",
            "already_exists": true
        })));
    }

    let favorite: UserFavorite = sqlx::query_as(
        r#"INSERT INTO user_favorites 
           (user_id, room_slug, item_type, item_id, title, excerpt, href, thumbnail_url)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING *"#,
    )
    .bind(auth.id)
    .bind(&input.room_slug)
    .bind(&input.item_type)
    .bind(input.item_id)
    .bind(&input.title)
    .bind(&input.excerpt)
    .bind(&input.href)
    .bind(&input.thumbnail_url)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to add favorite: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "User {} added favorite: {} #{}",
        auth.id, input.item_type, input.item_id
    );

    Ok(Json(json!({
        "success": true,
        "data": favorite,
        "message": "Added to favorites"
    })))
}

/// Remove item from favorites
async fn remove_favorite(
    State(state): State<AppState>,
    auth: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM user_favorites WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(auth.id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Favorite not found"})),
        ));
    }

    info!("User {} removed favorite #{}", auth.id, id);

    Ok(Json(json!({
        "success": true,
        "message": "Removed from favorites"
    })))
}

/// Remove favorite by item type and ID
async fn remove_favorite_by_item(
    State(state): State<AppState>,
    auth: User,
    Query(query): Query<CheckFavoriteQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "DELETE FROM user_favorites WHERE user_id = $1 AND item_type = $2 AND item_id = $3",
    )
    .bind(auth.id)
    .bind(&query.item_type)
    .bind(query.item_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "removed": result.rows_affected() > 0,
        "message": if result.rows_affected() > 0 { "Removed from favorites" } else { "Not in favorites" }
    })))
}

/// Check if item is favorited
async fn check_favorite(
    State(state): State<AppState>,
    auth: User,
    Query(query): Query<CheckFavoriteQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let favorite: Option<UserFavorite> = sqlx::query_as(
        "SELECT * FROM user_favorites WHERE user_id = $1 AND item_type = $2 AND item_id = $3",
    )
    .bind(auth.id)
    .bind(&query.item_type)
    .bind(query.item_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "is_favorited": favorite.is_some(),
        "data": favorite
    })))
}

/// Get favorites for a specific room
async fn list_room_favorites(
    State(state): State<AppState>,
    auth: User,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let favorites: Vec<UserFavorite> = sqlx::query_as(
        r#"SELECT * FROM user_favorites 
           WHERE user_id = $1 AND room_slug = $2
           ORDER BY created_at DESC"#,
    )
    .bind(auth.id)
    .bind(&room_slug)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch room favorites: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": favorites,
        "room_slug": room_slug
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_favorites))
        .route("/", post(add_favorite))
        .route("/:id", delete(remove_favorite))
        .route("/remove", delete(remove_favorite_by_item))
        .route("/check", get(check_favorite))
        .route("/room/:room_slug", get(list_room_favorites))
}
