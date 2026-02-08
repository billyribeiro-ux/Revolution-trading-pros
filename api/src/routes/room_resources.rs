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

use crate::middleware::admin::AdminUser;
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
    // ICT 7 NEW: Access control
    pub access_level: Option<String>, // free, member, premium, vip
    // ICT 7 NEW: Versioning
    pub version: Option<i32>,
    pub previous_version_id: Option<i64>,
    pub is_latest_version: Option<bool>,
    // ICT 7 NEW: Course/Lesson linking
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
    // ICT 7 NEW: Secure download
    pub secure_token: Option<String>,
    pub secure_token_expires: Option<chrono::NaiveDateTime>,
    // ICT 7 NEW: Storage tracking
    pub file_hash: Option<String>,
    pub storage_provider: Option<String>,
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
    // ICT 7 NEW: Access control and versioning filters
    pub access_level: Option<String>, // free, member, premium, vip
    pub course_id: Option<i64>,       // Filter by course
    pub lesson_id: Option<i64>,       // Filter by lesson
    pub latest_only: Option<bool>,    // Only latest versions (default true)
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
    // ICT 7 NEW: Access control and linking
    pub access_level: Option<String>, // free, member, premium, vip (default: premium)
    pub course_id: Option<i64>,       // Link to course
    pub lesson_id: Option<i64>,       // Link to lesson
    pub course_order: Option<i32>,    // Order in course
    pub file_hash: Option<String>,    // SHA-256 for deduplication
    pub storage_provider: Option<String>, // r2, bunny, s3, local
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
    // ICT 7 NEW: Access control and linking
    pub access_level: Option<String>,
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
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
    pub secure_download_url: Option<String>, // ICT 7: Signed URL for secure downloads
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
    // ICT 7 NEW: Access control
    pub access_level: String,
    pub requires_premium: bool,
    // ICT 7 NEW: Versioning
    pub version: i32,
    pub has_previous_version: bool,
    pub is_latest_version: bool,
    // ICT 7 NEW: Course/Lesson linking
    pub course_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub course_order: Option<i32>,
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
    let access_level = resource
        .access_level
        .clone()
        .unwrap_or_else(|| "premium".to_string());
    let requires_premium = access_level != "free";

    ResourceResponse {
        id: resource.id,
        title: resource.title.clone(),
        slug: resource.slug.clone(),
        description: resource.description.clone(),
        resource_type: resource.resource_type.clone(),
        content_type: resource.content_type.clone(),
        file_url: resource.file_url.clone(),
        embed_url: get_embed_url(&resource),
        secure_download_url: None, // Generated on-demand via /download endpoint
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
        section: resource.section.clone(),
        created_at: resource.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        // ICT 7 NEW: Access control
        access_level: access_level.clone(),
        requires_premium,
        // ICT 7 NEW: Versioning
        version: resource.version.unwrap_or(1),
        has_previous_version: resource.previous_version_id.is_some(),
        is_latest_version: resource.is_latest_version.unwrap_or(true),
        // ICT 7 NEW: Course/Lesson linking
        course_id: resource.course_id,
        lesson_id: resource.lesson_id,
        course_order: resource.course_order,
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

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.room_id.is_some() {
        conditions.push(format!("trading_room_id = ${}", param_idx));
        param_idx += 1;
    }
    if query.resource_type.is_some() {
        conditions.push(format!("resource_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.content_type.is_some() {
        conditions.push(format!("content_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.is_featured.is_some() {
        conditions.push(format!("is_featured = ${}", param_idx));
        param_idx += 1;
    }
    if query.difficulty_level.is_some() {
        conditions.push(format!("difficulty_level = ${}", param_idx));
        param_idx += 1;
    }
    if query.section.is_some() {
        conditions.push(format!("section = ${}", param_idx));
        param_idx += 1;
    }
    // Tags: each tag gets its own JSONB containment check
    let tag_list: Vec<String> = query
        .tags
        .as_ref()
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    for _ in &tag_list {
        conditions.push(format!("tags @> ${}", param_idx));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(title ILIKE ${} OR description ILIKE ${})",
            param_idx, param_idx
        ));
        param_idx += 1;
    }
    if query.access_level.is_some() {
        conditions.push(format!("access_level = ${}", param_idx));
        param_idx += 1;
    }
    if query.course_id.is_some() {
        conditions.push(format!("course_id = ${}", param_idx));
        param_idx += 1;
    }
    if query.lesson_id.is_some() {
        conditions.push(format!("lesson_id = ${}", param_idx));
        param_idx += 1;
    }

    // ICT 7 NEW: Only show latest versions by default
    let show_latest_only = query.latest_only.unwrap_or(true);
    if show_latest_only {
        conditions.push("(is_latest_version = true OR is_latest_version IS NULL)".to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT * FROM room_resources WHERE is_published = true AND deleted_at IS NULL{} ORDER BY is_pinned DESC, is_featured DESC, resource_date DESC, created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    let count_sql = format!(
        "SELECT COUNT(*) FROM room_resources WHERE is_published = true AND deleted_at IS NULL{}",
        where_clause
    );

    // Helper macro-like closure to bind params to a query
    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, RoomResource>(&sql);
    if let Some(room_id) = query.room_id {
        q = q.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        q = q.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        q = q.bind(content_type);
    }
    if let Some(is_featured) = query.is_featured {
        q = q.bind(is_featured);
    }
    if let Some(ref difficulty) = query.difficulty_level {
        q = q.bind(difficulty);
    }
    if let Some(ref section) = query.section {
        q = q.bind(section);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        q = q.bind(tag_json);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        q = q.bind(search_pattern);
    }
    if let Some(ref access_level) = query.access_level {
        q = q.bind(access_level);
    }
    if let Some(course_id) = query.course_id {
        q = q.bind(course_id);
    }
    if let Some(lesson_id) = query.lesson_id {
        q = q.bind(lesson_id);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    let resources: Vec<RoomResource> = q.fetch_all(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Bind parameters for the count query (same filters, no LIMIT/OFFSET)
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(room_id) = query.room_id {
        cq = cq.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        cq = cq.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        cq = cq.bind(content_type);
    }
    if let Some(is_featured) = query.is_featured {
        cq = cq.bind(is_featured);
    }
    if let Some(ref difficulty) = query.difficulty_level {
        cq = cq.bind(difficulty);
    }
    if let Some(ref section) = query.section {
        cq = cq.bind(section);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        cq = cq.bind(tag_json);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        cq = cq.bind(search_pattern);
    }
    if let Some(ref access_level) = query.access_level {
        cq = cq.bind(access_level);
    }
    if let Some(course_id) = query.course_id {
        cq = cq.bind(course_id);
    }
    if let Some(lesson_id) = query.lesson_id {
        cq = cq.bind(lesson_id);
    }

    let total: (i64,) = cq.fetch_one(&state.db.pool).await.unwrap_or((0,));

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
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<ResourceListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.room_id.is_some() {
        conditions.push(format!("trading_room_id = ${}", param_idx));
        param_idx += 1;
    }
    if query.resource_type.is_some() {
        conditions.push(format!("resource_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.content_type.is_some() {
        conditions.push(format!("content_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.is_published.is_some() {
        conditions.push(format!("is_published = ${}", param_idx));
        param_idx += 1;
    }
    if query.section.is_some() {
        conditions.push(format!("section = ${}", param_idx));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(title ILIKE ${} OR description ILIKE ${})",
            param_idx, param_idx
        ));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT * FROM room_resources WHERE deleted_at IS NULL{} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    let count_sql = format!(
        "SELECT COUNT(*) FROM room_resources WHERE deleted_at IS NULL{}",
        where_clause
    );

    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, RoomResource>(&sql);
    if let Some(room_id) = query.room_id {
        q = q.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        q = q.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        q = q.bind(content_type);
    }
    if let Some(is_published) = query.is_published {
        q = q.bind(is_published);
    }
    if let Some(ref section) = query.section {
        q = q.bind(section);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        q = q.bind(search_pattern);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    let resources: Vec<RoomResource> = q.fetch_all(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Bind parameters for the count query (same filters, no LIMIT/OFFSET)
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(room_id) = query.room_id {
        cq = cq.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        cq = cq.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        cq = cq.bind(content_type);
    }
    if let Some(is_published) = query.is_published {
        cq = cq.bind(is_published);
    }
    if let Some(ref section) = query.section {
        cq = cq.bind(section);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        cq = cq.bind(search_pattern);
    }

    let total: (i64,) = cq.fetch_one(&state.db.pool).await.unwrap_or((0,));

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
    _admin: AdminUser,
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
    _admin: AdminUser,
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
    _admin: AdminUser,
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
// ICT 7 NEW: SECURE DOWNLOAD
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/room-resources/:id/secure-download - Generate secure download URL
async fn generate_secure_download(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Generate a secure token
    let token: String = sqlx::query_scalar("SELECT generate_secure_download_token($1, 24)")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Get the resource to include in response
    let resource: RoomResource =
        sqlx::query_as("SELECT * FROM room_resources WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
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

    // Construct the secure download URL
    let secure_url = format!("/api/room-resources/{}/download?token={}", id, token);

    // Track download attempt
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "success": true,
        "download_url": secure_url,
        "file_url": resource.file_url,
        "filename": format!("{}.{}", resource.slug, resource.mime_type.as_deref().unwrap_or("bin").split('/').next_back().unwrap_or("bin")),
        "expires_in_hours": 24
    })))
}

/// GET /api/room-resources/:id/download - Download with optional token verification
async fn download_resource(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the resource
    let resource: RoomResource = sqlx::query_as(
        "SELECT * FROM room_resources WHERE id = $1 AND is_published = true AND deleted_at IS NULL",
    )
    .bind(id)
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

    // Check if resource requires premium access
    let access_level = resource.access_level.as_deref().unwrap_or("premium");
    if access_level != "free" {
        // Verify token for premium resources
        if let Some(token) = params.get("token") {
            let is_valid: bool =
                sqlx::query_scalar("SELECT validate_secure_download_token($1, $2)")
                    .bind(id)
                    .bind(token)
                    .fetch_one(&state.db.pool)
                    .await
                    .unwrap_or(false);

            if !is_valid {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Invalid or expired download token"})),
                ));
            }
        }
        // Note: In production, you'd also verify user session/membership here
    }

    // Track download
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "success": true,
        "file_url": resource.file_url,
        "filename": format!("{}.{}", resource.slug, resource.mime_type.as_deref().unwrap_or("bin").split('/').next_back().unwrap_or("bin")),
        "file_size": resource.file_size,
        "mime_type": resource.mime_type
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: VERSION HISTORY
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources/:id/versions - Get version history for a resource
async fn get_version_history(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get all versions of this resource (including current and previous)
    let versions: Vec<RoomResource> = sqlx::query_as(
        r#"
        WITH RECURSIVE version_chain AS (
            SELECT * FROM room_resources WHERE id = $1 AND deleted_at IS NULL
            UNION ALL
            SELECT r.* FROM room_resources r
            INNER JOIN version_chain vc ON r.id = vc.previous_version_id
            WHERE r.deleted_at IS NULL
        )
        SELECT * FROM version_chain ORDER BY version DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let responses: Vec<ResourceResponse> = versions.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total_versions": responses.len()
    })))
}

/// POST /api/admin/room-resources/:id/new-version - Create a new version of resource
async fn create_new_version(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateVersionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let new_id: i64 = sqlx::query_scalar("SELECT create_resource_version($1, $2, $3)")
        .bind(id)
        .bind(&input.file_url)
        .bind(input.file_size)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Get the new resource
    let resource: RoomResource = sqlx::query_as("SELECT * FROM room_resources WHERE id = $1")
        .bind(new_id)
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
        "message": "New version created successfully",
        "data": resource_to_response(resource)
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateVersionRequest {
    pub file_url: String,
    pub file_size: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: BULK OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct BulkCreateRequest {
    pub resources: Vec<CreateResourceRequest>,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequest {
    pub ids: Vec<i64>,
    pub updates: BulkUpdateFields,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateFields {
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_pinned: Option<bool>,
    pub access_level: Option<String>,
    pub category: Option<String>,
    pub section: Option<String>,
}

/// POST /api/admin/room-resources/bulk-create - Create multiple resources at once
async fn bulk_create_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkCreateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut created_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for resource_input in input.resources {
        let slug = slugify(&resource_input.title);
        let resource_date = resource_input
            .resource_date
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Utc::now().date_naive());

        let tags_json = resource_input
            .tags
            .as_ref()
            .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
            .unwrap_or(json!([]));

        let result = sqlx::query(
            r#"
            INSERT INTO room_resources (
                title, slug, description, resource_type, content_type,
                file_url, mime_type, file_size, video_platform, bunny_video_guid,
                bunny_library_id, duration, thumbnail_url, width, height,
                trading_room_id, trader_id, resource_date, is_published, is_featured,
                is_pinned, section, category, tags, difficulty_level, access_level,
                course_id, lesson_id, course_order, storage_provider, file_hash
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31
            )
            "#,
        )
        .bind(&resource_input.title)
        .bind(&slug)
        .bind(&resource_input.description)
        .bind(&resource_input.resource_type)
        .bind(&resource_input.content_type)
        .bind(&resource_input.file_url)
        .bind(&resource_input.mime_type)
        .bind(resource_input.file_size)
        .bind(&resource_input.video_platform)
        .bind(&resource_input.bunny_video_guid)
        .bind(resource_input.bunny_library_id)
        .bind(resource_input.duration)
        .bind(&resource_input.thumbnail_url)
        .bind(resource_input.width)
        .bind(resource_input.height)
        .bind(resource_input.trading_room_id)
        .bind(resource_input.trader_id)
        .bind(resource_date)
        .bind(resource_input.is_published.unwrap_or(false))
        .bind(resource_input.is_featured.unwrap_or(false))
        .bind(resource_input.is_pinned.unwrap_or(false))
        .bind(&resource_input.section)
        .bind(&resource_input.category)
        .bind(tags_json)
        .bind(&resource_input.difficulty_level)
        .bind(resource_input.access_level.as_deref().unwrap_or("premium"))
        .bind(resource_input.course_id)
        .bind(resource_input.lesson_id)
        .bind(resource_input.course_order)
        .bind(resource_input.storage_provider.as_deref().unwrap_or("r2"))
        .bind(&resource_input.file_hash)
        .execute(&state.db.pool)
        .await;

        match result {
            Ok(_) => created_count += 1,
            Err(e) => errors.push(format!(
                "Failed to create '{}': {}",
                resource_input.title, e
            )),
        }
    }

    Ok(Json(json!({
        "success": errors.is_empty(),
        "created_count": created_count,
        "errors": errors,
        "message": format!("Created {} resources", created_count)
    })))
}

/// PUT /api/admin/room-resources/bulk-update - Update multiple resources at once
async fn bulk_update_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkUpdateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.updates.is_published.is_some() {
        updates.push(format!("is_published = ${}", param_idx));
        param_idx += 1;
    }
    if input.updates.is_featured.is_some() {
        updates.push(format!("is_featured = ${}", param_idx));
        param_idx += 1;
    }
    if input.updates.is_pinned.is_some() {
        updates.push(format!("is_pinned = ${}", param_idx));
        param_idx += 1;
    }
    if input.updates.access_level.is_some() {
        updates.push(format!("access_level = ${}", param_idx));
        param_idx += 1;
    }
    if input.updates.category.is_some() {
        updates.push(format!("category = ${}", param_idx));
        param_idx += 1;
    }
    if input.updates.section.is_some() {
        updates.push(format!("section = ${}", param_idx));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let ids_placeholder: String = input
        .ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", param_idx + i))
        .collect::<Vec<_>>()
        .join(",");

    let query_str = format!(
        "UPDATE room_resources SET {}, updated_at = NOW() WHERE id IN ({}) AND deleted_at IS NULL",
        updates.join(", "),
        ids_placeholder
    );

    let mut query = sqlx::query(&query_str);

    if let Some(v) = input.updates.is_published {
        query = query.bind(v);
    }
    if let Some(v) = input.updates.is_featured {
        query = query.bind(v);
    }
    if let Some(v) = input.updates.is_pinned {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.access_level {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.category {
        query = query.bind(v);
    }
    if let Some(ref v) = input.updates.section {
        query = query.bind(v);
    }

    for id in &input.ids {
        query = query.bind(*id);
    }

    let result = query.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "updated_count": result.rows_affected(),
        "message": format!("Updated {} resources", result.rows_affected())
    })))
}

/// DELETE /api/admin/room-resources/bulk-delete - Delete multiple resources at once
async fn bulk_delete_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(ids): Json<Vec<i64>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No IDs provided"})),
        ));
    }

    let placeholders: String = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect::<Vec<_>>()
        .join(",");

    let query_str = format!(
        "UPDATE room_resources SET deleted_at = NOW() WHERE id IN ({}) AND deleted_at IS NULL",
        placeholders
    );

    let mut query = sqlx::query(&query_str);
    for id in &ids {
        query = query.bind(*id);
    }

    let result = query.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "deleted_count": result.rows_affected(),
        "message": format!("Deleted {} resources", result.rows_affected())
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: FILE UPLOAD LIMITS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UploadLimit {
    pub resource_type: String,
    pub max_file_size_bytes: i64,
    pub allowed_mime_types: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub requires_premium: bool,
}

/// GET /api/admin/room-resources/upload-limits - Get file upload limits
async fn get_upload_limits(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limits: Vec<UploadLimit> = sqlx::query_as(
        "SELECT resource_type, max_file_size_bytes, allowed_mime_types, allowed_extensions, requires_premium FROM resource_upload_limits ORDER BY resource_type"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": limits
    })))
}

