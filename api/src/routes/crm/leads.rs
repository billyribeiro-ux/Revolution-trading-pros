//! CRM lead handlers (contacts with lead status) and lead-bulk + abandoned-cart-bulk ops
//! (split from `crm.rs` lines 260-272 AbandonedCart type
//! + 1714-2085 lead handlers + 3273-3386 bulk handlers).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::FromRow;

use super::contacts::{CreateContactInput, UpdateContactInput};
use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    utils::errors::{self, ApiError},
};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmLead {
    pub id: i64,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub score: Option<i32>,
    pub tags: Option<serde_json::Value>,
    pub custom_fields: Option<serde_json::Value>,
    pub last_contacted_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeadFilters {
    pub lifecycle_stage: Option<String>,
    pub owner_id: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AbandonedCart {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub email: String,
    pub cart_total_cents: i64,
    pub cart_items: Option<serde_json::Value>,
    pub status: String,
    pub recovery_email_sent: bool,
    pub recovered: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteInput {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BulkStatusInput {
    pub ids: Vec<i64>,
    pub status: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Leads
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/leads - List all leads
fn lead_status(status: &str) -> bool {
    matches!(
        status,
        "new" | "contacted" | "qualified" | "proposal" | "negotiation" | "won" | "lost" | "lead"
    )
}
fn lead_view(mut row: Value) -> Value {
    if let Some(id) = row["id"].as_i64() {
        row["id"] = json!(id.to_string());
    }
    row["company_name"] = row["company"].clone();
    row["is_hot"] = row["lead_score"]
        .as_i64()
        .map_or(Value::Null, |score| json!(score >= 75));
    row["is_starred"] = Value::Null;
    row["estimated_value"] = Value::Null;
    row
}
type LeadResponse = Result<Json<Value>, ApiError>;
async fn list_leads(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(filters): Query<LeadFilters>,
) -> LeadResponse {
    if filters
        .status
        .as_deref()
        .is_some_and(|status| !lead_status(status))
    {
        return Err(ApiError::validation_error("Choose a supported lead status"));
    }
    let filters = super::contacts::ContactFilters {
        page: filters.page,
        per_page: filters.per_page,
        status: filters.status,
        search: filters.search,
        source: filters.source,
        lifecycle_stage: filters.lifecycle_stage,
        owner_id: filters.owner_id,
        sort_by: filters.sort_by,
        sort_order: filters.sort_order,
        lead_only: true,
        ..Default::default()
    };
    let Json(mut response) =
        super::contacts::list_contacts(State(state), admin, Query(filters)).await?;
    if let Some(rows) = response["data"].as_array_mut() {
        for row in rows {
            *row = lead_view(row.take());
        }
    }
    Ok(Json(response))
}
async fn get_lead_stats(State(state): State<AppState>, _: AdminUser) -> LeadResponse {
    let value:Value=sqlx::query_scalar("WITH leads AS (SELECT c.*,CASE WHEN c.score_assessment->>'status'='calculated' AND c.score_assessed_at IS NOT NULL AND c.score_assessment->'policy'=(SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings') AND c.score_assessment->'score'=to_jsonb(c.score) THEN c.score ELSE NULL END AS measured_score FROM contacts c WHERE c.status IN ('new','contacted','qualified','proposal','negotiation','won','lost','lead')) SELECT jsonb_build_object('total',COUNT(*),'new',COUNT(*) FILTER(WHERE status IN ('new','lead')),'contacted',COUNT(*) FILTER(WHERE status='contacted'),'qualified',COUNT(*) FILTER(WHERE status='qualified'),'proposal',COUNT(*) FILTER(WHERE status='proposal'),'negotiation',COUNT(*) FILTER(WHERE status='negotiation'),'won',COUNT(*) FILTER(WHERE status='won'),'lost',COUNT(*) FILTER(WHERE status='lost'),'hot_leads',COUNT(*) FILTER(WHERE status NOT IN ('won','lost') AND measured_score>=75),'measured_score_leads',COUNT(measured_score),'unknown_score_leads',COUNT(*) FILTER(WHERE measured_score IS NULL),'total_value',NULL,'conversion_rate',ROUND(100.0*COUNT(*) FILTER(WHERE status='won')/NULLIF(COUNT(*) FILTER(WHERE status IN ('won','lost')),0),2),'metrics_scope','recorded_lead_status_and_current_assessments') FROM leads")
        .fetch_one(&state.db.pool).await.map_err(|error|{tracing::error!(%error,"CRM lead stats unavailable");errors::service_unavailable("Lead statistics are unavailable")})?;
    Ok(Json(value))
}
async fn create_lead(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(mut input): Json<CreateContactInput>,
) -> LeadResponse {
    input.validate(true).map_err(|(status, Json(value))| {
        ApiError::new(
            status,
            value["error"].as_str().unwrap_or("Invalid lead fields"),
        )
    })?;
    if input.status.is_none() {
        input.status = Some("new".into());
    }
    let Json(row) = super::contacts::create_contact(State(state), admin, Json(input)).await?;
    Ok(Json(lead_view(row)))
}
async fn get_lead(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> LeadResponse {
    let Json(row) = super::contacts::get_contact(State(state), admin, Path(id)).await?;
    if row["status"]
        .as_str()
        .is_none_or(|status| !lead_status(status))
    {
        return Err(errors::not_found("Lead"));
    }
    Ok(Json(lead_view(row)))
}
async fn update_lead(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactInput>,
) -> LeadResponse {
    input.validate(true).map_err(|(status, Json(value))| {
        ApiError::new(
            status,
            value["error"].as_str().unwrap_or("Invalid lead fields"),
        )
    })?;
    let Json(row) =
        super::contacts::update_contact(State(state), admin, Path(id), Json(input)).await?;
    Ok(Json(lead_view(row)))
}
async fn delete_lead(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> LeadResponse {
    super::contacts::delete_contact(State(state), admin, Path(id)).await
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Bulk Operations
// ═══════════════════════════════════════════════════════════════════════════

/// POST /admin/crm/leads/bulk-delete - Bulk delete leads
async fn bulk_delete_leads(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<BulkDeleteInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No IDs provided"})),
        ));
    }

    let deleted = sqlx::query("DELETE FROM contacts WHERE id = ANY($1) AND status = 'lead'")
        .bind(&input.ids)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "message": "Leads deleted successfully",
        "deleted_count": deleted.rows_affected()
    })))
}

/// POST /admin/crm/leads/bulk-status - Bulk update lead status
async fn bulk_update_lead_status(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<BulkStatusInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No IDs provided"})),
        ));
    }

    let valid_statuses = ["new", "contacted", "qualified", "unqualified", "nurturing"];
    if !valid_statuses.contains(&input.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid status"})),
        ));
    }

    let updated = sqlx::query(
        "UPDATE contacts SET status = $1, updated_at = NOW() AT TIME ZONE 'UTC' WHERE id = ANY($2)",
    )
    .bind(&input.status)
    .bind(&input.ids)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "message": "Lead statuses updated successfully",
        "updated_count": updated.rows_affected()
    })))
}

/// POST /admin/crm/abandoned-carts/bulk-delete - Bulk delete abandoned carts
async fn bulk_delete_abandoned_carts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<BulkDeleteInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No IDs provided"})),
        ));
    }

    let deleted = sqlx::query("DELETE FROM crm_abandoned_carts WHERE id = ANY($1)")
        .bind(&input.ids)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "message": "Abandoned carts deleted successfully",
        "deleted_count": deleted.rows_affected()
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/leads", get(list_leads).post(create_lead))
        .route("/leads/stats", get(get_lead_stats))
        .route(
            "/leads/{id}",
            get(get_lead).put(update_lead).delete(delete_lead),
        )
        // Bulk operations
        .route("/leads/bulk-delete", post(bulk_delete_leads))
        .route("/leads/bulk-status", post(bulk_update_lead_status))
        .route(
            "/abandoned-carts/bulk-delete",
            post(bulk_delete_abandoned_carts),
        )
}
