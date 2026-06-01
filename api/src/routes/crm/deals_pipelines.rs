//! CRM deals + pipelines handlers
//! (split from `crm.rs` lines 2231-3270 — types, pipeline handlers,
//! deal handlers, deal forecast).
//!
//! NOTE: The original `crm::router()` only registered `/deals` GET → list_deals.
//! All other deal/pipeline handlers (create_deal, update_deal, win_deal,
//! lose_deal, list_pipelines, …) were defined but unrouted. We preserve
//! that behavior here: the router only registers list_deals. The other
//! handlers remain available for future routing without re-introducing the
//! megafile.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, AppState};

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
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub pipeline_id: Option<i64>,
    pub stage_id: Option<i64>,
    pub status: Option<String>,
    pub owner_id: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDealInput {
    pub name: String,
    pub contact_id: Option<i64>,
    pub company_id: Option<i64>,
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
pub struct UpdateDealInput {
    pub name: Option<String>,
    // Batch 5c: integer-cents (see CreateDealInput).
    pub amount_cents: Option<i64>,
    pub probability: Option<i32>,
    pub expected_close_date: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDealStageInput {
    pub stage_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WinDealInput {
    pub won_details: Option<String>,
    pub close_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoseDealInput {
    pub lost_reason: String,
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

/// GET /admin/crm/pipelines - List all pipelines
async fn list_pipelines(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let pipelines: Vec<CrmPipeline> = sqlx::query_as(
        r"
        SELECT id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        FROM crm_pipelines
        WHERE is_active = true
        ORDER BY position ASC, created_at ASC
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // For each pipeline, get its stages and stats
    let mut result = Vec::new();
    for pipeline in pipelines {
        let stages: Vec<CrmPipelineStage> = sqlx::query_as(
            r"
            SELECT id, pipeline_id, name, description, position, probability,
                   is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
            FROM crm_pipeline_stages
            WHERE pipeline_id = $1
            ORDER BY position ASC
            ",
        )
        .bind(pipeline.id)
        .fetch_all(&state.db.pool)
        .await
        // FIX-2026-04-26: .unwrap_or_default();
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))? ;

        let deals_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM crm_deals WHERE pipeline_id = $1")
                .bind(pipeline.id)
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((0,));

        let total_value: (Option<sqlx::types::Decimal>,) = sqlx::query_as(
            "SELECT SUM(amount) FROM crm_deals WHERE pipeline_id = $1 AND status = 'open'",
        )
        .bind(pipeline.id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((None,));

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
            "total_value": total_value.0.map(|v| v.to_string()).unwrap_or_else(|| "0".to_string()),
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
    let pipeline: CrmPipeline = sqlx::query_as(
        "SELECT id, name, description, is_default, is_active, color, icon, position, created_at, updated_at FROM crm_pipelines WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Pipeline not found"}))))?;

    let stages: Vec<CrmPipelineStage> = sqlx::query_as(
        r"
        SELECT id, pipeline_id, name, description, position, probability,
               is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
        FROM crm_pipeline_stages
        WHERE pipeline_id = $1
        ORDER BY position ASC
        ",
    )
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
    let pipeline: CrmPipeline = sqlx::query_as(
        r"
        INSERT INTO crm_pipelines (name, description, is_default, is_active, color, icon, position, created_at, updated_at)
        VALUES ($1, $2, false, true, $3, $4, (SELECT COALESCE(MAX(position), 0) + 1 FROM crm_pipelines), NOW(), NOW())
        RETURNING id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        ",
    )
    .bind(&input.name)
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
    let pipeline: CrmPipeline = sqlx::query_as(
        r"
        UPDATE crm_pipelines SET
            name = $2,
            description = $3,
            color = COALESCE($4, color),
            icon = COALESCE($5, icon),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.color)
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
        "color": pipeline.color
    })))
}

/// DELETE /admin/crm/pipelines/:id - Delete a pipeline
async fn delete_pipeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check if it's the default pipeline
    let pipeline: CrmPipeline = sqlx::query_as(
        "SELECT id, name, description, is_default, is_active, color, icon, position, created_at, updated_at FROM crm_pipelines WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Pipeline not found"}))))?;

    if pipeline.is_default {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Cannot delete default pipeline"})),
        ));
    }

    sqlx::query("DELETE FROM crm_pipelines WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Pipeline deleted successfully"})))
}

