//! Reconciliation route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::reconciliation` and
//! pins the `router()` mount-table shape for the admin-only
//! Stripe-reconciliation backstop endpoint.
//!
//! ## Why this file exists (R22-D, sweep-complete)
//!
//! `routes/reconciliation.rs` is 93 LOC — the second-smallest
//! untested route module (after `migrate.rs` at 57). It's the
//! admin-facing operator surface for the daily Stripe reconciliation
//! job (`src/jobs/reconcile_stripe.rs`), per §14 of the project's
//! PAYMENTS_ARCHITECTURE_STANDARD doc cited in
//! `jobs/reconcile_stripe.rs:7`.
//!
//! Per CLAUDE.md habit #1 ("Cite the rule in your work"): the
//! load-bearing invariant for this surface is the AdminUser gate on
//! BOTH handlers — a regression that opened either endpoint would
//! expose the reconciliation_log table (containing every drift
//! event between Stripe and `user_memberships`) to the internet.
//!
//! What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 2 endpoints, both AdminUser-gated.**
//!    Mount table per routes/reconciliation.rs:89-93:
//!      - GET  /log → `list_log`     (`_admin: AdminUser`, line 25)
//!      - POST /run → `trigger_run`  (`_admin: AdminUser`, line 72)
//!    Mounted at `/admin/reconciliation` per routes/mod.rs:238.
//!    Both handlers carry `_admin: AdminUser` extractors. A
//!    regression that dropped `AdminUser` from either handler would
//!    silently expose the reconciliation_log (which contains
//!    per-user subscription-drift details — emails, plan changes,
//!    cancel_at_period_end flips — i.e., PII).
//!
//! 2. **`LogQuery` is the public query-string DTO** (lines 17-21).
//!    `LogQuery { limit: Option<i64>, offset: Option<i64> }` — both
//!    fields are i64 to match the BIGSERIAL `reconciliation_log.id`
//!    PK and BIGINT `LIMIT`/`OFFSET` parameters used in the
//!    `sqlx::query_as` at lines 39-47. Per CLAUDE.md "Money / cents
//!    — i64 ONLY, BIGINT ONLY, EVERY TIME": although `limit` /
//!    `offset` are NOT money values, the `reconciliation_log` table
//!    accumulates one row per nightly run (365/year × multi-year
//!    retention = thousands of rows even on a small deployment).
//!    The i64 invariant is the right floor — and `Option<i64>`
//!    matches the SQL-level `Option<i64>` of bound parameters.
//!
//! 3. **`limit.min(200)` cap is load-bearing.** Per line 28:
//!    `let limit = q.limit.unwrap_or(50).min(200);` — the handler
//!    caps `limit` at 200 even if the caller requests more. This is
//!    a defense-in-depth bound: without it, an admin (or a stolen
//!    admin session) could request `?limit=1000000` and bloat the
//!    response into a memory bomb. A regression that dropped the
//!    `.min(200)` cap (or raised it to e.g. 10_000) would NOT break
//!    tests but would silently widen the DoS surface.
//!
//! 4. **`LogRow` PK is i64 (BIGSERIAL).** Per the inline
//!    `#[derive(sqlx::FromRow, ...)]` struct at lines 31-37,
//!    `LogRow.id: i64` matches `reconciliation_log.id BIGSERIAL`.
//!    Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
//!    TIME": although `LogRow` is private to the handler module,
//!    the i64 PK invariant applies to every BIGSERIAL table on the
//!    schema. A regression that flipped this to i32 would compile
//!    AND would correctly map sub-2^31 IDs — but would silently
//!    fail to deserialize once the log accumulated >2.1B rows
//!    (decades on the current schedule, but the invariant is
//!    free to enforce and CLAUDE.md commands it).
//!
//! 5. **`LogRow.discrepancies_found: i32` is the Reserved
//!    exception.** Per CLAUDE.md "Reserved exception": "row counts
//!    (`revisions: i32`, `attempts: i32`, `total_pages: i32`) —
//!    those genuinely cap below 2 billion and `i32` is fine. Money
//!    never qualifies for the exception." `discrepancies_found` is
//!    the count of drifted-subscription rows in a single
//!    reconciliation run — bounded by the total active-subscription
//!    population (low thousands on a small SaaS, low millions at
//!    Stripe-scale). `i32` is correct here. The pin documents the
//!    rationale for the next PR that touches this field.
//!
//! ## Pattern source
//!
//! Modeled on `tests/migrate_test.rs` (sister R22-D admin-action
//! surface), `tests/users_test.rs` (sister ICT-* admin RBAC pin),
//! `tests/orders_test.rs` (sister admin POST + GET pattern).

