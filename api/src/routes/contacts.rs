//! CRM Contacts routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

#![allow(clippy::map_flatten)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

// ICT 7 SECURITY FIX: Use AdminUser for all contact management (admin-only)
use crate::{middleware::admin::AdminUser, AppState};

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ContactRow {
    pub id: i64,
    pub user_id: Option<i64>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub source: Option<String>,
    pub status: String,
    pub tags: Option<serde_json::Value>,
    pub custom_fields: Option<serde_json::Value>,
    pub last_contacted_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ContactListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateContactRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub source: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContactRequest {
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

/// List contacts (admin)
/// ICT 7 SECURITY FIX: Use parameterized queries to prevent SQL injection
async fn list_contacts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<ContactListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // ICT 7: Validate status against allowlist
    let valid_statuses = ["lead", "prospect", "customer", "inactive", "churned"];
    let status = query.status.as_ref().and_then(|s| {
        if valid_statuses.contains(&s.to_lowercase().as_str()) {
            Some(s.to_lowercase())
        } else {
            None
        }
    });

    // ICT 7: Prepare search pattern for ILIKE
    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    // ICT 7: Use fully parameterized queries
    let contacts: Vec<ContactRow> = sqlx::query_as(
        r#"
        SELECT * FROM contacts
        WHERE ($1::TEXT IS NULL OR status = $1)
        AND ($2::TEXT IS NULL OR (
            email ILIKE $2 OR
            first_name ILIKE $2 OR
            last_name ILIKE $2 OR
            company ILIKE $2
        ))
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(&status)
    .bind(&search_pattern)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM contacts
        WHERE ($1::TEXT IS NULL OR status = $1)
        AND ($2::TEXT IS NULL OR (
            email ILIKE $2 OR
            first_name ILIKE $2 OR
            last_name ILIKE $2 OR
            company ILIKE $2
        ))
        "#,
    )
    .bind(&status)
    .bind(&search_pattern)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

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

/// Get contact by ID (admin)
async fn get_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactRow>, (StatusCode, Json<serde_json::Value>)> {
    let contact: ContactRow = sqlx::query_as("SELECT * FROM contacts WHERE id = $1")
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

/// Create contact (admin)
async fn create_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateContactRequest>,
) -> Result<Json<ContactRow>, (StatusCode, Json<serde_json::Value>)> {
    let tags = input.tags.map(|t| serde_json::to_value(t).ok()).flatten();
    let status = input.status.unwrap_or_else(|| "lead".to_string());

    let contact: ContactRow = sqlx::query_as(
        r#"
        INSERT INTO contacts (email, first_name, last_name, phone, company, job_title, source, status, tags, custom_fields, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())
        RETURNING *
        "#
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

/// Update contact (admin)
/// ICT 7 SECURITY FIX: Use parameterized queries to prevent SQL injection
async fn update_contact(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactRequest>,
) -> Result<Json<ContactRow>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7: Validate status against allowlist
    let valid_statuses = ["lead", "prospect", "customer", "inactive", "churned"];
    let status = input.status.as_ref().and_then(|s| {
        if valid_statuses.contains(&s.to_lowercase().as_str()) {
            Some(s.to_lowercase())
        } else {
            None
        }
    });

    // ICT 7: Use fully parameterized query with COALESCE for optional updates
    let contact: ContactRow = sqlx::query_as(
        r#"
        UPDATE contacts SET
            email = COALESCE($2, email),
            first_name = COALESCE($3, first_name),
            last_name = COALESCE($4, last_name),
            phone = COALESCE($5, phone),
            company = COALESCE($6, company),
            status = COALESCE($7, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&input.email)
    .bind(&input.first_name)
    .bind(&input.last_name)
    .bind(&input.phone)
    .bind(&input.company)
    .bind(&status)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(contact))
}

/// Delete contact (admin)
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

/// Contact stats (admin)
async fn contact_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let leads: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'lead'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let prospects: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'prospect'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let customers: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'customer'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total": total.0,
        "leads": leads.0,
        "prospects": prospects.0,
        "customers": customers.0
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_contacts).post(create_contact))
        .route("/stats", get(contact_stats))
        .route(
            "/:id",
            get(get_contact).put(update_contact).delete(delete_contact),
        )
}
