//! Admin Members - Segments
//!
//! Member segmentation for targeted marketing.

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
pub(super) async fn list_segments(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<SegmentQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{s}%"));

    // Gracefully handle missing table - return empty array instead of 500 error
    let segments: Vec<MemberSegment> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, rules, member_count, is_active, created_at, updated_at
        FROM member_segments
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_active = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(query.is_active)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_else(|e| {
        tracing::warn!(
            "Database query failed in list_segments (table may not exist): {}",
            e
        );
        Vec::new()
    });

    let total: i64 = sqlx::query_as::<_, (i64,)>(
        r"
        SELECT COUNT(*)
        FROM member_segments
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_active = $2)
        ",
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
pub(super) async fn get_segment(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
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
pub(super) async fn create_segment(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateSegmentRequest>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
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

    let segment: MemberSegment = sqlx::query_as(
        r"
        INSERT INTO member_segments (name, slug, description, rules, member_count, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, $5, NOW(), NOW())
        RETURNING id, name, slug, description, rules, member_count, is_active, created_at, updated_at
        "
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
pub(super) async fn update_segment(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSegmentRequest>,
) -> Result<Json<MemberSegment>, (StatusCode, Json<serde_json::Value>)> {
    // Build UPDATE query dynamically
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
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${param_count}"));
    }
    if input.rules.is_some() {
        param_count += 1;
        set_clauses.push(format!("rules = ${param_count}"));
    }
    if input.is_active.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_active = ${param_count}"));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        "UPDATE member_segments SET {} WHERE id = $1 RETURNING id, name, slug, description, rules, member_count, is_active, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder =
        sqlx::query_as::<_, MemberSegment>(sqlx::AssertSqlSafe(sql.as_str())).bind(id);

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

    let segment = query_builder.fetch_one(&state.db.pool).await.map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "Segment with this slug already exists"})),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    })?;

    Ok(Json(segment))
}

/// DELETE /admin/members/segments/:id - Delete segment
///
/// FIX-2026-04-26 (audit 02 §P1-10): Destructive — super-admin only.
pub(super) async fn delete_segment(
    State(state): State<AppState>,
    SuperAdminUser(user): SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM member_segments WHERE id = $1")
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
            Json(json!({"error": "Segment not found"})),
        ));
    }

    Ok(Json(json!({"message": "Segment deleted successfully"})))
}
