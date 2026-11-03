//! CRM email-template handlers
//! (split from `crm.rs` lines 214-225 types + 1287-1392 handlers).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
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
pub struct EmailTemplate {
    pub id: i64,
    pub title: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
    pub thumbnail: Option<String>,
    pub is_global: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateInput {
    pub title: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_templates(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let templates: Vec<EmailTemplate> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, name) AS title, subject,
               COALESCE(body, content) AS body, category, thumbnail,
               COALESCE(is_global, false) AS is_global, created_at, updated_at
        FROM crm_templates
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
        "data": templates,
        "meta": { "total": templates.len() }
    })))
}

async fn create_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTemplateInput>,
) -> Result<Json<EmailTemplate>, (StatusCode, Json<serde_json::Value>)> {
    let template: EmailTemplate = sqlx::query_as(
        r"
        INSERT INTO crm_templates (name, title, subject, content, body, category, is_global, created_at, updated_at)
        VALUES ($1, $1, $2, COALESCE($3, ''), $3, $4, false, NOW(), NOW())
        RETURNING id, COALESCE(title, name) AS title, subject, COALESCE(body, content) AS body,
                  category, thumbnail, COALESCE(is_global, false) AS is_global, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&input.subject)
    .bind(&input.body)
    .bind(&input.category)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(template))
}

async fn get_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<EmailTemplate>, (StatusCode, Json<serde_json::Value>)> {
    let template: EmailTemplate = sqlx::query_as(
        r"SELECT id, COALESCE(title, name) AS title, subject,
                 COALESCE(body, content) AS body, category, thumbnail,
                 COALESCE(is_global, false) AS is_global, created_at, updated_at
          FROM crm_templates WHERE id = $1",
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
            Json(json!({"error": "Template not found"})),
        )
    })?;

    Ok(Json(template))
}

async fn delete_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_templates WHERE id = $1")
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
            Json(json!({"error": "Template not found"})),
        ));
    }

    Ok(Json(json!({"message": "Template deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/templates", get(list_templates).post(create_template))
        .route("/templates/{id}", get(get_template).delete(delete_template))
}
