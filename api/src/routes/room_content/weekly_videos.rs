//! Weekly Video handlers (get current / list / create / list archived)
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use super::{CreateWeeklyVideoRequest, ListQuery, PaginationMeta, WeeklyVideo};
use crate::cache::{cache_keys, cache_ttl};
use crate::routes::room_content::ensure_room_access;
use crate::{middleware::admin::AdminUser, models::User, AppState};

/// Cached response for weekly video
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeeklyVideoResponse {
    data: Option<WeeklyVideo>,
}

/// Cached response for weekly videos list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeeklyVideosResponse {
    data: Vec<WeeklyVideo>,
    meta: PaginationMeta,
}

/// Archive query parameters
#[derive(Debug, Deserialize)]
pub struct ArchiveQuery {
    pub year: Option<i32>,
}

/// Archived week response with stats
#[derive(Debug, Serialize)]
pub struct ArchivedWeekResponse {
    pub id: i64,
    pub week_of: NaiveDate,
    pub week_title: String,
    pub video_title: String,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub alert_count: i64,
    pub trade_count: i64,
    pub win_rate: Option<f64>,
}

/// Get current weekly video for a room
/// ICT 7+ Phase 2: Redis cached with 1 hour TTL
pub(super) async fn get_weekly_video(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let cache_key = cache_keys::weekly_video(&room_slug);

    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::WEEKLY_VIDEO, || async {
            let video: Option<WeeklyVideo> = sqlx::query_as(
                r"SELECT * FROM room_weekly_videos
                   WHERE room_slug = $1 AND is_current = true AND is_published = true
                   LIMIT 1",
            )
            .bind(&room_slug)
            .fetch_optional(&state.db.pool)
            .await?;

            Ok(WeeklyVideoResponse { data: video })
        })
        .await
        .map_err(|e| {
            error!("Failed to fetch weekly video: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "data": response.data
    })))
}

/// List all weekly videos (including archived)
/// ICT 7+ Phase 2: Redis cached with 1 hour TTL
pub(super) async fn list_weekly_videos(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let cache_key = cache_keys::weekly_videos(&room_slug, page, per_page);

    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::WEEKLY_VIDEOS, || async {
            let videos: Vec<WeeklyVideo> = sqlx::query_as(
                r"SELECT * FROM room_weekly_videos
                   WHERE room_slug = $1
                   ORDER BY week_of DESC
                   LIMIT $2 OFFSET $3",
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await?;

            let total: (i64,) =
                sqlx::query_as("SELECT COUNT(*) FROM room_weekly_videos WHERE room_slug = $1")
                    .bind(&room_slug)
                    .fetch_one(&state.db.pool)
                    .await?;

            Ok(WeeklyVideosResponse {
                data: videos,
                meta: PaginationMeta {
                    current_page: page,
                    per_page,
                    total: total.0,
                    total_pages: (total.0 as f64 / per_page as f64).ceil() as i64,
                },
            })
        })
        .await
        .map_err(|e| {
            error!("Failed to fetch weekly videos: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "data": response.data,
        "meta": response.meta
    })))
}

/// Create/publish a new weekly video (archives the previous one)
/// ICT 7+ Phase 2: Invalidates weekly video cache after creation
pub(super) async fn create_weekly_video(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateWeeklyVideoRequest>,
) -> Result<Json<WeeklyVideo>, (StatusCode, Json<serde_json::Value>)> {
    let week_of = NaiveDate::parse_from_str(&input.week_of, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid week_of date format"})),
        )
    })?;

    // Archive existing current video for this room
    sqlx::query(
        "UPDATE room_weekly_videos SET is_current = false, archived_at = NOW() WHERE room_slug = $1 AND is_current = true"
    )
    .bind(&input.room_slug)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Create new current video
    // room_id references trading_rooms.id, NOT membership_plans.id
    let video: WeeklyVideo = sqlx::query_as(
        r"INSERT INTO room_weekly_videos
           (room_id, room_slug, week_of, week_title, video_title, video_url, video_platform, thumbnail_url, duration, description, is_current, is_published)
           VALUES (
               (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
               $1, $2, $3, $4, $5, $6, $7, $8, $9, true, true
           )
           RETURNING *"
    )
    .bind(&input.room_slug)
    .bind(week_of)
    .bind(&input.week_title)
    .bind(&input.video_title)
    .bind(&input.video_url)
    .bind(input.video_platform.unwrap_or_else(|| "bunny".to_string()))
    .bind(&input.thumbnail_url)
    .bind(&input.duration)
    .bind(&input.description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create weekly video: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Invalidate weekly video cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_weekly_video_change(&input.room_slug)
        .await
    {
        error!("Failed to invalidate weekly video cache: {}", e);
    }

    info!(
        "Created weekly video: {} for {}",
        video.video_title, input.room_slug
    );
    Ok(Json(video))
}

/// List archived weekly videos for a room (past weeks only)
pub(super) async fn list_archived_videos(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
    Query(query): Query<ArchiveQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let year = query.year.unwrap_or_else(|| Utc::now().year());

    // Query archived videos with alert/trade counts
    #[allow(clippy::type_complexity)]
    let videos: Vec<(
        i64,
        NaiveDate,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
    )> = sqlx::query_as(
        r"SELECT
            v.id,
            v.week_of,
            v.week_title,
            v.video_title,
            v.video_url,
            v.thumbnail_url,
            v.duration
        FROM room_weekly_videos v
        WHERE v.room_slug = $1
        AND v.is_current = FALSE
        AND EXTRACT(YEAR FROM v.week_of) = $2
        ORDER BY v.week_of DESC",
    )
    .bind(&room_slug)
    .bind(year)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch archived videos: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Build response with counts for each week
    let mut response_data = Vec::new();
    for (id, week_of, week_title, video_title, video_url, thumbnail_url, duration) in videos {
        // Get alert count for this week
        let alert_count: (i64,) = sqlx::query_as(
            r"SELECT COUNT(*) FROM room_alerts
               WHERE room_slug = $1
               AND DATE_TRUNC('week', published_at) = DATE_TRUNC('week', $2::date)",
        )
        .bind(&room_slug)
        .bind(week_of)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

        // Get trade count and win rate for this week
        let trade_stats: (i64, Option<f64>) = sqlx::query_as(
            r"SELECT
                COUNT(*),
                ROUND(100.0 * COUNT(*) FILTER (WHERE result = 'WIN') / NULLIF(COUNT(*) FILTER (WHERE status = 'closed'), 0), 1)
               FROM room_trades
               WHERE room_slug = $1
               AND DATE_TRUNC('week', entry_date) = DATE_TRUNC('week', $2::date)
               AND deleted_at IS NULL"
        )
        .bind(&room_slug)
        .bind(week_of)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0, None));

        response_data.push(json!({
            "id": id,
            "week_of": week_of.format("%Y-%m-%d").to_string(),
            "week_title": week_title,
            "video_title": video_title,
            "video_url": video_url,
            "thumbnail_url": thumbnail_url,
            "duration": duration,
            "alert_count": alert_count.0,
            "trade_count": trade_stats.0,
            "win_rate": trade_stats.1
        }));
    }

    Ok(Json(json!({
        "success": true,
        "data": response_data
    })))
}
