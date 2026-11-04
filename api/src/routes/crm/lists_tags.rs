//! CRM contact-lists + tags handlers
//! (split from `crm.rs` lines 90-112 types + 504-713 handlers).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use super::ListFilters;
use crate::{AppState, middleware::admin::AdminUser};

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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactTag {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub contacts_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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

#[derive(Deserialize)]
struct ContactListFilters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    is_public: Option<bool>,
}

async fn list_contact_lists(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ContactListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let page = filters.page.unwrap_or(1).max(1);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let lists: Vec<ContactList> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, name) AS title,
               COALESCE(slug, LOWER(REPLACE(name, ' ', '-'))) AS slug,
               description, COALESCE(is_public, false) AS is_public,
               (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count, created_at, updated_at
        FROM crm_lists
        WHERE ($1::text IS NULL OR COALESCE(title, name) ILIKE $1)
          AND ($2::boolean IS NULL OR COALESCE(is_public,false) = $2)
        ORDER BY created_at DESC, id DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(filters.is_public)
    .bind(per_page)
    .bind(page.saturating_sub(1).saturating_mul(per_page))
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_lists WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1) AND ($2::boolean IS NULL OR COALESCE(is_public,false)=$2)")
        .bind(search_pattern.as_deref()).bind(filters.is_public).fetch_one(&state.db.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?;
    Ok(Json(json!({
        "data": lists,
        "meta": { "total": total, "current_page": page, "per_page": per_page, "total_pages": ((total+per_page-1)/per_page).max(1) }
    })))
}

fn validate_list(input: &mut CreateListInput) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    input.title = input.title.trim().to_string();
    if input.title.is_empty()
        || input.title.len() > 255
        || input.description.as_ref().is_some_and(|s| s.len() > 50_000)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error":"Provide a list title under 255 characters and description under 50 KB"}),
            ),
        ));
    }
    Ok(())
}

async fn update_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(mut input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    validate_list(&mut input)?;
    let result = sqlx::query_as("UPDATE crm_lists SET name=$2,title=$2,description=$3,is_public=$4,updated_at=NOW() WHERE id=$1 RETURNING id,COALESCE(title,name) AS title,COALESCE(slug,'') AS slug,description,COALESCE(is_public,false) AS is_public,(SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count,created_at,updated_at")
        .bind(id).bind(input.title).bind(input.description).bind(input.is_public.unwrap_or(false))
        .fetch_optional(&state.db.pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND,Json(json!({"error":"List not found"}))))?;
    Ok(Json(result))
}

async fn create_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(mut input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    validate_list(&mut input)?;
    let slug = input.title.to_lowercase().replace(' ', "-");

    let list: ContactList = sqlx::query_as(
        r"
        INSERT INTO crm_lists (name, title, slug, description, is_public, contacts_count, created_at, updated_at)
        VALUES ($1, $1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, COALESCE(title, name) AS title, COALESCE(slug, '') AS slug, description,
                  COALESCE(is_public, false) AS is_public, (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count,
                  created_at, updated_at
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
        r"SELECT id, COALESCE(title, name) AS title,
                 COALESCE(slug, LOWER(REPLACE(name, ' ', '-'))) AS slug,
                 description, COALESCE(is_public, false) AS is_public,
                 (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count, created_at, updated_at
          FROM crm_lists WHERE id = $1",
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
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "List not found"})),
        )
    })?;

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
    let result = sqlx::query("DELETE FROM crm_lists WHERE id = $1")
        .bind(id)
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
            Json(json!({"error": "List not found"})),
        ));
    }

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
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let tags: Vec<ContactTag> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, name) AS title,
               COALESCE(slug, LOWER(REPLACE(name, ' ', '-'))) AS slug,
               description, color,
               COALESCE(contacts_count, 0) as contacts_count, created_at, updated_at
        FROM crm_tags
        WHERE ($1::text IS NULL OR COALESCE(title, name) ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
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
        INSERT INTO crm_tags (name, title, slug, description, color, contacts_count, created_at, updated_at)
        VALUES ($1, $1, $2, $3, COALESCE($4, '#6366f1'), 0, NOW(), NOW())
        RETURNING id, COALESCE(title, name) AS title, COALESCE(slug, '') AS slug, description, color,
                  COALESCE(contacts_count, 0) AS contacts_count, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") || e.to_string().contains("23505") {
            (StatusCode::CONFLICT, Json(json!({"error": "Tag with this name already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(tag))
}

async fn delete_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_tags WHERE id = $1")
        .bind(id)
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
            Json(json!({"error": "Tag not found"})),
        ));
    }

    Ok(Json(json!({"message": "Tag deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/lists", get(list_contact_lists).post(create_contact_list))
        .route(
            "/lists/{id}",
            get(get_contact_list)
                .put(update_contact_list)
                .delete(delete_contact_list),
        )
        .route(
            "/contact-tags",
            get(list_contact_tags).post(create_contact_tag),
        )
        .route("/contact-tags/{id}", delete(delete_contact_tag))
}
