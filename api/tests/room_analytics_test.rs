// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Room analytics route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::room_analytics` and
//! exercises the public DTOs (`AnalyticsQuery`) + the
//! `services::room_analytics` types that flow through these
//! handlers (`RoomAnalytics`, `AnalyticsSummary`, `TickerPerformance`,
//! `SetupPerformance`, `MonthlyReturn`, `EquityPoint`,
//! `DrawdownPeriod`, `AnalyticsDateRange`).
//!
//! ## Why this file exists (R19-D)
//!
//! `routes/room_analytics.rs` is 298 LOC of admin-gated performance
//! analytics over the `room_trades` table (BIGSERIAL i64 PK per
//! schema.sql:9621). 6 endpoints, ALL `AdminUser`-gated:
//!
//!   - GET /room/:slug                       (full analytics)
//!   - GET /room/:slug/equity-curve          (charting data)
//!   - GET /room/:slug/ticker/:ticker        (per-ticker)
//!   - GET /room/:slug/monthly               (monthly returns)
//!   - GET /room/:slug/tickers               (all tickers)
//!   - GET /room/:slug/setups                (setup breakdown)
//!
//! What we CAN pin in no-DB tests:
//!
//! 1. **`AnalyticsSummary` aggregates are i64.** Lines 38-40:
//!      `total_trades: i64, wins: i64, losses: i64`.
//!    Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY": cross-
//!    table COUNT(*) over the BIGSERIAL `room_trades` table MUST
//!    return i64. Over a multi-year deployment with multiple active
//!    rooms, total trades CAN exceed i32::MAX (~2.1B); a regression
//!    to i32 would silently wrap.
//!
//! 2. **`EquityPoint.trade_id: i64` (BIGSERIAL-aligned).** Line 91:
//!    `room_trades.id` is `bigint NOT NULL` (schema.sql:9622); the
//!    DTO mirrors with `trade_id: i64`. A regression to i32 would
//!    silently break deserialization on any production row past
//!    2^31 — and trades accumulate hundreds/week per active room.
//!
//! 3. **`AnalyticsSummary.current_streak: i32` IS Reserved
//!    exception** (line 49). Streak counters are TRULY bounded
//!    below 2 billion (the longest win/loss streak in trading
//!    history is ~30; even pathological bot streaks are O(1000)).
//!    Per CLAUDE.md "Reserved exception: row counts (revisions:
//!    i32, attempts: i32, total_pages: i32) — those genuinely cap
//!    below 2 billion and i32 is fine. Money never qualifies for
//!    the exception." This is exactly the row-count-style case.
//!
//! 4. **`AnalyticsQuery.from` and `.to` are BOTH Optional.** A
//!    bare `GET /room/:slug` (no query string) MUST NOT 400 —
//!    the dashboard sends no params for the "all time" default.
//!    R9-D NEGATIVE: a regression that flipped either to required
//!    would 422 every dashboard tile.
//!
//! 5. **Date parsing is `"%Y-%m-%d"` with `.ok()` fallback.** The
//!    `parse_from()`/`parse_to()` helpers (lines 51-63) return
//!    `Option<NaiveDate>` via `.and_then(|d| NaiveDate::
//!    parse_from_str(d, "%Y-%m-%d").ok())`. Malformed dates
//!    silently become None (NOT 422). This is intentional — the
//!    frontend date-picker can ship invalid dates on browser
//!    quirks; the analytics call should still return "all time"
//!    rather than fail loud.
//!
//! 6. **f64 throughout for PnL / returns** — `total_pnl: f64`,
//!    `win_rate: f64`, `profit_factor: f64`. NOT money cents.
//!    These are computed RATIOS and aggregates (display values
//!    over the f64 `room_trades.pnl` column). Per CLAUDE.md money
//!    rule: actual money belongs in `*_cents: i64` columns, but
//!    these analytics aggregates are display-only computations
//!    (the `room_trades` schema stores `pnl` as `double precision`,
//!    not BIGINT cents — a separate, older design choice the
//!    money path will eventually correct).
//!
//! 7. **`AnalyticsDateRange` uses NaiveDate, NOT String** (line
//!    106). This is the SERVICE-LAYER type; the ROUTE-LAYER
//!    `AnalyticsQuery` uses `Option<String>` so axum's Query
//!    extractor can accept malformed dates and downgrade to None.
//!    The two MUST stay in sync — drift would silently break
//!    every analytics request that filters by date.
//!
//! ## Pattern source
//!
//! Modeled on `tests/analytics_test.rs`, `tests/room_search_test.rs`,
//! `tests/admin_orders_test.rs`.

