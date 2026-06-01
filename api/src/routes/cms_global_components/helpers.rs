//! Authorization checks and shared utilities for the global components routes.

use axum::http::StatusCode;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError};

use super::types::sqlx_err;

// ============================================================================
// AUTHORIZATION HELPERS
// ============================================================================

/// Check if user has CMS admin privileges
#[allow(clippy::result_large_err)]
pub(super) fn require_cms_admin(user: &User) -> Result<(), ApiError> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin" | "developer") {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Admin access required").with_code("FORBIDDEN"))
    }
}

/// Check if user has CMS editor privileges
#[allow(clippy::result_large_err)]
pub(super) fn require_cms_editor(user: &User) -> Result<(), ApiError> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing" | "developer"
    ) {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Editor access required").with_code("FORBIDDEN"))
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate a URL-friendly slug from a name
pub(super) fn generate_slug(name: &str) -> String {
    slug::slugify(name)
}

/// Ensure slug uniqueness by appending a suffix if needed
pub(super) async fn ensure_unique_slug(
    pool: &sqlx::PgPool,
    base_slug: &str,
    exclude_id: Option<Uuid>,
) -> Result<String, ApiError> {
    let mut slug = base_slug.to_string();
    let mut suffix = 0;

    loop {
        let exists: bool = match exclude_id {
            Some(id) => {
                let result: (bool,) = sqlx::query_as(
                    r"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_global_components
                        WHERE slug = $1 AND id != $2 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(&slug)
                .bind(id)
                .fetch_one(pool)
                .await
                .map_err(sqlx_err)?;
                result.0
            }
            None => {
                let result: (bool,) = sqlx::query_as(
                    r"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_global_components
                        WHERE slug = $1 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(&slug)
                .fetch_one(pool)
                .await
                .map_err(sqlx_err)?;
                result.0
            }
        };

        if !exists {
            return Ok(slug);
        }

        suffix += 1;
        slug = format!("{base_slug}-{suffix}");
    }
}

/// Get CMS user ID from platform user
pub(super) async fn get_cms_user_id(pool: &sqlx::PgPool, user_id: i64) -> Option<Uuid> {
    sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .ok()?
}

/// Create a version history record
pub(super) async fn create_version_record(
    pool: &sqlx::PgPool,
    component_id: Uuid,
    version: i32,
    component_data: &JsonValue,
    change_message: Option<&str>,
    created_by: Option<Uuid>,
) -> Result<(), ApiError> {
    sqlx::query(
        r"
        INSERT INTO cms_global_component_versions (
            id, component_id, version, component_data, change_message, created_by, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        ",
    )
    .bind(Uuid::new_v4())
    .bind(component_id)
    .bind(version)
    .bind(component_data)
    .bind(change_message)
    .bind(created_by)
    .execute(pool)
    .await
    .map_err(sqlx_err)?;

    Ok(())
}
