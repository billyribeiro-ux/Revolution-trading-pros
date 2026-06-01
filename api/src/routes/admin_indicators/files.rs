//! ICT 7 Grade — File Management endpoints for indicators.
//! Multi-platform file downloads with version management.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::types::{CreateFileRequest, IndicatorFileRow, UpdateFileRequest};

/// ICT 7: List all files for an indicator
pub(super) async fn list_indicator_files(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let files: Vec<IndicatorFileRow> = sqlx::query_as(
        r"
        SELECT * FROM indicator_files
        WHERE indicator_id = $1
        ORDER BY platform, display_order, created_at DESC
        ",
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

/// ICT 7: Add a new file to an indicator
pub(super) async fn create_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
    Json(input): Json<CreateFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify indicator exists
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE id = $1")
        .bind(indicator_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if indicator.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Indicator not found"})),
        ));
    }

    // If this is marked as current version, unmark other files for same platform
    if input.version.is_some() {
        let _ = sqlx::query(
            r"
            UPDATE indicator_files
            SET is_current_version = false
            WHERE indicator_id = $1 AND platform = $2
            ",
        )
        .bind(indicator_id)
        .bind(&input.platform)
        .execute(&state.db.pool)
        .await;
    }

    let file: IndicatorFileRow = sqlx::query_as(
        r"
        INSERT INTO indicator_files (
            indicator_id, file_name, original_name, file_path, file_size_bytes,
            file_type, mime_type, platform, platform_version, storage_key, cdn_url,
            version, changelog, display_name, display_order, is_current_version, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, true, true)
        RETURNING *
        ",
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
    .bind(&input.version)
    .bind(&input.changelog)
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
        "message": "File added successfully",
        "data": file
    })))
}

/// ICT 7: Update an indicator file
pub(super) async fn update_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i32)>,
    Json(input): Json<UpdateFileRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // If marking as current version, unmark others first
    if input.is_current_version == Some(true) {
        let file: Option<(String,)> = sqlx::query_as(
            "SELECT platform FROM indicator_files WHERE id = $1 AND indicator_id = $2",
        )
        .bind(file_id)
        .bind(indicator_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

        if let Some((platform,)) = file {
            let _ = sqlx::query(
                r"
                UPDATE indicator_files
                SET is_current_version = false
                WHERE indicator_id = $1 AND platform = $2 AND id != $3
                ",
            )
            .bind(indicator_id)
            .bind(&platform)
            .bind(file_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    let file: IndicatorFileRow = sqlx::query_as(
        r"
        UPDATE indicator_files SET
            display_name = COALESCE($1, display_name),
            display_order = COALESCE($2, display_order),
            platform_version = COALESCE($3, platform_version),
            version = COALESCE($4, version),
            changelog = COALESCE($5, changelog),
            is_current_version = COALESCE($6, is_current_version),
            is_active = COALESCE($7, is_active),
            updated_at = NOW()
        WHERE id = $8 AND indicator_id = $9
        RETURNING *
        ",
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
        "message": "File updated successfully",
        "data": file
    })))
}

/// ICT 7: Delete an indicator file
pub(super) async fn delete_indicator_file(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((indicator_id, file_id)): Path<(i64, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM indicator_files WHERE id = $1 AND indicator_id = $2")
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

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "File not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "File deleted successfully"
    })))
}
