//! Admin Members Controller - ICT 11+ Principal Engineer
//! Apple ICT 7 Grade - January 2026
//!
//! Member segments, tags, and saved filters management for CRM.
//! All routes require admin privileges.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;

use crate::{
    models::User,
    AppState,
};

// ===============================================================================
// AUTHORIZATION
// ===============================================================================

/// Check if user has admin privileges (admin, super-admin, or developer role)
fn require_admin(user: &User) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" || role == "developer" {
        Ok(())
    } else {
        Err((StatusCode::FORBIDDEN, Json(json!({
            "error": "Access denied",
            "message": "This action requires admin privileges"
        }))))
    }
}

// ===============================================================================
// SEGMENTS - Member segmentation for targeted marketing
// ===============================================================================

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MemberSegment {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub member_count: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct SegmentQuery {
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSegmentRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

/// GET /admin/members/segments - List all member segments
async fn list_segments(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<SegmentQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{}%", s));

    // Gracefully handle missing table - return empty array instead of 500 error
    let segments: Vec<MemberSegment> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, rules, member_count, is_active, created_at, updated_at
        FROM member_segments
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_active = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(query.is_active)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|e| {
        tracing::warn!("Database query failed in list_segments (table may not exist): {}", e);
        Vec::new()
    });

    let total: i64 = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM member_segments
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_active = $2)
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(query.is_active)
    .fetch_one(&state.db.pool)
    .await
    .map(|r| r.0)
    .unwrap_or(0);

    Ok(Json(json!({
        "segments": segments,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/members/segments/:id - Get single segment
async fn get_segment(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let segment: MemberSegment = sqlx::query_as(
        "SELECT id, name, slug, description, rules, member_count, is_active, created_at, updated_at FROM member_segments WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))))?;

    Ok(Json(segment))
}

/// POST /admin/members/segments - Create new segment
async fn create_segment(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateSegmentRequest>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Generate slug if not provided
    let slug = input.slug.unwrap_or_else(|| {
        input.name.to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect()
    });

    let segment: MemberSegment = sqlx::query_as(
        r#"
        INSERT INTO member_segments (name, slug, description, rules, member_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, $5, NOW(), NOW())
        RETURNING id, name, slug, description, rules, member_count, is_active, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.rules)
    .bind(input.is_active.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (StatusCode::CONFLICT, Json(json!({"error": "Segment with this slug already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(segment))
}

/// PUT /admin/members/segments/:id - Update segment
async fn update_segment(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSegmentRequest>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Build UPDATE query dynamically
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${}", param_count));
    }
    if input.slug.is_some() {
        param_count += 1;
        set_clauses.push(format!("slug = ${}", param_count));
    }
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${}", param_count));
    }
    if input.rules.is_some() {
        param_count += 1;
        set_clauses.push(format!("rules = ${}", param_count));
    }
    if input.is_active.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_active = ${}", param_count));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "No fields to update"}))));
    }

    let sql = format!(
        "UPDATE member_segments SET {} WHERE id = $1 RETURNING id, name, slug, description, rules, member_count, is_active, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, MemberSegment>(&sql).bind(id);

    if let Some(ref name) = input.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref slug) = input.slug {
        query_builder = query_builder.bind(slug);
    }
    if let Some(ref description) = input.description {
        query_builder = query_builder.bind(description);
    }
    if let Some(ref rules) = input.rules {
        query_builder = query_builder.bind(rules);
    }
    if let Some(is_active) = input.is_active {
        query_builder = query_builder.bind(is_active);
    }

    let segment = query_builder
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                (StatusCode::CONFLICT, Json(json!({"error": "Segment with this slug already exists"})))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
            }
        })?;

    Ok(Json(segment))
}

/// DELETE /admin/members/segments/:id - Delete segment
async fn delete_segment(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let result = sqlx::query("DELETE FROM member_segments WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Segment not found"}))));
    }

    Ok(Json(json!({"message": "Segment deleted successfully"})))
}

// ===============================================================================
// MEMBER TAGS - Tagging system for members
// ===============================================================================

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

/// GET /admin/members/tags - List all member tags
async fn list_member_tags(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<MemberTagQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(100).min(500);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{}%", s));

    // Gracefully handle missing table - return empty array instead of 500 error
    let tags: Vec<MemberTag> = sqlx::query_as(
        r#"
        SELECT id, name, slug, color, description, member_count, created_at, updated_at
        FROM member_tags
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        ORDER BY name ASC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|e| {
        tracing::warn!("Database query failed in list_member_tags (table may not exist): {}", e);
        Vec::new()
    });

    let total: i64 = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM member_tags
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        "#
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
async fn get_member_tag(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

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
async fn create_member_tag(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateMemberTagRequest>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Generate slug if not provided
    let slug = input.slug.unwrap_or_else(|| {
        input.name.to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect()
    });

    let tag: MemberTag = sqlx::query_as(
        r#"
        INSERT INTO member_tags (name, slug, color, description, member_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, name, slug, color, description, member_count, created_at, updated_at
        "#
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
async fn update_member_tag(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberTagRequest>,
) -> Result<Json<MemberTag>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${}", param_count));
    }
    if input.slug.is_some() {
        param_count += 1;
        set_clauses.push(format!("slug = ${}", param_count));
    }
    if input.color.is_some() {
        param_count += 1;
        set_clauses.push(format!("color = ${}", param_count));
    }
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${}", param_count));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "No fields to update"}))));
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

    let tag = query_builder
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

/// DELETE /admin/members/tags/:id - Delete tag
async fn delete_member_tag(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let result = sqlx::query("DELETE FROM member_tags WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Tag not found"}))));
    }

    Ok(Json(json!({"message": "Tag deleted successfully"})))
}

