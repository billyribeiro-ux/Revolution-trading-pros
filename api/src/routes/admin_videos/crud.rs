//! Core CRUD handlers for unified videos plus admin bulk operations
//! wired into the main `/admin/videos` router.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::models::video::{
    get_all_tags, get_content_types, get_difficulty_levels, get_platforms, BulkAssignRequest,
    BulkDeleteRequest, BulkPublishRequest, CreateVideoRequest, PaginationMeta, RoomInfo,
    TraderInfo, UpdateVideoRequest, VideoListQuery, VideoStats, VideoStatsResponse, VideoTypeStats,
};
use crate::AppState;

use super::helpers::{slugify, video_to_response};
use super::UnifiedVideoRow;

/// List videos with filtering and pagination
pub(super) async fn list_videos(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.content_type.is_some() {
        conditions.push(format!("content_type = ${param_idx}"));
        param_idx += 1;
    }
    if query.is_published.is_some() {
        conditions.push(format!("is_published = ${param_idx}"));
        param_idx += 1;
    }
    if query.is_featured.is_some() {
        conditions.push(format!("is_featured = ${param_idx}"));
        param_idx += 1;
    }
    if query.trader_id.is_some() {
        conditions.push(format!("trader_id = ${param_idx}"));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(title ILIKE ${param_idx} OR description ILIKE ${param_idx})"
        ));
        param_idx += 1;
    }
    // Tags: each tag gets its own JSONB containment check
    let tag_list: Vec<String> = query
        .tags
        .as_ref()
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    for _ in &tag_list {
        conditions.push(format!("tags @> ${param_idx}"));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    // Whitelist validate sort_by and sort_dir to prevent ORDER BY injection
    let valid_sort_columns = [
        "video_date",
        "title",
        "created_at",
        "updated_at",
        "views_count",
        "id",
    ];
    let sort_by = if valid_sort_columns.contains(&query.sort_by.as_deref().unwrap_or("video_date"))
    {
        query.sort_by.as_deref().unwrap_or("video_date")
    } else {
        "video_date"
    };
    let sort_dir = match query.sort_dir.as_deref() {
        Some("ASC") | Some("asc") => "ASC",
        _ => "DESC",
    };

    let sql = format!(
        "SELECT * FROM unified_videos WHERE deleted_at IS NULL{} ORDER BY {} {} LIMIT ${} OFFSET ${}",
        where_clause, sort_by, sort_dir, param_idx, param_idx + 1
    );
    let count_sql =
        format!("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL{where_clause}");

    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, UnifiedVideoRow>(sqlx::AssertSqlSafe(sql.as_str()));
    if let Some(ref content_type) = query.content_type {
        q = q.bind(content_type);
    }
    if let Some(is_published) = query.is_published {
        q = q.bind(is_published);
    }
    if let Some(is_featured) = query.is_featured {
        q = q.bind(is_featured);
    }
    if let Some(trader_id) = query.trader_id {
        q = q.bind(trader_id);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        q = q.bind(search_pattern);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        q = q.bind(tag_json);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    let videos: Vec<UnifiedVideoRow> = q.fetch_all(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Bind parameters for the count query (same filters, no LIMIT/OFFSET)
    let mut cq = sqlx::query_as::<_, (i64,)>(sqlx::AssertSqlSafe(count_sql.as_str()));
    if let Some(ref content_type) = query.content_type {
        cq = cq.bind(content_type);
    }
    if let Some(is_published) = query.is_published {
        cq = cq.bind(is_published);
    }
    if let Some(is_featured) = query.is_featured {
        cq = cq.bind(is_featured);
    }
    if let Some(trader_id) = query.trader_id {
        cq = cq.bind(trader_id);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        cq = cq.bind(search_pattern);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        cq = cq.bind(tag_json);
    }

    let total: (i64,) = cq.fetch_one(&state.db.pool).await.unwrap_or((0,));

    // Fetch related data for each video
    let mut responses = Vec::new();
    for video in videos {
        let trader: Option<TraderInfo> = if let Some(tid) = video.trader_id {
            sqlx::query_as("SELECT id, name, slug FROM traders WHERE id = $1")
                .bind(tid)
                .fetch_optional(&state.db.pool)
                .await
                .unwrap_or(None)
        } else {
            None
        };
        let rooms: Vec<RoomInfo> = sqlx::query_as(
            r"SELECT tr.id, tr.name, tr.slug
               FROM trading_rooms tr
               JOIN video_room_assignments vra ON vra.trading_room_id = tr.id
               WHERE vra.video_id = $1",
        )
        .bind(video.id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();
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
pub(super) async fn get_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: UnifiedVideoRow =
        sqlx::query_as("SELECT * FROM unified_videos WHERE id = $1 AND deleted_at IS NULL")
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

    let trader: Option<TraderInfo> = if let Some(tid) = video.trader_id {
        sqlx::query_as("SELECT id, name, slug FROM traders WHERE id = $1")
            .bind(tid)
            .fetch_optional(&state.db.pool)
            .await
            .unwrap_or(None)
    } else {
        None
    };
    let rooms: Vec<RoomInfo> = sqlx::query_as(
        r"SELECT tr.id, tr.name, tr.slug
           FROM trading_rooms tr
           JOIN video_room_assignments vra ON vra.trading_room_id = tr.id
           WHERE vra.video_id = $1",
    )
    .bind(video.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": video_to_response(video, trader, rooms)
    })))
}

/// Create new video
pub(super) async fn create_video(
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

    let video_date = NaiveDate::parse_from_str(&input.video_date, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid video_date format. Use YYYY-MM-DD"})),
        )
    })?;

    let tags_json = input
        .tags
        .as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
        .unwrap_or(json!([]));

    let video: UnifiedVideoRow = sqlx::query_as(
        r"INSERT INTO unified_videos (
            title, slug, description, video_url, video_platform, content_type,
            video_date, trader_id, is_published, is_featured, tags,
            thumbnail_url, difficulty_level, category, session_type, duration
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING *",
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
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2)",
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
pub(super) async fn update_video(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates = Vec::new();
    let mut bind_count = 1;

    if input.title.is_some() {
        updates.push(format!("title = ${bind_count}"));
        bind_count += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${bind_count}"));
        bind_count += 1;
    }
    if input.video_url.is_some() {
        updates.push(format!("video_url = ${bind_count}"));
        bind_count += 1;
    }
    if input.thumbnail_url.is_some() {
        updates.push(format!("thumbnail_url = ${bind_count}"));
        bind_count += 1;
    }
    if input.is_published.is_some() {
        updates.push(format!("is_published = ${bind_count}"));
        bind_count += 1;
    }
    if input.is_featured.is_some() {
        updates.push(format!("is_featured = ${bind_count}"));
        bind_count += 1;
    }
    if input.tags.is_some() {
        updates.push(format!("tags = ${bind_count}"));
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

    let mut query = sqlx::query_as::<_, UnifiedVideoRow>(sqlx::AssertSqlSafe(sql.as_str()));

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

    let video = query.fetch_one(&state.db.pool).await.map_err(|e| {
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
pub(super) async fn delete_video(
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
pub(super) async fn get_stats(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<VideoStatsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let published: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unified_videos WHERE is_published = true AND deleted_at IS NULL",
    )
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

    let total_views: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(views_count), 0) FROM unified_videos WHERE deleted_at IS NULL",
    )
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
            this_week: {
                let r: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL \
                     AND created_at >= date_trunc('week', NOW())",
                )
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((0,));
                r.0
            },
            this_month: {
                let r: (i64,) = sqlx::query_as(
                    "SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL \
                     AND created_at >= date_trunc('month', NOW())",
                )
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((0,));
                r.0
            },
        },
    }))
}

