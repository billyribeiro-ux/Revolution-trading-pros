//! Admin Video Management Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full CRUD operations for unified video system with:
//! - Thumbnails & Tags
//! - Multi-platform support (Bunny.net, Vimeo, YouTube, Wistia)
//! - Room assignments
//! - Bulk operations
//! - Content type filtering (learning_center, daily_video, weekly_watchlist, room_archive)

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

use crate::models::video::{
    BulkAssignRequest, BulkDeleteRequest, BulkPublishRequest, CreateVideoRequest,
    PaginationMeta, TagDetail, TraderInfo, RoomInfo, UpdateVideoRequest, VideoListQuery,
    VideoResponse, VideoStats, VideoStatsResponse, VideoTypeStats, get_all_tags,
    get_content_types, get_difficulty_levels, get_platforms,
};
use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedVideoRow {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: String,
    pub video_platform: String,
    pub video_id: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub duration: Option<i32>,
    pub quality: Option<String>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub session_type: Option<String>,
    pub chapter_timestamps: Option<serde_json::Value>,
    pub trader_id: Option<i64>,
    pub video_date: NaiveDate,
    pub is_published: bool,
    pub is_featured: bool,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub scheduled_at: Option<chrono::NaiveDateTime>,
    pub tags: Option<serde_json::Value>,
    pub views_count: i32,
    pub likes_count: i32,
    pub completion_rate: i32,
    pub bunny_library_id: Option<i64>,
    pub bunny_encoding_status: Option<String>,
    pub bunny_thumbnail_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
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

fn get_embed_url(video: &UnifiedVideoRow) -> String {
    match video.video_platform.as_str() {
        "bunny" => {
            if let (Some(guid), Some(lib_id)) = (&video.bunny_video_guid, video.bunny_library_id) {
                format!("https://iframe.mediadelivery.net/embed/{}/{}", lib_id, guid)
            } else {
                video.video_url.clone()
            }
        }
        "vimeo" => {
            if let Some(id) = &video.video_id {
                format!("https://player.vimeo.com/video/{}", id)
            } else {
                video.video_url.clone()
            }
        }
        "youtube" => {
            if let Some(id) = &video.video_id {
                format!("https://www.youtube.com/embed/{}", id)
            } else {
                video.video_url.clone()
            }
        }
        _ => video.video_url.clone(),
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

fn get_tags_vec(tags: &Option<serde_json::Value>) -> Vec<String> {
    tags.as_ref()
        .and_then(|t| serde_json::from_value::<Vec<String>>(t.clone()).ok())
        .unwrap_or_default()
}

fn get_tag_details(tags: &Option<serde_json::Value>) -> Vec<TagDetail> {
    get_tags_vec(tags)
        .iter()
        .filter_map(|slug| {
            get_all_tags().iter().find(|t| &t.slug == slug).cloned()
        })
        .collect()
}

fn video_to_response(video: UnifiedVideoRow, trader: Option<TraderInfo>, rooms: Vec<RoomInfo>) -> VideoResponse {
    VideoResponse {
        id: video.id,
        title: video.title.clone(),
        slug: video.slug.clone(),
        description: video.description.clone(),
        video_url: video.video_url.clone(),
        embed_url: get_embed_url(&video),
        video_platform: video.video_platform.clone(),
        thumbnail_url: video.thumbnail_url.clone(),
        duration: video.duration,
        formatted_duration: format_duration(video.duration),
        content_type: video.content_type.clone(),
        video_date: video.video_date.to_string(),
        formatted_date: video.video_date.format("%B %d, %Y").to_string(),
        is_published: video.is_published,
        is_featured: video.is_featured,
        tags: get_tags_vec(&video.tags),
        tag_details: get_tag_details(&video.tags),
        views_count: video.views_count,
        trader,
        rooms,
        created_at: video.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// List videos with filtering and pagination
async fn list_videos(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM unified_videos WHERE deleted_at IS NULL");
    let mut count_sql = String::from("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL");

    // Apply filters
    if let Some(ref content_type) = query.content_type {
        let filter = format!(" AND content_type = '{}'", content_type.replace('\'', "''"));
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(is_published) = query.is_published {
        let filter = format!(" AND is_published = {}", is_published);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(is_featured) = query.is_featured {
        let filter = format!(" AND is_featured = {}", is_featured);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(trader_id) = query.trader_id {
        let filter = format!(" AND trader_id = {}", trader_id);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref search) = query.search {
        let search_term = search.replace('\'', "''");
        let filter = format!(" AND (title ILIKE '%{}%' OR description ILIKE '%{}%')", search_term, search_term);
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    if let Some(ref tags) = query.tags {
        let tag_list: Vec<&str> = tags.split(',').collect();
        for tag in tag_list {
            let filter = format!(" AND tags @> '[\"{}\"]]'", tag.trim().replace('\'', "''"));
            sql.push_str(&filter);
            count_sql.push_str(&filter);
        }
    }

    // Sorting
    let sort_by = query.sort_by.as_deref().unwrap_or("video_date");
    let sort_dir = query.sort_dir.as_deref().unwrap_or("DESC");
    sql.push_str(&format!(" ORDER BY {} {}", sort_by, sort_dir));
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let videos: Vec<UnifiedVideoRow> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Fetch related data for each video
    let mut responses = Vec::new();
    for video in videos {
        let trader = None; // TODO: Fetch trader if trader_id exists
        let rooms: Vec<RoomInfo> = vec![]; // TODO: Fetch room assignments
        responses.push(video_to_response(video, trader, rooms));
    }

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

/// Get single video by ID
async fn get_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: UnifiedVideoRow = sqlx::query_as(
        "SELECT * FROM unified_videos WHERE id = $1 AND deleted_at IS NULL"
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
            Json(json!({"error": "Video not found"})),
        )
    })?;

    let trader = None; // TODO: Fetch trader
    let rooms: Vec<RoomInfo> = vec![]; // TODO: Fetch rooms

    Ok(Json(json!({
        "success": true,
        "data": video_to_response(video, trader, rooms)
    })))
}

/// Create new video
async fn create_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.title);

    // Check for duplicate slug
    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM unified_videos WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "A video with this slug already exists"})),
        ));
    }

    let video_date = NaiveDate::parse_from_str(&input.video_date, "%Y-%m-%d")
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid video_date format. Use YYYY-MM-DD"})),
            )
        })?;

    let tags_json = input.tags.as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
        .unwrap_or(json!([]));

    let video: UnifiedVideoRow = sqlx::query_as(
        r#"INSERT INTO unified_videos (
            title, slug, description, video_url, video_platform, content_type,
            video_date, trader_id, is_published, is_featured, tags,
            thumbnail_url, difficulty_level, category, session_type, duration
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING *"#,
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.video_url)
    .bind(input.video_platform.as_deref().unwrap_or("bunny"))
    .bind(&input.content_type)
    .bind(video_date)
    .bind(input.trader_id)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_featured.unwrap_or(false))
    .bind(&tags_json)
    .bind(&input.thumbnail_url)
    .bind(&input.difficulty_level)
    .bind(&input.category)
    .bind(&input.session_type)
    .bind(input.duration)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create video: {}", e)})),
        )
    })?;

    // Handle room assignments
    if let Some(room_ids) = input.room_ids {
        for room_id in room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2)"
            )
            .bind(video.id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Video created successfully",
        "data": video_to_response(video, None, vec![])
    })))
}

