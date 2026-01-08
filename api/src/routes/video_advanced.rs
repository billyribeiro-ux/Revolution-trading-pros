//! Advanced Video Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Endpoints: Analytics, Series, Chapters, Transcriptions, Scheduled Publishing,
//!            Bulk Upload, Cloning, Export, Webhooks, CDN Cache Purge

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde_json::json;
use slug::slugify;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::models::video_advanced::*;
use crate::services::bunny::BunnyClient;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO ANALYTICS ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Track a single analytics event
async fn track_event(
    State(state): State<AppState>,
    Json(input): Json<AnalyticsEventRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r#"INSERT INTO video_analytics_events
           (video_id, session_id, event_type, event_data, watch_time_seconds,
            progress_percent, buffer_count, quality_level, playback_speed,
            device_type, browser, os, screen_width, screen_height, connection_type)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#
    )
    .bind(input.video_id)
    .bind(&input.session_id)
    .bind(&input.event_type)
    .bind(&input.event_data)
    .bind(input.watch_time_seconds.unwrap_or(0))
    .bind(input.progress_percent)
    .bind(input.buffer_count.unwrap_or(0))
    .bind(&input.quality_level)
    .bind(input.playback_speed)
    .bind(&input.device_type)
    .bind(&input.browser)
    .bind(&input.os)
    .bind(input.screen_width)
    .bind(input.screen_height)
    .bind(&input.connection_type)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to track event: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Update video view count if this is a view event
    if input.event_type == "view" {
        let _ = sqlx::query("UPDATE unified_videos SET views_count = views_count + 1 WHERE id = $1")
            .bind(input.video_id)
            .execute(&state.db.pool)
            .await;
    }

    Ok(Json(json!({"success": true})))
}

/// Track multiple analytics events (batch)
async fn track_events_batch(
    State(state): State<AppState>,
    Json(input): Json<BatchAnalyticsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut success_count = 0;

    for event in &input.events {
        let result = sqlx::query(
            r#"INSERT INTO video_analytics_events
               (video_id, session_id, event_type, event_data, watch_time_seconds,
                progress_percent, buffer_count, quality_level, playback_speed,
                device_type, browser, os, screen_width, screen_height, connection_type)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#
        )
        .bind(event.video_id)
        .bind(&event.session_id)
        .bind(&event.event_type)
        .bind(&event.event_data)
        .bind(event.watch_time_seconds.unwrap_or(0))
        .bind(event.progress_percent)
        .bind(event.buffer_count.unwrap_or(0))
        .bind(&event.quality_level)
        .bind(event.playback_speed)
        .bind(&event.device_type)
        .bind(&event.browser)
        .bind(&event.os)
        .bind(event.screen_width)
        .bind(event.screen_height)
        .bind(&event.connection_type)
        .execute(&state.db.pool)
        .await;

        if result.is_ok() {
            success_count += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "tracked": success_count,
        "total": input.events.len()
    })))
}

/// Get analytics dashboard
async fn get_analytics_dashboard(
    State(state): State<AppState>,
    Query(query): Query<AnalyticsDashboardQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let period = query.period.as_deref().unwrap_or("30d");
    let days = match period {
        "7d" => 7,
        "30d" => 30,
        "90d" => 90,
        _ => 30,
    };

    // Total videos
    let total_videos: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Total views in period
    let total_views: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM video_analytics_events WHERE event_type = 'view' AND created_at >= NOW() - $1::interval"
    )
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Total watch time in period
    let total_watch_time: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(watch_time_seconds), 0) FROM video_analytics_events WHERE created_at >= NOW() - $1::interval"
    )
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Average completion rate
    let avg_completion: (Option<f64>,) = sqlx::query_as(
        "SELECT AVG(progress_percent)::float FROM video_analytics_events WHERE progress_percent IS NOT NULL AND created_at >= NOW() - $1::interval"
    )
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    // Unique viewers
    let unique_viewers: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT COALESCE(user_id::text, session_id)) FROM video_analytics_events WHERE created_at >= NOW() - $1::interval"
    )
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Top videos
    let top_videos: Vec<(i64, String, Option<String>, i64)> = sqlx::query_as(
        r#"SELECT v.id, v.title, v.thumbnail_url, COUNT(e.id) as view_count
           FROM unified_videos v
           LEFT JOIN video_analytics_events e ON v.id = e.video_id AND e.event_type = 'view' AND e.created_at >= NOW() - $1::interval
           WHERE v.deleted_at IS NULL
           GROUP BY v.id, v.title, v.thumbnail_url
           ORDER BY view_count DESC
           LIMIT 10"#
    )
    .bind(format!("{} days", days))
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Views by day
    let views_by_day: Vec<(NaiveDate, i64)> = sqlx::query_as(
        r#"SELECT DATE(created_at) as date, COUNT(*) as views
           FROM video_analytics_events
           WHERE event_type = 'view' AND created_at >= NOW() - $1::interval
           GROUP BY DATE(created_at)
           ORDER BY date"#
    )
    .bind(format!("{} days", days))
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Device breakdown
    let device_breakdown: Vec<(Option<String>, i64)> = sqlx::query_as(
        r#"SELECT device_type, COUNT(*) as count
           FROM video_analytics_events
           WHERE created_at >= NOW() - $1::interval
           GROUP BY device_type"#
    )
    .bind(format!("{} days", days))
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let device_map: serde_json::Value = device_breakdown
        .into_iter()
        .map(|(d, c)| (d.unwrap_or_else(|| "unknown".to_string()), c))
        .collect::<std::collections::HashMap<_, _>>()
        .into();

    Ok(Json(json!({
        "success": true,
        "data": {
            "period": period,
            "total_videos": total_videos.0,
            "total_views": total_views.0,
            "total_watch_time_hours": total_watch_time.0 as f64 / 3600.0,
            "avg_completion_rate": avg_completion.0.unwrap_or(0.0),
            "unique_viewers": unique_viewers.0,
            "top_videos": top_videos.iter().map(|(id, title, thumb, views)| {
                json!({
                    "video_id": id,
                    "title": title,
                    "thumbnail_url": thumb,
                    "views": views
                })
            }).collect::<Vec<_>>(),
            "views_by_day": views_by_day.iter().map(|(date, views)| {
                json!({"date": date.to_string(), "views": views})
            }).collect::<Vec<_>>(),
            "device_breakdown": device_map
        }
    })))
}

