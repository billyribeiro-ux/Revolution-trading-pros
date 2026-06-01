//! CRM lead handlers (contacts with lead status) and lead-bulk + abandoned-cart-bulk ops
//! (split from `crm.rs` lines 260-272 AbandonedCart type
//! + 1714-2085 lead handlers + 3273-3386 bulk handlers).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use super::contacts::{CreateContactInput, UpdateContactInput};
use crate::{middleware::admin::AdminUser, AppState};

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
pub struct LeadFilters {
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
    pub cart_total: f64,
    pub cart_items: Option<serde_json::Value>,
    pub status: String,
    pub recovery_email_sent: bool,
    pub recovered: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
async fn list_leads(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<LeadFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    // Lead statuses from the frontend Lead interface
    let lead_statuses = [
        "new",
        "contacted",
        "qualified",
        "proposal",
        "negotiation",
        "won",
        "lost",
        "lead",
    ];

    let leads: Vec<CrmLead> = sqlx::query_as(
        r"
        SELECT id, email, first_name, last_name, phone, company, job_title,
               status, source, COALESCE(score, 0) as score, tags, custom_fields,
               last_contacted_at, created_at, updated_at
        FROM contacts
        WHERE status IN ('new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost', 'lead')
          AND ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR source = $2)
          AND ($3::text IS NULL OR email ILIKE $3 OR first_name ILIKE $3 OR last_name ILIKE $3 OR company ILIKE $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(filters.status.as_deref())
    .bind(filters.source.as_deref())
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    // FIX-2026-04-26: .unwrap_or_default();
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Transform to match frontend Lead interface
    let transformed_leads: Vec<serde_json::Value> = leads
        .iter()
        .map(|lead| {
            let full_name = match (&lead.first_name, &lead.last_name) {
                (Some(f), Some(l)) => format!("{f} {l}"),
                (Some(f), None) => f.clone(),
                (None, Some(l)) => l.clone(),
                (None, None) => lead.email.clone(),
            };
            let is_hot = lead.score.unwrap_or(0) >= 75;
            json!({
                "id": lead.id.to_string(),
                "email": lead.email,
                "phone": lead.phone,
                "first_name": lead.first_name,
                "last_name": lead.last_name,
                "full_name": full_name,
                "company_name": lead.company,
                "job_title": lead.job_title,
                "status": lead.status,
                "source": lead.source.clone().unwrap_or_else(|| "other".to_string()),
                "lead_score": lead.score.unwrap_or(0),
                "is_hot": is_hot,
                "is_starred": false,
                "tags": lead.tags,
                "estimated_value": 0,
                "notes_count": 0,
                "activities_count": 0,
                "last_contacted_at": lead.last_contacted_at,
                "created_at": lead.created_at,
                "updated_at": lead.updated_at
            })
        })
        .collect();

    // Get total count
    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM contacts
        WHERE status IN ('new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost', 'lead')
          AND ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR source = $2)
          AND ($3::text IS NULL OR email ILIKE $3 OR first_name ILIKE $3 OR last_name ILIKE $3 OR company ILIKE $3)
        ",
    )
    .bind(filters.status.as_deref())
    .bind(filters.source.as_deref())
    .bind(filters.search.as_ref().map(|s| format!("%{s}%")).as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "data": transformed_leads,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/crm/leads/stats - Lead statistics
async fn get_lead_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get counts by status
    let new_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'new'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let contacted_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'contacted'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));
    let qualified_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'qualified'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));
    let proposal_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'proposal'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));
    let negotiation_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'negotiation'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));
    let won_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'won'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let lost_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'lost'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));
    let lead_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'lead'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Hot leads (score >= 75)
    let hot_leads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE status IN ('new', 'contacted', 'qualified', 'proposal', 'negotiation', 'lead') AND score >= 75"
    ).fetch_one(&state.db.pool).await.unwrap_or((0,));

    let total = new_count.0
        + contacted_count.0
        + qualified_count.0
        + proposal_count.0
        + negotiation_count.0
        + won_count.0
        + lost_count.0
        + lead_count.0;

    // Calculate conversion rate (won / total closed)
    let total_closed = won_count.0 + lost_count.0;
    let conversion_rate = if total_closed > 0 {
        (won_count.0 as f64 / total_closed as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(json!({
        "total": total,
        "new": new_count.0 + lead_count.0,
        "contacted": contacted_count.0,
        "qualified": qualified_count.0,
        "proposal": proposal_count.0,
        "negotiation": negotiation_count.0,
        "won": won_count.0,
        "lost": lost_count.0,
        "hot_leads": hot_leads.0,
        "total_value": 0,
        "conversion_rate": conversion_rate
    })))
}

