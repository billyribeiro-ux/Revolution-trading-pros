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
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::models::indicator::{
    Indicator, IndicatorFile, IndicatorListItem, IndicatorQueryParams, IndicatorVideo,
    UserIndicatorOwnership,
};
use crate::models::User;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTES (No auth required)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7 SECURITY FIX: Fully parameterized queries to prevent SQL injection
async fn list_public_indicators(
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

    // ICT 7: Use parameterized queries - matching actual database schema
    let indicators: Vec<IndicatorListItem> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, price, is_active, platform, thumbnail, created_at
        FROM indicators
        WHERE is_active = true
        AND ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
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
        r#"
        SELECT COUNT(*) FROM indicators
        WHERE is_active = true
        AND ($1::BOOLEAN IS NULL OR is_active = $1)
        AND ($2::TEXT IS NULL OR LOWER(platform) = $2)
        AND ($3::TEXT IS NULL OR name ILIKE '%' || $3 || '%' OR description ILIKE '%' || $3 || '%')
        "#,
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

async fn get_public_indicator(
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

// ═══════════════════════════════════════════════════════════════════════════════════
// MEMBER ROUTES (Ownership verification)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7 FIX: Uses i64 for indicator_id (matching database schema)
async fn get_my_indicators(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // ICT 7 FIX: Database uses BIGINT (i64) for indicator_id, not UUID
    // Using user_indicators table which has user_id and indicator_id as i64
    let indicators: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,                   // id
            String,                // name
            String,                // slug
            Option<String>,        // description (was tagline)
            Option<String>,        // thumbnail (was logo_url)
            Option<String>,        // download_url (was card_image_url)
            chrono::NaiveDateTime, // purchased_at (was access_granted_at)
        ),
    >(
        r#"
        SELECT i.id, i.name, i.slug, i.description, i.thumbnail, i.download_url, ui.purchased_at
        FROM user_indicators ui
        JOIN indicators i ON ui.indicator_id = i.id
        WHERE ui.user_id = $1
        AND i.is_active = true
        ORDER BY ui.purchased_at DESC
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
    .map(
        |(id, name, slug, description, thumbnail, download_url, purchased_at)| {
            json!({
                "id": id.to_string(),
                "name": name,
                "slug": slug,
                "tagline": description,
                "logo_url": thumbnail,
                "card_image_url": thumbnail,
                "access_granted_at": purchased_at.and_utc().to_rfc3339()
            })
        },
    )
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": indicators
    })))
}

