//! CMS Routes - Apple ICT 11+ Principal Engineer Grade
//! January 2026
//!
//! Enterprise CMS API endpoints for:
//! - Content versioning with rollback
//! - Audit logging for compliance
//! - Workflow management with approvals
//! - Webhook configuration and delivery
//! - Publish scheduling
//! - Preview token generation
//! - i18n/Localization support

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{
    models::User,
    services::cms::{
        self, AuditContext, AuditLog, AuditLogQuery, ContentTranslation, ContentVersion,
        Locale, PreviewToken, ScheduledContent, Webhook, WorkflowStatus,
    },
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════
// AUTHORIZATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════

fn require_admin(user: &User) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, Json(json!({
            "error": "Access denied",
            "message": "This action requires admin privileges"
        }))))
    }
}

fn require_editor(user: &User) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" || role == "editor" {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, Json(json!({
            "error": "Access denied",
            "message": "This action requires editor privileges"
        }))))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONTENT VERSIONING ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub data: JsonValue,
    pub change_summary: Option<String>,
    pub changed_fields: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct VersionHistoryQuery {
    pub limit: Option<i64>,
}

/// Get version history for content
/// GET /admin/cms/versions/:content_type/:content_id
async fn get_version_history(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Query(query): Query<VersionHistoryQuery>,
) -> Result<Json<Vec<ContentVersion>>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let limit = query.limit.unwrap_or(25).min(100);

    let versions = cms::get_version_history(&state.db.pool, &content_type, content_id, limit)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(versions))
}

/// Create a new content version
/// POST /admin/cms/versions/:content_type/:content_id
async fn create_version(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Json(input): Json<CreateVersionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let version_id = cms::create_version(
        &state.db.pool,
        &content_type,
        content_id,
        input.data,
        user.id,
        input.change_summary.as_deref(),
        input.changed_fields,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "version_id": version_id,
        "message": "Version created successfully"
    })))
}

/// Get a specific version
/// GET /admin/cms/versions/:content_type/:content_id/:version_number
async fn get_version(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, version_number)): Path<(String, i64, i32)>,
) -> Result<Json<ContentVersion>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let version = cms::get_version(&state.db.pool, &content_type, content_id, version_number)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Version not found"}))))?;

    Ok(Json(version))
}

