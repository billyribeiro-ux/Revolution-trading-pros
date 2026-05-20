//! Room search route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::room_search` and
//! exercises the public DTOs (`SearchParams`, `SuggestionParams`,
//! `SearchApiResponse`, `AlertsSearchResponse`, `TradesSearchResponse`,
//! `TradePlansSearchResponse`, `SuggestionsResponse`) + the
//! `services::room_search` types that flow through these handlers
//! (`SearchFilters`, `Pagination`, `SearchableContentType`,
//! `AlertSearchResult`, `TradeSearchResult`, `TradePlanSearchResult`,
//! `SearchResults`, `SearchSuggestions`, `PaginationMeta`).
//!
//! ## Why this file exists (R16-D)
//!
//! `routes/room_search.rs` is 518 LOC of authenticated full-text
//! search over `room_alerts`, `room_trades`, `room_trade_plans`
//! (all BIGSERIAL i64 PKs, schema.sql:9353/9520/9621). Every
//! handler takes `_user: User` (login-gated, not admin-gated —
//! members can search content in rooms they belong to). The
//! handler performs a multi-table tsquery search via the
//! `RoomSearchService`. What we CAN pin in no-DB tests:
//!
//! 1. **PK types — `AlertSearchResult.id`, `TradeSearchResult.id`,
//!    `TradePlanSearchResult.id` are all `i64` (BIGSERIAL).**
//!
//!    All three search tables (`room_alerts`, `room_trades`,
//!    `room_trade_plans`) are `bigint NOT NULL` per schema.sql
//!    lines 9353, 9520, 9621. A regression to `i32` would silently
//!    break search result deserialization on any row past 2^31 —
//!    given trades alone accumulate hundreds of rows per active
//!    room per week, this ceiling is non-trivially close on a
//!    multi-year production deployment.
//!
//! 2. **`SearchParams.q` is required; everything else Optional or
//!    has a serde-default.**
//!
//!    The route handler `search_room` (line 158) trims `q` and
//!    400s on empty / >500 chars. `types`, `from`, `to`, `ticker`
//!    are Optional. `limit` and `offset` use `#[serde(default = "fn")]`
//!    (default_limit = 20, offset = 0). A regression that flipped
//!    any of those to required would 422 every search the
//!    autocomplete dropdown fires.
//!
//!    R9-D NEGATIVE: missing `q` MUST fail.
//!
//! 3. **`SearchParams.limit` is i32, NOT i64.** The handler clamps
//!    via `params.limit.clamp(1, 100)` — i32 fits comfortably.
//!    Per CLAUDE.md "Reserved exception": pagination-page-size
//!    counters are bounded (max 100 per the clamp). MONEY never
//!    qualifies, but `limit` doesn't represent money.
//!
//! 4. **`SearchableContentType` serde-renames to snake_case** —
//!    the enum has `#[serde(rename_all = "snake_case")]` so
//!    `Alerts → "alerts"`, `Trades → "trades"`, `TradePlans →
//!    "trade_plans"` on the wire. The handler `parse_filters`
//!    (line 432) also accepts "tradeplans" as a soft alias via
//!    string matching, but the canonical serde wire is
//!    `"trade_plans"`. Pin the wire format.
//!
//! 5. **Search-result relevance scores are f32** — `relevance_score:
//!    f32` on all three result types. This is a tsquery rank, not
//!    money. A regression to f64 would double the bandwidth and
//!    silently invalidate the GIN index optimisations.
//!
//! 6. **Router mount table — 5 routes, ALL user-authenticated.**
//!    A regression that dropped the `_user: User` extractor from
//!    any handler would let unauthenticated bots scrape alpha
//!    research from production rooms.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_popups_test.rs`,
//! `tests/room_content_test.rs`, `tests/trading_rooms_test.rs`,
//! `tests/posts_test.rs`.

use chrono::{DateTime, NaiveDate, Utc};
use revolution_api::routes::room_search::{
    AlertsSearchResponse, SearchApiResponse, SearchParams, SuggestionParams, SuggestionsResponse,
    TradePlansSearchResponse, TradesSearchResponse,
};
use revolution_api::services::room_search::{
    AlertSearchResult, Pagination, SearchFilters, SearchResults, SearchSuggestions,
    SearchableContentType, TradePlanSearchResult, TradeSearchResult,
};

// ── 1. PK types — all search results use i64 BIGSERIAL ──────────────