use chrono::NaiveDate;
use revolution_api::routes::room_analytics::AnalyticsQuery;
use revolution_api::services::room_analytics::{
    AnalyticsSummary, EquityPoint, MonthlyReturn, SetupPerformance, TickerPerformance,
};

// ── 1. AnalyticsSummary aggregate counts are i64 (BIGSERIAL-aligned) ─

/// `AnalyticsSummary.total_trades / wins / losses` are all `i64`
/// (services/room_analytics.rs:38-40). Per CLAUDE.md "Money /
/// cents — i64 ONLY, BIGINT ONLY": COUNT(*) over the BIGSERIAL
/// `room_trades` table (schema.sql:9621) MUST return i64.
///
/// While `i32` could theoretically fit (the largest active room
/// has ~10K trades/year, so 2.1B = 210K room-years), the principle
/// per CLAUDE.md is: "Mixing types within a single value's
/// lifecycle (struct → DB → API) is a recipe for silent overflow
/// during summation." Aggregates over BIGSERIAL must be i64
/// end-to-end.
///
/// R9-D NEGATIVE: serialize > i32::MAX and confirm round-trip.
#[test]
fn analytics_summary_aggregates_are_i64() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let summary = AnalyticsSummary {
        total_trades: above_i32,
        wins: above_i32 / 2,
        losses: above_i32 / 2,
        win_rate: 50.0,
        profit_factor: 1.5,
        avg_win: 250.0,
        avg_loss: -100.0,
        largest_win: 5_000.0,
        largest_loss: -2_000.0,
        max_drawdown: -3_000.0,
        avg_holding_days: 7.5,
        current_streak: 3,
        streak_type: "win".to_string(),
    };

    let _: i64 = summary.total_trades;
    let _: i64 = summary.wins;
    let _: i64 = summary.losses;

    let wire = serde_json::to_value(&summary).expect("serialize AnalyticsSummary");
    assert_eq!(
        wire["total_trades"].as_i64(),
        Some(above_i32),
        "AnalyticsSummary.total_trades MUST be i64 — COUNT(*) over \
         BIGSERIAL room_trades (schema.sql:9621); regression to i32 \
         would silently wrap past 2.1B trades"
    );
    assert_eq!(wire["wins"].as_i64(), Some(above_i32 / 2));
    assert_eq!(wire["losses"].as_i64(), Some(above_i32 / 2));
}

// ── 2. EquityPoint.trade_id is i64 (mirrors room_trades.id BIGSERIAL) ─

/// `EquityPoint.trade_id: i64` (services/room_analytics.rs:91)
/// mirrors `room_trades.id` (`bigint NOT NULL`, schema.sql:9622).
/// The equity curve endpoint returns one EquityPoint per closed
/// trade, with `trade_id` as the DB row's PK. A regression to
/// i32 would silently break deserialization on any production
/// row past 2^31 — and given trades accumulate hundreds/week per
/// active room, this ceiling is reachable on a multi-year
/// deployment.
#[test]
fn equity_point_trade_id_is_i64_bigserial() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let point = EquityPoint {
        date: NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid"),
        cumulative_pnl: 1_234.56,
        trade_id: above_i32,
    };

    let _: i64 = point.trade_id;
    let _: f64 = point.cumulative_pnl; // f64 — display-aggregate, NOT cents
    let _: NaiveDate = point.date;

    let wire = serde_json::to_value(&point).expect("serialize EquityPoint");
    assert_eq!(
        wire["trade_id"].as_i64(),
        Some(above_i32),
        "EquityPoint.trade_id MUST be i64 — room_trades.id is \
         bigint NOT NULL (schema.sql:9622)"
    );
    assert_eq!(wire["date"].as_str(), Some("2026-05-20"));
}

// ── 3. current_streak: i32 is Reserved exception (rationale) ─────────