/// POST /admin/crm/pipelines/:id/stages - Add a stage to a pipeline
async fn add_pipeline_stage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(pipeline_id): Path<i64>,
    Json(input): Json<CreateStageInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let position = input.position.unwrap_or(0);
    let probability = input.probability.unwrap_or(50);

    let stage: CrmPipelineStage = sqlx::query_as(
        r"
        INSERT INTO crm_pipeline_stages (pipeline_id, name, description, position, probability,
                                          is_closed_won, is_closed_lost, color, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, pipeline_id, name, description, position, probability,
                  is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
        ",
    )
    .bind(pipeline_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(position)
    .bind(probability)
    .bind(input.is_closed_won.unwrap_or(false))
    .bind(input.is_closed_lost.unwrap_or(false))
    .bind(input.color.unwrap_or_else(|| "#6366f1".to_string()))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

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
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let deals: Vec<CrmDeal> = sqlx::query_as(
        r"
        SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
               amount, currency, probability, status, expected_close_date, close_date,
               lost_reason, won_details, priority, source_channel, source_campaign,
               tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        FROM crm_deals
        WHERE ($1::bigint IS NULL OR pipeline_id = $1)
          AND ($2::bigint IS NULL OR stage_id = $2)
          AND ($3::text IS NULL OR status = $3)
          AND ($4::bigint IS NULL OR owner_id = $4)
          AND ($5::text IS NULL OR name ILIKE $5)
        ORDER BY created_at DESC
        LIMIT $6 OFFSET $7
        ",
    )
    .bind(filters.pipeline_id)
    .bind(filters.stage_id)
    .bind(filters.status.as_deref())
    .bind(filters.owner_id)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
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
        ",
    )
    .bind(filters.pipeline_id)
    .bind(filters.stage_id)
    .bind(filters.status.as_deref())
    .bind(filters.owner_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let deal_data: Vec<serde_json::Value> = deals
        .iter()
        .map(|d| {
            json!({
                "id": d.id.to_string(),
                "name": d.name,
                "contact_id": d.contact_id.map(|id| id.to_string()),
                "company_id": d.company_id.map(|id| id.to_string()),
                "pipeline_id": d.pipeline_id.to_string(),
                "stage_id": d.stage_id.to_string(),
                "owner_id": d.owner_id.map(|id| id.to_string()),
                "amount": d.amount.to_string().parse::<f64>().unwrap_or(0.0),
                "currency": d.currency,
                "probability": d.probability,
                "status": d.status,
                "expected_close_date": d.expected_close_date,
                "close_date": d.close_date,
                "lost_reason": d.lost_reason,
                "won_details": d.won_details,
                "priority": d.priority,
                "source_channel": d.source_channel,
                "source_campaign": d.source_campaign,
                "tags": d.tags,
                "custom_fields": d.custom_fields,
                "stage_entered_at": d.stage_entered_at,
                "closed_at": d.closed_at,
                "created_at": d.created_at,
                "updated_at": d.updated_at
            })
        })
        .collect();

    Ok(Json(json!({
        "data": deal_data,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/crm/deals/:id - Get a single deal (defined; not routed)
async fn get_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let deal: CrmDeal = sqlx::query_as(
        r"
        SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
               amount, currency, probability, status, expected_close_date, close_date,
               lost_reason, won_details, priority, source_channel, source_campaign,
               tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        FROM crm_deals WHERE id = $1
        ",
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
            Json(json!({"error": "Deal not found"})),
        )
    })?;

    Ok(Json(json!({
        "id": deal.id.to_string(),
        "name": deal.name,
        "contact_id": deal.contact_id.map(|id| id.to_string()),
        "company_id": deal.company_id.map(|id| id.to_string()),
        "pipeline_id": deal.pipeline_id.to_string(),
        "stage_id": deal.stage_id.to_string(),
        "owner_id": deal.owner_id.map(|id| id.to_string()),
        "amount": deal.amount.to_string().parse::<f64>().unwrap_or(0.0),
        "currency": deal.currency,
        "probability": deal.probability,
        "status": deal.status,
        "expected_close_date": deal.expected_close_date,
        "close_date": deal.close_date,
        "lost_reason": deal.lost_reason,
        "won_details": deal.won_details,
        "priority": deal.priority,
        "source_channel": deal.source_channel,
        "source_campaign": deal.source_campaign,
        "tags": deal.tags,
        "custom_fields": deal.custom_fields,
        "stage_entered_at": deal.stage_entered_at,
        "closed_at": deal.closed_at,
        "created_at": deal.created_at,
        "updated_at": deal.updated_at
    })))
}

/// POST /admin/crm/deals - Create a new deal
async fn create_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Batch 5c: input is integer cents; DB column is NUMERIC(15,2).
    // Convert at the SQL boundary so in-Rust math stays in i64 cents.
    let amount_cents: i64 = input.amount_cents.unwrap_or(0);
    let currency = input.currency.unwrap_or_else(|| "USD".to_string());
    let probability = input.probability.unwrap_or(50);
    let priority = input.priority.unwrap_or_else(|| "medium".to_string());
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());

    let expected_close_date: Option<chrono::NaiveDate> = input
        .expected_close_date
        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let deal: CrmDeal = sqlx::query_as(
        r"
        INSERT INTO crm_deals (name, contact_id, company_id, pipeline_id, stage_id, amount, currency,
                               probability, status, expected_close_date, priority, source_channel,
                               source_campaign, tags, custom_fields, stage_entered_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6::BIGINT / 100.0, $7, $8, 'open', $9, $10, $11, $12, $13, $14, NOW(), NOW(), NOW())
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(input.contact_id)
    .bind(input.company_id)
    .bind(input.pipeline_id)
    .bind(input.stage_id)
    .bind(amount_cents)
    .bind(&currency)
    .bind(probability)
    .bind(expected_close_date)
    .bind(&priority)
    .bind(&input.source_channel)
    .bind(&input.source_campaign)
    .bind(&tags)
    .bind(&input.custom_fields)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "id": deal.id.to_string(),
        "name": deal.name,
        "pipeline_id": deal.pipeline_id.to_string(),
        "stage_id": deal.stage_id.to_string(),
        "amount": deal.amount.to_string().parse::<f64>().unwrap_or(0.0),
        "status": deal.status
    })))
}

