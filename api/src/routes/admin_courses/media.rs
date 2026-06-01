//! Downloads CRUD + video / file upload URL handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::models::course::{CourseDownload, CreateDownloadRequest, UpdateDownloadRequest};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOAD CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn list_downloads(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let downloads: Vec<CourseDownload> = sqlx::query_as(
        r"
        SELECT * FROM course_downloads
        WHERE course_id = $1 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $1)
        ORDER BY sort_order
        ",
    )
    .bind(course_id)
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
        "data": downloads
    })))
}

pub(super) async fn create_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<CreateDownloadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let download: CourseDownload = sqlx::query_as(
        r"
        INSERT INTO course_downloads (
            course_id, module_id, lesson_id, title, description, file_name, file_path,
            file_size_bytes, file_type, mime_type, download_url, category, sort_order, is_public
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING *
        ",
    )
    .bind(input.course_id.or(Some(course_id)))
    .bind(input.module_id)
    .bind(input.lesson_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.file_name)
    .bind(&input.file_path)
    .bind(input.file_size_bytes)
    .bind(&input.file_type)
    .bind(&input.mime_type)
    .bind(&input.download_url)
    .bind(input.category.as_deref().unwrap_or("resource"))
    .bind(input.sort_order.unwrap_or(0))
    .bind(input.is_public.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download created successfully",
        "data": download
    })))
}

pub(super) async fn update_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, download_id)): Path<(Uuid, i64)>,
    Json(input): Json<UpdateDownloadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let download: CourseDownload = sqlx::query_as(
        r"
        UPDATE course_downloads SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            category = COALESCE($3, category),
            sort_order = COALESCE($4, sort_order),
            is_public = COALESCE($5, is_public),
            updated_at = NOW()
        WHERE id = $6 AND (course_id = $7 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $7))
        RETURNING *
        ",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.category)
    .bind(input.sort_order)
    .bind(input.is_public)
    .bind(download_id)
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download updated successfully",
        "data": download
    })))
}

pub(super) async fn delete_download(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, download_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        "DELETE FROM course_downloads WHERE id = $1 AND (course_id = $2 OR lesson_id IN (SELECT id FROM lessons WHERE course_id = $2))",
    )
    .bind(download_id)
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to delete download: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Download deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO UPLOAD (Bunny Stream TUS)
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn create_video_upload(
    _admin: AdminUser,
    State(_state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input["title"].as_str().unwrap_or("Untitled Video");

    let library_id = std::env::var("BUNNY_STREAM_LIBRARY_ID")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let api_key = std::env::var("BUNNY_STREAM_API_KEY")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if library_id.is_empty() || api_key.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Bunny.net Stream not configured"})),
        ));
    }

    // Create video in Bunny Stream
    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://video.bunnycdn.com/library/{library_id}/videos"
        ))
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
    let expiry = chrono::Utc::now().timestamp() + 7200; // 2 hour expiry

    // Generate TUS upload URL with authentication
    let tus_url = format!(
        "https://video.bunnycdn.com/tusupload?libraryId={library_id}&videoId={video_guid}&expirationTime={expiry}"
    );

    // Generate authorization signature for TUS
    let signature_string = format!("{library_id}{api_key}{expiry}");
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
// FILE UPLOAD URL (Bunny Storage)
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn get_upload_url(
    _admin: AdminUser,
    State(_state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let file_name = input["file_name"].as_str().unwrap_or("file");
    let file_type = input["file_type"]
        .as_str()
        .unwrap_or("application/octet-stream");

    let storage_zone =
        std::env::var("BUNNY_STORAGE_ZONE").unwrap_or_else(|_| "revolution-downloads".to_string());
    let cdn_url = std::env::var("BUNNY_CDN_URL")
        .unwrap_or_else(|_| "https://revolution-downloads.b-cdn.net".to_string());

    let file_path = format!("courses/{course_id}/{file_name}");
    let upload_url = format!("https://storage.bunnycdn.com/{storage_zone}/{file_path}");
    let download_url = format!("{cdn_url}/{file_path}");

    // SECURITY: Do NOT return BUNNY_STORAGE_API_KEY to the client.
    // File uploads should go through a backend proxy endpoint instead of
    // exposing storage credentials to the frontend.
    Ok(Json(json!({
        "success": true,
        "data": {
            "upload_url": upload_url,
            "download_url": download_url,
            "file_path": file_path,
            "storage_zone": storage_zone,
            "content_type": file_type
        }
    })))
}
