//! Organization shared helpers — audit logging + slug generation.
//!
//! R27-B3 (2026-05-20): extracted verbatim from the original
//! `organization.rs`. Both functions are crate-private (`pub(super)`)
//! and reused across the profile / teams / departments handler files.
//! SQL, parameter order, and column list preserved byte-for-byte —
//! the audit-log shape is observed by the `admin_audit_logs` table and
//! by downstream admin-audit consumers.

#[allow(clippy::too_many_arguments)]
pub(super) async fn log_organization_audit(
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

/// Generate URL-safe slug from name
pub(super) fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
