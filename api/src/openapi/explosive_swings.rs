//! Explosive Swings API OpenAPI Documentation
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT 7+ Grade - January 2026
//!
//! OpenAPI/Swagger documentation for the Explosive Swings trading room API.
//! Provides comprehensive documentation for alerts, trades, trade plans,
//! stats, search, export, and analytics endpoints.

use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

// ═══════════════════════════════════════════════════════════════════════════════════
// SCHEMA DEFINITIONS - Response Types
// ═══════════════════════════════════════════════════════════════════════════════════

/// Alert response with full TOS (ThinkOrSwim) format support
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlertResponse {
    /// Unique alert identifier
    pub id: i64,
    /// Room identifier
    pub room_id: i64,
    /// Room slug
    pub room_slug: String,
    /// Alert type: ENTRY, UPDATE, or EXIT
    pub alert_type: String,
    /// Ticker symbol
    pub ticker: String,
    /// Alert title
    pub title: Option<String>,
    /// Alert message
    pub message: String,
    /// Additional notes
    pub notes: Option<String>,
    /// Trade type: options or shares
    pub trade_type: Option<String>,
    /// Action: BUY or SELL
    pub action: Option<String>,
    /// Quantity
    pub quantity: Option<i32>,
    /// Option type: CALL or PUT
    pub option_type: Option<String>,
    /// Strike price
    pub strike: Option<f64>,
    /// Expiration date (YYYY-MM-DD)
    #[schema(example = "2026-02-21")]
    pub expiration: Option<String>,
    /// Contract type: Weeklys, Monthly, or LEAPS
    pub contract_type: Option<String>,
    /// Order type: MKT or LMT
    pub order_type: Option<String>,
    /// Limit price
    pub limit_price: Option<f64>,
    /// Fill price
    pub fill_price: Option<f64>,
    /// Full TOS format string
    #[schema(example = "BUY +1 NVDA 100 (Weeklys) 21 FEB 26 950 CALL @MKT")]
    pub tos_string: Option<String>,
    /// Reference to entry alert for updates/exits
    pub entry_alert_id: Option<i64>,
    /// Reference to trade plan
    pub trade_plan_id: Option<i64>,
    /// Whether alert is new/unread
    pub is_new: bool,
    /// Whether alert is published
    pub is_published: bool,
    /// Whether alert is pinned
    pub is_pinned: bool,
    /// When alert was published
    pub published_at: String,
    /// When alert was created
    pub created_at: String,
    /// When alert was last updated
    pub updated_at: String,
}

/// Request to create a new alert
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlertCreateRequest {
    /// Room slug (e.g., "explosive-swings")
    #[schema(example = "explosive-swings")]
    pub room_slug: String,
    /// Alert type: ENTRY, UPDATE, or EXIT
    #[schema(example = "ENTRY")]
    pub alert_type: String,
    /// Ticker symbol
    #[schema(example = "NVDA")]
    pub ticker: String,
    /// Alert title
    #[schema(example = "NVDA Call Entry")]
    pub title: String,
    /// Alert message
    #[schema(example = "Entering NVDA calls on strength")]
    pub message: String,
    /// Additional notes
    pub notes: Option<String>,
    /// Trade type: options or shares
    pub trade_type: Option<String>,
    /// Action: BUY or SELL
    pub action: Option<String>,
    /// Quantity
    pub quantity: Option<i32>,
    /// Option type: CALL or PUT
    pub option_type: Option<String>,
    /// Strike price
    pub strike: Option<f64>,
    /// Expiration date (YYYY-MM-DD)
    pub expiration: Option<String>,
    /// Contract type: Weeklys, Monthly, or LEAPS
    pub contract_type: Option<String>,
    /// Order type: MKT or LMT
    pub order_type: Option<String>,
    /// Limit price
    pub limit_price: Option<f64>,
    /// Fill price
    pub fill_price: Option<f64>,
    /// Full TOS format string
    pub tos_string: Option<String>,
    /// Reference to entry alert for updates/exits
    pub entry_alert_id: Option<i64>,
    /// Reference to trade plan
    pub trade_plan_id: Option<i64>,
    /// Whether alert is new/unread (default: true)
    pub is_new: Option<bool>,
    /// Whether alert is published (default: true)
    pub is_published: Option<bool>,
    /// Whether alert is pinned (default: false)
    pub is_pinned: Option<bool>,
}

