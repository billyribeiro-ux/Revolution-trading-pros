//! Public catalog routes (no auth required).
//!
//! - `list_public_indicators` — paginated grid of active indicators
//! - `get_public_indicator` — detail page with preview videos + platform list
//!
//! Money path: `IndicatorListItem.price_cents` is `Option<i64>` and
//! flows through verbatim. The `is_published` / `is_active` filters
//! preserve the original SQL byte-for-byte.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::models::indicator::{
    Indicator, IndicatorListItem, IndicatorQueryParams, IndicatorVideo,
};
use crate::AppState;

/// ICT 7 SECURITY FIX: Fully parameterized queries to prevent SQL injection
pub(super) async fn list_public_indicators(
    State(state): State<AppState>,
    Query(params): Query<IndicatorQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // ICT 7: Validate platform against allowlist
    let valid_platforms = [
        "tradingview",
        "thinkorswim",
        "metatrader",
        "ninjatrader",
        "tradestation",
        "sierrachart",
        "ctrader",
    ];
    let platform = params.platform.as_ref().and_then(|p| {
        let lower = p.to_lowercase();
        if valid_platforms.contains(&lower.as_str()) {
            Some(lower)
        } else {
            None
        }
    });

    // ICT 7: Sanitize search input
    let search = params.search.as_ref().map(|s| {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
            .take(100)
            .collect::<String>()
    });

    // ICT 7: Use parameterized queries - matching actual database schema.
    // Money is integer cents (Migration 061 dropped legacy NUMERIC `price`).
    let indicators: Vec<IndicatorListItem> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, price_cents, is_active, platform, thumbnail, created_at
        FROM indicators
        WHERE is_active = true
        AND ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(params.is_free)
    .bind(&platform)
    .bind(&search)
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // ICT 7: Parameterized count query
    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM indicators
        WHERE is_active = true
        AND ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ",
    )
    .bind(params.is_free)
    .bind(&platform)
    .bind(&search)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicators": indicators,
            "total": total.0,
            "page": page,
            "per_page": per_page
        }
    })))
}

pub(super) async fn get_public_indicator(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: Indicator =
        sqlx::query_as("SELECT * FROM indicators WHERE slug = $1 AND is_published = true")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Database error: {}", e)})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Indicator not found"})),
                )
            })?;

    // Increment view count
    sqlx::query("UPDATE indicators SET view_count = view_count + 1 WHERE id = $1")
        .bind(indicator.id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Get preview videos only (not all videos)
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 AND is_preview = true ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get available platforms (but not actual files)
    let platforms: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT platform FROM indicator_files WHERE indicator_id = $1 AND is_active = true",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator": indicator,
            "preview_videos": videos,
            "available_platforms": platforms.into_iter().map(|p| p.0).collect::<Vec<_>>()
        }
    })))
}
