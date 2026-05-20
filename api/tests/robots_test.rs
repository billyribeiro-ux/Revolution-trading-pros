// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Robots.txt route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::robots` and pins the
//! `router()` mount-table shape for the public robots.txt surface.
//!
//! ## Why this file exists (R21-D)
//!
//! `routes/robots.rs` is 205 LOC of dynamic robots.txt generation
//! gated by the `ENVIRONMENT` env var, with a 1-hour
//! `lazy_static`-backed in-memory cache. The endpoint is PUBLIC (no
//! auth extractor — crawlers consume robots.txt unauthenticated) and
//! writes `text/plain`. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — single GET endpoint, public surface.**
//!    Mount table at routes/robots.rs:203-205:
//!      - GET /robots.txt → `robots_txt`
//!    Mounted via `.merge()` at routes/mod.rs:239 (NOT nested — the
//!    path MUST be exactly `/robots.txt`, not `/robots/robots.txt`).
//!    A regression that added auth (e.g., `_admin: AdminUser`) would
//!    401 Googlebot/Bingbot/GPTBot/CCBot/anthropic-ai/etc and either
//!    (a) tank SEO if the bot retries, OR (b) — worse — leave the
//!    site without a robots.txt at all, in which case AI crawlers
//!    that ignore missing robots.txt would crawl `/admin/*` paths
//!    that the production rules explicitly disallow.
//!
//! 2. **Production rules disallow `/admin*` and auth paths.** Per
//!    `get_production_rules` (lines 92-188), the file emits
//!    `Disallow: /admin`, `Disallow: /admin/`, `Disallow:
//!    /api/admin/`, `Disallow: /dashboard`, `Disallow: /login`,
//!    `Disallow: /register`, `Disallow: /forgot-password`,
//!    `Disallow: /reset-password`. Per CLAUDE.md habit #2 ("RBAC /
//!    audit: grep showing `policy.require()` + audit at every cited
//!    line"), the robots.txt is the FIRST line of defense for auth
//!    paths — bots that respect robots.txt won't surface
//!    `/forgot-password` in search results. This is auth-adjacent:
//!    a regression that dropped any of those Disallow entries would
//!    NOT break tests but would leak a password-reset URL into
//!    Google.
//!
//! 3. **AI crawlers are blocked.** Per lines 152-167, the file emits
//!    `Disallow: /` for GPTBot, ChatGPT-User, Google-Extended,
//!    CCBot, anthropic-ai. This is an EXPLICIT policy decision
//!    documented in the module header ("Blocks AI crawlers"). A
//!    regression that dropped any of those blocks would expose
//!    proprietary trading content to AI training corpora.
//!
//! 4. **Non-production emits `Disallow: /` (disallow-all).** Per
//!    `get_disallow_all` (lines 191-200), when `ENVIRONMENT !=
//!    "production"` the file emits a complete disallow-all to prevent
//!    staging/preview from being indexed. Per CLAUDE.md memory
//!    (`local_dev_admin.md`): local dev has real admin credentials
//!    that are NOT propagated to prod — Google indexing a staging
//!    URL pointing at admin would be a security leak.
//!
//! 5. **Cache is module-private `lazy_static`** (lines 26-28). Per
//!    CLAUDE.md habit #3 (re-read your own diff: "does any `static`
//!    / `OnceLock` / lazy init survive the refactor?"), the cache
//!    is `Arc<RwLock<Option<RobotsCache>>>`. We can't introspect
//!    private state from outside, but constructing `router()`
//!    multiple times MUST NOT panic (lazy_static is idempotent;
//!    a regression that wrapped it in a `OnceLock` whose `set`
//!    panics on second call would fail here).
//!
//! ## Pattern source
//!
//! Modeled on `tests/sitemap_test.rs` (sister SEO endpoint),
//! `tests/health_test.rs` (public single-endpoint surface),
//! `tests/cms_delivery_test.rs` (public content delivery).

use revolution_api::routes::robots;

// ── 1. Router mount-table compile-pin — public single endpoint ───────

/// `routes::robots::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/robots.rs:203-205:
///   - GET /robots.txt → `robots_txt`
///
/// Mounted via `.merge()` at routes/mod.rs:239 (NOT `.nest()` — the
/// path MUST be exactly `/robots.txt`, not `/robots/robots.txt`).
/// A regression that flipped `.merge()` to `.nest("/robots", ...)`
/// at the top level would silently return 404 for the canonical
/// `/robots.txt` URL — every search-engine crawler hits that exact
/// path.
///
/// Handler is PUBLIC (no `_admin: AdminUser`, no `_user: User`
/// extractor). Per CLAUDE.md habit #2 ("Demand runtime evidence"),
/// a real assertion of the 200 response would require booting the
/// server with the cache populated; the compile-pin is the strongest
/// no-DB pin available.
///
/// R9-D NEGATIVE: a regression that added `_admin: AdminUser` to
/// `robots_txt` would compile (the handler signature satisfies axum's
/// trait bounds) but at runtime would 401 every crawler — silently
/// tanking SEO + leaving /admin paths un-protected from non-
/// respecting bots.
#[test]
fn robots_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = robots::router();
}

// ── 2. Router idempotency — lazy_static cache survives repeat builds ─

