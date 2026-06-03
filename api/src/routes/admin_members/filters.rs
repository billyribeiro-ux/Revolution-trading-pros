//! Admin Members - Saved Filters
//!
//! Saved member filter presets (CRUD).

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
pub(super) async fn list_member_filters(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<MemberFilterQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{s}%"));

    // Gracefully handle missing table - return empty array instead of 500 error
    let filters: Vec<MemberFilter> = sqlx::query_as(
        r"
        SELECT id, name, description, filters, is_default, is_public, created_by, created_at, updated_at
        FROM member_filters
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        ORDER BY is_default DESC, name ASC
        LIMIT $3 OFFSET $4
        "
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
        r"
        SELECT COUNT(*)
        FROM member_filters
        WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_public = $2)
        ",
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
pub(super) async fn get_member_filter(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
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
pub(super) async fn create_member_filter(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateMemberFilterRequest>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
    let filter: MemberFilter = sqlx::query_as(
        r"
        INSERT INTO member_filters (name, description, filters, is_default, is_public, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        RETURNING id, name, description, filters, is_default, is_public, created_by, created_at, updated_at
        "
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
pub(super) async fn update_member_filter(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberFilterRequest>,
) -> Result<Json<MemberFilter>, (StatusCode, Json<serde_json::Value>)> {
    let mut set_clauses = Vec::new();
    let mut param_count = 1;

    if input.name.is_some() {
        param_count += 1;
        set_clauses.push(format!("name = ${param_count}"));
    }
    if input.description.is_some() {
        param_count += 1;
        set_clauses.push(format!("description = ${param_count}"));
    }
    if input.filters.is_some() {
        param_count += 1;
        set_clauses.push(format!("filters = ${param_count}"));
    }
    if input.is_default.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_default = ${param_count}"));
    }
    if input.is_public.is_some() {
        param_count += 1;
        set_clauses.push(format!("is_public = ${param_count}"));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    if set_clauses.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        "UPDATE member_filters SET {} WHERE id = $1 RETURNING id, name, description, filters, is_default, is_public, created_by, created_at, updated_at",
        set_clauses.join(", ")
    );

    let mut query_builder =
        sqlx::query_as::<_, MemberFilter>(sqlx::AssertSqlSafe(sql.as_str())).bind(id);

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

    let filter = query_builder.fetch_one(&state.db.pool).await.map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "Filter with this name already exists"})),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    })?;

    Ok(Json(filter))
}

/// DELETE /admin/members/filters/:id - Delete filter
///
/// FIX-2026-04-26 (audit 02 §P1-10): Destructive — super-admin only.
pub(super) async fn delete_member_filter(
    State(state): State<AppState>,
    SuperAdminUser(user): SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM member_filters WHERE id = $1")
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
            Json(json!({"error": "Filter not found"})),
        ));
    }

    Ok(Json(json!({"message": "Filter deleted successfully"})))
}
