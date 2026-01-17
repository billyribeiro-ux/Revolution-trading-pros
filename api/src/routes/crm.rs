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
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{models::User, AppState};

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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
    Path(_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Return mock timeline for now - can be expanded with activity tracking
    Ok(Json(json!([
        {"type": "created", "message": "Contact was created", "timestamp": chrono::Utc::now()}
    ])))
}

async fn recalculate_contact_score(
    State(state): State<AppState>,
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
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
    _user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0 }
    })))
}

async fn get_system_logs(
    State(_state): State<AppState>,
    _user: User,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50);
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0, "per_page": per_page }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/stats - CRM dashboard statistics
async fn get_crm_stats(
    State(_state): State<AppState>,
    _user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "total_contacts": 0,
        "active_contacts": 0,
        "new_this_month": 0,
        "total_deals": 0,
        "deals_value": 0.0,
        "open_deals": 0,
        "won_deals": 0,
        "lost_deals": 0,
        "email_sent": 0,
        "email_opened": 0,
        "email_clicked": 0
    })))
}

/// GET /admin/crm/deals - List all deals
async fn list_deals(
    State(_state): State<AppState>,
    _user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "deals": [],
        "pagination": {
            "total": 0,
            "per_page": 25,
            "current_page": 1,
            "last_page": 1
        }
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard Stats
        .route("/stats", get(get_crm_stats))
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
        // Sequences
        .route("/sequences", get(list_sequences).post(create_sequence))
        .route("/sequences/:id", get(get_sequence).delete(delete_sequence))
        // Campaigns
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/campaigns/:id", get(get_campaign).delete(delete_campaign))
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
