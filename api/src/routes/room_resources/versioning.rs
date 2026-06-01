//! Room resources versioning routes (split from `room_resources.rs`
//! lines 1201-1285).
//!
//! Version-history retrieval (public) and admin-only new-version creation.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::helpers::resource_to_response;
use super::{ResourceResponse, RoomResource};

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: VERSION HISTORY
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources/:id/versions - Get version history for a resource
pub(super) async fn get_version_history(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get all versions of this resource (including current and previous)
    let versions: Vec<RoomResource> = sqlx::query_as(
        r"
        WITH RECURSIVE version_chain AS (
            SELECT * FROM room_resources WHERE id = $1 AND deleted_at IS NULL
            UNION ALL
            SELECT r.* FROM room_resources r
            INNER JOIN version_chain vc ON r.id = vc.previous_version_id
            WHERE r.deleted_at IS NULL
        )
        SELECT * FROM version_chain ORDER BY version DESC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let responses: Vec<ResourceResponse> = versions.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total_versions": responses.len()
    })))
}

/// POST /api/admin/room-resources/:id/new-version - Create a new version of resource
pub(super) async fn create_new_version(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateVersionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let new_id: i64 = sqlx::query_scalar("SELECT create_resource_version($1, $2, $3)")
        .bind(id)
        .bind(&input.file_url)
        .bind(input.file_size)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Get the new resource
    let resource: RoomResource = sqlx::query_as("SELECT * FROM room_resources WHERE id = $1")
        .bind(new_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "New version created successfully",
        "data": resource_to_response(resource)
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub file_url: String,
    pub file_size: Option<i64>,
}