/// PUT /admin/crm/deals/:id - Update a deal
async fn update_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());
    let expected_close_date: Option<chrono::NaiveDate> = input
        .expected_close_date
        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let deal: CrmDeal = sqlx::query_as(
        r"
        UPDATE crm_deals SET
            name = COALESCE($2, name),
            amount = COALESCE($3::BIGINT / 100.0, amount),
            probability = COALESCE($4, probability),
            expected_close_date = COALESCE($5, expected_close_date),
            priority = COALESCE($6, priority),
            tags = COALESCE($7, tags),
            custom_fields = COALESCE($8, custom_fields),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.name)
    .bind(input.amount_cents)
    .bind(input.probability)
    .bind(expected_close_date)
    .bind(&input.priority)
    .bind(&tags)
    .bind(&input.custom_fields)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "id": deal.id.to_string(),
        "name": deal.name,
        "amount": deal.amount.to_string().parse::<f64>().unwrap_or(0.0),
        "status": deal.status
    })))
}

/// PATCH /admin/crm/deals/:id/stage - Update deal stage
async fn update_deal_stage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDealStageInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current deal to track stage history
    let current_deal: CrmDeal = sqlx::query_as(
        "SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id, amount, currency, probability, status, expected_close_date, close_date, lost_reason, won_details, priority, source_channel, source_campaign, tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at FROM crm_deals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Deal not found"}))))?;

    // Record stage history
    let duration = chrono::Utc::now()
        .naive_utc()
        .signed_duration_since(current_deal.stage_entered_at);
    sqlx::query(
        r"
        INSERT INTO crm_deal_stage_history (deal_id, from_stage_id, to_stage_id, reason, duration_in_stage, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ",
    )
    .bind(id)
    .bind(current_deal.stage_id)
    .bind(input.stage_id)
    .bind(&input.reason)
    .bind(duration.num_seconds() as i32)
    .execute(&state.db.pool)
    .await
    .ok(); // Ignore errors in history tracking

    // Update the deal stage
    let deal: CrmDeal = sqlx::query_as(
        r"
        UPDATE crm_deals SET
            stage_id = $2,
            stage_entered_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(input.stage_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "id": deal.id.to_string(),
        "name": deal.name,
        "stage_id": deal.stage_id.to_string(),
        "status": deal.status
    })))
}

/// POST /admin/crm/deals/:id/win - Mark deal as won
async fn win_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<WinDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let close_date: Option<chrono::NaiveDate> = input
        .close_date
        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())
        .or_else(|| Some(chrono::Utc::now().date_naive()));

    // Get the won stage for this pipeline
    let deal: CrmDeal = sqlx::query_as(
        "SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id, amount, currency, probability, status, expected_close_date, close_date, lost_reason, won_details, priority, source_channel, source_campaign, tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at FROM crm_deals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Deal not found"}))))?;

    let won_stage: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM crm_pipeline_stages WHERE pipeline_id = $1 AND is_closed_won = true LIMIT 1"
    )
    .bind(deal.pipeline_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let stage_id = won_stage.map(|s| s.0).unwrap_or(deal.stage_id);

    let updated_deal: CrmDeal = sqlx::query_as(
        r"
        UPDATE crm_deals SET
            status = 'won',
            stage_id = $2,
            won_details = $3,
            close_date = $4,
            closed_at = NOW(),
            probability = 100,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(stage_id)
    .bind(&input.won_details)
    .bind(close_date)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "id": updated_deal.id.to_string(),
        "name": updated_deal.name,
        "status": updated_deal.status,
        "amount": updated_deal.amount.to_string().parse::<f64>().unwrap_or(0.0),
        "won_details": updated_deal.won_details,
        "close_date": updated_deal.close_date
    })))
}