/// Update video
async fn update_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut bind_count = 1;

    if input.title.is_some() {
        updates.push(format!("title = ${}", bind_count));
        bind_count += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${}", bind_count));
        bind_count += 1;
    }
    if input.video_url.is_some() {
        updates.push(format!("video_url = ${}", bind_count));
        bind_count += 1;
    }
    if input.thumbnail_url.is_some() {
        updates.push(format!("thumbnail_url = ${}", bind_count));
        bind_count += 1;
    }
    if input.is_published.is_some() {
        updates.push(format!("is_published = ${}", bind_count));
        bind_count += 1;
    }
    if input.is_featured.is_some() {
        updates.push(format!("is_featured = ${}", bind_count));
        bind_count += 1;
    }
    if input.tags.is_some() {
        updates.push(format!("tags = ${}", bind_count));
        bind_count += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    updates.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE unified_videos SET {} WHERE id = ${} AND deleted_at IS NULL RETURNING *",
        updates.join(", "),
        bind_count
    );

    let mut query = sqlx::query_as::<_, UnifiedVideoRow>(&sql);

    if let Some(ref title) = input.title {
        query = query.bind(title);
    }
    if let Some(ref description) = input.description {
        query = query.bind(description);
    }
    if let Some(ref video_url) = input.video_url {
        query = query.bind(video_url);
    }
    if let Some(ref thumbnail_url) = input.thumbnail_url {
        query = query.bind(thumbnail_url);
    }
    if let Some(is_published) = input.is_published {
        query = query.bind(is_published);
    }
    if let Some(is_featured) = input.is_featured {
        query = query.bind(is_featured);
    }
    if let Some(ref tags) = input.tags {
        let tags_json = serde_json::to_value(tags).unwrap_or(json!([]));
        query = query.bind(tags_json);
    }

    query = query.bind(id);

    let video = query
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to update video: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Video updated successfully",
        "data": video_to_response(video, None, vec![])
    })))
}