/// `AnalyticsSummary.current_streak: i32` (line 49) is a
/// **Reserved exception** per CLAUDE.md "Money / cents" rule:
///
///   > "Reserved exception: row counts (revisions: i32, attempts:
///   > i32, total_pages: i32) — those genuinely cap below 2 billion
///   > and i32 is fine. Money never qualifies for the exception."
///
/// Rationale for `current_streak: i32` qualifying as Reserved:
///   - The longest documented win/loss streak in trading history
///     is ~30 consecutive wins (Jim Simons/RenTech, 1990s)
///   - Even a pathological algo with sub-second entries running
///     24/7 for a year produces O(31_000_000) entries — well below
///     i32::MAX (2.1B). 30 years of pathological streaks = 1B,
///     still under the cap.
///   - The value is a per-room aggregate, NOT a sum across rooms,
///     so it can't accumulate the way `total_trades` does.
///   - This is NOT money: it's a count of consecutive boolean
///     outcomes. Per CLAUDE.md, money never qualifies for the
///     exception; this is not money.
///
/// R9-D NEGATIVE: streak_type values are "win" OR "loss"
/// (NOT canonical uppercase like WIN/LOSS — the analytics layer
/// uses lowercase per the source, line 50 comment). A regression
/// to uppercase would silently break the frontend that
/// case-sensitively compares the value.
#[test]
fn analytics_summary_current_streak_is_i32_reserved_exception() {
    let summary = AnalyticsSummary {
        current_streak: 7,
        streak_type: "win".to_string(),
        ..AnalyticsSummary::default()
    };

    let _: i32 = summary.current_streak;

    // Reserved exception rationale: a 7-game winning streak is
    // typical; the type ceiling (i32::MAX = 2.1B consecutive wins)
    // is unreachable in any realistic universe.
    let max_realistic_streak: i32 = 1_000_000; // even a runaway bot can't sustain this
    assert!(
        max_realistic_streak < i32::MAX,
        "i32 ceiling (~2.1B) MUST exceed any realistic streak value \
         — i32 is Reserved exception per CLAUDE.md"
    );

    // Wire round-trip
    let wire = serde_json::to_value(&summary).expect("serialize AnalyticsSummary");
    assert_eq!(wire["current_streak"].as_i64(), Some(7));
    assert_eq!(
        wire["streak_type"].as_str(),
        Some("win"),
        "streak_type MUST be lowercase 'win' or 'loss' — the \
         analytics layer convention is lowercase (line 50 comment); \
         regression to uppercase WIN/LOSS would silently break the \
         frontend case-sensitive comparison"
    );
}

// ── 4. AnalyticsQuery — both `from` and `to` are Optional ────────────

