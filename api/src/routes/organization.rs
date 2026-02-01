//! Organization Routes - ICT Level 7 Principal Engineer Grade
//! Apple ICT 7 Principal Engineer Grade - January 2026
//!
//! SECURITY: All endpoints require AdminUser authentication
//! Features:
//! - Organization profile management (company info, branding)
//! - Teams CRUD with member counts
//! - Departments CRUD with hierarchy
//! - Audit logging for all changes

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - ORGANIZATION PROFILE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationProfile {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub website_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub business_hours: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
    pub tax_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub website_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub business_hours: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
    pub tax_id: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub search: Option<String>,
    pub active_only: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER - AUDIT LOGGING
// ═══════════════════════════════════════════════════════════════════════════

async fn log_organization_audit(
    pool: &sqlx::PgPool,
    admin_id: i64,
    admin_email: &str,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    old_value: Option<serde_json::Value>,
    new_value: Option<serde_json::Value>,
    metadata: Option<serde_json::Value>,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO admin_audit_logs (
            admin_id, admin_email, action, entity_type, entity_id,
            old_value, new_value, metadata, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6::text, $7::text, $8, NOW())
        "#,
    )
    .bind(admin_id)
    .bind(admin_email)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(old_value.map(|v| v.to_string()))
    .bind(new_value.map(|v| v.to_string()))
    .bind(metadata)
    .execute(pool)
    .await;
}

/// Generate URL-safe slug from name
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════
// ORGANIZATION PROFILE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/organization/profile - Get organization profile
#[tracing::instrument(skip(state, admin))]
async fn get_organization_profile(
    State(state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin fetching organization profile"
    );

    let profile: Option<OrganizationProfile> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, logo_url, favicon_url,
               primary_color, secondary_color, contact_email, contact_phone,
               address, city, state, country, postal_code, website_url,
               social_links, business_hours, timezone, currency, tax_id,
               created_at, updated_at
        FROM organization_profile
        LIMIT 1
        "#,
    )
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error", "details": e.to_string()})),
        )
    })?;

    match profile {
        Some(p) => Ok(Json(json!({
            "success": true,
            "data": p
        }))),
        None => {
            // Return default profile if none exists
            Ok(Json(json!({
                "success": true,
                "data": {
                    "id": 0,
                    "name": "Revolution Trading Pros",
                    "slug": "revolution-trading-pros",
                    "description": null,
                    "timezone": "America/New_York",
                    "currency": "USD"
                }
            })))
        }
    }
}

/// PUT /admin/organization/profile - Update organization profile
#[tracing::instrument(skip(state, admin, input))]
async fn update_organization_profile(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<UpdateOrganizationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin updating organization profile"
    );

    // Get old profile for audit
    let old_profile: Option<OrganizationProfile> = sqlx::query_as(
        "SELECT * FROM organization_profile LIMIT 1"
    )
    .fetch_optional(state.db.pool())
    .await
    .ok()
    .flatten();

    let old_value = old_profile.as_ref().map(|p| json!({
        "name": p.name,
        "contact_email": p.contact_email
    }));

    // Generate slug if name is provided
    let slug = input.name.as_ref().map(|n| generate_slug(n));

    let profile: OrganizationProfile = sqlx::query_as(
        r#"
        INSERT INTO organization_profile (
            id, name, slug, description, logo_url, favicon_url,
            primary_color, secondary_color, contact_email, contact_phone,
            address, city, state, country, postal_code, website_url,
            social_links, business_hours, timezone, currency, tax_id,
            created_at, updated_at
        ) VALUES (
            1,
            COALESCE($1, 'Revolution Trading Pros'),
            COALESCE($2, 'revolution-trading-pros'),
            $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            COALESCE((SELECT created_at FROM organization_profile WHERE id = 1), NOW()),
            NOW()
        )
        ON CONFLICT (id) DO UPDATE SET
            name = COALESCE($1, organization_profile.name),
            slug = COALESCE($2, organization_profile.slug),
            description = COALESCE($3, organization_profile.description),
            logo_url = COALESCE($4, organization_profile.logo_url),
            favicon_url = COALESCE($5, organization_profile.favicon_url),
            primary_color = COALESCE($6, organization_profile.primary_color),
            secondary_color = COALESCE($7, organization_profile.secondary_color),
            contact_email = COALESCE($8, organization_profile.contact_email),
            contact_phone = COALESCE($9, organization_profile.contact_phone),
            address = COALESCE($10, organization_profile.address),
            city = COALESCE($11, organization_profile.city),
            state = COALESCE($12, organization_profile.state),
            country = COALESCE($13, organization_profile.country),
            postal_code = COALESCE($14, organization_profile.postal_code),
            website_url = COALESCE($15, organization_profile.website_url),
            social_links = COALESCE($16, organization_profile.social_links),
            business_hours = COALESCE($17, organization_profile.business_hours),
            timezone = COALESCE($18, organization_profile.timezone),
            currency = COALESCE($19, organization_profile.currency),
            tax_id = COALESCE($20, organization_profile.tax_id),
            updated_at = NOW()
        RETURNING id, name, slug, description, logo_url, favicon_url,
                  primary_color, secondary_color, contact_email, contact_phone,
                  address, city, state, country, postal_code, website_url,
                  social_links, business_hours, timezone, currency, tax_id,
                  created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.logo_url)
    .bind(&input.favicon_url)
    .bind(&input.primary_color)
    .bind(&input.secondary_color)
    .bind(&input.contact_email)
    .bind(&input.contact_phone)
    .bind(&input.address)
    .bind(&input.city)
    .bind(&input.state)
    .bind(&input.country)
    .bind(&input.postal_code)
    .bind(&input.website_url)
    .bind(&input.social_links)
    .bind(&input.business_hours)
    .bind(&input.timezone)
    .bind(&input.currency)
    .bind(&input.tax_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to update organization profile: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update profile", "details": e.to_string()})),
        )
    })?;

    // Audit log
    log_organization_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "organization.profile.updated",
        "organization_profile",
        Some(profile.id),
        old_value,
        Some(json!({"name": profile.name, "contact_email": profile.contact_email})),
        None,
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        "Organization profile updated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Organization profile updated",
        "data": profile
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// TEAMS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/organization/teams - List all teams
#[tracing::instrument(skip(state, admin))]
async fn list_teams(
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
        r#"
        SELECT t.id, t.name, t.slug, t.description, t.color, t.icon, t.is_active,
               COALESCE((SELECT COUNT(*) FROM team_members tm WHERE tm.team_id = t.id)::int, 0) as member_count,
               t.created_at, t.updated_at
        FROM teams t
        WHERE 1=1
        "#,
    );

    let mut bind_count = 0;

    if query.active_only.unwrap_or(false) {
        sql.push_str(" AND t.is_active = true");
    }

    if query.search.is_some() {
        bind_count += 1;
        sql.push_str(&format!(
            " AND (t.name ILIKE '%' || ${} || '%' OR t.description ILIKE '%' || ${} || '%')",
            bind_count, bind_count
        ));
    }

    sql.push_str(" ORDER BY t.name ASC");

    let mut query_builder = sqlx::query_as::<_, Team>(&sql);

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
async fn get_team(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        team_id = id,
        "Admin fetching team"
    );

    let team: Option<Team> = sqlx::query_as(
        r#"
        SELECT t.id, t.name, t.slug, t.description, t.color, t.icon, t.is_active,
               COALESCE((SELECT COUNT(*) FROM team_members tm WHERE tm.team_id = t.id)::int, 0) as member_count,
               t.created_at, t.updated_at
        FROM teams t
        WHERE t.id = $1
        "#,
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
async fn create_team(
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
async fn update_team(
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

    if old_team.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Team not found"})),
        ));
    }

    let old_team = old_team.unwrap();
    let old_value = json!({"name": old_team.name, "is_active": old_team.is_active});

    // Generate new slug if name changes
    let new_slug = input.name.as_ref().map(|n| generate_slug(n));

    let team: Team = sqlx::query_as(
        r#"
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
        "#,
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

    tracing::info!(
        admin_id = admin.0.id,
        team_id = team.id,
        "Team updated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Team updated successfully",
        "data": team
    })))
}