/// Restore a previous version
/// POST /admin/cms/versions/:content_type/:content_id/:version_number/restore
async fn restore_version(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, version_number)): Path<(String, i64, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let new_version_id = cms::restore_version(
        &state.db.pool,
        &content_type,
        content_id,
        version_number,
        user.id,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Log the restore action
    let _ = cms::log_audit(
        &state.db.pool,
        "content.version.restored",
        &content_type,
        Some(content_id),
        &AuditContext {
            user_id: Some(user.id),
            user_email: Some(user.email.clone()),
            ip_address: None,
            user_agent: None,
        },
        None,
        Some(json!({ "restored_version": version_number, "new_version_id": new_version_id })),
        None,
    )
    .await;

    Ok(Json(json!({
        "new_version_id": new_version_id,
        "restored_from": version_number,
        "message": "Version restored successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// AUDIT LOGGING ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

/// Get audit logs with filtering
/// GET /admin/cms/audit-logs
async fn get_audit_logs(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLog>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let logs = cms::get_audit_logs(&state.db.pool, &query)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(logs))
}

// ═══════════════════════════════════════════════════════════════════════════
// WORKFLOW MANAGEMENT ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct TransitionWorkflowRequest {
    pub to_stage: String,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignForReviewRequest {
    pub assigned_to: i64,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<String>,
    pub notes: Option<String>,
}

/// Get workflow status for content
/// GET /admin/cms/workflow/:content_type/:content_id
async fn get_workflow_status(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
) -> Result<Json<WorkflowStatus>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let status = cms::get_or_create_workflow_status(&state.db.pool, &content_type, content_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(status))
}

/// Transition workflow to new stage
/// POST /admin/cms/workflow/:content_type/:content_id/transition
async fn transition_workflow(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Json(input): Json<TransitionWorkflowRequest>,
) -> Result<Json<WorkflowStatus>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let status = cms::transition_workflow(
        &state.db.pool,
        &content_type,
        content_id,
        &input.to_stage,
        user.id,
        input.comment.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Log the workflow transition
    let _ = cms::log_audit(
        &state.db.pool,
        "content.workflow.transitioned",
        &content_type,
        Some(content_id),
        &AuditContext {
            user_id: Some(user.id),
            user_email: Some(user.email.clone()),
            ip_address: None,
            user_agent: None,
        },
        None,
        Some(json!({ "to_stage": input.to_stage, "comment": input.comment })),
        None,
    )
    .await;

    Ok(Json(status))
}

/// Assign content for review
/// POST /admin/cms/workflow/:content_type/:content_id/assign
async fn assign_for_review(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Json(input): Json<AssignForReviewRequest>,
) -> Result<Json<WorkflowStatus>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let status = cms::assign_for_review(
        &state.db.pool,
        &content_type,
        content_id,
        input.assigned_to,
        user.id,
        input.due_date,
        input.priority.as_deref(),
        input.notes.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(status))
}

/// Get all pending content assigned to user
/// GET /admin/cms/workflow/my-assignments
async fn get_my_assignments(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Vec<WorkflowStatus>>, (StatusCode, Json<serde_json::Value>)> {
    let assignments: Vec<WorkflowStatus> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, workflow_id, current_stage,
               assigned_to, assigned_by, assigned_at, due_date, priority,
               notes, created_at, updated_at
        FROM content_workflow_status
        WHERE assigned_to = $1 AND current_stage NOT IN ('published', 'archived')
        ORDER BY
            CASE priority
                WHEN 'urgent' THEN 1
                WHEN 'high' THEN 2
                WHEN 'normal' THEN 3
                WHEN 'low' THEN 4
                ELSE 5
            END,
            due_date NULLS LAST,
            created_at DESC
        "#,
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(assignments))
}

// ═══════════════════════════════════════════════════════════════════════════
// WEBHOOK MANAGEMENT ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub content_types: Option<Vec<String>>,
    pub headers: Option<JsonValue>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWebhookRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub secret: Option<String>,
    pub events: Option<Vec<String>>,
    pub content_types: Option<Vec<String>>,
    pub headers: Option<JsonValue>,
    pub is_active: Option<bool>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,
}

/// List all webhooks
/// GET /admin/cms/webhooks
async fn list_webhooks(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Vec<Webhook>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let webhooks: Vec<Webhook> = sqlx::query_as(
        r#"
        SELECT id, name, description, url, secret, events, content_types,
               is_active, retry_count, timeout_seconds, headers, created_by,
               created_at, updated_at
        FROM webhooks
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(webhooks))
}

/// Create a new webhook
/// POST /admin/cms/webhooks
async fn create_webhook(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateWebhookRequest>,
) -> Result<Json<Webhook>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let webhook: Webhook = sqlx::query_as(
        r#"
        INSERT INTO webhooks (
            name, description, url, secret, events, content_types,
            is_active, retry_count, timeout_seconds, headers, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, true, $7, $8, $9, $10)
        RETURNING id, name, description, url, secret, events, content_types,
                  is_active, retry_count, timeout_seconds, headers, created_by,
                  created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.url)
    .bind(&input.secret)
    .bind(&input.events)
    .bind(&input.content_types)
    .bind(input.retry_count.unwrap_or(3))
    .bind(input.timeout_seconds.unwrap_or(30))
    .bind(&input.headers)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(webhook))
}

