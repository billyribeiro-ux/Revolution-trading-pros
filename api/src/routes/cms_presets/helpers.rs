//! Helper utilities shared across CMS preset handlers: RBAC checks,
//! slug generation, slug uniqueness enforcement, and the shared
//! `ApiResult` / `ApiResultEmpty` type aliases.
//!
//! Split from the original `cms_presets.rs` (R25-B). Pure structural
//! move — behaviour and signatures are unchanged.

use axum::{http::StatusCode, Json};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError};

// ============================================================================
// TYPE ALIASES
// ============================================================================

pub(super) type ApiResult<T> = Result<Json<T>, ApiError>;
pub(super) type ApiResultEmpty = Result<Json<JsonValue>, ApiError>;

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
    block_type: &str,
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
                        SELECT 1 FROM cms_presets
                        WHERE block_type = $1 AND slug = $2 AND id != $3 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(block_type)
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
                        SELECT 1 FROM cms_presets
                        WHERE block_type = $1 AND slug = $2 AND deleted_at IS NULL
                    )
                    ",
                )
                .bind(block_type)
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
