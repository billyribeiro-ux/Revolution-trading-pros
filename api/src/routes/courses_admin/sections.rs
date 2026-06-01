//! Course section endpoints
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// SECTION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a section in a course
pub(super) async fn create_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateSectionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_create", admin_id = %admin.id, course_id = %course_id, "Admin creating section");
    // Get max sort order
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_sections WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let unlock_date: Option<DateTime<Utc>> = input.unlock_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    let section: CourseSection = sqlx::query_as(
        r"INSERT INTO course_sections
           (course_id, title, description, sort_order, section_type, unlock_type,
            unlock_after_section_id, unlock_date, is_published)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING *",
    )
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(sort_order)
    .bind(input.section_type.as_deref().unwrap_or("standard"))
    .bind(input.unlock_type.as_deref().unwrap_or("immediate"))
    .bind(input.unlock_after_section_id)
    .bind(unlock_date)
    .bind(input.is_published.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create section: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!("Created section: {} in course {}", section.title, course_id);

    Ok(Json(json!({
        "success": true,
        "section": {
            "id": section.id,
            "title": section.title,
            "sort_order": section.sort_order
        }
    })))
}

/// Update a section
/// ICT 7+ SECURITY: Uses parameterized queries to prevent SQL injection
pub(super) async fn update_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateSectionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_update", admin_id = %admin.id, course_id = %course_id, section_id = %section_id, "Admin updating section");

    // Parse unlock_date if provided
    let unlock_date: Option<DateTime<Utc>> = input.unlock_date.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    // ICT 7+ SECURITY FIX: Parameterized query
    sqlx::query(
        r"UPDATE course_sections SET
            title = COALESCE($3, title),
            description = COALESCE($4, description),
            section_type = COALESCE($5, section_type),
            unlock_type = COALESCE($6, unlock_type),
            unlock_after_section_id = COALESCE($7, unlock_after_section_id),
            unlock_date = COALESCE($8, unlock_date),
            is_published = COALESCE($9, is_published),
            updated_at = NOW()
        WHERE id = $1 AND course_id = $2",
    )
    .bind(section_id)
    .bind(course_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.section_type)
    .bind(&input.unlock_type)
    .bind(input.unlock_after_section_id)
    .bind(unlock_date)
    .bind(input.is_published)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update section: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({"success": true})))
}

/// Delete a section
pub(super) async fn delete_section(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, section_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "section_delete", admin_id = %admin.id, course_id = %course_id, section_id = %section_id, "Admin deleting section");
    // Delete associated lessons and resources first
    sqlx::query("DELETE FROM course_resources WHERE section_id = $1")
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();

    sqlx::query("DELETE FROM course_lessons WHERE section_id = $1")
        .bind(section_id)
        .execute(&state.db.pool)
        .await
        .ok();

    let result = sqlx::query("DELETE FROM course_sections WHERE id = $1 AND course_id = $2")
        .bind(section_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete section: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Section not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}

/// Reorder sections
pub(super) async fn reorder_sections(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<ReorderItemsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.items {
        sqlx::query(
            "UPDATE course_sections SET sort_order = $1, updated_at = NOW() WHERE id = $2 AND course_id = $3"
        )
        .bind(item.sort_order)
        .bind(item.id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true})))
}