/// Get video options (tags, platforms, content types, etc.)
pub(super) async fn get_options(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "data": {
            "content_types": get_content_types(),
            "platforms": get_platforms(),
            "difficulty_levels": get_difficulty_levels(),
            "tags": get_all_tags(),
            "trading_rooms": sqlx::query_as::<_, RoomInfo>(
                "SELECT id, name, slug FROM trading_rooms WHERE is_active = true ORDER BY name"
            )
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default(),
            "traders": sqlx::query_as::<_, TraderInfo>(
                "SELECT id, name, slug FROM traders ORDER BY name"
            )
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default()
        }
    })))
}

/// Bulk publish/unpublish videos
/// ICT 7 SECURITY FIX: Use parameterized query with ANY() instead of string interpolation
pub(super) async fn bulk_publish(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkPublishRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.video_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No video IDs provided"})),
        ));
    }

    // ICT 7: Use parameterized query with ANY() array to prevent SQL injection
    sqlx::query(
        "UPDATE unified_videos SET is_published = $1, updated_at = NOW() WHERE id = ANY($2) AND deleted_at IS NULL"
    )
    .bind(input.publish)
    .bind(&input.video_ids)
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
/// ICT 7 SECURITY FIX: Use parameterized query with ANY() instead of string interpolation
pub(super) async fn bulk_delete(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkDeleteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.video_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No video IDs provided"})),
        ));
    }

    // ICT 7: Use parameterized query with ANY() array to prevent SQL injection
    if input.force.unwrap_or(false) {
        sqlx::query("DELETE FROM unified_videos WHERE id = ANY($1)")
            .bind(&input.video_ids)
            .execute(&state.db.pool)
            .await
    } else {
        sqlx::query("UPDATE unified_videos SET deleted_at = NOW() WHERE id = ANY($1)")
            .bind(&input.video_ids)
            .execute(&state.db.pool)
            .await
    }
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
/// ICT 7 SECURITY FIX: Use parameterized query with ANY() instead of string interpolation
pub(super) async fn bulk_assign(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<BulkAssignRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.clear_existing.unwrap_or(false) {
        // ICT 7: Use parameterized query with ANY() array
        let _ = sqlx::query("DELETE FROM video_room_assignments WHERE video_id = ANY($1)")
            .bind(&input.video_ids)
            .execute(&state.db.pool)
            .await;
    }

    for video_id in &input.video_ids {
        for room_id in &input.room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id)
                 VALUES ($1, $2) ON CONFLICT (video_id, trading_room_id) DO NOTHING",
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