/// All three room search tables (`room_alerts`, `room_trades`,
/// `room_trade_plans`) are `bigint NOT NULL` per schema.sql:9353,
/// 9520, 9621. The Rust search-result structs mirror with `id:
/// i64`. A regression to i32 would silently break deserialization
/// on any production row past 2^31 — trades alone accumulate
/// hundreds of rows per active room per week.
#[test]
fn search_result_pks_are_i64_bigserial() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let alert = AlertSearchResult {
        id: above_i32,
        ticker: "NVDA".to_string(),
        title: Some("Breakout alert".to_string()),
        alert_type: "breakout".to_string(),
        message: "NVDA breaking $1000".to_string(),
        published_at: DateTime::<Utc>::default(),
        relevance_score: 0.85,
        highlight: "<b>NVDA</b> breaking".to_string(),
    };
    let _: i64 = alert.id;
    let alert_wire = serde_json::to_value(&alert).expect("serialize AlertSearchResult");
    assert_eq!(
        alert_wire["id"].as_i64(),
        Some(above_i32),
        "AlertSearchResult.id MUST be i64 — room_alerts.id is bigint (schema.sql:9353)"
    );

    let trade = TradeSearchResult {
        id: above_i32,
        ticker: "AAPL".to_string(),
        direction: "long".to_string(),
        status: "closed".to_string(),
        entry_price: 180.5,
        exit_price: Some(195.25),
        entry_date: NaiveDate::from_ymd_opt(2026, 5, 1).expect("valid"),
        exit_date: Some(NaiveDate::from_ymd_opt(2026, 5, 15).expect("valid")),
        pnl_percent: Some(8.17),
        result: Some("win".to_string()),
        notes: Some("Held two weeks".to_string()),
        relevance_score: 0.92,
        highlight: "<b>AAPL</b> long".to_string(),
    };
    let _: i64 = trade.id;
    let _: f32 = trade.relevance_score;
    // entry_price IS f64 — these are display prices, NOT money cents.
    // Per CLAUDE.md money rule, real money belongs in `*_cents: i64`
    // columns; trade-history prices stored as f64 are display-only
    // and never used for arithmetic-summed financial totals here.
    let _: f64 = trade.entry_price;
    let _: Option<f64> = trade.exit_price;
    let trade_wire = serde_json::to_value(&trade).expect("serialize TradeSearchResult");
    assert_eq!(trade_wire["id"].as_i64(), Some(above_i32));

    let plan = TradePlanSearchResult {
        id: above_i32,
        ticker: "TSLA".to_string(),
        bias: "long".to_string(),
        entry: Some("250".to_string()),
        target1: Some("280".to_string()),
        stop: Some("240".to_string()),
        week_of: NaiveDate::from_ymd_opt(2026, 5, 19).expect("valid"),
        notes: None,
        is_active: true,
        relevance_score: 0.71,
        highlight: "<b>TSLA</b> long".to_string(),
    };
    let _: i64 = plan.id;
    let plan_wire = serde_json::to_value(&plan).expect("serialize TradePlanSearchResult");
    assert_eq!(plan_wire["id"].as_i64(), Some(above_i32));
}

// ── 2. SearchParams — q required, types/dates/ticker Optional ───────

