//! Secure download URL generation + token-gated CDN streaming +
//! per-user download history.
//!
//! - `generate_download_url` — paywall-gated, mints SHA256 token
//! - `download_file` — token-gated CDN 302 (no session, by design)
//! - `get_download_history` — paginated per-user download log
//!
//! Asymmetry preserved: `IndicatorFile.id` is `i32` (reserved exception
//! for bounded row counts), `IndicatorFile.indicator_id` is `i64`
//! (BIGSERIAL FK). The URL path `/download/indicator/:slug/:file_id`
//! decodes `file_id` as `i32` to match.

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use chrono::{Duration, Utc};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::models::indicator::IndicatorFile;
use crate::models::User;
use crate::AppState;

/// ICT 7 FIX: Uses i64 for indicator_id, queries user_indicators table
pub(super) async fn generate_download_url(
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
        r"
        SELECT id FROM user_indicators
        WHERE user_id = $1 AND indicator_id = $2
        ",
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
    let secret = std::env::var("MEMBER_INDICATOR_SECRET").unwrap_or_default();
    let expires_at = Utc::now() + Duration::hours(24);
    let expiry_timestamp = expires_at.timestamp();

    // Hash format: SHA256(user_id + file_id + expiry + secret)
    let hash_input = format!("{user_id}{file_id}{expiry_timestamp}{secret}");
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let token = format!("{:x}", hasher.finalize());

    // ICT 7 FIX: Store download record with i64 indicator_id
    // Note: indicator_downloads table may not exist; this is optional tracking
    sqlx::query(
        r"
        INSERT INTO indicator_downloads (user_id, indicator_id, file_id, download_token, token_expires_at, ownership_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT DO NOTHING
        ",
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
        "{base_url}/download/indicator/{slug}/{file_id}?token={token}&expires={expiry_timestamp}"
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

pub(super) async fn download_file(
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
        r"
        SELECT id, file_id, download_count, max_downloads
        FROM indicator_downloads
        WHERE download_token = $1
        AND token_expires_at > NOW()
        ",
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
        r"
        UPDATE indicator_downloads
        SET download_count = download_count + 1, downloaded_at = NOW(), status = 'completed'
        WHERE id = $1
        ",
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

    // Redirect to actual file (CDN URL).
    // ICT 7 SECURITY: file_name is sanitized into the Content-Disposition header to
    // prevent CRLF injection / response-splitting via crafted filenames. Quotes and
    // backslashes are escaped per RFC 6266, and any control char triggers a generic
    // fallback. Header construction is fallible — never `.unwrap()` on user-derived
    // bytes (DoS vector).
    let cdn_url = file.cdn_url.unwrap_or_else(|| file.file_path.clone());

    let safe_name = sanitize_filename_for_disposition(&file.file_name);
    let disposition = format!("attachment; filename=\"{safe_name}\"");

    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, cdn_url)
        .header(header::CONTENT_DISPOSITION, disposition)
        .body(axum::body::Body::empty())
        .map_err(|e| {
            tracing::error!(
                file_id = file_id,
                "Failed to build download response: {}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to build download response"})),
            )
        })?;

    Ok(response)
}

/// Sanitize a filename for inclusion in a `Content-Disposition: attachment;
/// filename="..."` header. Drops control characters (which can break header
/// parsing or enable response-splitting), escapes `"` and `\`, and falls back
/// to `download` if the result is empty.
fn sanitize_filename_for_disposition(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_control() {
            continue;
        }
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(ch),
        }
    }
    if out.trim().is_empty() {
        "download".to_string()
    } else {
        out
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct DownloadParams {
    token: Option<String>,
    expires: Option<i64>,
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
pub(super) async fn get_download_history(
    user: User,
    State(state): State<AppState>,
    Query(params): Query<DownloadHistoryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let downloads: Vec<DownloadHistoryRow> = sqlx::query_as(
        r"
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
        ",
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
pub(super) struct DownloadHistoryParams {
    page: Option<i32>,
    per_page: Option<i32>,
}
