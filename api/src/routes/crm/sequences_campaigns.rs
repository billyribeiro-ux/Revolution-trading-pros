//! Recurring-campaign catalog. Sequence authoring lives in sequences.rs.
use super::ListFilters;
use crate::{AppState, middleware::admin::AdminUser};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

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

pub(super) fn router() -> Router<AppState> {
    Router::new().route("/recurring-campaigns", get(list_recurring_campaigns))
}
