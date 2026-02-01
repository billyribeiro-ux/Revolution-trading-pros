//! Indicator Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full-service indicator management matching existing database schema.
//! Database uses BIGINT id, not UUID.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPES - Match actual database schema (BIGINT id)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct IndicatorQueryParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIndicatorRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIndicatorRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7 SECURITY FIX: Fully parameterized queries to prevent SQL injection
async fn list_indicators(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<IndicatorQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * per_page;

    // ICT 7: Validate platform against allowlist to prevent injection
    let valid_platforms = ["tradingview", "thinkorswim", "metatrader", "ninjatrader", "tradestation", "sierrachart", "ctrader"];
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

    // ICT 7: Use fully parameterized query - NO string concatenation
    let indicators: Vec<IndicatorRow> = sqlx::query_as(
        r#"
        SELECT * FROM indicators
        WHERE ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
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
        r#"
        SELECT COUNT(*) FROM indicators
        WHERE ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        "#,
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

async fn get_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = if let Ok(numeric_id) = id.parse::<i64>() {
        sqlx::query_as("SELECT * FROM indicators WHERE id = $1")
            .bind(numeric_id)
            .fetch_optional(&state.db.pool)
            .await
    } else {
        sqlx::query_as("SELECT * FROM indicators WHERE slug = $1")
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

async fn create_indicator(
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

    let indicator: IndicatorRow = sqlx::query_as(
        r#"INSERT INTO indicators (
            name, slug, description, long_description, price, platform, version,
            download_url, documentation_url, thumbnail, features, requirements,
            meta_title, meta_description, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, true)
        RETURNING *"#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price.unwrap_or(0.0))
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

async fn update_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as(
        r#"UPDATE indicators SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            long_description = COALESCE($4, long_description),
            price = COALESCE($5, price),
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
        RETURNING *"#,
    )
    .bind(&input.name)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price)
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

async fn delete_indicator(
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

async fn toggle_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as(
        r#"UPDATE indicators 
           SET is_active = NOT COALESCE(is_active, false), updated_at = NOW()
           WHERE id = $1
           RETURNING *"#,
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

// ═══════════════════════════════════════════════════════════════════════════════════
// FILE MANAGEMENT ENDPOINTS (ICT 7 Grade - February 2026)
// Multi-platform file downloads with version management
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorFileRow {
    pub id: i32,
    pub indicator_id: i64,
    pub file_name: String,
    pub original_name: Option<String>,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub checksum_sha256: Option<String>,
    pub platform: String,
    pub platform_version: Option<String>,
    pub storage_provider: Option<String>,
    pub storage_bucket: Option<String>,
    pub storage_key: Option<String>,
    pub cdn_url: Option<String>,
    pub version: Option<String>,
    pub is_current_version: Option<bool>,
    pub changelog: Option<String>,
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
    pub download_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFileRequest {
    pub file_name: String,
    pub original_name: Option<String>,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub platform: String,
    pub platform_version: Option<String>,
    pub storage_key: Option<String>,
    pub cdn_url: Option<String>,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFileRequest {
    pub display_name: Option<String>,
    pub display_order: Option<i32>,
    pub platform_version: Option<String>,
    pub version: Option<String>,
    pub changelog: Option<String>,
    pub is_current_version: Option<bool>,
    pub is_active: Option<bool>,
}

/// ICT 7: List all files for an indicator
async fn list_indicator_files(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let files: Vec<IndicatorFileRow> = sqlx::query_as(
        r#"
        SELECT * FROM indicator_files
        WHERE indicator_id = $1
        ORDER BY platform, display_order, created_at DESC
        "#,
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": files
    })))
}

/// ICT 7: Add a new file to an indicator
async fn create_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify indicator exists
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE id = $1")
        .bind(indicator_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if indicator.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Indicator not found"})),
        ));
    }

    // If this is marked as current version, unmark other files for same platform
    if input.version.is_some() {
        let _ = sqlx::query(
            r#"
            UPDATE indicator_files
            SET is_current_version = false
            WHERE indicator_id = $1 AND platform = $2
            "#,
        )
        .bind(indicator_id)
        .bind(&input.platform)
        .execute(&state.db.pool)
        .await;
    }

    let file: IndicatorFileRow = sqlx::query_as(
        r#"
        INSERT INTO indicator_files (
            indicator_id, file_name, original_name, file_path, file_size_bytes,
            file_type, mime_type, platform, platform_version, storage_key, cdn_url,
            version, changelog, display_name, display_order, is_current_version, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, true, true)
        RETURNING *
        "#,
    )
    .bind(indicator_id)
    .bind(&input.file_name)
    .bind(&input.original_name)
    .bind(&input.file_path)
    .bind(input.file_size_bytes)
    .bind(&input.file_type)
    .bind(&input.mime_type)
    .bind(&input.platform)
    .bind(&input.platform_version)
    .bind(&input.storage_key)
    .bind(&input.cdn_url)
    .bind(&input.version)
    .bind(&input.changelog)
    .bind(&input.display_name)
    .bind(input.display_order.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create file: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "File added successfully",
        "data": file
    })))
}

