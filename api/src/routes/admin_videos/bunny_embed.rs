//! Bunny.net integration: webhook receiver, transcoding status, thumbnail
//! generation, and embed-code rendering.
//!
//! ICT 7 ADDITIONS: Transcoding, Thumbnails, Embed Code

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

/// POST /video-advanced/bunny/webhook - Handle Bunny.net transcoding webhook
/// ICT 7 ADDITION: Transcoding status tracking
pub(super) async fn bunny_webhook(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    // Extract webhook data from Bunny.net
    let video_guid = payload
        .get("VideoGuid")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let status = payload.get("Status").and_then(|v| v.as_i64()).unwrap_or(0);

    // Map Bunny status codes: 0=Queued, 1=Processing, 2=Encoding, 3=Finished, 4=Resolution Finished, 5=Failed
    let encoding_status = match status {
        0 => "queued",
        1 => "processing",
        2 => "encoding",
        3 | 4 => "completed",
        5 => "failed",
        _ => "unknown",
    };

    // Extract video metadata from webhook
    let width = payload
        .get("Width")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let height = payload
        .get("Height")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);
    let duration = payload
        .get("Length")
        .and_then(|v| v.as_f64())
        .map(|v| v as i32);
    let thumbnail_url = payload.get("ThumbnailFileName").and_then(|v| v.as_str());

    // Update video record with transcoding status
    if !video_guid.is_empty() {
        let mut query = String::from(
            "UPDATE unified_videos SET bunny_encoding_status = $1, updated_at = NOW()",
        );
        let mut param_idx = 2;

        if duration.is_some() {
            query.push_str(&format!(", duration = ${param_idx}"));
            param_idx += 1;
        }
        if thumbnail_url.is_some() {
            query.push_str(&format!(", bunny_thumbnail_url = ${param_idx}"));
            param_idx += 1;
        }
        if let (Some(_w), Some(_h)) = (width, height) {
            query.push_str(&format!(
                ", metadata = jsonb_set(COALESCE(metadata, '{{}}'::jsonb), '{{resolution}}', jsonb_build_object('width', ${}::int, 'height', ${}::int))",
                param_idx, param_idx + 1
            ));
            param_idx += 2;
        }

        query.push_str(&format!(" WHERE bunny_video_guid = ${param_idx}"));

        let mut sqlx_query = sqlx::query(sqlx::AssertSqlSafe(query.as_str())).bind(encoding_status);

        if let Some(dur) = duration {
            sqlx_query = sqlx_query.bind(dur);
        }
        if let Some(thumb) = thumbnail_url {
            sqlx_query = sqlx_query.bind(thumb);
        }
        if let (Some(w), Some(h)) = (width, height) {
            sqlx_query = sqlx_query.bind(w);
            sqlx_query = sqlx_query.bind(h);
        }
        sqlx_query = sqlx_query.bind(video_guid);

        let _ = sqlx_query.execute(&state.db.pool).await;
    }

    Json(json!({
        "success": true,
        "message": "Webhook processed",
        "video_guid": video_guid,
        "encoding_status": encoding_status
    }))
}

/// GET /video-advanced/videos/:id/transcoding-status
/// ICT 7 ADDITION: Check transcoding status for a video
pub(super) async fn get_transcoding_status(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[allow(clippy::type_complexity)]
    let status: Option<(Option<String>, Option<String>, Option<i32>, Option<serde_json::Value>)> = sqlx::query_as(
        "SELECT bunny_encoding_status, bunny_thumbnail_url, duration, metadata FROM unified_videos WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match status {
        Some((encoding_status, thumbnail_url, duration, metadata)) => {
            let is_ready = encoding_status.as_deref() == Some("completed");
            let status_str = encoding_status.unwrap_or_else(|| "unknown".to_string());
            Ok(Json(json!({
                "success": true,
                "data": {
                    "video_id": id,
                    "encoding_status": status_str,
                    "is_ready": is_ready,
                    "thumbnail_url": thumbnail_url,
                    "duration": duration,
                    "metadata": metadata
                }
            })))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        )),
    }
}

