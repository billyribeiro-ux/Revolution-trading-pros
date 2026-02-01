//! Page Layout Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! ICT 7 SECURITY AUDIT: Complete rewrite to add authentication and fix SQL injection

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::page_layout::{
    CreatePageLayoutRequest, PageLayout, PageLayoutListItem, PageLayoutQueryParams,
    PageLayoutVersion, PaginatedLayouts, UpdatePageLayoutRequest,
};
use crate::models::User;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// AUTHORIZATION - ICT 7 SECURITY FIX: Added admin authentication
// ═══════════════════════════════════════════════════════════════════════════════════

/// Check if user has admin privileges (admin, super-admin, or developer role)
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

// ═══════════════════════════════════════════════════════════════════════════════════
// PAGE LAYOUT CRUD - ICT 7 SECURITY FIX: All endpoints now require authentication
// and use parameterized queries
// ═══════════════════════════════════════════════════════════════════════════════════

/// GET /admin/page-layouts - List all page layouts with secure filtering
/// ICT 7 SECURITY FIX: Added authentication + parameterized queries for filtering
async fn list_layouts(
    State(state): State<AppState>,
    user: User,
    Query(params): Query<PageLayoutQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = ((page - 1) * per_page) as i64;

    // ICT 7 SECURITY FIX: Use parameterized queries instead of string interpolation
    let layouts: Vec<PageLayoutListItem> = sqlx::query_as(
        r#"
        SELECT id, course_id, title, slug, status, version, created_at, updated_at
        FROM page_layouts
        WHERE ($1::uuid IS NULL OR course_id = $1)
          AND ($2::text IS NULL OR status = $2)
        ORDER BY updated_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(params.course_id)
    .bind(params.status.as_deref())
    .bind(per_page as i64)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_layouts: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // ICT 7 SECURITY FIX: Parameterized count query
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM page_layouts
        WHERE ($1::uuid IS NULL OR course_id = $1)
          AND ($2::text IS NULL OR status = $2)
        "#,
    )
    .bind(params.course_id)
    .bind(params.status.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_layouts count: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(json!({
        "success": true,
        "data": PaginatedLayouts {
            layouts,
            total: total.0,
            page,
            per_page,
            total_pages,
        }
    })))
}

/// GET /admin/page-layouts/:id - Get single layout by ID
/// ICT 7 SECURITY FIX: Added authentication
async fn get_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let layout: PageLayout = sqlx::query_as(r#"SELECT * FROM page_layouts WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error in get_layout: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Layout not found"})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "data": layout
    })))
}

/// GET /admin/page-layouts/course/:course_id - Get layout by course ID
/// ICT 7 SECURITY FIX: Added authentication
async fn get_layout_by_course(
    State(state): State<AppState>,
    user: User,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let layout: Option<PageLayout> = sqlx::query_as(
        r#"SELECT * FROM page_layouts WHERE course_id = $1 ORDER BY updated_at DESC LIMIT 1"#,
    )
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in get_layout_by_course: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": layout
    })))
}

/// POST /admin/page-layouts - Create new page layout
/// ICT 7 SECURITY FIX: Added authentication + input validation
async fn create_layout(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreatePageLayoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Input validation
    if input.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Title cannot be empty"})),
        ));
    }

    if input.title.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Title too long (max 255 characters)"})),
        ));
    }

    let slug = input.slug.unwrap_or_else(|| slugify(&input.title));
    let blocks = input.blocks.unwrap_or_else(|| json!([]));

    let layout: PageLayout = sqlx::query_as(
        r#"
        INSERT INTO page_layouts (course_id, title, slug, description, blocks, status, created_by)
        VALUES ($1, $2, $3, $4, $5, 'draft', $6)
        RETURNING *
        "#,
    )
    .bind(input.course_id)
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&blocks)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "A layout with this slug already exists"})),
            )
        } else {
            tracing::error!("Database error in create_layout: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to create layout"})),
            )
        }
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %layout.id,
        "Page layout created"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout created successfully",
        "data": layout
    })))
}

/// PUT /admin/page-layouts/:id - Update page layout
/// ICT 7 SECURITY FIX: Added authentication + input validation
async fn update_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePageLayoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Input validation
    if let Some(ref title) = input.title {
        if title.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Title cannot be empty"})),
            ));
        }
        if title.len() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Title too long (max 255 characters)"})),
            ));
        }
    }

    // First, save current version to history if blocks are being updated
    if input.blocks.is_some() {
        let current: Option<PageLayout> =
            sqlx::query_as("SELECT * FROM page_layouts WHERE id = $1")
                .bind(id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

        if let Some(layout) = current {
            let _ = sqlx::query(
                r#"
                INSERT INTO page_layout_versions (layout_id, version, blocks, created_by)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (layout_id, version) DO NOTHING
                "#,
            )
            .bind(id)
            .bind(layout.version.unwrap_or(1))
            .bind(&layout.blocks)
            .bind(user.id)
            .execute(&state.db.pool)
            .await;
        }
    }

    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts SET
            title = COALESCE($1, title),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            status = COALESCE($4, status),
            blocks = COALESCE($5, blocks),
            version = CASE WHEN $5 IS NOT NULL THEN COALESCE(version, 0) + 1 ELSE version END,
            published_at = CASE WHEN $4 = 'published' THEN NOW() ELSE published_at END,
            updated_at = NOW(),
            updated_by = $7
        WHERE id = $6
        RETURNING *
        "#,
    )
    .bind(&input.title)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.status)
    .bind(&input.blocks)
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in update_layout: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update layout"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Layout not found"})),
        )
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %id,
        "Page layout updated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout updated successfully",
        "data": layout
    })))
}