/// POST /admin/crm/leads - Create a new lead
async fn create_lead(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateContactInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());
    let status = input.status.unwrap_or_else(|| "new".to_string());

    let lead: CrmLead = sqlx::query_as(
        r"
        INSERT INTO contacts (email, first_name, last_name, phone, company, job_title,
                              source, status, tags, custom_fields, score, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, NOW(), NOW())
        RETURNING id, email, first_name, last_name, phone, company, job_title,
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        ",
    )
    .bind(&input.email)
    .bind(&input.first_name)
    .bind(&input.last_name)
    .bind(&input.phone)
    .bind(&input.company)
    .bind(&input.job_title)
    .bind(&input.source)
    .bind(&status)
    .bind(&tags)
    .bind(&input.custom_fields)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            (StatusCode::CONFLICT, Json(json!({"error": "Lead with this email already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    let full_name = match (&lead.first_name, &lead.last_name) {
        (Some(f), Some(l)) => format!("{f} {l}"),
        (Some(f), None) => f.clone(),
        (None, Some(l)) => l.clone(),
        (None, None) => lead.email.clone(),
    };

    Ok(Json(json!({
        "id": lead.id.to_string(),
        "email": lead.email,
        "first_name": lead.first_name,
        "last_name": lead.last_name,
        "full_name": full_name,
        "phone": lead.phone,
        "company_name": lead.company,
        "job_title": lead.job_title,
        "status": lead.status,
        "source": lead.source,
        "lead_score": lead.score.unwrap_or(0),
        "is_hot": false,
        "is_starred": false,
        "created_at": lead.created_at,
        "updated_at": lead.updated_at
    })))
}

/// PUT /admin/crm/leads/:id - Update a lead
async fn update_lead(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());

    let lead: CrmLead = sqlx::query_as(
        r"
        UPDATE contacts SET
            email = COALESCE($2, email),
            first_name = COALESCE($3, first_name),
            last_name = COALESCE($4, last_name),
            phone = COALESCE($5, phone),
            company = COALESCE($6, company),
            job_title = COALESCE($7, job_title),
            status = COALESCE($8, status),
            tags = COALESCE($9, tags),
            custom_fields = COALESCE($10, custom_fields),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, first_name, last_name, phone, company, job_title,
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.email)
    .bind(&input.first_name)
    .bind(&input.last_name)
    .bind(&input.phone)
    .bind(&input.company)
    .bind(&input.job_title)
    .bind(&input.status)
    .bind(&tags)
    .bind(&input.custom_fields)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let full_name = match (&lead.first_name, &lead.last_name) {
        (Some(f), Some(l)) => format!("{f} {l}"),
        (Some(f), None) => f.clone(),
        (None, Some(l)) => l.clone(),
        (None, None) => lead.email.clone(),
    };

    Ok(Json(json!({
        "id": lead.id.to_string(),
        "email": lead.email,
        "first_name": lead.first_name,
        "last_name": lead.last_name,
        "full_name": full_name,
        "phone": lead.phone,
        "company_name": lead.company,
        "job_title": lead.job_title,
        "status": lead.status,
        "source": lead.source,
        "lead_score": lead.score.unwrap_or(0),
        "is_hot": lead.score.unwrap_or(0) >= 75,
        "is_starred": false,
        "created_at": lead.created_at,
        "updated_at": lead.updated_at
    })))
}

/// DELETE /admin/crm/leads/:id - Delete a lead
async fn delete_lead(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM contacts WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Lead deleted successfully"})))
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

    let updated =
        sqlx::query("UPDATE contacts SET status = $1, updated_at = NOW() WHERE id = ANY($2)")
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
    use axum::routing::put;
    Router::new()
        .route("/leads", get(list_leads).post(create_lead))
        .route("/leads/stats", get(get_lead_stats))
        .route("/leads/:id", put(update_lead).delete(delete_lead))
        // Bulk operations
        .route("/leads/bulk-delete", post(bulk_delete_leads))
        .route("/leads/bulk-status", post(bulk_update_lead_status))
        .route(
            "/abandoned-carts/bulk-delete",
            post(bulk_delete_abandoned_carts),
        )
}
