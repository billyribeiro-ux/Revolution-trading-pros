//! CMS Assets — usage tracking
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   GET  /:id/usage         — list content rows referencing this asset
//!   POST /:id/track-usage   — upsert usage record + bump usage_count

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::AssetUsage;

/// GET /cms/assets/:id/usage - Get content using this asset
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn get_asset_usage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<AssetUsage>>, ApiError> {
    let usage: Vec<AssetUsage> = sqlx::query_as(
        r"
        SELECT u.id, u.asset_id, u.content_type, u.content_id,
               c.title as content_title, c.slug as content_slug,
               u.field_name, u.created_at
        FROM cms_asset_usage u
        LEFT JOIN cms_content c ON c.id = u.content_id
        WHERE u.asset_id = $1
        ORDER BY u.created_at DESC
        ",
    )
    .bind(id)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(usage))
}

/// POST /cms/assets/:id/track-usage - Track asset usage in content
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn track_usage(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<JsonValue>, ApiError> {
    let content_type = payload
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let content_id = payload
        .get("content_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());
    let field_name = payload
        .get("field_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    if let Some(content_id) = content_id {
        // Upsert usage record
        sqlx::query(
            r"
            INSERT INTO cms_asset_usage (id, asset_id, content_type, content_id, field_name, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (asset_id, content_id, field_name) DO UPDATE SET created_at = NOW()
            ",
        )
        .bind(Uuid::new_v4())
        .bind(id)
        .bind(content_type)
        .bind(content_id)
        .bind(field_name)
        .execute(state.db.pool())
        .await
        .ok();

        // Update usage count
        sqlx::query(
            "UPDATE cms_assets SET usage_count = usage_count + 1, last_used_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(state.db.pool())
        .await
        .ok();
    }

    Ok(Json(json!({ "success": true })))
}