/// POST /api/admin/room-resources/validate-upload - Validate file before upload
async fn validate_upload(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<ValidateUploadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get limits for this resource type
    let limit: Option<UploadLimit> = sqlx::query_as(
        "SELECT resource_type, max_file_size_bytes, allowed_mime_types, allowed_extensions, requires_premium FROM resource_upload_limits WHERE resource_type = $1"
    )
    .bind(&input.resource_type)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let limit = match limit {
        Some(l) => l,
        None => {
            return Ok(Json(json!({
                "success": true,
                "valid": true,
                "message": "No restrictions for this resource type"
            })));
        }
    };

    // Validate file size
    if input.file_size > limit.max_file_size_bytes {
        let max_mb = limit.max_file_size_bytes / 1_048_576;
        return Ok(Json(json!({
            "success": true,
            "valid": false,
            "error": format!("File size exceeds maximum of {} MB", max_mb)
        })));
    }

    // Validate MIME type
    if let Some(ref mime) = input.mime_type {
        if !limit.allowed_mime_types.contains(mime) {
            return Ok(Json(json!({
                "success": true,
                "valid": false,
                "error": format!("MIME type '{}' is not allowed. Allowed: {:?}", mime, limit.allowed_mime_types)
            })));
        }
    }

    // Validate extension
    if let Some(ref ext) = input.extension {
        let ext_lower = ext.to_lowercase();
        if !limit.allowed_extensions.contains(&ext_lower) {
            return Ok(Json(json!({
                "success": true,
                "valid": false,
                "error": format!("Extension '.{}' is not allowed. Allowed: {:?}", ext, limit.allowed_extensions)
            })));
        }
    }

    Ok(Json(json!({
        "success": true,
        "valid": true,
        "message": "File is valid for upload",
        "requires_premium": limit.requires_premium
    })))
}

