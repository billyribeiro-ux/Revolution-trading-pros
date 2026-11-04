//! CRM deals + pipelines handlers
//! (split from `crm.rs` lines 2231-3270 — types, pipeline handlers,
//! deal handlers, deal forecast).
//!
//! Mounted pipeline and deal endpoints with shared transactional mutations.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{AppState, middleware::admin::AdminUser};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmPipeline {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmPipelineStage {
    pub id: i64,
    pub pipeline_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub probability: i32,
    pub is_closed_won: bool,
    pub is_closed_lost: bool,
    pub auto_advance_after_days: Option<i32>,
    pub color: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmDeal {
    pub id: i64,
    pub name: String,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub pipeline_id: i64,
    pub stage_id: i64,
    pub owner_id: Option<i64>,
    pub amount: sqlx::types::Decimal,
    pub currency: String,
    pub probability: i32,
    pub status: String,
    pub expected_close_date: Option<chrono::NaiveDate>,
    pub close_date: Option<chrono::NaiveDate>,
    pub lost_reason: Option<String>,
    pub won_details: Option<String>,
    pub priority: String,
    pub source_channel: Option<String>,
    pub source_campaign: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub custom_fields: Option<serde_json::Value>,
    pub stage_entered_at: chrono::NaiveDateTime,
    pub closed_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct DealFilters {
    pub currency: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub pipeline_id: Option<i64>,
    pub stage_id: Option<i64>,
    pub status: Option<String>,
    pub owner_id: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDealInput {
    pub name: String,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
    pub owner_id: Option<i64>,
    pub pipeline_id: i64,
    pub stage_id: i64,
    // Batch 5c: integer-cents convention (matches Batch 1 unification).
    // DB column `crm_deals.amount` stays NUMERIC(15,2); we convert at
    // the SQL boundary as `cents::BIGINT / 100.0`.
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
    pub probability: Option<i32>,
    pub expected_close_date: Option<String>,
    pub priority: Option<String>,
    pub source_channel: Option<String>,
    pub source_campaign: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDealInput {
    pub name: Option<String>,
    // Batch 5c: integer-cents (see CreateDealInput).
    pub amount_cents: Option<i64>,
    pub probability: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub expected_close_date: Option<Option<String>>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
    pub pipeline_id: Option<i64>,
    pub stage_id: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub contact_id: Option<Option<i64>>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub company_id: Option<Option<i64>>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub owner_id: Option<Option<i64>>,
    pub currency: Option<String>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub source_channel: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_patch")]
    pub source_campaign: Option<Option<String>>,
    pub lost_reason: Option<String>,
    pub stage_change_reason: Option<String>,
}

fn deserialize_patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDealStageInput {
    pub stage_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WinDealInput {
    pub won_details: Option<String>,
    pub close_date: Option<String>,
    pub stage_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoseDealInput {
    pub lost_reason: String,
    pub stage_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePipelineInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStageInput {
    pub name: String,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub probability: Option<i32>,
    pub is_closed_won: Option<bool>,
    pub is_closed_lost: Option<bool>,
    pub color: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Pipelines (defined but not routed in the pre-split source)
// ═══════════════════════════════════════════════════════════════════════════

pub(super) type CrmError = (StatusCode, Json<serde_json::Value>);
pub(super) fn crm_db_error(error: sqlx::Error) -> CrmError {
    tracing::error!(%error,"Deal persistence query failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error":"Unable to access deal data"})),
    )
}
fn decimal_to_cents(val: sqlx::types::Decimal) -> Result<i64, CrmError> {
    (val * sqlx::types::Decimal::from(100))
        .round()
        .to_string()
        .parse::<i64>()
        .map_err(|error| {
            tracing::error!(%error,"Invalid stored deal amount");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":"Stored deal amount cannot be represented in cents"})),
            )
        })
}
#[derive(FromRow)]
pub(super) struct DealRow {
    #[sqlx(flatten)]
    pub(super) deal: CrmDeal,
    stage_changes_count: i64,
    contact: Option<serde_json::Value>,
    company: Option<serde_json::Value>,
    owner: Option<serde_json::Value>,
}
impl std::ops::Deref for DealRow {
    type Target = CrmDeal;
    fn deref(&self) -> &CrmDeal {
        &self.deal
    }
}
pub(super) fn deal_json(d: &DealRow) -> Result<serde_json::Value, CrmError> {
    let cents = decimal_to_cents(d.amount)?;
    let weighted = cents
        .checked_mul(i64::from(d.probability))
        .and_then(|v| v.checked_add(if cents < 0 { -50 } else { 50 }))
        .map(|v| v / 100)
        .ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":"Weighted deal amount exceeds supported range"})),
            )
        })?;
    let now = chrono::Utc::now().naive_utc();
    Ok(json!({
        "id":d.id.to_string(),"name":d.name,"contact_id":d.contact_id.map(|id|id.to_string()),"company_id":d.company_id.map(|id|id.to_string()),"pipeline_id":d.pipeline_id.to_string(),"stage_id":d.stage_id.to_string(),"owner_id":d.owner_id.map(|id|id.to_string()),
        "amount_cents":cents,"amount":cents as f64/100.0,"weighted_value":weighted as f64/100.0,"currency":d.currency,"probability":d.probability,"status":d.status,"expected_close_date":d.expected_close_date,"close_date":d.close_date,"lost_reason":d.lost_reason,"won_details":d.won_details,"priority":d.priority,"source_channel":d.source_channel,"source_campaign":d.source_campaign,"tags":d.tags,"custom_fields":d.custom_fields,
        "days_in_stage":now.signed_duration_since(d.stage_entered_at).num_days().max(0),"days_in_pipeline":now.signed_duration_since(d.created_at).num_days().max(0),"stage_changes_count":d.stage_changes_count,
        "stage_entered_at":d.stage_entered_at.and_utc(),"closed_at":d.closed_at.map(|time|time.and_utc()),"created_at":d.created_at.and_utc(),"updated_at":d.updated_at.and_utc(),"contact":d.contact,"company":d.company,"owner":d.owner
    }))
}

