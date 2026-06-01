//! Audit-log helper (split from `connections.rs` lines 154-188,
//! R20-B maintainability pass, 2026-05-20). Behavior preserved.

/// Insert one row into `admin_audit_logs`. Best-effort: errors are
/// swallowed because the caller is on the request hot path and we
/// never want audit failures to surface as 5xx.
#[allow(clippy::too_many_arguments)]
pub(super) async fn log_connection_audit(
    pool: &sqlx::PgPool,
    admin_id: i64,
    admin_email: &str,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    old_value: Option<serde_json::Value>,
    new_value: Option<serde_json::Value>,
    metadata: Option<serde_json::Value>,
) {
    let _ = sqlx::query(
        r"
        INSERT INTO admin_audit_logs (
            admin_id, admin_email, action, entity_type, entity_id,
            old_value, new_value, metadata, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6::text, $7::text, $8, NOW())
        ",
    )
    .bind(admin_id)
    .bind(admin_email)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(old_value.map(|v| v.to_string()))
    .bind(new_value.map(|v| v.to_string()))
    .bind(metadata)
    .execute(pool)
    .await;
}
