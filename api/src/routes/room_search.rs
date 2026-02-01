//! Room Content Search API Routes
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 2026
//!
//! Full-text search endpoints for trading room content:
//!
//!   - GET /api/room-search/:room_slug - Unified search across alerts, trades, trade plans
//!   - GET /api/room-search/:room_slug/alerts - Search alerts only
//!   - GET /api/room-search/:room_slug/trades - Search trades only
//!   - GET /api/room-search/:room_slug/trade-plans - Search trade plans only
//!   - GET /api/room-search/:room_slug/suggestions - Get autocomplete suggestions
//!
//! Features:
//!
//!   - PostgreSQL native full-text search with GIN indexes
//!   - Relevance ranking with ts_rank_cd
//!   - Search result highlighting
//!   - Date range filtering
//!   - Ticker filtering
//!   - Content type filtering
//!   - Pagination
//!
//! ═══════════════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, instrument};

use crate::models::User;
use crate::services::room_search::{
    AlertSearchResult, Pagination, RoomSearchService, SearchFilters, SearchResults,
    SearchSuggestions, SearchableContentType, TradePlanSearchResult, TradeSearchResult,
};
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    /// The search query string
    pub q: String,

    /// Content types to search (comma-separated: alerts,trades,trade_plans)
    /// Defaults to all types if not specified
    #[serde(default)]
    pub types: Option<String>,

    /// Start date filter (YYYY-MM-DD)
    pub from: Option<String>,

    /// End date filter (YYYY-MM-DD)
    pub to: Option<String>,

    /// Filter by specific ticker symbol
    pub ticker: Option<String>,

    /// Maximum results to return (default: 20, max: 100)
    #[serde(default = "default_limit")]
    pub limit: i32,

    /// Offset for pagination (default: 0)
    #[serde(default)]
    pub offset: i32,
}

fn default_limit() -> i32 {
    20
}

/// Suggestion query parameters
#[derive(Debug, Deserialize)]
pub struct SuggestionParams {
    /// The prefix to match
    pub q: String,

    /// Maximum suggestions to return (default: 10)
    #[serde(default = "default_suggestion_limit")]
    pub limit: i32,
}

fn default_suggestion_limit() -> i32 {
    10
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct SearchApiResponse {
    pub success: bool,
    pub data: SearchResults,
}

/// Alerts-only response
#[derive(Debug, Serialize)]
pub struct AlertsSearchResponse {
    pub success: bool,
    pub data: Vec<AlertSearchResult>,
    pub query: String,
    pub took_ms: u64,
    pub total: usize,
}

/// Trades-only response
#[derive(Debug, Serialize)]
pub struct TradesSearchResponse {
    pub success: bool,
    pub data: Vec<TradeSearchResult>,
    pub query: String,
    pub took_ms: u64,
    pub total: usize,
}

/// Trade plans-only response
#[derive(Debug, Serialize)]
pub struct TradePlansSearchResponse {
    pub success: bool,
    pub data: Vec<TradePlanSearchResult>,
    pub query: String,
    pub took_ms: u64,
    pub total: usize,
}

/// Suggestions response
#[derive(Debug, Serialize)]
pub struct SuggestionsResponse {
    pub success: bool,
    pub data: SearchSuggestions,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Unified search across all content types
///
/// GET /api/room-search/:room_slug?q=NVDA&types=alerts,trades&from=2026-01-01&limit=20
#[instrument(skip(state, _user), fields(room = %room_slug, query = %params.q))]
async fn search_room(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<SearchParams>,
    _user: User,
) -> Result<Json<SearchApiResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Validate query
    let query = params.q.trim();
    if query.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Search query 'q' is required and cannot be empty"
            })),
        ));
    }

    if query.len() > 500 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Search query too long (max 500 characters)"
            })),
        ));
    }

    // Parse filters
    let filters = parse_filters(&params)?;
    let pagination = Pagination {
        limit: params.limit.clamp(1, 100),
        offset: params.offset.max(0),
    };

    // Create search service and execute search
    let search_service = RoomSearchService::new(state.db.pool.clone());

    let results = search_service
        .search_room(&room_slug, query, filters, pagination)
        .await
        .map_err(|e| {
            error!("Search failed for room {}: {}", room_slug, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Search failed: {}", e)
                })),
            )
        })?;

    info!(
        "Search completed: room={}, query='{}', results={}, took={}ms",
        room_slug, query, results.total_count, results.took_ms
    );

    Ok(Json(SearchApiResponse {
        success: true,
        data: results,
    }))
}

/// Search alerts only
///
/// GET /api/room-search/:room_slug/alerts?q=NVDA&from=2026-01-01
#[instrument(skip(state, _user), fields(room = %room_slug, query = %params.q))]
async fn search_alerts(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<SearchParams>,
    _user: User,
) -> Result<Json<AlertsSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let query = params.q.trim();
    if query.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Search query 'q' is required"
            })),
        ));
    }

    let filters = parse_filters(&params)?;
    let pagination = Pagination {
        limit: params.limit.clamp(1, 100),
        offset: params.offset.max(0),
    };

    let search_service = RoomSearchService::new(state.db.pool.clone());
    let start = std::time::Instant::now();

    let results = search_service
        .search_alerts(&room_slug, query, &filters, &pagination)
        .await
        .map_err(|e| {
            error!("Alert search failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Search failed: {}", e)
                })),
            )
        })?;

    let took_ms = start.elapsed().as_millis() as u64;
    let total = results.len();

    Ok(Json(AlertsSearchResponse {
        success: true,
        data: results,
        query: query.to_string(),
        took_ms,
        total,
    }))
}

