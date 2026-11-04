//! Authorization + slug utilities shared across CMS Datasources handlers.

use axum::http::StatusCode;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError};

// ============================================================================
// AUTHORIZATION HELPERS
// ============================================================================

/// Check if user has CMS admin privileges
pub(super) fn require_cms_admin(user: &User) -> Result<(), ApiError> {
    if crate::middleware::admin::is_admin_role(user.role.as_deref()) {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Admin access required").with_code("FORBIDDEN"))
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate a URL-friendly slug from a name
pub(super) fn generate_slug(name: &str) -> String {
    slug::slugify(name)
}

/// Ensure slug uniqueness
pub(super) async fn ensure_unique_slug(
    pool: &sqlx::PgPool,
    base_slug: &str,
    exclude_id: Option<Uuid>,
) -> Result<String, ApiError> {
    if base_slug.is_empty()
        || base_slug.len() > 100
        || !base_slug
            .bytes()
            .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == b'-')
    {
        return Err(ApiError::validation_error(
            "Slug must contain 1–100 lowercase letters, digits or hyphens",
        ));
    }
    let mut slug = base_slug.to_string();
    let mut suffix = 0;

    loop {
        let exists: bool = match exclude_id {
            Some(id) => {
                let result: (bool,) = sqlx::query_as(
                    r"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_datasources
                        WHERE slug = $1 AND id != $2
                    )
                    ",
                )
                .bind(&slug)
                .bind(id)
                .fetch_one(pool)
                .await
                .map_err(ApiError::from)?;
                result.0
            }
            None => {
                let result: (bool,) = sqlx::query_as(
                    r"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_datasources
                        WHERE slug = $1
                    )
                    ",
                )
                .bind(&slug)
                .fetch_one(pool)
                .await
                .map_err(ApiError::from)?;
                result.0
            }
        };

        if !exists {
            return Ok(slug);
        }

        suffix += 1;
        let ending = format!("-{suffix}");
        let prefix = &base_slug[..base_slug.len().min(100 - ending.len())];
        slug = format!("{}{ending}", prefix.trim_end_matches('-'));
    }
}

pub(super) fn pagination(
    limit: Option<i64>,
    offset: Option<i64>,
    default: i64,
    max: i64,
) -> Result<(i64, i64), ApiError> {
    let limit = limit.unwrap_or(default);
    let offset = offset.unwrap_or(0);
    if !(1..=max).contains(&limit) || offset < 0 {
        return Err(ApiError::validation_error(&format!(
            "Limit must be 1–{max} and offset must not be negative"
        )));
    }
    Ok((limit, offset))
}

pub(super) fn text(value: &str, field: &str, max: usize, required: bool) -> Result<(), ApiError> {
    if value.contains('\0') || value.chars().count() > max || required && value.trim().is_empty() {
        return Err(ApiError::validation_error(&format!(
            "{field} must contain {}–{max} characters without null characters",
            if required { 1 } else { 0 }
        )));
    }
    Ok(())
}
pub(super) fn metadata(value: &serde_json::Value) -> Result<(), ApiError> {
    fn valid(value: &serde_json::Value, depth: usize) -> bool {
        if depth > 32 {
            return false;
        }
        match value {
            serde_json::Value::String(value) => !value.contains('\0'),
            serde_json::Value::Array(values) => values.iter().all(|value| valid(value, depth + 1)),
            serde_json::Value::Object(values) => values
                .iter()
                .all(|(key, value)| !key.contains('\0') && valid(value, depth + 1)),
            _ => true,
        }
    }
    if !value.is_object() || value.to_string().len() > 65536 || !valid(value, 0) {
        return Err(ApiError::validation_error(
            "Metadata must be an object of at most 64 KiB without null characters or excessive nesting",
        ));
    }
    Ok(())
}
pub(super) async fn require_writable(
    pool: &sqlx::PgPool,
    id: Uuid,
    user: &User,
) -> Result<(), ApiError> {
    let flags: Option<(bool, bool)> = sqlx::query_as(
        "SELECT is_locked,is_system FROM cms_datasources WHERE id=$1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        tracing::error!(%error,"Unable to check datasource access");
        ApiError::database_error("Unable to check datasource access")
    })?;
    let (locked, system) = flags.ok_or_else(|| ApiError::not_found("Datasource not found"))?;
    if locked || system {
        require_cms_admin(user)?;
    }
    Ok(())
}
