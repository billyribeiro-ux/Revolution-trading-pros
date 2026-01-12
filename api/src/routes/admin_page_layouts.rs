//! Page Layout Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026

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
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// PAGE LAYOUT CRUD
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_layouts(
    State(state): State<AppState>,
    Query(params): Query<PageLayoutQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut query = String::from(
        r#"
        SELECT id, course_id, title, slug, status, version, created_at, updated_at
        FROM page_layouts
        WHERE 1=1
        "#,
    );

    if let Some(ref course_id) = params.course_id {
        query.push_str(&format!(" AND course_id = '{}'", course_id));
    }

    if let Some(ref status) = params.status {
        query.push_str(&format!(" AND status = '{}'", status));
    }

    query.push_str(" ORDER BY updated_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let layouts: Vec<PageLayoutListItem> = sqlx::query_as(&query)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let count_query = if params.course_id.is_some() {
        format!("SELECT COUNT(*) FROM page_layouts WHERE course_id = '{}'", params.course_id.unwrap())
    } else {
        "SELECT COUNT(*) FROM page_layouts".to_string()
    };

    let total: (i64,) = sqlx::query_as(&count_query)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
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

async fn get_layout(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let layout: PageLayout = sqlx::query_as(
        r#"SELECT * FROM page_layouts WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
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

async fn get_layout_by_course(
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let layout: Option<PageLayout> = sqlx::query_as(
        r#"SELECT * FROM page_layouts WHERE course_id = $1 ORDER BY updated_at DESC LIMIT 1"#,
    )
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": layout
    })))
}

async fn create_layout(
    State(state): State<AppState>,
    Json(input): Json<CreatePageLayoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.unwrap_or_else(|| slugify(&input.title));
    let blocks = input.blocks.unwrap_or_else(|| json!([]));

    let layout: PageLayout = sqlx::query_as(
        r#"
        INSERT INTO page_layouts (course_id, title, slug, description, blocks, status)
        VALUES ($1, $2, $3, $4, $5, 'draft')
        RETURNING *
        "#,
    )
    .bind(input.course_id)
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&blocks)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create layout: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Layout created successfully",
        "data": layout
    })))
}

async fn update_layout(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePageLayoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // First, save current version to history if blocks are being updated
    if input.blocks.is_some() {
        let current: Option<PageLayout> = sqlx::query_as(
            "SELECT * FROM page_layouts WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

        if let Some(layout) = current {
            let _ = sqlx::query(
                r#"
                INSERT INTO page_layout_versions (layout_id, version, blocks)
                VALUES ($1, $2, $3)
                ON CONFLICT (layout_id, version) DO NOTHING
                "#,
            )
            .bind(id)
            .bind(layout.version.unwrap_or(1))
            .bind(&layout.blocks)
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
            updated_at = NOW()
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
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update layout: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Layout updated successfully",
        "data": layout
    })))
}

async fn delete_layout(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM page_layouts WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete layout: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Layout deleted successfully"
    })))
}

async fn publish_layout(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts 
        SET status = 'published', published_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to publish layout: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Layout published successfully",
        "data": layout
    })))
}

async fn get_layout_versions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let versions: Vec<PageLayoutVersion> = sqlx::query_as(
        r#"
        SELECT * FROM page_layout_versions 
        WHERE layout_id = $1 
        ORDER BY version DESC
        LIMIT 20
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": versions
    })))
}

async fn restore_layout_version(
    State(state): State<AppState>,
    Path((id, version)): Path<(Uuid, i32)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let version_data: PageLayoutVersion = sqlx::query_as(
        "SELECT * FROM page_layout_versions WHERE layout_id = $1 AND version = $2",
    )
    .bind(id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Version not found"})),
        )
    })?;

    let layout: PageLayout = sqlx::query_as(
        r#"
        UPDATE page_layouts SET
            blocks = $1,
            version = COALESCE(version, 0) + 1,
            updated_at = NOW()
        WHERE id = $2
        RETURNING *
        "#,
    )
    .bind(&version_data.blocks)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to restore version: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("Restored to version {}", version),
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
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_layouts).post(create_layout))
        .route("/{id}", get(get_layout).put(update_layout).delete(delete_layout))
        .route("/{id}/publish", post(publish_layout))
        .route("/{id}/versions", get(get_layout_versions))
        .route("/{id}/versions/{version}/restore", post(restore_layout_version))
        .route("/course/{course_id}", get(get_layout_by_course))
}
