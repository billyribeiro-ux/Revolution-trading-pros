//! CMS Service - Apple ICT 11+ Principal Engineer
//!
//! Comprehensive Content Management System service providing:
//! - Content versioning with full history
//! - Audit logging for compliance
//! - Workflow management with approvals
//! - Webhook delivery for integrations
//! - Publish scheduling
//! - Preview token generation
//! - i18n/Localization support
//!
//! @version 1.0.0 - January 2026

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// CONTENT VERSIONING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContentVersion {
    pub id: i64,
    pub content_type: String,
    pub content_id: i64,
    pub version_number: i32,
    pub is_current: bool,
    pub data: JsonValue,
    pub change_summary: Option<String>,
    pub changed_fields: Option<Vec<String>>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// Create a new version of content
pub async fn create_version(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    data: JsonValue,
    created_by: i64,
    change_summary: Option<&str>,
    changed_fields: Option<Vec<String>>,
) -> Result<i64> {
    let version_id: i64 = sqlx::query_scalar(
        r#"
        SELECT create_content_version($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(&data)
    .bind(created_by)
    .bind(change_summary)
    .bind(changed_fields.as_ref())
    .fetch_one(pool)
    .await?;

    Ok(version_id)
}

/// Get version history for content
pub async fn get_version_history(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    limit: i64,
) -> Result<Vec<ContentVersion>> {
    let versions: Vec<ContentVersion> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, version_number, is_current,
               data, change_summary, changed_fields, created_by, created_at
        FROM content_versions
        WHERE content_type = $1 AND content_id = $2
        ORDER BY version_number DESC
        LIMIT $3
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(versions)
}

/// Get a specific version
pub async fn get_version(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    version_number: i32,
) -> Result<Option<ContentVersion>> {
    let version: Option<ContentVersion> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, version_number, is_current,
               data, change_summary, changed_fields, created_by, created_at
        FROM content_versions
        WHERE content_type = $1 AND content_id = $2 AND version_number = $3
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(version_number)
    .fetch_optional(pool)
    .await?;

    Ok(version)
}

/// Restore a previous version
pub async fn restore_version(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    version_number: i32,
    restored_by: i64,
) -> Result<i64> {
    // Get the version to restore
    let version = get_version(pool, content_type, content_id, version_number)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Version not found"))?;

    // Create a new version with the restored data
    let new_version_id = create_version(
        pool,
        content_type,
        content_id,
        version.data,
        restored_by,
        Some(&format!("Restored from version {}", version_number)),
        None,
    )
    .await?;

    Ok(new_version_id)
}

// ═══════════════════════════════════════════════════════════════════════════
// AUDIT LOGGING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditContext {
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Log an audit event
pub async fn log_audit(
    pool: &PgPool,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    context: &AuditContext,
    old_values: Option<JsonValue>,
    new_values: Option<JsonValue>,
    metadata: Option<JsonValue>,
) -> Result<i64> {
    let audit_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO audit_logs (
            action, entity_type, entity_id, user_id, user_email,
            old_values, new_values, metadata, ip_address, user_agent
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9::inet, $10
        )
        RETURNING id
        "#,
    )
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(context.user_id)
    .bind(&context.user_email)
    .bind(&old_values)
    .bind(&new_values)
    .bind(&metadata)
    .bind(&context.ip_address)
    .bind(&context.user_agent)
    .fetch_one(pool)
    .await?;

    Ok(audit_id)
}

/// Get audit logs with filtering
#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<i64>,
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AuditLog {
    pub id: i64,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i64>,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_audit_logs(
    pool: &PgPool,
    query: &AuditLogQuery,
) -> Result<Vec<AuditLog>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    let logs: Vec<AuditLog> = sqlx::query_as(
        r#"
        SELECT id, action, entity_type, entity_id, user_id, user_email,
               old_values, new_values, metadata, created_at
        FROM audit_logs
        WHERE ($1::text IS NULL OR entity_type = $1)
          AND ($2::bigint IS NULL OR entity_id = $2)
          AND ($3::bigint IS NULL OR user_id = $3)
          AND ($4::text IS NULL OR action = $4)
          AND ($5::timestamptz IS NULL OR created_at >= $5)
          AND ($6::timestamptz IS NULL OR created_at <= $6)
        ORDER BY created_at DESC
        LIMIT $7 OFFSET $8
        "#,
    )
    .bind(&query.entity_type)
    .bind(query.entity_id)
    .bind(query.user_id)
    .bind(&query.action)
    .bind(query.from_date)
    .bind(query.to_date)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(logs)
}

// ═══════════════════════════════════════════════════════════════════════════
// WORKFLOW MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkflowStatus {
    pub id: i64,
    pub content_type: String,
    pub content_id: i64,
    pub workflow_id: Option<i64>,
    pub current_stage: String,
    pub assigned_to: Option<i64>,
    pub assigned_by: Option<i64>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Get or create workflow status for content
pub async fn get_or_create_workflow_status(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
) -> Result<WorkflowStatus> {
    // Try to get existing status
    let existing: Option<WorkflowStatus> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, workflow_id, current_stage,
               assigned_to, assigned_by, assigned_at, due_date, priority,
               notes, created_at, updated_at
        FROM content_workflow_status
        WHERE content_type = $1 AND content_id = $2
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .fetch_optional(pool)
    .await?;

    if let Some(status) = existing {
        return Ok(status);
    }

    // Create new status
    let status: WorkflowStatus = sqlx::query_as(
        r#"
        INSERT INTO content_workflow_status (content_type, content_id, current_stage)
        VALUES ($1, $2, 'draft')
        RETURNING id, content_type, content_id, workflow_id, current_stage,
                  assigned_to, assigned_by, assigned_at, due_date, priority,
                  notes, created_at, updated_at
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(status)
}

/// Transition workflow to a new stage
pub async fn transition_workflow(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    to_stage: &str,
    transitioned_by: i64,
    comment: Option<&str>,
) -> Result<WorkflowStatus> {
    let status = get_or_create_workflow_status(pool, content_type, content_id).await?;
    let from_stage = status.current_stage.clone();

    // Update status
    let updated: WorkflowStatus = sqlx::query_as(
        r#"
        UPDATE content_workflow_status
        SET current_stage = $1, updated_at = NOW()
        WHERE content_type = $2 AND content_id = $3
        RETURNING id, content_type, content_id, workflow_id, current_stage,
                  assigned_to, assigned_by, assigned_at, due_date, priority,
                  notes, created_at, updated_at
        "#,
    )
    .bind(to_stage)
    .bind(content_type)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    // Log the transition
    sqlx::query(
        r#"
        INSERT INTO workflow_transitions (workflow_status_id, from_stage, to_stage, transitioned_by, comment)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(status.id)
    .bind(&from_stage)
    .bind(to_stage)
    .bind(transitioned_by)
    .bind(comment)
    .execute(pool)
    .await?;

    Ok(updated)
}

/// Assign content to a user for review
pub async fn assign_for_review(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    assigned_to: i64,
    assigned_by: i64,
    due_date: Option<DateTime<Utc>>,
    priority: Option<&str>,
    notes: Option<&str>,
) -> Result<WorkflowStatus> {
    let status: WorkflowStatus = sqlx::query_as(
        r#"
        UPDATE content_workflow_status
        SET assigned_to = $1, assigned_by = $2, assigned_at = NOW(),
            due_date = $3, priority = $4, notes = $5, updated_at = NOW()
        WHERE content_type = $6 AND content_id = $7
        RETURNING id, content_type, content_id, workflow_id, current_stage,
                  assigned_to, assigned_by, assigned_at, due_date, priority,
                  notes, created_at, updated_at
        "#,
    )
    .bind(assigned_to)
    .bind(assigned_by)
    .bind(due_date)
    .bind(priority.unwrap_or("normal"))
    .bind(notes)
    .bind(content_type)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(status)
}

// ═══════════════════════════════════════════════════════════════════════════
// WEBHOOKS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub content_types: Option<Vec<String>>,
    pub is_active: bool,
    pub retry_count: i32,
    pub timeout_seconds: i32,
    pub headers: Option<JsonValue>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Queue a webhook delivery
pub async fn queue_webhook_delivery(
    pool: &PgPool,
    event_type: &str,
    entity_type: &str,
    payload: JsonValue,
) -> Result<i64> {
    // Get active webhooks for this event
    let webhooks: Vec<Webhook> = sqlx::query_as(
        r#"
        SELECT id, name, description, url, secret, events, content_types,
               is_active, retry_count, timeout_seconds, headers, created_by,
               created_at, updated_at
        FROM webhooks
        WHERE is_active = true
          AND $1 = ANY(events)
          AND (content_types IS NULL OR $2 = ANY(content_types))
        "#,
    )
    .bind(event_type)
    .bind(entity_type)
    .fetch_all(pool)
    .await?;

    let mut delivery_count = 0i64;

    for webhook in webhooks {
        sqlx::query(
            r#"
            INSERT INTO webhook_deliveries (webhook_id, event_type, payload, status)
            VALUES ($1, $2, $3, 'pending')
            "#,
        )
        .bind(webhook.id)
        .bind(event_type)
        .bind(&payload)
        .execute(pool)
        .await?;

        delivery_count += 1;
    }

    Ok(delivery_count)
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLISH SCHEDULING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScheduledContent {
    pub id: i64,
    pub content_type: String,
    pub content_id: i64,
    pub scheduled_action: String,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: Option<String>,
    pub payload: Option<JsonValue>,
    pub status: String,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Schedule content for future publication
pub async fn schedule_publish(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    scheduled_at: DateTime<Utc>,
    created_by: i64,
) -> Result<ScheduledContent> {
    let scheduled: ScheduledContent = sqlx::query_as(
        r#"
        INSERT INTO scheduled_content (
            content_type, content_id, scheduled_action, scheduled_at, created_by
        ) VALUES ($1, $2, 'publish', $3, $4)
        ON CONFLICT (content_type, content_id, scheduled_action, status)
        DO UPDATE SET scheduled_at = $3, updated_at = NOW()
        RETURNING id, content_type, content_id, scheduled_action, scheduled_at,
                  timezone, payload, status, executed_at, error_message,
                  created_by, created_at, updated_at
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(scheduled_at)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(scheduled)
}

/// Cancel a scheduled action
pub async fn cancel_scheduled(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    scheduled_action: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE scheduled_content
        SET status = 'cancelled', updated_at = NOW()
        WHERE content_type = $1 AND content_id = $2
          AND scheduled_action = $3 AND status = 'scheduled'
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(scheduled_action)
    .execute(pool)
    .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// PREVIEW TOKENS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreviewToken {
    pub id: i64,
    pub content_type: String,
    pub content_id: i64,
    pub token: Uuid,
    pub expires_at: DateTime<Utc>,
    pub max_views: Option<i32>,
    pub view_count: i32,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

/// Generate a preview token for content
pub async fn generate_preview_token(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    created_by: i64,
    expires_in_hours: i64,
    max_views: Option<i32>,
) -> Result<PreviewToken> {
    let expires_at = Utc::now() + Duration::hours(expires_in_hours);

    let token: PreviewToken = sqlx::query_as(
        r#"
        INSERT INTO preview_tokens (
            content_type, content_id, expires_at, max_views, created_by
        ) VALUES ($1, $2, $3, $4, $5)
        RETURNING id, content_type, content_id, token, expires_at,
                  max_views, view_count, created_by, created_at, last_accessed_at
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(expires_at)
    .bind(max_views)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(token)
}

/// Validate and consume a preview token
pub async fn validate_preview_token(
    pool: &PgPool,
    token: Uuid,
) -> Result<Option<PreviewToken>> {
    // Get and update the token atomically
    let preview_token: Option<PreviewToken> = sqlx::query_as(
        r#"
        UPDATE preview_tokens
        SET view_count = view_count + 1, last_accessed_at = NOW()
        WHERE token = $1
          AND expires_at > NOW()
          AND (max_views IS NULL OR view_count < max_views)
        RETURNING id, content_type, content_id, token, expires_at,
                  max_views, view_count, created_by, created_at, last_accessed_at
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(preview_token)
}

// ═══════════════════════════════════════════════════════════════════════════
// i18n / LOCALIZATION
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Locale {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub native_name: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub rtl: bool,
    pub fallback_locale: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContentTranslation {
    pub id: i64,
    pub content_type: String,
    pub content_id: i64,
    pub locale: String,
    pub translations: JsonValue,
    pub status: String,
    pub translated_by: Option<i64>,
    pub reviewed_by: Option<i64>,
    pub machine_translated: bool,
    pub quality_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Get all active locales
pub async fn get_active_locales(pool: &PgPool) -> Result<Vec<Locale>> {
    let locales: Vec<Locale> = sqlx::query_as(
        r#"
        SELECT id, code, name, native_name, is_default, is_active,
               rtl, fallback_locale, created_at, updated_at
        FROM locales
        WHERE is_active = true
        ORDER BY is_default DESC, name ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(locales)
}

/// Get translations for content
pub async fn get_translations(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
) -> Result<Vec<ContentTranslation>> {
    let translations: Vec<ContentTranslation> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, locale, translations, status,
               translated_by, reviewed_by, machine_translated, quality_score,
               created_at, updated_at
        FROM content_translations
        WHERE content_type = $1 AND content_id = $2
        ORDER BY locale
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .fetch_all(pool)
    .await?;

    Ok(translations)
}

/// Get translation for specific locale
pub async fn get_translation(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    locale: &str,
) -> Result<Option<ContentTranslation>> {
    let translation: Option<ContentTranslation> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, locale, translations, status,
               translated_by, reviewed_by, machine_translated, quality_score,
               created_at, updated_at
        FROM content_translations
        WHERE content_type = $1 AND content_id = $2 AND locale = $3
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(locale)
    .fetch_optional(pool)
    .await?;

    Ok(translation)
}

/// Save or update a translation
pub async fn save_translation(
    pool: &PgPool,
    content_type: &str,
    content_id: i64,
    locale: &str,
    translations: JsonValue,
    translated_by: i64,
    machine_translated: bool,
) -> Result<ContentTranslation> {
    let translation: ContentTranslation = sqlx::query_as(
        r#"
        INSERT INTO content_translations (
            content_type, content_id, locale, translations,
            translated_by, machine_translated
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (content_type, content_id, locale)
        DO UPDATE SET
            translations = $4,
            translated_by = $5,
            machine_translated = $6,
            status = 'draft',
            updated_at = NOW()
        RETURNING id, content_type, content_id, locale, translations, status,
                  translated_by, reviewed_by, machine_translated, quality_score,
                  created_at, updated_at
        "#,
    )
    .bind(content_type)
    .bind(content_id)
    .bind(locale)
    .bind(&translations)
    .bind(translated_by)
    .bind(machine_translated)
    .fetch_one(pool)
    .await?;

    Ok(translation)
}