#[derive(Debug, Deserialize)]
pub struct ValidateUploadRequest {
    pub resource_type: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: COURSE RESOURCES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources/by-course/:course_id - Get resources for a course
async fn get_course_resources(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resources: Vec<RoomResource> = sqlx::query_as(
        "SELECT * FROM room_resources WHERE course_id = $1 AND is_published = true AND deleted_at IS NULL ORDER BY course_order ASC, created_at ASC"
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total": responses.len()
    })))
}

/// GET /api/room-resources/by-lesson/:lesson_id - Get resources for a lesson
async fn get_lesson_resources(
    State(state): State<AppState>,
    Path(lesson_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resources: Vec<RoomResource> = sqlx::query_as(
        "SELECT * FROM room_resources WHERE lesson_id = $1 AND is_published = true AND deleted_at IS NULL ORDER BY course_order ASC, created_at ASC"
    )
    .bind(lesson_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total": responses.len()
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: ETF/STOCK LISTS MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockList {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub list_type: String, // etf, stock, watchlist, sector
    pub trading_room_id: i64,
    pub symbols: serde_json::Value, // JSON array of {symbol, name, sector, notes}
    pub is_active: bool,
    pub is_featured: bool,
    pub sort_order: i32,
    pub week_of: Option<chrono::NaiveDate>,
    pub created_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateStockListRequest {
    pub name: String,
    pub description: Option<String>,
    pub list_type: String,
    pub trading_room_id: i64,
    pub symbols: Vec<StockSymbol>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub week_of: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockSymbol {
    pub symbol: String,
    pub name: Option<String>,
    pub sector: Option<String>,
    pub notes: Option<String>,
    pub price_target: Option<f64>,
    pub entry_price: Option<f64>,
    pub stop_loss: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StockListQuery {
    pub room_id: Option<i64>,
    pub list_type: Option<String>,
    pub is_active: Option<bool>,
    pub week_of: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// GET /api/room-resources/stock-lists - List stock/ETF lists
async fn list_stock_lists(
    State(state): State<AppState>,
    Query(query): Query<StockListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.room_id.is_some() {
        conditions.push(format!("trading_room_id = ${}", param_idx));
        param_idx += 1;
    }
    if query.list_type.is_some() {
        conditions.push(format!("list_type = ${}", param_idx));
        param_idx += 1;
    }
    if query.is_active.is_some() {
        conditions.push(format!("is_active = ${}", param_idx));
        param_idx += 1;
    }
    if query.week_of.is_some() {
        conditions.push(format!("week_of = ${}", param_idx));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT * FROM stock_lists WHERE 1=1{} ORDER BY is_featured DESC, week_of DESC NULLS LAST, created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    let count_sql = format!("SELECT COUNT(*) FROM stock_lists WHERE 1=1{}", where_clause);

    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, StockList>(&sql);
    if let Some(room_id) = query.room_id {
        q = q.bind(room_id);
    }
    if let Some(ref list_type) = query.list_type {
        q = q.bind(list_type);
    }
    if let Some(is_active) = query.is_active {
        q = q.bind(is_active);
    }
    if let Some(ref week_of) = query.week_of {
        let week_date = NaiveDate::parse_from_str(week_of, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        q = q.bind(week_date);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    let lists: Vec<StockList> = q.fetch_all(&state.db.pool).await.unwrap_or_default();

    // Bind parameters for the count query
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(room_id) = query.room_id {
        cq = cq.bind(room_id);
    }
    if let Some(ref list_type) = query.list_type {
        cq = cq.bind(list_type);
    }
    if let Some(is_active) = query.is_active {
        cq = cq.bind(is_active);
    }
    if let Some(ref week_of) = query.week_of {
        let week_date = NaiveDate::parse_from_str(week_of, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        cq = cq.bind(week_date);
    }

    let total: (i64,) = cq.fetch_one(&state.db.pool).await.unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": lists,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": ((total.0 as f64) / (per_page as f64)).ceil() as i64
        }
    })))
}

/// GET /api/room-resources/stock-lists/:id - Get single stock list
async fn get_stock_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: StockList = sqlx::query_as("SELECT * FROM stock_lists WHERE id = $1")
        .bind(id)
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
                Json(json!({"error": "Stock list not found"})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "data": list
    })))
}

/// POST /api/admin/room-resources/stock-lists - Create stock list
async fn create_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateStockListRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.name);
    let symbols_json = serde_json::to_value(&input.symbols).unwrap_or(json!([]));
    let week_of = input
        .week_of
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let list: StockList = sqlx::query_as(
        r#"
        INSERT INTO stock_lists (name, slug, description, list_type, trading_room_id, symbols, is_active, is_featured, week_of)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.list_type)
    .bind(input.trading_room_id)
    .bind(symbols_json)
    .bind(input.is_active.unwrap_or(true))
    .bind(input.is_featured.unwrap_or(false))
    .bind(week_of)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "message": "Stock list created successfully",
        "data": list
    })))
}