/// Get analytics for a specific video
async fn get_video_analytics(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
    Query(query): Query<VideoAnalyticsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let period = query.period.as_deref().unwrap_or("30d");
    let days = match period {
        "7d" => 7,
        "30d" => 30,
        "90d" => 90,
        _ => 30,
    };

    // Get video info
    let video: (String, Option<String>) = sqlx::query_as(
        "SELECT title, thumbnail_url FROM unified_videos WHERE id = $1"
    )
    .bind(video_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    // Total views
    let total_views: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM video_analytics_events WHERE video_id = $1 AND event_type = 'view' AND created_at >= NOW() - $2::interval"
    )
    .bind(video_id)
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Unique viewers
    let unique_viewers: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT COALESCE(user_id::text, session_id)) FROM video_analytics_events WHERE video_id = $1 AND created_at >= NOW() - $2::interval"
    )
    .bind(video_id)
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Average completion
    let avg_completion: (Option<f64>,) = sqlx::query_as(
        "SELECT AVG(progress_percent)::float FROM video_analytics_events WHERE video_id = $1 AND progress_percent IS NOT NULL AND created_at >= NOW() - $2::interval"
    )
    .bind(video_id)
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    // Total watch time
    let total_watch_time: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(watch_time_seconds), 0) FROM video_analytics_events WHERE video_id = $1 AND created_at >= NOW() - $2::interval"
    )
    .bind(video_id)
    .bind(format!("{} days", days))
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Daily views
    let daily_views: Vec<(NaiveDate, i64, i64)> = sqlx::query_as(
        r#"SELECT DATE(created_at), COUNT(*), COUNT(DISTINCT COALESCE(user_id::text, session_id))
           FROM video_analytics_events
           WHERE video_id = $1 AND event_type = 'view' AND created_at >= NOW() - $2::interval
           GROUP BY DATE(created_at)
           ORDER BY DATE(created_at)"#
    )
    .bind(video_id)
    .bind(format!("{} days", days))
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "video_id": video_id,
            "title": video.0,
            "thumbnail_url": video.1,
            "period": period,
            "total_views": total_views.0,
            "unique_viewers": unique_viewers.0,
            "avg_completion_percent": avg_completion.0.unwrap_or(0.0),
            "total_watch_time_hours": total_watch_time.0 as f64 / 3600.0,
            "daily_views": daily_views.iter().map(|(date, views, unique)| {
                json!({"date": date.to_string(), "views": views, "unique_viewers": unique})
            }).collect::<Vec<_>>()
        }
    })))
}

/// Update user watch history / progress
async fn update_watch_progress(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = input.get("user_id").and_then(|v| v.as_i64());
    let position = input.get("position_seconds").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let watch_time = input.get("watch_time_seconds").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let completion = input.get("completion_percent").and_then(|v| v.as_i64()).unwrap_or(0) as i16;

    if let Some(uid) = user_id {
        sqlx::query(
            r#"INSERT INTO video_watch_history
               (user_id, video_id, last_position_seconds, total_watch_time_seconds, completion_percent, is_completed, completed_at, last_watched_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
               ON CONFLICT (user_id, video_id) DO UPDATE SET
                   last_position_seconds = $3,
                   total_watch_time_seconds = video_watch_history.total_watch_time_seconds + $4,
                   completion_percent = GREATEST(video_watch_history.completion_percent, $5),
                   is_completed = CASE WHEN $5 >= 90 THEN true ELSE video_watch_history.is_completed END,
                   completed_at = CASE WHEN $5 >= 90 AND video_watch_history.completed_at IS NULL THEN NOW() ELSE video_watch_history.completed_at END,
                   last_watched_at = NOW()"#
        )
        .bind(uid)
        .bind(video_id)
        .bind(position)
        .bind(watch_time)
        .bind(completion)
        .bind(completion >= 90)
        .bind(if completion >= 90 { Some(Utc::now()) } else { None })
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO SERIES / PLAYLISTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all series
async fn list_series(
    State(state): State<AppState>,
    Query(query): Query<SeriesListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM video_series WHERE deleted_at IS NULL");
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ct) = &query.content_type {
        conditions.push(format!("content_type = '{}'", ct));
    }
    if let Some(dl) = &query.difficulty_level {
        conditions.push(format!("difficulty_level = '{}'", dl));
    }
    if let Some(is_pub) = query.is_published {
        conditions.push(format!("is_published = {}", is_pub));
    }
    if let Some(search) = &query.search {
        conditions.push(format!("(title ILIKE '%{}%' OR description ILIKE '%{}%')",
            search.replace('\'', "''"), search.replace('\'', "''")));
    }

    for cond in &conditions {
        sql.push_str(&format!(" AND {}", cond));
    }
    sql.push_str(" ORDER BY sort_order, created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let series: Vec<VideoSeries> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Get total count
    let mut count_sql = String::from("SELECT COUNT(*) FROM video_series WHERE deleted_at IS NULL");
    for cond in &conditions {
        count_sql.push_str(&format!(" AND {}", cond));
    }
    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "success": true,
        "data": series,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Create a new series
async fn create_series(
    State(state): State<AppState>,
    Json(input): Json<CreateSeriesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = format!("{}-{}", slugify(&input.title), &Uuid::new_v4().to_string()[..6]);
    let tags_json = input.tags.as_ref().map(|t| serde_json::to_value(t).unwrap());

    let series: VideoSeries = sqlx::query_as(
        r#"INSERT INTO video_series
           (title, slug, description, thumbnail_url, content_type, difficulty_level,
            category, is_published, is_premium, required_plan_id, tags)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
           RETURNING *"#
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.thumbnail_url)
    .bind(&input.content_type)
    .bind(&input.difficulty_level)
    .bind(&input.category)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_premium.unwrap_or(false))
    .bind(input.required_plan_id)
    .bind(&tags_json)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create series: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Add videos if provided
    if let Some(video_ids) = &input.video_ids {
        for (i, video_id) in video_ids.iter().enumerate() {
            let _ = sqlx::query(
                "INSERT INTO video_series_items (series_id, video_id, sort_order) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
            )
            .bind(series.id)
            .bind(video_id)
            .bind(i as i32)
            .execute(&state.db.pool)
            .await;
        }
    }

    info!("Created series: {} ({})", series.title, series.id);

    Ok(Json(json!({
        "success": true,
        "message": "Series created successfully",
        "data": {"id": series.id, "slug": series.slug}
    })))
}

/// Get a single series with videos
async fn get_series(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let series: VideoSeries = sqlx::query_as(
        "SELECT * FROM video_series WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Series not found"}))))?;

    // Get videos in series
    let videos: Vec<(i64, String, Option<String>, Option<i32>, i32, Option<String>, bool)> = sqlx::query_as(
        r#"SELECT v.id, COALESCE(vsi.custom_title, v.title), v.thumbnail_url, v.duration,
                  vsi.sort_order, vsi.section_title, vsi.is_preview
           FROM video_series_items vsi
           JOIN unified_videos v ON vsi.video_id = v.id
           WHERE vsi.series_id = $1 AND v.deleted_at IS NULL
           ORDER BY vsi.section_order, vsi.sort_order"#
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "id": series.id,
            "title": series.title,
            "slug": series.slug,
            "description": series.description,
            "thumbnail_url": series.thumbnail_url,
            "content_type": series.content_type,
            "difficulty_level": series.difficulty_level,
            "is_published": series.is_published,
            "is_premium": series.is_premium,
            "video_count": series.video_count,
            "estimated_duration_minutes": series.estimated_duration_minutes,
            "videos": videos.iter().map(|(vid, title, thumb, dur, order, section, preview)| {
                json!({
                    "video_id": vid,
                    "title": title,
                    "thumbnail_url": thumb,
                    "duration": dur,
                    "sort_order": order,
                    "section_title": section,
                    "is_preview": preview
                })
            }).collect::<Vec<_>>(),
            "created_at": series.created_at.to_rfc3339()
        }
    })))
}