macro_rules! crm_pipeline_cols {
    () => {
        "id, name, description, is_default, is_active, color, icon, position, \
         (created_at AT TIME ZONE 'UTC') AS created_at, \
         (updated_at AT TIME ZONE 'UTC') AS updated_at"
    };
}

macro_rules! crm_stage_cols {
    () => {
        "id, pipeline_id, name, description, position, probability, \
         is_closed_won, is_closed_lost, auto_advance_after_days, color, \
         (created_at AT TIME ZONE 'UTC') AS created_at, \
         (updated_at AT TIME ZONE 'UTC') AS updated_at"
    };
}

macro_rules! crm_deal_cols {
    () => {
        "id, name, contact_id, company_id, pipeline_id, stage_id, owner_id, \
         amount, currency, probability, status, expected_close_date, close_date, \
         lost_reason, won_details, priority, source_channel, source_campaign, \
         tags, custom_fields, \
         (stage_entered_at AT TIME ZONE 'UTC') AS stage_entered_at, \
         (closed_at AT TIME ZONE 'UTC') AS closed_at, \
         (created_at AT TIME ZONE 'UTC') AS created_at, \
         (updated_at AT TIME ZONE 'UTC') AS updated_at, \
         (SELECT COUNT(*) FROM crm_deal_stage_history h WHERE h.deal_id=crm_deals.id) AS stage_changes_count, \
         (SELECT jsonb_build_object('id',c.id::text,'first_name',c.first_name,'last_name',c.last_name,'full_name',COALESCE(NULLIF(trim(concat_ws(' ',c.first_name,c.last_name)),''),c.email),'email',c.email,'phone',c.phone) FROM contacts c WHERE c.id=crm_deals.contact_id) AS contact, \
         (SELECT jsonb_build_object('id',c.id::text,'name',c.name) FROM crm_companies c WHERE c.id=crm_deals.company_id) AS company, \
         (SELECT jsonb_build_object('id',u.id::text,'name',u.name,'email',u.email) FROM users u WHERE u.id=crm_deals.owner_id) AS owner"
    };
}