/// Update a webhook
/// PUT /admin/cms/webhooks/:id
async fn update_webhook(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateWebhookRequest>,
) -> Result<Json<Webhook>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let webhook: Webhook = sqlx::query_as(
        r#"
        UPDATE webhooks SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            url = COALESCE($3, url),
            secret = COALESCE($4, secret),
            events = COALESCE($5, events),
            content_types = COALESCE($6, content_types),
            is_active = COALESCE($7, is_active),
            retry_count = COALESCE($8, retry_count),
            timeout_seconds = COALESCE($9, timeout_seconds),
            headers = COALESCE($10, headers),
            updated_at = NOW()
        WHERE id = $11
        RETURNING id, name, description, url, secret, events, content_types,
                  is_active, retry_count, timeout_seconds, headers, created_by,
                  created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.url)
    .bind(&input.secret)
    .bind(&input.events)
    .bind(&input.content_types)
    .bind(input.is_active)
    .bind(input.retry_count)
    .bind(input.timeout_seconds)
    .bind(&input.headers)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(webhook))
}

/// Delete a webhook
/// DELETE /admin/cms/webhooks/:id
async fn delete_webhook(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query("DELETE FROM webhooks WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Webhook deleted successfully"})))
}

/// Get webhook delivery history
/// GET /admin/cms/webhooks/:id/deliveries
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WebhookDelivery {
    pub id: i64,
    pub webhook_id: i64,
    pub event_type: String,
    pub payload: JsonValue,
    pub status: String,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub attempts: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

async fn get_webhook_deliveries(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let deliveries: Vec<WebhookDelivery> = sqlx::query_as(
        r#"
        SELECT id, webhook_id, event_type, payload, status, response_status,
               response_body, attempts, next_retry_at, completed_at, created_at
        FROM webhook_deliveries
        WHERE webhook_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM webhook_deliveries WHERE webhook_id = $1"
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": deliveries,
        "meta": { "total": total.0, "limit": limit, "offset": offset }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLISH SCHEDULING ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SchedulePublishRequest {
    pub scheduled_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Get scheduled content
/// GET /admin/cms/scheduled
async fn get_scheduled_content(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<Vec<ScheduledContent>>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let scheduled: Vec<ScheduledContent> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, scheduled_action, scheduled_at,
               timezone, payload, status, executed_at, error_message,
               created_by, created_at, updated_at
        FROM scheduled_content
        WHERE status = 'scheduled'
        ORDER BY scheduled_at ASC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(scheduled))
}

/// Schedule content for future publication
/// POST /admin/cms/scheduled/:content_type/:content_id
async fn schedule_publish(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Json(input): Json<SchedulePublishRequest>,
) -> Result<Json<ScheduledContent>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let scheduled = cms::schedule_publish(
        &state.db.pool,
        &content_type,
        content_id,
        input.scheduled_at,
        user.id,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(scheduled))
}

/// Cancel scheduled publication
/// DELETE /admin/cms/scheduled/:content_type/:content_id
async fn cancel_scheduled(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    cms::cancel_scheduled(&state.db.pool, &content_type, content_id, "publish")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Scheduled publication cancelled"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// PREVIEW TOKEN ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreatePreviewTokenRequest {
    pub expires_in_hours: Option<i64>,
    pub max_views: Option<i32>,
}

/// Generate a preview token for unpublished content
/// POST /admin/cms/preview/:content_type/:content_id
async fn create_preview_token(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
    Json(input): Json<CreatePreviewTokenRequest>,
) -> Result<Json<PreviewToken>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let token = cms::generate_preview_token(
        &state.db.pool,
        &content_type,
        content_id,
        user.id,
        input.expires_in_hours.unwrap_or(24),
        input.max_views,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(token))
}

/// Validate a preview token (public endpoint for frontend)
/// GET /preview/:token
async fn validate_preview_token(
    State(state): State<AppState>,
    Path(token): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let preview = cms::validate_preview_token(&state.db.pool, token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({
            "error": "Invalid or expired preview token"
        }))))?;

    Ok(Json(json!({
        "valid": true,
        "content_type": preview.content_type,
        "content_id": preview.content_id,
        "expires_at": preview.expires_at,
        "views_remaining": preview.max_views.map(|m| m - preview.view_count)
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// i18n / LOCALIZATION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateLocaleRequest {
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_default: Option<bool>,
    pub rtl: Option<bool>,
    pub fallback_locale: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveTranslationRequest {
    pub translations: JsonValue,
    pub machine_translated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewTranslationRequest {
    pub quality_score: Option<i32>,
}

/// Get all active locales
/// GET /admin/cms/locales
async fn get_locales(
    State(state): State<AppState>,
) -> Result<Json<Vec<Locale>>, (StatusCode, Json<serde_json::Value>)> {
    let locales = cms::get_active_locales(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(locales))
}

/// Create a new locale
/// POST /admin/cms/locales
async fn create_locale(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateLocaleRequest>,
) -> Result<Json<Locale>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let locale: Locale = sqlx::query_as(
        r#"
        INSERT INTO locales (code, name, native_name, is_default, rtl, fallback_locale, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, true)
        RETURNING id, code, name, native_name, is_default, is_active, rtl, fallback_locale, created_at, updated_at
        "#,
    )
    .bind(&input.code)
    .bind(&input.name)
    .bind(&input.native_name)
    .bind(input.is_default.unwrap_or(false))
    .bind(input.rtl.unwrap_or(false))
    .bind(&input.fallback_locale)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(locale))
}

/// Update locale
/// PUT /admin/cms/locales/:code
async fn update_locale(
    State(state): State<AppState>,
    user: User,
    Path(code): Path<String>,
    Json(input): Json<CreateLocaleRequest>,
) -> Result<Json<Locale>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let locale: Locale = sqlx::query_as(
        r#"
        UPDATE locales SET
            name = $1,
            native_name = $2,
            is_default = $3,
            rtl = $4,
            fallback_locale = $5,
            updated_at = NOW()
        WHERE code = $6
        RETURNING id, code, name, native_name, is_default, is_active, rtl, fallback_locale, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.native_name)
    .bind(input.is_default.unwrap_or(false))
    .bind(input.rtl.unwrap_or(false))
    .bind(&input.fallback_locale)
    .bind(&code)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(locale))
}

/// Delete/deactivate locale
/// DELETE /admin/cms/locales/:code
async fn delete_locale(
    State(state): State<AppState>,
    user: User,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Don't actually delete, just deactivate
    sqlx::query("UPDATE locales SET is_active = false, updated_at = NOW() WHERE code = $1")
        .bind(&code)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Locale deactivated"})))
}

/// Get all translations for content
/// GET /admin/cms/translations/:content_type/:content_id
async fn get_translations(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id)): Path<(String, i64)>,
) -> Result<Json<Vec<ContentTranslation>>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let translations = cms::get_translations(&state.db.pool, &content_type, content_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(translations))
}

