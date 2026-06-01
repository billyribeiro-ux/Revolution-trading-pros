//! Admin Members - Tags
//!
//! Member tagging system: CRUD + per-user assign/unassign + bulk-assign.

use crate::middleware::admin::{AdminUser, SuperAdminUser};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MemberTag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub member_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct MemberTagQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemberTagRequest {
    pub name: String,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberTagRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignTagRequest {
    pub user_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BulkAssignTagsRequest {
    pub user_ids: Vec<i64>,
    pub tag_ids: Vec<i64>,
}

/// GET /admin/members/tags - List all member tags
pub(super) async fn list_member_tags(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<MemberTagQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(100).min(500);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{s}%"));

    // Gracefully handle missing table - return empty array instead of 500 error
    let tags: Vec<MemberTag> = sqlx::query_as(
        r"
        SELECT id, name, slug, color, description, member_count, created_at, updated_at
        FROM member_tags
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        ORDER BY name ASC
        LIMIT $2 OFFSET $3
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|e| {
        tracing::warn!(
            "Database query failed in list_member_tags (table may not exist): {}",
            e
        );
        Vec::new()
    });

    let total: i64 = sqlx::query_as::<_, (i64,)>(
        r"
        SELECT COUNT(*)
        FROM member_tags
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        ",
    )
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .map(|r| r.0)
    .unwrap_or(0);

    Ok(Json(json!({
        "tags": tags,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/members/tags/:id - Get single tag
pub(super) async fn get_member_tag(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    let tag: MemberTag = sqlx::query_as(
        "SELECT id, name, slug, color, description, member_count, created_at, updated_at FROM member_tags WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Tag not found"}))))?;

    Ok(Json(tag))
}

/// POST /admin/members/tags - Create new tag
pub(super) async fn create_member_tag(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateMemberTagRequest>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    // Generate slug if not provided
    let slug = input.slug.unwrap_or_else(|| {
        input
            .name
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect()
    });

    let tag: MemberTag = sqlx::query_as(
        r"
        INSERT INTO member_tags (name, slug, color, description, member_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, name, slug, color, description, member_count, created_at, updated_at
        "
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.color)
    .bind(&input.description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (StatusCode::CONFLICT, Json(json!({"error": "Tag with this slug already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(tag))
}

/// PUT /admin/members/tags/:id - Update tag
pub(super) async fn update_member_tag(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberTagRequest>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${param_count}"));
    }
    if input.slug.is_some() {
        param_count += 1;
        set_clauses.push(format!("slug = ${param_count}"));
    }
    if input.color.is_some() {
        param_count += 1;
        set_clauses.push(format!("color = ${param_count}"));
    }
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${param_count}"));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        "UPDATE member_tags SET {} WHERE id = $1 RETURNING id, name, slug, color, description, member_count, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, MemberTag>(&sql).bind(id);

    if let Some(ref name) = input.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref slug) = input.slug {
        query_builder = query_builder.bind(slug);
    }
    if let Some(ref color) = input.color {
        query_builder = query_builder.bind(color);
    }
    if let Some(ref description) = input.description {
        query_builder = query_builder.bind(description);
    }

    let tag = query_builder.fetch_one(&state.db.pool).await.map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "Tag with this slug already exists"})),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    })?;

    Ok(Json(tag))
}

/// DELETE /admin/members/tags/:id - Delete tag
///
/// FIX-2026-04-26 (audit 02 §P1-10): Destructive — super-admin only.
pub(super) async fn delete_member_tag(
    State(state): State<AppState>,
    SuperAdminUser(user): SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM member_tags WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Tag not found"})),
        ));
    }

    Ok(Json(json!({"message": "Tag deleted successfully"})))
}

/// POST /admin/members/tags/assign - Assign tag to user
pub(super) async fn assign_tag_to_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r"
        INSERT INTO user_member_tags (user_id, tag_id, created_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (user_id, tag_id) DO NOTHING
        ",
    )
    .bind(input.user_id)
    .bind(input.tag_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Update member count
    sqlx::query(
        "UPDATE member_tags SET member_count = (SELECT COUNT(*) FROM user_member_tags WHERE tag_id = $1) WHERE id = $1"
    )
    .bind(input.tag_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({"message": "Tag assigned successfully"})))
}

/// DELETE /admin/members/tags/unassign - Remove tag from user
pub(super) async fn unassign_tag_from_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM user_member_tags WHERE user_id = $1 AND tag_id = $2")
        .bind(input.user_id)
        .bind(input.tag_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Update member count
    sqlx::query(
        "UPDATE member_tags SET member_count = (SELECT COUNT(*) FROM user_member_tags WHERE tag_id = $1) WHERE id = $1"
    )
    .bind(input.tag_id)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({"message": "Tag removed successfully"})))
}

/// POST /admin/members/tags/bulk-assign - Bulk assign tags to users
///
/// FIX-2026-04-26 (audit 02 §P1-10): Bulk privileged write — super-admin only.
pub(super) async fn bulk_assign_tags(
    State(state): State<AppState>,
    SuperAdminUser(user): SuperAdminUser,
    Json(input): Json<BulkAssignTagsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut assigned_count = 0;

    for user_id in &input.user_ids {
        for tag_id in &input.tag_ids {
            let result = sqlx::query(
                r"
                INSERT INTO user_member_tags (user_id, tag_id, created_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (user_id, tag_id) DO NOTHING
                ",
            )
            .bind(user_id)
            .bind(tag_id)
            .execute(&state.db.pool)
            .await;

            if result.is_ok() {
                assigned_count += 1;
            }
        }
    }

    // Update member counts for all affected tags
    for tag_id in &input.tag_ids {
        sqlx::query(
            "UPDATE member_tags SET member_count = (SELECT COUNT(*) FROM user_member_tags WHERE tag_id = $1) WHERE id = $1"
        )
        .bind(tag_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({
        "message": "Tags assigned successfully",
        "assigned_count": assigned_count
    })))
}
