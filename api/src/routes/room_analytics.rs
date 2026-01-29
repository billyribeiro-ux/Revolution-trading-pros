//! Room Analytics Routes - Explosive Swings Performance Analytics API
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 11+ Grade - January 2026
//!
//! REST API endpoints for trading room performance analytics:
//!
//!   - GET /api/analytics/room/{slug} - Full room analytics
//!   - GET /api/analytics/room/{slug}/equity-curve - Equity curve data
//!   - GET /api/analytics/room/{slug}/ticker/{ticker} - Ticker-specific analytics
//!   - GET /api/analytics/room/{slug}/monthly - Monthly returns
//!   - GET /api/analytics/room/{slug}/tickers - All ticker performance
//!   - GET /api/analytics/room/{slug}/setups - Setup performance
//!
//! ═══════════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use tracing::error;

use crate::{
    services::room_analytics::{
        EquityPoint, MonthlyReturn, RoomAnalytics, RoomAnalyticsService, SetupPerformance,
        TickerPerformance,
    },
    utils::errors::ApiError,
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// QUERY PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Date range query parameters for analytics endpoints
#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    /// Start date filter (YYYY-MM-DD format)
    pub from: Option<String>,
    /// End date filter (YYYY-MM-DD format)
    pub to: Option<String>,
}

impl AnalyticsQuery {
    /// Parse from date string to NaiveDate
    pub fn parse_from(&self) -> Option<NaiveDate> {
        self.from
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }

    /// Parse to date string to NaiveDate
    pub fn parse_to(&self) -> Option<NaiveDate> {
        self.to
            .as_ref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// GET /api/analytics/room/{slug}
///
/// Get comprehensive analytics for a trading room including:
/// - Summary statistics (win rate, profit factor, drawdown)
/// - Ticker performance breakdown
/// - Setup performance analysis
/// - Monthly returns
/// - Equity curve
/// - Drawdown periods
///
/// # Query Parameters
/// - `from` - Optional start date (YYYY-MM-DD)
/// - `to` - Optional end date (YYYY-MM-DD)
///
/// # Response
/// Returns complete RoomAnalytics object with all metrics
async fn get_room_analytics(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<RoomAnalytics>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let analytics = service
        .get_room_analytics(&slug, query.parse_from(), query.parse_to())
        .await
        .map_err(|e| {
            error!("Failed to get room analytics for {}: {}", slug, e);
            ApiError::internal_error(&format!("Failed to get analytics: {}", e))
        })?;

    Ok(Json(analytics))
}

/// GET /api/analytics/room/{slug}/equity-curve
///
/// Get equity curve data points for charting.
/// Returns cumulative P&L over time with trade IDs.
///
/// # Query Parameters
/// - `from` - Optional start date (YYYY-MM-DD)
/// - `to` - Optional end date (YYYY-MM-DD)
async fn get_equity_curve(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let equity_curve: Vec<EquityPoint> = service
        .get_equity_curve(&slug, query.parse_from(), query.parse_to())
        .await
        .map_err(|e| {
            error!("Failed to get equity curve for {}: {}", slug, e);
            ApiError::internal_error(&format!("Failed to get equity curve: {}", e))
        })?;

    Ok(Json(json!({
        "success": true,
        "data": equity_curve,
        "meta": {
            "room_slug": slug,
            "points": equity_curve.len(),
            "from": query.from,
            "to": query.to
        }
    })))
}

/// GET /api/analytics/room/{slug}/ticker/{ticker}
///
/// Get performance analytics for a specific ticker symbol.
///
/// # Path Parameters
/// - `slug` - Room identifier
/// - `ticker` - Stock ticker symbol (case-insensitive)
async fn get_ticker_analytics(
    State(state): State<AppState>,
    Path((slug, ticker)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let analytics: Option<TickerPerformance> = service
        .get_ticker_analytics(&slug, &ticker)
        .await
        .map_err(|e| {
        error!(
            "Failed to get ticker analytics for {} in {}: {}",
            ticker, slug, e
        );
        ApiError::internal_error(&format!("Failed to get ticker analytics: {}", e))
    })?;

    match analytics {
        Some(data) => Ok(Json(json!({
            "success": true,
            "data": data
        }))),
        None => Err(ApiError::not_found(&format!(
            "No trades found for ticker {} in room {}",
            ticker, slug
        ))),
    }
}

/// GET /api/analytics/room/{slug}/monthly
///
/// Get monthly returns breakdown.
///
/// # Query Parameters
/// - `from` - Optional start date (YYYY-MM-DD)
/// - `to` - Optional end date (YYYY-MM-DD)
async fn get_monthly_returns(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let monthly: Vec<MonthlyReturn> = service
        .get_monthly_returns(&slug, query.parse_from(), query.parse_to())
        .await
        .map_err(|e| {
            error!("Failed to get monthly returns for {}: {}", slug, e);
            ApiError::internal_error(&format!("Failed to get monthly returns: {}", e))
        })?;

    Ok(Json(json!({
        "success": true,
        "data": monthly,
        "meta": {
            "room_slug": slug,
            "months": monthly.len()
        }
    })))
}

/// GET /api/analytics/room/{slug}/tickers
///
/// Get performance breakdown by ticker symbol.
///
/// # Query Parameters
/// - `from` - Optional start date (YYYY-MM-DD)
/// - `to` - Optional end date (YYYY-MM-DD)
async fn get_ticker_performance(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let tickers: Vec<TickerPerformance> = service
        .get_ticker_performance(&slug, query.parse_from(), query.parse_to())
        .await
        .map_err(|e| {
            error!("Failed to get ticker performance for {}: {}", slug, e);
            ApiError::internal_error(&format!("Failed to get ticker performance: {}", e))
        })?;

    Ok(Json(json!({
        "success": true,
        "data": tickers,
        "meta": {
            "room_slug": slug,
            "ticker_count": tickers.len()
        }
    })))
}

/// GET /api/analytics/room/{slug}/setups
///
/// Get performance breakdown by setup type.
///
/// # Query Parameters
/// - `from` - Optional start date (YYYY-MM-DD)
/// - `to` - Optional end date (YYYY-MM-DD)
async fn get_setup_performance(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let service = RoomAnalyticsService::new(state.db.pool.clone());

    let setups: Vec<SetupPerformance> = service
        .get_setup_performance(&slug, query.parse_from(), query.parse_to())
        .await
        .map_err(|e| {
            error!("Failed to get setup performance for {}: {}", slug, e);
            ApiError::internal_error(&format!("Failed to get setup performance: {}", e))
        })?;

    Ok(Json(json!({
        "success": true,
        "data": setups,
        "meta": {
            "room_slug": slug,
            "setup_count": setups.len()
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create the room analytics router
///
/// Registers all room analytics endpoints under /api/analytics/room
pub fn router() -> Router<AppState> {
    Router::new()
        // Full analytics for a room
        .route("/room/{slug}", get(get_room_analytics))
        // Equity curve data
        .route("/room/{slug}/equity-curve", get(get_equity_curve))
        // Specific ticker analytics
        .route("/room/{slug}/ticker/{ticker}", get(get_ticker_analytics))
        // Monthly returns
        .route("/room/{slug}/monthly", get(get_monthly_returns))
        // All tickers performance
        .route("/room/{slug}/tickers", get(get_ticker_performance))
        // Setup performance
        .route("/room/{slug}/setups", get(get_setup_performance))
}
