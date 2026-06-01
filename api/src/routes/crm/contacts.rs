//! CRM contacts handlers (split from `crm.rs` lines 30-84 + 278-503).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmContact {
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
pub struct ContactFilters {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub list_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContactInput {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContactInput {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_contacts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ContactFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{s}%"));

    let contacts: Vec<CrmContact> = sqlx::query_as(
        r"
        SELECT id, email, first_name, last_name, phone, company, job_title,
               status, source, COALESCE(score, 0) as score, tags, custom_fields,
               last_contacted_at, created_at, updated_at
        FROM contacts
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR first_name ILIKE $2 OR last_name ILIKE $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(filters.status.as_deref())
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("CRM contacts list error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let search_pattern2 = filters.search.as_ref().map(|s| format!("%{s}%"));
    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM contacts
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR first_name ILIKE $2 OR last_name ILIKE $2)
        ",
    )
    .bind(filters.status.as_deref())
    .bind(search_pattern2.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "data": contacts,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

async fn get_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmContact>, (StatusCode, Json<serde_json::Value>)> {
    let contact: CrmContact = sqlx::query_as(
        r"SELECT id, email, first_name, last_name, phone, company, job_title,
                  status, source, COALESCE(score, 0) as score, tags, custom_fields,
                  last_contacted_at, created_at, updated_at
           FROM contacts WHERE id = $1",
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
            Json(json!({"error": "Contact not found"})),
        )
    })?;

    Ok(Json(contact))
}

async fn create_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateContactInput>,
) -> Result<Json<CrmContact>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());
    let status = input.status.unwrap_or_else(|| "lead".to_string());

    let contact: CrmContact = sqlx::query_as(
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
            (StatusCode::CONFLICT, Json(json!({"error": "Contact with this email already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(contact))
}

async fn update_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactInput>,
) -> Result<Json<CrmContact>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());

    let contact: CrmContact = sqlx::query_as(
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

    Ok(Json(contact))
}

async fn delete_contact(
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

    Ok(Json(json!({"message": "Contact deleted successfully"})))
}

async fn get_contact_timeline(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return mock timeline for now - can be expanded with activity tracking
    Ok(Json(json!([
        {"type": "created", "message": "Contact was created", "timestamp": chrono::Utc::now()}
    ])))
}

async fn recalculate_contact_score(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmContact>, (StatusCode, Json<serde_json::Value>)> {
    // Simple scoring algorithm - can be expanded
    let contact: CrmContact = sqlx::query_as(
        r"
        UPDATE contacts SET score = 50, updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, first_name, last_name, phone, company, job_title,
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        ",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(contact))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/contacts", get(list_contacts).post(create_contact))
        .route(
            "/contacts/:id",
            get(get_contact).put(update_contact).delete(delete_contact),
        )
        .route("/contacts/:id/timeline", get(get_contact_timeline))
        .route(
            "/contacts/:id/recalculate-score",
            post(recalculate_contact_score),
        )
}