// ===============================================================================
// SAVED FILTERS - Saved member filter presets
// ===============================================================================

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MemberFilter {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: bool,
    pub is_public: bool,
    pub created_by: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct MemberFilterQuery {
    pub search: Option<String>,
    pub is_public: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemberFilterRequest {
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: Option<bool>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberFilterRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub is_default: Option<bool>,
    pub is_public: Option<bool>,
}

/// GET /admin/members/filters - List all saved filters
async fn list_member_filters(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<MemberFilterQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{}%", s));

    // Gracefully handle missing table - return empty array instead of 500 error
    let filters: Vec<MemberFilter> = sqlx::query_as(
        r#"
        SELECT id, name, description, filters, is_default, is_public, created_by, created_at, updated_at
        FROM member_filters
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        ORDER BY is_default DESC, name ASC
        LIMIT $3 OFFSET $4
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(query.is_public)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|e| {
        tracing::warn!("Database query failed in list_member_filters (table may not exist): {}", e);
        Vec::new()
    });

    let total: i64 = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT COUNT(*)
        FROM member_filters
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(query.is_public)
    .fetch_one(&state.db.pool)
    .await
    .map(|r| r.0)
    .unwrap_or(0);

    Ok(Json(json!({
        "filters": filters,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// GET /admin/members/filters/:id - Get single filter
async fn get_member_filter(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let filter: MemberFilter = sqlx::query_as(
        "SELECT id, name, description, filters, is_default, is_public, created_by, created_at, updated_at FROM member_filters WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Filter not found"}))))?;

    Ok(Json(filter))
}

/// POST /admin/members/filters - Create new filter
async fn create_member_filter(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateMemberFilterRequest>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let filter: MemberFilter = sqlx::query_as(
        r#"
        INSERT INTO member_filters (name, description, filters, is_default, is_public, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING id, name, description, filters, is_default, is_public, created_by, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.filters)
    .bind(input.is_default.unwrap_or(false))
    .bind(input.is_public.unwrap_or(true))
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (StatusCode::CONFLICT, Json(json!({"error": "Filter with this name already exists"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    Ok(Json(filter))
}

/// PUT /admin/members/filters/:id - Update filter
async fn update_member_filter(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberFilterRequest>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${}", param_count));
    }
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${}", param_count));
    }
    if input.filters.is_some() {
        param_count += 1;
        set_clauses.push(format!("filters = ${}", param_count));
    }
    if input.is_default.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_default = ${}", param_count));
    }
    if input.is_public.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_public = ${}", param_count));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "No fields to update"}))));
    }

    let sql = format!(
        "UPDATE member_filters SET {} WHERE id = $1 RETURNING id, name, description, filters, is_default, is_public, created_by, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, MemberFilter>(&sql).bind(id);

    if let Some(ref name) = input.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref description) = input.description {
        query_builder = query_builder.bind(description);
    }
    if let Some(ref filters) = input.filters {
        query_builder = query_builder.bind(filters);
    }
    if let Some(is_default) = input.is_default {
        query_builder = query_builder.bind(is_default);
    }
    if let Some(is_public) = input.is_public {
        query_builder = query_builder.bind(is_public);
    }

    let filter = query_builder
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                (StatusCode::CONFLICT, Json(json!({"error": "Filter with this name already exists"})))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
            }
        })?;

    Ok(Json(filter))
}

/// DELETE /admin/members/filters/:id - Delete filter
async fn delete_member_filter(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let result = sqlx::query("DELETE FROM member_filters WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Filter not found"}))));
    }

    Ok(Json(json!({"message": "Filter deleted successfully"})))
}

// ===============================================================================
// USER-TAG ASSOCIATIONS
// ===============================================================================

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

/// POST /admin/members/tags/assign - Assign tag to user
async fn assign_tag_to_user(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query(
        r#"
        INSERT INTO user_member_tags (user_id, tag_id, created_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (user_id, tag_id) DO NOTHING
        "#
    )
    .bind(input.user_id)
    .bind(input.tag_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

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
async fn unassign_tag_from_user(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query("DELETE FROM user_member_tags WHERE user_id = $1 AND tag_id = $2")
        .bind(input.user_id)
        .bind(input.tag_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

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
async fn bulk_assign_tags(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<BulkAssignTagsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let mut assigned_count = 0;

    for user_id in &input.user_ids {
        for tag_id in &input.tag_ids {
            let result = sqlx::query(
                r#"
                INSERT INTO user_member_tags (user_id, tag_id, created_at)
                VALUES ($1, $2, NOW())
                ON CONFLICT (user_id, tag_id) DO NOTHING
                "#
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

// ===============================================================================
// ROUTER
// ===============================================================================

/// Build the admin members router for segments, tags, and filters
pub fn router() -> Router<AppState> {
    Router::new()
        // Segments
        .route("/segments", get(list_segments).post(create_segment))
        .route("/segments/:id", get(get_segment).put(update_segment).delete(delete_segment))
        // Tags
        .route("/tags", get(list_member_tags).post(create_member_tag))
        .route("/tags/:id", get(get_member_tag).put(update_member_tag).delete(delete_member_tag))
        .route("/tags/assign", post(assign_tag_to_user))
        .route("/tags/unassign", delete(unassign_tag_from_user))
        .route("/tags/bulk-assign", post(bulk_assign_tags))
        // Filters
        .route("/filters", get(list_member_filters).post(create_member_filter))
        .route("/filters/:id", get(get_member_filter).put(update_member_filter).delete(delete_member_filter))
}
