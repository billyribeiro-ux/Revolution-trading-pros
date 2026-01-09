//! Indicator Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full-service indicator management with:
//! - Multi-platform file downloads
//! - Secure hash-based download URLs
//! - Video management via Bunny Stream
//! - Ownership and analytics

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use chrono::{Duration, Utc};
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::models::indicator::{
    CreateIndicatorFileRequest, CreateIndicatorRequest, CreateIndicatorVideoRequest,
    GrantOwnershipRequest, Indicator, IndicatorFile, IndicatorListItem, IndicatorQueryParams,
    IndicatorVideo, PaginatedIndicators, TradingPlatform, UpdateIndicatorFileRequest,
    UpdateIndicatorRequest, UserIndicatorOwnership,
};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// INDICATOR CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_indicators(
    State(state): State<AppState>,
    Query(params): Query<IndicatorQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut query = String::from(
        r#"
        SELECT id, name, slug, tagline, price_cents, is_free, sale_price_cents,
               logo_url, card_image_url, status, is_published, view_count,
               download_count, created_at
        FROM indicators
        WHERE 1=1
        "#,
    );

    if let Some(ref status) = params.status {
        query.push_str(&format!(" AND status = '{}'", status));
    }

    if let Some(is_free) = params.is_free {
        query.push_str(&format!(" AND is_free = {}", is_free));
    }

    if let Some(ref search) = params.search {
        let search = search.replace('\'', "''");
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR tagline ILIKE '%{}%')",
            search, search
        ));
    }

    let sort_by = params.sort_by.unwrap_or_else(|| "created_at".to_string());
    let sort_order = params.sort_order.unwrap_or_else(|| "DESC".to_string());
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_order));
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let indicators: Vec<IndicatorListItem> = sqlx::query_as(&query)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM indicators")
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
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: Indicator = if let Ok(uuid) = Uuid::parse_str(&id) {
        sqlx::query_as("SELECT * FROM indicators WHERE id = $1")
            .bind(uuid)
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

    let files: Vec<IndicatorFile> = sqlx::query_as(
        "SELECT * FROM indicator_files WHERE indicator_id = $1 AND is_active = true ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let platforms: Vec<String> = files.iter().map(|f| f.platform.clone()).collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator": indicator,
            "files": files,
            "videos": videos,
            "platforms": platforms
        }
    })))
}