/// `SearchParams.q` is the only required field — the handler
/// trims it (line 158) and 400s on empty or >500 chars. Every
/// other field is Optional or has a serde-default.
///
/// `limit: i32` and `offset: i32` carry `#[serde(default = "fn")]`
/// (default_limit = 20, offset = 0). A query string of
/// `?q=NVDA` MUST parse with defaults applied.
#[test]
fn search_params_q_required_others_default() {
    // Minimal — only `q`
    let minimal: SearchParams = serde_json::from_value(serde_json::json!({
        "q": "NVDA",
    }))
    .expect("SearchParams with only q MUST parse (defaults fill in)");
    assert_eq!(minimal.q, "NVDA");
    assert!(minimal.types.is_none());
    assert!(minimal.from.is_none());
    assert!(minimal.to.is_none());
    assert!(minimal.ticker.is_none());
    assert_eq!(minimal.limit, 20_i32, "limit MUST default to 20");
    assert_eq!(minimal.offset, 0_i32, "offset MUST default to 0");

    // Full
    let full: SearchParams = serde_json::from_value(serde_json::json!({
        "q": "breakout",
        "types": "alerts,trades",
        "from": "2026-01-01",
        "to": "2026-05-20",
        "ticker": "NVDA",
        "limit": 50,
        "offset": 100,
    }))
    .expect("Full SearchParams MUST parse");
    assert_eq!(full.q, "breakout");
    assert_eq!(full.types.as_deref(), Some("alerts,trades"));
    assert_eq!(full.limit, 50_i32);
    assert_eq!(full.offset, 100_i32);

    // R9-D NEGATIVE: missing q MUST fail
    assert!(
        serde_json::from_value::<SearchParams>(serde_json::json!({
            "types": "alerts",
        }))
        .is_err(),
        "SearchParams without `q` MUST fail (only required field)"
    );

    // R9-D NEGATIVE: q as non-string MUST fail
    assert!(
        serde_json::from_value::<SearchParams>(serde_json::json!({
            "q": 42,
        }))
        .is_err(),
        "SearchParams.q MUST reject non-string"
    );

    // R9-D NEGATIVE: limit > i32::MAX MUST fail
    // (the Rust type is i32 — Reserved exception, page-size bounded
    // by the handler's `.clamp(1, 100)` to a max of 100).
    assert!(
        serde_json::from_value::<SearchParams>(serde_json::json!({
            "q": "NVDA",
            "limit": (i32::MAX as i64) + 1,
        }))
        .is_err(),
        "SearchParams.limit > i32::MAX MUST fail — i32 is Reserved exception (page-size bounded to ≤100)"
    );

    // R9-D NEGATIVE: limit as string MUST fail
    assert!(
        serde_json::from_value::<SearchParams>(serde_json::json!({
            "q": "NVDA",
            "limit": "ten",
        }))
        .is_err(),
        "SearchParams.limit MUST reject string input"
    );
}

// ── 3. SuggestionParams — q required, limit defaults to 10 ──────────

/// `SuggestionParams.q` is the autocomplete prefix (required).
/// `limit: i32` defaults to 10 via `#[serde(default = "fn")]`. The
/// autocomplete dropdown fires on every keystroke after the user
/// types 2+ chars, so this DTO has to parse with just `q`.
#[test]
fn suggestion_params_q_required_limit_defaults() {
    let minimal: SuggestionParams = serde_json::from_value(serde_json::json!({
        "q": "NV",
    }))
    .expect("SuggestionParams with only q MUST parse");
    assert_eq!(minimal.q, "NV");
    assert_eq!(minimal.limit, 10_i32, "default suggestion limit MUST be 10");

    let with_limit: SuggestionParams = serde_json::from_value(serde_json::json!({
        "q": "AAPL",
        "limit": 25,
    }))
    .expect("SuggestionParams with limit MUST parse");
    assert_eq!(with_limit.limit, 25_i32);

    // R9-D NEGATIVE: missing q MUST fail
    assert!(
        serde_json::from_value::<SuggestionParams>(serde_json::json!({
            "limit": 5,
        }))
        .is_err(),
        "SuggestionParams without q MUST fail"
    );

    // R9-D NEGATIVE: empty body MUST fail
    assert!(
        serde_json::from_value::<SuggestionParams>(serde_json::json!({})).is_err(),
        "SuggestionParams empty body MUST fail (q required)"
    );
}

// ── 4. SearchableContentType serde wire is snake_case ────────────────

