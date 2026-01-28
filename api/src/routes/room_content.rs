//! Room Content Management API - Trade Plans, Alerts, Weekly Videos
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 13, 2026
//!
//! Complete CRUD API for:
//! - Trade Plan entries (ticker, bias, entry, targets, stop, options, notes)
//! - Alerts (Entry/Exit/Update with expandable notes)
//! - Weekly Videos (featured video per room with auto-archive)
//! - Stats cache (auto-calculated performance metrics)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tracing::{error, info};

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Trade Plan Entry from database
#[derive(Debug, Serialize, FromRow)]
pub struct TradePlanEntry {
    pub id: i64,
    pub room_id: i64,
    pub room_slug: String,
    pub week_of: NaiveDate,
    pub ticker: String,
    pub bias: String,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub target2: Option<String>,
    pub target3: Option<String>,
    pub runner: Option<String>,
    pub runner_stop: Option<String>,
    pub stop: Option<String>,
    pub options_strike: Option<String>,
    pub options_exp: Option<NaiveDate>,
    pub notes: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Alert from database - Full TOS (ThinkOrSwim) Format Support
#[derive(Debug, Serialize, FromRow)]
pub struct RoomAlert {
    pub id: i64,
    pub room_id: i64,
    pub room_slug: String,
    pub alert_type: String,
    pub ticker: String,
    pub title: Option<String>,
    pub message: String,
    pub notes: Option<String>,
    // TOS Format Fields
    pub trade_type: Option<String>, // 'options' | 'shares'
    pub action: Option<String>,     // 'BUY' | 'SELL'
    pub quantity: Option<i32>,
    pub option_type: Option<String>, // 'CALL' | 'PUT'
    pub strike: Option<f64>,
    pub expiration: Option<NaiveDate>,
    pub contract_type: Option<String>, // 'Weeklys' | 'Monthly' | 'LEAPS'
    pub order_type: Option<String>,    // 'MKT' | 'LMT'
    pub limit_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub tos_string: Option<String>, // Full TOS format string
    // Linking
    pub entry_alert_id: Option<i64>,
    pub trade_plan_id: Option<i64>,
    // Status
    pub is_new: bool,
    pub is_published: bool,
    pub is_pinned: bool,
    pub published_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Weekly Video from database
#[derive(Debug, Serialize, FromRow)]
pub struct WeeklyVideo {
    pub id: i64,
    pub room_id: i64,
    pub room_slug: String,
    pub week_of: NaiveDate,
    pub week_title: String,
    pub video_title: String,
    pub video_url: String,
    pub video_platform: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
    pub is_current: bool,
    pub is_published: bool,
    pub published_at: chrono::DateTime<Utc>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Room Stats from database - Enhanced metrics
#[derive(Debug, Serialize, FromRow)]
pub struct RoomStats {
    pub id: i64,
    pub room_id: i64,
    pub room_slug: String,
    pub win_rate: Option<f64>,
    pub weekly_profit: Option<String>,
    pub monthly_profit: Option<String>,
    pub active_trades: Option<i32>,
    pub closed_this_week: Option<i32>,
    pub total_trades: Option<i32>,
    pub wins: Option<i32>,
    pub losses: Option<i32>,
    pub avg_win: Option<f64>,
    pub avg_loss: Option<f64>,
    pub profit_factor: Option<f64>,
    pub avg_holding_days: Option<f64>,
    pub largest_win: Option<f64>,
    pub largest_loss: Option<f64>,
    pub current_streak: Option<i32>,
    pub calculated_at: chrono::DateTime<Utc>,
}

/// Trade from database - Trade Tracker
#[derive(Debug, Serialize, FromRow)]
pub struct RoomTrade {
    pub id: i64,
    pub room_id: i64,
    pub room_slug: String,
    pub ticker: String,
    pub trade_type: String,
    pub direction: String,
    pub quantity: i32,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<NaiveDate>,
    pub contract_type: Option<String>,
    pub entry_alert_id: Option<i64>,
    pub entry_price: f64,
    pub entry_date: NaiveDate,
    pub entry_tos_string: Option<String>,
    pub exit_alert_id: Option<i64>,
    pub exit_price: Option<f64>,
    pub exit_date: Option<NaiveDate>,
    pub exit_tos_string: Option<String>,
    pub setup: Option<String>,
    pub status: String,
    pub result: Option<String>,
    pub pnl: Option<f64>,
    pub pnl_percent: Option<f64>,
    pub holding_days: Option<i32>,
    pub notes: Option<String>,
    pub was_updated: Option<bool>,
    pub invalidation_reason: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub week_of: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTradePlanRequest {
    pub room_slug: String,
    pub week_of: Option<String>,
    pub ticker: String,
    pub bias: String,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub target2: Option<String>,
    pub target3: Option<String>,
    pub runner: Option<String>,
    pub runner_stop: Option<String>,
    pub stop: Option<String>,
    pub options_strike: Option<String>,
    pub options_exp: Option<String>,
    pub notes: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTradePlanRequest {
    pub ticker: Option<String>,
    pub bias: Option<String>,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub target2: Option<String>,
    pub target3: Option<String>,
    pub runner: Option<String>,
    pub runner_stop: Option<String>,
    pub stop: Option<String>,
    pub options_strike: Option<String>,
    pub options_exp: Option<String>,
    pub notes: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub room_slug: String,
    pub alert_type: String,
    pub ticker: String,
    pub title: String,
    pub message: String,
    pub notes: Option<String>,
    // TOS Format Fields
    pub trade_type: Option<String>,
    pub action: Option<String>,
    pub quantity: Option<i32>,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<String>,
    pub contract_type: Option<String>,
    pub order_type: Option<String>,
    pub limit_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub tos_string: Option<String>,
    // Linking
    pub entry_alert_id: Option<i64>,
    pub trade_plan_id: Option<i64>,
    // Status
    pub is_new: Option<bool>,
    pub is_published: Option<bool>,
    pub is_pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAlertRequest {
    pub alert_type: Option<String>,
    pub ticker: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub notes: Option<String>,
    // TOS Format Fields
    pub trade_type: Option<String>,
    pub action: Option<String>,
    pub quantity: Option<i32>,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<String>,
    pub contract_type: Option<String>,
    pub order_type: Option<String>,
    pub limit_price: Option<f64>,
    pub fill_price: Option<f64>,
    pub tos_string: Option<String>,
    // Linking
    pub entry_alert_id: Option<i64>,
    pub trade_plan_id: Option<i64>,
    // Status
    pub is_new: Option<bool>,
    pub is_published: Option<bool>,
    pub is_pinned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWeeklyVideoRequest {
    pub room_slug: String,
    pub week_of: String,
    pub week_title: String,
    pub video_title: String,
    pub video_url: String,
    pub video_platform: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTradeRequest {
    pub room_slug: String,
    pub ticker: String,
    pub trade_type: String,
    pub direction: String,
    pub quantity: i32,
    pub option_type: Option<String>,
    pub strike: Option<f64>,
    pub expiration: Option<String>,
    pub contract_type: Option<String>,
    pub entry_alert_id: Option<i64>,
    pub entry_price: f64,
    pub entry_date: String,
    pub entry_tos_string: Option<String>,
    pub setup: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CloseTradeRequest {
    pub exit_alert_id: Option<i64>,
    pub exit_price: f64,
    pub exit_date: Option<String>,
    pub exit_tos_string: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvalidateTradeRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTradeRequest {
    pub ticker: Option<String>,
    pub entry_price: Option<f64>,
    pub quantity: Option<i32>,
    pub stop: Option<f64>,
    pub target1: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TradeListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub ticker: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItem {
    pub id: i64,
    pub sort_order: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List trade plan entries for a room
async fn list_trade_plans(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // Parse week_of or use current date
    let week_filter = query
        .week_of
        .and_then(|w| NaiveDate::parse_from_str(&w, "%Y-%m-%d").ok());

    let entries: Vec<TradePlanEntry> = if let Some(week) = week_filter {
        sqlx::query_as(
            r#"SELECT * FROM room_trade_plans 
               WHERE room_slug = $1 AND week_of = $2 AND deleted_at IS NULL AND is_active = true
               ORDER BY sort_order ASC, created_at DESC
               LIMIT $3 OFFSET $4"#
        )
        .bind(&room_slug)
        .bind(week)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else {
        // Get current week's entries
        sqlx::query_as(
            r#"SELECT * FROM room_trade_plans 
               WHERE room_slug = $1 AND deleted_at IS NULL AND is_active = true
               AND week_of = (SELECT MAX(week_of) FROM room_trade_plans WHERE room_slug = $1 AND deleted_at IS NULL)
               ORDER BY sort_order ASC, created_at DESC
               LIMIT $2 OFFSET $3"#
        )
        .bind(&room_slug)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    }.map_err(|e| {
        error!("Failed to fetch trade plans: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM room_trade_plans WHERE room_slug = $1 AND deleted_at IS NULL AND is_active = true"
    )
    .bind(&room_slug)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": entries,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Create a new trade plan entry
async fn create_trade_plan(
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
        r#"INSERT INTO room_trade_plans 
           (room_id, room_slug, week_of, ticker, bias, entry, target1, target2, target3, runner, runner_stop, stop, options_strike, options_exp, notes, sort_order)
           VALUES (
               COALESCE(
                   (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
                   (SELECT id FROM membership_plans WHERE slug = $1 LIMIT 1),
                   0
               ),
               $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
           )
           RETURNING *"#
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

    info!(
        "Created trade plan entry: {} for {}",
        entry.ticker, input.room_slug
    );
    Ok(Json(entry))
}

/// Update a trade plan entry
async fn update_trade_plan(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTradePlanRequest>,
) -> Result<Json<TradePlanEntry>, (StatusCode, Json<serde_json::Value>)> {
    let options_exp = input
        .options_exp
        .and_then(|e| NaiveDate::parse_from_str(&e, "%Y-%m-%d").ok());

    let entry: TradePlanEntry = sqlx::query_as(
        r#"UPDATE room_trade_plans SET
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
           RETURNING *"#,
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

    Ok(Json(entry))
}

/// Delete a trade plan entry (soft delete)
async fn delete_trade_plan(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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

    Ok(Json(
        json!({"success": true, "message": "Trade plan entry deleted"}),
    ))
}

/// Reorder trade plan entries
async fn reorder_trade_plans(
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

    Ok(Json(
        json!({"success": true, "message": "Trade plans reordered"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERTS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// List alerts for a room
async fn list_alerts(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let alerts: Vec<RoomAlert> = sqlx::query_as(
        r#"SELECT * FROM room_alerts 
           WHERE room_slug = $1 AND deleted_at IS NULL AND is_published = true
           ORDER BY is_pinned DESC, published_at DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(&room_slug)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to fetch alerts: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM room_alerts WHERE room_slug = $1 AND deleted_at IS NULL",
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

    Ok(Json(json!({
        "data": alerts,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Create a new alert with full TOS support
async fn create_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateAlertRequest>,
) -> Result<Json<RoomAlert>, (StatusCode, Json<serde_json::Value>)> {
    // Parse expiration date if provided
    let expiration = input
        .expiration
        .as_ref()
        .and_then(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok());

    let alert: RoomAlert = sqlx::query_as(
        r#"INSERT INTO room_alerts
           (room_id, room_slug, alert_type, ticker, title, message, notes,
            trade_type, action, quantity, option_type, strike, expiration, contract_type,
            order_type, limit_price, fill_price, tos_string, entry_alert_id, trade_plan_id,
            is_new, is_published, is_pinned)
           VALUES (
               (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
               $1, $2, $3, $4, $5, $6,
               $7, $8, $9, $10, $11, $12, $13,
               $14, $15, $16, $17, $18, $19,
               $20, $21, $22
           )
           RETURNING *"#,
    )
    .bind(&input.room_slug)
    .bind(input.alert_type.to_uppercase())
    .bind(input.ticker.to_uppercase())
    .bind(&input.title)
    .bind(&input.message)
    .bind(&input.notes)
    .bind(&input.trade_type)
    .bind(input.action.as_ref().map(|a| a.to_uppercase()))
    .bind(input.quantity)
    .bind(input.option_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.strike)
    .bind(expiration)
    .bind(&input.contract_type)
    .bind(input.order_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.limit_price)
    .bind(input.fill_price)
    .bind(&input.tos_string)
    .bind(input.entry_alert_id)
    .bind(input.trade_plan_id)
    .bind(input.is_new.unwrap_or(true))
    .bind(input.is_published.unwrap_or(true))
    .bind(input.is_pinned.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create alert: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Created alert: {} {} for {} (TOS: {:?})",
        alert.alert_type, alert.ticker, input.room_slug, alert.tos_string
    );
    Ok(Json(alert))
}

/// Update an alert with full TOS support
async fn update_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateAlertRequest>,
) -> Result<Json<RoomAlert>, (StatusCode, Json<serde_json::Value>)> {
    // Parse expiration date if provided
    let expiration = input
        .expiration
        .as_ref()
        .and_then(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d").ok());

    let alert: RoomAlert = sqlx::query_as(
        r#"UPDATE room_alerts SET
           alert_type = COALESCE($2, alert_type),
           ticker = COALESCE($3, ticker),
           title = COALESCE($4, title),
           message = COALESCE($5, message),
           notes = COALESCE($6, notes),
           trade_type = COALESCE($7, trade_type),
           action = COALESCE($8, action),
           quantity = COALESCE($9, quantity),
           option_type = COALESCE($10, option_type),
           strike = COALESCE($11, strike),
           expiration = COALESCE($12, expiration),
           contract_type = COALESCE($13, contract_type),
           order_type = COALESCE($14, order_type),
           limit_price = COALESCE($15, limit_price),
           fill_price = COALESCE($16, fill_price),
           tos_string = COALESCE($17, tos_string),
           entry_alert_id = COALESCE($18, entry_alert_id),
           trade_plan_id = COALESCE($19, trade_plan_id),
           is_new = COALESCE($20, is_new),
           is_published = COALESCE($21, is_published),
           is_pinned = COALESCE($22, is_pinned),
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *"#,
    )
    .bind(id)
    .bind(input.alert_type.map(|t| t.to_uppercase()))
    .bind(input.ticker.map(|t| t.to_uppercase()))
    .bind(&input.title)
    .bind(&input.message)
    .bind(&input.notes)
    .bind(&input.trade_type)
    .bind(input.action.as_ref().map(|a| a.to_uppercase()))
    .bind(input.quantity)
    .bind(input.option_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.strike)
    .bind(expiration)
    .bind(&input.contract_type)
    .bind(input.order_type.as_ref().map(|o| o.to_uppercase()))
    .bind(input.limit_price)
    .bind(input.fill_price)
    .bind(&input.tos_string)
    .bind(input.entry_alert_id)
    .bind(input.trade_plan_id)
    .bind(input.is_new)
    .bind(input.is_published)
    .bind(input.is_pinned)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to update alert: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(alert))
}

/// Delete an alert (soft delete)
async fn delete_alert(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE room_alerts SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"success": true, "message": "Alert deleted"})))
}

/// Mark alert as read (not new)
async fn mark_alert_read(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("UPDATE room_alerts SET is_new = false WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"success": true})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// WEEKLY VIDEO HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get current weekly video for a room
async fn get_weekly_video(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let video: Option<WeeklyVideo> = sqlx::query_as(
        r#"SELECT * FROM room_weekly_videos 
           WHERE room_slug = $1 AND is_current = true AND is_published = true
           LIMIT 1"#,
    )
    .bind(&room_slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "data": video
    })))
}

/// List all weekly videos (including archived)
async fn list_weekly_videos(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let videos: Vec<WeeklyVideo> = sqlx::query_as(
        r#"SELECT * FROM room_weekly_videos 
           WHERE room_slug = $1
           ORDER BY week_of DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(&room_slug)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM room_weekly_videos WHERE room_slug = $1")
            .bind(&room_slug)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "data": videos,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}

/// Create/publish a new weekly video (archives the previous one)
async fn create_weekly_video(
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
        r#"INSERT INTO room_weekly_videos 
           (room_id, room_slug, week_of, week_title, video_title, video_url, video_platform, thumbnail_url, duration, description, is_current, is_published)
           VALUES (
               (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
               $1, $2, $3, $4, $5, $6, $7, $8, $9, true, true
           )
           RETURNING *"#
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

    info!(
        "Created weekly video: {} for {}",
        video.video_title, input.room_slug
    );
    Ok(Json(video))
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

/// List archived weekly videos for a room (past weeks only)
async fn list_archived_videos(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(query): Query<ArchiveQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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
        r#"SELECT 
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
        ORDER BY v.week_of DESC"#,
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
            r#"SELECT COUNT(*) FROM room_alerts 
               WHERE room_slug = $1 
               AND DATE_TRUNC('week', published_at) = DATE_TRUNC('week', $2::date)"#,
        )
        .bind(&room_slug)
        .bind(week_of)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

        // Get trade count and win rate for this week
        let trade_stats: (i64, Option<f64>) = sqlx::query_as(
            r#"SELECT 
                COUNT(*),
                ROUND(100.0 * COUNT(*) FILTER (WHERE result = 'WIN') / NULLIF(COUNT(*) FILTER (WHERE status = 'closed'), 0), 1)
               FROM room_trades 
               WHERE room_slug = $1 
               AND DATE_TRUNC('week', entry_date) = DATE_TRUNC('week', $2::date)
               AND deleted_at IS NULL"#
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

// ═══════════════════════════════════════════════════════════════════════════════════
// STATS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get room stats
async fn get_room_stats(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let stats: Option<RoomStats> =
        sqlx::query_as("SELECT * FROM room_stats_cache WHERE room_slug = $1")
            .bind(&room_slug)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "data": stats
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADES HANDLERS (Trade Tracker)
// ═══════════════════════════════════════════════════════════════════════════════════

/// List trades for a room
async fn list_trades(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(query): Query<TradeListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let trades: Vec<RoomTrade> = match (&query.status, &query.ticker) {
        (Some(status), Some(ticker)) => {
            sqlx::query_as(
                r#"SELECT * FROM room_trades
                   WHERE room_slug = $1 AND deleted_at IS NULL AND status = $4 AND ticker = $5
                   ORDER BY entry_date DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
            .bind(status)
            .bind(ticker.to_uppercase())
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(status), None) => {
            sqlx::query_as(
                r#"SELECT * FROM room_trades
                   WHERE room_slug = $1 AND deleted_at IS NULL AND status = $4
                   ORDER BY entry_date DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
            .bind(status)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, Some(ticker)) => {
            sqlx::query_as(
                r#"SELECT * FROM room_trades
                   WHERE room_slug = $1 AND deleted_at IS NULL AND ticker = $4
                   ORDER BY entry_date DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
            .bind(ticker.to_uppercase())
            .fetch_all(&state.db.pool)
            .await
        }
        (None, None) => {
            sqlx::query_as(
                r#"SELECT * FROM room_trades
                   WHERE room_slug = $1 AND deleted_at IS NULL
                   ORDER BY entry_date DESC
                   LIMIT $2 OFFSET $3"#,
            )
            .bind(&room_slug)
            .bind(per_page)
            .bind(offset)
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

    Ok(Json(json!({
        "data": trades,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Create a new trade
async fn create_trade(
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

    let trade: RoomTrade = sqlx::query_as(
        r#"INSERT INTO room_trades
           (room_id, room_slug, ticker, trade_type, direction, quantity,
            option_type, strike, expiration, contract_type,
            entry_alert_id, entry_price, entry_date, entry_tos_string,
            setup, status, notes)
           VALUES (
               (SELECT id FROM trading_rooms WHERE slug = $1 LIMIT 1),
               $1, $2, $3, $4, $5,
               $6, $7, $8, $9,
               $10, $11, $12, $13,
               $14, 'open', $15
           )
           RETURNING *"#,
    )
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

    info!(
        "Created trade: {} {} for {}",
        trade.ticker, trade.direction, input.room_slug
    );
    Ok(Json(trade))
}

/// Close a trade
async fn close_trade(
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
    let existing: RoomTrade =
        sqlx::query_as("SELECT * FROM room_trades WHERE id = $1 AND deleted_at IS NULL")
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

    // Calculate P&L
    let pnl = if existing.direction == "long" {
        (input.exit_price - existing.entry_price) * existing.quantity as f64
    } else {
        (existing.entry_price - input.exit_price) * existing.quantity as f64
    };

    let pnl_percent = ((input.exit_price - existing.entry_price) / existing.entry_price) * 100.0;
    let holding_days = (exit_date - existing.entry_date).num_days() as i32;
    let result = if pnl > 0.0 { "WIN" } else { "LOSS" };

    let trade: RoomTrade = sqlx::query_as(
        r#"UPDATE room_trades SET
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
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *"#,
    )
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
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to close trade: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Closed trade {}: {} {} P&L: ${:.2}",
        id, trade.ticker, result, pnl
    );
    Ok(Json(trade))
}

/// Invalidate a trade (for setups that didn't trigger)
async fn invalidate_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<InvalidateTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let trade: RoomTrade = sqlx::query_as(
        r#"UPDATE room_trades SET
           status = 'invalidated',
           invalidation_reason = $2,
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *"#,
    )
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

    info!(
        "Invalidated trade {}: {} - {}",
        id, trade.ticker, input.reason
    );
    Ok(Json(trade))
}

/// Update a trade (mark as updated)
async fn update_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTradeRequest>,
) -> Result<Json<RoomTrade>, (StatusCode, Json<serde_json::Value>)> {
    let trade: RoomTrade = sqlx::query_as(
        r#"UPDATE room_trades SET
           ticker = COALESCE($2, ticker),
           entry_price = COALESCE($3, entry_price),
           quantity = COALESCE($4, quantity),
           notes = COALESCE($5, notes),
           was_updated = true,
           updated_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
           RETURNING *"#,
    )
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

    info!("Updated trade {}: {}", id, trade.ticker);
    Ok(Json(trade))
}

/// Delete a trade (soft delete)
async fn delete_trade(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
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

    Ok(Json(json!({"success": true, "message": "Trade deleted"})))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTER (Member access)
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        // Trade Plans (read-only for members)
        .route("/rooms/:room_slug/trade-plan", get(list_trade_plans))
        // Alerts (read-only for members)
        .route("/rooms/:room_slug/alerts", get(list_alerts))
        .route("/rooms/:room_slug/alerts/:id/read", post(mark_alert_read))
        // Trades (read-only for members)
        .route("/rooms/:room_slug/trades", get(list_trades))
        // Weekly Video
        .route("/rooms/:room_slug/weekly-video", get(get_weekly_video))
        .route("/rooms/:room_slug/weekly-videos", get(list_weekly_videos))
        .route(
            "/weekly-video/:room_slug/archive",
            get(list_archived_videos),
        )
        // Stats
        .route("/rooms/:room_slug/stats", get(get_room_stats))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ADMIN ROUTER (Admin access required)
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Trade Plans CRUD
        .route("/rooms/:room_slug/trade-plan", get(list_trade_plans))
        .route("/trade-plan", post(create_trade_plan))
        .route("/trade-plan/:id", put(update_trade_plan))
        .route("/trade-plan/:id", delete(delete_trade_plan))
        .route(
            "/rooms/:room_slug/trade-plan/reorder",
            put(reorder_trade_plans),
        )
        // Alerts CRUD
        .route("/rooms/:room_slug/alerts", get(list_alerts))
        .route("/alerts", post(create_alert))
        .route("/alerts/:id", put(update_alert))
        .route("/alerts/:id", delete(delete_alert))
        // Trades CRUD (Trade Tracker)
        .route("/rooms/:room_slug/trades", get(list_trades))
        .route("/trades", post(create_trade))
        .route("/trades/:id", put(update_trade))
        .route("/trades/:id/close", put(close_trade))
        .route("/trades/:id/invalidate", post(invalidate_trade))
        .route("/trades/:id", delete(delete_trade))
        // Weekly Videos CRUD
        .route("/rooms/:room_slug/weekly-video", get(get_weekly_video))
        .route("/rooms/:room_slug/weekly-videos", get(list_weekly_videos))
        .route("/weekly-video", post(create_weekly_video))
        // Stats
        .route("/rooms/:room_slug/stats", get(get_room_stats))
}