/// POST /admin/crm/deals/:id/lose - Mark deal as lost
async fn lose_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<LoseDealInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the lost stage for this pipeline
    let deal: CrmDeal = sqlx::query_as(
        "SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id, amount, currency, probability, status, expected_close_date, close_date, lost_reason, won_details, priority, source_channel, source_campaign, tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at FROM crm_deals WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Deal not found"}))))?;

    let lost_stage: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM crm_pipeline_stages WHERE pipeline_id = $1 AND is_closed_lost = true LIMIT 1"
    )
    .bind(deal.pipeline_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let stage_id = lost_stage.map(|s| s.0).unwrap_or(deal.stage_id);

    let updated_deal: CrmDeal = sqlx::query_as(
        r"
        UPDATE crm_deals SET
            status = 'lost',
            stage_id = $2,
            lost_reason = $3,
            close_date = CURRENT_DATE,
            closed_at = NOW(),
            probability = 0,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(stage_id)
    .bind(&input.lost_reason)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "id": updated_deal.id.to_string(),
        "name": updated_deal.name,
        "status": updated_deal.status,
        "lost_reason": updated_deal.lost_reason
    })))
}

/// DELETE /admin/crm/deals/:id - Delete a deal
async fn delete_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_deals WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

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

    let (start_date, end_date) = match period {
        "this_week" => {
            let now = chrono::Utc::now().date_naive();
            let start = now - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
            let end = start + chrono::Duration::days(6);
            (start, end)
        }
        "this_month" => {
            let now = chrono::Utc::now().date_naive();
            let start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);
            let end = (start + chrono::Duration::days(31))
                .with_day(1)
                .unwrap_or(now)
                - chrono::Duration::days(1);
            (start, end)
        }
        "this_quarter" => {
            let now = chrono::Utc::now().date_naive();
            let quarter_start_month = ((now.month() - 1) / 3) * 3 + 1;
            let start =
                chrono::NaiveDate::from_ymd_opt(now.year(), quarter_start_month, 1).unwrap_or(now);
            let end = (start + chrono::Duration::days(92))
                .with_day(1)
                .unwrap_or(now)
                - chrono::Duration::days(1);
            (start, end)
        }
        _ => {
            let now = chrono::Utc::now().date_naive();
            let start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);
            let end = (start + chrono::Duration::days(31))
                .with_day(1)
                .unwrap_or(now)
                - chrono::Duration::days(1);
            (start, end)
        }
    };

    // Get deals with expected close dates in the period
    let forecast_deals: Vec<(sqlx::types::Decimal, i32)> = sqlx::query_as(
        r"
        SELECT amount, probability FROM crm_deals
        WHERE status = 'open'
          AND expected_close_date >= $1
          AND expected_close_date <= $2
        ",
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let deals_count = forecast_deals.len() as i64;
    let mut commit = 0.0f64;
    let mut best_case = 0.0f64;
    let mut pipeline_total = 0.0f64;

    for (amount, probability) in &forecast_deals {
        let amt: f64 = amount.to_string().parse().unwrap_or(0.0);
        pipeline_total += amt;
        let weighted = amt * (*probability as f64 / 100.0);
        if *probability >= 75 {
            commit += amt;
        }
        best_case += weighted;
    }

    Ok(Json(json!({
        "period": period,
        "commit": commit,
        "best_case": best_case,
        "pipeline": pipeline_total,
        "worst_case": best_case * 0.6,
        "deals_count": deals_count
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    // Preserve exact pre-split routing: only `/deals` GET was registered
    // for this sub-domain in the original `crm::router()`. The other deal
    // and pipeline handlers (create_deal, update_deal, win_deal, lose_deal,
    // list_pipelines, get_pipeline, create_pipeline, update_pipeline,
    // delete_pipeline, add_pipeline_stage, update_deal_stage, get_deal,
    // delete_deal, get_deal_forecast) were defined but unrouted in the
    // pre-split source. They remain defined here so future router
    // registrations are a one-line change.
    Router::new().route("/deals", get(list_deals))
}