/// The enum has `#[serde(rename_all = "snake_case")]` (services/
/// room_search.rs:93). Wire format: `"alerts"`, `"trades"`,
/// `"trade_plans"`. A regression to camelCase or to the variant
/// name `TradePlans` would silently break the response JSON
/// (the unified search result `#[serde(tag = "type")]` enum
/// embeds these tags in every result row).
#[test]
fn searchable_content_type_serializes_snake_case() {
    let alerts = serde_json::to_value(SearchableContentType::Alerts).expect("ser Alerts");
    assert_eq!(
        alerts.as_str(),
        Some("alerts"),
        "Alerts MUST serialize to \"alerts\" (snake_case rename)"
    );

    let trades = serde_json::to_value(SearchableContentType::Trades).expect("ser Trades");
    assert_eq!(trades.as_str(), Some("trades"));

    let plans = serde_json::to_value(SearchableContentType::TradePlans).expect("ser TradePlans");
    assert_eq!(
        plans.as_str(),
        Some("trade_plans"),
        "TradePlans MUST serialize to \"trade_plans\" (snake_case rename — NOT \"tradePlans\")"
    );

    // Round-trip: deserialize wire string back to enum
    let back: SearchableContentType =
        serde_json::from_value(serde_json::json!("trade_plans")).expect("de trade_plans");
    assert!(matches!(back, SearchableContentType::TradePlans));

    // R9-D NEGATIVE: camelCase MUST NOT deserialize
    assert!(
        serde_json::from_value::<SearchableContentType>(serde_json::json!("tradePlans")).is_err(),
        "tradePlans (camelCase) MUST NOT deserialize — wire is snake_case"
    );

    // R9-D NEGATIVE: PascalCase variant name MUST NOT deserialize
    assert!(
        serde_json::from_value::<SearchableContentType>(serde_json::json!("TradePlans")).is_err(),
        "PascalCase variant MUST NOT deserialize"
    );

    // R9-D NEGATIVE: unknown variant MUST fail
    assert!(
        serde_json::from_value::<SearchableContentType>(serde_json::json!("unknown")).is_err(),
        "Unknown variant MUST fail to deserialize"
    );

    // Display impl mirrors snake_case wire (relied on for tsquery construction)
    assert_eq!(format!("{}", SearchableContentType::Alerts), "alerts");
    assert_eq!(format!("{}", SearchableContentType::Trades), "trades");
    assert_eq!(
        format!("{}", SearchableContentType::TradePlans),
        "trade_plans"
    );
}

// ── 5. SearchResults aggregate counts are i64; relevance is f32 ─────

