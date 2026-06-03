//! Room resources bulk operations (split from `room_resources.rs`
//! lines 1287-1544).
//!
//! Admin-only bulk create/update/delete for resources.

use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::helpers::slugify;
use super::CreateResourceRequest;

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: BULK OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct BulkCreateRequest {
    pub resources: Vec<CreateResourceRequest>,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequest {
    pub ids: Vec<i64>,
    pub updates: BulkUpdateFields,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateFields {
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub access_level: Option<String>,
    pub category: Option<String>,
    pub section: Option<String>,
}

/// POST /api/admin/room-resources/bulk-create - Create multiple resources at once
pub(super) async fn bulk_create_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkCreateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut created_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for resource_input in input.resources {
        let slug = slugify(&resource_input.title);
        let resource_date = resource_input
            .resource_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Utc::now().date_naive());

        let tags_json = resource_input
            .tags
            .as_ref()
            .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
            .unwrap_or(json!([]));

        let result = sqlx::query(
            r"
            INSERT INTO room_resources (
                title, slug, description, resource_type, content_type,
                file_url, mime_type, file_size, video_platform, bunny_video_guid,
                bunny_library_id, duration, thumbnail_url, width, height,
                trading_room_id, trader_id, resource_date, is_published, is_featured,
                is_pinned, section, category, tags, difficulty_level, access_level,
                course_id, lesson_id, course_order, storage_provider, file_hash
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31
            )
            ",
        )
        .bind(&resource_input.title)
        .bind(&slug)
        .bind(&resource_input.description)
        .bind(&resource_input.resource_type)
        .bind(&resource_input.content_type)
        .bind(&resource_input.file_url)
        .bind(&resource_input.mime_type)
        .bind(resource_input.file_size)
        .bind(&resource_input.video_platform)
        .bind(&resource_input.bunny_video_guid)
        .bind(resource_input.bunny_library_id)
        .bind(resource_input.duration)
        .bind(&resource_input.thumbnail_url)
        .bind(resource_input.width)
        .bind(resource_input.height)
        .bind(resource_input.trading_room_id)
        .bind(resource_input.trader_id)
        .bind(resource_date)
        .bind(resource_input.is_published.unwrap_or(false))
        .bind(resource_input.is_featured.unwrap_or(false))
        .bind(resource_input.is_pinned.unwrap_or(false))
        .bind(&resource_input.section)
        .bind(&resource_input.category)
        .bind(tags_json)
        .bind(&resource_input.difficulty_level)
        .bind(resource_input.access_level.as_deref().unwrap_or("premium"))
        .bind(resource_input.course_id)
        .bind(resource_input.lesson_id)
        .bind(resource_input.course_order)
        .bind(resource_input.storage_provider.as_deref().unwrap_or("r2"))
        .bind(&resource_input.file_hash)
        .execute(&state.db.pool)
        .await;

        match result {
            Ok(_) => created_count += 1,
            Err(e) => errors.push(format!(
                "Failed to create '{}': {}",
                resource_input.title, e
            )),
        }
    }

    Ok(Json(json!({
        "success": errors.is_empty(),
        "created_count": created_count,
        "errors": errors,
        "message": format!("Created {} resources", created_count)
    })))
}

/// PUT /api/admin/room-resources/bulk-update - Update multiple resources at once
pub(super) async fn bulk_update_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkUpdateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.updates.is_published.is_some() {
        updates.push(format!("is_published = ${param_idx}"));
        param_idx += 1;
    }
    if input.updates.is_featured.is_some() {
        updates.push(format!("is_featured = ${param_idx}"));
        param_idx += 1;
    }
    if input.updates.is_pinned.is_some() {
        updates.push(format!("is_pinned = ${param_idx}"));
        param_idx += 1;
    }
    if input.updates.access_level.is_some() {
        updates.push(format!("access_level = ${param_idx}"));
        param_idx += 1;
    }
    if input.updates.category.is_some() {
        updates.push(format!("category = ${param_idx}"));
        param_idx += 1;
    }
    if input.updates.section.is_some() {
        updates.push(format!("section = ${param_idx}"));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let ids_placeholder: String = input
        .ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", param_idx + i))
        .collect::<Vec<_>>()
        .join(",");

    // SAFETY: `updates` is composed of literal `column = $idx` fragments
    // built above from a fixed allow-list of column names
    // (is_published / is_featured / is_pinned / access_level / category /
    // section). `ids_placeholder` is a comma-separated list of `${idx}`
    // placeholders. No user input is formatted into the SQL text — every
    // value is bound below.
    let query_str = format!(
        "UPDATE room_resources SET {}, updated_at = NOW() WHERE id IN ({}) AND deleted_at IS NULL",
        updates.join(", "),
        ids_placeholder
    );

    let mut query = sqlx::query(sqlx::AssertSqlSafe(query_str.as_str()));

    if let Some(v) = input.updates.is_published {
        query = query.bind(v);
    }
    if let Some(v) = input.updates.is_featured {
        query = query.bind(v);
    }
    if let Some(v) = input.updates.is_pinned {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.access_level {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.category {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.section {
        query = query.bind(v);
    }

    for id in &input.ids {
        query = query.bind(*id);
    }

    let result = query.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "updated_count": result.rows_affected(),
        "message": format!("Updated {} resources", result.rows_affected())
    })))
}

/// DELETE /api/admin/room-resources/bulk-delete - Delete multiple resources at once
pub(super) async fn bulk_delete_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(ids): Json<Vec<i64>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No IDs provided"})),
        ));
    }

    let placeholders: String = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let query_str = format!(
        "UPDATE room_resources SET deleted_at = NOW() WHERE id IN ({placeholders}) AND deleted_at IS NULL"
    );

    let mut query = sqlx::query(sqlx::AssertSqlSafe(query_str.as_str()));
    for id in &ids {
        query = query.bind(*id);
    }

    let result = query.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "deleted_count": result.rows_affected(),
        "message": format!("Deleted {} resources", result.rows_affected())
    })))
}
