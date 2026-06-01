//! CMS Assets — library statistics & tag aggregation
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   GET /stats — DAM library totals (size, by-type counts, recent/unused)
//!   GET /tags  — distinct tags across all non-deleted assets

use axum::{extract::State, Json};

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::{AssetStats, TypeStats};
use super::helpers::format_bytes;

/// GET /cms/assets/stats - Get asset library statistics
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn get_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<AssetStats>, ApiError> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cms_assets WHERE deleted_at IS NULL")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    let total_size: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(file_size), 0) FROM cms_assets WHERE deleted_at IS NULL",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let images: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'image/%' AND deleted_at IS NULL",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let videos: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'video/%' AND deleted_at IS NULL",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let audio: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'audio/%' AND deleted_at IS NULL",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let documents: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE mime_type LIKE 'application/%' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let recent: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE created_at > NOW() - INTERVAL '7 days' AND deleted_at IS NULL"
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let unused: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM cms_assets WHERE usage_count = 0 AND deleted_at IS NULL",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    Ok(Json(AssetStats {
        total_assets: total,
        total_size,
        total_size_formatted: format_bytes(total_size),
        by_type: TypeStats {
            images,
            videos,
            audio,
            documents,
            other: total - images - videos - audio - documents,
        },
        recent_uploads: recent,
        unused_assets: unused,
    }))
}

/// GET /cms/assets/tags - Get all unique tags
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn get_all_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<String>>, ApiError> {
    let tags: Vec<(String,)> = sqlx::query_as(
        r"
        SELECT DISTINCT unnest(tags) as tag
        FROM cms_assets
        WHERE deleted_at IS NULL AND tags IS NOT NULL
        ORDER BY tag
        ",
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(tags.into_iter().map(|(t,)| t).collect()))
}