/// `SearchResults.total_count: i64` (cross-table COUNT(*) — the
/// total may exceed i32::MAX over years of room activity). The
/// per-result `relevance_score: f32` (tsquery rank — single-
/// precision is the PostgreSQL `real` column type, doubling to
/// f64 would silently invalidate ts_rank_cd output).
///
/// `SearchResults.took_ms: u64` (timing in milliseconds —
/// `Duration::as_millis()` returns u128 but the field is u64,
/// truncation is intentional). `Pagination.limit/offset` are i32
/// (Reserved exception — page parameters bounded by the handler's
/// .clamp(1, 100) and .max(0)).
#[test]
fn search_results_total_count_i64_relevance_f32() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let results = SearchResults {
        alerts: vec![],
        trades: vec![],
        trade_plans: vec![],
        total_count: above_i32,
        query: "NVDA".to_string(),
        took_ms: 42_u64,
        pagination: revolution_api::services::room_search::PaginationMeta {
            limit: 20,
            offset: 0,
            has_more: true,
        },
    };
    let _: i64 = results.total_count;
    let _: u64 = results.took_ms;
    let _: i32 = results.pagination.limit;
    let _: i32 = results.pagination.offset;

    let wire = serde_json::to_value(&results).expect("serialize SearchResults");
    assert_eq!(
        wire["total_count"].as_i64(),
        Some(above_i32),
        "SearchResults.total_count MUST be i64 — aggregate across all 3 search tables"
    );
    assert_eq!(wire["pagination"]["limit"].as_i64(), Some(20));
    assert_eq!(wire["pagination"]["has_more"].as_bool(), Some(true));

    // SearchFilters — value types
    let filters = SearchFilters {
        types: vec![
            SearchableContentType::Alerts,
            SearchableContentType::TradePlans,
        ],
        from_date: Some(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid")),
        to_date: Some(NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid")),
        ticker: Some("NVDA".to_string()),
    };
    assert_eq!(filters.types.len(), 2);
    assert_eq!(filters.ticker.as_deref(), Some("NVDA"));

    // Pagination — i32 (Reserved exception, page-bounded)
    let page = Pagination {
        limit: 100,
        offset: 0,
    };
    let _: i32 = page.limit;
    let _: i32 = page.offset;

    // Suggestions — tickers Vec<String>
    let sugg = SearchSuggestions {
        tickers: vec!["NVDA".to_string(), "NDX".to_string()],
    };
    let sugg_wire = serde_json::to_value(&sugg).expect("serialize SearchSuggestions");
    assert_eq!(sugg_wire["tickers"][0].as_str(), Some("NVDA"));
}

// ── 6. Response wrappers carry took_ms u64, total usize ──────────────

/// `AlertsSearchResponse`, `TradesSearchResponse`,
/// `TradePlansSearchResponse` all carry `took_ms: u64`, `total:
/// usize`. The `total` is the `.len()` of the returned vec
/// (handler `search_alerts` line 259 — `let total = results.len()`),
/// so usize is correct. R9-D NEGATIVE pin: usize is platform-
/// dependent BUT JSON-serializes as a number — confirm the wire
/// shape doesn't accidentally serialize as a string.
#[test]
fn search_response_wrappers_carry_took_ms_total() {
    let alerts_resp = AlertsSearchResponse {
        success: true,
        data: vec![],
        query: "test".to_string(),
        took_ms: 50,
        total: 0,
    };
    let wire = serde_json::to_value(&alerts_resp).expect("serialize AlertsSearchResponse");
    assert_eq!(wire["success"].as_bool(), Some(true));
    assert_eq!(wire["took_ms"].as_u64(), Some(50));
    assert_eq!(
        wire["total"].as_u64(),
        Some(0),
        "AlertsSearchResponse.total (usize) MUST serialize as JSON number, not string"
    );
    assert_eq!(wire["query"].as_str(), Some("test"));
    assert!(wire["data"].is_array());

    // TradesSearchResponse parallel shape
    let trades_resp = TradesSearchResponse {
        success: true,
        data: vec![],
        query: "long".to_string(),
        took_ms: 12,
        total: 0,
    };
    let wire = serde_json::to_value(&trades_resp).expect("serialize TradesSearchResponse");
    assert_eq!(wire["took_ms"].as_u64(), Some(12));

    // TradePlansSearchResponse parallel shape
    let plans_resp = TradePlansSearchResponse {
        success: true,
        data: vec![],
        query: "weekly".to_string(),
        took_ms: 7,
        total: 0,
    };
    let wire = serde_json::to_value(&plans_resp).expect("serialize TradePlansSearchResponse");
    assert_eq!(wire["took_ms"].as_u64(), Some(7));

    // SearchApiResponse wraps SearchResults
    let api_resp = SearchApiResponse {
        success: true,
        data: SearchResults {
            alerts: vec![],
            trades: vec![],
            trade_plans: vec![],
            total_count: 0,
            query: "NVDA".to_string(),
            took_ms: 25,
            pagination: revolution_api::services::room_search::PaginationMeta {
                limit: 20,
                offset: 0,
                has_more: false,
            },
        },
    };
    let wire = serde_json::to_value(&api_resp).expect("serialize SearchApiResponse");
    assert_eq!(wire["data"]["took_ms"].as_u64(), Some(25));

    // SuggestionsResponse wraps SearchSuggestions
    let sugg_resp = SuggestionsResponse {
        success: true,
        data: SearchSuggestions {
            tickers: vec!["NVDA".to_string()],
        },
    };
    let wire = serde_json::to_value(&sugg_resp).expect("serialize SuggestionsResponse");
    assert_eq!(wire["data"]["tickers"][0].as_str(), Some("NVDA"));
}

// ── 7. Router mount-table compile-pin (5 routes, user-gated) ────────

/// `routes::room_search::router()` MUST build as `Router<AppState>`.
/// Mount table (all `_user: User`-gated — login required, but NOT
/// admin-only since members search content in rooms they join):
///   - GET /:room_slug                  (search_room)
///   - GET /:room_slug/alerts           (search_alerts)
///   - GET /:room_slug/trades           (search_trades)
///   - GET /:room_slug/trade-plans      (search_trade_plans)
///   - GET /:room_slug/suggestions      (get_suggestions)
///
/// The `_user: User` extractor is load-bearing. A regression that
/// dropped it from any handler (e.g., "let's expose suggestions
/// publicly for SEO") would leak production trading alpha to
/// unauthenticated scrapers. The compile-pin catches signature
/// drift; the runtime auth gate is enforced by the `User`
/// extractor itself.
///
/// `Path<String>` on `:room_slug` is correct (rooms have a slug,
/// not a numeric ID, in the user-facing URL).
#[test]
fn room_search_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::room_search::router();
}

/// Idempotent construction. Per CLAUDE.md habit #3: pin that
/// nothing global (no `OnceLock`, no `static mut`) lives inside
/// the router constructor. The `RoomSearchService::new(pool)`
/// call is per-request, not cached — a refactor that hoisted
/// the service into a `OnceLock<Arc<RoomSearchService>>` would
/// break ownership semantics around the pool clone and silently
/// fail this test the second time `router()` runs.
#[test]
fn room_search_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::room_search::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::room_search::router();
}