/// Search trades only
///
/// GET /api/room-search/:room_slug/trades?q=NVDA&from=2026-01-01
#[instrument(skip(state, _user), fields(room = %room_slug, query = %params.q))]
async fn search_trades(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<SearchParams>,
    _user: User,
) -> Result<Json<TradesSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let query = params.q.trim();
    if query.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Search query 'q' is required"
            })),
        ));
    }

    let filters = parse_filters(&params)?;
    let pagination = Pagination {
        limit: params.limit.clamp(1, 100),
        offset: params.offset.max(0),
    };

    let search_service = RoomSearchService::new(state.db.pool.clone());
    let start = std::time::Instant::now();

    let results = search_service
        .search_trades(&room_slug, query, &filters, &pagination)
        .await
        .map_err(|e| {
            error!("Trade search failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Search failed: {}", e)
                })),
            )
        })?;

    let took_ms = start.elapsed().as_millis() as u64;
    let total = results.len();

    Ok(Json(TradesSearchResponse {
        success: true,
        data: results,
        query: query.to_string(),
        took_ms,
        total,
    }))
}

/// Search trade plans only
///
/// GET /api/room-search/:room_slug/trade-plans?q=NVDA&from=2026-01-01
#[instrument(skip(state, _user), fields(room = %room_slug, query = %params.q))]
async fn search_trade_plans(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<SearchParams>,
    _user: User,
) -> Result<Json<TradePlansSearchResponse>, (StatusCode, Json<serde_json::Value>)> {
    let query = params.q.trim();
    if query.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Search query 'q' is required"
            })),
        ));
    }

    let filters = parse_filters(&params)?;
    let pagination = Pagination {
        limit: params.limit.clamp(1, 100),
        offset: params.offset.max(0),
    };

    let search_service = RoomSearchService::new(state.db.pool.clone());
    let start = std::time::Instant::now();

    let results = search_service
        .search_trade_plans(&room_slug, query, &filters, &pagination)
        .await
        .map_err(|e| {
            error!("Trade plan search failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Search failed: {}", e)
                })),
            )
        })?;

    let took_ms = start.elapsed().as_millis() as u64;
    let total = results.len();

    Ok(Json(TradePlansSearchResponse {
        success: true,
        data: results,
        query: query.to_string(),
        took_ms,
        total,
    }))
}

/// Get autocomplete suggestions
///
/// GET /api/room-search/:room_slug/suggestions?q=NV&limit=10
#[instrument(skip(state, _user), fields(room = %room_slug, prefix = %params.q))]
async fn get_suggestions(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<SuggestionParams>,
    _user: User,
) -> Result<Json<SuggestionsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let prefix = params.q.trim();
    if prefix.is_empty() {
        return Ok(Json(SuggestionsResponse {
            success: true,
            data: SearchSuggestions { tickers: vec![] },
        }));
    }

    let limit = params.limit.clamp(1, 20);

    let search_service = RoomSearchService::new(state.db.pool.clone());

    let suggestions = search_service
        .get_suggestions(&room_slug, prefix, limit)
        .await
        .map_err(|e| {
            error!("Suggestions failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to get suggestions: {}", e)
                })),
            )
        })?;

    Ok(Json(SuggestionsResponse {
        success: true,
        data: suggestions,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Parse search filters from query parameters
fn parse_filters(
    params: &SearchParams,
) -> Result<SearchFilters, (StatusCode, Json<serde_json::Value>)> {
    // Parse content types
    let types = if let Some(types_str) = &params.types {
        types_str
            .split(',')
            .filter_map(|t| match t.trim().to_lowercase().as_str() {
                "alerts" => Some(SearchableContentType::Alerts),
                "trades" => Some(SearchableContentType::Trades),
                "trade_plans" | "tradeplans" => Some(SearchableContentType::TradePlans),
                _ => None,
            })
            .collect()
    } else {
        vec![]
    };

    // Parse dates
    let from_date = if let Some(from_str) = &params.from {
        Some(
            NaiveDate::parse_from_str(from_str, "%Y-%m-%d").map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "error": "Invalid 'from' date format. Use YYYY-MM-DD"
                    })),
                )
            })?,
        )
    } else {
        None
    };

    let to_date = if let Some(to_str) = &params.to {
        Some(NaiveDate::parse_from_str(to_str, "%Y-%m-%d").map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Invalid 'to' date format. Use YYYY-MM-DD"
                })),
            )
        })?)
    } else {
        None
    };

    // Validate date range
    if let (Some(from), Some(to)) = (from_date, to_date) {
        if from > to {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "'from' date cannot be after 'to' date"
                })),
            ));
        }
    }

    Ok(SearchFilters {
        types,
        from_date,
        to_date,
        ticker: params.ticker.clone(),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Build the room search router
///
/// Routes:
/// - GET /:room_slug - Unified search
/// - GET /:room_slug/alerts - Search alerts
/// - GET /:room_slug/trades - Search trades
/// - GET /:room_slug/trade-plans - Search trade plans
/// - GET /:room_slug/suggestions - Get suggestions
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{room_slug}", get(search_room))
        .route("/{room_slug}/alerts", get(search_alerts))
        .route("/{room_slug}/trades", get(search_trades))
        .route("/{room_slug}/trade-plans", get(search_trade_plans))
        .route("/{room_slug}/suggestions", get(get_suggestions))
}
