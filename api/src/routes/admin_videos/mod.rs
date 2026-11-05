//! Admin Video Management Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full CRUD operations for unified video system with:
//! - Thumbnails & Tags
//! - Multi-platform support (Bunny.net, Vimeo, YouTube, Wistia)
//! - Room assignments
//! - Bulk operations
//! - Content type filtering (learning_center, daily_video, weekly_watchlist, room_archive)
//!
//! Module split (R7-B, 2026-05-20):
//! - `helpers`        — slugify / format_duration / get_embed_url / video_to_response
//! - `crud`           — list / get / create / update / delete / stats / options / bulk publish-delete-assign
//! - `analytics`      — dashboard + tracking + per-video analytics + watch progress
//! - `series_chapters` — series CRUD + chapters CRUD
//! - `operations`     — clone / duration / bulk-edit / CSV export / CDN purge / scheduled-jobs / bulk-upload / bulk-tags / bulk-feature
//! - `bunny_embed`    — Bunny webhook + transcoding-status + thumbnail-generation + embed-code

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

mod analytics;
mod bunny_embed;
mod crud;
mod helpers;
mod operations;
mod series_chapters;

// ═══════════════════════════════════════════════════════════════════════════
// SHARED TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedVideoRow {
    pub id: i64,
    pub title: String,
    pub slug: String,
    #[sqlx(default)]
    pub description: Option<String>,
    #[sqlx(default)]
    pub video_url: Option<String>,
    #[sqlx(default)]
    pub video_platform: Option<String>,
    #[sqlx(default)]
    pub video_id: Option<String>,
    #[sqlx(default)]
    pub bunny_video_guid: Option<String>,
    #[sqlx(default)]
    pub thumbnail_url: Option<String>,
    #[sqlx(default)]
    pub thumbnail_path: Option<String>,
    #[sqlx(default)]
    pub duration: Option<i32>,
    #[sqlx(default)]
    pub quality: Option<String>,
    #[sqlx(default)]
    pub content_type: String,
    #[sqlx(default)]
    pub difficulty_level: Option<String>,
    #[sqlx(default)]
    pub category: Option<String>,
    #[sqlx(default)]
    pub session_type: Option<String>,
    #[sqlx(default)]
    pub chapter_timestamps: Option<serde_json::Value>,
    #[sqlx(default)]
    pub trader_id: Option<i64>,
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalDate")]
    pub video_date: Option<NaiveDate>,
    #[sqlx(default)]
    pub is_published: Option<bool>,
    #[sqlx(default)]
    pub is_featured: Option<bool>,
    #[sqlx(default)]
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalTimestamp")]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(default)]
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalTimestamp")]
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(default)]
    pub tags: Option<serde_json::Value>,
    #[sqlx(default)]
    pub views_count: Option<i32>,
    #[sqlx(default)]
    pub likes_count: i32,
    #[sqlx(default)]
    pub completion_rate: i32,
    #[sqlx(default)]
    pub bunny_library_id: Option<i64>,
    #[sqlx(default)]
    pub bunny_encoding_status: Option<String>,
    #[sqlx(default)]
    pub bunny_thumbnail_url: Option<String>,
    #[sqlx(default)]
    pub metadata: Option<serde_json::Value>,
    #[sqlx(default)]
    pub created_by: Option<i64>,
    #[sqlx(default)]
    pub updated_by: Option<i64>,
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalTimestamp")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalTimestamp")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[sqlx(default)]
    #[sqlx(try_from = "crate::models::video::temporal::PgSafeOptionalTimestamp")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(crud::list_videos).post(crud::create_video))
        .route(
            "/{id}",
            get(crud::get_video)
                .put(crud::update_video)
                .delete(crud::delete_video),
        )
        .route("/stats", get(crud::get_stats))
        .route("/options", get(crud::get_options))
        .route("/bulk/publish", post(crud::bulk_publish))
        .route("/bulk/delete", post(crud::bulk_delete))
        .route("/bulk/assign", post(crud::bulk_assign))
}

/// Video-advanced router for /video-advanced endpoints
/// ICT 7 Grade - Frontend compatibility
pub fn analytics_router() -> Router<AppState> {
    Router::new()
        // Analytics
        .route("/analytics/dashboard", get(analytics::analytics_dashboard))
        .route("/analytics/track", post(analytics::track_video_event))
        .route(
            "/analytics/track-batch",
            post(analytics::track_video_events_batch),
        )
        .route("/analytics/video/{id}", get(analytics::get_video_analytics))
        .route(
            "/analytics/progress/{id}",
            post(analytics::update_watch_progress),
        )
        // Series
        .route(
            "/series",
            get(series_chapters::list_series).post(series_chapters::create_series),
        )
        .route(
            "/series/{id}",
            get(series_chapters::get_series)
                .put(series_chapters::update_series)
                .delete(series_chapters::delete_series),
        )
        .route(
            "/series/{id}/videos",
            post(series_chapters::add_series_videos),
        )
        .route(
            "/series/{id}/videos/{video_id}",
            delete(series_chapters::remove_series_video),
        )
        .route(
            "/series/{id}/reorder",
            post(series_chapters::reorder_series_videos),
        )
        // Chapters
        .route(
            "/videos/{id}/chapters",
            get(series_chapters::list_chapters).post(series_chapters::create_chapter),
        )
        .route(
            "/videos/{id}/chapters/bulk",
            post(series_chapters::bulk_create_chapters),
        )
        .route(
            "/videos/{id}/chapters/{chapter_id}",
            put(series_chapters::update_chapter).delete(series_chapters::delete_chapter),
        )
        // Scheduled Jobs
        .route(
            "/scheduled-jobs",
            get(operations::list_scheduled_jobs).post(operations::create_scheduled_job),
        )
        .route(
            "/scheduled-jobs/{id}/cancel",
            post(operations::cancel_scheduled_job),
        )
        // Bulk Upload
        .route("/bulk-upload", post(operations::init_bulk_upload))
        .route("/bulk-upload/{batch_id}", get(operations::get_batch_status))
        .route(
            "/bulk-upload/item/{id}",
            put(operations::update_upload_item),
        )
        // Video Operations
        .route("/videos/{id}/clone", post(operations::clone_video))
        .route(
            "/videos/{id}/duration",
            post(operations::fetch_video_duration),
        )
        .route(
            "/videos/fetch-durations",
            post(operations::fetch_all_durations),
        )
        .route("/bulk-edit", post(operations::bulk_edit_videos))
        .route("/export/csv", get(operations::export_videos_csv))
        .route("/rooms/{id}/reorder", post(operations::reorder_room_videos))
        // CDN
        .route("/cdn/purge/{id}", post(operations::purge_video_cdn))
        .route("/cdn/purge-all", post(operations::purge_all_cdn))
        // ICT 7 ADDITIONS: Transcoding, Thumbnails, Embed, Bulk Operations
        .route("/bunny/webhook", post(bunny_embed::bunny_webhook))
        .route(
            "/videos/{id}/transcoding-status",
            get(bunny_embed::get_transcoding_status),
        )
        .route(
            "/videos/{id}/generate-thumbnail",
            post(bunny_embed::generate_thumbnail),
        )
        .route("/videos/{id}/embed-code", get(bunny_embed::get_embed_code))
        .route("/bulk/tags", post(operations::bulk_update_tags))
        .route("/bulk/feature", post(operations::bulk_feature))
}
