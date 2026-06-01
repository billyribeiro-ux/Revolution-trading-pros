//! CRM contact-lists + tags handlers
//! (split from `crm.rs` lines 90-112 types + 504-713 handlers).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use super::ListFilters;
use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactList {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub contacts_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactTag {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub contacts_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagInput {
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Lists
// ═══════════════════════════════════════════════════════════════════════════

async fn list_contact_lists(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let lists: Vec<ContactList> = sqlx::query_as(
        r"
        SELECT id, title, slug, description, is_public,
               COALESCE(contacts_count, 0) as contacts_count, created_at, updated_at
        FROM crm_lists
        WHERE ($1::text IS NULL OR title ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        ORDER BY created_at DESC
        LIMIT $3
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(filters.is_public)
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "data": lists,
        "meta": { "total": lists.len() }
    })))
}

async fn create_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");

    let list: ContactList = sqlx::query_as(
        r"
        INSERT INTO crm_lists (title, slug, description, is_public, contacts_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, title, slug, description, is_public, contacts_count, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(input.is_public.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(list))
}

async fn get_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: ContactList = sqlx::query_as(
        "SELECT id, title, slug, description, is_public, contacts_count, created_at, updated_at FROM crm_lists WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "List not found"}))))?;

    Ok(Json(json!({
        "list": list,
        "contacts_count": list.contacts_count
    })))
}

async fn delete_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_lists WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "List deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Tags
// ═══════════════════════════════════════════════════════════════════════════

async fn list_contact_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let tags: Vec<ContactTag> = sqlx::query_as(
        r"
        SELECT id, title, slug, description, color,
               COALESCE(contacts_count, 0) as contacts_count, created_at, updated_at
        FROM crm_tags
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "data": tags,
        "meta": { "total": tags.len() }
    })))
}

async fn create_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTagInput>,
) -> Result<Json<ContactTag>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");

    let tag: ContactTag = sqlx::query_as(
        r"
        INSERT INTO crm_tags (title, slug, description, color, contacts_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, title, slug, description, color, contacts_count, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(tag))
}

async fn delete_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_tags WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Tag deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/lists", get(list_contact_lists).post(create_contact_list))
        .route(
            "/lists/:id",
            get(get_contact_list).delete(delete_contact_list),
        )
        .route(
            "/contact-tags",
            get(list_contact_tags).post(create_contact_tag),
        )
        .route("/contact-tags/:id", delete(delete_contact_tag))
}
