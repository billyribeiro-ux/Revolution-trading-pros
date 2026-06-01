//! Admin dashboard per-room content stats.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! Backs `GET /api/admin/rooms/stats`, which the admin dashboard
//! consumes via the `RoomStats` interface. SQL, error propagation via
//! `?` (intentional — see fn-level docs), tracing fields, and the
//! `RoomStatsRow` wire shape are preserved exactly.

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::fallback::get_fallback_rooms;

/// Per-room content counts returned to /admin/dashboard.
///
/// FIX-2026-04-26: shape matches `RoomStats` interface in
/// `routes/admin/dashboard/+page.svelte` exactly. The page expects an
/// array (one entry per room) and matches by `room_id` (which it
/// populates from `ROOMS[*].id`, i.e. the room slug). Counts that
/// can't be sourced for a given room default to zero.
#[derive(Debug, Serialize)]
pub(super) struct RoomStatsRow {
    room_id: String,
    watchlist_count: i64,
    modules_count: i64,
    articles_count: i64,
}

/// Get per-room content stats for the admin dashboard.
/// GET /api/admin/rooms/stats
///
/// Sources each count via dedicated `COALESCE(SUM/COUNT, 0)` queries on
/// `watchlist_entries`, `unified_videos`, and `room_resources` joined by
/// `room_slug`. Each room enumerated from `trading_rooms` (with the same
/// hardcoded fallback list as `list_trading_rooms`) gets a row, with zeros
/// for any subquery that fails — but unlike `unwrap_or_default()`, errors
/// are propagated via `?` so the endpoint surfaces real DB failures
/// (FIX-2026-04-26 error-propagation pattern).
pub(super) async fn admin_get_rooms_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<RoomStatsRow>>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_get_rooms_stats",
        "ICT 7 AUDIT: Admin fetching rooms stats"
    );

    // Collect the canonical room-slug list from `trading_rooms` if available,
    // otherwise fall back to the hardcoded room slugs used elsewhere.
    let db_slugs: Vec<String> = sqlx::query_scalar::<_, String>(
        r"SELECT slug FROM trading_rooms WHERE is_active = true ORDER BY sort_order ASC",
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let room_slugs: Vec<String> = if db_slugs.is_empty() {
        get_fallback_rooms()
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|r| r.get("slug").and_then(|s| s.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        db_slugs
    };

    let mut stats: Vec<RoomStatsRow> = Vec::with_capacity(room_slugs.len());

    for slug in room_slugs {
        let watchlist_count: i64 = sqlx::query_scalar(
            r"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM watchlist_entries
               WHERE room_slug = $1 AND is_active = true",
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "watchlist_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("watchlist_count query failed: {}", e)})),
            )
        })?;

        let modules_count: i64 = sqlx::query_scalar(
            r"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM unified_videos
               WHERE room_slug = $1
                 AND is_published = true
                 AND deleted_at IS NULL",
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "modules_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("modules_count query failed: {}", e)})),
            )
        })?;

        // "Articles" maps to non-video room resources (PDFs, docs, etc.).
        let articles_count: i64 = sqlx::query_scalar(
            r"SELECT COALESCE(COUNT(*), 0)::BIGINT
               FROM room_resources
               WHERE room_slug = $1
                 AND is_published = true
                 AND deleted_at IS NULL
                 AND resource_type <> 'video'",
        )
        .bind(&slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "trading_rooms", room_slug = %slug, "articles_count query failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("articles_count query failed: {}", e)})),
            )
        })?;

        stats.push(RoomStatsRow {
            room_id: slug,
            watchlist_count,
            modules_count,
            articles_count,
        });
    }

    Ok(Json(stats))
}