pub(super) const DEAL_COLS: &str = crm_deal_cols!();

fn validate_pipeline(input: &CreatePipelineInput) -> Result<(), CrmError> {
    super::deal_mutations::text(&input.name, "Pipeline name", 255, true)?;
    if let Some(v) = &input.description {
        super::deal_mutations::text(v, "Description", 5000, false)?;
    }
    if let Some(v) = &input.icon {
        super::deal_mutations::text(v, "Icon", 100, false)?;
    }
    validate_color(input.color.as_deref())
}
fn validate_color(value: Option<&str>) -> Result<(), CrmError> {
    if value.is_some_and(|v| {
        v.len() != 7 || !v.starts_with('#') || !v[1..].bytes().all(|b| b.is_ascii_hexdigit())
    }) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Color must be a six-digit hex color"})),
        ));
    }
    Ok(())
}

/// GET /admin/crm/pipelines - List all pipelines
async fn list_pipelines(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let pipelines: Vec<CrmPipeline> = sqlx::query_as(concat!(
        "SELECT ",
        crm_pipeline_cols!(),
        " FROM crm_pipelines WHERE is_active = true ORDER BY position ASC, created_at ASC"
    ))
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // For each pipeline, get its stages and stats
    let mut result = Vec::new();
    for pipeline in pipelines {
        let stages: Vec<CrmPipelineStage> = sqlx::query_as(concat!(
            "SELECT ",
            crm_stage_cols!(),
            " FROM crm_pipeline_stages WHERE pipeline_id = $1 ORDER BY position ASC"
        ))
        .bind(pipeline.id)
        .fetch_all(&state.db.pool)
        .await
        // FIX-2026-04-26: .unwrap_or_default();
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        let deals_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM crm_deals WHERE pipeline_id = $1")
                .bind(pipeline.id)
                .fetch_one(&state.db.pool)
                .await
                .map_err(crm_db_error)?;

        let total_value_cents: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(ROUND(amount * 100)::BIGINT), 0)::BIGINT FROM crm_deals WHERE pipeline_id = $1 AND status = 'open' AND currency='USD'",
        )
        .bind(pipeline.id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(crm_db_error)?;

        let currency_totals:Vec<serde_json::Value>=sqlx::query_scalar("SELECT jsonb_build_object('currency',currency,'amount_cents',ROUND(SUM(amount)*100)::bigint,'amount',SUM(amount)) FROM crm_deals WHERE pipeline_id=$1 AND status='open' GROUP BY currency ORDER BY currency").bind(pipeline.id).fetch_all(&state.db.pool).await.map_err(crm_db_error)?;
        result.push(json!({
            "id": pipeline.id.to_string(),
            "name": pipeline.name,
            "description": pipeline.description,
            "is_default": pipeline.is_default,
            "is_active": pipeline.is_active,
            "color": pipeline.color.unwrap_or_else(|| "#6366f1".to_string()),
            "icon": pipeline.icon,
            "position": pipeline.position,
            "deals_count": deals_count.0,
            "total_value_cents": total_value_cents.0,
            "total_value": total_value_cents.0 as f64/100.0,
            "currency":"USD",
            "totals_by_currency":currency_totals,
            "stages": stages.iter().map(|s| json!({
                "id": s.id.to_string(),
                "pipeline_id": s.pipeline_id.to_string(),
                "name": s.name,
                "description": s.description,
                "position": s.position,
                "probability": s.probability,
                "is_closed_won": s.is_closed_won,
                "is_closed_lost": s.is_closed_lost,
                "color": s.color.clone().unwrap_or_else(|| "#6366f1".to_string()),
                "created_at": s.created_at,
                "updated_at": s.updated_at
            })).collect::<Vec<_>>(),
            "created_at": pipeline.created_at,
            "updated_at": pipeline.updated_at
        }));
    }

    Ok(Json(json!(result)))
}

