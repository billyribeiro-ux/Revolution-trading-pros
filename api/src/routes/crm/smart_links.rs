//! CRM smart-link handlers
//! (split from `crm.rs` lines 200-212 types + 1196-1286 handlers).

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
pub struct SmartLink {
    pub id: i64,
    pub title: String,
    pub short: String,
    pub target_url: Option<String>,
    pub is_active: bool,
    pub click_count: i32,
    pub unique_clicks: i32,
    pub actions: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateSmartLinkInput {
    pub title: String,
    pub target_url: String,
    pub actions: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_smart_links(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let links: Vec<SmartLink> = sqlx::query_as(
        r"
        SELECT id, title, short, target_url, is_active, click_count, unique_clicks,
               actions, created_at, updated_at
        FROM crm_smart_links
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
        "data": links,
        "meta": { "total": links.len() }
    })))
}

async fn create_smart_link(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSmartLinkInput>,
) -> Result<Json<SmartLink>, (StatusCode, Json<serde_json::Value>)> {
    let short = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let link: SmartLink = sqlx::query_as(
        r"
        INSERT INTO crm_smart_links (title, short, target_url, is_active, click_count, unique_clicks, actions, created_at, updated_at)
        VALUES ($1, $2, $3, true, 0, 0, $4, NOW(), NOW())
        RETURNING id, title, short, target_url, is_active, click_count, unique_clicks, actions, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&short)
    .bind(&input.target_url)
    .bind(&input.actions)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(link))
}

async fn delete_smart_link(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_smart_links WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Smart link deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/smart-links",
            get(list_smart_links).post(create_smart_link),
        )
        .route("/smart-links/:id", delete(delete_smart_link))
}
