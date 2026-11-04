//! CRM email-sequences, campaigns, recurring-campaigns handlers
//! (split from `crm.rs` lines 133-180 types + 851-1090 handlers
//! + 2124-2157 `duplicate_campaign`).

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
pub struct EmailSequence {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub trigger_type: Option<String>,
    pub email_count: i32,
    pub total_subscribers: i32,
    pub emails_sent: i32,
    pub open_rate: f64,
    pub click_rate: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: i64,
    pub title: String,
    pub subject: Option<String>,
    pub status: String,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub recipients_count: i32,
    pub emails_sent: i32,
    pub opens: i32,
    pub clicks: i32,
    pub open_rate: f64,
    pub click_rate: f64,
    pub template_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringCampaign {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub scheduling_settings: Option<serde_json::Value>,
    pub total_campaigns_sent: i32,
    pub total_emails_sent: i32,
    pub total_revenue_cents: i64,
    pub last_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub next_scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSequenceInput {
    pub title: String,
    pub trigger_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignInput {
    pub title: String,
    pub subject: Option<String>,
    pub template_id: Option<i64>,
    pub scheduled_at: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Sequences
// ═══════════════════════════════════════════════════════════════════════════

async fn list_sequences(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let sequences: Vec<EmailSequence> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, name) AS title, COALESCE(status, 'draft') AS status,
               trigger_type, COALESCE(email_count, 0) AS email_count,
               COALESCE(total_subscribers, 0) AS total_subscribers,
               COALESCE(emails_sent, 0) AS emails_sent,
               COALESCE(open_rate, 0)::float8 AS open_rate,
               COALESCE(click_rate, 0)::float8 AS click_rate,
               created_at, updated_at
        FROM crm_sequences
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
        "data": sequences,
        "meta": { "total": sequences.len() }
    })))
}

async fn create_sequence(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSequenceInput>,
) -> Result<Json<EmailSequence>, (StatusCode, Json<serde_json::Value>)> {
    let status = input.status.unwrap_or_else(|| "draft".to_string());

    let sequence: EmailSequence = sqlx::query_as(
        r"
        INSERT INTO crm_sequences (name, title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $1, $2, $3, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, COALESCE(title, name) AS title, COALESCE(status, 'draft') AS status,
                  trigger_type, COALESCE(email_count, 0) AS email_count,
                  COALESCE(total_subscribers, 0) AS total_subscribers,
                  COALESCE(emails_sent, 0) AS emails_sent,
                  COALESCE(open_rate, 0)::float8 AS open_rate,
                  COALESCE(click_rate, 0)::float8 AS click_rate,
                  created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&status)
    .bind(&input.trigger_type)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(sequence))
}

async fn get_sequence(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<EmailSequence>, (StatusCode, Json<serde_json::Value>)> {
    let sequence: EmailSequence = sqlx::query_as(
        r"SELECT id, COALESCE(title, name) AS title, COALESCE(status, 'draft') AS status,
                 trigger_type, COALESCE(email_count, 0) AS email_count,
                 COALESCE(total_subscribers, 0) AS total_subscribers,
                 COALESCE(emails_sent, 0) AS emails_sent,
                 COALESCE(open_rate, 0)::float8 AS open_rate,
                 COALESCE(click_rate, 0)::float8 AS click_rate,
                 created_at, updated_at
          FROM crm_sequences WHERE id = $1",
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
            Json(json!({"error": "Sequence not found"})),
        )
    })?;

    Ok(Json(sequence))
}

async fn delete_sequence(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_sequences WHERE id = $1")
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
            Json(json!({"error": "Sequence not found"})),
        ));
    }

    Ok(Json(json!({"message": "Sequence deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Campaigns
// ═══════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Recurring Campaigns
// ═══════════════════════════════════════════════════════════════════════════

async fn list_recurring_campaigns(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let campaigns: Vec<RecurringCampaign> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, '') AS title, COALESCE(status, 'active') AS status,
               COALESCE(scheduling_settings, schedule) AS scheduling_settings,
               COALESCE(total_campaigns_sent, 0) AS total_campaigns_sent,
               COALESCE(total_emails_sent, 0) AS total_emails_sent,
               ROUND(COALESCE(total_revenue, 0) * 100)::BIGINT AS total_revenue_cents,
               last_sent_at, COALESCE(next_scheduled_at, next_run_at) AS next_scheduled_at,
               created_at, updated_at
        FROM crm_recurring_campaigns
        WHERE ($1::text IS NULL OR title ILIKE $1)
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
        "data": campaigns,
        "meta": { "total": campaigns.len() }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/sequences", get(list_sequences).post(create_sequence))
        .route("/sequences/{id}", get(get_sequence).delete(delete_sequence))
        .route("/recurring-campaigns", get(list_recurring_campaigns))
}