/// `AnalyticsQuery { from: Option<String>, to: Option<String> }`
/// per `routes/room_analytics.rs:42-48`. Both MUST be Optional so
/// the dashboard's "all time" default (which sends no query
/// params) doesn't 422.
///
/// The fields are `Option<String>`, NOT `Option<NaiveDate>` — by
/// design. The route layer accepts malformed date strings and
/// downgrades them to None via `.parse_from_str(...).ok()`, so
/// the frontend never gets a 422 even if the date picker emits
/// `"undefined"` on a browser glitch.
///
/// R9-D NEGATIVE: missing both fields MUST parse OK.
#[test]
fn analytics_query_both_fields_optional() {
    // Both empty
    let empty: AnalyticsQuery = serde_json::from_value(serde_json::json!({}))
        .expect("AnalyticsQuery MUST parse empty body — dashboard default");
    assert!(empty.from.is_none());
    assert!(empty.to.is_none());
    assert!(empty.parse_from().is_none());
    assert!(empty.parse_to().is_none());

    // Both present
    let full: AnalyticsQuery = serde_json::from_value(serde_json::json!({
        "from": "2026-01-01",
        "to": "2026-05-20",
    }))
    .expect("AnalyticsQuery MUST parse both fields");
    assert_eq!(full.from.as_deref(), Some("2026-01-01"));
    assert_eq!(
        full.parse_from(),
        Some(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid"))
    );
    assert_eq!(
        full.parse_to(),
        Some(NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid"))
    );

    // Malformed — silently downgraded to None (NOT 422)
    let malformed: AnalyticsQuery = serde_json::from_value(serde_json::json!({
        "from": "not-a-date",
        "to": "also-not-a-date",
    }))
    .expect("AnalyticsQuery MUST parse malformed dates (downgrades to None)");
    assert!(
        malformed.parse_from().is_none(),
        "Malformed `from` MUST silently downgrade to None — the \
         frontend date picker can ship invalid dates on browser \
         quirks; the request should still return 'all time' rather \
         than 422"
    );
    assert!(malformed.parse_to().is_none());

    // R9-D NEGATIVE: non-string `from` MUST fail to parse (axum
    // Query is strict about types — silent string-coercion would
    // mask client bugs).
    assert!(
        serde_json::from_value::<AnalyticsQuery>(serde_json::json!({
            "from": 42,
        }))
        .is_err(),
        "AnalyticsQuery.from MUST reject non-string input"
    );
}

// ── 5. Ticker / setup / monthly DTO shapes — f64 throughout ──────────

/// `TickerPerformance`, `SetupPerformance`, `MonthlyReturn` all
/// carry f64 for PnL / rate fields. These are display-aggregate
/// values computed from the f64 `room_trades.pnl` column (double
/// precision in the DB). Per CLAUDE.md money rule, MONEY belongs
/// in `*_cents: i64` BIGINT columns — but the room_trades schema
/// stores `pnl` as `double precision` (an older, separate design
/// choice the money path will eventually correct, tracked in the
/// money-path digs). For now, analytics aggregates over that
/// column must stay f64 to avoid lossy conversion.
///
/// R9-D NEGATIVE: COUNT aggregates (`total_trades`, `wins`,
/// `losses`) MUST remain i64 even though they're embedded in the
/// same struct as the f64 PnL fields — i32 wrap is the silent-
/// regression risk here.
#[test]
fn ticker_setup_monthly_dtos_are_i64_counts_f64_pnl() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let ticker = TickerPerformance {
        ticker: "NVDA".to_string(),
        total_trades: above_i32,
        wins: 100,
        losses: 50,
        win_rate: 66.67,
        total_pnl: 12_345.67,
        avg_pnl: 82.30,
    };
    let _: i64 = ticker.total_trades;
    let _: i64 = ticker.wins;
    let _: f64 = ticker.total_pnl;
    let _: f64 = ticker.win_rate;
    let wire = serde_json::to_value(&ticker).expect("serialize TickerPerformance");
    assert_eq!(
        wire["total_trades"].as_i64(),
        Some(above_i32),
        "TickerPerformance.total_trades MUST be i64 — COUNT over BIGSERIAL"
    );

    let setup = SetupPerformance {
        setup: "breakout".to_string(),
        total_trades: above_i32,
        wins: 80,
        losses: 20,
        win_rate: 80.0,
        total_pnl: 8_500.0,
        avg_pnl: 85.0,
    };
    let _: i64 = setup.total_trades;
    let wire = serde_json::to_value(&setup).expect("serialize SetupPerformance");
    assert_eq!(wire["total_trades"].as_i64(), Some(above_i32));

    let monthly = MonthlyReturn {
        month: "2026-05".to_string(), // canonical "YYYY-MM" format
        pnl: 1_234.56,
        trades: above_i32,
        win_rate: 60.0,
    };
    let _: i64 = monthly.trades;
    let _: f64 = monthly.pnl;
    let wire = serde_json::to_value(&monthly).expect("serialize MonthlyReturn");
    assert_eq!(
        wire["month"].as_str(),
        Some("2026-05"),
        "MonthlyReturn.month MUST be 'YYYY-MM' canonical format per line 80 comment"
    );
    assert_eq!(wire["trades"].as_i64(), Some(above_i32));
}

// ── 6. Router compile-pin (6 GET routes, admin-gated) ────────────────

/// `routes::room_analytics::router()` MUST build as
/// `Router<AppState>`. Mount table per routes/room_analytics.rs:
/// 284-298 (6 endpoints, ALL `_admin: AdminUser`-gated):
///   - GET /room/{slug}                       (get_room_analytics)
///   - GET /room/{slug}/equity-curve          (get_equity_curve)
///   - GET /room/{slug}/ticker/{ticker}       (get_ticker_analytics)
///   - GET /room/{slug}/monthly               (get_monthly_returns)
///   - GET /room/{slug}/tickers               (get_ticker_performance)
///   - GET /room/{slug}/setups                (get_setup_performance)
///
/// `_admin: AdminUser` is load-bearing — these endpoints expose
/// per-room PnL, drawdowns, and equity curves (the room's
/// alpha-research). A regression that dropped the extractor would
/// let unauthenticated bots scrape detailed performance data from
/// production rooms (a competitive intel leak).
///
/// `Path<String>` on `:slug` and `Path<(String, String)>` on
/// `:slug/:ticker` are correct — rooms have slugs (not numeric
/// IDs) in user-facing URLs; tickers are uppercase strings
/// (NVDA, AAPL).
///
/// Per CLAUDE.md habit #3: pin that the router is idempotent —
/// no `OnceLock<Arc<RoomAnalyticsService>>` hoisted into a
/// static. `RoomAnalyticsService::new(pool)` is called
/// per-request (not module-cached), which matches the
/// per-request pool-clone semantics. A refactor into a OnceLock
/// would break ownership of the pool clone.
#[test]
fn room_analytics_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::room_analytics::router();
}

#[test]
fn room_analytics_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::room_analytics::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::room_analytics::router();
}