/// POST /video-advanced/videos/:id/generate-thumbnail
/// ICT 7 ADDITION: Trigger thumbnail generation for Bunny video
pub(super) async fn generate_thumbnail(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get video's Bunny GUID and library ID
    let video: Option<(Option<String>, Option<i64>)> = sqlx::query_as(
        "SELECT bunny_video_guid, bunny_library_id FROM unified_videos WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match video {
        Some((Some(guid), Some(library_id))) => {
            // Generate thumbnail URL from timestamp
            let timestamp = input
                .get("timestamp_seconds")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let thumbnail_url =
                format!("https://vz-{library_id}.b-cdn.net/{guid}/thumbnail.jpg?time={timestamp}");

            // Update video record with new thumbnail
            let _ = sqlx::query(
                "UPDATE unified_videos SET thumbnail_url = $1, bunny_thumbnail_url = $1, updated_at = NOW() WHERE id = $2"
            )
            .bind(&thumbnail_url)
            .bind(id)
            .execute(&state.db.pool)
            .await;

            Ok(Json(json!({
                "success": true,
                "data": {
                    "video_id": id,
                    "thumbnail_url": thumbnail_url,
                    "timestamp_seconds": timestamp
                }
            })))
        }
        _ => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Video does not have Bunny CDN configuration"})),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct EmbedCodeQuery {
    width: Option<i32>,
    height: Option<i32>,
    autoplay: Option<bool>,
    responsive: Option<bool>,
}

/// GET /video-advanced/videos/:id/embed-code
/// ICT 7 ADDITION: Generate embed code for video
pub(super) async fn get_embed_code(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(params): Query<EmbedCodeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: Option<(String, Option<String>, Option<i64>, String)> = sqlx::query_as(
        "SELECT video_platform, bunny_video_guid, bunny_library_id, title FROM unified_videos WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    match video {
        Some((platform, guid, library_id, title)) => {
            let width = params.width.unwrap_or(640);
            let height = params.height.unwrap_or(360);
            let autoplay = params.autoplay.unwrap_or(false);
            let responsive = params.responsive.unwrap_or(true);

            let (iframe_url, embed_html) = match platform.as_str() {
                "bunny" => {
                    if let (Some(g), Some(lib)) = (guid, library_id) {
                        let url = format!(
                            "https://iframe.mediadelivery.net/embed/{lib}/{g}?autoplay={autoplay}&responsive={responsive}"
                        );
                        let html = if responsive {
                            format!(
                                r#"<div style="position:relative;padding-top:56.25%;"><iframe src="{url}" loading="lazy" style="border:none;position:absolute;top:0;height:100%;width:100%;" allow="accelerometer;gyroscope;autoplay;encrypted-media;picture-in-picture;" allowfullscreen="true"></iframe></div>"#
                            )
                        } else {
                            format!(
                                r#"<iframe src="{url}" width="{width}" height="{height}" loading="lazy" style="border:none;" allow="accelerometer;gyroscope;autoplay;encrypted-media;picture-in-picture;" allowfullscreen="true"></iframe>"#
                            )
                        };
                        (url, html)
                    } else {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": "Missing Bunny configuration"})),
                        ));
                    }
                }
                _ => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Platform not supported for embed code generation"})),
                    ))
                }
            };

            Ok(Json(json!({
                "success": true,
                "data": {
                    "video_id": id,
                    "title": title,
                    "platform": platform,
                    "iframe_url": iframe_url,
                    "embed_html": embed_html,
                    "settings": {
                        "width": width,
                        "height": height,
                        "autoplay": autoplay,
                        "responsive": responsive
                    }
                }
            })))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Video not found"})),
        )),
    }
}