/// ICT 7 FIX: Uses i64 for indicator_id, queries user_indicators table
async fn get_indicator_downloads(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get indicator
    let indicator: Indicator = sqlx::query_as("SELECT * FROM indicators WHERE slug = $1")
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

    // ICT 7 FIX: Check ownership using user_indicators table (uses i64)
    let ownership: Option<(i64, i64, i64, chrono::NaiveDateTime, Option<String>)> = sqlx::query_as(
        r#"
        SELECT id, user_id, indicator_id, purchased_at, license_key
        FROM user_indicators
        WHERE user_id = $1 AND indicator_id = $2
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

    // ICT 7 FIX: Get files from indicator_files table (uses i64 indicator_id)
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

    // ICT 7 FIX: Get videos - use indicator_videos table if exists
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build ownership response
    let ownership_info = ownership.map(|(id, user_id, indicator_id, purchased_at, license_key)| {
        json!({
            "id": id,
            "user_id": user_id,
            "indicator_id": indicator_id,
            "access_granted_at": purchased_at.and_utc().to_rfc3339(),
            "license_key": license_key,
            "is_lifetime_access": true
        })
    });

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator": indicator,
            "files": files,
            "videos": videos,
            "ownership": ownership_info
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SECURE DOWNLOAD URL GENERATION (WordPress-compatible hash-based)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7 FIX: Uses i64 for indicator_id, queries user_indicators table
async fn generate_download_url(
    user: User,
    State(state): State<AppState>,
    Path((slug, file_id)): Path<(String, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // ICT 7 FIX: Get indicator with i64 id
    let indicator: (i64,) = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
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

    // ICT 7 FIX: Check ownership using user_indicators table (uses i64)
    let has_access: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT id FROM user_indicators
        WHERE user_id = $1 AND indicator_id = $2
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

    // ICT 7 FIX: Get file with i64 indicator_id
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
    let secret = std::env::var("MEMBER_INDICATOR_SECRET")
        .unwrap_or_else(|_| "".to_string());
    let expires_at = Utc::now() + Duration::hours(24);
    let expiry_timestamp = expires_at.timestamp();

    // Hash format: SHA256(user_id + file_id + expiry + secret)
    let hash_input = format!("{}{}{}{}", user_id, file_id, expiry_timestamp, secret);
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let token = format!("{:x}", hasher.finalize());

    // ICT 7 FIX: Store download record with i64 indicator_id
    // Note: indicator_downloads table may not exist; this is optional tracking
    sqlx::query(
        r#"
        INSERT INTO indicator_downloads (user_id, indicator_id, file_id, download_token, token_expires_at, ownership_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(user_id as i32)
    .bind(indicator.0)
    .bind(file_id)
    .bind(&token)
    .bind(expires_at)
    .bind(has_access.map(|a| a.0 as i32).unwrap_or(0))
    .execute(&state.db.pool)
    .await
    .ok();

    // Build secure download URL
    let base_url = std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "https://api.revolution-trading.com".to_string());
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
    let file: IndicatorFile = sqlx::query_as("SELECT * FROM indicator_files WHERE id = $1")
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
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file.file_name),
        )
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
// LICENSE KEY MANAGEMENT (ICT 7 Grade - February 2026)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7: Generate a license key for an indicator
fn generate_license_key(user_id: i64, indicator_id: i64) -> String {
    use sha2::{Digest, Sha256};
    let secret = std::env::var("MEMBER_LICENSE_SECRET")
        .unwrap_or_else(|_| "".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let input = format!("{}-{}-{}-{}", user_id, indicator_id, timestamp, secret);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    // Format: XXXX-XXXX-XXXX-XXXX (16 chars from hash)
    let key_chars: String = hash.chars().take(16).collect();
    format!(
        "{}-{}-{}-{}",
        &key_chars[0..4].to_uppercase(),
        &key_chars[4..8].to_uppercase(),
        &key_chars[8..12].to_uppercase(),
        &key_chars[12..16].to_uppercase()
    )
}

/// ICT 7: Validate a license key
async fn validate_license_key(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<LicenseValidateParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let license_key = params.license_key.as_deref().unwrap_or("");

    if license_key.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "License key is required", "valid": false})),
        ));
    }

    // Get indicator
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let indicator_id = match indicator {
        Some((id,)) => id,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found", "valid": false})),
            ));
        }
    };

    // Check if license key matches any user's ownership
    let ownership: Option<(
        i64,
        i64,
        chrono::NaiveDateTime,
        Option<chrono::NaiveDateTime>,
    )> = sqlx::query_as(
        r#"
        SELECT id, user_id, purchased_at, expires_at
        FROM user_indicators
        WHERE indicator_id = $1 AND license_key = $2
        "#,
    )
    .bind(indicator_id)
    .bind(license_key)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    match ownership {
        Some((id, user_id, purchased_at, expires_at)) => {
            // Check if expired
            let is_expired = expires_at
                .map(|exp| exp < chrono::Utc::now().naive_utc())
                .unwrap_or(false);

            if is_expired {
                return Ok(Json(json!({
                    "success": true,
                    "valid": false,
                    "reason": "License has expired",
                    "expires_at": expires_at
                })));
            }

            Ok(Json(json!({
                "success": true,
                "valid": true,
                "license_key": license_key,
                "user_id": user_id,
                "purchased_at": purchased_at.and_utc().to_rfc3339(),
                "expires_at": expires_at.map(|e| e.and_utc().to_rfc3339()),
                "is_lifetime": expires_at.is_none()
            })))
        }
        None => Ok(Json(json!({
            "success": true,
            "valid": false,
            "reason": "Invalid license key"
        }))),
    }
}

#[derive(Debug, serde::Deserialize)]
struct LicenseValidateParams {
    license_key: Option<String>,
}

