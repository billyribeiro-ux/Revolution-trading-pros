//! Organization departments handlers — CRUD under /admin/organization/departments.
//!
//! R27-B3 (2026-05-20): extracted verbatim from the original
//! `organization.rs`. SQL (parent-id hierarchy validation, circular-
//! reference guard, refusal-to-delete-with-children / -with-members
//! invariants, NULLS FIRST ordering), audit log call sites, tracing
//! fields, and JSON error envelopes are preserved byte-for-byte.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dtos::{CreateDepartmentRequest, Department, ListQuery, UpdateDepartmentRequest};
use super::helpers::{generate_slug, log_organization_audit};
use crate::{middleware::admin::AdminUser, AppState};

/// GET /admin/organization/departments - List all departments
#[tracing::instrument(skip(state, admin))]
pub(super) async fn list_departments(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin listing departments"
    );

    let mut sql = String::from(
        r"
        SELECT d.id, d.name, d.slug, d.description, d.color, d.icon, d.parent_id, d.is_active,
               COALESCE((SELECT COUNT(*) FROM department_members dm WHERE dm.department_id = d.id)::int, 0) as member_count,
               d.created_at, d.updated_at
        FROM departments d
        WHERE 1=1
        ",
    );

    let mut bind_count = 0;

    if query.active_only.unwrap_or(false) {
        sql.push_str(" AND d.is_active = true");
    }

    if query.search.is_some() {
        bind_count += 1;
        sql.push_str(&format!(
            " AND (d.name ILIKE '%' || ${bind_count} || '%' OR d.description ILIKE '%' || ${bind_count} || '%')"
        ));
    }

    sql.push_str(" ORDER BY d.parent_id NULLS FIRST, d.name ASC");

    let mut query_builder = sqlx::query_as::<_, Department>(sqlx::AssertSqlSafe(sql.as_str()));

    if let Some(ref search) = query.search {
        query_builder = query_builder.bind(search);
    }

    let departments: Vec<Department> = query_builder
        .fetch_all(state.db.pool())
        .await
        .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": departments,
        "total": departments.len()
    })))
}

/// GET /admin/organization/departments/:id - Get single department
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_department(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        department_id = id,
        "Admin fetching department"
    );

    let department: Option<Department> = sqlx::query_as(
        r"
        SELECT d.id, d.name, d.slug, d.description, d.color, d.icon, d.parent_id, d.is_active,
               COALESCE((SELECT COUNT(*) FROM department_members dm WHERE dm.department_id = d.id)::int, 0) as member_count,
               d.created_at, d.updated_at
        FROM departments d
        WHERE d.id = $1
        ",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match department {
        Some(d) => Ok(Json(json!({
            "success": true,
            "data": d
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Department not found"})),
        )),
    }
}

/// POST /admin/organization/departments - Create department
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn create_department(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateDepartmentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate name
    if input.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Department name is required"})),
        ));
    }

    if input.name.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Department name must be 100 characters or less"})),
        ));
    }

    // Validate parent_id if provided
    if let Some(parent_id) = input.parent_id {
        let parent_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM departments WHERE id = $1)")
                .bind(parent_id)
                .fetch_one(state.db.pool())
                .await
                .unwrap_or(false);

        if !parent_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Parent department not found"})),
            ));
        }
    }

    let slug = generate_slug(&input.name);

    let department: Department = sqlx::query_as(
        r"
        INSERT INTO departments (name, slug, description, color, icon, parent_id, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, true, NOW(), NOW())
        RETURNING id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.parent_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create department: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Audit log
    log_organization_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "department.created",
        "department",
        Some(department.id),
        None,
        Some(json!({"name": department.name, "slug": department.slug, "parent_id": department.parent_id})),
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        department_id = department.id,
        department_name = %department.name,
        "Department created"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Department created successfully",
        "data": department
    })))
}

/// PUT /admin/organization/departments/:id - Update department
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn update_department(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDepartmentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate name if provided
    if let Some(ref name) = input.name {
        if name.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Department name cannot be empty"})),
            ));
        }
        if name.len() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Department name must be 100 characters or less"})),
            ));
        }
    }

    // Prevent circular parent reference
    if let Some(parent_id) = input.parent_id {
        if parent_id == id {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Department cannot be its own parent"})),
            ));
        }

        // Check if parent exists
        let parent_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM departments WHERE id = $1)")
                .bind(parent_id)
                .fetch_one(state.db.pool())
                .await
                .unwrap_or(false);

        if !parent_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Parent department not found"})),
            ));
        }
    }

    // Get old department for audit
    let old_dept: Option<Department> = sqlx::query_as(
        "SELECT id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at FROM departments WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let Some(old_dept) = old_dept else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Department not found"})),
        ));
    };
    let old_value = json!({"name": old_dept.name, "is_active": old_dept.is_active, "parent_id": old_dept.parent_id});

    let new_slug = input.name.as_ref().map(|n| generate_slug(n));

    let department: Department = sqlx::query_as(
        r"
        UPDATE departments
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            color = COALESCE($5, color),
            icon = COALESCE($6, icon),
            parent_id = COALESCE($7, parent_id),
            is_active = COALESCE($8, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&new_slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.parent_id)
    .bind(input.is_active)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to update department: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Audit log
    log_organization_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "department.updated",
        "department",
        Some(department.id),
        Some(old_value),
        Some(json!({"name": department.name, "is_active": department.is_active, "parent_id": department.parent_id})),
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        department_id = department.id,
        "Department updated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Department updated successfully",
        "data": department
    })))
}

/// DELETE /admin/organization/departments/:id - Delete department
#[tracing::instrument(skip(state, admin))]
pub(super) async fn delete_department(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get department for audit
    let old_dept: Option<Department> = sqlx::query_as(
        "SELECT id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at FROM departments WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let Some(old_dept) = old_dept else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Department not found"})),
        ));
    };

    // Check for child departments
    let child_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM departments WHERE parent_id = $1")
            .bind(id)
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    if child_count > 0 {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Cannot delete department with child departments",
                "child_count": child_count,
                "suggestion": "Delete or reassign child departments first"
            })),
        ));
    }

    // Check for members
    let member_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM department_members WHERE department_id = $1")
            .bind(id)
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    if member_count > 0 {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Cannot delete department with members",
                "member_count": member_count,
                "suggestion": "Remove all members from the department first"
            })),
        ));
    }

    sqlx::query("DELETE FROM departments WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete department: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Audit log
    log_organization_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "department.deleted",
        "department",
        Some(id),
        Some(json!({"name": old_dept.name, "slug": old_dept.slug})),
        None,
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        department_id = id,
        department_name = %old_dept.name,
        "Department deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Department deleted successfully"
    })))
}