/// Delete video (soft delete)
async fn delete_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE unified_videos SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete video: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Video deleted successfully"
    })))
}

/// Get video statistics
async fn get_stats(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<VideoStatsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let published: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE is_published = true AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let daily_video: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'daily_video' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let weekly_watchlist: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'weekly_watchlist' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let learning_center: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'learning_center' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let room_archive: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'room_archive' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let total_views: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(views_count), 0) FROM unified_videos WHERE deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(VideoStatsResponse {
        success: true,
        data: VideoStats {
            total: total.0,
            published: published.0,
            by_type: VideoTypeStats {
                daily_video: daily_video.0,
                weekly_watchlist: weekly_watchlist.0,
                learning_center: learning_center.0,
                room_archive: room_archive.0,
            },
            total_views: total_views.0,
            this_week: 0, // TODO: Calculate
            this_month: 0, // TODO: Calculate
        },
    }))
}

/// Get video options (tags, platforms, content types, etc.)
async fn get_options(
    _admin: AdminUser,
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "data": {
            "content_types": get_content_types(),
            "platforms": get_platforms(),
            "difficulty_levels": get_difficulty_levels(),
            "tags": get_all_tags(),
            "trading_rooms": [], // TODO: Fetch from database
            "traders": [] // TODO: Fetch from database
        }
    })))
}

/// Bulk publish/unpublish videos
async fn bulk_publish(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkPublishRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ids: Vec<String> = input.video_ids.iter().map(|id| id.to_string()).collect();
    let ids_str = ids.join(",");

    let sql = format!(
        "UPDATE unified_videos SET is_published = $1, updated_at = NOW() WHERE id IN ({}) AND deleted_at IS NULL",
        ids_str
    );

    sqlx::query(&sql)
        .bind(input.publish)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to bulk publish: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos {} successfully", input.video_ids.len(), if input.publish { "published" } else { "unpublished" })
    })))
}

/// Bulk delete videos
async fn bulk_delete(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkDeleteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ids: Vec<String> = input.video_ids.iter().map(|id| id.to_string()).collect();
    let ids_str = ids.join(",");

    let sql = if input.force.unwrap_or(false) {
        format!("DELETE FROM unified_videos WHERE id IN ({})", ids_str)
    } else {
        format!("UPDATE unified_videos SET deleted_at = NOW() WHERE id IN ({})", ids_str)
    };

    sqlx::query(&sql)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to bulk delete: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos deleted successfully", input.video_ids.len())
    })))
}

/// Bulk assign videos to rooms
async fn bulk_assign(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkAssignRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.clear_existing.unwrap_or(false) {
        let ids: Vec<String> = input.video_ids.iter().map(|id| id.to_string()).collect();
        let ids_str = ids.join(",");
        let sql = format!("DELETE FROM video_room_assignments WHERE video_id IN ({})", ids_str);
        let _ = sqlx::query(&sql).execute(&state.db.pool).await;
    }

    for video_id in &input.video_ids {
        for room_id in &input.room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) 
                 VALUES ($1, $2) ON CONFLICT (video_id, trading_room_id) DO NOTHING"
            )
            .bind(video_id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos assigned to {} rooms", input.video_ids.len(), input.room_ids.len())
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_videos).post(create_video))
        .route("/:id", get(get_video).put(update_video).delete(delete_video))
        .route("/stats", get(get_stats))
        .route("/options", get(get_options))
        .route("/bulk/publish", post(bulk_publish))
        .route("/bulk/delete", post(bulk_delete))
        .route("/bulk/assign", post(bulk_assign))
}