/// ICT 7: Get or generate license key for owned indicator
async fn get_license_key(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get indicator
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let indicator_id = match indicator {
        Some((id,)) => id,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found"})),
            ));
        }
    };

    // Check ownership and get existing license key
    let ownership: Option<(i64, Option<String>)> = sqlx::query_as(
        "SELECT id, license_key FROM user_indicators WHERE user_id = $1 AND indicator_id = $2",
    )
    .bind(user_id)
    .bind(indicator_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    match ownership {
        Some((ownership_id, existing_key)) => {
            let license_key = match existing_key {
                Some(key) if !key.is_empty() => key,
                _ => {
                    // Generate new license key
                    let new_key = generate_license_key(user_id, indicator_id);
                    // Save to database
                    sqlx::query("UPDATE user_indicators SET license_key = $1 WHERE id = $2")
                        .bind(&new_key)
                        .bind(ownership_id)
                        .execute(&state.db.pool)
                        .await
                        .ok();
                    new_key
                }
            };

            Ok(Json(json!({
                "success": true,
                "data": {
                    "license_key": license_key,
                    "indicator_id": indicator_id
                }
            })))
        }
        None => Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You do not own this indicator"})),
        )),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// USER DOWNLOAD HISTORY (ICT 7 Grade - February 2026)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct DownloadHistoryRow {
    id: i32,
    indicator_id: i64,
    indicator_name: Option<String>,
    indicator_slug: Option<String>,
    file_id: i32,
    file_name: Option<String>,
    platform: Option<String>,
    downloaded_at: Option<chrono::NaiveDateTime>,
    status: Option<String>,
}

/// ICT 7: Get user's download history
async fn get_download_history(
    user: User,
    State(state): State<AppState>,
    Query(params): Query<DownloadHistoryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let downloads: Vec<DownloadHistoryRow> = sqlx::query_as(
        r#"
        SELECT
            d.id,
            d.indicator_id,
            i.name as indicator_name,
            i.slug as indicator_slug,
            d.file_id,
            f.file_name,
            f.platform,
            d.downloaded_at,
            d.status
        FROM indicator_downloads d
        LEFT JOIN indicators i ON d.indicator_id = i.id
        LEFT JOIN indicator_files f ON d.file_id = f.id
        WHERE d.user_id = $1
        ORDER BY d.downloaded_at DESC NULLS LAST, d.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id as i32)
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM indicator_downloads WHERE user_id = $1")
            .bind(user_id as i32)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": {
            "downloads": downloads,
            "total": total.0,
            "page": page,
            "per_page": per_page,
            "total_pages": ((total.0 as f64) / (per_page as f64)).ceil() as i32
        }
    })))
}