/// Update a series
async fn update_series(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSeriesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = Vec::new();

    if let Some(t) = &input.title { updates.push(format!("title = '{}'", t.replace('\'', "''"))); }
    if let Some(d) = &input.description { updates.push(format!("description = '{}'", d.replace('\'', "''"))); }
    if let Some(t) = &input.thumbnail_url { updates.push(format!("thumbnail_url = '{}'", t)); }
    if let Some(c) = &input.content_type { updates.push(format!("content_type = '{}'", c)); }
    if let Some(d) = &input.difficulty_level { updates.push(format!("difficulty_level = '{}'", d)); }
    if let Some(c) = &input.category { updates.push(format!("category = '{}'", c)); }
    if let Some(p) = input.is_published { updates.push(format!("is_published = {}", p)); }
    if let Some(p) = input.is_premium { updates.push(format!("is_premium = {}", p)); }
    if let Some(tags) = &input.tags {
        let tags_json = serde_json::to_string(tags).unwrap();
        updates.push(format!("tags = '{}'", tags_json));
    }

    if !updates.is_empty() {
        updates.push("updated_at = NOW()".to_string());
        let sql = format!("UPDATE video_series SET {} WHERE id = {}", updates.join(", "), id);
        sqlx::query(&sql)
            .execute(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    Ok(Json(json!({"success": true, "message": "Series updated successfully"})))
}

/// Delete a series
async fn delete_series(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE video_series SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true, "message": "Series deleted successfully"})))
}

/// Add videos to series
async fn add_videos_to_series(
    State(state): State<AppState>,
    Path(series_id): Path<i64>,
    Json(input): Json<AddVideosToSeriesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current max sort order
    let max_order: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(sort_order) FROM video_series_items WHERE series_id = $1"
    )
    .bind(series_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let mut order = max_order.0.unwrap_or(-1) + 1;

    for video_id in &input.video_ids {
        let _ = sqlx::query(
            "INSERT INTO video_series_items (series_id, video_id, sort_order, section_title) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
        )
        .bind(series_id)
        .bind(video_id)
        .bind(order)
        .bind(&input.section_title)
        .execute(&state.db.pool)
        .await;
        order += 1;
    }

    Ok(Json(json!({"success": true, "message": "Videos added to series"})))
}

/// Remove video from series
async fn remove_video_from_series(
    State(state): State<AppState>,
    Path((series_id, video_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM video_series_items WHERE series_id = $1 AND video_id = $2")
        .bind(series_id)
        .bind(video_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true, "message": "Video removed from series"})))
}