/// PUT /api/admin/room-resources/stock-lists/:id - Update stock list
async fn update_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateStockListRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let symbols_json = serde_json::to_value(&input.symbols).unwrap_or(json!([]));
    let week_of = input
        .week_of
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let list: StockList = sqlx::query_as(
        r#"
        UPDATE stock_lists SET
            name = $1, description = $2, list_type = $3, symbols = $4,
            is_active = $5, is_featured = $6, week_of = $7, updated_at = NOW()
        WHERE id = $8
        RETURNING *
        "#,
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.list_type)
    .bind(symbols_json)
    .bind(input.is_active.unwrap_or(true))
    .bind(input.is_featured.unwrap_or(false))
    .bind(week_of)
    .bind(id)
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
            Json(json!({"error": "Stock list not found"})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Stock list updated successfully",
        "data": list
    })))
}

/// DELETE /api/admin/room-resources/stock-lists/:id - Delete stock list
async fn delete_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM stock_lists WHERE id = $1")
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
            Json(json!({"error": "Stock list not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Stock list deleted successfully"
    })))
}

/// GET /api/room-resources/stock-lists/latest/:room_id - Get latest watchlist for room
async fn get_latest_watchlist(
    State(state): State<AppState>,
    Path(room_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: Option<StockList> = sqlx::query_as(
        "SELECT * FROM stock_lists WHERE trading_room_id = $1 AND list_type = 'watchlist' AND is_active = true ORDER BY week_of DESC NULLS LAST, created_at DESC LIMIT 1"
    )
    .bind(room_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "data": list
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RECENTLY ACCESSED TRACKING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct RecentlyAccessed {
    pub id: i64,
    pub user_id: i64,
    pub resource_id: i64,
    pub resource_type: String,
    pub resource_title: String,
    pub resource_thumbnail: Option<String>,
    pub accessed_at: chrono::NaiveDateTime,
}

/// POST /api/room-resources/:id/track-access - Track resource access (requires auth)
async fn track_resource_access(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Extract user ID from auth header/session (simplified - in production use proper auth)
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    if let Some(uid) = user_id {
        // Get resource info
        let resource: Option<(String, String, Option<String>)> = sqlx::query_as(
            "SELECT resource_type, title, thumbnail_url FROM room_resources WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .unwrap_or(None);

        if let Some((resource_type, title, thumbnail)) = resource {
            // Upsert into recently_accessed
            let _ = sqlx::query(
                r#"
                INSERT INTO resource_access_log (user_id, resource_id, resource_type, resource_title, resource_thumbnail)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (user_id, resource_id) DO UPDATE SET accessed_at = NOW()
                "#
            )
            .bind(uid)
            .bind(id)
            .bind(&resource_type)
            .bind(&title)
            .bind(&thumbnail)
            .execute(&state.db.pool)
            .await;

            // Increment view count
            let _ = sqlx::query(
                "UPDATE room_resources SET views_count = views_count + 1 WHERE id = $1",
            )
            .bind(id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({"success": true})))
}

/// GET /api/room-resources/recently-accessed - Get user's recently accessed resources
async fn get_recently_accessed(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let limit: i64 = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
        .min(50);

    if let Some(uid) = user_id {
        let recent: Vec<RecentlyAccessed> = sqlx::query_as(
            "SELECT * FROM resource_access_log WHERE user_id = $1 ORDER BY accessed_at DESC LIMIT $2"
        )
        .bind(uid)
        .bind(limit)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        Ok(Json(json!({
            "success": true,
            "data": recent
        })))
    } else {
        Ok(Json(json!({
            "success": true,
            "data": []
        })))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE FAVORITES
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/room-resources/:id/favorite - Add to favorites
async fn add_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let result = sqlx::query(
        "INSERT INTO resource_favorites (user_id, resource_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "added": result.rows_affected() > 0,
        "message": if result.rows_affected() > 0 { "Added to favorites" } else { "Already in favorites" }
    })))
}

/// DELETE /api/room-resources/:id/favorite - Remove from favorites
async fn remove_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let result =
        sqlx::query("DELETE FROM resource_favorites WHERE user_id = $1 AND resource_id = $2")
            .bind(user_id)
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
        "removed": result.rows_affected() > 0,
        "message": if result.rows_affected() > 0 { "Removed from favorites" } else { "Not in favorites" }
    })))
}