#[derive(Debug, serde::Deserialize)]
struct DownloadHistoryParams {
    page: Option<i32>,
    per_page: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// UPDATE NOTIFICATIONS (ICT 7 Grade - February 2026)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7: Check for indicator updates for owned indicators
async fn check_updates(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get user's owned indicators with update info
    #[allow(clippy::type_complexity)]
    let updates: Vec<(
        i64,
        String,
        String,
        Option<String>,
        Option<String>,
        chrono::NaiveDateTime,
    )> = sqlx::query_as(
        r#"
        SELECT
            i.id,
            i.name,
            i.slug,
            i.version,
            (SELECT f.version FROM indicator_files f
             WHERE f.indicator_id = i.id AND f.is_current_version = true
             LIMIT 1) as latest_file_version,
            COALESCE(i.updated_at, i.created_at) as last_updated
        FROM user_indicators ui
        JOIN indicators i ON ui.indicator_id = i.id
        WHERE ui.user_id = $1 AND i.is_active = true
        ORDER BY i.updated_at DESC NULLS LAST
        "#,
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let updates_list: Vec<serde_json::Value> = updates
        .into_iter()
        .filter_map(|(id, name, slug, version, file_version, updated_at)| {
            // Check if there's a newer version available
            let has_update = version
                .as_ref()
                .and_then(|v| file_version.as_ref().map(|fv| fv != v))
                .unwrap_or(false);

            if has_update {
                Some(json!({
                    "indicator_id": id,
                    "name": name,
                    "slug": slug,
                    "current_version": version,
                    "latest_version": file_version,
                    "updated_at": updated_at.and_utc().to_rfc3339()
                }))
            } else {
                None
            }
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "updates_available": !updates_list.is_empty(),
            "count": updates_list.len(),
            "updates": updates_list
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INSTALLATION GUIDES (ICT 7 Grade - February 2026)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7: Get installation guide for a specific platform
async fn get_installation_guide(
    State(state): State<AppState>,
    Path((slug, platform)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get indicator
    let indicator: Option<(i64, Option<String>)> =
        sqlx::query_as("SELECT id, documentation_url FROM indicators WHERE slug = $1")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();

    let (indicator_id, doc_url) = match indicator {
        Some(data) => data,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found"})),
            ));
        }
    };

    // Get platform-specific installation guide from trading_platforms table
    let platform_guide: Option<(String, Option<String>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT name, install_instructions, documentation_url
        FROM trading_platforms
        WHERE LOWER(slug) = LOWER($1) OR LOWER(name) = LOWER($1)
        "#,
    )
    .bind(&platform)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    // Get platform-specific files for this indicator
    let files: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT file_name, display_name, version
        FROM indicator_files
        WHERE indicator_id = $1 AND LOWER(platform) = LOWER($2) AND is_active = true
        ORDER BY is_current_version DESC, display_order
        "#,
    )
    .bind(indicator_id)
    .bind(&platform)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let guide = match platform_guide {
        Some((name, instructions, platform_doc_url)) => {
            json!({
                "platform": name,
                "instructions": instructions.unwrap_or_else(|| get_default_instructions(&platform)),
                "documentation_url": platform_doc_url.or(doc_url),
                "files": files.into_iter().map(|(file_name, display_name, version)| {
                    json!({
                        "file_name": file_name,
                        "display_name": display_name,
                        "version": version
                    })
                }).collect::<Vec<_>>()
            })
        }
        None => {
            // Return default instructions for the platform
            json!({
                "platform": platform,
                "instructions": get_default_instructions(&platform),
                "documentation_url": doc_url,
                "files": files.into_iter().map(|(file_name, display_name, version)| {
                    json!({
                        "file_name": file_name,
                        "display_name": display_name,
                        "version": version
                    })
                }).collect::<Vec<_>>()
            })
        }
    };

    Ok(Json(json!({
        "success": true,
        "data": guide
    })))
}

/// Get default installation instructions for a platform
fn get_default_instructions(platform: &str) -> String {
    match platform.to_lowercase().as_str() {
        "thinkorswim" | "tos" => r#"
## ThinkorSwim Installation

1. Download the indicator file (.ts extension)
2. Open ThinkorSwim and go to Setup > Open Shared Item
3. Paste the shareable link or import the .ts file
4. The indicator will appear in your Studies menu
5. Apply to any chart by right-clicking and selecting Studies > Add Study

**Tip:** Make sure to save the workspace after adding the indicator.
"#.to_string(),
        "tradingview" => r#"
## TradingView Installation

1. This indicator requires TradingView access
2. Go to your TradingView account
3. Open the Pine Script editor (at the bottom of your chart)
4. Click "Indicators" and search for the indicator name
5. If it's invite-only, ensure your TradingView username is registered

**Note:** TradingView indicators are cloud-based and don't require file downloads.
"#.to_string(),
        "metatrader" | "mt4" | "mt5" => r#"
## MetaTrader Installation

1. Download the indicator file (.mq4/.mq5 or .ex4/.ex5)
2. Open MetaTrader and go to File > Open Data Folder
3. Navigate to MQL4/Indicators (or MQL5/Indicators)
4. Copy the downloaded file into this folder
5. Restart MetaTrader or refresh the Navigator panel
6. Find the indicator in Navigator > Indicators

**Tip:** Make sure to compile .mq4/.mq5 files if needed.
"#.to_string(),
        "ninjatrader" => r#"
## NinjaTrader Installation

1. Download the indicator .zip file
2. Open NinjaTrader Control Center
3. Go to Tools > Import > NinjaScript Add-On
4. Select the downloaded .zip file
5. Follow the import wizard
6. The indicator will appear in your Indicators list

**Note:** Restart NinjaTrader if the indicator doesn't appear immediately.
"#.to_string(),
        "tradestation" => r#"
## TradeStation Installation

1. Download the indicator .eld file
2. Open TradeStation
3. Go to File > Import/Export EasyLanguage
4. Select Import EasyLanguage file
5. Browse to the downloaded .eld file
6. Follow the import wizard

**Tip:** You may need to verify the indicator in the TradeStation scanner.
"#.to_string(),
        _ => format!(
            "## {} Installation\n\n1. Download the indicator file\n2. Follow the platform's standard import process\n3. Consult the platform's documentation for detailed steps\n\nContact support if you need assistance.",
            platform
        ),
    }
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
        .route("/history", get(get_download_history))
        .route("/updates", get(check_updates))
        .route("/:slug", get(get_indicator_downloads))
        .route("/:slug/download/:file_id", get(generate_download_url))
        .route("/:slug/license", get(get_license_key))
        .route("/:slug/validate", get(validate_license_key))
        .route("/:slug/guide/:platform", get(get_installation_guide))
}

pub fn download_router() -> Router<AppState> {
    Router::new().route("/indicator/:slug/:file_id", get(download_file))
}
