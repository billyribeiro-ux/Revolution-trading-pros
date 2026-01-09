//! Member Indicator API Routes - Secure Downloads
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! - Public indicator listing
//! - Ownership verification
//! - Secure hash-based download URLs (WordPress-compatible)
//! - Download tracking

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Json, Router,
};
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use serde_json::json;
use uuid::Uuid;

use crate::models::indicator::{
    Indicator, IndicatorFile, IndicatorListItem, IndicatorQueryParams, IndicatorVideo,
    UserIndicatorOwnership,
};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTES (No auth required)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_public_indicators(
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
        WHERE is_published = true AND status = 'published'
        "#,
    );

    if let Some(is_free) = params.is_free {
        query.push_str(&format!(" AND is_free = {}", is_free));
    }

    if let Some(ref platform) = params.platform {
        query.push_str(&format!(
            " AND supported_platforms ? '{}'",
            platform.replace('\'', "''")
        ));
    }

    if let Some(ref search) = params.search {
        let search = search.replace('\'', "''");
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR tagline ILIKE '%{}%')",
            search, search
        ));
    }

    query.push_str(" ORDER BY created_at DESC");
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

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM indicators WHERE is_published = true AND status = 'published'",
    )
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

async fn get_public_indicator(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: Indicator = sqlx::query_as(
        "SELECT * FROM indicators WHERE slug = $1 AND is_published = true",
    )
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

// ═══════════════════════════════════════════════════════════════════════════════════
// MEMBER ROUTES (Ownership verification)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn get_my_indicators(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Get user_id from auth middleware
    let user_id: i32 = 1; // Placeholder

    let indicators: Vec<serde_json::Value> = sqlx::query_as::<_, (Uuid, String, String, Option<String>, Option<String>, Option<String>, chrono::DateTime<chrono::Utc>)>(
        r#"
        SELECT i.id, i.name, i.slug, i.tagline, i.logo_url, i.card_image_url, uio.access_granted_at
        FROM user_indicator_ownership uio
        JOIN indicators i ON uio.indicator_id = i.id
        WHERE uio.user_id = $1 
        AND uio.is_active = true
        AND (uio.access_expires_at IS NULL OR uio.access_expires_at > NOW())
        ORDER BY uio.access_granted_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .into_iter()
    .map(|(id, name, slug, tagline, logo_url, card_image_url, access_granted_at)| {
        json!({
            "id": id,
            "name": name,
            "slug": slug,
            "tagline": tagline,
            "logo_url": logo_url,
            "card_image_url": card_image_url,
            "access_granted_at": access_granted_at
        })
    })
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": indicators
    })))
}