/// GET /admin/crm/pipelines/:id - Get single pipeline with stages
async fn get_pipeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let pipeline: CrmPipeline = sqlx::query_as(concat!(
        "SELECT ",
        crm_pipeline_cols!(),
        " FROM crm_pipelines WHERE id = $1"
    ))
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
            Json(json!({"error": "Pipeline not found"})),
        )
    })?;

    let stages: Vec<CrmPipelineStage> = sqlx::query_as(concat!(
        "SELECT ",
        crm_stage_cols!(),
        " FROM crm_pipeline_stages WHERE pipeline_id = $1 ORDER BY position ASC"
    ))
    .bind(id)
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
        "id": pipeline.id.to_string(),
        "name": pipeline.name,
        "description": pipeline.description,
        "is_default": pipeline.is_default,
        "is_active": pipeline.is_active,
        "color": pipeline.color.unwrap_or_else(|| "#6366f1".to_string()),
        "icon": pipeline.icon,
        "position": pipeline.position,
        "stages": stages.iter().map(|s| json!({
            "id": s.id.to_string(),
            "pipeline_id": s.pipeline_id.to_string(),
            "name": s.name,
            "description": s.description,
            "position": s.position,
            "probability": s.probability,
            "is_closed_won": s.is_closed_won,
            "is_closed_lost": s.is_closed_lost,
            "color": s.color.clone().unwrap_or_else(|| "#6366f1".to_string())
        })).collect::<Vec<_>>(),
        "created_at": pipeline.created_at,
        "updated_at": pipeline.updated_at
    })))
}

/// POST /admin/crm/pipelines - Create a new pipeline
async fn create_pipeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreatePipelineInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    validate_pipeline(&input)?;
    let pipeline: CrmPipeline = sqlx::query_as(concat!(
        "INSERT INTO crm_pipelines (name, description, is_default, is_active, color, icon, position, created_at, updated_at)
         VALUES ($1, $2, false, true, $3, $4, (SELECT COALESCE(MAX(position), 0) + 1 FROM crm_pipelines), NOW(), NOW())
         RETURNING ",
        crm_pipeline_cols!()
    ))
    .bind(input.name.trim())
    .bind(&input.description)
    .bind(input.color.unwrap_or_else(|| "#6366f1".to_string()))
    .bind(&input.icon)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "id": pipeline.id.to_string(),
        "name": pipeline.name,
        "description": pipeline.description,
        "is_default": pipeline.is_default,
        "is_active": pipeline.is_active,
        "color": pipeline.color,
        "stages": []
    })))
}

/// PUT /admin/crm/pipelines/:id - Update a pipeline
async fn update_pipeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreatePipelineInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    validate_pipeline(&input)?;
    let pipeline: CrmPipeline = sqlx::query_as(concat!(
        "UPDATE crm_pipelines SET
            name = $2,
            description = $3,
            color = COALESCE($4, color),
            icon = COALESCE($5, icon),
            updated_at = NOW()
        WHERE id = $1
        RETURNING ",
        crm_pipeline_cols!()
    ))
    .bind(id)
    .bind(input.name.trim())
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
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
            Json(json!({"error": "Pipeline not found"})),
        )
    })?;

    Ok(Json(json!({
        "id": pipeline.id.to_string(),
        "name": pipeline.name,
        "description": pipeline.description,
        "is_default": pipeline.is_default,
        "is_active": pipeline.is_active,
        "color": pipeline.color
    })))
}

/// DELETE /admin/crm/pipelines/:id - Delete a pipeline
async fn delete_pipeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    // Check if it's the default pipeline
    let pipeline: CrmPipeline = sqlx::query_as(concat!(
        "SELECT ",
        crm_pipeline_cols!(),
        " FROM crm_pipelines WHERE id = $1 FOR UPDATE"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
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
            Json(json!({"error": "Pipeline not found"})),
        )
    })?;

    if pipeline.is_default {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Cannot delete default pipeline"})),
        ));
    }

    let occupied: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_deals WHERE pipeline_id=$1)")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(crm_db_error)?;
    if occupied {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error":"Move or delete the pipeline's deals before deleting it"})),
        ));
    }
    sqlx::query("DELETE FROM crm_pipelines WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    tx.commit().await.map_err(crm_db_error)?;
    Ok(Json(json!({"message": "Pipeline deleted successfully"})))
}

