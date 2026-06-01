//! Trade Plan handlers (list / create / update / delete / reorder)
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error, info};

use super::{
    ensure_room_access, CreateTradePlanRequest, ListQuery, PaginationMeta, ReorderRequest,
    TradePlanEntry, UpdateTradePlanRequest,
};
use crate::cache::{cache_keys, cache_ttl};
use crate::{middleware::admin::AdminUser, models::User, AppState};

/// Cached response for trade plans list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TradePlansResponse {
    data: Vec<TradePlanEntry>,
    meta: PaginationMeta,
}

/// List trade plan entries for a room
/// ICT 7+ Phase 2: Redis cached with graceful degradation
pub(super) async fn list_trade_plans(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // Parse week_of or use current date
    let week_filter = query.week_of.as_deref();

    // Generate cache key
    let cache_key = cache_keys::trade_plans(&room_slug, week_filter, page, per_page);

    // Try to get from cache
    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::TRADE_PLANS, || async {
            let week_parsed = week_filter.and_then(|w| NaiveDate::parse_from_str(w, "%Y-%m-%d").ok());

            let entries: Vec<TradePlanEntry> = if let Some(week) = week_parsed {
                sqlx::query_as(
                    r"SELECT * FROM room_trade_plans
                       WHERE room_slug = $1 AND week_of = $2 AND deleted_at IS NULL AND is_active = true
                       ORDER BY sort_order ASC, created_at DESC
                       LIMIT $3 OFFSET $4"
                )
                .bind(&room_slug)
                .bind(week)
                .bind(per_page)
                .bind(offset)
                .fetch_all(&state.db.pool)
                .await?
            } else {
                // Get current week's entries
                sqlx::query_as(
                    r"SELECT * FROM room_trade_plans
                       WHERE room_slug = $1 AND deleted_at IS NULL AND is_active = true
                       AND week_of = (SELECT MAX(week_of) FROM room_trade_plans WHERE room_slug = $1 AND deleted_at IS NULL)
                       ORDER BY sort_order ASC, created_at DESC
                       LIMIT $2 OFFSET $3"
                )
                .bind(&room_slug)
                .bind(per_page)
                .bind(offset)
                .fetch_all(&state.db.pool)
                .await?
            };

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM room_trade_plans WHERE room_slug = $1 AND deleted_at IS NULL AND is_active = true"
            )
            .bind(&room_slug)
            .fetch_one(&state.db.pool)
            .await?;

            Ok(TradePlansResponse {
                data: entries,
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
            error!("Failed to fetch trade plans: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    debug!(
        target: "cache",
        room_slug = %room_slug,
        cache_key = %cache_key,
        "Trade plans response served"
    );

    Ok(Json(json!({
        "data": response.data,
        "meta": response.meta
    })))
}

/// Create a new trade plan entry
/// ICT 7+ Phase 2: Invalidates trade plans cache after creation
pub(super) async fn create_trade_plan(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTradePlanRequest>,
) -> Result<Json<TradePlanEntry>, (StatusCode, Json<serde_json::Value>)> {
    let week_of = input
        .week_of
        .and_then(|w| NaiveDate::parse_from_str(&w, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    let options_exp = input
        .options_exp
        .and_then(|e| NaiveDate::parse_from_str(&e, "%Y-%m-%d").ok());

    let entry: TradePlanEntry = sqlx::query_as(
        r"INSERT INTO room_trade_plans
           (room_id, room_slug, week_of, ticker, bias, entry, target1, target2, target3, runner, runner_stop, stop, options_strike, options_exp, notes, sort_order)
           VALUES (
               COALESCE(
                   (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
                   (SELECT id FROM membership_plans WHERE slug = $1 LIMIT 1),
                   0
               ),
               $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
           )
           RETURNING *"
    )
    .bind(&input.room_slug)
    .bind(week_of)
    .bind(input.ticker.to_uppercase())
    .bind(input.bias.to_uppercase())
    .bind(&input.entry)
    .bind(&input.target1)
    .bind(&input.target2)
    .bind(&input.target3)
    .bind(&input.runner)
    .bind(&input.runner_stop)
    .bind(&input.stop)
    .bind(&input.options_strike)
    .bind(options_exp)
    .bind(&input.notes)
    .bind(input.sort_order.unwrap_or(0))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create trade plan: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    // Invalidate trade plans cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_plan_change(&input.room_slug, Some(entry.id))
        .await
    {
        error!("Failed to invalidate trade plans cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_plan_created(crate::routes::websocket::TradePlanPayload {
            id: entry.id,
            room_slug: entry.room_slug.clone(),
            ticker: entry.ticker.clone(),
            bias: entry.bias.clone(),
            entry: entry.entry.clone(),
            target1: entry.target1.clone(),
            target2: entry.target2.clone(),
            target3: entry.target3.clone(),
            runner: entry.runner.clone(),
            stop: entry.stop.clone(),
            options_strike: entry.options_strike.clone(),
            options_exp: entry.options_exp.map(|d| d.to_string()),
            notes: entry.notes.clone(),
        })
        .await;

    info!(
        "Created trade plan entry: {} for {}",
        entry.ticker, input.room_slug
    );
    Ok(Json(entry))
}

/// Update a trade plan entry
/// ICT 7+ Phase 2: Invalidates trade plans cache after update
pub(super) async fn update_trade_plan(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTradePlanRequest>,
) -> Result<Json<TradePlanEntry>, (StatusCode, Json<serde_json::Value>)> {
    let options_exp = input
        .options_exp
        .and_then(|e| NaiveDate::parse_from_str(&e, "%Y-%m-%d").ok());

    let entry: TradePlanEntry = sqlx::query_as(
        r"UPDATE room_trade_plans SET
           ticker = COALESCE($2, ticker),
           bias = COALESCE($3, bias),
           entry = COALESCE($4, entry),
           target1 = COALESCE($5, target1),
           target2 = COALESCE($6, target2),
           target3 = COALESCE($7, target3),
           runner = COALESCE($8, runner),
           runner_stop = COALESCE($9, runner_stop),
           stop = COALESCE($10, stop),
           options_strike = COALESCE($11, options_strike),
           options_exp = COALESCE($12, options_exp),
           notes = COALESCE($13, notes),
           sort_order = COALESCE($14, sort_order),
           is_active = COALESCE($15, is_active)
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *",
    )
    .bind(id)
    .bind(input.ticker.map(|t| t.to_uppercase()))
    .bind(input.bias.map(|b| b.to_uppercase()))
    .bind(&input.entry)
    .bind(&input.target1)
    .bind(&input.target2)
    .bind(&input.target3)
    .bind(&input.runner)
    .bind(&input.runner_stop)
    .bind(&input.stop)
    .bind(&input.options_strike)
    .bind(options_exp)
    .bind(&input.notes)
    .bind(input.sort_order)
    .bind(input.is_active)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update trade plan: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate trade plans cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_plan_change(&entry.room_slug, Some(id))
        .await
    {
        error!("Failed to invalidate trade plans cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_plan_updated(crate::routes::websocket::TradePlanPayload {
            id: entry.id,
            room_slug: entry.room_slug.clone(),
            ticker: entry.ticker.clone(),
            bias: entry.bias.clone(),
            entry: entry.entry.clone(),
            target1: entry.target1.clone(),
            target2: entry.target2.clone(),
            target3: entry.target3.clone(),
            runner: entry.runner.clone(),
            stop: entry.stop.clone(),
            options_strike: entry.options_strike.clone(),
            options_exp: entry.options_exp.map(|d| d.to_string()),
            notes: entry.notes.clone(),
        })
        .await;

    Ok(Json(entry))
}

/// Delete a trade plan entry (soft delete)
/// ICT 7+ Phase 2: Invalidates trade plans cache after deletion
pub(super) async fn delete_trade_plan(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get room_slug before deletion for cache invalidation
    let entry: Option<(String,)> =
        sqlx::query_as("SELECT room_slug FROM room_trade_plans WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    sqlx::query("UPDATE room_trade_plans SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Invalidate trade plans cache and broadcast WebSocket event
    if let Some((room_slug,)) = entry {
        if let Err(e) = state
            .services
            .cache_invalidator
            .on_trade_plan_change(&room_slug, Some(id))
            .await
        {
            error!("Failed to invalidate trade plans cache: {}", e);
        }

        // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
        state
            .ws_manager
            .broadcast_trade_plan_deleted(&room_slug, id)
            .await;
    }

    Ok(Json(
        json!({"success": true, "message": "Trade plan entry deleted"}),
    ))
}

/// Reorder trade plan entries
/// ICT 7+ Phase 2: Invalidates trade plans cache after reorder
pub(super) async fn reorder_trade_plans(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(room_slug): Path<String>,
    Json(input): Json<ReorderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for item in input.items {
        sqlx::query("UPDATE room_trade_plans SET sort_order = $2 WHERE id = $1 AND room_slug = $3")
            .bind(item.id)
            .bind(item.sort_order)
            .bind(&room_slug)
            .execute(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;
    }

    // Invalidate trade plans cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .invalidate_trade_plans(&room_slug)
        .await
    {
        error!("Failed to invalidate trade plans cache: {}", e);
    }

    Ok(Json(
        json!({"success": true, "message": "Trade plans reordered"}),
    ))
}