/// Trade response from the trade tracker
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradeResponse {
    /// Unique trade identifier
    pub id: i64,
    /// Room identifier
    pub room_id: i64,
    /// Room slug
    pub room_slug: String,
    /// Ticker symbol
    pub ticker: String,
    /// Trade type: options or shares
    pub trade_type: String,
    /// Direction: long or short
    pub direction: String,
    /// Quantity
    pub quantity: i32,
    /// Option type: CALL or PUT (for options)
    pub option_type: Option<String>,
    /// Strike price (for options)
    pub strike: Option<f64>,
    /// Expiration date (for options)
    pub expiration: Option<String>,
    /// Contract type: Weeklys, Monthly, or LEAPS
    pub contract_type: Option<String>,
    /// Entry alert reference
    pub entry_alert_id: Option<i64>,
    /// Entry price
    pub entry_price: f64,
    /// Entry date
    #[schema(example = "2026-01-28")]
    pub entry_date: String,
    /// Entry TOS string
    pub entry_tos_string: Option<String>,
    /// Exit alert reference
    pub exit_alert_id: Option<i64>,
    /// Exit price
    pub exit_price: Option<f64>,
    /// Exit date
    pub exit_date: Option<String>,
    /// Exit TOS string
    pub exit_tos_string: Option<String>,
    /// Trade setup type
    pub setup: Option<String>,
    /// Trade status: open, closed, or invalidated
    pub status: String,
    /// Trade result: WIN or LOSS
    pub result: Option<String>,
    /// Profit/Loss in dollars
    pub pnl: Option<f64>,
    /// Profit/Loss percentage
    pub pnl_percent: Option<f64>,
    /// Holding period in days
    pub holding_days: Option<i32>,
    /// Trade notes
    pub notes: Option<String>,
    /// Whether trade was updated after entry
    pub was_updated: Option<bool>,
    /// Reason for invalidation
    pub invalidation_reason: Option<String>,
    /// When trade was created
    pub created_at: String,
    /// When trade was last updated
    pub updated_at: String,
}

/// Request to create a new trade
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradeCreateRequest {
    /// Room slug
    #[schema(example = "explosive-swings")]
    pub room_slug: String,
    /// Ticker symbol
    #[schema(example = "NVDA")]
    pub ticker: String,
    /// Trade type: options or shares
    #[schema(example = "options")]
    pub trade_type: String,
    /// Direction: long or short
    #[schema(example = "long")]
    pub direction: String,
    /// Quantity
    #[schema(example = 5)]
    pub quantity: i32,
    /// Option type: CALL or PUT
    pub option_type: Option<String>,
    /// Strike price
    pub strike: Option<f64>,
    /// Expiration date (YYYY-MM-DD)
    pub expiration: Option<String>,
    /// Contract type
    pub contract_type: Option<String>,
    /// Entry alert reference
    pub entry_alert_id: Option<i64>,
    /// Entry price
    #[schema(example = 12.50)]
    pub entry_price: f64,
    /// Entry date (YYYY-MM-DD)
    #[schema(example = "2026-01-28")]
    pub entry_date: String,
    /// Entry TOS string
    pub entry_tos_string: Option<String>,
    /// Trade setup type
    pub setup: Option<String>,
    /// Trade notes
    pub notes: Option<String>,
}

/// Trade plan response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradePlanResponse {
    /// Unique identifier
    pub id: i64,
    /// Room identifier
    pub room_id: i64,
    /// Room slug
    pub room_slug: String,
    /// Week of the trade plan
    #[schema(example = "2026-01-27")]
    pub week_of: String,
    /// Ticker symbol
    pub ticker: String,
    /// Bias: BULLISH or BEARISH
    pub bias: String,
    /// Entry level
    pub entry: Option<String>,
    /// Target 1
    pub target1: Option<String>,
    /// Target 2
    pub target2: Option<String>,
    /// Target 3
    pub target3: Option<String>,
    /// Runner target
    pub runner: Option<String>,
    /// Runner stop
    pub runner_stop: Option<String>,
    /// Stop loss level
    pub stop: Option<String>,
    /// Options strike
    pub options_strike: Option<String>,
    /// Options expiration
    pub options_exp: Option<String>,
    /// Notes
    pub notes: Option<String>,
    /// Sort order
    pub sort_order: i32,
    /// Whether plan is active
    pub is_active: bool,
    /// When created
    pub created_at: String,
    /// When updated
    pub updated_at: String,
}

