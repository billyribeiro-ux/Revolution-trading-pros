//! Paywall-gated ownership listing routes.
//!
//! - `get_my_indicators` — list a user's owned indicators
//! - `get_indicator_downloads` — show files + videos + ownership for one
//!   indicator the user owns
//!
//! Paywall invariant: every handler here takes `user: User` extractor
//! and checks `user_indicators` ownership before returning data. The
//! `Indicator.id (i64)` <-> `UserIndicatorOwnership.indicator_id (i64)`
//! FK chain is the gate — preserved byte-for-byte from the original.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::models::indicator::{Indicator, IndicatorFile, IndicatorVideo};
use crate::models::User;
use crate::AppState;

/// ICT 7 FIX: Uses i64 for indicator_id (matching database schema)
pub(super) async fn get_my_indicators(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // ICT 7 FIX: Database uses BIGINT (i64) for indicator_id, not UUID
    // Using user_indicators table which has user_id and indicator_id as i64
    let indicators: Vec<serde_json::Value> = sqlx::query_as::<
        _,
        (
            i64,                   // id
            String,                // name
            String,                // slug
            Option<String>,        // description (was tagline)
            Option<String>,        // thumbnail (was logo_url)
            Option<String>,        // download_url (was card_image_url)
            chrono::NaiveDateTime, // purchased_at (was access_granted_at)
        ),
    >(
        r"
        SELECT i.id, i.name, i.slug, i.description, i.thumbnail, i.download_url, ui.purchased_at
        FROM user_indicators ui
        JOIN indicators i ON ui.indicator_id = i.id
        WHERE ui.user_id = $1
        AND i.is_active = true
        ORDER BY ui.purchased_at DESC
        ",
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .into_iter()
    .map(
        |(id, name, slug, description, thumbnail, download_url, purchased_at)| {
            json!({
                "id": id.to_string(),
                "name": name,
                "slug": slug,
                "tagline": description,
                "logo_url": thumbnail,
                "card_image_url": thumbnail,
                "access_granted_at": purchased_at.and_utc().to_rfc3339()
            })
        },
    )
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": indicators
    })))
}

/// ICT 7 FIX: Uses i64 for indicator_id, queries user_indicators table
pub(super) async fn get_indicator_downloads(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get indicator
    let indicator: Indicator = sqlx::query_as("SELECT * FROM indicators WHERE slug = $1")
        .bind(&slug)
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
                Json(json!({"error": "Indicator not found"})),
            )
        })?;

    // ICT 7 FIX: Check ownership using user_indicators table (uses i64)
    let ownership: Option<(i64, i64, i64, chrono::NaiveDateTime, Option<String>)> = sqlx::query_as(
        r"
        SELECT id, user_id, indicator_id, purchased_at, license_key
        FROM user_indicators
        WHERE user_id = $1 AND indicator_id = $2
        ",
    )
    .bind(user_id)
    .bind(indicator.id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if ownership.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You do not own this indicator"})),
        ));
    }

    // ICT 7 FIX: Get files from indicator_files table (uses i64 indicator_id)
    let files: Vec<IndicatorFile> = sqlx::query_as(
        r"
        SELECT * FROM indicator_files
        WHERE indicator_id = $1 AND is_active = true
        ORDER BY platform, display_order
        ",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // ICT 7 FIX: Get videos - use indicator_videos table if exists
    let videos: Vec<IndicatorVideo> = sqlx::query_as(
        "SELECT * FROM indicator_videos WHERE indicator_id = $1 ORDER BY display_order",
    )
    .bind(indicator.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build ownership response
    let ownership_info = ownership.map(|(id, user_id, indicator_id, purchased_at, license_key)| {
        json!({
            "id": id,
            "user_id": user_id,
            "indicator_id": indicator_id,
            "access_granted_at": purchased_at.and_utc().to_rfc3339(),
            "license_key": license_key,
            "is_lifetime_access": true
        })
    });

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator": indicator,
            "files": files,
            "videos": videos,
            "ownership": ownership_info
        }
    })))
}