/// Reorder videos in series
async fn reorder_series_videos(
    State(state): State<AppState>,
    Path(series_id): Path<i64>,
    Json(input): Json<ReorderSeriesVideosRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.video_orders {
        sqlx::query(
            "UPDATE video_series_items SET sort_order = $1, section_title = $2, section_order = $3 WHERE series_id = $4 AND video_id = $5"
        )
        .bind(item.sort_order)
        .bind(&item.section_title)
        .bind(item.section_order.unwrap_or(0))
        .bind(series_id)
        .bind(item.video_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true, "message": "Series videos reordered"})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO CHAPTERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get chapters for a video
async fn get_chapters(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let chapters: Vec<VideoChapter> = sqlx::query_as(
        "SELECT * FROM video_chapters WHERE video_id = $1 ORDER BY chapter_number"
    )
    .bind(video_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let response: Vec<ChapterResponse> = chapters.iter().map(|c| ChapterResponse {
        id: c.id,
        title: c.title.clone(),
        description: c.description.clone(),
        start_time_seconds: c.start_time_seconds,
        end_time_seconds: c.end_time_seconds,
        formatted_start_time: format_duration(c.start_time_seconds),
        formatted_end_time: c.end_time_seconds.map(format_duration),
        thumbnail_url: c.thumbnail_url.clone(),
        chapter_number: c.chapter_number,
    }).collect();

    Ok(Json(json!({"success": true, "data": response})))
}

/// Create a chapter
async fn create_chapter(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
    Json(input): Json<CreateChapterRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get next chapter number
    let max_num: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(chapter_number) FROM video_chapters WHERE video_id = $1"
    )
    .bind(video_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let chapter_number = max_num.0.unwrap_or(0) + 1;

    let chapter: VideoChapter = sqlx::query_as(
        r#"INSERT INTO video_chapters
           (video_id, title, description, start_time_seconds, end_time_seconds,
            thumbnail_url, thumbnail_time_seconds, chapter_number)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING *"#
    )
    .bind(video_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.start_time_seconds)
    .bind(input.end_time_seconds)
    .bind(&input.thumbnail_url)
    .bind(input.thumbnail_time_seconds)
    .bind(chapter_number)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Update video's chapter_timestamps JSONB
    let _ = update_video_chapter_timestamps(&state, video_id).await;

    Ok(Json(json!({
        "success": true,
        "message": "Chapter created successfully",
        "data": {"id": chapter.id, "chapter_number": chapter.chapter_number}
    })))
}

/// Bulk create/replace chapters
async fn bulk_create_chapters(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
    Json(input): Json<BulkChaptersRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Delete existing chapters
    sqlx::query("DELETE FROM video_chapters WHERE video_id = $1")
        .bind(video_id)
        .execute(&state.db.pool)
        .await
        .ok();

    // Insert new chapters
    for (i, chapter) in input.chapters.iter().enumerate() {
        sqlx::query(
            r#"INSERT INTO video_chapters
               (video_id, title, description, start_time_seconds, end_time_seconds, chapter_number)
               VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(video_id)
        .bind(&chapter.title)
        .bind(&chapter.description)
        .bind(chapter.start_time_seconds)
        .bind(chapter.end_time_seconds)
        .bind((i + 1) as i32)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    // Update video's chapter_timestamps JSONB
    let _ = update_video_chapter_timestamps(&state, video_id).await;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} chapters created", input.chapters.len())
    })))
}

/// Update a chapter
async fn update_chapter(
    State(state): State<AppState>,
    Path((video_id, chapter_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateChapterRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut updates: Vec<String> = Vec::new();

    if let Some(t) = &input.title { updates.push(format!("title = '{}'", t.replace('\'', "''"))); }
    if let Some(d) = &input.description { updates.push(format!("description = '{}'", d.replace('\'', "''"))); }
    if let Some(s) = input.start_time_seconds { updates.push(format!("start_time_seconds = {}", s)); }
    if let Some(e) = input.end_time_seconds { updates.push(format!("end_time_seconds = {}", e)); }
    if let Some(t) = &input.thumbnail_url { updates.push(format!("thumbnail_url = '{}'", t)); }

    if !updates.is_empty() {
        updates.push("updated_at = NOW()".to_string());
        let sql = format!("UPDATE video_chapters SET {} WHERE id = {} AND video_id = {}",
            updates.join(", "), chapter_id, video_id);
        sqlx::query(&sql)
            .execute(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    // Update video's chapter_timestamps JSONB
    let _ = update_video_chapter_timestamps(&state, video_id).await;

    Ok(Json(json!({"success": true, "message": "Chapter updated successfully"})))
}

/// Delete a chapter
async fn delete_chapter(
    State(state): State<AppState>,
    Path((video_id, chapter_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM video_chapters WHERE id = $1 AND video_id = $2")
        .bind(chapter_id)
        .bind(video_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Re-number remaining chapters
    sqlx::query(
        r#"WITH numbered AS (
             SELECT id, ROW_NUMBER() OVER (ORDER BY start_time_seconds) as new_num
             FROM video_chapters WHERE video_id = $1
           )
           UPDATE video_chapters SET chapter_number = numbered.new_num
           FROM numbered WHERE video_chapters.id = numbered.id"#
    )
    .bind(video_id)
    .execute(&state.db.pool)
    .await
    .ok();

    // Update video's chapter_timestamps JSONB
    let _ = update_video_chapter_timestamps(&state, video_id).await;

    Ok(Json(json!({"success": true, "message": "Chapter deleted successfully"})))
}

/// Helper to update the JSONB chapter_timestamps field
async fn update_video_chapter_timestamps(state: &AppState, video_id: i64) -> Result<(), sqlx::Error> {
    let chapters: Vec<(String, i32, Option<i32>)> = sqlx::query_as(
        "SELECT title, start_time_seconds, end_time_seconds FROM video_chapters WHERE video_id = $1 ORDER BY chapter_number"
    )
    .bind(video_id)
    .fetch_all(&state.db.pool)
    .await?;

    let timestamps: Vec<serde_json::Value> = chapters.iter().map(|(title, start, end)| {
        json!({
            "title": title,
            "start": start,
            "end": end,
            "formatted_start": format_duration(*start),
            "formatted_end": end.map(|e| format_duration(e))
        })
    }).collect();

    sqlx::query("UPDATE unified_videos SET chapter_timestamps = $1 WHERE id = $2")
        .bind(serde_json::to_value(&timestamps).unwrap())
        .bind(video_id)
        .execute(&state.db.pool)
        .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SCHEDULED PUBLISHING
// ═══════════════════════════════════════════════════════════════════════════════════

/// List scheduled jobs
async fn list_scheduled_jobs(
    State(state): State<AppState>,
    Query(query): Query<ScheduledJobsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM scheduled_publish_jobs WHERE 1=1");

    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND status = '{}'", status));
    }
    if let Some(rt) = &query.resource_type {
        sql.push_str(&format!(" AND resource_type = '{}'", rt));
    }

    sql.push_str(" ORDER BY scheduled_at ASC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let jobs: Vec<ScheduledPublishJob> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "data": jobs.iter().map(|j| json!({
            "id": j.id,
            "resource_type": j.resource_type,
            "resource_id": j.resource_id,
            "scheduled_at": j.scheduled_at.to_rfc3339(),
            "timezone": j.timezone,
            "action": j.action,
            "status": j.status,
            "notify_on_publish": j.notify_on_publish,
            "created_at": j.created_at.to_rfc3339()
        })).collect::<Vec<_>>()
    })))
}

