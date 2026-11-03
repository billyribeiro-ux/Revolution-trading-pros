//! Trade handlers (list / create / close / invalidate / update / delete)
//!
//! "Trade Tracker" feature: tracks executed trades with entry/exit prices,
//! P&L calculation, and status lifecycle (open → closed | invalidated).
//!
//! Split from `room_content.rs` (R10-B maintainability pass, 2026-05-20).
//! Behavior unchanged: routes are still wired through `mod.rs`'s
//! `public_router()` / `admin_router()` exactly as before.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info};

use super::{
    CloseTradeRequest, CreateTradeRequest, InvalidateTradeRequest, PaginationMeta, RoomTrade,
    TradeListQuery, UpdateTradeRequest, ensure_room_access,
};
use crate::cache::{cache_keys, cache_ttl};
use crate::{AppState, middleware::admin::AdminUser, models::User};

macro_rules! room_trade_cols {
    () => {
        "id, COALESCE(room_id, 0) AS room_id, room_slug, ticker, trade_type, direction, quantity, \
         option_type, strike::float8 AS strike, expiration, contract_type, entry_alert_id, \
         entry_price::float8 AS entry_price, entry_date, entry_tos_string, exit_alert_id, \
         exit_price::float8 AS exit_price, exit_date, exit_tos_string, setup, status, result, \
         pnl::float8 AS pnl, pnl_percent::float8 AS pnl_percent, holding_days, notes, \
         was_updated, invalidation_reason, created_at, updated_at"
    };
}

/// Cached response for trades list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TradesResponse {
    data: Vec<RoomTrade>,
    meta: PaginationMeta,
}

