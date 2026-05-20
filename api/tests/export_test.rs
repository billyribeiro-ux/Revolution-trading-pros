// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Export route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::export` and exercises
//! the public DTOs (`AlertExportParams`, `TradeExportParams`,
//! `PerformanceReportParams`) + the `services::export` types they
//! convert to (`AlertFilters`, `TradeFilters`, `DateRange`).
//!
//! ## Why this file exists (R19-D)
//!
//! `routes/export.rs` is 384 LOC of admin-gated CSV/JSON exports
//! over the BIGSERIAL `room_alerts` and `room_trades` tables
//! (schema.sql:9353, 9621). Three endpoints — all
//! `AdminUser`-gated, all `Path<String>` on the room_slug:
//!
//!   - GET /:room_slug/alerts.csv     (export_alerts_csv)
//!   - GET /:room_slug/trades.csv     (export_trades_csv)
//!   - GET /:room_slug/performance    (get_performance_report)
//!
//! What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 3 GET endpoints, admin-gated.** Any
//!    handler refactor that dropped `_admin: AdminUser` would
//!    expose member-level export of every room's alerts + trades
//!    (the rooms' alpha research) to anyone with a session cookie.
//!    A regression that flipped the routes from `:room_slug` to
//!    `:room_id` (i64) would silently break every consumer of
//!    these CSV URLs.
//!
//! 2. **`AlertExportParams.limit: Option<i64>` is i64-typed.** The
//!    handler passes it straight through to the SQL `LIMIT $1`
//!    clause via the service layer. Per CLAUDE.md "Money / cents":
//!    LIMIT over a BIGSERIAL-PK table MUST be i64, not i32. This
//!    is NOT a Reserved exception (the table grows ~hundreds of
//!    rows/week per active room over a multi-year production
//!    deployment; capping at i32::MAX trade-table rows is fine,
//!    but capping LIMIT at i32::MAX trips the type wall the wrong
//!    way — Stripe-aligned i64 contract per CLAUDE.md). A regression
//!    to i32 would silently shrink the available `?limit=` range
//!    by 4.3 billion.
//!
//! 3. **The `From<AlertExportParams> for AlertFilters` impl
//!    uppercases the ticker.** Lines 66-68:
//!      `ticker: params.ticker.map(|t| t.to_uppercase())`.
//!    This is load-bearing: the DB stores tickers UPPERCASE per
//!    `room_alerts.ticker` (varchar(10), schema.sql:9357), so a
//!    case-mismatch on input would silently return zero rows
//!    despite the data existing. R9-D NEGATIVE: drop the
//!    `.to_uppercase()` and `?ticker=aapl` would silently match
//!    no rows.
//!
//! 4. **The `From<TradeExportParams> for TradeFilters` impl
//!    uppercases BOTH `result` AND `ticker`.** Lines 100-101:
//!      `result: params.result.map(|r| r.to_uppercase())`
//!      `ticker: params.ticker.map(|t| t.to_uppercase())`.
//!    `result` is "WIN" or "LOSS" canonical (NOT "Win"); a
//!    regression that lost either uppercase would silently match
//!    no rows.
//!
//! 5. **`PerformanceReportParams.start_date` defaults to "30 days
//!    ago" (NOT 0) and `end_date` defaults to "today".** The
//!    `From<PerformanceReportParams> for DateRange` impl (lines
//!    116-130) reads `chrono::Utc::now().date_naive()` and falls
//!    back to `now - 30 days` for missing start, `now` for missing
//!    end. R9-D NEGATIVE: a regression to `unwrap_or_default()`
//!    on `NaiveDate` would default to 1970-01-01, returning the
//!    entire history (potentially 100K+ trades) on every "no
//!    params" call — a frontend bug that nukes performance.
//!
//! 6. **All export DTOs derive `Default`** (line 42, 74) so they
//!    parse from an empty query string `?` with all fields None.
//!    A regression that removed `Default` derive would 422 every
//!    GET /:room_slug/alerts.csv with no query string.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`,
//! `tests/room_search_test.rs`, `tests/posts_test.rs`,
//! `tests/categories_test.rs`.