/// GET /api/room-resources/:id/favorite - Check if favorited
async fn check_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    if let Some(uid) = user_id {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM resource_favorites WHERE user_id = $1 AND resource_id = $2)"
        )
        .bind(uid)
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(false);

        Ok(Json(json!({
            "success": true,
            "is_favorited": exists
        })))
    } else {
        Ok(Json(json!({
            "success": true,
            "is_favorited": false
        })))
    }
}

/// GET /api/room-resources/favorites - Get user's favorite resources
async fn get_favorite_resources(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let page: i64 = params
        .get("page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1);
    let per_page: i64 = params
        .get("per_page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(20)
        .min(50);
    let offset = (page - 1) * per_page;

    let resources: Vec<RoomResource> = sqlx::query_as(
        r#"
        SELECT r.* FROM room_resources r
        INNER JOIN resource_favorites f ON r.id = f.resource_id
        WHERE f.user_id = $1 AND r.deleted_at IS NULL
        ORDER BY f.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM resource_favorites WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": ((total.0 as f64) / (per_page as f64)).ceil() as i64
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct ResourceAnalytics {
    pub total_resources: i64,
    pub total_views: i64,
    pub total_downloads: i64,
    pub total_favorites: i64,
    pub by_type: Vec<TypeStats>,
    pub by_access_level: Vec<AccessStats>,
    pub top_viewed: Vec<ResourceStats>,
    pub top_downloaded: Vec<ResourceStats>,
    pub recent_uploads: Vec<ResourceStats>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TypeStats {
    pub resource_type: String,
    pub count: i64,
    pub total_views: i64,
    pub total_downloads: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccessStats {
    pub access_level: String,
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ResourceStats {
    pub id: i64,
    pub title: String,
    pub resource_type: String,
    pub views_count: i32,
    pub downloads_count: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// GET /api/admin/room-resources/analytics - Get resource analytics
async fn get_resource_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let room_id: Option<i64> = params.get("room_id").and_then(|s| s.parse().ok());

    // Use parameterized queries to prevent SQL injection
    let room_filter = if room_id.is_some() {
        " AND trading_room_id = $1"
    } else {
        ""
    };

    // Total counts
    let totals_sql = format!(
        "SELECT COUNT(*), COALESCE(SUM(views_count), 0), COALESCE(SUM(downloads_count), 0) FROM room_resources WHERE deleted_at IS NULL{}",
        room_filter
    );
    let mut totals_q = sqlx::query_as::<_, (i64, i64, i64)>(&totals_sql);
    if let Some(rid) = room_id {
        totals_q = totals_q.bind(rid);
    }
    let totals: (i64, i64, i64) = totals_q
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0, 0, 0));

    let total_favorites: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM resource_favorites")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // By type
    let by_type_sql = format!(
        "SELECT resource_type, COUNT(*) as count, COALESCE(SUM(views_count), 0) as total_views, COALESCE(SUM(downloads_count), 0) as total_downloads FROM room_resources WHERE deleted_at IS NULL{} GROUP BY resource_type ORDER BY count DESC",
        room_filter
    );
    let mut by_type_q = sqlx::query_as::<_, TypeStats>(&by_type_sql);
    if let Some(rid) = room_id {
        by_type_q = by_type_q.bind(rid);
    }
    let by_type: Vec<TypeStats> = by_type_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // By access level
    let by_access_sql = format!(
        "SELECT COALESCE(access_level, 'premium') as access_level, COUNT(*) as count FROM room_resources WHERE deleted_at IS NULL{} GROUP BY access_level ORDER BY count DESC",
        room_filter
    );
    let mut by_access_q = sqlx::query_as::<_, AccessStats>(&by_access_sql);
    if let Some(rid) = room_id {
        by_access_q = by_access_q.bind(rid);
    }
    let by_access_level: Vec<AccessStats> = by_access_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Top viewed
    let top_viewed_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{} ORDER BY views_count DESC LIMIT 10",
        room_filter
    );
    let mut top_viewed_q = sqlx::query_as::<_, ResourceStats>(&top_viewed_sql);
    if let Some(rid) = room_id {
        top_viewed_q = top_viewed_q.bind(rid);
    }
    let top_viewed: Vec<ResourceStats> = top_viewed_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Top downloaded
    let top_downloaded_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{} ORDER BY downloads_count DESC LIMIT 10",
        room_filter
    );
    let mut top_downloaded_q = sqlx::query_as::<_, ResourceStats>(&top_downloaded_sql);
    if let Some(rid) = room_id {
        top_downloaded_q = top_downloaded_q.bind(rid);
    }
    let top_downloaded: Vec<ResourceStats> = top_downloaded_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Recent uploads
    let recent_uploads_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{} ORDER BY created_at DESC LIMIT 10",
        room_filter
    );
    let mut recent_uploads_q = sqlx::query_as::<_, ResourceStats>(&recent_uploads_sql);
    if let Some(rid) = room_id {
        recent_uploads_q = recent_uploads_q.bind(rid);
    }
    let recent_uploads: Vec<ResourceStats> = recent_uploads_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    let analytics = ResourceAnalytics {
        total_resources: totals.0,
        total_views: totals.1,
        total_downloads: totals.2,
        total_favorites: total_favorites.0,
        by_type,
        by_access_level,
        top_viewed,
        top_downloaded,
        recent_uploads,
    };

    Ok(Json(json!({
        "success": true,
        "data": analytics
    })))
}

/// GET /api/admin/room-resources/download-logs - Get download logs
async fn get_download_logs(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resource_id: Option<i64> = params.get("resource_id").and_then(|s| s.parse().ok());
    let page: i64 = params
        .get("page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1);
    let per_page: i64 = params
        .get("per_page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(50)
        .min(100);
    let offset = (page - 1) * per_page;

    let (logs, total): (Vec<serde_json::Value>, (i64,)) = if let Some(rid) = resource_id {
        #[allow(clippy::type_complexity)]
        let logs: Vec<(i64, i64, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, resource_id, user_id, ip_address::TEXT, downloaded_at FROM resource_download_logs WHERE resource_id = $1 ORDER BY downloaded_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(rid)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM resource_download_logs WHERE resource_id = $1")
                .bind(rid)
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((0,));

        let json_logs: Vec<serde_json::Value> = logs.into_iter().map(|(id, rid, uid, ip, ts)| {
            json!({"id": id, "resource_id": rid, "user_id": uid, "ip_address": ip, "downloaded_at": ts})
        }).collect();

        (json_logs, total)
    } else {
        #[allow(clippy::type_complexity)]
        let logs: Vec<(i64, i64, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, resource_id, user_id, ip_address::TEXT, downloaded_at FROM resource_download_logs ORDER BY downloaded_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM resource_download_logs")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

        let json_logs: Vec<serde_json::Value> = logs.into_iter().map(|(id, rid, uid, ip, ts)| {
            json!({"id": id, "resource_id": rid, "user_id": uid, "ip_address": ip, "downloaded_at": ts})
        }).collect();

        (json_logs, total)
    };

    Ok(Json(json!({
        "success": true,
        "data": logs,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_resources))
        .route("/:id_or_slug", get(get_resource))
        .route("/:id/download", get(download_resource))
        .route("/:id/download", post(track_download))
        .route("/:id/secure-download", post(generate_secure_download))
        .route("/:id/versions", get(get_version_history))
        .route("/by-course/:course_id", get(get_course_resources))
        .route("/by-lesson/:lesson_id", get(get_lesson_resources))
        // ICT 7 NEW: User engagement
        .route("/:id/track-access", post(track_resource_access))
        .route("/recently-accessed", get(get_recently_accessed))
        .route("/:id/favorite", get(check_resource_favorite))
        .route("/:id/favorite", post(add_resource_favorite))
        .route("/:id/favorite", delete(remove_resource_favorite))
        .route("/favorites", get(get_favorite_resources))
        // ICT 7 NEW: Stock/ETF lists
        .route("/stock-lists", get(list_stock_lists))
        .route("/stock-lists/:id", get(get_stock_list))
        .route("/stock-lists/latest/:room_id", get(get_latest_watchlist))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_resources))
        .route("/", post(create_resource))
        .route("/:id", put(update_resource))
        .route("/:id", delete(delete_resource))
        .route("/:id/new-version", post(create_new_version))
        .route("/bulk-create", post(bulk_create_resources))
        .route("/bulk-update", put(bulk_update_resources))
        .route("/bulk-delete", delete(bulk_delete_resources))
        .route("/upload-limits", get(get_upload_limits))
        .route("/validate-upload", post(validate_upload))
        // ICT 7 NEW: Analytics
        .route("/analytics", get(get_resource_analytics))
        .route("/download-logs", get(get_download_logs))
        // ICT 7 NEW: Stock/ETF lists admin
        .route("/stock-lists", post(create_stock_list))
        .route("/stock-lists/:id", put(update_stock_list))
        .route("/stock-lists/:id", delete(delete_stock_list))
}
