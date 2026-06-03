//! Organization teams handlers — CRUD under /admin/organization/teams.
//!
//! R27-B3 (2026-05-20): extracted verbatim from the original
//! `organization.rs`. SQL (member_count subselect, slug uniqueness
//! check, refusal-to-delete-with-members invariant), audit log call
//! sites, tracing fields, and JSON error envelopes are preserved
//! byte-for-byte.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dtos::{CreateTeamRequest, ListQuery, Team, UpdateTeamRequest};
use super::helpers::{generate_slug, log_organization_audit};
use crate::{middleware::admin::AdminUser, AppState};

/// GET /admin/organization/teams - List all teams
#[tracing::instrument(skip(state, admin))]
pub(super) async fn list_teams(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin listing teams"
    );

    let mut sql = String::from(
        r"
        SELECT t.id, t.name, t.slug, t.description, t.color, t.icon, t.is_active,
               COALESCE((SELECT COUNT(*) FROM team_members tm WHERE tm.team_id = t.id)::int, 0) as member_count,
               t.created_at, t.updated_at
        FROM teams t
        WHERE 1=1
        ",
    );

    let mut bind_count = 0;

    if query.active_only.unwrap_or(false) {
        sql.push_str(" AND t.is_active = true");
    }

    if query.search.is_some() {
        bind_count += 1;
        sql.push_str(&format!(
            " AND (t.name ILIKE '%' || ${bind_count} || '%' OR t.description ILIKE '%' || ${bind_count} || '%')"
        ));
    }

    sql.push_str(" ORDER BY t.name ASC");

    let mut query_builder = sqlx::query_as::<_, Team>(sqlx::AssertSqlSafe(sql.as_str()));

    if let Some(ref search) = query.search {
        query_builder = query_builder.bind(search);
    }

    let teams: Vec<Team> = query_builder
        .fetch_all(state.db.pool())
        .await
        .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": teams,
        "total": teams.len()
    })))
}

/// GET /admin/organization/teams/:id - Get single team
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_team(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(admin_id = admin.0.id, team_id = id, "Admin fetching team");

    let team: Option<Team> = sqlx::query_as(
        r"
        SELECT t.id, t.name, t.slug, t.description, t.color, t.icon, t.is_active,
               COALESCE((SELECT COUNT(*) FROM team_members tm WHERE tm.team_id = t.id)::int, 0) as member_count,
               t.created_at, t.updated_at
        FROM teams t
        WHERE t.id = $1
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
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn create_team(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateTeamRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate name
    if input.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Team name is required"})),
        ));
    }

    if input.name.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Team name must be 100 characters or less"})),
        ));
    }

    let slug = generate_slug(&input.name);

    // Check for duplicate slug
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM teams WHERE slug = $1)")
        .bind(&slug)
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(false);

    if exists {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "A team with this name already exists"})),
        ));
    }

    let team: Team = sqlx::query_as(
        r"
        INSERT INTO teams (name, slug, description, color, icon, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, true, NOW(), NOW())
        RETURNING id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create team: {}", e);
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
        "team.created",
        "team",
        Some(team.id),
        None,
        Some(json!({"name": team.name, "slug": team.slug})),
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        team_id = team.id,
        team_name = %team.name,
        "Team created"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Team created successfully",
        "data": team
    })))
}

/// PUT /admin/organization/teams/:id - Update team
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn update_team(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTeamRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate name if provided
    if let Some(ref name) = input.name {
        if name.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Team name cannot be empty"})),
            ));
        }
        if name.len() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Team name must be 100 characters or less"})),
            ));
        }
    }

    // Get old team for audit
    let old_team: Option<Team> = sqlx::query_as(
        "SELECT id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at FROM teams WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let Some(old_team) = old_team else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Team not found"})),
        ));
    };
    let old_value = json!({"name": old_team.name, "is_active": old_team.is_active});

    // Generate new slug if name changes
    let new_slug = input.name.as_ref().map(|n| generate_slug(n));

    let team: Team = sqlx::query_as(
        r"
        UPDATE teams
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            color = COALESCE($5, color),
            icon = COALESCE($6, icon),
            is_active = COALESCE($7, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.name)
    .bind(&new_slug)
    .bind(&input.description)
    .bind(&input.color)
    .bind(&input.icon)
    .bind(input.is_active)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to update team: {}", e);
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
        "team.updated",
        "team",
        Some(team.id),
        Some(old_value),
        Some(json!({"name": team.name, "is_active": team.is_active})),
        None,
    )
    .await;

    tracing::info!(admin_id = admin.0.id, team_id = team.id, "Team updated");

    Ok(Json(json!({
        "success": true,
        "message": "Team updated successfully",
        "data": team
    })))
}

/// DELETE /admin/organization/teams/:id - Delete team
#[tracing::instrument(skip(state, admin))]
pub(super) async fn delete_team(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get team for audit before deletion
    let old_team: Option<Team> = sqlx::query_as(
        "SELECT id, name, slug, description, color, icon, is_active, 0 as member_count, created_at, updated_at FROM teams WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let Some(old_team) = old_team else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Team not found"})),
        ));
    };

    // Check if team has members
    let member_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM team_members WHERE team_id = $1")
            .bind(id)
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    if member_count > 0 {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "error": "Cannot delete team with members",
                "member_count": member_count,
                "suggestion": "Remove all members from the team first or deactivate the team instead"
            })),
        ));
    }

    sqlx::query("DELETE FROM teams WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete team: {}", e);
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
        "team.deleted",
        "team",
        Some(id),
        Some(json!({"name": old_team.name, "slug": old_team.slug})),
        None,
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        team_id = id,
        team_name = %old_team.name,
        "Team deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Team deleted successfully"
    })))
}
