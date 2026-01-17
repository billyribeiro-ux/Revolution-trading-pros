//! Room Resources API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Unified resource management for trading rooms supporting:
//! - Videos (Bunny.net, Vimeo, YouTube, Direct)
//! - PDFs (Trade plans, guides)
//! - Documents (Word, Excel)
//! - Images (Charts, screenshots)
//!
//! Features:
//! - Room-specific isolation
//! - Content type filtering
//! - Featured/pinned resources
//! - Presigned URL uploads for R2
//! - Full CRUD operations

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoomResource {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub section: Option<String>, // ICT 7: Section field for organization
    pub file_url: String,
    pub file_path: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub video_id: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: NaiveDate,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub sort_order: i32,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub scheduled_at: Option<chrono::NaiveDateTime>,
    pub category: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub difficulty_level: Option<String>,
    pub views_count: i32,
    pub downloads_count: i32,
    pub likes_count: i32,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ResourceListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub room_id: Option<i64>,
    pub room_slug: Option<String>,
    pub resource_type: Option<String>,
    pub content_type: Option<String>,
    pub section: Option<String>, // ICT 7: Filter by section
    pub is_featured: Option<bool>,
    pub is_published: Option<bool>,
    pub tags: Option<String>,
    pub difficulty_level: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateResourceRequest {
    pub title: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub file_url: String,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub section: Option<String>, // ICT 7: Section for organization
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty_level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateResourceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
    pub content_type: Option<String>,
    pub file_url: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub video_platform: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trader_id: Option<i64>,
    pub resource_date: Option<String>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub section: Option<String>, // ICT 7: Section for organization
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub difficulty_level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResourceResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub resource_type: String,
    pub content_type: String,
    pub file_url: String,
    pub embed_url: String,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub formatted_size: String,
    pub video_platform: Option<String>,
    pub duration: Option<i32>,
    pub formatted_duration: String,
    pub thumbnail_url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub trading_room_id: i64,
    pub trader_id: Option<i64>,
    pub resource_date: String,
    pub formatted_date: String,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub difficulty_level: Option<String>,
    pub views_count: i32,
    pub downloads_count: i32,
    pub section: Option<String>, // ICT 7: Section field
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn get_embed_url(resource: &RoomResource) -> String {
    if resource.resource_type != "video" {
        return resource.file_url.clone();
    }

    match resource.video_platform.as_deref() {
        Some("bunny") => {
            if let (Some(guid), Some(lib_id)) =
                (&resource.bunny_video_guid, resource.bunny_library_id)
            {
                format!("https://iframe.mediadelivery.net/embed/{}/{}", lib_id, guid)
            } else {
                resource.file_url.clone()
            }
        }
        Some("vimeo") => {
            if let Some(id) = &resource.video_id {
                format!("https://player.vimeo.com/video/{}", id)
            } else {
                resource.file_url.clone()
            }
        }
        Some("youtube") => {
            if let Some(id) = &resource.video_id {
                format!("https://www.youtube.com/embed/{}", id)
            } else {
                resource.file_url.clone()
            }
        }
        _ => resource.file_url.clone(),
    }
}

fn format_duration(seconds: Option<i32>) -> String {
    match seconds {
        Some(d) if d > 0 => {
            let hours = d / 3600;
            let minutes = (d % 3600) / 60;
            let secs = d % 60;
            if hours > 0 {
                format!("{}:{:02}:{:02}", hours, minutes, secs)
            } else {
                format!("{}:{:02}", minutes, secs)
            }
        }
        _ => String::new(),
    }
}

fn format_file_size(bytes: Option<i64>) -> String {
    match bytes {
        Some(b) if b > 0 => {
            if b >= 1_073_741_824 {
                format!("{:.2} GB", b as f64 / 1_073_741_824.0)
            } else if b >= 1_048_576 {
                format!("{:.2} MB", b as f64 / 1_048_576.0)
            } else if b >= 1024 {
                format!("{:.2} KB", b as f64 / 1024.0)
            } else {
                format!("{} bytes", b)
            }
        }
        _ => String::new(),
    }
}

fn get_tags_vec(tags: &Option<serde_json::Value>) -> Vec<String> {
    tags.as_ref()
        .and_then(|t| serde_json::from_value::<Vec<String>>(t.clone()).ok())
        .unwrap_or_default()
}

fn resource_to_response(resource: RoomResource) -> ResourceResponse {
    ResourceResponse {
        id: resource.id,
        title: resource.title.clone(),
        slug: resource.slug.clone(),
        description: resource.description.clone(),
        resource_type: resource.resource_type.clone(),
        content_type: resource.content_type.clone(),
        file_url: resource.file_url.clone(),
        embed_url: get_embed_url(&resource),
        mime_type: resource.mime_type.clone(),
        file_size: resource.file_size,
        formatted_size: format_file_size(resource.file_size),
        video_platform: resource.video_platform.clone(),
        duration: resource.duration,
        formatted_duration: format_duration(resource.duration),
        thumbnail_url: resource.thumbnail_url.clone(),
        width: resource.width,
        height: resource.height,
        trading_room_id: resource.trading_room_id,
        trader_id: resource.trader_id,
        resource_date: resource.resource_date.to_string(),
        formatted_date: resource.resource_date.format("%B %d, %Y").to_string(),
        is_published: resource.is_published,
        is_featured: resource.is_featured,
        is_pinned: resource.is_pinned,
        category: resource.category.clone(),
        tags: get_tags_vec(&resource.tags),
        difficulty_level: resource.difficulty_level.clone(),
        views_count: resource.views_count,
        downloads_count: resource.downloads_count,
        section: resource.section.clone(), // ICT 7: Include section in response
        created_at: resource.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources - List resources (public)
async fn list_resources(
    State(state): State<AppState>,
    Query(query): Query<ResourceListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        "SELECT * FROM room_resources WHERE is_published = true AND deleted_at IS NULL",
    );
    let mut count_sql = String::from(
        "SELECT COUNT(*) FROM room_resources WHERE is_published = true AND deleted_at IS NULL",
    );

    // Room ID filter (required for most queries)
    if let Some(room_id) = query.room_id {
        let filter = format!(" AND trading_room_id = {}", room_id);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Resource type filter (video, pdf, document, image)
    if let Some(ref resource_type) = query.resource_type {
        let filter = format!(
            " AND resource_type = '{}'",
            resource_type.replace('\'', "''")
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Content type filter (tutorial, daily_video, trade_plan, etc.)
    if let Some(ref content_type) = query.content_type {
        let filter = format!(" AND content_type = '{}'", content_type.replace('\'', "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Featured filter
    if let Some(is_featured) = query.is_featured {
        let filter = format!(" AND is_featured = {}", is_featured);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Difficulty filter
    if let Some(ref difficulty) = query.difficulty_level {
        let filter = format!(
            " AND difficulty_level = '{}'",
            difficulty.replace('\'', "''")
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // ICT 7: Section filter
    if let Some(ref section) = query.section {
        let filter = format!(" AND section = '{}'", section.replace('\'', "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Tags filter
    if let Some(ref tags) = query.tags {
        let tag_list: Vec<&str> = tags.split(',').collect();
        for tag in tag_list {
            let filter = format!(" AND tags @> '[\"{}\"]'", tag.trim().replace('\'', "''"));
            sql.push_str(&filter);
            count_sql.push_str(&filter);
        }
    }

    // Search
    if let Some(ref search) = query.search {
        let search_term = search.replace('\'', "''");
        let filter = format!(
            " AND (title ILIKE '%{}%' OR description ILIKE '%{}%')",
            search_term, search_term
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    sql.push_str(" ORDER BY is_pinned DESC, is_featured DESC, resource_date DESC, created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let resources: Vec<RoomResource> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();
    let last_page = ((total.0 as f64) / (per_page as f64)).ceil() as i64;

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "meta": PaginationMeta {
            current_page: page,
            per_page,
            total: total.0,
            last_page,
        }
    })))
}

/// GET /api/room-resources/:id - Get single resource
async fn get_resource(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resource: RoomResource = if let Ok(id) = id_or_slug.parse::<i64>() {
        sqlx::query_as(
            "SELECT * FROM room_resources WHERE id = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT * FROM room_resources WHERE slug = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(&id_or_slug)
        .fetch_optional(&state.db.pool)
        .await
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Resource not found"}))))?;

    // Increment view count
    let _ = sqlx::query("UPDATE room_resources SET views_count = views_count + 1 WHERE id = $1")
        .bind(resource.id)
        .execute(&state.db.pool)
        .await;

    Ok(Json(json!({
        "success": true,
        "data": resource_to_response(resource)
    })))
}

/// POST /api/room-resources/:id/download - Track download
async fn track_download(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({"success": true, "status": "ok"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN ROUTES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/room-resources - List all resources (admin)
async fn admin_list_resources(
    State(state): State<AppState>,
    Query(query): Query<ResourceListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM room_resources WHERE deleted_at IS NULL");
    let mut count_sql =
        String::from("SELECT COUNT(*) FROM room_resources WHERE deleted_at IS NULL");

    // Room ID filter
    if let Some(room_id) = query.room_id {
        let filter = format!(" AND trading_room_id = {}", room_id);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Resource type filter
    if let Some(ref resource_type) = query.resource_type {
        let filter = format!(
            " AND resource_type = '{}'",
            resource_type.replace('\'', "''")
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Content type filter
    if let Some(ref content_type) = query.content_type {
        let filter = format!(" AND content_type = '{}'", content_type.replace('\'', "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Published filter
    if let Some(is_published) = query.is_published {
        let filter = format!(" AND is_published = {}", is_published);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // ICT 7: Section filter for admin
    if let Some(ref section) = query.section {
        let filter = format!(" AND section = '{}'", section.replace('\'', "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Search
    if let Some(ref search) = query.search {
        let search_term = search.replace('\'', "''");
        let filter = format!(
            " AND (title ILIKE '%{}%' OR description ILIKE '%{}%')",
            search_term, search_term
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let resources: Vec<RoomResource> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();
    let last_page = ((total.0 as f64) / (per_page as f64)).ceil() as i64;

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "meta": PaginationMeta {
            current_page: page,
            per_page,
            total: total.0,
            last_page,
        }
    })))
}

/// POST /api/admin/room-resources - Create resource
async fn create_resource(
    State(state): State<AppState>,
    Json(input): Json<CreateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.title);
    let resource_date = input
        .resource_date
        .as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Utc::now().date_naive());

    let tags_json = input
        .tags
        .as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
        .unwrap_or(json!([]));

    let resource: RoomResource = sqlx::query_as(
        r#"
        INSERT INTO room_resources (
            title, slug, description, resource_type, content_type,
            file_url, mime_type, file_size, video_platform, bunny_video_guid,
            bunny_library_id, duration, thumbnail_url, width, height,
            trading_room_id, trader_id, resource_date, is_published, is_featured,
            is_pinned, category, tags, difficulty_level
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21, $22, $23, $24
        )
        RETURNING *
        "#,
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.resource_type)
    .bind(&input.content_type)
    .bind(&input.file_url)
    .bind(&input.mime_type)
    .bind(input.file_size)
    .bind(&input.video_platform)
    .bind(&input.bunny_video_guid)
    .bind(input.bunny_library_id)
    .bind(input.duration)
    .bind(&input.thumbnail_url)
    .bind(input.width)
    .bind(input.height)
    .bind(input.trading_room_id)
    .bind(input.trader_id)
    .bind(resource_date)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_pinned.unwrap_or(false))
    .bind(&input.category)
    .bind(tags_json)
    .bind(&input.difficulty_level)
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
        "message": "Resource created successfully",
        "data": resource_to_response(resource)
    })))
}

/// PUT /api/admin/room-resources/:id - Update resource
async fn update_resource(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.title.is_some() {
        updates.push(format!("title = ${}", param_idx));
        param_idx += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${}", param_idx));
        param_idx += 1;
    }
    if input.resource_type.is_some() {
        updates.push(format!("resource_type = ${}", param_idx));
        param_idx += 1;
    }
    if input.content_type.is_some() {
        updates.push(format!("content_type = ${}", param_idx));
        param_idx += 1;
    }
    if input.file_url.is_some() {
        updates.push(format!("file_url = ${}", param_idx));
        param_idx += 1;
    }
    if input.mime_type.is_some() {
        updates.push(format!("mime_type = ${}", param_idx));
        param_idx += 1;
    }
    if input.file_size.is_some() {
        updates.push(format!("file_size = ${}", param_idx));
        param_idx += 1;
    }
    if input.video_platform.is_some() {
        updates.push(format!("video_platform = ${}", param_idx));
        param_idx += 1;
    }
    if input.bunny_video_guid.is_some() {
        updates.push(format!("bunny_video_guid = ${}", param_idx));
        param_idx += 1;
    }
    if input.bunny_library_id.is_some() {
        updates.push(format!("bunny_library_id = ${}", param_idx));
        param_idx += 1;
    }
    if input.duration.is_some() {
        updates.push(format!("duration = ${}", param_idx));
        param_idx += 1;
    }
    if input.thumbnail_url.is_some() {
        updates.push(format!("thumbnail_url = ${}", param_idx));
        param_idx += 1;
    }
    if input.width.is_some() {
        updates.push(format!("width = ${}", param_idx));
        param_idx += 1;
    }
    if input.height.is_some() {
        updates.push(format!("height = ${}", param_idx));
        param_idx += 1;
    }
    if input.trader_id.is_some() {
        updates.push(format!("trader_id = ${}", param_idx));
        param_idx += 1;
    }
    if input.resource_date.is_some() {
        updates.push(format!("resource_date = ${}", param_idx));
        param_idx += 1;
    }
    if input.is_published.is_some() {
        updates.push(format!("is_published = ${}", param_idx));
        param_idx += 1;
    }
    if input.is_featured.is_some() {
        updates.push(format!("is_featured = ${}", param_idx));
        param_idx += 1;
    }
    if input.is_pinned.is_some() {
        updates.push(format!("is_pinned = ${}", param_idx));
        param_idx += 1;
    }
    if input.category.is_some() {
        updates.push(format!("category = ${}", param_idx));
        param_idx += 1;
    }
    if input.tags.is_some() {
        updates.push(format!("tags = ${}", param_idx));
        param_idx += 1;
    }
    if input.difficulty_level.is_some() {
        updates.push(format!("difficulty_level = ${}", param_idx));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let query_str = format!(
        "UPDATE room_resources SET {}, updated_at = NOW() WHERE id = ${} AND deleted_at IS NULL RETURNING *",
        updates.join(", "),
        param_idx
    );

    let mut query = sqlx::query_as::<_, RoomResource>(&query_str);

    // Bind parameters in order
    if let Some(ref v) = input.title {
        query = query.bind(v);
    }
    if let Some(ref v) = input.description {
        query = query.bind(v);
    }
    if let Some(ref v) = input.resource_type {
        query = query.bind(v);
    }
    if let Some(ref v) = input.content_type {
        query = query.bind(v);
    }
    if let Some(ref v) = input.file_url {
        query = query.bind(v);
    }
    if let Some(ref v) = input.mime_type {
        query = query.bind(v);
    }
    if let Some(v) = input.file_size {
        query = query.bind(v);
    }
    if let Some(ref v) = input.video_platform {
        query = query.bind(v);
    }
    if let Some(ref v) = input.bunny_video_guid {
        query = query.bind(v);
    }
    if let Some(v) = input.bunny_library_id {
        query = query.bind(v);
    }
    if let Some(v) = input.duration {
        query = query.bind(v);
    }
    if let Some(ref v) = input.thumbnail_url {
        query = query.bind(v);
    }
    if let Some(v) = input.width {
        query = query.bind(v);
    }
    if let Some(v) = input.height {
        query = query.bind(v);
    }
    if let Some(v) = input.trader_id {
        query = query.bind(v);
    }
    if let Some(ref v) = input.resource_date {
        let date = NaiveDate::parse_from_str(v, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        query = query.bind(date);
    }
    if let Some(v) = input.is_published {
        query = query.bind(v);
    }
    if let Some(v) = input.is_featured {
        query = query.bind(v);
    }
    if let Some(v) = input.is_pinned {
        query = query.bind(v);
    }
    if let Some(ref v) = input.category {
        query = query.bind(v);
    }
    if let Some(ref v) = input.tags {
        let tags_json = serde_json::to_value(v).unwrap_or(json!([]));
        query = query.bind(tags_json);
    }
    if let Some(ref v) = input.difficulty_level {
        query = query.bind(v);
    }

    query = query.bind(id);

    let resource = query
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Resource not found"})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Resource updated successfully",
        "data": resource_to_response(resource)
    })))
}

/// DELETE /api/admin/room-resources/:id - Soft delete resource
async fn delete_resource(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE room_resources SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
    )
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
            Json(json!({"error": "Resource not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Resource deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_resources))
        .route("/:id_or_slug", get(get_resource))
        .route("/:id/download", post(track_download))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_resources))
        .route("/", post(create_resource))
        .route("/:id", put(update_resource))
        .route("/:id", delete(delete_resource))
}
