//! Room resources secure-download routes (split from `room_resources.rs`
//! lines 1075-1199).
//!
//! Handles signed-token download URL generation and the download endpoint
//! that verifies tokens for premium resources.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{models::User, AppState};

use super::RoomResource;

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: SECURE DOWNLOAD
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/room-resources/:id/secure-download - Generate secure download URL
///
/// SECURITY (P0): requires an authenticated `User`. Previously unauthenticated,
/// this handler minted a signed download token and leaked `file_url` for any
/// resource to any caller.
pub(super) async fn generate_secure_download(
    State(state): State<AppState>,
    _user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Generate a secure token
    let token: String = sqlx::query_scalar("SELECT generate_secure_download_token($1, 24)")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Get the resource to include in response
    let resource: RoomResource =
        sqlx::query_as("SELECT * FROM room_resources WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Resource not found"})),
                )
            })?;

    // Construct the secure download URL
    let secure_url = format!("/api/room-resources/{id}/download?token={token}");

    // Track download attempt
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "success": true,
        "download_url": secure_url,
        "file_url": resource.file_url,
        "filename": format!("{}.{}", resource.slug, resource.mime_type.as_deref().unwrap_or("bin").split('/').next_back().unwrap_or("bin")),
        "expires_in_hours": 24
    })))
}

/// GET /api/room-resources/:id/download - Download with mandatory token verification
///
/// SECURITY (P0): requires an authenticated `User`, and for any non-`free`
/// resource the `token` query param is MANDATORY — a missing token is rejected
/// with 401 instead of falling through and returning `file_url`.
pub(super) async fn download_resource(
    State(state): State<AppState>,
    _user: User,
    Path(id): Path<i64>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the resource
    let resource: RoomResource = sqlx::query_as(
        "SELECT * FROM room_resources WHERE id = $1 AND is_published = true AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resource not found"})),
        )
    })?;

    // Check if resource requires premium access
    let access_level = resource.access_level.as_deref().unwrap_or("premium");
    if access_level != "free" {
        // SECURITY (P0): the token is MANDATORY for premium resources. A missing
        // token must never fall through to returning `file_url`; reject with 401.
        let token = params.get("token").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Download token required"})),
            )
        })?;

        let is_valid: bool = sqlx::query_scalar("SELECT validate_secure_download_token($1, $2)")
            .bind(id)
            .bind(token)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(false);

        if !is_valid {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Invalid or expired download token"})),
            ));
        }
    }

    // Track download
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "success": true,
        "file_url": resource.file_url,
        "filename": format!("{}.{}", resource.slug, resource.mime_type.as_deref().unwrap_or("bin").split('/').next_back().unwrap_or("bin")),
        "file_size": resource.file_size,
        "mime_type": resource.mime_type
    })))
}
