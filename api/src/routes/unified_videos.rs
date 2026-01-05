//! Unified Video Routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Complete CRUD API for video management across all content types:
//! - Daily Videos, Weekly Watchlist, Learning Center, Room Archives
//! - Bunny.net integration for lightning-fast delivery

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use serde_json::json;
use slug::slugify;
use tracing::{error, info};

use crate::models::video::*;
use crate::services::bunny::BunnyClient;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// LIST VIDEOS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_videos(
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build dynamic query
    let mut sql = String::from(
        "SELECT v.*, 
         COALESCE(json_agg(DISTINCT jsonb_build_object('id', tr.id, 'name', tr.name, 'slug', tr.slug)) 
                  FILTER (WHERE tr.id IS NOT NULL), '[]') as rooms_json,
         jsonb_build_object('id', rt.id, 'name', rt.name, 'slug', rt.slug) as trader_json
         FROM unified_videos v
         LEFT JOIN video_room_assignments vra ON v.id = vra.video_id
         LEFT JOIN trading_rooms tr ON vra.trading_room_id = tr.id
         LEFT JOIN room_traders rt ON v.trader_id = rt.id
         WHERE v.deleted_at IS NULL"
    );

    let mut conditions: Vec<String> = Vec::new();

    if let Some(ct) = &query.content_type {
        conditions.push(format!("v.content_type = '{}'", ct));
    }

    if let Some(room_id) = query.room_id {
        conditions.push(format!("vra.trading_room_id = {}", room_id));
    }

    if let Some(trader_id) = query.trader_id {
        conditions.push(format!("v.trader_id = {}", trader_id));
    }

    if let Some(is_pub) = query.is_published {
        conditions.push(format!("v.is_published = {}", is_pub));
    }

    if let Some(is_feat) = query.is_featured {
        conditions.push(format!("v.is_featured = {}", is_feat));
    }

    if let Some(search) = &query.search {
        conditions.push(format!(
            "(v.title ILIKE '%{}%' OR v.description ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    if let Some(tags) = &query.tags {
        let tag_list: Vec<&str> = tags.split(',').collect();
        for tag in tag_list {
            conditions.push(format!("v.tags @> '[\"{}\"]'", tag.trim()));
        }
    }

    for cond in &conditions {
        sql.push_str(&format!(" AND {}", cond));
    }

    sql.push_str(" GROUP BY v.id, rt.id");

    // Sorting
    let sort_by = query.sort_by.as_deref().unwrap_or("video_date");
    let sort_dir = query.sort_dir.as_deref().unwrap_or("desc");
    sql.push_str(&format!(" ORDER BY v.{} {}", sort_by, sort_dir));

    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    // Execute query
    let rows: Vec<(UnifiedVideo, Option<serde_json::Value>, Option<serde_json::Value>)> = 
        sqlx::query_as(&sql)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch videos: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
            })?;

    // Build response
    let videos: Vec<VideoResponse> = rows.iter().map(|(v, rooms_json, trader_json)| {
        let rooms: Vec<RoomInfo> = rooms_json
            .as_ref()
            .and_then(|j| serde_json::from_value(j.clone()).ok())
            .unwrap_or_default();

        let trader: Option<TraderInfo> = trader_json
            .as_ref()
            .and_then(|j| serde_json::from_value(j.clone()).ok());

        VideoResponse {
            id: v.id,
            title: v.title.clone(),
            slug: v.slug.clone(),
            description: v.description.clone(),
            video_url: v.video_url.clone(),
            embed_url: v.get_embed_url(),
            video_platform: v.video_platform.clone(),
            thumbnail_url: v.thumbnail_url.clone().or(v.bunny_thumbnail_url.clone()),
            duration: v.duration,
            formatted_duration: v.get_formatted_duration(),
            content_type: v.content_type.clone(),
            video_date: v.video_date.to_string(),
            formatted_date: v.video_date.format("%B %d, %Y").to_string(),
            is_published: v.is_published,
            is_featured: v.is_featured,
            tags: v.get_tags_vec(),
            tag_details: v.get_tag_details(),
            views_count: v.views_count,
            trader,
            rooms,
            created_at: v.created_at.to_rfc3339(),
        }
    }).collect();

    // Get total count
    let count_sql = "SELECT COUNT(DISTINCT v.id) FROM unified_videos v 
                     LEFT JOIN video_room_assignments vra ON v.id = vra.video_id
                     WHERE v.deleted_at IS NULL";
    let total: (i64,) = sqlx::query_as(count_sql)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let last_page = (total.0 as f64 / per_page as f64).ceil() as i64;

    Ok(Json(json!({
        "success": true,
        "data": videos,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": last_page
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET SINGLE VIDEO
// ═══════════════════════════════════════════════════════════════════════════

async fn get_video(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: UnifiedVideo = sqlx::query_as(
        "SELECT * FROM unified_videos WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    // Get rooms
    let rooms: Vec<RoomInfo> = sqlx::query_as(
        "SELECT tr.id, tr.name, tr.slug FROM trading_rooms tr
         INNER JOIN video_room_assignments vra ON tr.id = vra.trading_room_id
         WHERE vra.video_id = $1"
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get trader
    let trader: Option<TraderInfo> = if let Some(tid) = video.trader_id {
        sqlx::query_as("SELECT id, name, slug FROM room_traders WHERE id = $1")
            .bind(tid)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    let response = VideoResponse {
        id: video.id,
        title: video.title.clone(),
        slug: video.slug.clone(),
        description: video.description.clone(),
        video_url: video.video_url.clone(),
        embed_url: video.get_embed_url(),
        video_platform: video.video_platform.clone(),
        thumbnail_url: video.thumbnail_url.clone().or(video.bunny_thumbnail_url.clone()),
        duration: video.duration,
        formatted_duration: video.get_formatted_duration(),
        content_type: video.content_type.clone(),
        video_date: video.video_date.to_string(),
        formatted_date: video.video_date.format("%B %d, %Y").to_string(),
        is_published: video.is_published,
        is_featured: video.is_featured,
        tags: video.get_tags_vec(),
        tag_details: video.get_tag_details(),
        views_count: video.views_count,
        trader,
        rooms,
        created_at: video.created_at.to_rfc3339(),
    };

    Ok(Json(json!({
        "success": true,
        "data": response
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// CREATE VIDEO
// ═══════════════════════════════════════════════════════════════════════════

async fn create_video(
    State(state): State<AppState>,
    Json(input): Json<CreateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = format!("{}-{}", slugify(&input.title), uuid::Uuid::new_v4().to_string()[..6].to_string());
    let video_date = NaiveDate::parse_from_str(&input.video_date, "%Y-%m-%d")
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid date format"}))))?;
    
    let platform = input.video_platform.as_deref().unwrap_or("bunny");
    let tags_json = input.tags.as_ref().map(|t| serde_json::to_value(t).unwrap());

    // Create video record
    let video: UnifiedVideo = sqlx::query_as(
        r#"INSERT INTO unified_videos 
           (title, slug, description, video_url, video_platform, content_type, video_date,
            trader_id, is_published, is_featured, tags, thumbnail_url, difficulty_level,
            category, session_type, duration, published_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
           RETURNING *"#
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.video_url)
    .bind(platform)
    .bind(&input.content_type)
    .bind(video_date)
    .bind(input.trader_id)
    .bind(input.is_published.unwrap_or(true))
    .bind(input.is_featured.unwrap_or(false))
    .bind(&tags_json)
    .bind(&input.thumbnail_url)
    .bind(&input.difficulty_level)
    .bind(&input.category)
    .bind(&input.session_type)
    .bind(input.duration)
    .bind(if input.is_published.unwrap_or(true) { Some(Utc::now()) } else { None })
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create video: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Assign to rooms
    if input.upload_to_all.unwrap_or(false) {
        // Get all active room IDs
        let rooms: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM trading_rooms WHERE is_active = true"
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        for (room_id,) in rooms {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(video.id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    } else if let Some(room_ids) = &input.room_ids {
        for room_id in room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(video.id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    info!("Created video: {} ({})", video.title, video.id);

    Ok(Json(json!({
        "success": true,
        "message": "Video created successfully",
        "data": {
            "id": video.id,
            "slug": video.slug
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// UPDATE VIDEO
// ═══════════════════════════════════════════════════════════════════════════

async fn update_video(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check video exists
    let _: (i64,) = sqlx::query_as("SELECT id FROM unified_videos WHERE id = $1 AND deleted_at IS NULL")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    // Build update query dynamically
    let mut updates: Vec<String> = Vec::new();
    
    if let Some(t) = &input.title { updates.push(format!("title = '{}'", t.replace('\'', "''"))); }
    if let Some(d) = &input.description { updates.push(format!("description = '{}'", d.replace('\'', "''"))); }
    if let Some(u) = &input.video_url { updates.push(format!("video_url = '{}'", u)); }
    if let Some(p) = &input.video_platform { updates.push(format!("video_platform = '{}'", p)); }
    if let Some(c) = &input.content_type { updates.push(format!("content_type = '{}'", c)); }
    if let Some(d) = &input.video_date { updates.push(format!("video_date = '{}'", d)); }
    if let Some(t) = input.trader_id { updates.push(format!("trader_id = {}", t)); }
    if let Some(p) = input.is_published { updates.push(format!("is_published = {}", p)); }
    if let Some(f) = input.is_featured { updates.push(format!("is_featured = {}", f)); }
    if let Some(t) = &input.thumbnail_url { updates.push(format!("thumbnail_url = '{}'", t)); }
    if let Some(d) = &input.difficulty_level { updates.push(format!("difficulty_level = '{}'", d)); }
    if let Some(c) = &input.category { updates.push(format!("category = '{}'", c)); }
    if let Some(s) = &input.session_type { updates.push(format!("session_type = '{}'", s)); }
    if let Some(d) = input.duration { updates.push(format!("duration = {}", d)); }
    if let Some(tags) = &input.tags {
        let tags_json = serde_json::to_string(tags).unwrap();
        updates.push(format!("tags = '{}'", tags_json));
    }

    if !updates.is_empty() {
        let sql = format!("UPDATE unified_videos SET {} WHERE id = {}", updates.join(", "), id);
        sqlx::query(&sql)
            .execute(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    // Update room assignments if provided
    if input.upload_to_all.unwrap_or(false) {
        sqlx::query("DELETE FROM video_room_assignments WHERE video_id = $1")
            .bind(id)
            .execute(&state.db.pool)
            .await
            .ok();

        let rooms: Vec<(i64,)> = sqlx::query_as("SELECT id FROM trading_rooms WHERE is_active = true")
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

        for (room_id,) in rooms {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    } else if let Some(room_ids) = &input.room_ids {
        sqlx::query("DELETE FROM video_room_assignments WHERE video_id = $1")
            .bind(id)
            .execute(&state.db.pool)
            .await
            .ok();

        for room_id in room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    info!("Updated video: {}", id);

    Ok(Json(json!({
        "success": true,
        "message": "Video updated successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// DELETE VIDEO
// ═══════════════════════════════════════════════════════════════════════════

async fn delete_video(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Soft delete
    sqlx::query("UPDATE unified_videos SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    info!("Deleted video: {}", id);

    Ok(Json(json!({
        "success": true,
        "message": "Video deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// BULK OPERATIONS
// ═══════════════════════════════════════════════════════════════════════════

async fn bulk_assign(
    State(state): State<AppState>,
    Json(input): Json<BulkAssignRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut count = 0;

    for video_id in &input.video_ids {
        if input.clear_existing.unwrap_or(false) {
            sqlx::query("DELETE FROM video_room_assignments WHERE video_id = $1")
                .bind(video_id)
                .execute(&state.db.pool)
                .await
                .ok();
        }

        for room_id in &input.room_ids {
            let result = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(video_id)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;

            if result.is_ok() {
                count += 1;
            }
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": format!("{} assignments created", count),
        "count": count
    })))
}

async fn bulk_publish(
    State(state): State<AppState>,
    Json(input): Json<BulkPublishRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ids: Vec<String> = input.video_ids.iter().map(|id| id.to_string()).collect();
    let ids_str = ids.join(",");

    let sql = format!(
        "UPDATE unified_videos SET is_published = {}, published_at = {} WHERE id IN ({})",
        input.publish,
        if input.publish { "NOW()" } else { "NULL" },
        ids_str
    );

    let result = sqlx::query(&sql)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let action = if input.publish { "published" } else { "unpublished" };

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos {}", result.rows_affected(), action),
        "count": result.rows_affected()
    })))
}

async fn bulk_delete(
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

    let result = sqlx::query(&sql)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos deleted", result.rows_affected()),
        "count": result.rows_affected()
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// STATISTICS & OPTIONS
// ═══════════════════════════════════════════════════════════════════════════

async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let published: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE is_published = true AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let daily: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'daily_video' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let watchlist: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'weekly_watchlist' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let learning: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'learning_center' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let archive: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unified_videos WHERE content_type = 'room_archive' AND deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let views: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(views_count), 0) FROM unified_videos WHERE deleted_at IS NULL")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": {
            "total": total.0,
            "published": published.0,
            "by_type": {
                "daily_video": daily.0,
                "weekly_watchlist": watchlist.0,
                "learning_center": learning.0,
                "room_archive": archive.0
            },
            "total_views": views.0
        }
    })))
}

async fn get_options(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get trading rooms
    let rooms: Vec<RoomInfo> = sqlx::query_as(
        "SELECT id, name, slug FROM trading_rooms WHERE is_active = true ORDER BY sort_order"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get traders
    let traders: Vec<TraderInfo> = sqlx::query_as(
        "SELECT id, name, slug FROM room_traders ORDER BY name"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "content_types": get_content_types(),
            "platforms": get_platforms(),
            "difficulty_levels": get_difficulty_levels(),
            "tags": get_all_tags(),
            "trading_rooms": rooms,
            "traders": traders
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// BUNNY.NET DIRECT UPLOAD
// ═══════════════════════════════════════════════════════════════════════════

async fn create_upload_url(
    State(_state): State<AppState>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();
    
    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured. Set BUNNY_STREAM_API_KEY and BUNNY_STREAM_LIBRARY_ID"
        }))));
    }

    let title = input.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled Video");

    // Create video in Bunny
    let video = bunny.create_video(title).await.map_err(|e| {
        error!("Failed to create Bunny video: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Get upload URL
    let upload_url = bunny.get_upload_url(&video.guid).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "video_guid": video.guid,
            "library_id": video.video_library_id,
            "upload_url": upload_url,
            "embed_url": bunny.get_embed_url(&video.guid),
            "play_url": bunny.get_play_url(&video.guid)
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_videos).post(create_video))
        .route("/options", get(get_options))
        .route("/stats", get(get_stats))
        .route("/upload-url", post(create_upload_url))
        .route("/bulk-assign", post(bulk_assign))
        .route("/bulk-publish", post(bulk_publish))
        .route("/bulk-delete", post(bulk_delete))
        .route("/:id", get(get_video).put(update_video).delete(delete_video))
}
