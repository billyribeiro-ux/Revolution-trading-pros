//! Admin video handlers for trading rooms — ICT 7 FIX surface.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! Backs `/api/admin/trading-rooms/videos` and
//! `/api/admin/trading-rooms/videos/:slug` so the frontend video-picker
//! and per-room video list keep working. SQL, room_slug tracing field,
//! and the audit log shape are preserved exactly.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::types::VideosQuery;

/// List videos for trading rooms (admin) - ICT 7: Requires admin auth
pub(super) async fn admin_list_videos(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<VideosQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_videos",
        "ICT 7 AUDIT: Admin listing videos"
    );
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Query videos with optional room filter
    let videos: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<i32>,
            String,
            bool,
            chrono::NaiveDateTime,
        ),
    >(
        r"
        SELECT v.id, v.title, v.slug, v.description, v.video_url, v.thumbnail_url,
               v.duration, v.content_type, v.is_published, v.created_at
        FROM unified_videos v
        WHERE v.deleted_at IS NULL
          AND ($1::text IS NULL OR v.content_type = $1)
          AND ($2::boolean IS NULL OR v.is_published = $2)
        ORDER BY v.created_at DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(&query.content_type)
    .bind(query.is_published)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|r| {
                json!({
                    "id": r.0,
                    "title": r.1,
                    "slug": r.2,
                    "description": r.3,
                    "video_url": r.4,
                    "thumbnail_url": r.5,
                    "duration": r.6,
                    "content_type": r.7,
                    "is_published": r.8,
                    "created_at": r.9.to_string()
                })
            })
            .collect()
    })
    .unwrap_or_default();

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM unified_videos WHERE deleted_at IS NULL")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": videos,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// List videos for a specific trading room by slug (admin) - ICT 7: Requires admin auth
pub(super) async fn admin_list_videos_by_room(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(slug): Path<String>,
    Query(query): Query<VideosQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_videos_by_room",
        room_slug = %slug,
        "ICT 7 AUDIT: Admin listing videos by room"
    );
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Get room ID from slug
    let room_id: Option<i64> = sqlx::query_scalar("SELECT id FROM trading_rooms WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let videos: Vec<serde_json::Value> = if let Some(rid) = room_id {
        sqlx::query_as::<
            _,
            (
                i64,
                String,
                String,
                Option<String>,
                String,
                Option<String>,
                Option<i32>,
                String,
                bool,
                chrono::NaiveDateTime,
            ),
        >(
            r"
            SELECT v.id, v.title, v.slug, v.description, v.video_url, v.thumbnail_url,
                   v.duration, v.content_type, v.is_published, v.created_at
            FROM unified_videos v
            JOIN video_room_assignments vra ON v.id = vra.video_id
            WHERE vra.trading_room_id = $1
              AND v.deleted_at IS NULL
              AND ($2::text IS NULL OR v.content_type = $2)
              AND ($3::boolean IS NULL OR v.is_published = $3)
            ORDER BY v.created_at DESC
            LIMIT $4 OFFSET $5
            ",
        )
        .bind(rid)
        .bind(&query.content_type)
        .bind(query.is_published)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|r| {
                    json!({
                        "id": r.0,
                        "title": r.1,
                        "slug": r.2,
                        "description": r.3,
                        "video_url": r.4,
                        "thumbnail_url": r.5,
                        "duration": r.6,
                        "content_type": r.7,
                        "is_published": r.8,
                        "created_at": r.9.to_string()
                    })
                })
                .collect()
        })
        .unwrap_or_default()
    } else {
        vec![]
    };

    let total = videos.len() as i64;

    Ok(Json(json!({
        "success": true,
        "data": videos,
        "room_slug": slug,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total,
            "total_pages": (total as f64 / per_page as f64).ceil().max(1.0) as i64
        }
    })))
}
