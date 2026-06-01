//! CRM automation-funnel handlers
//! (split from `crm.rs` lines 186-198 types + 1091-1195 handlers).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
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
pub struct AutomationFunnel {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub trigger_type: String,
    pub trigger_name: Option<String>,
    pub actions_count: i32,
    pub subscribers_count: i32,
    pub conversion_rate: f64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateAutomationInput {
    pub title: String,
    pub trigger_type: String,
    pub trigger_name: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_automations(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let automations: Vec<AutomationFunnel> = sqlx::query_as(
        r"
        SELECT id, title, status, trigger_type, trigger_name, actions_count,
               subscribers_count, conversion_rate, created_at, updated_at
        FROM crm_automations
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
        "data": automations,
        "meta": { "total": automations.len() }
    })))
}

async fn create_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateAutomationInput>,
) -> Result<Json<AutomationFunnel>, (StatusCode, Json<serde_json::Value>)> {
    let automation: AutomationFunnel = sqlx::query_as(
        r"
        INSERT INTO crm_automations (title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at)
        VALUES ($1, 'draft', $2, $3, 0, 0, 0.0, NOW(), NOW())
        RETURNING id, title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&input.trigger_type)
    .bind(&input.trigger_name)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(automation))
}

async fn get_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<AutomationFunnel>, (StatusCode, Json<serde_json::Value>)> {
    let automation: AutomationFunnel = sqlx::query_as(
        "SELECT id, title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at FROM crm_automations WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Automation not found"}))))?;

    Ok(Json(automation))
}

async fn delete_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_automations WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Automation deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/automations",
            get(list_automations).post(create_automation),
        )
        .route(
            "/automations/:id",
            get(get_automation).delete(delete_automation),
        )
}
