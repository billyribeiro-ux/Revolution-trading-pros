//! Enhanced Indicator Admin Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Complete indicator management: CRUD, platforms, files, videos, docs, TradingView access

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use slug::slugify;
use tracing::{error, info, warn};

use crate::models::indicator_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR CRUD ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all indicators with filters and pagination
async fn list_indicators(
    State(state): State<AppState>,
    Query(query): Query<IndicatorListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        r#"SELECT * FROM indicators_enhanced WHERE deleted_at IS NULL"#
    );
    let mut count_sql = String::from(
        r#"SELECT COUNT(*) FROM indicators_enhanced WHERE deleted_at IS NULL"#
    );

    let mut conditions: Vec<String> = vec![];

    if let Some(ref category) = query.category {
        conditions.push(format!("category = '{}'", category));
    }
    if let Some(is_published) = query.is_published {
        conditions.push(format!("is_published = {}", is_published));
    }
    if let Some(is_featured) = query.is_featured {
        conditions.push(format!("is_featured = {}", is_featured));
    }
    if let Some(has_tv) = query.has_tradingview {
        conditions.push(format!("has_tradingview_access = {}", has_tv));
    }
    if let Some(platform_id) = query.platform_id {
        conditions.push(format!("supported_platforms @> '[{}]'::jsonb", platform_id));
    }
    if let Some(ref search) = query.search {
        conditions.push(format!(
            "(name ILIKE '%{}%' OR description ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    if !conditions.is_empty() {
        let where_clause = format!(" AND {}", conditions.join(" AND "));
        sql.push_str(&where_clause);
        count_sql.push_str(&where_clause);
    }

    sql.push_str(&format!(
        " ORDER BY is_featured DESC, created_at DESC LIMIT {} OFFSET {}",
        per_page, offset
    ));

    let indicators: Vec<IndicatorEnhanced> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to list indicators: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let indicator_responses: Vec<serde_json::Value> = indicators
        .iter()
        .map(|i| {
            json!({
                "id": i.id,
                "name": i.name,
                "slug": i.slug,
                "subtitle": i.subtitle,
                "short_description": i.short_description,
                "thumbnail_url": i.thumbnail_url,
                "category": i.category,
                "version": i.version,
                "is_published": i.is_published,
                "is_featured": i.is_featured,
                "is_free": i.is_free,
                "total_downloads": i.total_downloads,
                "has_tradingview_access": i.has_tradingview_access,
                "created_at": i.created_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(json!({
        "indicators": indicator_responses,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get a single indicator with all related data
async fn get_indicator(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorEnhanced = sqlx::query_as(
        "SELECT * FROM indicators_enhanced WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(indicator_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to get indicator: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?
    .ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(json!({"error": "Indicator not found"})))
    })?;

    // Get videos
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 ORDER BY sort_order"
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get platform files with platform info
    let platform_files: Vec<(IndicatorPlatformFile, String, String, Option<String>)> = sqlx::query_as(
        r#"SELECT f.*, p.name, p.display_name, p.icon_url
           FROM indicator_platform_files f
           JOIN indicator_platforms p ON f.platform_id = p.id
           WHERE f.indicator_id = $1
           ORDER BY p.sort_order, f.created_at DESC"#
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get documentation
    let docs: Vec<IndicatorDocumentation> = sqlx::query_as(
        "SELECT * FROM indicator_documentation WHERE indicator_id = $1 ORDER BY sort_order"
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build responses
    let video_responses: Vec<serde_json::Value> = videos
        .iter()
        .map(|v| json!({
            "id": v.id,
            "title": v.title,
            "description": v.description,
            "video_url": v.video_url,
            "embed_url": get_indicator_embed_url(&v.video_url, &v.bunny_video_guid, v.bunny_library_id),
            "thumbnail_url": v.thumbnail_url,
            "duration_seconds": v.duration_seconds,
            "formatted_duration": v.duration_seconds.map(format_duration_seconds),
            "video_type": v.video_type,
            "sort_order": v.sort_order,
            "is_preview": v.is_preview,
            "is_published": v.is_published,
            "view_count": v.view_count
        }))
        .collect();

    let file_responses: Vec<serde_json::Value> = platform_files
        .iter()
        .map(|(f, platform_name, platform_display, platform_icon)| json!({
            "id": f.id,
            "platform": {
                "id": f.platform_id,
                "name": platform_name,
                "display_name": platform_display,
                "icon_url": platform_icon
            },
            "file_url": f.file_url,
            "file_name": f.file_name,
            "file_size_bytes": f.file_size_bytes,
            "formatted_size": format_file_size(f.file_size_bytes.unwrap_or(0)),
            "version": f.version,
            "version_notes": f.version_notes,
            "installation_notes": f.installation_notes,
            "is_latest": f.is_latest,
            "download_count": f.download_count,
            "created_at": f.created_at.to_rfc3339()
        }))
        .collect();

    let doc_responses: Vec<serde_json::Value> = docs
        .iter()
        .map(|d| json!({
            "id": d.id,
            "title": d.title,
            "doc_type": d.doc_type,
            "content_html": d.content_html,
            "file_url": d.file_url,
            "file_name": d.file_name,
            "sort_order": d.sort_order,
            "is_published": d.is_published
        }))
        .collect();

    let tags = parse_tags(&indicator.tags);
    let supported_platforms = parse_supported_platforms(&indicator.supported_platforms);

    Ok(Json(json!({
        "id": indicator.id,
        "name": indicator.name,
        "slug": indicator.slug,
        "subtitle": indicator.subtitle,
        "description": indicator.description,
        "description_html": indicator.description_html,
        "short_description": indicator.short_description,
        "thumbnail_url": indicator.thumbnail_url,
        "preview_image_url": indicator.preview_image_url,
        "preview_video_url": indicator.preview_video_url,
        "category": indicator.category,
        "tags": tags,
        "version": indicator.version,
        "version_notes": indicator.version_notes,
        "release_date": indicator.release_date.map(|d| d.to_rfc3339()),
        "is_published": indicator.is_published,
        "is_featured": indicator.is_featured,
        "is_free": indicator.is_free,
        "required_plan_id": indicator.required_plan_id,
        "price_cents": indicator.price_cents,
        "total_downloads": indicator.total_downloads,
        "supported_platform_ids": supported_platforms,
        "has_tradingview_access": indicator.has_tradingview_access,
        "tradingview_invite_only": indicator.tradingview_invite_only,
        "videos": video_responses,
        "platform_files": file_responses,
        "documentation": doc_responses,
        "created_at": indicator.created_at.to_rfc3339(),
        "updated_at": indicator.updated_at.to_rfc3339()
    })))
}

/// Create a new indicator
async fn create_indicator(
    State(state): State<AppState>,
    Json(input): Json<CreateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.name);
    let tags_json = input.tags.as_ref().map(|t| serde_json::to_value(t).unwrap_or_default());

    let indicator: IndicatorEnhanced = sqlx::query_as(
        r#"INSERT INTO indicators_enhanced
           (name, slug, subtitle, description, description_html, short_description,
            thumbnail_url, preview_image_url, preview_video_url, category, tags,
            version, version_notes, is_published, is_featured, is_free, required_plan_id,
            price_cents, has_tradingview_access, tradingview_invite_only)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
           RETURNING *"#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.subtitle)
    .bind(&input.description)
    .bind(&input.description_html)
    .bind(&input.short_description)
    .bind(&input.thumbnail_url)
    .bind(&input.preview_image_url)
    .bind(&input.preview_video_url)
    .bind(&input.category)
    .bind(&tags_json)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .bind(&input.version_notes)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_free.unwrap_or(false))
    .bind(input.required_plan_id)
    .bind(input.price_cents)
    .bind(input.has_tradingview_access.unwrap_or(false))
    .bind(input.tradingview_invite_only.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create indicator: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Created indicator: {} ({})", indicator.name, indicator.id);

    Ok(Json(json!({
        "success": true,
        "indicator": {
            "id": indicator.id,
            "name": indicator.name,
            "slug": indicator.slug
        }
    })))
}

/// Update an indicator
async fn update_indicator(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<UpdateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = vec![];

    if let Some(ref name) = input.name {
        updates.push(format!("name = '{}'", name.replace('\'', "''")));
        updates.push(format!("slug = '{}'", slugify(name)));
    }
    if let Some(ref subtitle) = input.subtitle {
        updates.push(format!("subtitle = '{}'", subtitle.replace('\'', "''")));
    }
    if let Some(ref description) = input.description {
        updates.push(format!("description = '{}'", description.replace('\'', "''")));
    }
    if let Some(ref description_html) = input.description_html {
        updates.push(format!("description_html = '{}'", description_html.replace('\'', "''")));
    }
    if let Some(ref short_desc) = input.short_description {
        updates.push(format!("short_description = '{}'", short_desc.replace('\'', "''")));
    }
    if let Some(ref thumbnail_url) = input.thumbnail_url {
        updates.push(format!("thumbnail_url = '{}'", thumbnail_url.replace('\'', "''")));
    }
    if let Some(ref preview_image_url) = input.preview_image_url {
        updates.push(format!("preview_image_url = '{}'", preview_image_url.replace('\'', "''")));
    }
    if let Some(ref preview_video_url) = input.preview_video_url {
        updates.push(format!("preview_video_url = '{}'", preview_video_url.replace('\'', "''")));
    }
    if let Some(ref category) = input.category {
        updates.push(format!("category = '{}'", category.replace('\'', "''")));
    }
    if let Some(ref tags) = input.tags {
        let tags_json = serde_json::to_string(tags).unwrap_or_else(|_| "[]".to_string());
        updates.push(format!("tags = '{}'::jsonb", tags_json));
    }
    if let Some(ref version) = input.version {
        updates.push(format!("version = '{}'", version));
        updates.push("release_date = NOW()".to_string());
    }
    if let Some(ref version_notes) = input.version_notes {
        updates.push(format!("version_notes = '{}'", version_notes.replace('\'', "''")));
    }
    if let Some(is_published) = input.is_published {
        updates.push(format!("is_published = {}", is_published));
    }
    if let Some(is_featured) = input.is_featured {
        updates.push(format!("is_featured = {}", is_featured));
    }
    if let Some(is_free) = input.is_free {
        updates.push(format!("is_free = {}", is_free));
    }
    if let Some(required_plan_id) = input.required_plan_id {
        updates.push(format!("required_plan_id = {}", required_plan_id));
    }
    if let Some(price_cents) = input.price_cents {
        updates.push(format!("price_cents = {}", price_cents));
    }
    if let Some(has_tv) = input.has_tradingview_access {
        updates.push(format!("has_tradingview_access = {}", has_tv));
    }
    if let Some(tv_invite) = input.tradingview_invite_only {
        updates.push(format!("tradingview_invite_only = {}", tv_invite));
    }

    if updates.is_empty() {
        return Ok(Json(json!({"success": true, "message": "No changes"})));
    }

    updates.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE indicators_enhanced SET {} WHERE id = $1 AND deleted_at IS NULL RETURNING id, name",
        updates.join(", ")
    );

    let result: Option<(i64, String)> = sqlx::query_as(&sql)
        .bind(indicator_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to update indicator: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    match result {
        Some((id, name)) => {
            info!("Updated indicator: {} ({})", name, id);
            Ok(Json(json!({"success": true, "indicator_id": id})))
        }
        None => Err((StatusCode::NOT_FOUND, Json(json!({"error": "Indicator not found"}))))
    }
}

/// Delete an indicator (soft delete)
async fn delete_indicator(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE indicators_enhanced SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to delete indicator: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Indicator not found"}))));
    }

    info!("Soft deleted indicator: {}", indicator_id);
    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PLATFORM ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all platforms
async fn list_platforms(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let platforms: Vec<IndicatorPlatform> = sqlx::query_as(
        "SELECT * FROM indicator_platforms ORDER BY sort_order"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let platform_responses: Vec<serde_json::Value> = platforms
        .iter()
        .map(|p| json!({
            "id": p.id,
            "name": p.name,
            "slug": p.slug,
            "display_name": p.display_name,
            "icon_url": p.icon_url,
            "file_extension": p.file_extension,
            "installation_instructions": p.installation_instructions,
            "is_active": p.is_active
        }))
        .collect();

    Ok(Json(json!({"platforms": platform_responses})))
}

/// Create a new platform
async fn create_platform(
    State(state): State<AppState>,
    Json(input): Json<CreatePlatformRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.name);

    // Get max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM indicator_platforms"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let platform: IndicatorPlatform = sqlx::query_as(
        r#"INSERT INTO indicator_platforms
           (name, slug, display_name, icon_url, file_extension, installation_instructions, sort_order, is_active)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING *"#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.display_name)
    .bind(&input.icon_url)
    .bind(&input.file_extension)
    .bind(&input.installation_instructions)
    .bind(sort_order)
    .bind(input.is_active.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create platform: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Created platform: {} ({})", platform.name, platform.id);

    Ok(Json(json!({
        "success": true,
        "platform": {
            "id": platform.id,
            "name": platform.name,
            "slug": platform.slug
        }
    })))
}

/// Update a platform
async fn update_platform(
    State(state): State<AppState>,
    Path(platform_id): Path<i64>,
    Json(input): Json<UpdatePlatformRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = vec![];

    if let Some(ref name) = input.name {
        updates.push(format!("name = '{}'", name.replace('\'', "''")));
        updates.push(format!("slug = '{}'", slugify(name)));
    }
    if let Some(ref display_name) = input.display_name {
        updates.push(format!("display_name = '{}'", display_name.replace('\'', "''")));
    }
    if let Some(ref icon_url) = input.icon_url {
        updates.push(format!("icon_url = '{}'", icon_url.replace('\'', "''")));
    }
    if let Some(ref file_extension) = input.file_extension {
        updates.push(format!("file_extension = '{}'", file_extension));
    }
    if let Some(ref instructions) = input.installation_instructions {
        updates.push(format!("installation_instructions = '{}'", instructions.replace('\'', "''")));
    }
    if let Some(is_active) = input.is_active {
        updates.push(format!("is_active = {}", is_active));
    }

    if updates.is_empty() {
        return Ok(Json(json!({"success": true, "message": "No changes"})));
    }

    let sql = format!(
        "UPDATE indicator_platforms SET {} WHERE id = $1",
        updates.join(", ")
    );

    sqlx::query(&sql)
        .bind(platform_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to update platform: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create an indicator video
async fn create_video(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateIndicatorVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM indicator_videos WHERE indicator_id = $1"
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let video: IndicatorVideo = sqlx::query_as(
        r#"INSERT INTO indicator_videos
           (indicator_id, title, description, video_url, bunny_video_guid, thumbnail_url,
            video_type, sort_order, is_preview, is_published)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING *"#
    )
    .bind(indicator_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(input.video_type.as_deref().unwrap_or("tutorial"))
    .bind(sort_order)
    .bind(input.is_preview.unwrap_or(false))
    .bind(input.is_published.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create video: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Created indicator video: {} for indicator {}", video.title, indicator_id);

    Ok(Json(json!({
        "success": true,
        "video": {
            "id": video.id,
            "title": video.title,
            "sort_order": video.sort_order
        }
    })))
}

/// Update a video
async fn update_video(
    State(state): State<AppState>,
    Path((indicator_id, video_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateIndicatorVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = vec![];

    if let Some(ref title) = input.title {
        updates.push(format!("title = '{}'", title.replace('\'', "''")));
    }
    if let Some(ref description) = input.description {
        updates.push(format!("description = '{}'", description.replace('\'', "''")));
    }
    if let Some(ref video_url) = input.video_url {
        updates.push(format!("video_url = '{}'", video_url.replace('\'', "''")));
    }
    if let Some(ref bunny_guid) = input.bunny_video_guid {
        updates.push(format!("bunny_video_guid = '{}'", bunny_guid));
    }
    if let Some(ref thumbnail_url) = input.thumbnail_url {
        updates.push(format!("thumbnail_url = '{}'", thumbnail_url.replace('\'', "''")));
    }
    if let Some(duration) = input.duration_seconds {
        updates.push(format!("duration_seconds = {}", duration));
    }
    if let Some(ref video_type) = input.video_type {
        updates.push(format!("video_type = '{}'", video_type));
    }
    if let Some(is_preview) = input.is_preview {
        updates.push(format!("is_preview = {}", is_preview));
    }
    if let Some(is_published) = input.is_published {
        updates.push(format!("is_published = {}", is_published));
    }

    if updates.is_empty() {
        return Ok(Json(json!({"success": true, "message": "No changes"})));
    }

    updates.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE indicator_videos SET {} WHERE id = $1 AND indicator_id = $2",
        updates.join(", ")
    );

    sqlx::query(&sql)
        .bind(video_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to update video: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a video
async fn delete_video(
    State(state): State<AppState>,
    Path((indicator_id, video_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "DELETE FROM indicator_videos WHERE id = $1 AND indicator_id = $2"
    )
    .bind(video_id)
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to delete video: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))));
    }

    Ok(Json(json!({"success": true})))
}

/// Reorder videos
async fn reorder_videos(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<ReorderItemsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.items {
        sqlx::query(
            "UPDATE indicator_videos SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND indicator_id = $3"
        )
        .bind(item.sort_order)
        .bind(item.id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PLATFORM FILE ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a platform file
async fn create_platform_file(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreatePlatformFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // If this is latest version, mark other files for this platform as not latest
    if input.is_latest.unwrap_or(true) {
        sqlx::query(
            "UPDATE indicator_platform_files SET is_latest = false WHERE indicator_id = $1 AND platform_id = $2"
        )
        .bind(indicator_id)
        .bind(input.platform_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    let file: IndicatorPlatformFile = sqlx::query_as(
        r#"INSERT INTO indicator_platform_files
           (indicator_id, platform_id, file_url, file_name, file_size_bytes,
            version, version_notes, installation_notes, is_latest, checksum_sha256)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING *"#
    )
    .bind(indicator_id)
    .bind(input.platform_id)
    .bind(&input.file_url)
    .bind(&input.file_name)
    .bind(input.file_size_bytes)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .bind(&input.version_notes)
    .bind(&input.installation_notes)
    .bind(input.is_latest.unwrap_or(true))
    .bind(&input.checksum_sha256)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create platform file: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Update indicator's supported platforms
    sqlx::query(
        r#"UPDATE indicators_enhanced
           SET supported_platforms = (
             SELECT COALESCE(jsonb_agg(DISTINCT platform_id), '[]'::jsonb)
             FROM indicator_platform_files
             WHERE indicator_id = $1
           )
           WHERE id = $1"#
    )
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .ok();

    info!("Created platform file: {} for indicator {}", file.file_name, indicator_id);

    Ok(Json(json!({
        "success": true,
        "file": {
            "id": file.id,
            "file_name": file.file_name,
            "version": file.version,
            "formatted_size": format_file_size(file.file_size_bytes.unwrap_or(0))
        }
    })))
}

/// Update a platform file
async fn update_platform_file(
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i64)>,
    Json(input): Json<UpdatePlatformFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = vec![];

    if let Some(ref file_url) = input.file_url {
        updates.push(format!("file_url = '{}'", file_url.replace('\'', "''")));
    }
    if let Some(ref file_name) = input.file_name {
        updates.push(format!("file_name = '{}'", file_name.replace('\'', "''")));
    }
    if let Some(file_size) = input.file_size_bytes {
        updates.push(format!("file_size_bytes = {}", file_size));
    }
    if let Some(ref version) = input.version {
        updates.push(format!("version = '{}'", version));
    }
    if let Some(ref version_notes) = input.version_notes {
        updates.push(format!("version_notes = '{}'", version_notes.replace('\'', "''")));
    }
    if let Some(ref installation_notes) = input.installation_notes {
        updates.push(format!("installation_notes = '{}'", installation_notes.replace('\'', "''")));
    }
    if let Some(is_latest) = input.is_latest {
        updates.push(format!("is_latest = {}", is_latest));
    }
    if let Some(ref checksum) = input.checksum_sha256 {
        updates.push(format!("checksum_sha256 = '{}'", checksum));
    }

    if updates.is_empty() {
        return Ok(Json(json!({"success": true, "message": "No changes"})));
    }

    updates.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE indicator_platform_files SET {} WHERE id = $1 AND indicator_id = $2",
        updates.join(", ")
    );

    sqlx::query(&sql)
        .bind(file_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to update platform file: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a platform file
async fn delete_platform_file(
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "DELETE FROM indicator_platform_files WHERE id = $1 AND indicator_id = $2"
    )
    .bind(file_id)
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to delete platform file: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "File not found"}))));
    }

    // Update indicator's supported platforms
    sqlx::query(
        r#"UPDATE indicators_enhanced
           SET supported_platforms = (
             SELECT COALESCE(jsonb_agg(DISTINCT platform_id), '[]'::jsonb)
             FROM indicator_platform_files
             WHERE indicator_id = $1
           )
           WHERE id = $1"#
    )
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({"success": true})))
}

/// Bulk upload files for multiple platforms
async fn bulk_upload_files(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<BulkUploadFilesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut created = 0;

    for file in &input.files {
        // Mark existing files for this platform as not latest
        sqlx::query(
            "UPDATE indicator_platform_files SET is_latest = false WHERE indicator_id = $1 AND platform_id = $2"
        )
        .bind(indicator_id)
        .bind(file.platform_id)
        .execute(&state.db.pool)
        .await
        .ok();

        let result = sqlx::query(
            r#"INSERT INTO indicator_platform_files
               (indicator_id, platform_id, file_url, file_name, file_size_bytes, version, is_latest)
               VALUES ($1, $2, $3, $4, $5, $6, true)"#
        )
        .bind(indicator_id)
        .bind(file.platform_id)
        .bind(&file.file_url)
        .bind(&file.file_name)
        .bind(file.file_size_bytes)
        .bind(file.version.as_deref().unwrap_or("1.0"))
        .execute(&state.db.pool)
        .await;

        if result.is_ok() {
            created += 1;
        }
    }

    // Update indicator's supported platforms
    sqlx::query(
        r#"UPDATE indicators_enhanced
           SET supported_platforms = (
             SELECT COALESCE(jsonb_agg(DISTINCT platform_id), '[]'::jsonb)
             FROM indicator_platform_files
             WHERE indicator_id = $1
           )
           WHERE id = $1"#
    )
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({
        "success": true,
        "created": created,
        "total": input.files.len()
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOCUMENTATION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create documentation
async fn create_documentation(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateDocumentationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM indicator_documentation WHERE indicator_id = $1"
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let doc: IndicatorDocumentation = sqlx::query_as(
        r#"INSERT INTO indicator_documentation
           (indicator_id, title, doc_type, content_html, file_url, file_name, sort_order, is_published)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING *"#
    )
    .bind(indicator_id)
    .bind(&input.title)
    .bind(input.doc_type.as_deref().unwrap_or("guide"))
    .bind(&input.content_html)
    .bind(&input.file_url)
    .bind(&input.file_name)
    .bind(sort_order)
    .bind(input.is_published.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create documentation: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Created documentation: {} for indicator {}", doc.title, indicator_id);

    Ok(Json(json!({
        "success": true,
        "documentation": {
            "id": doc.id,
            "title": doc.title,
            "doc_type": doc.doc_type
        }
    })))
}

/// Update documentation
async fn update_documentation(
    State(state): State<AppState>,
    Path((indicator_id, doc_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateDocumentationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = vec![];

    if let Some(ref title) = input.title {
        updates.push(format!("title = '{}'", title.replace('\'', "''")));
    }
    if let Some(ref doc_type) = input.doc_type {
        updates.push(format!("doc_type = '{}'", doc_type));
    }
    if let Some(ref content_html) = input.content_html {
        updates.push(format!("content_html = '{}'", content_html.replace('\'', "''")));
    }
    if let Some(ref file_url) = input.file_url {
        updates.push(format!("file_url = '{}'", file_url.replace('\'', "''")));
    }
    if let Some(ref file_name) = input.file_name {
        updates.push(format!("file_name = '{}'", file_name.replace('\'', "''")));
    }
    if let Some(is_published) = input.is_published {
        updates.push(format!("is_published = {}", is_published));
    }

    if updates.is_empty() {
        return Ok(Json(json!({"success": true, "message": "No changes"})));
    }

    updates.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE indicator_documentation SET {} WHERE id = $1 AND indicator_id = $2",
        updates.join(", ")
    );

    sqlx::query(&sql)
        .bind(doc_id)
        .bind(indicator_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to update documentation: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    Ok(Json(json!({"success": true})))
}

/// Delete documentation
async fn delete_documentation(
    State(state): State<AppState>,
    Path((indicator_id, doc_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "DELETE FROM indicator_documentation WHERE id = $1 AND indicator_id = $2"
    )
    .bind(doc_id)
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to delete documentation: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Documentation not found"}))));
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADINGVIEW ACCESS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Grant TradingView access to a user
async fn grant_tradingview_access(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<GrantTradingViewAccessRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate username format
    if !validate_tradingview_username(&input.tradingview_username) {
        return Err((StatusCode::BAD_REQUEST, Json(json!({
            "error": "Invalid TradingView username. Must be 3-20 characters, alphanumeric with underscores, starting with a letter."
        }))));
    }

    let expires_at: Option<DateTime<Utc>> = input.expires_at.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&Utc))
    });

    let access: IndicatorTradingViewAccess = sqlx::query_as(
        r#"INSERT INTO indicator_tradingview_access
           (indicator_id, user_id, tradingview_username, access_type, expires_at, notes)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (indicator_id, tradingview_username) DO UPDATE SET
             is_active = true,
             access_type = $4,
             expires_at = $5,
             notes = COALESCE($6, indicator_tradingview_access.notes)
           RETURNING *"#
    )
    .bind(indicator_id)
    .bind(input.user_id)
    .bind(&input.tradingview_username)
    .bind(input.access_type.as_deref().unwrap_or("standard"))
    .bind(expires_at)
    .bind(&input.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to grant TradingView access: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    info!("Granted TradingView access for {} to indicator {}", input.tradingview_username, indicator_id);

    Ok(Json(json!({
        "success": true,
        "access": {
            "id": access.id,
            "tradingview_username": access.tradingview_username,
            "access_type": access.access_type,
            "granted_at": access.granted_at.to_rfc3339()
        }
    })))
}

/// Bulk grant TradingView access
async fn bulk_grant_tradingview_access(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<BulkGrantTradingViewAccessRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut granted = 0;
    let mut errors: Vec<String> = vec![];

    for access in &input.accesses {
        if !validate_tradingview_username(&access.tradingview_username) {
            errors.push(format!("Invalid username: {}", access.tradingview_username));
            continue;
        }

        let result = sqlx::query(
            r#"INSERT INTO indicator_tradingview_access
               (indicator_id, user_id, tradingview_username, access_type)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (indicator_id, tradingview_username) DO UPDATE SET
                 is_active = true,
                 access_type = $4"#
        )
        .bind(indicator_id)
        .bind(access.user_id)
        .bind(&access.tradingview_username)
        .bind(access.access_type.as_deref().unwrap_or("standard"))
        .execute(&state.db.pool)
        .await;

        if result.is_ok() {
            granted += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "granted": granted,
        "total": input.accesses.len(),
        "errors": errors
    })))
}

/// List TradingView accesses for an indicator
async fn list_tradingview_accesses(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Query(query): Query<TradingViewAccessQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = format!(
        r#"SELECT ta.*, u.email as user_email, u.first_name, u.last_name
           FROM indicator_tradingview_access ta
           LEFT JOIN users u ON ta.user_id = u.id
           WHERE ta.indicator_id = {}"#,
        indicator_id
    );

    if let Some(is_active) = query.is_active {
        sql.push_str(&format!(" AND ta.is_active = {}", is_active));
    }
    if let Some(is_synced) = query.is_synced {
        sql.push_str(&format!(" AND ta.synced_to_tradingview = {}", is_synced));
    }
    if let Some(ref search) = query.search {
        sql.push_str(&format!(
            " AND (ta.tradingview_username ILIKE '%{}%' OR u.email ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    sql.push_str(&format!(
        " ORDER BY ta.granted_at DESC LIMIT {} OFFSET {}",
        per_page, offset
    ));

    let accesses: Vec<(IndicatorTradingViewAccess, Option<String>, Option<String>, Option<String>)> =
        sqlx::query_as(&sql)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    let access_responses: Vec<serde_json::Value> = accesses
        .iter()
        .map(|(a, email, first_name, last_name)| {
            let user_name = match (first_name, last_name) {
                (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
                (Some(f), None) => Some(f.clone()),
                _ => None,
            };

            let is_expired = a.expires_at.map(|exp| exp < Utc::now()).unwrap_or(false);

            json!({
                "id": a.id,
                "user_id": a.user_id,
                "user_email": email,
                "user_name": user_name,
                "tradingview_username": a.tradingview_username,
                "access_type": a.access_type,
                "granted_at": a.granted_at.to_rfc3339(),
                "expires_at": a.expires_at.map(|d| d.to_rfc3339()),
                "is_active": a.is_active,
                "is_expired": is_expired,
                "synced_to_tradingview": a.synced_to_tradingview,
                "last_sync_at": a.last_sync_at.map(|d| d.to_rfc3339()),
                "sync_error": a.sync_error,
                "notes": a.notes
            })
        })
        .collect();

    let total: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM indicator_tradingview_access WHERE indicator_id = {}",
        indicator_id
    ))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "accesses": access_responses,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

/// Revoke TradingView access
async fn revoke_tradingview_access(
    State(state): State<AppState>,
    Path((indicator_id, access_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE indicator_tradingview_access SET is_active = false WHERE id = $1 AND indicator_id = $2"
    )
    .bind(access_id)
    .bind(indicator_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to revoke TradingView access: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Access not found"}))));
    }

    info!("Revoked TradingView access: {}", access_id);
    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOAD & STATS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Log a download
async fn log_download(
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = input.get("user_id").and_then(|v| v.as_i64())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({"error": "user_id required"}))))?;

    // Get platform_id from file
    let file_info: Option<(i64,)> = sqlx::query_as(
        "SELECT platform_id FROM indicator_platform_files WHERE id = $1"
    )
    .bind(file_id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let platform_id = file_info.map(|f| f.0).unwrap_or(0);

    sqlx::query(
        r#"INSERT INTO indicator_download_log
           (user_id, indicator_id, platform_file_id, platform_id, ip_address, user_agent)
           VALUES ($1, $2, $3, $4, $5, $6)"#
    )
    .bind(user_id)
    .bind(indicator_id)
    .bind(file_id)
    .bind(platform_id)
    .bind(input.get("ip_address").and_then(|v| v.as_str()))
    .bind(input.get("user_agent").and_then(|v| v.as_str()))
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to log download: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(json!({"success": true})))
}

/// Get indicator stats
async fn get_indicator_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_indicators: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM indicators_enhanced WHERE deleted_at IS NULL"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let published_indicators: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM indicators_enhanced WHERE is_published = true AND deleted_at IS NULL"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_downloads: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_downloads), 0) FROM indicators_enhanced WHERE deleted_at IS NULL"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let total_tradingview_users: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM indicator_tradingview_access WHERE is_active = true"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Downloads by platform
    let downloads_by_platform: Vec<(String, i64)> = sqlx::query_as(
        r#"SELECT p.display_name, COUNT(d.id)
           FROM indicator_download_log d
           JOIN indicator_platforms p ON d.platform_id = p.id
           GROUP BY p.id, p.display_name
           ORDER BY COUNT(d.id) DESC"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Top indicators
    let top_indicators: Vec<(i64, String, i64)> = sqlx::query_as(
        r#"SELECT id, name, total_downloads
           FROM indicators_enhanced
           WHERE deleted_at IS NULL
           ORDER BY total_downloads DESC
           LIMIT 10"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "total_indicators": total_indicators.0,
        "published_indicators": published_indicators.0,
        "total_downloads": total_downloads.0,
        "total_tradingview_users": total_tradingview_users.0,
        "downloads_by_platform": downloads_by_platform.iter().map(|(name, count)| {
            json!({"platform_name": name, "download_count": count})
        }).collect::<Vec<_>>(),
        "top_indicators": top_indicators.iter().map(|(id, name, downloads)| {
            json!({"indicator_id": id, "indicator_name": name, "download_count": downloads})
        }).collect::<Vec<_>>()
    })))
}

/// Get download log
async fn get_download_log(
    State(state): State<AppState>,
    Query(query): Query<DownloadLogQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        r#"SELECT d.*, u.email, i.name as indicator_name, p.display_name as platform_name, f.file_name
           FROM indicator_download_log d
           LEFT JOIN users u ON d.user_id = u.id
           LEFT JOIN indicators_enhanced i ON d.indicator_id = i.id
           LEFT JOIN indicator_platforms p ON d.platform_id = p.id
           LEFT JOIN indicator_platform_files f ON d.platform_file_id = f.id
           WHERE 1=1"#
    );

    if let Some(user_id) = query.user_id {
        sql.push_str(&format!(" AND d.user_id = {}", user_id));
    }
    if let Some(platform_id) = query.platform_id {
        sql.push_str(&format!(" AND d.platform_id = {}", platform_id));
    }
    if let Some(ref from_date) = query.from_date {
        sql.push_str(&format!(" AND d.downloaded_at >= '{}'", from_date));
    }
    if let Some(ref to_date) = query.to_date {
        sql.push_str(&format!(" AND d.downloaded_at <= '{}'", to_date));
    }

    sql.push_str(&format!(
        " ORDER BY d.downloaded_at DESC LIMIT {} OFFSET {}",
        per_page, offset
    ));

    let logs: Vec<(i64, i64, i64, i64, i64, DateTime<Utc>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> =
        sqlx::query_as(&sql)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    let log_responses: Vec<serde_json::Value> = logs
        .iter()
        .map(|(id, user_id, indicator_id, file_id, platform_id, downloaded_at, _ip, _ua, email, indicator_name, platform_name, file_name)| {
            json!({
                "id": id,
                "user_id": user_id,
                "user_email": email,
                "indicator_id": indicator_id,
                "indicator_name": indicator_name,
                "platform_name": platform_name,
                "file_name": file_name,
                "downloaded_at": downloaded_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(json!({
        "downloads": log_responses,
        "pagination": {
            "page": page,
            "per_page": per_page
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// USER ACCESS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Grant user access to an indicator
async fn grant_user_access(
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = input.get("user_id").and_then(|v| v.as_i64())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({"error": "user_id required"}))))?;

    let access_source = input.get("access_source").and_then(|v| v.as_str()).unwrap_or("admin");

    let expires_at: Option<DateTime<Utc>> = input.get("expires_at")
        .and_then(|v| v.as_str())
        .and_then(|d| DateTime::parse_from_rfc3339(d).ok().map(|dt| dt.with_timezone(&Utc)));

    let access: UserIndicatorAccess = sqlx::query_as(
        r#"INSERT INTO user_indicator_access
           (user_id, indicator_id, access_source, expires_at)
           VALUES ($1, $2, $3, $4)
           ON CONFLICT (user_id, indicator_id) DO UPDATE SET
             is_active = true,
             expires_at = $4
           RETURNING *"#
    )
    .bind(user_id)
    .bind(indicator_id)
    .bind(access_source)
    .bind(expires_at)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to grant user access: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(json!({
        "success": true,
        "access": {
            "id": access.id,
            "user_id": access.user_id,
            "indicator_id": access.indicator_id,
            "granted_at": access.granted_at.to_rfc3339()
        }
    })))
}

/// Revoke user access
async fn revoke_user_access(
    State(state): State<AppState>,
    Path((indicator_id, user_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE user_indicator_access SET is_active = false WHERE indicator_id = $1 AND user_id = $2"
    )
    .bind(indicator_id)
    .bind(user_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to revoke user access: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Access not found"}))));
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER SETUP
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Indicators
        .route("/", get(list_indicators).post(create_indicator))
        .route("/stats", get(get_indicator_stats))
        .route("/downloads", get(get_download_log))
        .route("/:indicator_id", get(get_indicator).put(update_indicator).delete(delete_indicator))
        // Platforms
        .route("/platforms", get(list_platforms).post(create_platform))
        .route("/platforms/:platform_id", put(update_platform))
        // Videos
        .route("/:indicator_id/videos", post(create_video))
        .route("/:indicator_id/videos/reorder", put(reorder_videos))
        .route("/:indicator_id/videos/:video_id", put(update_video).delete(delete_video))
        // Platform Files
        .route("/:indicator_id/files", post(create_platform_file))
        .route("/:indicator_id/files/bulk", post(bulk_upload_files))
        .route("/:indicator_id/files/:file_id", put(update_platform_file).delete(delete_platform_file))
        .route("/:indicator_id/files/:file_id/download", post(log_download))
        // Documentation
        .route("/:indicator_id/docs", post(create_documentation))
        .route("/:indicator_id/docs/:doc_id", put(update_documentation).delete(delete_documentation))
        // TradingView Access
        .route("/:indicator_id/tradingview", get(list_tradingview_accesses).post(grant_tradingview_access))
        .route("/:indicator_id/tradingview/bulk", post(bulk_grant_tradingview_access))
        .route("/:indicator_id/tradingview/:access_id", delete(revoke_tradingview_access))
        // User Access
        .route("/:indicator_id/access", post(grant_user_access))
        .route("/:indicator_id/access/:user_id", delete(revoke_user_access))
}