/// List trades for a room
/// ICT 7+ Phase 2: Redis cached with 5 minute TTL
pub(super) async fn list_trades(
    State(state): State<AppState>,
    user: User,
    Path(room_slug): Path<String>,
    Query(query): Query<TradeListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    ensure_room_access(&state, &user).await?;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Generate cache key (status filter included, ticker not cached due to dynamic nature)
    let cache_key = cache_keys::trades(&room_slug, query.status.as_deref(), page, per_page);

    // If ticker filter is used, bypass cache (rare use case)
    if let Some(ticker_ref) = &query.ticker {
        let ticker = ticker_ref.to_uppercase();
        let trades: Vec<RoomTrade> = match &query.status {
            Some(status) => {
                sqlx::query_as(concat!(
                    "SELECT ",
                    room_trade_cols!(),
                    " FROM room_trades
                     WHERE room_slug = $1 AND deleted_at IS NULL AND status = $4 AND ticker = $5
                     ORDER BY entry_date DESC
                     LIMIT $2 OFFSET $3"
                ))
                .bind(&room_slug)
                .bind(per_page)
                .bind(offset)
                .bind(status)
                .bind(&ticker)
                .fetch_all(&state.db.pool)
                .await
            }
            None => {
                sqlx::query_as(concat!(
                    "SELECT ",
                    room_trade_cols!(),
                    " FROM room_trades
                     WHERE room_slug = $1 AND deleted_at IS NULL AND ticker = $4
                     ORDER BY entry_date DESC
                     LIMIT $2 OFFSET $3"
                ))
                .bind(&room_slug)
                .bind(per_page)
                .bind(offset)
                .bind(&ticker)
                .fetch_all(&state.db.pool)
                .await
            }
        }
        .map_err(|e| {
            error!("Failed to fetch trades: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM room_trades WHERE room_slug = $1 AND deleted_at IS NULL",
        )
        .bind(&room_slug)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        return Ok(Json(json!({
            "data": trades,
            "meta": {
                "current_page": page,
                "per_page": per_page,
                "total": total.0,
                "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
            }
        })));
    }

    // Use cache for non-ticker filtered queries
    let status_filter = query.status.clone();
    let response = state
        .services
        .cache
        .get_or_fetch(&cache_key, cache_ttl::TRADES, || async {
            let trades: Vec<RoomTrade> = match &status_filter {
                Some(status) => {
                    sqlx::query_as(concat!(
                        "SELECT ",
                        room_trade_cols!(),
                        " FROM room_trades
                         WHERE room_slug = $1 AND deleted_at IS NULL AND status = $4
                         ORDER BY entry_date DESC
                         LIMIT $2 OFFSET $3"
                    ))
                    .bind(&room_slug)
                    .bind(per_page)
                    .bind(offset)
                    .bind(status)
                    .fetch_all(&state.db.pool)
                    .await?
                }
                None => {
                    sqlx::query_as(concat!(
                        "SELECT ",
                        room_trade_cols!(),
                        " FROM room_trades
                         WHERE room_slug = $1 AND deleted_at IS NULL
                         ORDER BY entry_date DESC
                         LIMIT $2 OFFSET $3"
                    ))
                    .bind(&room_slug)
                    .bind(per_page)
                    .bind(offset)
                    .fetch_all(&state.db.pool)
                    .await?
                }
            };

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM room_trades WHERE room_slug = $1 AND deleted_at IS NULL",
            )
            .bind(&room_slug)
            .fetch_one(&state.db.pool)
            .await?;

            Ok(TradesResponse {
                data: trades,
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
            error!("Failed to fetch trades: {}", e);
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

/// Create a new trade
/// ICT 7+ Phase 2: Invalidates trades cache after creation
pub(super) async fn create_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let entry_date = NaiveDate::parse_from_str(&input.entry_date, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid entry_date format. Expected YYYY-MM-DD"})),
        )
    })?;

    let expiration = input
        .expiration
        .as_ref()
        .and_then(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok());

    let trade: RoomTrade = sqlx::query_as(concat!(
        "INSERT INTO room_trades
           (room_id, room_slug, ticker, trade_type, direction, quantity,
            option_type, strike, expiration, contract_type,
            entry_alert_id, entry_price, entry_date, entry_tos_string,
            setup, status, notes)
           VALUES (
               COALESCE((SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1), 0),
               $1, $2, $3, $4, $5,
               $6, $7, $8, $9,
               $10, $11, $12, $13,
               $14, 'open', $15
           )
           RETURNING ",
        room_trade_cols!()
    ))
    .bind(&input.room_slug)
    .bind(input.ticker.to_uppercase())
    .bind(&input.trade_type)
    .bind(&input.direction)
    .bind(input.quantity)
    .bind(input.option_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.strike)
    .bind(expiration)
    .bind(&input.contract_type)
    .bind(input.entry_alert_id)
    .bind(input.entry_price)
    .bind(entry_date)
    .bind(&input.entry_tos_string)
    .bind(&input.setup)
    .bind(&input.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create trade: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate trades cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_change(&input.room_slug, Some(trade.id))
        .await
    {
        error!("Failed to invalidate trades cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_created(crate::routes::websocket::TradePayload {
            id: trade.id,
            room_slug: trade.room_slug.clone(),
            ticker: trade.ticker.clone(),
            direction: trade.direction.clone(),
            status: trade.status.clone(),
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            pnl_percent: trade.pnl_percent,
            result: trade.result.clone(),
            invalidation_reason: trade.invalidation_reason.clone(),
            was_updated: trade.was_updated,
            entry_date: trade.entry_date.to_string(),
            exit_date: trade.exit_date.map(|d| d.to_string()),
        })
        .await;

    info!(
        "Created trade: {} {} for {}",
        trade.ticker, trade.direction, input.room_slug
    );
    Ok(Json(trade))
}

/// Close a trade
/// ICT 7+ Phase 2: Invalidates trades cache after closing
pub(super) async fn close_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CloseTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let exit_date = input
        .exit_date
        .as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    // First get the trade to calculate P&L
    let existing: RoomTrade = sqlx::query_as(concat!(
        "SELECT ",
        room_trade_cols!(),
        " FROM room_trades WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Trade not found: {}", e);
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Trade not found"})),
        )
    })?;

    if existing.status != "open" {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": format!("Trade is already {}", existing.status)})),
        ));
    }

    // Calculate P&L (case-insensitive direction check)
    let price_delta = if existing.direction.eq_ignore_ascii_case("long") {
        input.exit_price - existing.entry_price
    } else {
        existing.entry_price - input.exit_price
    };
    let pnl = price_delta * existing.quantity as f64;

    let pnl_percent = if existing.entry_price > 0.0 {
        (price_delta / existing.entry_price) * 100.0
    } else {
        0.0
    };
    let holding_days = (exit_date - existing.entry_date).num_days() as i32;
    let result = if pnl > 0.0 { "WIN" } else { "LOSS" };

    let trade: RoomTrade = sqlx::query_as(concat!(
        "UPDATE room_trades SET
           exit_alert_id = $2,
           exit_price = $3,
           exit_date = $4,
           exit_tos_string = $5,
           status = 'closed',
           result = $6,
           pnl = $7,
           pnl_percent = $8,
           holding_days = $9,
           notes = COALESCE($10, notes),
           updated_at = NOW()
           WHERE id = $1 AND status = 'open' AND deleted_at IS NULL
           RETURNING ",
        room_trade_cols!()
    ))
    .bind(id)
    .bind(input.exit_alert_id)
    .bind(input.exit_price)
    .bind(exit_date)
    .bind(&input.exit_tos_string)
    .bind(result)
    .bind(pnl)
    .bind(pnl_percent)
    .bind(holding_days)
    .bind(&input.notes)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to close trade: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::CONFLICT,
            Json(json!({"error": "Trade is already closed or invalidated"})),
        )
    })?;

    // Invalidate trades cache (closing affects stats)
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_change(&trade.room_slug, Some(id))
        .await
    {
        error!("Failed to invalidate trades cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_closed(crate::routes::websocket::TradePayload {
            id: trade.id,
            room_slug: trade.room_slug.clone(),
            ticker: trade.ticker.clone(),
            direction: trade.direction.clone(),
            status: trade.status.clone(),
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            pnl_percent: trade.pnl_percent,
            result: trade.result.clone(),
            invalidation_reason: trade.invalidation_reason.clone(),
            was_updated: trade.was_updated,
            entry_date: trade.entry_date.to_string(),
            exit_date: trade.exit_date.map(|d| d.to_string()),
        })
        .await;

    info!(
        "Closed trade {}: {} {} P&L: ${:.2}",
        id, trade.ticker, result, pnl
    );
    Ok(Json(trade))
}