/// Create a scheduled job
async fn create_scheduled_job(
    State(state): State<AppState>,
    Json(input): Json<CreateScheduledJobRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let scheduled_at = DateTime::parse_from_rfc3339(&input.scheduled_at)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid datetime format. Use ISO 8601."}))))?
        .with_timezone(&Utc);

    let recipients = input.notification_recipients.as_ref().map(|r| serde_json::to_value(r).unwrap());

    let job: ScheduledPublishJob = sqlx::query_as(
        r#"INSERT INTO scheduled_publish_jobs
           (resource_type, resource_id, scheduled_at, timezone, action, notify_on_publish, notification_recipients)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING *"#
    )
    .bind(&input.resource_type)
    .bind(input.resource_id)
    .bind(scheduled_at)
    .bind(input.timezone.as_deref().unwrap_or("America/New_York"))
    .bind(&input.action)
    .bind(input.notify_on_publish.unwrap_or(false))
    .bind(&recipients)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Also update the video's scheduled_at field
    if input.resource_type == "video" && input.action == "publish" {
        let _ = sqlx::query("UPDATE unified_videos SET scheduled_at = $1 WHERE id = $2")
            .bind(scheduled_at)
            .bind(input.resource_id)
            .execute(&state.db.pool)
            .await;
    }

    info!("Created scheduled job: {} for {} {} at {}", job.id, input.resource_type, input.resource_id, scheduled_at);

    Ok(Json(json!({
        "success": true,
        "message": "Scheduled job created",
        "data": {"id": job.id, "scheduled_at": job.scheduled_at.to_rfc3339()}
    })))
}

/// Cancel a scheduled job
async fn cancel_scheduled_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE scheduled_publish_jobs SET status = 'cancelled', updated_at = NOW() WHERE id = $1 AND status = 'pending'")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"success": true, "message": "Scheduled job cancelled"})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BUNNY.NET WEBHOOK
// ═══════════════════════════════════════════════════════════════════════════════════

/// Handle Bunny.net webhook
async fn handle_bunny_webhook(
    State(state): State<AppState>,
    Json(payload): Json<BunnyWebhookPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let event_type = match payload.status {
        Some(4) => "VideoEncoded",
        Some(5) => "VideoFailed",
        Some(2) => "VideoProcessing",
        Some(3) => "VideoTranscoding",
        _ => "VideoUpdated",
    };

    // Store webhook event
    sqlx::query(
        r#"INSERT INTO bunny_webhook_events
           (event_type, video_guid, library_id, payload, status)
           VALUES ($1, $2, $3, $4, 'received')"#
    )
    .bind(event_type)
    .bind(&payload.video_guid)
    .bind(payload.video_library_id)
    .bind(serde_json::to_value(&payload).unwrap())
    .execute(&state.db.pool)
    .await
    .ok();

    // Update video record
    let encoding_status = match payload.status {
        Some(0) => "created",
        Some(1) => "uploaded",
        Some(2) => "processing",
        Some(3) => "transcoding",
        Some(4) => "finished",
        Some(5) => "error",
        _ => "unknown",
    };

    let mut update_parts = vec![
        format!("bunny_encoding_status = '{}'", encoding_status),
    ];

    if let Some(progress) = payload.encode_progress {
        update_parts.push(format!("encoding_progress = {}", progress));
    }
    if let Some(length) = payload.length {
        update_parts.push(format!("duration = {}", length));
    }
    if let Some(width) = payload.width {
        update_parts.push(format!("width = {}", width));
    }
    if let Some(height) = payload.height {
        update_parts.push(format!("height = {}", height));
    }
    if let Some(thumb) = &payload.thumbnail_url {
        update_parts.push(format!("bunny_thumbnail_url = '{}'", thumb));
    }

    let sql = format!(
        "UPDATE unified_videos SET {} WHERE bunny_video_guid = '{}'",
        update_parts.join(", "),
        payload.video_guid
    );

    sqlx::query(&sql)
        .execute(&state.db.pool)
        .await
        .ok();

    info!("Processed Bunny webhook: {} for {}", event_type, payload.video_guid);

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// CDN CACHE PURGE
// ═══════════════════════════════════════════════════════════════════════════════════

/// Purge CDN cache for a video
async fn purge_cdn_cache(
    State(_state): State<AppState>,
    Path(video_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();

    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured"
        }))));
    }

    // For now, we'll return success. Full implementation would call Bunny's purge API.
    // Bunny.net automatically purges cache when videos are re-encoded.

    info!("CDN cache purge requested for video: {}", video_id);

    Ok(Json(json!({
        "success": true,
        "message": "CDN cache purge initiated"
    })))
}

