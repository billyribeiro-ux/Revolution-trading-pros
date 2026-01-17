//! Organization Routes - Teams & Departments
//! Apple ICT 7 Principal Engineer Grade - January 2026
//!
//! Admin routes for managing organizational structure:
//! - Teams (sales, support, development, etc.)
//! - Departments (trading, education, marketing, etc.)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// AUTHORIZATION
// ═══════════════════════════════════════════════════════════════════════════

fn require_admin(user: &User) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" || role == "developer" {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Access denied",
                "message": "This action requires admin privileges"
            })),
        ))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - TEAMS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_active: bool,
    pub member_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - DEPARTMENTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: bool,
    pub member_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TEAMS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/organization/teams - List all teams
async fn list_teams(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let teams: Vec<Team> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, color, icon, is_active, 
               0 as member_count, created_at, updated_at
        FROM teams
        ORDER BY name ASC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": teams
    })))
}

/// GET /admin/organization/teams/:id - Get single team
async fn get_team(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let team: Option<Team> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, color, icon, is_active,
               0 as member_count, created_at, updated_at
        FROM teams
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match team {
        Some(t) => Ok(Json(json!({
            "success": true,
            "data": t
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Team not found"})),
        )),
    }
}

/// POST /admin/organization/teams - Create team
async fn create_team(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateTeamRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let slug = input.name.to_lowercase().replace(' ', "-");

    let team: Team = sqlx::query_as(
        r#"
        INSERT INTO teams (name, slug, description, color, icon, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, true, NOW(), NOW())
        RETURNING id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
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
        "data": team
    })))
}

/// PUT /admin/organization/teams/:id - Update team
async fn update_team(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTeamRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let team: Team = sqlx::query_as(
        r#"
        UPDATE teams
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            color = COALESCE($4, color),
            icon = COALESCE($5, icon),
            is_active = COALESCE($6, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.is_active)
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
        "data": team
    })))
}

/// DELETE /admin/organization/teams/:id - Delete team
async fn delete_team(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query("DELETE FROM teams WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Team deleted"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// DEPARTMENTS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/organization/departments - List all departments
async fn list_departments(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let departments: Vec<Department> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, color, icon, parent_id, is_active,
               0 as member_count, created_at, updated_at
        FROM departments
        ORDER BY name ASC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": departments
    })))
}

/// GET /admin/organization/departments/:id - Get single department
async fn get_department(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let department: Option<Department> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, color, icon, parent_id, is_active,
               0 as member_count, created_at, updated_at
        FROM departments
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
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
async fn create_department(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateDepartmentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let slug = input.name.to_lowercase().replace(' ', "-");

    let department: Department = sqlx::query_as(
        r#"
        INSERT INTO departments (name, slug, description, color, icon, parent_id, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, true, NOW(), NOW())
        RETURNING id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.parent_id)
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
        "data": department
    })))
}

/// PUT /admin/organization/departments/:id - Update department
async fn update_department(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateDepartmentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let department: Department = sqlx::query_as(
        r#"
        UPDATE departments
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            color = COALESCE($4, color),
            icon = COALESCE($5, icon),
            parent_id = COALESCE($6, parent_id),
            is_active = COALESCE($7, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, color, icon, parent_id, is_active, 0 as member_count, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.parent_id)
    .bind(input.is_active)
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
        "data": department
    })))
}

/// DELETE /admin/organization/departments/:id - Delete department
async fn delete_department(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query("DELETE FROM departments WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Department deleted"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_teams).post(create_team))
        .route("/:id", get(get_team).put(update_team).delete(delete_team))
}

pub fn departments_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_departments).post(create_department))
        .route(
            "/:id",
            get(get_department)
                .put(update_department)
                .delete(delete_department),
        )
}