/// Invalidate a trade (for setups that didn't trigger)
/// ICT 7+ Phase 2: Invalidates trades cache after invalidation
pub(super) async fn invalidate_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<InvalidateTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let trade: RoomTrade = sqlx::query_as(concat!(
        "UPDATE room_trades SET
           status = 'invalidated',
           invalidation_reason = $2,
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING ",
        room_trade_cols!()
    ))
    .bind(id)
    .bind(&input.reason)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to invalidate trade: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate trades cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_change(&trade.room_slug, Some(id))
        .await
    {
        error!("Failed to invalidate trades cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_invalidated(crate::routes::websocket::TradePayload {
            id: trade.id,
            room_slug: trade.room_slug.clone(),
            ticker: trade.ticker.clone(),
            direction: trade.direction.clone(),
            status: trade.status.clone(),
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            pnl_percent: trade.pnl_percent,
            result: trade.result.clone(),
            invalidation_reason: trade.invalidation_reason.clone(),
            was_updated: trade.was_updated,
            entry_date: trade.entry_date.to_string(),
            exit_date: trade.exit_date.map(|d| d.to_string()),
        })
        .await;

    info!(
        "Invalidated trade {}: {} - {}",
        id, trade.ticker, input.reason
    );
    Ok(Json(trade))
}

/// Update a trade (mark as updated)
/// ICT 7+ Phase 2: Invalidates trades cache after update
pub(super) async fn update_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let trade: RoomTrade = sqlx::query_as(concat!(
        "UPDATE room_trades SET
           ticker = COALESCE($2, ticker),
           entry_price = COALESCE($3, entry_price),
           quantity = COALESCE($4, quantity),
           notes = COALESCE($5, notes),
           was_updated = true,
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING ",
        room_trade_cols!()
    ))
    .bind(id)
    .bind(input.ticker.map(|t| t.to_uppercase()))
    .bind(input.entry_price)
    .bind(input.quantity)
    .bind(&input.notes)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update trade: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Invalidate trades cache
    if let Err(e) = state
        .services
        .cache_invalidator
        .on_trade_change(&trade.room_slug, Some(id))
        .await
    {
        error!("Failed to invalidate trades cache: {}", e);
    }

    // ICT 7+ Phase 3: Broadcast WebSocket event for real-time updates
    state
        .ws_manager
        .broadcast_trade_updated(crate::routes::websocket::TradePayload {
            id: trade.id,
            room_slug: trade.room_slug.clone(),
            ticker: trade.ticker.clone(),
            direction: trade.direction.clone(),
            status: trade.status.clone(),
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            pnl_percent: trade.pnl_percent,
            result: trade.result.clone(),
            invalidation_reason: trade.invalidation_reason.clone(),
            was_updated: trade.was_updated,
            entry_date: trade.entry_date.to_string(),
            exit_date: trade.exit_date.map(|d| d.to_string()),
        })
        .await;

    info!("Updated trade {}: {}", id, trade.ticker);
    Ok(Json(trade))
}

/// Delete a trade (soft delete)
/// ICT 7+ Phase 2: Invalidates trades cache after deletion
pub(super) async fn delete_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get room_slug before deletion for cache invalidation
    let entry: Option<(String,)> =
        sqlx::query_as("SELECT room_slug FROM room_trades WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    sqlx::query("UPDATE room_trades SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Invalidate trades cache
    if let Some((room_slug,)) = entry
        && let Err(e) = state
            .services
            .cache_invalidator
            .on_trade_change(&room_slug, Some(id))
            .await
    {
        error!("Failed to invalidate trades cache: {}", e);
    }

    Ok(Json(json!({"success": true, "message": "Trade deleted"})))
}