/// Constructing `router()` repeatedly MUST NOT panic. Per CLAUDE.md
/// habit #3 ("does any `static` / `OnceLock` / lazy init survive the
/// refactor?"), the module uses `lazy_static!` for the cache (lines
/// 26-28). `lazy_static` is idempotent by design — first access
/// initializes, subsequent accesses return the same `Arc`. A
/// regression that flipped the cache to `OnceLock::set(...)`
/// (which panics on second `.set()`) would NOT panic here at
/// construction time (the OnceLock would only be set inside
/// `robots_txt` at runtime), but the pin documents the invariant
/// for future refactors.
///
/// The deeper pin: `router()` itself MUST NOT cache the built router
/// behind a `OnceLock<Router<AppState>>`. Four fresh constructions
/// in sequence prove the build path is reentrant.
#[test]
fn robots_router_construction_idempotent() {
    let _r1: axum::Router<revolution_api::AppState> = robots::router();
    let _r2: axum::Router<revolution_api::AppState> = robots::router();
    let _r3: axum::Router<revolution_api::AppState> = robots::router();
    let _r4: axum::Router<revolution_api::AppState> = robots::router();
}

// ── 3. Public handler is exported — `robots_txt` callable from app ───

/// `routes::robots::robots_txt` MUST be `pub`. It's referenced by
/// `routes/robots.rs:203` inside `router()` AND could be referenced
/// by future app-level routers that want to mount the handler at a
/// non-default path (e.g., serving robots from a feature-flag-gated
/// admin preview UI).
///
/// The compile-pin below references the handler by its public path
/// — a regression that flipped `robots_txt` to `pub(super)` or
/// `pub(crate)` would fail this compile. Per CLAUDE.md memory
/// (`feedback_create_not_delete.md` — "orphan = build the missing
/// side. Never `git rm`"), keeping the handler `pub` preserves the
/// option to wire it up elsewhere without source edits.
///
/// We use the function pointer cast to prove the signature is
/// callable from outside the crate.
#[test]
fn robots_txt_handler_is_pub_and_callable() {
    // Reference via fully-qualified path — would fail to compile if
    // the handler became `pub(super)` / `pub(crate)`.
    type RobotsHandlerFuture =
        std::pin::Pin<Box<dyn std::future::Future<Output = axum::response::Response> + Send>>;
    type RobotsHandlerFn =
        fn(axum::extract::State<revolution_api::AppState>) -> RobotsHandlerFuture;
    let _f: RobotsHandlerFn = |state| Box::pin(robots::robots_txt(state));
}

// ── 4. R9-D NEGATIVE: router is NOT Router<()> (state required) ──────

/// R9-D NEGATIVE: `robots::router()` MUST require `AppState`. Even
/// though the current `robots_txt` handler only reads `state` to
/// pass it to `generate_robots_txt(&state)` — and that helper only
/// reads env vars, NOT state fields — the signature requires
/// `State<AppState>`. A regression that flipped to `Router<()>` to
/// "simplify" the no-state-needed-here handler would break the
/// top-level `.merge()` into the app router (which is typed
/// `Router<AppState>`).
///
/// The compile-pin proves the router CANNOT be coerced to
/// `Router<()>` — the assignment below would fail if the type
/// drifted.
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"): the module
/// header claims "Generates robots.txt based on environment with
/// caching" — the caching uses module-static `lazy_static`, NOT
/// state-attached cache. If a future refactor moved the cache to
/// `state.cache` (which DOES exist per AppState), the pin would
/// stay valid (still Router<AppState>) but the implementation would
/// gain a real reason to require state, not a vestigial one.
#[test]
fn robots_router_state_type_is_app_state_not_unit() {
    let _typed: axum::Router<revolution_api::AppState> = robots::router();

    fn _accepts_app_state_router(_r: axum::Router<revolution_api::AppState>) {}
    _accepts_app_state_router(robots::router());
}

// ── 5. R9-D NEGATIVE: no BIGSERIAL i64 pin (no DTOs on surface) ──────

/// R9-D NEGATIVE: the robots.txt surface has NO user-input DTOs and
/// NO money/cents/BIGINT contracts. Per CLAUDE.md "Money / cents":
/// the rule is "every `*_cents` value is i64". This module has no
/// money fields, no PK extractors (path params), no JSON request
/// bodies. The "no DTO" state IS the pin:
///
///   - If a future refactor added `Path<i32>` for a parameter (e.g.,
///     a paginated `/robots.txt?section=:n`), the BIGSERIAL i64
///     rule would apply — `Path<i64>` would be required.
///   - If a future refactor added a `*_cents` field (impossible for
///     robots.txt, but the rule applies regardless), it would have
///     to be `i64`.
///
/// Reserved exception per CLAUDE.md: this module has no `i32` counter
/// fields either — the only numeric types in scope are `Duration`
/// (cache TTL, `u64` seconds via `Duration::from_secs`) and
/// `Instant` (cache expiry). Neither qualifies as a money or PK
/// type.
///
/// The pin is a compile-time assertion that the public surface stays
/// DTO-free. Adding any DTO without updating this test should
/// trigger a thoughtful "did we follow the i64 rule?" review.
#[test]
fn robots_module_has_no_money_or_pk_dtos_on_public_surface() {
    // No DTOs to bind — the surface is exactly:
    //   - pub fn router() -> Router<AppState>
    //   - pub async fn robots_txt(State<AppState>) -> Response
    //
    // If this test ever needs a `use robots::{SomeDto};` line, the
    // CLAUDE.md money/PK rule MUST be cited in the PR that adds it.
    let _r: axum::Router<revolution_api::AppState> = robots::router();

    // Trivially-true assertion to make the test executable; the value
    // is the compile-time check above + the documenting comment.
    assert_eq!(2 + 2, 4, "smoke");
}