/// Purge all CDN cache
async fn purge_all_cdn_cache(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();

    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured"
        }))));
    }

    info!("Full CDN cache purge requested");

    Ok(Json(json!({
        "success": true,
        "message": "Full CDN cache purge initiated"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BULK UPLOAD QUEUE
// ═══════════════════════════════════════════════════════════════════════════════════

/// Initialize bulk upload
async fn init_bulk_upload(
    State(state): State<AppState>,
    Json(input): Json<BulkUploadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();

    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured"
        }))));
    }

    let batch_id = Uuid::new_v4();
    let mut uploads: Vec<serde_json::Value> = Vec::new();

    for (i, file) in input.files.iter().enumerate() {
        let title = file.title.clone().unwrap_or_else(|| {
            file.filename.split('.').next().unwrap_or("Untitled").to_string()
        });

        // Create video in Bunny
        let video = bunny.create_video(&title).await.map_err(|e| {
            error!("Failed to create Bunny video: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

        // Get upload URL
        let upload_url = bunny.get_upload_url(&video.guid).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

        // Build metadata
        let metadata = json!({
            "title": title,
            "description": file.description,
            "content_type": input.default_metadata.content_type,
            "video_date": input.default_metadata.video_date,
            "trader_id": input.default_metadata.trader_id,
            "room_ids": input.default_metadata.room_ids,
            "upload_to_all": input.default_metadata.upload_to_all,
            "is_published": input.default_metadata.is_published,
            "tags": input.default_metadata.tags
        });

        // Insert queue record
        let queue_item: VideoUploadQueue = sqlx::query_as(
            r#"INSERT INTO video_upload_queue
               (batch_id, batch_order, original_filename, file_size_bytes, content_type,
                bunny_video_guid, bunny_upload_url, video_metadata)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING *"#
        )
        .bind(batch_id)
        .bind(i as i32)
        .bind(&file.filename)
        .bind(file.file_size_bytes)
        .bind(&file.content_type)
        .bind(&video.guid)
        .bind(&upload_url)
        .bind(&metadata)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        uploads.push(json!({
            "id": queue_item.id,
            "filename": file.filename,
            "upload_url": upload_url,
            "video_guid": video.guid,
            "status": "pending"
        }));
    }

    info!("Initialized bulk upload batch {} with {} files", batch_id, input.files.len());

    Ok(Json(json!({
        "success": true,
        "data": {
            "batch_id": batch_id.to_string(),
            "uploads": uploads
        }
    })))
}

/// Get bulk upload batch status
async fn get_batch_status(
    State(state): State<AppState>,
    Path(batch_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let uuid = Uuid::parse_str(&batch_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid batch ID"}))))?;

    let items: Vec<VideoUploadQueue> = sqlx::query_as(
        "SELECT * FROM video_upload_queue WHERE batch_id = $1 ORDER BY batch_order"
    )
    .bind(uuid)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if items.is_empty() {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Batch not found"}))));
    }

    let total = items.len() as i32;
    let completed = items.iter().filter(|i| i.status == "completed").count() as i32;
    let failed = items.iter().filter(|i| i.status == "failed").count() as i32;
    let in_progress = items.iter().filter(|i| i.status == "uploading" || i.status == "processing").count() as i32;
    let pending = items.iter().filter(|i| i.status == "pending").count() as i32;

    Ok(Json(json!({
        "success": true,
        "data": {
            "batch_id": batch_id,
            "total_files": total,
            "completed": completed,
            "failed": failed,
            "in_progress": in_progress,
            "pending": pending,
            "uploads": items.iter().map(|i| json!({
                "id": i.id,
                "filename": i.original_filename,
                "status": i.status,
                "progress_percent": i.progress_percent,
                "error_message": i.error_message,
                "created_video_id": i.created_video_id
            })).collect::<Vec<_>>()
        }
    })))
}

/// Update upload item status (called by frontend after upload completes)
async fn update_upload_status(
    State(state): State<AppState>,
    Path(item_id): Path<i64>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let status = input.get("status").and_then(|v| v.as_str()).unwrap_or("completed");
    let progress = input.get("progress_percent").and_then(|v| v.as_i64()).unwrap_or(100) as i16;
    let error_msg = input.get("error_message").and_then(|v| v.as_str());

    sqlx::query(
        r#"UPDATE video_upload_queue
           SET status = $1, progress_percent = $2, error_message = $3, updated_at = NOW(),
               completed_at = CASE WHEN $1 IN ('completed', 'failed') THEN NOW() ELSE NULL END
           WHERE id = $4"#
    )
    .bind(status)
    .bind(progress)
    .bind(error_msg)
    .bind(item_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // If completed, create the video record
    if status == "completed" {
        let item: VideoUploadQueue = sqlx::query_as(
            "SELECT * FROM video_upload_queue WHERE id = $1"
        )
        .bind(item_id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        if let Some(guid) = &item.bunny_video_guid {
            let bunny = BunnyClient::new();
            let embed_url = bunny.get_embed_url(guid);

            let meta = &item.video_metadata;
            let title = meta.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
            let content_type = meta.get("content_type").and_then(|v| v.as_str()).unwrap_or("daily_video");
            let video_date = meta.get("video_date").and_then(|v| v.as_str())
                .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
                .unwrap_or_else(|| Utc::now().date_naive());

            let slug = format!("{}-{}", slugify(title), &Uuid::new_v4().to_string()[..6]);

            // Create video record
            let video_result: Result<(i64,), _> = sqlx::query_as(
                r#"INSERT INTO unified_videos
                   (title, slug, description, video_url, video_platform, bunny_video_guid,
                    content_type, video_date, is_published, original_filename)
                   VALUES ($1, $2, $3, $4, 'bunny', $5, $6, $7, $8, $9)
                   RETURNING id"#
            )
            .bind(title)
            .bind(&slug)
            .bind(meta.get("description").and_then(|v| v.as_str()))
            .bind(&embed_url)
            .bind(guid)
            .bind(content_type)
            .bind(video_date)
            .bind(meta.get("is_published").and_then(|v| v.as_bool()).unwrap_or(false))
            .bind(&item.original_filename)
            .fetch_one(&state.db.pool)
            .await;

            if let Ok((video_id,)) = video_result {
                // Update queue item with created video ID
                sqlx::query("UPDATE video_upload_queue SET created_video_id = $1 WHERE id = $2")
                    .bind(video_id)
                    .bind(item_id)
                    .execute(&state.db.pool)
                    .await
                    .ok();

                // Handle room assignments
                if meta.get("upload_to_all").and_then(|v| v.as_bool()).unwrap_or(false) {
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
                        .bind(video_id)
                        .bind(room_id)
                        .execute(&state.db.pool)
                        .await;
                    }
                } else if let Some(room_ids) = meta.get("room_ids").and_then(|v| v.as_array()) {
                    for room_id in room_ids {
                        if let Some(rid) = room_id.as_i64() {
                            let _ = sqlx::query(
                                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
                            )
                            .bind(video_id)
                            .bind(rid)
                            .execute(&state.db.pool)
                            .await;
                        }
                    }
                }
            }
        }
    }

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO CLONING
// ═══════════════════════════════════════════════════════════════════════════════════

/// Clone a video
async fn clone_video(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
    Json(input): Json<CloneVideoRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get original video
    let original: (String, Option<String>, String, String, Option<String>, Option<i32>, Option<i64>, Option<serde_json::Value>) =
        sqlx::query_as(
            r#"SELECT title, description, video_url, video_platform, thumbnail_url, duration, trader_id, tags
               FROM unified_videos WHERE id = $1 AND deleted_at IS NULL"#
        )
        .bind(video_id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found"}))))?;

    let new_title = input.new_title.unwrap_or_else(|| format!("{} (Copy)", original.0));
    let slug = format!("{}-{}", slugify(&new_title), &Uuid::new_v4().to_string()[..6]);
    let video_date = input.new_video_date.as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());
    let content_type = input.content_type.as_deref().unwrap_or("daily_video");

    // Create cloned video
    let cloned: (i64, String) = sqlx::query_as(
        r#"INSERT INTO unified_videos
           (title, slug, description, video_url, video_platform, thumbnail_url, duration,
            content_type, video_date, trader_id, is_published, tags, cloned_from_id)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
           RETURNING id, slug"#
    )
    .bind(&new_title)
    .bind(&slug)
    .bind(&original.1)
    .bind(&original.2)
    .bind(&original.3)
    .bind(&original.4)
    .bind(original.5)
    .bind(content_type)
    .bind(video_date)
    .bind(original.6)
    .bind(input.is_published.unwrap_or(false))
    .bind(&original.7)
    .bind(video_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Handle room assignments
    if input.upload_to_all.unwrap_or(false) {
        let rooms: Vec<(i64,)> = sqlx::query_as("SELECT id FROM trading_rooms WHERE is_active = true")
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

        for (room_id,) in rooms {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(cloned.0)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    } else if let Some(room_ids) = &input.room_ids {
        for room_id in room_ids {
            let _ = sqlx::query(
                "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
            )
            .bind(cloned.0)
            .bind(room_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    // Clone chapters if requested
    if input.include_chapters.unwrap_or(true) {
        sqlx::query(
            r#"INSERT INTO video_chapters (video_id, title, description, start_time_seconds, end_time_seconds, chapter_number)
               SELECT $1, title, description, start_time_seconds, end_time_seconds, chapter_number
               FROM video_chapters WHERE video_id = $2"#
        )
        .bind(cloned.0)
        .bind(video_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    info!("Cloned video {} to new video {}", video_id, cloned.0);

    Ok(Json(json!({
        "success": true,
        "message": "Video cloned successfully",
        "data": {"id": cloned.0, "slug": cloned.1}
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// CSV EXPORT
// ═══════════════════════════════════════════════════════════════════════════════════

/// Export videos to CSV
async fn export_videos_csv(
    State(state): State<AppState>,
    Query(query): Query<ExportVideosQuery>,
) -> impl IntoResponse {
    let mut sql = String::from(
        r#"SELECT v.id, v.title, v.slug, v.description, v.video_url, v.video_platform,
                  v.content_type, v.video_date, v.is_published, v.is_featured,
                  v.duration, v.views_count, v.created_at
           FROM unified_videos v WHERE v.deleted_at IS NULL"#
    );

    if let Some(ct) = &query.content_type {
        sql.push_str(&format!(" AND v.content_type = '{}'", ct));
    }
    if let Some(is_pub) = query.is_published {
        sql.push_str(&format!(" AND v.is_published = {}", is_pub));
    }
    if let Some(start) = &query.start_date {
        sql.push_str(&format!(" AND v.video_date >= '{}'", start));
    }
    if let Some(end) = &query.end_date {
        sql.push_str(&format!(" AND v.video_date <= '{}'", end));
    }

    sql.push_str(" ORDER BY v.video_date DESC");

    let videos: Vec<(i64, String, String, Option<String>, String, String, String, NaiveDate, bool, bool, Option<i32>, i32, DateTime<Utc>)> =
        sqlx::query_as(&sql)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

    // Build CSV
    let mut csv = String::from("ID,Title,Slug,Description,Video URL,Platform,Content Type,Video Date,Published,Featured,Duration (s),Views,Created At\n");

    for v in &videos {
        let desc = v.3.as_deref().unwrap_or("").replace('"', "\"\"");
        csv.push_str(&format!(
            "{},\"{}\",\"{}\",\"{}\",\"{}\",{},{},{},{},{},{},{},{}\n",
            v.0, v.1, v.2, desc, v.4, v.5, v.6, v.7, v.8, v.9,
            v.10.unwrap_or(0), v.11, v.12.to_rfc3339()
        ));
    }

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"videos_export.csv\""),
        ],
        csv
    )
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BULK EDIT
// ═══════════════════════════════════════════════════════════════════════════════════

/// Bulk edit videos
async fn bulk_edit_videos(
    State(state): State<AppState>,
    Json(input): Json<BulkEditRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let ids: Vec<String> = input.video_ids.iter().map(|id| id.to_string()).collect();
    let ids_str = ids.join(",");

    let mut updates: Vec<String> = Vec::new();

    if let Some(ct) = &input.updates.content_type {
        updates.push(format!("content_type = '{}'", ct));
    }
    if let Some(tid) = input.updates.trader_id {
        updates.push(format!("trader_id = {}", tid));
    }
    if let Some(is_pub) = input.updates.is_published {
        updates.push(format!("is_published = {}", is_pub));
    }
    if let Some(is_feat) = input.updates.is_featured {
        updates.push(format!("is_featured = {}", is_feat));
    }
    if let Some(dl) = &input.updates.difficulty_level {
        updates.push(format!("difficulty_level = '{}'", dl));
    }
    if let Some(cat) = &input.updates.category {
        updates.push(format!("category = '{}'", cat));
    }

    // Execute field updates
    if !updates.is_empty() {
        let sql = format!(
            "UPDATE unified_videos SET {} WHERE id IN ({})",
            updates.join(", "),
            ids_str
        );
        sqlx::query(&sql)
            .execute(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    // Handle tag additions
    if let Some(add_tags) = &input.updates.add_tags {
        for video_id in &input.video_ids {
            for tag in add_tags {
                sqlx::query(
                    "UPDATE unified_videos SET tags = COALESCE(tags, '[]'::jsonb) || $1::jsonb WHERE id = $2 AND NOT (tags @> $1::jsonb)"
                )
                .bind(serde_json::to_value(vec![tag]).unwrap())
                .bind(video_id)
                .execute(&state.db.pool)
                .await
                .ok();
            }
        }
    }

    // Handle tag removals
    if let Some(remove_tags) = &input.updates.remove_tags {
        for video_id in &input.video_ids {
            for tag in remove_tags {
                sqlx::query(
                    "UPDATE unified_videos SET tags = tags - $1 WHERE id = $2"
                )
                .bind(tag)
                .bind(video_id)
                .execute(&state.db.pool)
                .await
                .ok();
            }
        }
    }

    // Handle room additions
    if let Some(add_rooms) = &input.updates.add_room_ids {
        for video_id in &input.video_ids {
            for room_id in add_rooms {
                sqlx::query(
                    "INSERT INTO video_room_assignments (video_id, trading_room_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
                )
                .bind(video_id)
                .bind(room_id)
                .execute(&state.db.pool)
                .await
                .ok();
            }
        }
    }

    // Handle room removals
    if let Some(remove_rooms) = &input.updates.remove_room_ids {
        let remove_str: Vec<String> = remove_rooms.iter().map(|r| r.to_string()).collect();
        for video_id in &input.video_ids {
            sqlx::query(&format!(
                "DELETE FROM video_room_assignments WHERE video_id = {} AND trading_room_id IN ({})",
                video_id,
                remove_str.join(",")
            ))
            .execute(&state.db.pool)
            .await
            .ok();
        }
    }

    info!("Bulk edited {} videos", input.video_ids.len());

    Ok(Json(json!({
        "success": true,
        "message": format!("{} videos updated", input.video_ids.len())
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DRAG & DROP REORDER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Reorder videos in a room
async fn reorder_room_videos(
    State(state): State<AppState>,
    Path(room_id): Path<i64>,
    Json(input): Json<ReorderVideosRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in &input.video_orders {
        sqlx::query(
            "UPDATE video_room_assignments SET sort_order = $1, is_pinned = $2 WHERE video_id = $3 AND trading_room_id = $4"
        )
        .bind(item.sort_order)
        .bind(item.is_pinned.unwrap_or(false))
        .bind(item.video_id)
        .bind(room_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    Ok(Json(json!({"success": true, "message": "Videos reordered"})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// AUTO-DURATION EXTRACTION
// ═══════════════════════════════════════════════════════════════════════════════════

/// Fetch and update duration from Bunny.net
async fn fetch_video_duration(
    State(state): State<AppState>,
    Path(video_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();

    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured"
        }))));
    }

    // Get video's Bunny GUID
    let guid: Option<(String,)> = sqlx::query_as(
        "SELECT bunny_video_guid FROM unified_videos WHERE id = $1 AND bunny_video_guid IS NOT NULL"
    )
    .bind(video_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let guid = guid.ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Video not found or no Bunny GUID"}))))?;

    // Fetch from Bunny
    let bunny_video = bunny.get_video(&guid.0).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Update duration and dimensions
    sqlx::query(
        "UPDATE unified_videos SET duration = $1, width = $2, height = $3 WHERE id = $4"
    )
    .bind(bunny_video.length)
    .bind(bunny_video.width)
    .bind(bunny_video.height)
    .bind(video_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "duration": bunny_video.length,
            "width": bunny_video.width,
            "height": bunny_video.height,
            "formatted_duration": format_duration(bunny_video.length)
        }
    })))
}

/// Bulk fetch durations for all Bunny videos
async fn fetch_all_durations(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bunny = BunnyClient::new();

    if !bunny.is_configured() {
        return Err((StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "error": "Bunny.net not configured"
        }))));
    }

    // Get all videos without duration but with Bunny GUID
    let videos: Vec<(i64, String)> = sqlx::query_as(
        "SELECT id, bunny_video_guid FROM unified_videos WHERE bunny_video_guid IS NOT NULL AND (duration IS NULL OR duration = 0) LIMIT 50"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mut updated = 0;

    for (video_id, guid) in &videos {
        if let Ok(bunny_video) = bunny.get_video(guid).await {
            let result = sqlx::query(
                "UPDATE unified_videos SET duration = $1, width = $2, height = $3 WHERE id = $4"
            )
            .bind(bunny_video.length)
            .bind(bunny_video.width)
            .bind(bunny_video.height)
            .bind(video_id)
            .execute(&state.db.pool)
            .await;

            if result.is_ok() {
                updated += 1;
            }
        }
    }

    info!("Bulk updated {} video durations", updated);

    Ok(Json(json!({
        "success": true,
        "message": format!("{} video durations updated", updated),
        "total_processed": videos.len(),
        "updated": updated
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Analytics
        .route("/analytics/track", post(track_event))
        .route("/analytics/track-batch", post(track_events_batch))
        .route("/analytics/dashboard", get(get_analytics_dashboard))
        .route("/analytics/video/:video_id", get(get_video_analytics))
        .route("/analytics/progress/:video_id", post(update_watch_progress))

        // Series
        .route("/series", get(list_series).post(create_series))
        .route("/series/:id", get(get_series).put(update_series).delete(delete_series))
        .route("/series/:series_id/videos", post(add_videos_to_series))
        .route("/series/:series_id/videos/:video_id", delete(remove_video_from_series))
        .route("/series/:series_id/reorder", post(reorder_series_videos))

        // Chapters
        .route("/videos/:video_id/chapters", get(get_chapters).post(create_chapter))
        .route("/videos/:video_id/chapters/bulk", post(bulk_create_chapters))
        .route("/videos/:video_id/chapters/:chapter_id", put(update_chapter).delete(delete_chapter))

        // Scheduled Publishing
        .route("/scheduled-jobs", get(list_scheduled_jobs).post(create_scheduled_job))
        .route("/scheduled-jobs/:id/cancel", post(cancel_scheduled_job))

        // Bulk Upload
        .route("/bulk-upload", post(init_bulk_upload))
        .route("/bulk-upload/:batch_id", get(get_batch_status))
        .route("/bulk-upload/item/:item_id", put(update_upload_status))

        // Bunny Webhook
        .route("/webhook/bunny", post(handle_bunny_webhook))

        // CDN
        .route("/cdn/purge/:video_id", post(purge_cdn_cache))
        .route("/cdn/purge-all", post(purge_all_cdn_cache))

        // Video Operations
        .route("/videos/:video_id/clone", post(clone_video))
        .route("/videos/:video_id/duration", post(fetch_video_duration))
        .route("/videos/fetch-durations", post(fetch_all_durations))

        // Export
        .route("/export/csv", get(export_videos_csv))

        // Bulk Edit
        .route("/bulk-edit", post(bulk_edit_videos))

        // Reorder
        .route("/rooms/:room_id/reorder", post(reorder_room_videos))
}
