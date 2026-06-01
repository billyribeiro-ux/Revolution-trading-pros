//! Authorization + slug utilities shared across CMS Datasources handlers.

use axum::http::StatusCode;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError};

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

/// Ensure slug uniqueness
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
                        SELECT 1 FROM cms_datasources
                        WHERE slug = $1 AND id != $2 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(&slug)
                .bind(id)
                .fetch_one(pool)
                .await
                .map_err(|e: sqlx::Error| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })?;
                result.0
            }
            None => {
                let result: (bool,) = sqlx::query_as(
                    r"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_datasources
                        WHERE slug = $1 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(&slug)
                .fetch_one(pool)
                .await
                .map_err(|e: sqlx::Error| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })?;
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