async fn get_indicator_downloads(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Get user_id from auth middleware
    let user_id: i32 = 1; // Placeholder

    // Get indicator
    let indicator: Indicator = sqlx::query_as(
        "SELECT * FROM indicators WHERE slug = $1",
    )
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

    // Check ownership
    let ownership: Option<UserIndicatorOwnership> = sqlx::query_as(
        r#"
        SELECT * FROM user_indicator_ownership 
        WHERE user_id = $1 AND indicator_id = $2 AND is_active = true
        AND (access_expires_at IS NULL OR access_expires_at > NOW())
        "#,
    )
    .bind(user_id)
    .bind(indicator.id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if ownership.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You do not own this indicator"})),
        ));
    }

    // Get all active files
    let files: Vec<IndicatorFile> = sqlx::query_as(
        r#"
        SELECT * FROM indicator_files 
        WHERE indicator_id = $1 AND is_active = true 
        ORDER BY platform, display_order
        "#,
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get all videos
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 AND is_published = true ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator": indicator,
            "files": files,
            "videos": videos,
            "ownership": ownership
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECURE DOWNLOAD URL GENERATION (WordPress-compatible hash-based)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn generate_download_url(
    State(state): State<AppState>,
    Path((slug, file_id)): Path<(String, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Get user_id from auth middleware
    let user_id: i32 = 1; // Placeholder

    // Get indicator
    let indicator: (Uuid,) = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
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

    // Check ownership
    let has_access: Option<(i32,)> = sqlx::query_as(
        r#"
        SELECT id FROM user_indicator_ownership 
        WHERE user_id = $1 AND indicator_id = $2 AND is_active = true
        AND (access_expires_at IS NULL OR access_expires_at > NOW())
        "#,
    )
    .bind(user_id)
    .bind(indicator.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if has_access.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You do not own this indicator"})),
        ));
    }

    // Get file
    let file: IndicatorFile = sqlx::query_as(
        "SELECT * FROM indicator_files WHERE id = $1 AND indicator_id = $2 AND is_active = true",
    )
    .bind(file_id)
    .bind(indicator.0)
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
            Json(json!({"error": "File not found"})),
        )
    })?;

    // Generate secure download token (WordPress-compatible hash)
    let secret = std::env::var("DOWNLOAD_SECRET_KEY").unwrap_or_else(|_| "revolution-secret-2026".to_string());
    let expires_at = Utc::now() + Duration::hours(24);
    let expiry_timestamp = expires_at.timestamp();

    // Hash format: SHA256(user_id + file_id + expiry + secret)
    let hash_input = format!("{}{}{}{}", user_id, file_id, expiry_timestamp, secret);
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let token = format!("{:x}", hasher.finalize());

    // Store download record
    sqlx::query(
        r#"
        INSERT INTO indicator_downloads (user_id, indicator_id, file_id, download_token, token_expires_at, ownership_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(user_id)
    .bind(indicator.0)
    .bind(file_id)
    .bind(&token)
    .bind(expires_at)
    .bind(has_access.unwrap().0)
    .execute(&state.db.pool)
    .await
    .ok();

    // Build secure download URL
    let base_url = std::env::var("API_BASE_URL").unwrap_or_else(|_| "https://api.revolution-trading.com".to_string());
    let download_url = format!(
        "{}/download/indicator/{}/{}?token={}&expires={}",
        base_url, slug, file_id, token, expiry_timestamp
    );

    Ok(Json(json!({
        "success": true,
        "data": {
            "download_url": download_url,
            "token": token,
            "expires_at": expires_at,
            "file_name": file.file_name,
            "file_size_bytes": file.file_size_bytes,
            "platform": file.platform
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECURE FILE DOWNLOAD (Token validation & streaming)
// ═══════════════════════════════════════════════════════════════════════════════════

async fn download_file(
    State(state): State<AppState>,
    Path((slug, file_id)): Path<(String, i32)>,
    Query(params): Query<DownloadParams>,
) -> Result<Response<axum::body::Body>, (StatusCode, Json<serde_json::Value>)> {
    let token = params.token.as_deref().unwrap_or("");
    let expires = params.expires.unwrap_or(0);

    // Validate expiry
    if Utc::now().timestamp() > expires {
        return Err((
            StatusCode::GONE,
            Json(json!({"error": "Download link has expired"})),
        ));
    }

    // Find download record
    let download: Option<(i32, i32, Option<i32>, Option<i32>)> = sqlx::query_as(
        r#"
        SELECT id, file_id, download_count, max_downloads 
        FROM indicator_downloads 
        WHERE download_token = $1 
        AND token_expires_at > NOW()
        "#,
    )
    .bind(token)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let download = download.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid or expired download token"})),
        )
    })?;

    // Check download limit
    let count = download.2.unwrap_or(0);
    let max = download.3.unwrap_or(5);
    if count >= max {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"error": "Download limit reached"})),
        ));
    }

    // Get file info
    let file: IndicatorFile = sqlx::query_as(
        "SELECT * FROM indicator_files WHERE id = $1",
    )
    .bind(file_id)
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
            Json(json!({"error": "File not found"})),
        )
    })?;

    // Update download count
    sqlx::query(
        r#"
        UPDATE indicator_downloads 
        SET download_count = download_count + 1, downloaded_at = NOW(), status = 'completed'
        WHERE id = $1
        "#,
    )
    .bind(download.0)
    .execute(&state.db.pool)
    .await
    .ok();

    // Update file download count
    sqlx::query("UPDATE indicator_files SET download_count = download_count + 1 WHERE id = $1")
        .bind(file_id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Redirect to actual file (CDN URL)
    let cdn_url = file.cdn_url.unwrap_or_else(|| file.file_path.clone());

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, cdn_url)
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file.file_name))
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}

#[derive(Debug, serde::Deserialize)]
struct DownloadParams {
    token: Option<String>,
    expires: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_public_indicators))
        .route("/:slug", get(get_public_indicator))
}

pub fn member_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_indicators))
        .route("/:slug", get(get_indicator_downloads))
        .route("/:slug/download/:file_id", get(generate_download_url))
}

pub fn download_router() -> Router<AppState> {
    Router::new()
        .route("/indicator/:slug/:file_id", get(download_file))
}