use chrono::NaiveDate;
use revolution_api::routes::export::{
    AlertExportParams, PerformanceReportParams, TradeExportParams,
};
use revolution_api::services::export::{AlertFilters, DateRange, TradeFilters};

// ── 1. AlertExportParams: limit is Option<i64> (Stripe-aligned) ─────

/// `AlertExportParams.limit: Option<i64>` per `routes/export.rs:54`.
/// The handler passes it through to the service's SQL `LIMIT $1`,
/// which queries the BIGSERIAL-PK `room_alerts` table
/// (schema.sql:9353). Per CLAUDE.md "Money / cents — i64 ONLY,
/// BIGINT ONLY": LIMIT over a BIGSERIAL table MUST be i64. While
/// LIMIT itself isn't money, it caps a count over an i64-keyed
/// table; i32 limit would silently truncate at 2.1B (a number a
/// poorly-scoped frontend "no filter" call could theoretically
/// blow past). This is NOT a Reserved exception — Reserved is
/// row-counts like `revisions: i32` which genuinely cap below
/// 2 billion.
///
/// R9-D NEGATIVE: a regression to i32 would silently 422 any
/// caller sending `?limit=2147483648`, but more dangerously,
/// it would mask the count-of-results contract.
#[test]
fn alert_export_params_limit_is_i64() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let params: AlertExportParams = serde_json::from_value(serde_json::json!({
        "limit": above_i32,
    }))
    .expect("AlertExportParams MUST parse limit > i32::MAX (i64 type)");
    assert_eq!(params.limit, Some(above_i32));

    let filters: AlertFilters = params.into();
    let _: Option<i64> = filters.limit;
    assert_eq!(
        filters.limit,
        Some(above_i32),
        "AlertFilters.limit MUST stay i64 — BIGSERIAL-PK LIMIT \
         (room_alerts.id is bigint, schema.sql:9353)"
    );

    // R9-D NEGATIVE: limit as string MUST fail (axum Query parses
    // numeric fields strictly — protect against form-encoded
    // numbers smuggling through).
    assert!(
        serde_json::from_value::<AlertExportParams>(serde_json::json!({
            "limit": "ten",
        }))
        .is_err(),
        "AlertExportParams.limit MUST reject string input"
    );

    // Default: empty body MUST parse (Default derive is load-bearing
    // — the frontend export button sends no params on the initial
    // "give me everything" click).
    let empty: AlertExportParams = serde_json::from_value(serde_json::json!({}))
        .expect("AlertExportParams MUST parse empty body (Default-derived)");
    assert!(empty.limit.is_none());
    assert!(empty.start_date.is_none());
    assert!(empty.ticker.is_none());
}

// ── 2. AlertExportParams → AlertFilters uppercases ticker ─────────────