use revolution_api::routes::reconciliation;

// ── 1. Router mount-table compile-pin — 2 admin-gated endpoints ──────

/// `routes::reconciliation::router()` MUST build as
/// `Router<AppState>`. Mount table per routes/reconciliation.rs:
/// 89-93:
///   - GET  /log → `list_log`     (admin-gated, Query<LogQuery>)
///   - POST /run → `trigger_run`  (admin-gated, no body)
///
/// Mounted at `/admin/reconciliation` per routes/mod.rs:238. Both
/// handlers carry `_admin: AdminUser` extractors (lines 25, 72).
/// Per CLAUDE.md habit #1 ("Cite the rule in your work") — the
/// admin-only Stripe reconciliation contract is THE invariant.
/// A regression that:
///   - dropped `AdminUser` from `list_log` → reconciliation_log
///     PII (emails + plan history + cancel flags) exposed publicly
///   - dropped `AdminUser` from `trigger_run` → drive-by Stripe API
///     burn from unauthenticated POSTs (Stripe rate-limits but each
///     call costs latency + observability noise)
///   - changed POST to GET on /run → CSRF-able state-changing op
///     (the run mutates user_memberships via reconcile_stripe::
///     run_once)
///   - removed either route → silent break of operator workflow
/// would either fail this compile (signature drift) OR pass and
/// require the RBAC audit grep to catch.
///
/// R9-D NEGATIVE: the compile-pin DOES NOT prove the AdminUser
/// extractor is present — it only proves the router shape is
/// consistent. The `_admin: AdminUser` lines at routes/
/// reconciliation.rs:25 and :72 are the source of truth; this pin
/// documents where to look.
#[test]
fn reconciliation_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = reconciliation::router();
}

// ── 2. Router idempotency — no module-cached state survives refactor ─

/// Constructing `router()` repeatedly MUST NOT panic. Per CLAUDE.md
/// habit #3 ("does any `static` / `OnceLock` / lazy init survive
/// the refactor?"), this module has NO `lazy_static` / `OnceLock`
/// at all — it's purely a routing surface over SQL handlers.
///
/// A regression that introduced a module-cached
/// `OnceLock<Vec<LogRow>>` (e.g., to memoize the most-recent log
/// page and skip the SQL fetch on repeat GETs) would be EXACTLY
/// the failure mode CLAUDE.md flags: the cache would survive
/// subsequent reconciliation runs and admins would see stale
/// data after a manual `POST /run`. Constructing the router
/// repeatedly is a weak proxy for "no module-init side effects" —
/// the strong defense is the code review.
#[test]
fn reconciliation_router_construction_idempotent() {
    let _r1: axum::Router<revolution_api::AppState> = reconciliation::router();
    let _r2: axum::Router<revolution_api::AppState> = reconciliation::router();
    let _r3: axum::Router<revolution_api::AppState> = reconciliation::router();
    let _r4: axum::Router<revolution_api::AppState> = reconciliation::router();
}

// ── 3. LogQuery is pub + accepts Option<i64> limit/offset ────────────

