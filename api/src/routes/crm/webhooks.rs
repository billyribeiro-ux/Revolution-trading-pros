//! CRM webhook handlers
//! (split from `crm.rs` lines 231-243 types + 1393-1491 handlers).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
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
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub events: Option<serde_json::Value>,
    pub is_active: bool,
    pub trigger_count: i32,
    pub failure_count: i32,
    pub last_triggered_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookInput {
    pub name: String,
    pub url: String,
    pub events: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_webhooks(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let webhooks: Vec<Webhook> = sqlx::query_as(
        r"
        SELECT id, name, url, events, is_active, trigger_count, failure_count,
               last_triggered_at, created_at, updated_at
        FROM crm_webhooks
        WHERE ($1::text IS NULL OR name ILIKE $1)
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
        "data": webhooks,
        "meta": { "total": webhooks.len() }
    })))
}

async fn create_webhook(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateWebhookInput>,
) -> Result<Json<Webhook>, (StatusCode, Json<serde_json::Value>)> {
    let webhook: Webhook = sqlx::query_as(
        r"
        INSERT INTO crm_webhooks (name, url, events, is_active, trigger_count, failure_count, created_at, updated_at)
        VALUES ($1, $2, $3, true, 0, 0, NOW(), NOW())
        RETURNING id, name, url, events, is_active, trigger_count, failure_count, last_triggered_at, created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&input.url)
    .bind(&input.events)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(webhook))
}

async fn delete_webhook(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_webhooks WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Webhook deleted successfully"})))
}

async fn test_webhook(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Mock test - actual implementation would send test payload
    Ok(Json(
        json!({"success": true, "message": "Test webhook sent", "webhook_id": id}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/{id}", delete(delete_webhook))
        .route("/webhooks/{id}/test", post(test_webhook))
}
