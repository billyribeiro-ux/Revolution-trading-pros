//! Stats handlers (get cached room performance stats)
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use super::{ensure_room_access, RoomStats};
use crate::cache::{cache_keys, cache_ttl};
use crate::{models::User, AppState};

/// Cached response for room stats
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatsResponse {
    data: Option<RoomStats>,
}

/// Get room stats
/// ICT 7+ Phase 2: Redis cached with 5 minute TTL
pub(super) async fn get_room_stats(
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
            let stats: Option<RoomStats> =
                sqlx::query_as("SELECT * FROM room_stats_cache WHERE room_slug = $1")
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