/// DELETE /admin/page-layouts/:id - Delete page layout
/// ICT 7 SECURITY FIX: Added authentication
async fn delete_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // First check if layout exists
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM page_layouts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error in delete_layout check: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    if exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Layout not found"})),
        ));
    }

    // Delete version history first (due to foreign key)
    let _ = sqlx::query("DELETE FROM page_layout_versions WHERE layout_id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await;

    // Delete the layout
    sqlx::query("DELETE FROM page_layouts WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error in delete_layout: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete layout"})),
            )
        })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %id,
        "Page layout deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout deleted successfully"
    })))
}

/// POST /admin/page-layouts/:id/publish - Publish page layout
/// ICT 7 SECURITY FIX: Added authentication
async fn publish_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts
        SET status = 'published', published_at = NOW(), updated_at = NOW(), published_by = $2
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in publish_layout: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to publish layout"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Layout not found"})),
        )
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %id,
        "Page layout published"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout published successfully",
        "data": layout
    })))
}

/// POST /admin/page-layouts/:id/unpublish - Unpublish page layout (revert to draft)
/// ICT 7 SECURITY FIX: Added new endpoint for completeness
async fn unpublish_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts
        SET status = 'draft', updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in unpublish_layout: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to unpublish layout"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Layout not found"})),
        )
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %id,
        "Page layout unpublished"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout unpublished successfully",
        "data": layout
    })))
}

/// GET /admin/page-layouts/:id/versions - Get layout version history
/// ICT 7 SECURITY FIX: Added authentication
async fn get_layout_versions(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let versions: Vec<PageLayoutVersion> = sqlx::query_as(
        r#"
        SELECT * FROM page_layout_versions
        WHERE layout_id = $1
        ORDER BY version DESC
        LIMIT 50
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in get_layout_versions: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": versions
    })))
}

/// POST /admin/page-layouts/:id/versions/:version/restore - Restore to specific version
/// ICT 7 SECURITY FIX: Added authentication
async fn restore_layout_version(
    State(state): State<AppState>,
    user: User,
    Path((id, version)): Path<(Uuid, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Validate version number
    if version < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid version number"})),
        ));
    }

    let version_data: PageLayoutVersion =
        sqlx::query_as("SELECT * FROM page_layout_versions WHERE layout_id = $1 AND version = $2")
            .bind(id)
            .bind(version)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error in restore_layout_version: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Database error"})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Version not found"})),
                )
            })?;

    // Save current state before restoring
    let current: Option<PageLayout> = sqlx::query_as("SELECT * FROM page_layouts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    if let Some(layout) = current {
        let _ = sqlx::query(
            r#"
            INSERT INTO page_layout_versions (layout_id, version, blocks, created_by)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (layout_id, version) DO NOTHING
            "#,
        )
        .bind(id)
        .bind(layout.version.unwrap_or(1))
        .bind(&layout.blocks)
        .bind(user.id)
        .execute(&state.db.pool)
        .await;
    }

    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts SET
            blocks = $1,
            version = COALESCE(version, 0) + 1,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $2
        RETURNING *
        "#,
    )
    .bind(&version_data.blocks)
    .bind(id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in restore_layout_version: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to restore version"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Layout not found"})),
        )
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        layout_id = %id,
        restored_version = version,
        "Page layout version restored"
    );

    Ok(Json(json!({
        "success": true,
        "message": format!("Restored to version {}", version),
        "data": layout
    })))
}

/// POST /admin/page-layouts/:id/duplicate - Duplicate a page layout
/// ICT 7: Added new endpoint for completeness
async fn duplicate_layout(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Get the original layout
    let original: PageLayout = sqlx::query_as("SELECT * FROM page_layouts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error in duplicate_layout: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Layout not found"})),
            )
        })?;

    // Create duplicate with new title and slug
    let new_title = format!("{} (Copy)", original.title);
    let new_slug = format!("{}-copy-{}", original.slug.as_deref().unwrap_or("layout"), chrono::Utc::now().timestamp());

    let layout: PageLayout = sqlx::query_as(
        r#"
        INSERT INTO page_layouts (course_id, title, slug, description, blocks, status, created_by)
        VALUES ($1, $2, $3, $4, $5, 'draft', $6)
        RETURNING *
        "#,
    )
    .bind(original.course_id)
    .bind(&new_title)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.blocks)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in duplicate_layout: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to duplicate layout"})),
        )
    })?;

    tracing::info!(
        target: "admin_audit",
        user_id = user.id,
        original_id = %id,
        new_id = %layout.id,
        "Page layout duplicated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Layout duplicated successfully",
        "data": layout
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER - ICT 7 SECURITY FIX: All routes now require admin authentication
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_layouts).post(create_layout))
        .route(
            "/{id}",
            get(get_layout).put(update_layout).delete(delete_layout),
        )
        .route("/{id}/publish", post(publish_layout))
        .route("/{id}/unpublish", post(unpublish_layout))
        .route("/{id}/duplicate", post(duplicate_layout))
        .route("/{id}/versions", get(get_layout_versions))
        .route(
            "/{id}/versions/{version}/restore",
            post(restore_layout_version),
        )
        .route("/course/{course_id}", get(get_layout_by_course))
}
