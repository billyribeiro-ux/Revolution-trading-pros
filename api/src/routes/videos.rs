//! Public Video Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Public-facing video endpoints using unified_videos table
//! Supports filtering by content_type, room, tags for learning center

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use sqlx::FromRow;

use crate::models::video::{
    get_all_tags, PaginationMeta, RoomInfo, TagDetail, TraderInfo, VideoResponse,
};
use crate::AppState;

#[derive(Debug, serde::Serialize, FromRow)]
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
    pub duration: Option<i32>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub video_date: NaiveDate,
    pub is_published: bool,
    pub is_featured: bool,
    pub tags: Option<serde_json::Value>,
    pub views_count: i32,
    pub trader_id: Option<i64>,
    pub bunny_library_id: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct VideoListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub content_type: Option<String>,
    pub room_id: Option<i64>,
    pub tags: Option<String>,
    pub difficulty_level: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_type: String,
    pub timestamp_seconds: Option<i32>,
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
        .filter_map(|slug| get_all_tags().iter().find(|t| &t.slug == slug).cloned())
        .collect()
}

fn video_to_response(video: UnifiedVideoRow) -> VideoResponse {
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
        trader: None,
        rooms: vec![],
        created_at: video.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}

/// List videos (public) - supports filtering by content_type, room, tags
async fn list_videos(
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        "SELECT v.* FROM unified_videos v WHERE v.is_published = true AND v.deleted_at IS NULL",
    );
    let mut count_sql = String::from(
        "SELECT COUNT(*) FROM unified_videos v WHERE v.is_published = true AND v.deleted_at IS NULL"
    );

    // Filter by content_type (e.g., learning_center)
    if let Some(ref content_type) = query.content_type {
        let filter = format!(
            " AND v.content_type = '{}'",
            content_type.replace('\'', "''")
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Filter by room
    if let Some(room_id) = query.room_id {
        let filter = format!(
            " AND EXISTS (SELECT 1 FROM video_room_assignments vra WHERE vra.video_id = v.id AND vra.trading_room_id = {})",
            room_id
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Filter by tags
    // ICT 7 FIX: Corrected JSONB containment syntax (was '[\"{}\"]]' - double bracket bug)
    if let Some(ref tags) = query.tags {
        let tag_list: Vec<&str> = tags.split(',').collect();
        for tag in tag_list {
            let filter = format!(" AND v.tags @> '[\"{}\"]'", tag.trim().replace('\'', "''"));
            sql.push_str(&filter);
            count_sql.push_str(&filter);
        }
    }

    // Filter by difficulty
    if let Some(ref difficulty) = query.difficulty_level {
        let filter = format!(
            " AND v.difficulty_level = '{}'",
            difficulty.replace('\'', "''")
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    // Search
    if let Some(ref search) = query.search {
        let search_term = search.replace('\'', "''");
        let filter = format!(
            " AND (v.title ILIKE '%{}%' OR v.description ILIKE '%{}%')",
            search_term, search_term
        );
        sql.push_str(&filter);
        count_sql.push_str(&filter);
    }

    sql.push_str(" ORDER BY v.video_date DESC, v.created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let videos: Vec<UnifiedVideoRow> = sqlx::query_as(&sql)
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

    let responses: Vec<VideoResponse> = videos.into_iter().map(video_to_response).collect();
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

/// Get video by ID or slug
async fn get_video(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: UnifiedVideoRow = if let Ok(id) = id_or_slug.parse::<i64>() {
        sqlx::query_as(
            "SELECT * FROM unified_videos WHERE id = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT * FROM unified_videos WHERE slug = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(&id_or_slug)
        .fetch_optional(&state.db.pool)
        .await
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    // Increment view count
    let _ = sqlx::query("UPDATE unified_videos SET views_count = views_count + 1 WHERE id = $1")
        .bind(video.id)
        .execute(&state.db.pool)
        .await;

    Ok(Json(json!({
        "success": true,
        "data": video_to_response(video)
    })))
}

/// Track video event
async fn track_event(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<TrackEventRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "INSERT INTO analytics_events (event_type, event_name, properties, created_at) VALUES ('video', $1, $2, NOW())"
    )
    .bind(&input.event_type)
    .bind(json!({
        "video_id": id,
        "timestamp_seconds": input.timestamp_seconds
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({"success": true, "status": "ok"})))
}

/// Get related videos based on tags and content type
/// ICT 7 ADDITION: Provides related video suggestions
async fn get_related_videos(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the current video's tags and content_type
    let video: Option<(serde_json::Value, String)> = sqlx::query_as(
        "SELECT COALESCE(tags, '[]'::jsonb), content_type FROM unified_videos WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let (tags, content_type) = match video {
        Some((t, c)) => (t, c),
        None => return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))
    };

    // Find related videos with same content_type or overlapping tags
    let related: Vec<UnifiedVideoRow> = sqlx::query_as(
        r#"SELECT * FROM unified_videos
           WHERE id != $1
             AND is_published = true
             AND deleted_at IS NULL
             AND (content_type = $2 OR tags ?| (SELECT array_agg(value::text) FROM jsonb_array_elements_text($3)))
           ORDER BY
             CASE WHEN content_type = $2 THEN 1 ELSE 2 END,
             video_date DESC
           LIMIT 6"#
    )
    .bind(id)
    .bind(&content_type)
    .bind(&tags)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let responses: Vec<VideoResponse> = related.into_iter().map(video_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses
    })))
}

/// Get weekly watchlist videos (current week)
/// ICT 7 ADDITION: Dedicated weekly videos endpoint
async fn get_weekly_videos(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current week's start and end dates
    let videos: Vec<UnifiedVideoRow> = sqlx::query_as(
        r#"SELECT * FROM unified_videos
           WHERE content_type = 'weekly_watchlist'
             AND is_published = true
             AND deleted_at IS NULL
             AND video_date >= date_trunc('week', CURRENT_DATE)
             AND video_date < date_trunc('week', CURRENT_DATE) + INTERVAL '7 days'
           ORDER BY video_date DESC, created_at DESC"#
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // If no videos for current week, get most recent weekly videos
    let videos = if videos.is_empty() {
        sqlx::query_as(
            r#"SELECT * FROM unified_videos
               WHERE content_type = 'weekly_watchlist'
                 AND is_published = true
                 AND deleted_at IS NULL
               ORDER BY video_date DESC
               LIMIT 10"#
        )
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    } else {
        videos
    };

    let responses: Vec<VideoResponse> = videos.into_iter().map(video_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses
    })))
}

/// Get user's watch history with progress
/// ICT 7 ADDITION: Watch history for resume functionality
async fn get_watch_history(
    State(state): State<AppState>,
    Query(params): Query<WatchHistoryQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = params.user_id.unwrap_or(0);
    let limit = params.limit.unwrap_or(20).min(50);

    let history: Vec<(i64, i32, i32, bool, chrono::NaiveDateTime, String, Option<String>, Option<i32>)> = sqlx::query_as(
        r#"SELECT wp.video_id, wp.current_time_seconds, wp.completion_percent, wp.completed,
                  wp.last_watched_at, v.title, v.thumbnail_url, v.duration
           FROM video_watch_progress wp
           JOIN unified_videos v ON v.id = wp.video_id
           WHERE wp.user_id = $1 AND v.deleted_at IS NULL
           ORDER BY wp.last_watched_at DESC
           LIMIT $2"#
    )
    .bind(user_id)
    .bind(limit as i32)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": history.iter().map(|(video_id, current_time, completion, completed, last_watched, title, thumb, duration)| {
            json!({
                "video_id": video_id,
                "title": title,
                "thumbnail_url": thumb,
                "current_time_seconds": current_time,
                "duration": duration,
                "completion_percent": completion,
                "completed": completed,
                "last_watched_at": last_watched.format("%Y-%m-%dT%H:%M:%S").to_string()
            })
        }).collect::<Vec<_>>()
    })))
}

#[derive(Debug, Deserialize)]
pub struct WatchHistoryQuery {
    pub user_id: Option<i64>,
    pub limit: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_videos))
        .route("/weekly", get(get_weekly_videos))
        .route("/history", get(get_watch_history))
        .route("/:id_or_slug", get(get_video))
        .route("/:id/related", get(get_related_videos))
        .route("/:id/track", post(track_event))
}
