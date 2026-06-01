//! Course resource (PDF, document, attachment) endpoints
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// RESOURCE ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a resource (PDF, document, etc.)
pub(super) async fn create_resource(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "resource_create", admin_id = %admin.id, course_id = %course_id, "Admin creating resource");
    // Get max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM course_resources WHERE course_id = $1 AND section_id IS NOT DISTINCT FROM $2 AND lesson_id IS NOT DISTINCT FROM $3"
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(input.lesson_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let resource: CourseResource = sqlx::query_as(
        r"INSERT INTO course_resources
           (course_id, section_id, lesson_id, title, description, file_url, file_name,
            file_type, file_size_bytes, sort_order, version)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
           RETURNING *",
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(input.lesson_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.file_url)
    .bind(&input.file_name)
    .bind(&input.file_type)
    .bind(input.file_size_bytes)
    .bind(sort_order)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create resource: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Created resource: {} for course {}",
        resource.title, course_id
    );

    Ok(Json(json!({
        "success": true,
        "resource": {
            "id": resource.id,
            "title": resource.title,
            "file_name": resource.file_name,
            "formatted_size": format_file_size(resource.file_size_bytes.unwrap_or(0))
        }
    })))
}

/// Delete a resource
pub(super) async fn delete_resource(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, resource_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "resource_delete", admin_id = %admin.id, course_id = %course_id, resource_id = %resource_id, "Admin deleting resource");
    let result = sqlx::query("DELETE FROM course_resources WHERE id = $1 AND course_id = $2")
        .bind(resource_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete resource: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resource not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}