/// ICT 7: Update an indicator file
async fn update_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i32)>,
    Json(input): Json<UpdateFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // If marking as current version, unmark others first
    if input.is_current_version == Some(true) {
        let file: Option<(String,)> = sqlx::query_as(
            "SELECT platform FROM indicator_files WHERE id = $1 AND indicator_id = $2",
        )
        .bind(file_id)
        .bind(indicator_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

        if let Some((platform,)) = file {
            let _ = sqlx::query(
                r#"
                UPDATE indicator_files
                SET is_current_version = false
                WHERE indicator_id = $1 AND platform = $2 AND id != $3
                "#,
            )
            .bind(indicator_id)
            .bind(&platform)
            .bind(file_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    let file: IndicatorFileRow = sqlx::query_as(
        r#"
        UPDATE indicator_files SET
            display_name = COALESCE($1, display_name),
            display_order = COALESCE($2, display_order),
            platform_version = COALESCE($3, platform_version),
            version = COALESCE($4, version),
            changelog = COALESCE($5, changelog),
            is_current_version = COALESCE($6, is_current_version),
            is_active = COALESCE($7, is_active),
            updated_at = NOW()
        WHERE id = $8 AND indicator_id = $9
        RETURNING *
        "#,
    )
    .bind(&input.display_name)
    .bind(input.display_order)
    .bind(&input.platform_version)
    .bind(&input.version)
    .bind(&input.changelog)
    .bind(input.is_current_version)
    .bind(input.is_active)
    .bind(file_id)
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update file: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "File updated successfully",
        "data": file
    })))
}

/// ICT 7: Delete an indicator file
async fn delete_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM indicator_files WHERE id = $1 AND indicator_id = $2")
        .bind(file_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete file: {}", e)})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "File deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO MANAGEMENT ENDPOINTS (ICT 7 Grade)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorVideoRow {
    pub id: i32,
    pub indicator_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<String>,
    pub embed_url: Option<String>,
    pub play_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<String>,
    pub embed_url: Option<String>,
    pub play_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub display_order: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_preview: Option<bool>,
}

/// ICT 7: List all videos for an indicator
async fn list_indicator_videos(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let videos: Vec<IndicatorVideoRow> = sqlx::query_as(
        r#"
        SELECT * FROM indicator_videos
        WHERE indicator_id = $1
        ORDER BY display_order, id
        "#,
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": videos
    })))
}

/// ICT 7: Add a new video to an indicator
async fn create_indicator_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: IndicatorVideoRow = sqlx::query_as(
        r#"
        INSERT INTO indicator_videos (
            indicator_id, title, description, bunny_video_guid, bunny_library_id,
            embed_url, play_url, thumbnail_url, duration_seconds, display_order,
            is_featured, is_preview
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(indicator_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.bunny_video_guid)
    .bind(&input.bunny_library_id)
    .bind(&input.embed_url)
    .bind(&input.play_url)
    .bind(&input.thumbnail_url)
    .bind(input.duration_seconds)
    .bind(input.display_order.unwrap_or(0))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_preview.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create video: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Video added successfully",
        "data": video
    })))
}

/// ICT 7: Delete an indicator video
async fn delete_indicator_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, video_id)): Path<(i64, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM indicator_videos WHERE id = $1 AND indicator_id = $2")
        .bind(video_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete video: {}", e)})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Video deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOAD ANALYTICS ENDPOINT (ICT 7 Grade)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7: Get download analytics for an indicator
async fn get_download_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Total downloads
    let total_downloads: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(COALESCE(download_count, 0)), 0) FROM indicator_files WHERE indicator_id = $1",
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Downloads by platform
    let by_platform: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT platform, COALESCE(SUM(COALESCE(download_count, 0)), 0) as count
        FROM indicator_files
        WHERE indicator_id = $1
        GROUP BY platform
        ORDER BY count DESC
        "#,
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator_id": indicator_id,
            "total_downloads": total_downloads.0,
            "by_platform": by_platform.into_iter().map(|(p, c)| json!({"platform": p, "downloads": c})).collect::<Vec<_>>()
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Indicator CRUD
        .route("/", get(list_indicators).post(create_indicator))
        .route(
            "/:id",
            get(get_indicator)
                .put(update_indicator)
                .delete(delete_indicator),
        )
        .route("/:id/toggle", post(toggle_indicator))
        // ICT 7: File management
        .route("/:id/files", get(list_indicator_files).post(create_indicator_file))
        .route("/:id/files/:file_id", put(update_indicator_file).delete(delete_indicator_file))
        // ICT 7: Video management
        .route("/:id/videos", get(list_indicator_videos).post(create_indicator_video))
        .route("/:id/videos/:video_id", delete(delete_indicator_video))
        // ICT 7: Analytics
        .route("/:id/analytics", get(get_download_analytics))
}
