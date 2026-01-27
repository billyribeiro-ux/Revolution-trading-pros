//! CMS Audit Logging Service - Apple ICT 7 Principal Engineer Grade
//!
//! Compliance-grade audit trail for all CMS operations.
//! Tracks user actions, changes, and system events for:
//! - Regulatory compliance (SOC 2, GDPR, HIPAA)
//! - Security forensics
//! - Change tracking
//! - User accountability
//!
//! @version 1.0.0 - January 27, 2026

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CmsAuditLog {
    pub id: Uuid,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug)]
pub struct AuditContext {
    pub user_id: Option<i64>,
    pub user_email: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUDIT LOGGING FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Log an audit event
#[allow(clippy::too_many_arguments)]
pub async fn log_audit(
    pool: &PgPool,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    context: &AuditContext,
    old_values: Option<JsonValue>,
    new_values: Option<JsonValue>,
    metadata: Option<JsonValue>,
) -> Result<Uuid> {
    let audit_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO cms_audit_logs (
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
pub async fn get_audit_logs(pool: &PgPool, query: &AuditLogQuery) -> Result<Vec<CmsAuditLog>> {
    let mut sql = String::from(
        r#"
        SELECT id, action, entity_type, entity_id, user_id, user_email,
               old_values, new_values, metadata, ip_address, user_agent, created_at
        FROM cms_audit_logs
        WHERE 1=1
        "#,
    );

    let mut bind_count = 0;

    if query.entity_type.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND entity_type = ${}", bind_count));
    }

    if query.entity_id.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND entity_id = ${}", bind_count));
    }

    if query.user_id.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND user_id = ${}", bind_count));
    }

    if query.action.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND action = ${}", bind_count));
    }

    if query.start_date.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND created_at >= ${}", bind_count));
    }

    if query.end_date.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND created_at <= ${}", bind_count));
    }

    sql.push_str(" ORDER BY created_at DESC");

    let limit = query.limit.unwrap_or(100).min(1000);
    bind_count += 1;
    sql.push_str(&format!(" LIMIT ${}", bind_count));

    let offset = query.offset.unwrap_or(0);
    bind_count += 1;
    sql.push_str(&format!(" OFFSET ${}", bind_count));

    let mut query_builder = sqlx::query_as::<_, CmsAuditLog>(&sql);

    if let Some(ref entity_type) = query.entity_type {
        query_builder = query_builder.bind(entity_type);
    }
    if let Some(entity_id) = query.entity_id {
        query_builder = query_builder.bind(entity_id);
    }
    if let Some(user_id) = query.user_id {
        query_builder = query_builder.bind(user_id);
    }
    if let Some(ref action) = query.action {
        query_builder = query_builder.bind(action);
    }
    if let Some(start_date) = query.start_date {
        query_builder = query_builder.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        query_builder = query_builder.bind(end_date);
    }

    query_builder = query_builder.bind(limit).bind(offset);

    let logs = query_builder.fetch_all(pool).await?;

    Ok(logs)
}

/// Get audit log count for pagination
pub async fn get_audit_log_count(pool: &PgPool, query: &AuditLogQuery) -> Result<i64> {
    let mut sql = String::from("SELECT COUNT(*) FROM cms_audit_logs WHERE 1=1");
    let mut bind_count = 0;

    if query.entity_type.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND entity_type = ${}", bind_count));
    }

    if query.entity_id.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND entity_id = ${}", bind_count));
    }

    if query.user_id.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND user_id = ${}", bind_count));
    }

    if query.action.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND action = ${}", bind_count));
    }

    if query.start_date.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND created_at >= ${}", bind_count));
    }

    if query.end_date.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND created_at <= ${}", bind_count));
    }

    let mut query_builder = sqlx::query_scalar::<_, i64>(&sql);

    if let Some(ref entity_type) = query.entity_type {
        query_builder = query_builder.bind(entity_type);
    }
    if let Some(entity_id) = query.entity_id {
        query_builder = query_builder.bind(entity_id);
    }
    if let Some(user_id) = query.user_id {
        query_builder = query_builder.bind(user_id);
    }
    if let Some(ref action) = query.action {
        query_builder = query_builder.bind(action);
    }
    if let Some(start_date) = query.start_date {
        query_builder = query_builder.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        query_builder = query_builder.bind(end_date);
    }

    let count = query_builder.fetch_one(pool).await?;

    Ok(count)
}

/// Clean up old audit logs (for data retention policies)
pub async fn cleanup_old_audit_logs(pool: &PgPool, days_to_keep: i32) -> Result<i64> {
    let result = sqlx::query(
        r#"
        DELETE FROM cms_audit_logs
        WHERE created_at < NOW() - INTERVAL '1 day' * $1
        "#,
    )
    .bind(days_to_keep)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONVENIENCE FUNCTIONS FOR COMMON AUDIT EVENTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Log content creation
pub async fn log_content_created(
    pool: &PgPool,
    content_id: Uuid,
    content_data: JsonValue,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "content.created",
        "cms_content",
        Some(content_id),
        context,
        None,
        Some(content_data),
        None,
    )
    .await
}

/// Log content update
pub async fn log_content_updated(
    pool: &PgPool,
    content_id: Uuid,
    old_data: JsonValue,
    new_data: JsonValue,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "content.updated",
        "cms_content",
        Some(content_id),
        context,
        Some(old_data),
        Some(new_data),
        None,
    )
    .await
}

/// Log content deletion
pub async fn log_content_deleted(
    pool: &PgPool,
    content_id: Uuid,
    content_data: JsonValue,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "content.deleted",
        "cms_content",
        Some(content_id),
        context,
        Some(content_data),
        None,
        None,
    )
    .await
}

/// Log content status change
pub async fn log_content_status_changed(
    pool: &PgPool,
    content_id: Uuid,
    old_status: &str,
    new_status: &str,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "content.status_changed",
        "cms_content",
        Some(content_id),
        context,
        Some(serde_json::json!({ "status": old_status })),
        Some(serde_json::json!({ "status": new_status })),
        None,
    )
    .await
}

/// Log asset upload
pub async fn log_asset_uploaded(
    pool: &PgPool,
    asset_id: Uuid,
    asset_data: JsonValue,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "asset.uploaded",
        "cms_asset",
        Some(asset_id),
        context,
        None,
        Some(asset_data),
        None,
    )
    .await
}

/// Log asset deletion
pub async fn log_asset_deleted(
    pool: &PgPool,
    asset_id: Uuid,
    asset_data: JsonValue,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "asset.deleted",
        "cms_asset",
        Some(asset_id),
        context,
        Some(asset_data),
        None,
        None,
    )
    .await
}

/// Log workflow transition
pub async fn log_workflow_transition(
    pool: &PgPool,
    content_id: Uuid,
    from_stage: &str,
    to_stage: &str,
    context: &AuditContext,
) -> Result<Uuid> {
    log_audit(
        pool,
        "workflow.transitioned",
        "cms_workflow",
        Some(content_id),
        context,
        Some(serde_json::json!({ "stage": from_stage })),
        Some(serde_json::json!({ "stage": to_stage })),
        None,
    )
    .await
}