/// Room statistics response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StatsResponse {
    /// Unique identifier
    pub id: i64,
    /// Room identifier
    pub room_id: i64,
    /// Room slug
    pub room_slug: String,
    /// Win rate percentage
    #[schema(example = 72.5)]
    pub win_rate: Option<f64>,
    /// Weekly profit
    pub weekly_profit: Option<String>,
    /// Monthly profit
    pub monthly_profit: Option<String>,
    /// Number of active trades
    pub active_trades: Option<i32>,
    /// Trades closed this week
    pub closed_this_week: Option<i32>,
    /// Total trades all time
    pub total_trades: Option<i32>,
    /// Total wins
    pub wins: Option<i32>,
    /// Total losses
    pub losses: Option<i32>,
    /// Average win amount
    pub avg_win: Option<f64>,
    /// Average loss amount
    pub avg_loss: Option<f64>,
    /// Profit factor
    pub profit_factor: Option<f64>,
    /// Average holding period in days
    pub avg_holding_days: Option<f64>,
    /// Largest win
    pub largest_win: Option<f64>,
    /// Largest loss
    pub largest_loss: Option<f64>,
    /// Current streak (positive = wins, negative = losses)
    pub current_streak: Option<i32>,
    /// When stats were last calculated
    pub calculated_at: String,
}

/// Search results response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchResults {
    /// Alert search results
    pub alerts: Vec<AlertSearchResult>,
    /// Trade search results
    pub trades: Vec<TradeSearchResult>,
    /// Trade plan search results
    pub trade_plans: Vec<TradePlanSearchResult>,
    /// Total count of all results
    pub total_count: i64,
    /// The original search query
    pub query: String,
    /// Search execution time in milliseconds
    pub took_ms: u64,
    /// Current pagination state
    pub pagination: PaginationMeta,
}

/// Alert search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlertSearchResult {
    pub id: i64,
    pub ticker: String,
    pub title: Option<String>,
    pub alert_type: String,
    pub message: String,
    pub published_at: String,
    /// Relevance score (0.0 - 1.0+)
    pub relevance_score: f32,
    /// Highlighted snippet with matched terms
    pub highlight: String,
}

/// Trade search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradeSearchResult {
    pub id: i64,
    pub ticker: String,
    pub direction: String,
    pub status: String,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub entry_date: String,
    pub exit_date: Option<String>,
    pub pnl_percent: Option<f64>,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub relevance_score: f32,
    pub highlight: String,
}

/// Trade plan search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TradePlanSearchResult {
    pub id: i64,
    pub ticker: String,
    pub bias: String,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub stop: Option<String>,
    pub week_of: String,
    pub notes: Option<String>,
    pub is_active: bool,
    pub relevance_score: f32,
    pub highlight: String,
}

/// Pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub limit: i32,
    pub offset: i32,
    pub has_more: bool,
}

/// Room analytics response with comprehensive metrics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomAnalytics {
    /// High-level performance summary
    pub summary: AnalyticsSummary,
    /// Performance breakdown by ticker
    pub performance_by_ticker: Vec<TickerPerformance>,
    /// Performance breakdown by setup type
    pub performance_by_setup: Vec<SetupPerformance>,
    /// Monthly performance data
    pub monthly_performance: Vec<MonthlyPerformance>,
    /// Daily P&L for equity curve
    pub daily_pnl: Vec<DailyPnL>,
    /// Alert effectiveness metrics
    pub alert_effectiveness: AlertEffectiveness,
    /// Win/loss streak analysis
    pub streak_analysis: StreakAnalysis,
}

/// Analytics summary with key metrics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsSummary {
    pub total_alerts: i64,
    pub total_trades: i64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub max_drawdown_percent: f64,
    pub avg_holding_days: f64,
    pub total_pnl: f64,
    pub total_pnl_percent: f64,
    pub avg_win_percent: f64,
    pub avg_loss_percent: f64,
    pub largest_win_percent: f64,
    pub largest_loss_percent: f64,
    pub risk_reward_ratio: f64,
    pub expectancy: f64,
}

/// Ticker-level performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TickerPerformance {
    pub ticker: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub total_pnl_percent: f64,
    pub avg_pnl: f64,
    pub avg_pnl_percent: f64,
    pub avg_holding_days: f64,
    pub largest_win_percent: f64,
    pub largest_loss_percent: f64,
}

/// Setup-level performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SetupPerformance {
    pub setup: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub total_pnl: f64,
    pub avg_pnl: f64,
    pub profit_factor: f64,
}

/// Monthly performance breakdown
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MonthlyPerformance {
    pub year: i32,
    pub month: i32,
    pub month_name: String,
    pub total_trades: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_rate: f64,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub is_positive: bool,
}