/// POST /admin/crm/pipelines/:id/stages - Add a stage to a pipeline
async fn add_pipeline_stage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(pipeline_id): Path<i64>,
    Json(input): Json<CreateStageInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::text(&input.name, "Stage name", 255, true)?;
    if let Some(v) = &input.description {
        super::deal_mutations::text(v, "Description", 5000, false)?;
    }
    validate_color(input.color.as_deref())?;
    let won = input.is_closed_won.unwrap_or(false);
    let lost = input.is_closed_lost.unwrap_or(false);
    let position = input.position.unwrap_or(0);
    let probability = input.probability.unwrap_or(if won {
        100
    } else if lost {
        0
    } else {
        50
    });
    if won && lost
        || !(0..=100).contains(&probability)
        || !(0..=1000).contains(&position)
        || won && probability != 100
        || lost && probability != 0
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error":"Stage must have a position from 0 to 1000 and a probability from 0 to 100; closing stages require 100% won or 0% lost, never both"}),
            ),
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let active: Option<i64> =
        sqlx::query_scalar("SELECT id FROM crm_pipelines WHERE id=$1 AND is_active=true FOR SHARE")
            .bind(pipeline_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(crm_db_error)?;
    if active.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Active pipeline not found"})),
        ));
    }

    let stage: CrmPipelineStage = sqlx::query_as(concat!(
        "INSERT INTO crm_pipeline_stages (pipeline_id, name, description, position, probability,
                                          is_closed_won, is_closed_lost, color, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
         RETURNING ",
        crm_stage_cols!()
    ))
    .bind(pipeline_id)
    .bind(input.name.trim())
    .bind(&input.description)
    .bind(position)
    .bind(probability)
    .bind(input.is_closed_won.unwrap_or(false))
    .bind(input.is_closed_lost.unwrap_or(false))
    .bind(input.color.unwrap_or_else(|| "#6366f1".to_string()))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    tx.commit().await.map_err(crm_db_error)?;
    Ok(Json(json!({
        "id": stage.id.to_string(),
        "pipeline_id": stage.pipeline_id.to_string(),
        "name": stage.name,
        "description": stage.description,
        "position": stage.position,
        "probability": stage.probability,
        "is_closed_won": stage.is_closed_won,
        "is_closed_lost": stage.is_closed_lost,
        "color": stage.color
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Deals
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/deals - List all deals (ROUTED)
async fn list_deals(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<DealFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if filters
        .currency
        .as_ref()
        .is_some_and(|v| v.len() != 3 || !v.bytes().all(|b| b.is_ascii_uppercase()))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Currency must be a three-letter uppercase code"})),
        ));
    }
    if filters
        .status
        .as_ref()
        .is_some_and(|v| !["open", "won", "lost"].contains(&v.as_str()))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid deal status"})),
        ));
    }
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).clamp(1, 100);
    let offset = page.saturating_sub(1).saturating_mul(per_page);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let deals: Vec<DealRow> = sqlx::query_as(concat!(
        "SELECT ",
        crm_deal_cols!(),
        " FROM crm_deals
        WHERE ($1::bigint IS NULL OR pipeline_id = $1)
          AND ($2::bigint IS NULL OR stage_id = $2)
          AND ($3::text IS NULL OR status = $3)
          AND ($4::bigint IS NULL OR owner_id = $4)
          AND ($5::text IS NULL OR name ILIKE $5)
          AND ($8::text IS NULL OR currency = $8)
        ORDER BY created_at DESC
        LIMIT $6 OFFSET $7"
    ))
    .bind(filters.pipeline_id)
    .bind(filters.stage_id)
    .bind(filters.status.as_deref())
    .bind(filters.owner_id)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .bind(filters.currency.as_deref())
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM crm_deals
        WHERE ($1::bigint IS NULL OR pipeline_id = $1)
          AND ($2::bigint IS NULL OR stage_id = $2)
          AND ($3::text IS NULL OR status = $3)
          AND ($4::bigint IS NULL OR owner_id = $4)
          AND ($5::text IS NULL OR name ILIKE $5)
          AND ($6::text IS NULL OR currency = $6)
        ",
    )
    .bind(filters.pipeline_id)
    .bind(filters.stage_id)
    .bind(filters.status.as_deref())
    .bind(filters.owner_id)
    .bind(search_pattern.as_deref())
    .bind(filters.currency.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .map_err(crm_db_error)?;

    // Monetary summaries are grouped by currency; never add unlike currencies.
    let summaries:Vec<serde_json::Value>=sqlx::query_scalar("SELECT jsonb_build_object('currency',currency,'total_deals',COUNT(*),'open_deals',COUNT(*) FILTER(WHERE status='open'),'won_deals',COUNT(*) FILTER(WHERE status='won'),'lost_deals',COUNT(*) FILTER(WHERE status='lost'),'open_value',COALESCE(SUM(amount) FILTER(WHERE status='open'),0),'weighted_value',COALESCE(SUM(ROUND(amount*probability/100,2)) FILTER(WHERE status='open'),0),'filtered_count',COUNT(*) FILTER(WHERE $6::text IS NULL OR status=$6),'filtered_value',COALESCE(SUM(amount) FILTER(WHERE $6::text IS NULL OR status=$6),0),'filtered_weighted_value',COALESCE(SUM(ROUND(amount*probability/100,2)) FILTER(WHERE $6::text IS NULL OR status=$6),0)) FROM crm_deals WHERE ($1::bigint IS NULL OR pipeline_id=$1) AND ($2::bigint IS NULL OR stage_id=$2) AND ($3::bigint IS NULL OR owner_id=$3) AND ($4::text IS NULL OR name ILIKE $4) AND ($5::text IS NULL OR currency=$5) GROUP BY currency ORDER BY currency")
        .bind(filters.pipeline_id).bind(filters.stage_id).bind(filters.owner_id).bind(search_pattern.as_deref()).bind(filters.currency.as_deref()).bind(filters.status.as_deref()).fetch_all(&state.db.pool).await.map_err(crm_db_error)?;

    let deal_data = deals.iter().map(deal_json).collect::<Result<Vec<_>, _>>()?;

    Ok(Json(json!({
        "data": deal_data,
        "summaries": summaries,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/crm/deals/:id - Get a single deal
async fn get_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let deal: DealRow = sqlx::query_as(concat!(
        "SELECT ",
        crm_deal_cols!(),
        " FROM crm_deals WHERE id = $1"
    ))
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
            Json(json!({"error": "Deal not found"})),
        )
    })?;

    Ok(Json(deal_json(&deal)?))
}

/// POST /admin/crm/deals - Create a new deal
async fn create_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::create(&state, _admin.0.id, input).await
}