async fn create_indicator(
    State(state): State<AppState>,
    Json(input): Json<CreateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.name));

    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
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

    let indicator: Indicator = sqlx::query_as(
        r#"
        INSERT INTO indicators (
            name, slug, tagline, description, short_description, long_description,
            price_cents, is_free, logo_url, card_image_url, banner_image_url,
            features, requirements, supported_platforms, version,
            meta_title, meta_description, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, 'draft')
        RETURNING *
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.tagline)
    .bind(&input.description)
    .bind(&input.short_description)
    .bind(&input.long_description)
    .bind(input.price_cents.unwrap_or(0))
    .bind(input.is_free.unwrap_or(false))
    .bind(&input.logo_url)
    .bind(&input.card_image_url)
    .bind(&input.banner_image_url)
    .bind(json!(input.features.unwrap_or_default()))
    .bind(json!(input.requirements.unwrap_or_default()))
    .bind(json!(input.supported_platforms.unwrap_or_default()))
    .bind(input.version.as_deref().unwrap_or("1.0"))
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut param_count = 1;

    macro_rules! add_update {
        ($field:ident, $col:expr) => {
            if input.$field.is_some() {
                updates.push(format!("{} = ${}", $col, param_count));
                param_count += 1;
            }
        };
    }

    add_update!(name, "name");
    add_update!(slug, "slug");
    add_update!(tagline, "tagline");
    add_update!(description, "description");
    add_update!(short_description, "short_description");
    add_update!(long_description, "long_description");
    add_update!(price_cents, "price_cents");
    add_update!(is_free, "is_free");
    add_update!(sale_price_cents, "sale_price_cents");
    add_update!(logo_url, "logo_url");
    add_update!(card_image_url, "card_image_url");
    add_update!(banner_image_url, "banner_image_url");
    add_update!(featured_video_url, "featured_video_url");
    add_update!(bunny_video_guid, "bunny_video_guid");
    add_update!(features, "features");
    add_update!(requirements, "requirements");
    add_update!(compatibility, "compatibility");
    add_update!(supported_platforms, "supported_platforms");
    add_update!(version, "version");
    add_update!(is_published, "is_published");
    add_update!(status, "status");
    add_update!(meta_title, "meta_title");
    add_update!(meta_description, "meta_description");
    add_update!(og_image_url, "og_image_url");

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    updates.push(format!("updated_at = NOW()"));
    let query = format!(
        "UPDATE indicators SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    let mut q = sqlx::query_as::<_, Indicator>(&query);

    if let Some(ref v) = input.name { q = q.bind(v); }
    if let Some(ref v) = input.slug { q = q.bind(v); }
    if let Some(ref v) = input.tagline { q = q.bind(v); }
    if let Some(ref v) = input.description { q = q.bind(v); }
    if let Some(ref v) = input.short_description { q = q.bind(v); }
    if let Some(ref v) = input.long_description { q = q.bind(v); }
    if let Some(v) = input.price_cents { q = q.bind(v); }
    if let Some(v) = input.is_free { q = q.bind(v); }
    if let Some(v) = input.sale_price_cents { q = q.bind(v); }
    if let Some(ref v) = input.logo_url { q = q.bind(v); }
    if let Some(ref v) = input.card_image_url { q = q.bind(v); }
    if let Some(ref v) = input.banner_image_url { q = q.bind(v); }
    if let Some(ref v) = input.featured_video_url { q = q.bind(v); }
    if let Some(ref v) = input.bunny_video_guid { q = q.bind(v); }
    if let Some(ref v) = input.features { q = q.bind(v); }
    if let Some(ref v) = input.requirements { q = q.bind(v); }
    if let Some(ref v) = input.compatibility { q = q.bind(v); }
    if let Some(ref v) = input.supported_platforms { q = q.bind(v); }
    if let Some(ref v) = input.version { q = q.bind(v); }
    if let Some(v) = input.is_published { q = q.bind(v); }
    if let Some(ref v) = input.status { q = q.bind(v); }
    if let Some(ref v) = input.meta_title { q = q.bind(v); }
    if let Some(ref v) = input.meta_description { q = q.bind(v); }
    if let Some(ref v) = input.og_image_url { q = q.bind(v); }

    q = q.bind(id);

    let indicator = q.fetch_one(&state.db.pool).await.map_err(|e| {
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
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
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

async fn publish_indicator(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: Indicator = sqlx::query_as(
        r#"
        UPDATE indicators 
        SET is_published = true, status = 'published', published_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to publish indicator: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator published successfully",
        "data": indicator
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// FILE MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_files(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let files: Vec<IndicatorFile> = sqlx::query_as(
        "SELECT * FROM indicator_files WHERE indicator_id = $1 ORDER BY platform, display_order",
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

async fn create_file(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
    Json(input): Json<CreateIndicatorFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let file: IndicatorFile = sqlx::query_as(
        r#"
        INSERT INTO indicator_files (
            indicator_id, file_name, original_name, file_path, file_size_bytes,
            file_type, mime_type, platform, platform_version, storage_key,
            cdn_url, version, display_name, display_order
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
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
    .bind(input.version.as_deref().unwrap_or("1.0"))
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
        "message": "File created successfully",
        "data": file
    })))
}

async fn update_file(
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(Uuid, i32)>,
    Json(input): Json<UpdateIndicatorFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let file: IndicatorFile = sqlx::query_as(
        r#"
        UPDATE indicator_files
        SET display_name = COALESCE($1, display_name),
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
        "data": file
    })))
}