/// Get translation for specific locale
/// GET /admin/cms/translations/:content_type/:content_id/:locale
async fn get_translation(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, locale)): Path<(String, i64, String)>,
) -> Result<Json<ContentTranslation>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let translation = cms::get_translation(&state.db.pool, &content_type, content_id, &locale)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Translation not found"}))))?;

    Ok(Json(translation))
}

/// Save or update a translation
/// PUT /admin/cms/translations/:content_type/:content_id/:locale
async fn save_translation(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, locale)): Path<(String, i64, String)>,
    Json(input): Json<SaveTranslationRequest>,
) -> Result<Json<ContentTranslation>, (StatusCode, Json<serde_json::Value>)> {
    require_editor(&user)?;

    let translation = cms::save_translation(
        &state.db.pool,
        &content_type,
        content_id,
        &locale,
        input.translations,
        user.id,
        input.machine_translated.unwrap_or(false),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(translation))
}

/// Mark translation as reviewed
/// POST /admin/cms/translations/:content_type/:content_id/:locale/review
async fn review_translation(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, locale)): Path<(String, i64, String)>,
    Json(input): Json<ReviewTranslationRequest>,
) -> Result<Json<ContentTranslation>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let translation: ContentTranslation = sqlx::query_as(
        r#"
        UPDATE content_translations SET
            status = 'reviewed',
            reviewed_by = $1,
            quality_score = $2,
            updated_at = NOW()
        WHERE content_type = $3 AND content_id = $4 AND locale = $5
        RETURNING id, content_type, content_id, locale, translations, status,
                  translated_by, reviewed_by, machine_translated, quality_score,
                  created_at, updated_at
        "#,
    )
    .bind(user.id)
    .bind(input.quality_score)
    .bind(&content_type)
    .bind(content_id)
    .bind(&locale)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(translation))
}

