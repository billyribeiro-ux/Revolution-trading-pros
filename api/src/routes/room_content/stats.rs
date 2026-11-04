//! Stats handlers (get cached room performance stats)
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use super::{RoomStats, ensure_room_access};
use crate::cache::{cache_keys, cache_ttl};
use crate::{AppState, models::User};

/// Cached response for room stats
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatsResponse {
    data: Option<RoomStats>,
}

/// Get room stats
/// ICT 7+ Phase 2: Redis cached with 5 minute TTL
pub(crate) async fn get_room_stats(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let cache_key = cache_keys::stats(&room_slug);

    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::STATS, || async {
            let stats: Option<RoomStats> = sqlx::query_as(
                r"SELECT
                    id,
                    COALESCE(room_id, (SELECT id FROM trading_rooms WHERE slug = room_stats_cache.room_slug LIMIT 1), 0) AS room_id,
                    room_slug,
                    win_rate::float8 AS win_rate,
                    weekly_profit,
                    monthly_profit,
                    active_trades,
                    closed_this_week,
                    total_trades,
                    wins,
                    losses,
                    avg_win::float8 AS avg_win,
                    avg_loss::float8 AS avg_loss,
                    profit_factor::float8 AS profit_factor,
                    avg_holding_days::float8 AS avg_holding_days,
                    largest_win::float8 AS largest_win,
                    largest_loss::float8 AS largest_loss,
                    current_streak,
                    calculated_at
                  FROM room_stats_cache
                  WHERE room_slug = $1",
            )
            .bind(&room_slug)
            .fetch_optional(&state.db.pool)
            .await?;

            Ok(StatsResponse { data: stats })
        })
        .await
        .map_err(|e| {
            error!("Failed to fetch room stats: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "data": response.data
    })))
}