/// DELETE /admin/organization/teams/:id - Delete team
#[tracing::instrument(skip(state, admin))]
async fn delete_team(
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

    if old_team.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Team not found"})),
        ));
    }

    let old_team = old_team.unwrap();

    // Check if team has members
    let member_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM team_members WHERE team_id = $1"
    )
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

// ═══════════════════════════════════════════════════════════════════════════
// DEPARTMENTS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/organization/departments - List all departments
#[tracing::instrument(skip(state, admin))]
async fn list_departments(
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
        r#"
        SELECT d.id, d.name, d.slug, d.description, d.color, d.icon, d.parent_id, d.is_active,
               COALESCE((SELECT COUNT(*) FROM department_members dm WHERE dm.department_id = d.id)::int, 0) as member_count,
               d.created_at, d.updated_at
        FROM departments d
        WHERE 1=1
        "#,
    );

    let mut bind_count = 0;

    if query.active_only.unwrap_or(false) {
        sql.push_str(" AND d.is_active = true");
    }

    if query.search.is_some() {
        bind_count += 1;
        sql.push_str(&format!(
            " AND (d.name ILIKE '%' || ${} || '%' OR d.description ILIKE '%' || ${} || '%')",
            bind_count, bind_count
        ));
    }

    sql.push_str(" ORDER BY d.parent_id NULLS FIRST, d.name ASC");

    let mut query_builder = sqlx::query_as::<_, Department>(&sql);

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
async fn get_department(
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
        r#"
        SELECT d.id, d.name, d.slug, d.description, d.color, d.icon, d.parent_id, d.is_active,
               COALESCE((SELECT COUNT(*) FROM department_members dm WHERE dm.department_id = d.id)::int, 0) as member_count,
               d.created_at, d.updated_at
        FROM departments d
        WHERE d.id = $1
        "#,
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
async fn create_department(
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
        let parent_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM departments WHERE id = $1)"
        )
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
async fn update_department(
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
        let parent_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM departments WHERE id = $1)"
        )
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

    if old_dept.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Department not found"})),
        ));
    }

    let old_dept = old_dept.unwrap();
    let old_value = json!({"name": old_dept.name, "is_active": old_dept.is_active, "parent_id": old_dept.parent_id});

    let new_slug = input.name.as_ref().map(|n| generate_slug(n));

    let department: Department = sqlx::query_as(
        r#"
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
        "#,
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
async fn delete_department(
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

    if old_dept.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Department not found"})),
        ));
    }

    let old_dept = old_dept.unwrap();

    // Check for child departments
    let child_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM departments WHERE parent_id = $1"
    )
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
    let member_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM department_members WHERE department_id = $1"
    )
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

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

/// Teams admin router
pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_teams).post(create_team))
        .route("/:id", get(get_team).put(update_team).delete(delete_team))
}

/// Departments admin router
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

/// Organization profile router
pub fn profile_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_organization_profile).put(update_organization_profile))
}

/// Full organization admin router (combined)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/profile", get(get_organization_profile).put(update_organization_profile))
        .nest("/teams", teams_router())
        .nest("/departments", departments_router())
}