/// Delete translation
/// DELETE /admin/cms/translations/:content_type/:content_id/:locale
async fn delete_translation(
    State(state): State<AppState>,
    user: User,
    Path((content_type, content_id, locale)): Path<(String, i64, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query(
        "DELETE FROM content_translations WHERE content_type = $1 AND content_id = $2 AND locale = $3"
    )
    .bind(&content_type)
    .bind(content_id)
    .bind(&locale)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Translation deleted"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// CMS STATISTICS & OVERVIEW
// ═══════════════════════════════════════════════════════════════════════════

/// Get CMS overview statistics
/// GET /admin/cms/stats
async fn get_cms_stats(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let total_versions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM content_versions")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let pending_workflows: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM content_workflow_status WHERE current_stage IN ('draft', 'review', 'pending_approval')"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let active_webhooks: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM webhooks WHERE is_active = true"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let scheduled_content: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM scheduled_content WHERE status = 'scheduled' AND scheduled_at > NOW()"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let active_previews: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM preview_tokens WHERE expires_at > NOW() AND (max_views IS NULL OR view_count < max_views)"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let active_locales: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM locales WHERE is_active = true"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_translations: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM content_translations")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let pending_webhook_deliveries: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM webhook_deliveries WHERE status = 'pending'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "versions": {
            "total": total_versions.0
        },
        "workflows": {
            "pending": pending_workflows.0
        },
        "webhooks": {
            "active": active_webhooks.0,
            "pending_deliveries": pending_webhook_deliveries.0
        },
        "scheduling": {
            "upcoming": scheduled_content.0
        },
        "previews": {
            "active": active_previews.0
        },
        "localization": {
            "active_locales": active_locales.0,
            "total_translations": total_translations.0
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Admin CMS routes (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Stats
        .route("/stats", get(get_cms_stats))
        // Content Versioning
        .route("/versions/:content_type/:content_id", get(get_version_history).post(create_version))
        .route("/versions/:content_type/:content_id/:version_number", get(get_version))
        .route("/versions/:content_type/:content_id/:version_number/restore", post(restore_version))
        // Audit Logs
        .route("/audit-logs", get(get_audit_logs))
        // Workflow Management
        .route("/workflow/my-assignments", get(get_my_assignments))
        .route("/workflow/:content_type/:content_id", get(get_workflow_status))
        .route("/workflow/:content_type/:content_id/transition", post(transition_workflow))
        .route("/workflow/:content_type/:content_id/assign", post(assign_for_review))
        // Webhooks
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/:id", put(update_webhook).delete(delete_webhook))
        .route("/webhooks/:id/deliveries", get(get_webhook_deliveries))
        // Publish Scheduling
        .route("/scheduled", get(get_scheduled_content))
        .route("/scheduled/:content_type/:content_id", post(schedule_publish).delete(cancel_scheduled))
        // Preview Tokens
        .route("/preview/:content_type/:content_id", post(create_preview_token))
        // Localization
        .route("/locales", get(get_locales).post(create_locale))
        .route("/locales/:code", put(update_locale).delete(delete_locale))
        .route("/translations/:content_type/:content_id", get(get_translations))
        .route("/translations/:content_type/:content_id/:locale", get(get_translation).put(save_translation).delete(delete_translation))
        .route("/translations/:content_type/:content_id/:locale/review", post(review_translation))
}

/// Public preview route (no auth required)
pub fn preview_router() -> Router<AppState> {
    Router::new()
        .route("/:token", get(validate_preview_token))
}
