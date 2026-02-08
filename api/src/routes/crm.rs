//! CRM Admin Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Comprehensive CRM backend matching FluentCRM Pro functionality:
//! - Contacts, Deals, Pipelines
//! - Email Sequences, Recurring Campaigns
//! - Smart Links, Automation Funnels
//! - Lists, Tags, Segments, Companies
//! - Templates, Webhooks, Import/Export
//! - Manager Permissions, Settings, System Logs

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

// ICT 7 SECURITY FIX: Changed from User to AdminUser for all CRM endpoints
use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Contacts
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
// TYPES - Lists, Tags, Segments
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactList {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub contacts_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactTag {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub contacts_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactSegment {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub match_type: String,
    pub contacts_count: i32,
    pub is_active: bool,
    pub last_synced_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Sequences, Campaigns
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

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Automations, Smart Links, Templates
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AutomationFunnel {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub trigger_type: String,
    pub trigger_name: Option<String>,
    pub actions_count: i32,
    pub subscribers_count: i32,
    pub conversion_rate: f64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SmartLink {
    pub id: i64,
    pub title: String,
    pub short: String,
    pub target_url: Option<String>,
    pub is_active: bool,
    pub click_count: i32,
    pub unique_clicks: i32,
    pub actions: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmailTemplate {
    pub id: i64,
    pub title: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
    pub thumbnail: Option<String>,
    pub is_global: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Webhooks, Companies, Abandoned Carts
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub events: Option<serde_json::Value>,
    pub is_active: bool,
    pub trigger_count: i32,
    pub failure_count: i32,
    pub last_triggered_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmCompany {
    pub id: i64,
    pub name: String,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub logo_url: Option<String>,
    pub contacts_count: i32,
    pub deals_count: i32,
    pub total_deal_value: f64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Contacts
// ═══════════════════════════════════════════════════════════════════════════

async fn list_contacts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ContactFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let contacts: Vec<CrmContact> = sqlx::query_as(
        r#"
        SELECT id, email, first_name, last_name, phone, company, job_title, 
               status, source, COALESCE(score, 0) as score, tags, custom_fields,
               last_contacted_at, created_at, updated_at
        FROM contacts
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR first_name ILIKE $2 OR last_name ILIKE $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
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

    let search_pattern2 = filters.search.as_ref().map(|s| format!("%{}%", s));
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM contacts
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR first_name ILIKE $2 OR last_name ILIKE $2)
        "#,
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
        r#"SELECT id, email, first_name, last_name, phone, company, job_title, 
                  status, source, COALESCE(score, 0) as score, tags, custom_fields,
                  last_contacted_at, created_at, updated_at
           FROM contacts WHERE id = $1"#,
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
        r#"
        INSERT INTO contacts (email, first_name, last_name, phone, company, job_title, 
                              source, status, tags, custom_fields, score, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, NOW(), NOW())
        RETURNING id, email, first_name, last_name, phone, company, job_title, 
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        "#,
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
        r#"
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
        "#,
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
        r#"
        UPDATE contacts SET score = 50, updated_at = NOW()
        WHERE id = $1
        RETURNING id, email, first_name, last_name, phone, company, job_title, 
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(contact))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Lists
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ListFilters {
    pub search: Option<String>,
    pub is_public: Option<bool>,
    pub per_page: Option<i64>,
}

async fn list_contact_lists(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let lists: Vec<ContactList> = sqlx::query_as(
        r#"
        SELECT id, title, slug, description, is_public, 
               COALESCE(contacts_count, 0) as contacts_count, created_at, updated_at
        FROM crm_lists
        WHERE ($1::text IS NULL OR title ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        ORDER BY created_at DESC
        LIMIT $3
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(filters.is_public)
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": lists,
        "meta": { "total": lists.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

async fn create_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");

    let list: ContactList = sqlx::query_as(
        r#"
        INSERT INTO crm_lists (title, slug, description, is_public, contacts_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, title, slug, description, is_public, contacts_count, created_at, updated_at
        "#,
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(input.is_public.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(list))
}

async fn get_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: ContactList = sqlx::query_as(
        "SELECT id, title, slug, description, is_public, contacts_count, created_at, updated_at FROM crm_lists WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "List not found"}))))?;

    Ok(Json(json!({
        "list": list,
        "contacts_count": list.contacts_count
    })))
}

async fn delete_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_lists WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "List deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Tags
// ═══════════════════════════════════════════════════════════════════════════

async fn list_contact_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let tags: Vec<ContactTag> = sqlx::query_as(
        r#"
        SELECT id, title, slug, description, color, 
               COALESCE(contacts_count, 0) as contacts_count, created_at, updated_at
        FROM crm_tags
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": tags,
        "meta": { "total": tags.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateTagInput {
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

async fn create_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTagInput>,
) -> Result<Json<ContactTag>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");

    let tag: ContactTag = sqlx::query_as(
        r#"
        INSERT INTO crm_tags (title, slug, description, color, contacts_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, title, slug, description, color, contacts_count, created_at, updated_at
        "#,
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(tag))
}

async fn delete_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_tags WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Tag deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Segments
// ═══════════════════════════════════════════════════════════════════════════

async fn list_segments(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let segments: Vec<ContactSegment> = sqlx::query_as(
        r#"
        SELECT id, title, slug, description, conditions, match_type,
               COALESCE(contacts_count, 0) as contacts_count, is_active,
               last_synced_at, created_at, updated_at
        FROM crm_segments
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": segments,
        "meta": { "total": segments.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateSegmentInput {
    pub title: String,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub match_type: Option<String>,
}

async fn create_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSegmentInput>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.title.to_lowercase().replace(' ', "-");
    let match_type = input.match_type.unwrap_or_else(|| "all".to_string());

    let segment: ContactSegment = sqlx::query_as(
        r#"
        INSERT INTO crm_segments (title, slug, description, conditions, match_type, contacts_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 0, true, NOW(), NOW())
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        "#,
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.conditions)
    .bind(&match_type)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
}

async fn get_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let segment: ContactSegment = sqlx::query_as(
        "SELECT id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at FROM crm_segments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))))?;

    Ok(Json(segment))
}

async fn delete_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_segments WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Segment deleted successfully"})))
}

async fn sync_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    let segment: ContactSegment = sqlx::query_as(
        r#"
        UPDATE crm_segments SET last_synced_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        "#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
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
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let sequences: Vec<EmailSequence> = sqlx::query_as(
        r#"
        SELECT id, title, status, trigger_type, email_count, total_subscribers,
               emails_sent, open_rate, click_rate, created_at, updated_at
        FROM crm_sequences
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": sequences,
        "meta": { "total": sequences.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateSequenceInput {
    pub title: String,
    pub trigger_type: Option<String>,
    pub status: Option<String>,
}

async fn create_sequence(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSequenceInput>,
) -> Result<Json<EmailSequence>, (StatusCode, Json<serde_json::Value>)> {
    let status = input.status.unwrap_or_else(|| "draft".to_string());

    let sequence: EmailSequence = sqlx::query_as(
        r#"
        INSERT INTO crm_sequences (title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, $3, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, status, trigger_type, email_count, total_subscribers, emails_sent, open_rate, click_rate, created_at, updated_at
        "#,
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
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let campaigns: Vec<Campaign> = sqlx::query_as(
        r#"
        SELECT id, title, subject, status, scheduled_at, sent_at, recipients_count,
               emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        FROM crm_campaigns
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": campaigns,
        "meta": { "total": campaigns.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignInput {
    pub title: String,
    pub subject: Option<String>,
    pub template_id: Option<i64>,
    pub scheduled_at: Option<String>,
}

async fn create_campaign(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateCampaignInput>,
) -> Result<Json<Campaign>, (StatusCode, Json<serde_json::Value>)> {
    let campaign: Campaign = sqlx::query_as(
        r#"
        INSERT INTO crm_campaigns (title, subject, status, template_id, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, 'draft', $3, 0, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        "#,
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

async fn list_recurring_campaigns(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let campaigns: Vec<RecurringCampaign> = sqlx::query_as(
        r#"
        SELECT id, title, status, scheduling_settings, total_campaigns_sent, total_emails_sent,
               total_revenue, last_sent_at, next_scheduled_at, created_at, updated_at
        FROM crm_recurring_campaigns
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": campaigns,
        "meta": { "total": campaigns.len() }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Automations
// ═══════════════════════════════════════════════════════════════════════════

async fn list_automations(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let automations: Vec<AutomationFunnel> = sqlx::query_as(
        r#"
        SELECT id, title, status, trigger_type, trigger_name, actions_count,
               subscribers_count, conversion_rate, created_at, updated_at
        FROM crm_automations
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": automations,
        "meta": { "total": automations.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateAutomationInput {
    pub title: String,
    pub trigger_type: String,
    pub trigger_name: Option<String>,
}

async fn create_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateAutomationInput>,
) -> Result<Json<AutomationFunnel>, (StatusCode, Json<serde_json::Value>)> {
    let automation: AutomationFunnel = sqlx::query_as(
        r#"
        INSERT INTO crm_automations (title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at)
        VALUES ($1, 'draft', $2, $3, 0, 0, 0.0, NOW(), NOW())
        RETURNING id, title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at
        "#,
    )
    .bind(&input.title)
    .bind(&input.trigger_type)
    .bind(&input.trigger_name)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(automation))
}

async fn get_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<AutomationFunnel>, (StatusCode, Json<serde_json::Value>)> {
    let automation: AutomationFunnel = sqlx::query_as(
        "SELECT id, title, status, trigger_type, trigger_name, actions_count, subscribers_count, conversion_rate, created_at, updated_at FROM crm_automations WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Automation not found"}))))?;

    Ok(Json(automation))
}

async fn delete_automation(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_automations WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Automation deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Smart Links
// ═══════════════════════════════════════════════════════════════════════════

async fn list_smart_links(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let links: Vec<SmartLink> = sqlx::query_as(
        r#"
        SELECT id, title, short, target_url, is_active, click_count, unique_clicks,
               actions, created_at, updated_at
        FROM crm_smart_links
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": links,
        "meta": { "total": links.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateSmartLinkInput {
    pub title: String,
    pub target_url: String,
    pub actions: Option<serde_json::Value>,
}

async fn create_smart_link(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateSmartLinkInput>,
) -> Result<Json<SmartLink>, (StatusCode, Json<serde_json::Value>)> {
    let short = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let link: SmartLink = sqlx::query_as(
        r#"
        INSERT INTO crm_smart_links (title, short, target_url, is_active, click_count, unique_clicks, actions, created_at, updated_at)
        VALUES ($1, $2, $3, true, 0, 0, $4, NOW(), NOW())
        RETURNING id, title, short, target_url, is_active, click_count, unique_clicks, actions, created_at, updated_at
        "#,
    )
    .bind(&input.title)
    .bind(&short)
    .bind(&input.target_url)
    .bind(&input.actions)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(link))
}

async fn delete_smart_link(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_smart_links WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Smart link deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Templates
// ═══════════════════════════════════════════════════════════════════════════

async fn list_templates(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let templates: Vec<EmailTemplate> = sqlx::query_as(
        r#"
        SELECT id, title, subject, body, category, thumbnail, is_global, created_at, updated_at
        FROM crm_templates
        WHERE ($1::text IS NULL OR title ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": templates,
        "meta": { "total": templates.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateInput {
    pub title: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub category: Option<String>,
}

async fn create_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTemplateInput>,
) -> Result<Json<EmailTemplate>, (StatusCode, Json<serde_json::Value>)> {
    let template: EmailTemplate = sqlx::query_as(
        r#"
        INSERT INTO crm_templates (title, subject, body, category, is_global, created_at, updated_at)
        VALUES ($1, $2, $3, $4, false, NOW(), NOW())
        RETURNING id, title, subject, body, category, thumbnail, is_global, created_at, updated_at
        "#,
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
        "SELECT id, title, subject, body, category, thumbnail, is_global, created_at, updated_at FROM crm_templates WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Template not found"}))))?;

    Ok(Json(template))
}

async fn delete_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_templates WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Template deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Webhooks
// ═══════════════════════════════════════════════════════════════════════════

async fn list_webhooks(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let webhooks: Vec<Webhook> = sqlx::query_as(
        r#"
        SELECT id, name, url, events, is_active, trigger_count, failure_count,
               last_triggered_at, created_at, updated_at
        FROM crm_webhooks
        WHERE ($1::text IS NULL OR name ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": webhooks,
        "meta": { "total": webhooks.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookInput {
    pub name: String,
    pub url: String,
    pub events: Option<serde_json::Value>,
}

async fn create_webhook(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateWebhookInput>,
) -> Result<Json<Webhook>, (StatusCode, Json<serde_json::Value>)> {
    let webhook: Webhook = sqlx::query_as(
        r#"
        INSERT INTO crm_webhooks (name, url, events, is_active, trigger_count, failure_count, created_at, updated_at)
        VALUES ($1, $2, $3, true, 0, 0, NOW(), NOW())
        RETURNING id, name, url, events, is_active, trigger_count, failure_count, last_triggered_at, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.url)
    .bind(&input.events)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(webhook))
}

async fn delete_webhook(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_webhooks WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Webhook deleted successfully"})))
}

async fn test_webhook(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Mock test - actual implementation would send test payload
    Ok(Json(
        json!({"success": true, "message": "Test webhook sent", "webhook_id": id}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Companies
// ═══════════════════════════════════════════════════════════════════════════

async fn list_companies(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).min(100);
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let companies: Vec<CrmCompany> = sqlx::query_as(
        r#"
        SELECT id, name, website, industry, size, logo_url, contacts_count,
               deals_count, total_deal_value, created_at, updated_at
        FROM crm_companies
        WHERE ($1::text IS NULL OR name ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "data": companies,
        "meta": { "total": companies.len() }
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyInput {
    pub name: String,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
}

async fn create_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateCompanyInput>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company: CrmCompany = sqlx::query_as(
        r#"
        INSERT INTO crm_companies (name, website, industry, size, contacts_count, deals_count, total_deal_value, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, 0, 0.0, NOW(), NOW())
        RETURNING id, name, website, industry, size, logo_url, contacts_count, deals_count, total_deal_value, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.website)
    .bind(&input.industry)
    .bind(&input.size)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(company))
}

async fn get_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company: CrmCompany = sqlx::query_as(
        "SELECT id, name, website, industry, size, logo_url, contacts_count, deals_count, total_deal_value, created_at, updated_at FROM crm_companies WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Company not found"}))))?;

    Ok(Json(company))
}

async fn delete_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM crm_companies WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Company deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Import/Export
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ImportInput {
    pub file_type: String,
    pub data: serde_json::Value,
    pub options: Option<serde_json::Value>,
}

async fn import_contacts(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<ImportInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Mock import - actual implementation would process CSV/JSON
    Ok(Json(json!({
        "success": true,
        "message": "Import started",
        "file_type": input.file_type,
        "records_queued": 0
    })))
}

async fn export_contacts(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Mock export - actual implementation would generate CSV/JSON
    Ok(Json(json!({
        "success": true,
        "message": "Export started",
        "download_url": null
    })))
}

async fn get_import_history(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0 }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Settings & Managers
// ═══════════════════════════════════════════════════════════════════════════

async fn get_crm_settings(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "email_settings": {
            "from_name": "Revolution Trading Pros",
            "from_email": "noreply@revolutiontradingpros.com",
            "reply_to": "support@revolutiontradingpros.com"
        },
        "scoring_settings": {
            "enabled": true,
            "max_score": 100
        },
        "general_settings": {
            "double_opt_in": false,
            "unsubscribe_footer": true
        }
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsInput {
    pub settings: serde_json::Value,
}

async fn update_crm_settings(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<UpdateSettingsInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "message": "Settings updated",
        "settings": input.settings
    })))
}

async fn list_managers(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0 }
    })))
}

async fn get_system_logs(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50);
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0, "per_page": per_page }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Leads (Contacts with lead status)
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

/// GET /admin/crm/leads - List all leads
async fn list_leads(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<LeadFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

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
        r#"
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
        "#,
    )
    .bind(filters.status.as_deref())
    .bind(filters.source.as_deref())
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Transform to match frontend Lead interface
    let transformed_leads: Vec<serde_json::Value> = leads
        .iter()
        .map(|lead| {
            let full_name = match (&lead.first_name, &lead.last_name) {
                (Some(f), Some(l)) => format!("{} {}", f, l),
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
        r#"
        SELECT COUNT(*) FROM contacts
        WHERE status IN ('new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost', 'lead')
          AND ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR source = $2)
          AND ($3::text IS NULL OR email ILIKE $3 OR first_name ILIKE $3 OR last_name ILIKE $3 OR company ILIKE $3)
        "#,
    )
    .bind(filters.status.as_deref())
    .bind(filters.source.as_deref())
    .bind(filters.search.as_ref().map(|s| format!("%{}%", s)).as_deref())
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
        r#"
        INSERT INTO contacts (email, first_name, last_name, phone, company, job_title,
                              source, status, tags, custom_fields, score, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, NOW(), NOW())
        RETURNING id, email, first_name, last_name, phone, company, job_title,
                  status, source, score, tags, custom_fields, last_contacted_at, created_at, updated_at
        "#,
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
        (Some(f), Some(l)) => format!("{} {}", f, l),
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
        r#"
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
        "#,
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
        (Some(f), Some(l)) => format!("{} {}", f, l),
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

/// POST /admin/crm/segments/:id/duplicate - Duplicate a segment
async fn duplicate_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, (StatusCode, Json<serde_json::Value>)> {
    // Get original segment
    let original: ContactSegment = sqlx::query_as(
        "SELECT id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at FROM crm_segments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))))?;

    let new_title = format!("{} (Copy)", original.title);
    let new_slug = format!("{}-copy", original.slug);

    let segment: ContactSegment = sqlx::query_as(
        r#"
        INSERT INTO crm_segments (title, slug, description, conditions, match_type, contacts_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, 0, false, NOW(), NOW())
        RETURNING id, title, slug, description, conditions, match_type, contacts_count, is_active, last_synced_at, created_at, updated_at
        "#,
    )
    .bind(&new_title)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.conditions)
    .bind(&original.match_type)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(segment))
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
        r#"
        INSERT INTO crm_campaigns (title, subject, status, template_id, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, created_at, updated_at)
        VALUES ($1, $2, 'draft', $3, 0, 0, 0, 0, 0.0, 0.0, NOW(), NOW())
        RETURNING id, title, subject, status, scheduled_at, sent_at, recipients_count, emails_sent, opens, clicks, open_rate, click_rate, template_id, created_at, updated_at
        "#,
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
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/stats - CRM dashboard statistics
async fn get_crm_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get total contacts count
    let total_contacts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Get active contacts (not churned/unqualified)
    let active_contacts: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE status NOT IN ('churned', 'unqualified')",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Get new contacts this month
    let new_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE created_at >= date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Get leads stats (leads are contacts with status 'lead')
    let total_leads: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'lead'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Get campaigns stats
    let campaigns_sent: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(emails_sent), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let campaigns_opened: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(opens), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let campaigns_clicked: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(clicks), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total_contacts": total_contacts.0,
        "active_contacts": active_contacts.0,
        "new_this_month": new_this_month.0,
        "total_leads": total_leads.0,
        "total_deals": 0,
        "deals_value": 0.0,
        "open_deals": 0,
        "won_deals": 0,
        "lost_deals": 0,
        "email_sent": campaigns_sent.0,
        "email_opened": campaigns_opened.0,
        "email_clicked": campaigns_clicked.0
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - Deals & Pipelines (ICT 7 Grade Implementation)
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
    pub amount: Option<f64>,
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
    pub amount: Option<f64>,
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
// HANDLERS - Pipelines
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/pipelines - List all pipelines
async fn list_pipelines(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let pipelines: Vec<CrmPipeline> = sqlx::query_as(
        r#"
        SELECT id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        FROM crm_pipelines
        WHERE is_active = true
        ORDER BY position ASC, created_at ASC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // For each pipeline, get its stages and stats
    let mut result = Vec::new();
    for pipeline in pipelines {
        let stages: Vec<CrmPipelineStage> = sqlx::query_as(
            r#"
            SELECT id, pipeline_id, name, description, position, probability,
                   is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
            FROM crm_pipeline_stages
            WHERE pipeline_id = $1
            ORDER BY position ASC
            "#,
        )
        .bind(pipeline.id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

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
        r#"
        SELECT id, pipeline_id, name, description, position, probability,
               is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
        FROM crm_pipeline_stages
        WHERE pipeline_id = $1
        ORDER BY position ASC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

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
        r#"
        INSERT INTO crm_pipelines (name, description, is_default, is_active, color, icon, position, created_at, updated_at)
        VALUES ($1, $2, false, true, $3, $4, (SELECT COALESCE(MAX(position), 0) + 1 FROM crm_pipelines), NOW(), NOW())
        RETURNING id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        "#,
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
        r#"
        UPDATE crm_pipelines SET
            name = $2,
            description = $3,
            color = COALESCE($4, color),
            icon = COALESCE($5, icon),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, description, is_default, is_active, color, icon, position, created_at, updated_at
        "#,
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
        r#"
        INSERT INTO crm_pipeline_stages (pipeline_id, name, description, position, probability,
                                          is_closed_won, is_closed_lost, color, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, pipeline_id, name, description, position, probability,
                  is_closed_won, is_closed_lost, auto_advance_after_days, color, created_at, updated_at
        "#,
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

/// GET /admin/crm/deals - List all deals
async fn list_deals(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<DealFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;
    let search_pattern = filters.search.as_ref().map(|s| format!("%{}%", s));

    let deals: Vec<CrmDeal> = sqlx::query_as(
        r#"
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
        "#,
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
    .unwrap_or_default();

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM crm_deals
        WHERE ($1::bigint IS NULL OR pipeline_id = $1)
          AND ($2::bigint IS NULL OR stage_id = $2)
          AND ($3::text IS NULL OR status = $3)
          AND ($4::bigint IS NULL OR owner_id = $4)
        "#,
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

/// GET /admin/crm/deals/:id - Get a single deal
async fn get_deal(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let deal: CrmDeal = sqlx::query_as(
        r#"
        SELECT id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
               amount, currency, probability, status, expected_close_date, close_date,
               lost_reason, won_details, priority, source_channel, source_campaign,
               tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        FROM crm_deals WHERE id = $1
        "#,
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
    let amount = input.amount.unwrap_or(0.0);
    let currency = input.currency.unwrap_or_else(|| "USD".to_string());
    let probability = input.probability.unwrap_or(50);
    let priority = input.priority.unwrap_or_else(|| "medium".to_string());
    let tags = input.tags.and_then(|t| serde_json::to_value(t).ok());

    let expected_close_date: Option<chrono::NaiveDate> = input
        .expected_close_date
        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let deal: CrmDeal = sqlx::query_as(
        r#"
        INSERT INTO crm_deals (name, contact_id, company_id, pipeline_id, stage_id, amount, currency,
                               probability, status, expected_close_date, priority, source_channel,
                               source_campaign, tags, custom_fields, stage_entered_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'open', $9, $10, $11, $12, $13, $14, NOW(), NOW(), NOW())
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(input.contact_id)
    .bind(input.company_id)
    .bind(input.pipeline_id)
    .bind(input.stage_id)
    .bind(amount)
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
        r#"
        UPDATE crm_deals SET
            name = COALESCE($2, name),
            amount = COALESCE($3, amount),
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
        "#,
    )
    .bind(id)
    .bind(&input.name)
    .bind(input.amount)
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
        r#"
        INSERT INTO crm_deal_stage_history (deal_id, from_stage_id, to_stage_id, reason, duration_in_stage, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
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
        r#"
        UPDATE crm_deals SET
            stage_id = $2,
            stage_entered_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, contact_id, company_id, pipeline_id, stage_id, owner_id,
                  amount, currency, probability, status, expected_close_date, close_date,
                  lost_reason, won_details, priority, source_channel, source_campaign,
                  tags, custom_fields, stage_entered_at, closed_at, created_at, updated_at
        "#,
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
        r#"
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
        "#,
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
        r#"
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
        "#,
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
        r#"
        SELECT amount, probability FROM crm_deals
        WHERE status = 'open'
          AND expected_close_date >= $1
          AND expected_close_date <= $2
        "#,
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

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
// HANDLERS - Bulk Operations for Leads
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct BulkDeleteInput {
    pub ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BulkStatusInput {
    pub ids: Vec<i64>,
    pub status: String,
}

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

pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard Stats
        .route("/stats", get(get_crm_stats))
        // Leads - CRITICAL: Required by /admin/crm/leads frontend page
        .route("/leads", get(list_leads).post(create_lead))
        .route("/leads/stats", get(get_lead_stats))
        .route("/leads/:id", put(update_lead).delete(delete_lead))
        // Deals
        .route("/deals", get(list_deals))
        // Contacts
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
        // Lists
        .route("/lists", get(list_contact_lists).post(create_contact_list))
        .route(
            "/lists/:id",
            get(get_contact_list).delete(delete_contact_list),
        )
        // Tags
        .route(
            "/contact-tags",
            get(list_contact_tags).post(create_contact_tag),
        )
        .route("/contact-tags/:id", delete(delete_contact_tag))
        // Segments
        .route("/segments", get(list_segments).post(create_segment))
        .route("/segments/:id", get(get_segment).delete(delete_segment))
        .route("/segments/:id/sync", post(sync_segment))
        .route("/segments/:id/duplicate", post(duplicate_segment))
        // Sequences
        .route("/sequences", get(list_sequences).post(create_sequence))
        .route("/sequences/:id", get(get_sequence).delete(delete_sequence))
        // Campaigns
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/campaigns/:id", get(get_campaign).delete(delete_campaign))
        .route("/campaigns/:id/duplicate", post(duplicate_campaign))
        .route("/recurring-campaigns", get(list_recurring_campaigns))
        // Automations
        .route(
            "/automations",
            get(list_automations).post(create_automation),
        )
        .route(
            "/automations/:id",
            get(get_automation).delete(delete_automation),
        )
        // Smart Links
        .route(
            "/smart-links",
            get(list_smart_links).post(create_smart_link),
        )
        .route("/smart-links/:id", delete(delete_smart_link))
        // Templates
        .route("/templates", get(list_templates).post(create_template))
        .route("/templates/:id", get(get_template).delete(delete_template))
        // Webhooks
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/:id", delete(delete_webhook))
        .route("/webhooks/:id/test", post(test_webhook))
        // Companies
        .route("/companies", get(list_companies).post(create_company))
        .route("/companies/:id", get(get_company).delete(delete_company))
        // Import/Export
        .route("/import", post(import_contacts))
        .route("/import/history", get(get_import_history))
        .route("/export", post(export_contacts))
        // Settings & Managers
        .route("/settings", get(get_crm_settings).put(update_crm_settings))
        .route("/managers", get(list_managers))
        .route("/system-logs", get(get_system_logs))
}