/// Daily P&L for equity curve
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DailyPnL {
    pub date: String,
    pub pnl: f64,
    pub pnl_percent: f64,
    pub cumulative_pnl: f64,
    pub cumulative_pnl_percent: f64,
    pub trade_count: i64,
}

/// Alert-to-trade conversion effectiveness
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AlertEffectiveness {
    pub total_alerts: i64,
    pub alerts_with_trades: i64,
    pub alerts_without_trades: i64,
    pub conversion_rate: f64,
    pub profitable_conversion_rate: f64,
    pub avg_time_to_trade_hours: f64,
    pub entry_alerts: i64,
    pub update_alerts: i64,
    pub exit_alerts: i64,
}

/// Win/loss streak analysis
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StreakAnalysis {
    pub current_streak: i32,
    pub current_streak_type: String,
    pub max_win_streak: i32,
    pub max_loss_streak: i32,
    pub avg_win_streak: f64,
    pub avg_loss_streak: f64,
}

/// List metadata for pagination
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

/// API error response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: String,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// OPENAPI DOCUMENTATION
// ═══════════════════════════════════════════════════════════════════════════════════

/// Explosive Swings API Documentation
///
/// Complete OpenAPI documentation for the Explosive Swings trading room API.
/// Includes endpoints for alerts, trades, trade plans, statistics, search,
/// export, and analytics.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Explosive Swings API",
        version = "1.0.0",
        description = "Trading room API for Explosive Swings - Comprehensive trade alerts, tracking, and analytics.\n\n## Authentication\nAll endpoints require Bearer token authentication: `Authorization: Bearer <token>`\n\n## Rate Limiting\nAPI requests are rate-limited to prevent abuse. See response headers for limit information.\n\n## Base URLs\n- Production: https://revolution-trading-pros-api.fly.dev\n- Development: http://localhost:8080",
        contact(
            name = "Revolution Trading Pros",
            email = "support@revolutiontradingpros.com"
        )
    ),
    servers(
        (url = "https://revolution-trading-pros-api.fly.dev", description = "Production"),
        (url = "http://localhost:8080", description = "Development")
    ),
    paths(
        // Alerts - documented via utoipa::path attributes on handlers
        // crate::routes::room_content::list_alerts,
        // crate::routes::room_content::create_alert,
        // crate::routes::room_content::update_alert,
        // crate::routes::room_content::delete_alert,
        // Trades
        // crate::routes::room_content::list_trades,
        // crate::routes::room_content::create_trade,
        // crate::routes::room_content::close_trade,
        // crate::routes::room_content::invalidate_trade,
        // Trade Plans
        // crate::routes::room_content::list_trade_plans,
        // crate::routes::room_content::create_trade_plan,
        // Stats
        // crate::routes::room_content::get_room_stats,
        // Search
        // crate::routes::room_search::search_room,
        // Export
        // crate::routes::export::export_alerts_csv,
        // crate::routes::export::export_trades_csv,
        // Analytics
        // crate::routes::analytics::get_room_analytics,
    ),
    components(
        schemas(
            AlertResponse,
            AlertCreateRequest,
            TradeResponse,
            TradeCreateRequest,
            TradePlanResponse,
            StatsResponse,
            SearchResults,
            AlertSearchResult,
            TradeSearchResult,
            TradePlanSearchResult,
            PaginationMeta,
            RoomAnalytics,
            AnalyticsSummary,
            TickerPerformance,
            SetupPerformance,
            MonthlyPerformance,
            DailyPnL,
            AlertEffectiveness,
            StreakAnalysis,
            ListMeta,
            ApiErrorResponse,
        )
    ),
    tags(
        (name = "alerts", description = "Trading alert operations - Entry, Update, and Exit alerts with TOS format support"),
        (name = "trades", description = "Trade tracking operations - Open, close, and track positions"),
        (name = "trade-plans", description = "Weekly trade plan operations - Planned setups with targets and stops"),
        (name = "stats", description = "Performance statistics - Win rate, P&L, and streak data"),
        (name = "search", description = "Full-text search across alerts, trades, and trade plans"),
        (name = "export", description = "Data export - CSV downloads for alerts and trades"),
        (name = "analytics", description = "Comprehensive analytics - Equity curves, drawdowns, and performance metrics"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ExplosiveSwingsApi;

/// Security configuration for bearer token authentication
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token for authentication"))
                        .build(),
                ),
            );
        }
    }
}
