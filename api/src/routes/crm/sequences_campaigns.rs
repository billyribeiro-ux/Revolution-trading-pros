//! CRM email-sequences, campaigns, recurring-campaigns handlers
//! (split from `crm.rs` lines 133-180 types + 851-1090 handlers
//! + 2124-2157 `duplicate_campaign`).

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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: i64,
    pub title: String,
    pub subject: Option<String>,
    pub status: String,
    pub scheduled_at: Option<chrono::NaiveDateTime>,
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub recipients_count: i32,
    pub emails_sent: i32,
    pub opens: i32,
    pub clicks: i32,
    pub open_rate: f64,
    pub click_rate: f64,
    pub template_id: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringCampaign {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub scheduling_settings: Option<serde_json::Value>,
    pub total_campaigns_sent: i32,
    pub total_emails_sent: i32,
    pub total_revenue: f64,
    pub last_sent_at: Option<chrono::NaiveDateTime>,
    pub next_scheduled_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let sequences: Vec<EmailSequence> = sqlx::query_as(
        r"
        SELECT id, title, status, trigger_type, email_count, total_subscribers,
               emails_sent, open_rate, click_rate, created_at, updated_at
        FROM crm_sequences
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
        INSERT INTO crm_sequences (title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, $3, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at
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
        "SELECT id, title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at FROM crm_sequences WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Sequence not found"}))))?;

    Ok(Json(sequence))
}

async fn delete_sequence(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_sequences WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Sequence deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Campaigns
// ═══════════════════════════════════════════════════════════════════════════

async fn list_campaigns(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let campaigns: Vec<Campaign> = sqlx::query_as(
        r"
        SELECT id, title, subject, status, scheduled_at, sent_at, recipients_count,
               emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        FROM crm_campaigns
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
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": campaigns,
        "meta": { "total": campaigns.len() }
    })))
}

async fn create_campaign(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateCampaignInput>,
) -> Result<Json<Campaign>, (StatusCode, Json<serde_json::Value>)> {
    let campaign: Campaign = sqlx::query_as(
        r"
        INSERT INTO crm_campaigns (title, subject, status, template_id, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, 'draft', $3, 0, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&input.subject)
    .bind(input.template_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(campaign))
}

async fn get_campaign(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Campaign>, (StatusCode, Json<serde_json::Value>)> {
    let campaign: Campaign = sqlx::query_as(
        "SELECT id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at FROM crm_campaigns WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Campaign not found"}))))?;

    Ok(Json(campaign))
}

async fn delete_campaign(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_campaigns WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Campaign deleted successfully"})))
}

/// POST /admin/crm/campaigns/:id/duplicate - Duplicate a campaign
async fn duplicate_campaign(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Campaign>, (StatusCode, Json<serde_json::Value>)> {
    // Get original campaign
    let original: Campaign = sqlx::query_as(
        "SELECT id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at FROM crm_campaigns WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Campaign not found"}))))?;

    let new_title = format!("{} (Copy)", original.title);

    let campaign: Campaign = sqlx::query_as(
        r"
        INSERT INTO crm_campaigns (title, subject, status, template_id, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, 'draft', $3, 0, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        ",
    )
    .bind(&new_title)
    .bind(&original.subject)
    .bind(original.template_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(campaign))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Recurring Campaigns
// ═══════════════════════════════════════════════════════════════════════════

async fn list_recurring_campaigns(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let campaigns: Vec<RecurringCampaign> = sqlx::query_as(
        r"
        SELECT id, title, status, scheduling_settings, total_campaigns_sent, total_emails_sent,
               total_revenue, last_sent_at, next_scheduled_at, created_at, updated_at
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
    // FIX-2026-04-26: .unwrap_or_default();
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
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/campaigns/{id}", get(get_campaign).delete(delete_campaign))
        .route("/campaigns/{id}/duplicate", post(duplicate_campaign))
        .route("/recurring-campaigns", get(list_recurring_campaigns))
}
