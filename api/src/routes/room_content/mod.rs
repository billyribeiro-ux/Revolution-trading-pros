//! Room Content Management API - Trade Plans, Alerts, Weekly Videos
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 13, 2026
//!
//! Complete CRUD API for:
//! - Trade Plan entries (ticker, bias, entry, targets, stop, options, notes)
//! - Alerts (Entry/Exit/Update with expandable notes)
//! - Weekly Videos (featured video per room with auto-archive)
//! - Stats cache (auto-calculated performance metrics)
//!
//! ## Phase 2: Redis Caching (January 2026)
//!
//! All GET endpoints now use Redis caching with graceful degradation.
//! Cache invalidation is triggered automatically on mutations.
//!
//! ## R10-B maintainability split (2026-05-20)
//!
//! Split from a single 2,224-LOC `room_content.rs` file. Public API
//! unchanged: external callers still see
//! `routes::room_content::public_router()`,
//! `routes::room_content::admin_router()`, and every previously-public
//! DTO (`TradePlanEntry`, `RoomAlert`, `WeeklyVideo`, `RoomStats`,
//! `RoomTrade`, and all `Create*Request` / `Update*Request` types).

use axum::{
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{models::User, AppState};

pub mod alerts;
pub mod stats;
pub mod trade_plans;
pub mod trades;
pub mod weekly_videos;

// Re-export weekly-video-specific types at the parent path so the
// pre-split public API (`room_content::ArchiveQuery`,
// `room_content::ArchivedWeekResponse`) keeps resolving.
pub use weekly_videos::{ArchiveQuery, ArchivedWeekResponse};

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Trade Plan Entry from database
/// ICT 7+ Phase 2: Added Deserialize for Redis caching support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
/// ICT 7+ Phase 2: Added Deserialize for Redis caching support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
/// ICT 7+ Phase 2: Added Deserialize for Redis caching support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
/// ICT 7+ Phase 2: Added Deserialize for Redis caching support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
/// ICT 7+ Phase 2: Added Deserialize for Redis caching support
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
// SHARED INTERNAL TYPES (used by multiple sub-modules)
// ═══════════════════════════════════════════════════════════════════════════════════

/// Pagination metadata (shared by list responses across sub-modules)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PaginationMeta {
    pub(crate) current_page: i64,
    pub(crate) per_page: i64,
    pub(crate) total: i64,
    pub(crate) total_pages: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SHARED HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// NF-1 (P0): room content (trade plans, alerts, trades, weekly videos,
/// stats) is the paid product. The `public_router` handlers were mounted
/// with NO authentication — served to anonymous internet users. This gate
/// requires an authenticated user who is either staff or holds an active /
/// trial membership. Fail-closed: a DB error never serves paid content.
pub(crate) async fn ensure_room_access(
    state: &AppState,
    user: &User,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("");
    if matches!(role, "admin" | "super_admin" | "super-admin" | "developer") {
        return Ok(());
    }
    let has_membership: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_memberships \
         WHERE user_id = $1 AND status IN ('active', 'trial', 'trialing') LIMIT 1",
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;
    if has_membership.is_some() {
        Ok(())
    } else {
        tracing::warn!(
            target: "security",
            event = "room_content_paywall_blocked",
            user_id = %user.id,
            "Non-member blocked from members-only room content"
        );
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Active membership required"})),
        ))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTER (Member access)
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        // Trade Plans (read-only for members)
        .route(
            "/rooms/:room_slug/trade-plan",
            get(trade_plans::list_trade_plans),
        )
        // Alerts (read-only for members)
        .route("/rooms/:room_slug/alerts", get(alerts::list_alerts))
        .route(
            "/rooms/:room_slug/alerts/:id/read",
            post(alerts::mark_alert_read),
        )
        // Trades (read-only for members)
        .route("/rooms/:room_slug/trades", get(trades::list_trades))
        // Weekly Video
        .route(
            "/rooms/:room_slug/weekly-video",
            get(weekly_videos::get_weekly_video),
        )
        .route(
            "/rooms/:room_slug/weekly-videos",
            get(weekly_videos::list_weekly_videos),
        )
        .route(
            "/weekly-video/:room_slug/archive",
            get(weekly_videos::list_archived_videos),
        )
        // Stats
        .route("/rooms/:room_slug/stats", get(stats::get_room_stats))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ADMIN ROUTER (Admin access required)
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Trade Plans CRUD
        .route(
            "/rooms/:room_slug/trade-plan",
            get(trade_plans::list_trade_plans),
        )
        .route("/trade-plan", post(trade_plans::create_trade_plan))
        .route("/trade-plan/:id", put(trade_plans::update_trade_plan))
        .route("/trade-plan/:id", delete(trade_plans::delete_trade_plan))
        .route(
            "/rooms/:room_slug/trade-plan/reorder",
            put(trade_plans::reorder_trade_plans),
        )
        // Alerts CRUD
        .route("/rooms/:room_slug/alerts", get(alerts::list_alerts))
        .route("/alerts", post(alerts::create_alert))
        .route("/alerts/:id", put(alerts::update_alert))
        .route("/alerts/:id", delete(alerts::delete_alert))
        // Trades CRUD (Trade Tracker)
        .route("/rooms/:room_slug/trades", get(trades::list_trades))
        .route("/trades", post(trades::create_trade))
        .route("/trades/:id", put(trades::update_trade))
        .route("/trades/:id/close", put(trades::close_trade))
        .route("/trades/:id/invalidate", post(trades::invalidate_trade))
        .route("/trades/:id", delete(trades::delete_trade))
        // Weekly Videos CRUD
        .route(
            "/rooms/:room_slug/weekly-video",
            get(weekly_videos::get_weekly_video),
        )
        .route(
            "/rooms/:room_slug/weekly-videos",
            get(weekly_videos::list_weekly_videos),
        )
        .route("/weekly-video", post(weekly_videos::create_weekly_video))
        // Stats
        .route("/rooms/:room_slug/stats", get(stats::get_room_stats))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    // ── TOS format string helpers ─────────────────────────────────────────────────
    // Mirrors the TosFormatter helpers that live in the integration test fixtures
    // (api/tests/explosive_swings/fixtures.rs).  The originals in that file are
    // commented out with a FIX-2026-04-26 marker.  These are the canonical
    // source-side copies; they only exist inside #[cfg(test)] because the
    // application itself stores the pre-formatted string rather than generating it.
    // FIX-2026-04-26: moved from api/tests/explosive_swings/alerts_test.rs

    #[allow(clippy::too_many_arguments)]
    fn tos_options(
        action: &str,
        quantity: i32,
        ticker: &str,
        shares_per_contract: i32,
        contract_type: &str,
        expiration: &NaiveDate,
        strike: f64,
        option_type: &str,
        order_type: &str,
        limit_price: Option<f64>,
    ) -> String {
        let qty_prefix = if action == "BUY" { "+" } else { "-" };
        let exp_str = expiration.format("%d %b %y").to_string().to_uppercase();
        let price_str = match (order_type, limit_price) {
            ("LMT", Some(price)) => format!("@{price:.2} LMT"),
            _ => "@MKT".to_string(),
        };
        format!(
            "{action} {qty_prefix}{quantity} {ticker} {shares_per_contract} ({contract_type}) {exp_str} {strike} {option_type} {price_str}"
        )
    }

    fn tos_shares(
        action: &str,
        quantity: i32,
        ticker: &str,
        order_type: &str,
        limit_price: Option<f64>,
    ) -> String {
        let qty_prefix = if action == "BUY" { "+" } else { "-" };
        let price_str = match (order_type, limit_price) {
            ("LMT", Some(price)) => format!("@{price:.2} LMT"),
            _ => "@MKT".to_string(),
        };
        format!("{action} {qty_prefix}{quantity} {ticker} {price_str}")
    }

    // ── TOS formatter unit tests ──────────────────────────────────────────────────

    #[test]
    fn test_tos_formatter_options_market_order() {
        let expiration = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
        let tos_string = tos_options(
            "BUY",
            1,
            "SPY",
            100,
            "Weeklys",
            &expiration,
            500.0,
            "CALL",
            "MKT",
            None,
        );
        assert!(tos_string.contains("BUY +1 SPY"));
        assert!(tos_string.contains("Weeklys"));
        assert!(tos_string.contains("500"));
        assert!(tos_string.contains("CALL"));
        assert!(tos_string.contains("@MKT"));
    }

    #[test]
    fn test_tos_formatter_options_limit_order() {
        let expiration = NaiveDate::from_ymd_opt(2026, 2, 21).unwrap();
        let tos_string = tos_options(
            "SELL",
            5,
            "AAPL",
            100,
            "Monthly",
            &expiration,
            175.0,
            "PUT",
            "LMT",
            Some(2.50),
        );
        assert!(tos_string.contains("SELL -5 AAPL"));
        assert!(tos_string.contains("Monthly"));
        assert!(tos_string.contains("175"));
        assert!(tos_string.contains("PUT"));
        assert!(tos_string.contains("@2.50 LMT"));
    }

    #[test]
    fn test_tos_formatter_shares() {
        let tos_string = tos_shares("BUY", 100, "GOOG", "MKT", None);
        assert_eq!(tos_string, "BUY +100 GOOG @MKT");
    }

    #[test]
    fn test_tos_formatter_shares_limit() {
        let tos_string = tos_shares("SELL", 50, "MSFT", "LMT", Some(400.00));
        assert_eq!(tos_string, "SELL -50 MSFT @400.00 LMT");
    }
}
