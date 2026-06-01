//! Indicator CRUD handlers (list / get / create / update / delete / toggle).
//!
//! ICT 7 SECURITY FIX: Fully parameterized queries to prevent SQL injection.
//! Money is integer cents (`price_cents: i64`); Migration 061 drops the
//! legacy NUMERIC `price` column.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::helpers::slugify;
use super::types::{
    CreateIndicatorRequest, IndicatorQueryParams, IndicatorRow, UpdateIndicatorRequest,
};

/// ICT 7 SECURITY FIX: Fully parameterized queries to prevent SQL injection
pub(super) async fn list_indicators(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<IndicatorQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // ICT 7: Validate platform against allowlist to prevent injection
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

    // ICT 7: Sanitize search to alphanumeric and spaces only
    let search = params.search.as_ref().map(|s| {
        s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
            .take(100)
            .collect::<String>()
    });

    // ICT 7: Use fully parameterized query - NO string concatenation.
    // Money is integer cents at the Rust boundary; project explicit columns
    // so the FromRow shape matches IndicatorRow.
    let indicators: Vec<IndicatorRow> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, long_description,
               price_cents,
               is_active, platform, version, download_url, documentation_url,
               thumbnail, screenshots, features, requirements,
               meta_title, meta_description, created_at, updated_at
        FROM indicators
        WHERE ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(params.is_active)
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
        WHERE ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ",
    )
    .bind(params.is_active)
    .bind(&platform)
    .bind(&search)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicators": indicators,
            "total": total.0,
            "page": page,
            "per_page": per_page,
            "total_pages": total_pages
        }
    })))
}

pub(super) async fn get_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let select_cols = r"id, name, slug, description, long_description,
        price_cents,
        is_active, platform, version, download_url, documentation_url,
        thumbnail, screenshots, features, requirements,
        meta_title, meta_description, created_at, updated_at";
    let by_id_sql = format!("SELECT {select_cols} FROM indicators WHERE id = $1");
    let by_slug_sql = format!("SELECT {select_cols} FROM indicators WHERE slug = $1");
    let indicator: IndicatorRow = if let Ok(numeric_id) = id.parse::<i64>() {
        sqlx::query_as(&by_id_sql)
            .bind(numeric_id)
            .fetch_optional(&state.db.pool)
            .await
    } else {
        sqlx::query_as(&by_slug_sql)
            .bind(&id)
            .fetch_optional(&state.db.pool)
            .await
    }
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

    Ok(Json(json!({
        "success": true,
        "data": indicator
    })))
}

pub(super) async fn create_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.clone().unwrap_or_else(|| slugify(&input.name));

    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "An indicator with this slug already exists"})),
        ));
    }

    // Money is integer cents. Migration 061 drops the legacy NUMERIC `price`
    // column; we only write `price_cents` here.
    let indicator: IndicatorRow = sqlx::query_as(
        r"INSERT INTO indicators (
            name, slug, description, long_description,
            price_cents,
            platform, version,
            download_url, documentation_url, thumbnail, features, requirements,
            meta_title, meta_description, is_active
        )
        VALUES ($1, $2, $3, $4,
            $5::BIGINT,
            $6, $7, $8, $9, $10, $11, $12, $13, $14, true)
        RETURNING id, name, slug, description, long_description,
            price_cents,
            is_active, platform, version, download_url, documentation_url,
            thumbnail, screenshots, features, requirements,
            meta_title, meta_description, created_at, updated_at",
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price_cents.unwrap_or(0))
    .bind(&input.platform)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .bind(&input.download_url)
    .bind(&input.documentation_url)
    .bind(&input.thumbnail)
    .bind(&input.features)
    .bind(&input.requirements)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create indicator: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator created successfully",
        "data": indicator
    })))
}

pub(super) async fn update_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Money is integer cents (Migration 061 drops legacy NUMERIC `price`).
    let indicator: IndicatorRow = sqlx::query_as(
        r"UPDATE indicators SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            long_description = COALESCE($4, long_description),
            price_cents = COALESCE($5::BIGINT, price_cents),
            is_active = COALESCE($6, is_active),
            platform = COALESCE($7, platform),
            version = COALESCE($8, version),
            download_url = COALESCE($9, download_url),
            documentation_url = COALESCE($10, documentation_url),
            thumbnail = COALESCE($11, thumbnail),
            features = COALESCE($12, features),
            requirements = COALESCE($13, requirements),
            meta_title = COALESCE($14, meta_title),
            meta_description = COALESCE($15, meta_description),
            updated_at = NOW()
        WHERE id = $16
        RETURNING id, name, slug, description, long_description,
            price_cents,
            is_active, platform, version, download_url, documentation_url,
            thumbnail, screenshots, features, requirements,
            meta_title, meta_description, created_at, updated_at",
    )
    .bind(&input.name)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price_cents)
    .bind(input.is_active)
    .bind(&input.platform)
    .bind(&input.version)
    .bind(&input.download_url)
    .bind(&input.documentation_url)
    .bind(&input.thumbnail)
    .bind(&input.features)
    .bind(&input.requirements)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update indicator: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator updated successfully",
        "data": indicator
    })))
}

pub(super) async fn delete_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM indicators WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete indicator: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator deleted successfully"
    })))
}

pub(super) async fn toggle_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as(
        r"UPDATE indicators
           SET is_active = NOT COALESCE(is_active, false), updated_at = NOW()
           WHERE id = $1
           RETURNING id, name, slug, description, long_description,
               price_cents,
               is_active, platform, version, download_url, documentation_url,
               thumbnail, screenshots, features, requirements,
               meta_title, meta_description, created_at, updated_at",
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to toggle indicator: {}", e)})),
        )
    })?;

    let status = if indicator.is_active.unwrap_or(false) {
        "activated"
    } else {
        "deactivated"
    };

    Ok(Json(json!({
        "success": true,
        "message": format!("Indicator {}", status),
        "data": indicator
    })))
}
