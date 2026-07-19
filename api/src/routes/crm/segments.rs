//! CRM segments handlers
//! (split from `crm.rs` lines 115-127 types + 715-845 handlers
//! + 2086-2122 `duplicate_segment`).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
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
pub struct ContactSegment {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub match_type: String,
    pub contacts_count: i32,
    pub is_active: bool,
    pub last_synced_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateSegmentInput {
    pub title: String,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub match_type: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_segments(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let segments: Vec<ContactSegment> = sqlx::query_as(
        r"
        SELECT id, title, slug, description, conditions, match_type,
               COALESCE(contacts_count, 0) as contacts_count, is_active,
               last_synced_at, created_at, updated_at
        FROM crm_segments
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
        "data": segments,
        "meta": { "total": segments.len() }
    })))
}

async fn create_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSegmentInput>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");
    let match_type = input.match_type.unwrap_or_else(|| "all".to_string());

    let segment: ContactSegment = sqlx::query_as(
        r"
        INSERT INTO crm_segments (title, slug, description, conditions, match_type, contacts_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 0, true, NOW(), NOW())
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.conditions)
    .bind(&match_type)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
}

async fn get_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let segment: ContactSegment = sqlx::query_as(
        "SELECT id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at FROM crm_segments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))))?;

    Ok(Json(segment))
}

async fn delete_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_segments WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Segment deleted successfully"})))
}

async fn sync_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let segment: ContactSegment = sqlx::query_as(
        r"
        UPDATE crm_segments SET last_synced_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
}

/// POST /admin/crm/segments/:id/duplicate - Duplicate a segment
async fn duplicate_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    // Get original segment
    let original: ContactSegment = sqlx::query_as(
        "SELECT id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at FROM crm_segments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))))?;

    let new_title = format!("{} (Copy)", original.title);
    let new_slug = format!("{}-copy", original.slug);

    let segment: ContactSegment = sqlx::query_as(
        r"
        INSERT INTO crm_segments (title, slug, description, conditions, match_type, contacts_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 0, false, NOW(), NOW())
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        ",
    )
    .bind(&new_title)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.conditions)
    .bind(&original.match_type)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/segments", get(list_segments).post(create_segment))
        .route("/segments/{id}", get(get_segment).delete(delete_segment))
        .route("/segments/{id}/sync", post(sync_segment))
        .route("/segments/{id}/duplicate", post(duplicate_segment))
}