/// PUT /admin/crm/deals/:id - Update a deal
async fn update_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::update(&state, _admin.0.id, id, input).await
}

/// PATCH /admin/crm/deals/:id/stage - Update deal stage
async fn update_deal_stage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDealStageInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::stage(&state, _admin.0.id, id, input).await
}

/// POST /admin/crm/deals/:id/win - Mark deal as won
async fn win_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<WinDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::win(&state, _admin.0.id, id, input).await
}

/// POST /admin/crm/deals/:id/lose - Mark deal as lost
async fn lose_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<LoseDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    super::deal_mutations::lose(&state, _admin.0.id, id, input).await
}

/// DELETE /admin/crm/deals/:id - Delete a deal
async fn delete_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_deals WHERE id = $1")
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
            Json(json!({"error": "Deal not found"})),
        ));
    }

    Ok(Json(json!({"message": "Deal deleted successfully"})))
}

/// GET /admin/crm/deals/forecast - Get deal forecast
async fn get_deal_forecast(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let period = params
        .get("period")
        .map(|s| s.as_str())
        .unwrap_or("this_month");

    let now = chrono::Utc::now().date_naive();
    let (start_date, end_date) = match period {
        "this_week" => {
            let start =
                now - chrono::Duration::days(i64::from(now.weekday().num_days_from_monday()));
            (start, start + chrono::Duration::days(6))
        }
        "this_month" | "this_quarter" => {
            let month = if period == "this_quarter" {
                ((now.month() - 1) / 3) * 3 + 1
            } else {
                now.month()
            };
            let start = chrono::NaiveDate::from_ymd_opt(now.year(), month, 1).ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"Unable to calculate forecast period"})),
                )
            })?;
            let end = start
                .checked_add_months(chrono::Months::new(if period == "this_quarter" {
                    3
                } else {
                    1
                }))
                .and_then(|d| d.pred_opt())
                .ok_or_else(|| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error":"Unable to calculate forecast period"})),
                    )
                })?;
            (start, end)
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Invalid forecast period"})),
            ));
        }
    };
    let currency = params.get("currency").map(String::as_str).unwrap_or("USD");
    if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Currency must be a three-letter uppercase code"})),
        ));
    }
    let pipeline = params
        .get("pipeline_id")
        .map(|v| {
            v.parse::<i64>().map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"Invalid pipeline ID"})),
                )
            })
        })
        .transpose()?;

    // Get deals with expected close dates in the period
    let forecast_deals: Vec<(sqlx::types::Decimal, i32)> = sqlx::query_as(
        r"
        SELECT amount, probability FROM crm_deals
        WHERE status = 'open'
          AND expected_close_date >= $1
          AND expected_close_date <= $2
          AND currency=$3 AND ($4::bigint IS NULL OR pipeline_id=$4)
        ",
    )
    .bind(start_date)
    .bind(end_date)
    .bind(currency)
    .bind(pipeline)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let mut commit = 0_i64;
    let mut weighted = 0_i64;
    let mut total = 0_i64;
    let overflow = || {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Forecast exceeds supported amount range"})),
        )
    };
    for (amount, probability) in &forecast_deals {
        let cents = decimal_to_cents(*amount)?;
        total = total.checked_add(cents).ok_or_else(overflow)?;
        let value = cents
            .checked_mul(i64::from(*probability))
            .and_then(|v| v.checked_add(50))
            .map(|v| v / 100)
            .ok_or_else(overflow)?;
        weighted = weighted.checked_add(value).ok_or_else(overflow)?;
        if *probability >= 75 {
            commit = commit.checked_add(cents).ok_or_else(overflow)?;
        }
    }
    // Preserve the existing projection model, and state its assumptions explicitly.
    let conservative = weighted
        .checked_mul(60)
        .map(|v| v / 100)
        .ok_or_else(overflow)?;
    Ok(Json(
        json!({"period":period,"currency":currency,"pipeline_id":pipeline.map(|id|id.to_string()),"start_date":start_date,"end_date":end_date,
        "commit_cents":commit,"best_case_cents":weighted,"pipeline_cents":total,"worst_case_cents":conservative,
        "commit":commit as f64/100.0,"best_case":weighted as f64/100.0,"pipeline":total as f64/100.0,"worst_case":conservative as f64/100.0,
        "deals_count":forecast_deals.len(),"model":"Open deals with expected close dates in the period. Commit sums deals with probability at least 75%; best case is probability-weighted value; worst case is 60% of that weighted value. These are projections, not guaranteed revenue."}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/pipelines", get(list_pipelines).post(create_pipeline))
        .route(
            "/pipelines/{id}",
            get(get_pipeline)
                .put(update_pipeline)
                .delete(delete_pipeline),
        )
        .route("/pipelines/{id}/stages", post(add_pipeline_stage))
        .route("/deals", get(list_deals).post(create_deal))
        .route("/deals/forecast", get(get_deal_forecast))
        .route(
            "/deals/{id}",
            get(get_deal).put(update_deal).delete(delete_deal),
        )
        .route("/deals/{id}/stage", patch(update_deal_stage))
        .route("/deals/{id}/win", post(win_deal))
        .route("/deals/{id}/lose", post(lose_deal))
}