async fn delete_file(
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(Uuid, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM indicator_files WHERE id = $1 AND indicator_id = $2")
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

    Ok(Json(json!({
        "success": true,
        "message": "File deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_videos(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 ORDER BY display_order",
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

async fn create_video(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
    Json(input): Json<CreateIndicatorVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let library_id = std::env::var("BUNNY_STREAM_LIBRARY_ID").unwrap_or_default();
    
    let embed_url = format!(
        "https://iframe.mediadelivery.net/embed/{}/{}",
        library_id, input.bunny_video_guid
    );
    let play_url = format!(
        "https://iframe.mediadelivery.net/play/{}/{}",
        library_id, input.bunny_video_guid
    );
    let thumbnail_url = format!(
        "https://vz-{}.b-cdn.net/{}/thumbnail.jpg",
        library_id, input.bunny_video_guid
    );

    let video: IndicatorVideo = sqlx::query_as(
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
    .bind(input.bunny_library_id.as_deref().unwrap_or(&library_id))
    .bind(&embed_url)
    .bind(&play_url)
    .bind(&thumbnail_url)
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
        "message": "Video created successfully",
        "data": video
    })))
}

async fn delete_video(
    State(state): State<AppState>,
    Path((indicator_id, video_id)): Path<(Uuid, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM indicator_videos WHERE id = $1 AND indicator_id = $2")
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

    Ok(Json(json!({
        "success": true,
        "message": "Video deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// OWNERSHIP MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_owners(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let owners: Vec<UserIndicatorOwnership> = sqlx::query_as(
        "SELECT * FROM user_indicator_ownership WHERE indicator_id = $1 ORDER BY created_at DESC",
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
        "data": owners
    })))
}

async fn grant_ownership(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
    Json(input): Json<GrantOwnershipRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ownership: UserIndicatorOwnership = sqlx::query_as(
        r#"
        INSERT INTO user_indicator_ownership (
            user_id, indicator_id, source, is_lifetime_access, access_expires_at, notes
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, indicator_id) DO UPDATE
        SET is_active = true, 
            is_lifetime_access = COALESCE($4, user_indicator_ownership.is_lifetime_access),
            access_expires_at = COALESCE($5, user_indicator_ownership.access_expires_at),
            notes = COALESCE($6, user_indicator_ownership.notes),
            updated_at = NOW()
        RETURNING *
        "#,
    )
    .bind(input.user_id)
    .bind(indicator_id)
    .bind(input.source.as_deref().unwrap_or("admin_grant"))
    .bind(input.is_lifetime_access.unwrap_or(true))
    .bind(input.access_expires_at)
    .bind(&input.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to grant ownership: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Ownership granted successfully",
        "data": ownership
    })))
}

async fn revoke_ownership(
    State(state): State<AppState>,
    Path((indicator_id, user_id)): Path<(Uuid, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r#"
        UPDATE user_indicator_ownership 
        SET is_active = false, revoked_at = NOW()
        WHERE indicator_id = $1 AND user_id = $2
        "#,
    )
    .bind(indicator_id)
    .bind(user_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to revoke ownership: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Ownership revoked successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// UPLOAD URL GENERATION
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_file_upload_url(
    State(_state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let file_name = input["file_name"].as_str().unwrap_or("file");
    let file_type = input["file_type"].as_str().unwrap_or("application/octet-stream");
    let platform = input["platform"].as_str().unwrap_or("generic");

    // Cloudflare R2 configuration
    let r2_account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID").unwrap_or_default();
    let r2_bucket = std::env::var("CLOUDFLARE_R2_BUCKET").unwrap_or_else(|_| "revolution-indicators".to_string());
    let r2_access_key = std::env::var("CLOUDFLARE_R2_ACCESS_KEY").unwrap_or_default();
    let cdn_url = std::env::var("CLOUDFLARE_R2_CDN_URL").unwrap_or_else(|_| "https://indicators.revolution-trading.com".to_string());

    let file_path = format!("indicators/{}/{}/{}", indicator_id, platform, file_name);
    let upload_url = format!(
        "https://{}.r2.cloudflarestorage.com/{}/{}",
        r2_account_id, r2_bucket, file_path
    );
    let download_url = format!("{}/{}", cdn_url, file_path);

    Ok(Json(json!({
        "success": true,
        "data": {
            "upload_url": upload_url,
            "download_url": download_url,
            "file_path": file_path,
            "bucket": r2_bucket,
            "platform": platform,
            "headers": {
                "Content-Type": file_type
            }
        }
    })))
}

async fn create_video_upload(
    State(_state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input["title"].as_str().unwrap_or("Indicator Video");
    
    let library_id = std::env::var("BUNNY_STREAM_LIBRARY_ID").unwrap_or_default();
    let api_key = std::env::var("BUNNY_STREAM_API_KEY").unwrap_or_default();
    
    if library_id.is_empty() || api_key.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Bunny.net Stream not configured"})),
        ));
    }

    let client = reqwest::Client::new();
    let response = client
        .post(format!("https://video.bunnycdn.com/library/{}/videos", library_id))
        .header("AccessKey", &api_key)
        .header("Content-Type", "application/json")
        .json(&json!({ "title": title }))
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create video: {}", e)})),
            )
        })?;

    if !response.status().is_success() {
        let text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Bunny API error: {}", text)})),
        ));
    }

    let video: serde_json::Value = response.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to parse response: {}", e)})),
        )
    })?;

    let video_guid = video["guid"].as_str().unwrap_or("");
    let expiry = Utc::now().timestamp() + 7200;

    let tus_url = format!(
        "https://video.bunnycdn.com/tusupload?libraryId={}&videoId={}&expirationTime={}",
        library_id, video_guid, expiry
    );

    let signature_string = format!("{}{}{}", library_id, api_key, expiry);
    let signature = format!("{:x}", md5::compute(signature_string.as_bytes()));

    Ok(Json(json!({
        "success": true,
        "data": {
            "video_guid": video_guid,
            "library_id": library_id,
            "tus_upload_url": tus_url,
            "auth_signature": signature,
            "auth_expiry": expiry,
            "embed_url": format!("https://iframe.mediadelivery.net/embed/{}/{}", library_id, video_guid),
            "play_url": format!("https://iframe.mediadelivery.net/play/{}/{}", library_id, video_guid),
            "thumbnail_url": format!("https://vz-{}.b-cdn.net/{}/thumbnail.jpg", library_id, video_guid)
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PLATFORMS
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_platforms(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let platforms: Vec<TradingPlatform> = sqlx::query_as(
        "SELECT * FROM trading_platforms WHERE is_active = true ORDER BY display_order",
    )
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
        "data": platforms
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_analytics(
    State(state): State<AppState>,
    Path(indicator_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_downloads: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(download_count), 0) FROM indicator_files WHERE indicator_id = $1",
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let unique_users: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT user_id) FROM indicator_downloads WHERE indicator_id = $1",
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let by_platform: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT platform, COALESCE(SUM(download_count), 0) as count
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
            "unique_users": unique_users.0,
            "by_platform": by_platform.into_iter().collect::<std::collections::HashMap<_, _>>()
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Indicators
        .route("/", get(list_indicators).post(create_indicator))
        .route("/:id", get(get_indicator).put(update_indicator).delete(delete_indicator))
        .route("/:id/publish", post(publish_indicator))
        // Files
        .route("/:id/files", get(list_files).post(create_file))
        .route("/:id/files/:file_id", put(update_file).delete(delete_file))
        .route("/:id/upload-url", post(get_file_upload_url))
        // Videos
        .route("/:id/videos", get(list_videos).post(create_video))
        .route("/:id/videos/:video_id", post(delete_video))
        .route("/:id/video-upload", post(create_video_upload))
        // Ownership
        .route("/:id/owners", get(list_owners).post(grant_ownership))
        .route("/:id/owners/:user_id", post(revoke_ownership))
        // Analytics
        .route("/:id/analytics", get(get_analytics))
        // Platforms
        .route("/platforms", get(list_platforms))
}