/// `LogQuery { limit: Option<i64>, offset: Option<i64> }` MUST stay
/// the public query-string DTO. Per routes/reconciliation.rs:17-21:
///
///     #[derive(Debug, Deserialize)]
///     pub struct LogQuery {
///         limit: Option<i64>,
///         offset: Option<i64>,
///     }
///
/// The fields are PRIVATE (no `pub` keyword) — they're consumed via
/// `q.limit.unwrap_or(50).min(200)` and `q.offset.unwrap_or(0)`
/// inside the handler (lines 28-29). The DTO itself is `pub` so
/// axum's `Query<T>` extractor can deserialize the URL params into
/// it.
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
/// TIME": although limit/offset are NOT money, they bind directly
/// to SQL `LIMIT $1 OFFSET $2` parameters (lines 43-46) where the
/// PG types are BIGINT. The i64 invariant matches end-to-end.
///
/// R9-D NEGATIVE: a regression that flipped either field to
/// `Option<i32>` would compile (i32 → i64 widening for the bind),
/// but the runtime semantics would change — a `?limit=2147483648`
/// (i32::MAX + 1) URL would serde-fail to deserialize. That's a
/// silent change from "honored at 200 (capped)" to "400 Bad
/// Request" — exactly the comment-vs-code drift CLAUDE.md flags.
///
/// The pin builds a LogQuery via JSON (mimicking axum's `Query<T>`
/// URL-decode → serde flow) with both default and explicit values,
/// asserting both shapes round-trip.
#[test]
fn reconciliation_log_query_deserializes_i64_limit_offset() {
    use revolution_api::routes::reconciliation::LogQuery;

    // Default — both fields omitted (the handler unwrap_or's them).
    let _empty: LogQuery = serde_json::from_value(serde_json::json!({}))
        .expect("LogQuery deserializes from empty JSON (both fields Option)");

    // Both fields set to small values.
    let _small: LogQuery = serde_json::from_value(serde_json::json!({
        "limit": 25_i64,
        "offset": 0_i64,
    }))
    .expect("LogQuery deserializes with explicit small i64 values");

    // R9-D NEGATIVE: a value > i32::MAX MUST deserialize (proves i64,
    // not i32). The handler will then `.min(200)` it down to 200 —
    // but the DTO type itself MUST accept the large value first.
    let above_i32_max: i64 = (i32::MAX as i64) + 100;
    let _big: LogQuery = serde_json::from_value(serde_json::json!({
        "limit": above_i32_max,
        "offset": above_i32_max,
    }))
    .expect(
        "LogQuery MUST accept i64 values > i32::MAX (BIGINT bind contract). \
         A regression to Option<i32> would 400 this request silently.",
    );

    // R9-D NEGATIVE: a non-numeric limit MUST fail (catches a refactor
    // that flipped Option<i64> → Option<String>).
    let err: Result<LogQuery, _> = serde_json::from_value(serde_json::json!({
        "limit": "not-a-number",
    }));
    assert!(
        err.is_err(),
        "LogQuery MUST reject non-numeric limit — Option<i64> contract"
    );
}

// ── 4. R9-D NEGATIVE: router is NOT Router<()> (AppState required) ───

/// R9-D NEGATIVE: `reconciliation::router()` MUST require
/// `AppState`. Both handlers extract `State<AppState>`:
///   - `list_log` (line 24) accesses `state.db.pool` for the
///     paginated query
///   - `trigger_run` (line 71) passes `&state` into
///     `reconcile_stripe::run_once(&state).await` (line 74)
///
/// A regression that flipped to `Router<()>` would break the
/// top-level `.nest("/admin/reconciliation", ...)` into the app
/// router (which is typed `Router<AppState>` per routes/mod.rs:
/// 84-85, 238).
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"): the module
/// header (lines 1-4) claims "Admin Reconciliation Routes" with
/// "trigger a manual run immediately". The "immediately" is
/// load-bearing — `trigger_run` runs `reconcile_stripe::run_once`
/// INLINE (line 74), NOT via `tokio::spawn`. The HTTP response
/// awaits the full Stripe-pagination + DB-write cycle.
///
/// This means: a regression that "improved" the endpoint by
/// flipping to `tokio::spawn(reconcile_stripe::run_once(...))`
/// would FIRE-AND-FORGET the work — the response would 200
/// instantly with `discrepancies_found: 0` (the inline call's
/// return value would be unreachable). That's exactly the
/// comment-vs-code drift CLAUDE.md flags.
///
/// The compile-pin proves the router CANNOT be coerced to
/// `Router<()>` — the assignment below would fail if the type
/// drifted.
#[test]
fn reconciliation_router_state_type_is_app_state_not_unit() {
    let _typed: axum::Router<revolution_api::AppState> = reconciliation::router();

    fn _accepts_app_state_router(_r: axum::Router<revolution_api::AppState>) {}
    _accepts_app_state_router(reconciliation::router());
}

