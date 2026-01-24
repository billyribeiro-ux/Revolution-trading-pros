/**
 * Bunny.net Video Upload API Routes
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple Principal Engineer ICT 7 Grade - January 2026
 *
 * Handles video uploads to Bunny.net Stream:
 * - Create video entry and get upload URL
 * - Check video processing status
 * - Track uploads in database
 *
 * @version 1.0.0
 */
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub library_id: Option<i64>,
    pub collection_id: Option<String>,
    pub room_slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateVideoResponse {
    pub success: bool,
    pub video_guid: String,
    pub upload_url: String,
    pub video_url: String,
    pub embed_url: String,
}

#[derive(Debug, Serialize)]
pub struct VideoStatusResponse {
    pub success: bool,
    pub status: String,
    pub status_code: i32,
    pub video_url: Option<String>,
    pub embed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════════════
// BUNNY API CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

const BUNNY_API_BASE: &str = "https://video.bunnycdn.com";
const BUNNY_STREAM_CDN: &str = "https://vz-2a93ec0c-6c4.b-cdn.net";
const DEFAULT_LIBRARY_ID: i64 = 389539;

fn get_bunny_api_key() -> String {
    std::env::var("BUNNY_STREAM_API_KEY")
        .or_else(|_| std::env::var("BUNNY_API_KEY"))
        .unwrap_or_default()
}

/// Format room slug into human-readable name for Bunny.net dashboard
fn format_room_name(room_slug: &str) -> String {
    match room_slug {
        "explosive-swings" => "Explosive Swings".to_string(),
        "spx-profit-pulse" => "SPX Profit Pulse".to_string(),
        "day-trading-room" => "Day Trading Room".to_string(),
        "swing-trading-room" => "Swing Trading Room".to_string(),
        "small-account-mentorship" => "Small Account Mentorship".to_string(),
        _ => {
            // Fallback: capitalize each word
            room_slug
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/admin/bunny/create-video - Create video entry and get upload URL
async fn create_video(
    State(state): State<AppState>,
    Json(input): Json<CreateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let api_key = get_bunny_api_key();
    if api_key.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": "Bunny API key not configured"})),
        ));
    }

    let library_id = input.library_id.unwrap_or(DEFAULT_LIBRARY_ID);

    // Format title with room prefix for Bunny.net dashboard organization
    let bunny_title = if let Some(ref room_slug) = input.room_slug {
        let room_name = format_room_name(room_slug);
        format!("[{}] {}", room_name, input.title)
    } else {
        input.title.clone()
    };

    // Create video in Bunny.net
    let client = reqwest::Client::new();
    let create_url = format!("{}/library/{}/videos", BUNNY_API_BASE, library_id);

    let mut body = json!({
        "title": bunny_title
    });

    if let Some(ref collection_id) = input.collection_id {
        body["collectionId"] = json!(collection_id);
    }

    let response = client
        .post(&create_url)
        .header("AccessKey", &api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"success": false, "error": format!("Bunny API error: {}", e)})),
            )
        })?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"success": false, "error": format!("Bunny API error: {}", error_text)})),
        ));
    }

    let bunny_response: serde_json::Value = response.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": format!("Parse error: {}", e)})),
        )
    })?;

    let video_guid = bunny_response["guid"].as_str().unwrap_or("");

    // Generate URLs
    let upload_url = format!(
        "{}/library/{}/videos/{}",
        BUNNY_API_BASE, library_id, video_guid
    );
    let video_url = format!("{}/{}/play_720p.mp4", BUNNY_STREAM_CDN, video_guid);
    let embed_url = format!(
        "https://iframe.mediadelivery.net/embed/{}/{}",
        library_id, video_guid
    );

    // Track upload in database with room_slug
    let _ = sqlx::query(
        r#"
        INSERT INTO bunny_uploads (video_guid, library_id, title, room_slug, status, upload_started_at)
        VALUES ($1, $2, $3, $4, 'uploading', NOW())
        ON CONFLICT (video_guid) DO UPDATE SET
            status = 'uploading',
            room_slug = EXCLUDED.room_slug,
            upload_started_at = NOW()
        "#,
    )
    .bind(video_guid)
    .bind(library_id)
    .bind(&input.title)
    .bind(&input.room_slug)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "success": true,
        "video_guid": video_guid,
        "upload_url": upload_url,
        "video_url": video_url,
        "embed_url": embed_url,
        "library_id": library_id
    })))
}

