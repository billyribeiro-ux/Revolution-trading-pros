//! Admin audit-log read endpoint (split from `connections.rs` lines
//! 1464-1536, R20-B maintainability pass, 2026-05-20). Behavior
//! preserved verbatim.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;
use std::collections::HashMap;

use crate::{middleware::admin::AdminUser, AppState};

/// GET /admin/connections/audit-logs - Get connection audit logs
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_audit_logs(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        "Admin fetching connection audit logs"
    );

    let limit: i64 = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(50)
        .min(500);
    let offset: i64 = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    #[derive(Debug, Serialize, FromRow)]
    struct AuditLogEntry {
        id: i64,
        admin_id: i64,
        admin_email: Option<String>,
        action: String,
        entity_type: String,
        entity_id: Option<i64>,
        old_value: Option<String>,
        new_value: Option<String>,
        metadata: Option<serde_json::Value>,
        created_at: DateTime<Utc>,
    }

    let logs: Vec<AuditLogEntry> = sqlx::query_as(
        r"
        SELECT id, admin_id, admin_email, action, entity_type, entity_id,
               old_value, new_value, metadata, created_at
        FROM admin_audit_logs
        WHERE entity_type IN ('service_connection', 'integration_webhook')
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        ",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM admin_audit_logs WHERE entity_type IN ('service_connection', 'integration_webhook')"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": logs,
        "pagination": {
            "total": total,
            "limit": limit,
            "offset": offset
        }
    })))
}