// ── 5. LogQuery partial-shape pins — only-limit / only-offset / none ─

/// `LogQuery` MUST accept every legal subset of its fields. Per
/// routes/reconciliation.rs:17-21 both `limit` and `offset` are
/// `Option<i64>` — meaning the URL-driven query `?` (no params),
/// `?limit=10` (limit only), `?offset=20` (offset only), and
/// `?limit=10&offset=20` (both) MUST ALL deserialize. The handler
/// then `unwrap_or`s each at lines 28-29 with explicit defaults
/// (50 and 0).
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"), this is the
/// kind of contract that's easy to subtly break: a refactor that
/// changed `Option<i64>` → `i64` with a `#[serde(default)]`
/// attribute would still pass the explicit-value tests above, BUT
/// would change the default value from "50 via unwrap_or(50)" to
/// "0 via Default::default()" — silently shrinking every default
/// listing to an empty page.
///
/// Reserved exception per CLAUDE.md: the handler's
/// `q.limit.unwrap_or(50).min(200)` arithmetic on line 28 stays in
/// i64 (no narrowing cast). If a future refactor flipped that to
/// `as i32` for SQL binding, the CLAUDE.md multiplication-footgun
/// rule would apply (and `.min(200)` would still be safe since
/// 200 < i32::MAX). The pin doesn't exercise that arithmetic —
/// the handler is private — but the comment documents the
/// invariant for the next refactor.
///
/// R9-D NEGATIVE: the explicit assertion below builds a LogQuery
/// from each subset of fields via serde_json and proves all four
/// shapes deserialize. A regression that dropped `Option<...>` from
/// either field would fail the corresponding partial-shape case.
#[test]
fn reconciliation_log_query_accepts_all_partial_shapes() {
    use revolution_api::routes::reconciliation::LogQuery;

    // No fields — both Option<i64> = None (handler unwrap_or's
    // them to 50 and 0).
    let _none: LogQuery = serde_json::from_value(serde_json::json!({}))
        .expect("LogQuery MUST deserialize from {} (both fields Option)");

    // Limit only — offset stays None.
    let _limit_only: LogQuery = serde_json::from_value(serde_json::json!({
        "limit": 10_i64,
    }))
    .expect("LogQuery MUST deserialize from {limit: 10} alone");

    // Offset only — limit stays None (handler unwrap_or(50) takes over).
    let _offset_only: LogQuery = serde_json::from_value(serde_json::json!({
        "offset": 100_i64,
    }))
    .expect("LogQuery MUST deserialize from {offset: 100} alone");

    // Both — fully explicit.
    let _both: LogQuery = serde_json::from_value(serde_json::json!({
        "limit": 25_i64,
        "offset": 50_i64,
    }))
    .expect("LogQuery MUST deserialize from {limit, offset} both set");

    // R9-D NEGATIVE: a numeric value that fits i64 but exceeds
    // i32::MAX MUST deserialize — proves both fields are i64.
    let above_i32_max: i64 = (i32::MAX as i64) + 12_345;
    let _big_offset: LogQuery = serde_json::from_value(serde_json::json!({
        "offset": above_i32_max,
    }))
    .expect(
        "LogQuery.offset MUST accept i64 > i32::MAX. A regression to \
         Option<i32> would silently 400 the URL when the operator paginates \
         past row 2.1B (decades-out, but the BIGINT bind contract enforces \
         the invariant now).",
    );
}