/// GET /api/admin/bunny/video-status/:guid - Check video processing status
async fn get_video_status(
    State(state): State<AppState>,
    Path(video_guid): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let api_key = get_bunny_api_key();
    if api_key.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": "Bunny API key not configured"})),
        ));
    }

    // Get library ID from database or use default
    let library_id: i64 =
        sqlx::query_scalar::<_, i64>("SELECT library_id FROM bunny_uploads WHERE video_guid = $1")
            .bind(&video_guid)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
            .unwrap_or(DEFAULT_LIBRARY_ID);

    // Get video status from Bunny.net
    let client = reqwest::Client::new();
    let status_url = format!(
        "{}/library/{}/videos/{}",
        BUNNY_API_BASE, library_id, video_guid
    );

    let response = client
        .get(&status_url)
        .header("AccessKey", &api_key)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"success": false, "error": format!("Bunny API error: {}", e)})),
            )
        })?;

    if !response.status().is_success() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"success": false, "error": "Video not found"})),
        ));
    }

    let bunny_video: serde_json::Value = response.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": format!("Parse error: {}", e)})),
        )
    })?;

    // Status codes: 0=created, 1=uploaded, 2=processing, 3=transcoding, 4=finished, 5=error
    let status_code = bunny_video["status"].as_i64().unwrap_or(0) as i32;
    let status = match status_code {
        0 => "created",
        1 => "uploaded",
        2 => "processing",
        3 => "transcoding",
        4 => "ready",
        5 => "failed",
        _ => "unknown",
    };

    let duration = bunny_video["length"].as_i64().map(|l| l as i32);
    let thumbnail_url = bunny_video["thumbnailFileName"]
        .as_str()
        .map(|t| format!("{}/{}/{}", BUNNY_STREAM_CDN, video_guid, t));

    // Update database status
    if status == "ready" || status == "failed" {
        let _ = sqlx::query(
            r#"
            UPDATE bunny_uploads SET 
                status = $1,
                duration_seconds = $2,
                thumbnail_url = $3,
                upload_completed_at = CASE WHEN $1 = 'ready' THEN NOW() ELSE upload_completed_at END
            WHERE video_guid = $4
            "#,
        )
        .bind(status)
        .bind(duration)
        .bind(&thumbnail_url)
        .bind(&video_guid)
        .execute(&state.db.pool)
        .await;
    }

    let video_url = format!("{}/{}/play_720p.mp4", BUNNY_STREAM_CDN, video_guid);
    let embed_url = format!(
        "https://iframe.mediadelivery.net/embed/{}/{}",
        library_id, video_guid
    );

    Ok(Json(json!({
        "success": true,
        "status": status,
        "status_code": status_code,
        "video_url": video_url,
        "embed_url": embed_url,
        "thumbnail_url": thumbnail_url,
        "duration": duration
    })))
}

/// GET /api/admin/bunny/uploads - List recent uploads
async fn list_uploads(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[allow(clippy::type_complexity)]
    let uploads: Vec<(String, i64, String, String, Option<i32>, Option<String>, chrono::NaiveDateTime)> =
        sqlx::query_as::<_, (String, i64, String, String, Option<i32>, Option<String>, chrono::NaiveDateTime)>(
            r#"
            SELECT video_guid, library_id, title, status, duration_seconds, thumbnail_url, upload_started_at
            FROM bunny_uploads
            ORDER BY upload_started_at DESC
            LIMIT 50
            "#
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    let uploads_json: Vec<serde_json::Value> = uploads
        .into_iter()
        .map(|u| {
            json!({
                "video_guid": u.0,
                "library_id": u.1,
                "title": u.2,
                "status": u.3,
                "duration": u.4,
                "thumbnail_url": u.5,
                "started_at": u.6.to_string()
            })
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": uploads_json
    })))
}

/// PUT /api/admin/bunny/upload - Upload video file to Bunny.net
async fn upload_video(
    State(_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let api_key = get_bunny_api_key();
    if api_key.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": "Bunny API key not configured"})),
        ));
    }

    let video_guid = params.get("video_guid").ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": "video_guid is required"})),
        )
    })?;

    let library_id: i64 = params
        .get("library_id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_LIBRARY_ID);

    if body.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "error": "No file data provided"})),
        ));
    }

    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("video/mp4");

    tracing::info!(
        "Uploading {} bytes to Bunny.net video {} in library {}",
        body.len(),
        video_guid,
        library_id
    );

    // Upload to Bunny.net
    let client = reqwest::Client::new();
    let upload_url = format!(
        "{}/library/{}/videos/{}",
        BUNNY_API_BASE, library_id, video_guid
    );

    let response = client
        .put(&upload_url)
        .header("AccessKey", &api_key)
        .header("Content-Type", content_type)
        .body(body.to_vec())
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Bunny upload error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"success": false, "error": format!("Bunny API error: {}", e)})),
            )
        })?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        tracing::error!("Bunny upload failed: {}", error_text);
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"success": false, "error": format!("Upload failed: {}", error_text)})),
        ));
    }

    let embed_url = format!(
        "https://iframe.mediadelivery.net/embed/{}/{}",
        library_id, video_guid
    );
    let video_url = format!("{}/{}/play_720p.mp4", BUNNY_STREAM_CDN, video_guid);

    Ok(Json(json!({
        "success": true,
        "video_guid": video_guid,
        "embed_url": embed_url,
        "video_url": video_url
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/create-video", post(create_video))
        .route("/upload", axum::routing::put(upload_video))
        .route("/video-status/:guid", get(get_video_status))
        .route("/uploads", get(list_uploads))
}
