//! Avatar management — upload + delete.
//!
//! R26-B split: handlers moved verbatim from `routes/user.rs`. File
//! validation (MIME allowlist, 5MB cap), unique filename generation,
//! upload-dir env override, and the `user_audit` tracing target are
//! preserved.

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{models::User, AppState};

/// Upload user avatar
/// POST /api/user/avatar
/// Accepts multipart form data with 'avatar' field
pub(super) async fn upload_avatar(
    State(state): State<AppState>,
    user: User,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Process multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Invalid form data: {}", e)})),
        )
    })? {
        let name = field.name().unwrap_or_default().to_string();
        if name == "avatar" {
            let content_type = field
                .content_type()
                .map(|ct| ct.to_string())
                .unwrap_or_default();

            // Validate content type
            if !["image/jpeg", "image/png", "image/gif", "image/webp"]
                .contains(&content_type.as_str())
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Invalid file type. Allowed: JPEG, PNG, GIF, WebP"
                    })),
                ));
            }

            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": format!("Failed to read file: {}", e)})),
                )
            })?;

            // Validate file size (max 5MB)
            if data.len() > 5 * 1024 * 1024 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "File too large. Maximum size is 5MB"})),
                ));
            }

            // Generate unique filename
            let ext = match content_type.as_str() {
                "image/jpeg" => "jpg",
                "image/png" => "png",
                "image/gif" => "gif",
                "image/webp" => "webp",
                _ => "jpg",
            };
            let filename = format!(
                "avatar_{}_{}.{}",
                user.id,
                chrono::Utc::now().timestamp(),
                ext
            );

            // Store avatar - use environment variable for upload path or default
            let upload_dir = std::env::var("AVATAR_UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads/avatars".to_string());

            // Ensure directory exists
            tokio::fs::create_dir_all(&upload_dir).await.map_err(|e| {
                tracing::error!("Failed to create avatar directory: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to save avatar"})),
                )
            })?;

            let file_path = format!("{upload_dir}/{filename}");
            tokio::fs::write(&file_path, &data).await.map_err(|e| {
                tracing::error!("Failed to write avatar file: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to save avatar"})),
                )
            })?;

            // Generate URL for avatar
            let avatar_url = format!("/uploads/avatars/{filename}");

            // Update user avatar_url in database
            sqlx::query("UPDATE users SET avatar_url = $1, updated_at = NOW() WHERE id = $2")
                .bind(&avatar_url)
                .bind(user.id)
                .execute(&state.db.pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    )
                })?;

            tracing::info!(
                target: "user_audit",
                event = "avatar_uploaded",
                user_id = %user.id,
                filename = %filename,
                "User avatar uploaded successfully"
            );

            return Ok(Json(json!({
                "success": true,
                "message": "Avatar uploaded successfully",
                "avatar_url": avatar_url
            })));
        }
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "No avatar file provided"})),
    ))
}

/// Delete user avatar
/// DELETE /api/user/avatar
pub(super) async fn delete_avatar(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current avatar URL
    if let Some(avatar_url) = &user.avatar_url {
        // Try to delete the file if it's a local file
        if avatar_url.starts_with("/uploads/") {
            let file_path = format!(".{avatar_url}");
            tokio::fs::remove_file(&file_path).await.ok(); // Ignore errors
        }
    }

    // Clear avatar_url in database
    sqlx::query("UPDATE users SET avatar_url = NULL, updated_at = NOW() WHERE id = $1")
        .bind(user.id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    tracing::info!(
        target: "user_audit",
        event = "avatar_deleted",
        user_id = %user.id,
        "User avatar deleted successfully"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Avatar removed successfully"
    })))
}