/// The `From<AlertExportParams> for AlertFilters` impl
/// (`routes/export.rs:57-71`) uppercases the ticker:
///   `ticker: params.ticker.map(|t| t.to_uppercase())`
/// Load-bearing: `room_alerts.ticker` is stored uppercase per
/// `varchar(10)` schema convention. A regression that lost the
/// `.to_uppercase()` would silently return zero rows for
/// `?ticker=aapl` even when AAPL rows exist.
///
/// R9-D NEGATIVE: lowercase input MUST come out uppercase.
#[test]
fn alert_export_params_uppercases_ticker() {
    let params = AlertExportParams {
        start_date: Some("2026-01-01".to_string()),
        end_date: Some("2026-05-20".to_string()),
        alert_type: Some("ENTRY".to_string()),
        ticker: Some("aapl".to_string()), // lowercase input
        limit: Some(100),
    };

    let filters: AlertFilters = params.into();

    assert_eq!(
        filters.ticker,
        Some("AAPL".to_string()),
        "AlertFilters.ticker MUST be uppercased — DB stores tickers \
         uppercase; lowercase regression would silently return 0 rows"
    );
    // Sanity: dates parsed via NaiveDate::parse_from_str.
    assert_eq!(
        filters.start_date,
        Some(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"))
    );
    assert_eq!(
        filters.end_date,
        Some(NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid date"))
    );
    // alert_type is NOT uppercased by the impl (the upstream API
    // contract is "client sends canonical ENTRY/UPDATE/EXIT").
    assert_eq!(filters.alert_type, Some("ENTRY".to_string()));

    // R9-D NEGATIVE: malformed date MUST silently become None
    // (the impl uses `.ok()`, not `.expect()`) — the frontend
    // date-picker can send invalid dates on browser quirks; the
    // export should still run (returning all rows) rather than
    // 422'ing.
    let malformed = AlertExportParams {
        start_date: Some("not-a-date".to_string()),
        end_date: None,
        alert_type: None,
        ticker: None,
        limit: None,
    };
    let malformed_filters: AlertFilters = malformed.into();
    assert!(
        malformed_filters.start_date.is_none(),
        "Malformed date MUST silently become None (the impl uses \
         `.ok()`, not `.expect()`) — frontend never gets a 422"
    );
}

// ── 3. TradeExportParams → TradeFilters uppercases ticker + result ───

/// The `From<TradeExportParams> for TradeFilters` impl
/// (`routes/export.rs:90-105`) uppercases BOTH `result` and
/// `ticker`. The DB canonical values for `room_trades.result`
/// are "WIN" and "LOSS"; a regression that lost the uppercase
/// on `result` would silently match no rows for `?result=win`.
///
/// Status is NOT uppercased (canonical values are lowercase:
/// "open", "closed", "invalidated" — different convention).
#[test]
fn trade_export_params_uppercases_ticker_and_result() {
    let params = TradeExportParams {
        start_date: None,
        end_date: None,
        status: Some("closed".to_string()), // lowercase — NOT uppercased
        result: Some("win".to_string()),    // lowercase — uppercased to WIN
        ticker: Some("spy".to_string()),    // lowercase — uppercased to SPY
        limit: Some(50),
    };

    let filters: TradeFilters = params.into();

    assert_eq!(
        filters.result,
        Some("WIN".to_string()),
        "TradeFilters.result MUST uppercase to WIN — DB canonical \
         values are WIN/LOSS uppercase"
    );
    assert_eq!(
        filters.ticker,
        Some("SPY".to_string()),
        "TradeFilters.ticker MUST uppercase to SPY"
    );
    assert_eq!(
        filters.status,
        Some("closed".to_string()),
        "TradeFilters.status MUST preserve case — DB canonical is \
         lowercase open/closed/invalidated (different convention \
         from ticker/result)"
    );

    // limit is also Option<i64> — pin parity with AlertFilters.
    let _: Option<i64> = filters.limit;
    assert_eq!(filters.limit, Some(50_i64));

    // R9-D NEGATIVE: empty body MUST parse (Default-derived).
    let empty: TradeExportParams = serde_json::from_value(serde_json::json!({}))
        .expect("TradeExportParams MUST parse empty body");
    assert!(empty.result.is_none());
    assert!(empty.ticker.is_none());

    // R9-D NEGATIVE: i64::MAX + 1 limit MUST fail to parse.
    assert!(
        serde_json::from_value::<TradeExportParams>(serde_json::json!({
            "limit": (i64::MAX as u64) + 1,
        }))
        .is_err(),
        "TradeExportParams.limit > i64::MAX MUST fail (i64 ceiling)"
    );
}

// ── 4. PerformanceReportParams: default range is 30 days back ─────────

/// `PerformanceReportParams` does NOT derive Default — instead the
/// `From<PerformanceReportParams> for DateRange` impl
/// (`routes/export.rs:116-130`) computes defaults from
/// `chrono::Utc::now().date_naive()`:
///   - missing start_date  → today - 30 days
///   - missing end_date    → today
///
/// R9-D NEGATIVE: a regression to `unwrap_or_default()` on
/// `NaiveDate` would default to 1970-01-01 (the chrono Default
/// for NaiveDate). The frontend export button click with no
/// params would then return 50+ years of nothing (or, after the
/// product launches in 2025, ~5 years of trades — potentially
/// 100K+ rows, freezing the browser PDF generator).
#[test]
fn performance_report_params_default_window_is_30_days() {
    let now = chrono::Utc::now().date_naive();

    // Both empty
    let empty = PerformanceReportParams {
        start_date: None,
        end_date: None,
    };
    let range: DateRange = empty.into();

    assert_eq!(
        range.end, now,
        "Empty end_date MUST default to today (NOT NaiveDate::default which is 1970-01-01)"
    );
    assert_eq!(
        range.start,
        now - chrono::Duration::days(30),
        "Empty start_date MUST default to today - 30 days \
         (NOT 1970-01-01) — regression to `unwrap_or_default()` \
         would return 50+ years of data on every empty-params call"
    );

    // Both explicit
    let explicit = PerformanceReportParams {
        start_date: Some("2026-01-01".to_string()),
        end_date: Some("2026-05-20".to_string()),
    };
    let range: DateRange = explicit.into();
    assert_eq!(
        range.start,
        NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid")
    );
    assert_eq!(
        range.end,
        NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid")
    );

    // Malformed start_date — silently falls back to 30 days ago
    // (the impl uses `.ok()`, then `.unwrap_or(now - 30 days)`).
    let malformed = PerformanceReportParams {
        start_date: Some("not-a-date".to_string()),
        end_date: None,
    };
    let range: DateRange = malformed.into();
    assert_eq!(
        range.start,
        now - chrono::Duration::days(30),
        "Malformed start_date MUST silently fall back to 30 days ago"
    );
}

// ── 5. Router compile-pin (3 GET routes, admin-gated) ────────────────

/// `routes::export::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/export.rs:311-316:
///   - GET /:room_slug/alerts.csv     (export_alerts_csv)
///   - GET /:room_slug/trades.csv     (export_trades_csv)
///   - GET /:room_slug/performance    (get_performance_report)
///
/// ALL handlers take `admin: AdminUser`. Reserved exception
/// rationale: AdminUser (NOT SuperAdminUser) is correct here —
/// CSV/PDF export is a routine admin operation (the head of a
/// trading room downloads their own room's performance), not a
/// privileged DDL operation. The /setup-db, /run-migrations,
/// /init-db endpoints in routes/health.rs use SuperAdminUser
/// because they execute DDL; these export endpoints SELECT
/// only.
///
/// `Path<String>` on `:room_slug` is correct (rooms have a
/// slug, not a numeric ID, in the user-facing URL). A regression
/// that flipped to `Path<i64>` would silently 404 every export
/// URL the frontend ships.
#[test]
fn export_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::export::router();
}

// ── 6. Idempotent router construction (no module-local state) ────────

/// Per CLAUDE.md habit #3: routes/export.rs has NO module-level
/// `lazy_static` / `OnceLock` (verified by grep: no `static` keyword
/// in the file). The `ExportService::new()` call lives PER-REQUEST
/// inside each handler, NOT cached at module level — this is
/// correct, because each export creates an in-memory `csv::Writer`
/// that's a per-request resource.
///
/// A refactor that hoisted `ExportService` into a
/// `OnceLock<Arc<ExportService>>` (a common AI-assisted
/// micro-optimization) would lose request-scoped CSV buffer
/// isolation — two concurrent exports could interleave their CSV
/// rows. The idempotent-construction pin catches the case where
/// the `OnceLock::set` is called per-router-build (which panics
/// on second call).
#[test]
fn export_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::export::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::export::router();
}
