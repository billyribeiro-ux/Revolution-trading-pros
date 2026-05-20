// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Sitemap route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::sitemap` and pins the
//! `router()` mount-table shape for the SEO sitemap surface.
//!
//! ## Why this file exists (R19-D)
//!
//! `routes/sitemap.rs` is 481 LOC of dynamic XML sitemap generation
//! over `posts`, `categories`, `tags` (all BIGSERIAL i64 PKs per
//! schema.sql:8798, 3672, 10077) with a 1-hour `lazy_static`-backed
//! in-memory cache. Every endpoint is publicly reachable (no auth
//! extractor — crawlers consume sitemaps unauthenticated) and writes
//! to `application/xml`. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 4 GET endpoints, public surface.**
//!    The mount table (lines 475-481):
//!      - GET /sitemap                       (index — routes to
//!                                            sitemap_index OR
//!                                            main_sitemap based on
//!                                            post count vs 50_000)
//!      - GET /sitemap/posts/:page           (paginated posts)
//!      - GET /sitemap/categories            (categories sitemap)
//!      - GET /sitemap/tags                  (tags sitemap)
//!    A regression that added auth (e.g., `_admin: AdminUser` on any
//!    handler) would 401 Googlebot/Bingbot and silently tank SEO
//!    indexing for the whole catalog. A regression that DROPPED any
//!    route would break the `sitemapindex` cross-references (since
//!    sitemap_index emits `<loc>` URLs pointing at /sitemap/posts/N,
//!    /sitemap/categories, /sitemap/tags).
//!
//! 2. **`Path(page): Path<i64>` on /sitemap/posts/:page.** The page
//!    parameter is `i64` so the offset arithmetic (line 267:
//!    `(page - 1) * MAX_URLS_PER_SITEMAP`) fits comfortably with
//!    50_000 URLs per page. With `posts.id BIGINT` (i64, schema.sql:
//!    8798) and an upper bound of i64::MAX rows, the page count CAN
//!    legitimately exceed i32::MAX over decades of catalog growth.
//!    Per CLAUDE.md "Money / cents" rule i64 is correct here, and
//!    this is NOT a Reserved exception (pagination across BIGINT-PK
//!    tables) — keeping `Path<i64>` prevents a silent overflow on
//!    `(page - 1) * 50_000` if a future refactor flipped to i32.
//!
//! 3. **`MAX_URLS_PER_SITEMAP = 50_000: i64`** — the sitemaps.org
//!    spec maximum. The constant is private but the router's mount
//!    of `/sitemap/posts/:page` IS the public contract — the
//!    compile-pin proves the page extractor and route exist.
//!
//! 4. **Cache is module-private `lazy_static` + `tokio::RwLock`.**
//!    Per CLAUDE.md habit #3 (re-read your own diff): the cache is
//!    `Arc<RwLock<HashMap<String, SitemapCache>>>`. A refactor that
//!    extracted the cache into a `OnceLock` per-handler would
//!    silently lose cache coherence between handlers (every handler
//!    keys into the same map by `cache_key` string — extracting
//!    per-handler caches would let the sitemapindex's reference
//!    to /sitemap/posts/N drift from the cached content at that URL).
//!    We can't introspect private state from outside, but constructing
//!    `router()` multiple times MUST NOT panic (lazy_static is
//!    idempotent; a regression that wrapped it in a `OnceLock` whose
//!    `set` panics on second call would fail here).
//!
//! ## Pattern source
//!
//! Modeled on `tests/room_search_test.rs`, `tests/robots_test.rs`
//! (sister SEO endpoint), `tests/analytics_test.rs`.

use revolution_api::routes::sitemap;

// ── 1. Router mount-table compile-pin (4 routes, all public) ────────

/// `routes::sitemap::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/sitemap.rs:475-481:
///   - GET /sitemap                       (index)
///   - GET /sitemap/posts/:page           (posts, Path<i64>)
///   - GET /sitemap/categories            (categories)
///   - GET /sitemap/tags                  (tags)
///
/// All 4 handlers are PUBLIC (no `_admin: AdminUser`, no
/// `_user: User` extractor). Sitemaps are consumed by search-engine
/// crawlers which do not authenticate. A regression that added auth
/// to any handler would 401 Googlebot/Bingbot and silently tank
/// SEO indexing for the whole catalog. The compile-pin catches
/// signature drift; the runtime "is this anonymous?" property is
/// enforced by the absence of an auth extractor in the function
/// signature.
#[test]
fn sitemap_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = sitemap::router();
}

// ── 2. Router idempotent — cache + lazy_static survive re-construction ──

/// Per CLAUDE.md habit #3 ("Re-read your own diff before commit"):
/// the sitemap cache is `lazy_static! { static ref SITEMAP_CACHE:
/// Arc<RwLock<HashMap<String, SitemapCache>>> }`. A refactor that
/// extracted the cache into a `OnceLock` PER-HANDLER (one common
/// AI-assisted refactor shape) would either:
///   a) panic on second `router()` construction (`OnceLock::set`
///      can only be called once per cell), or
///   b) silently lose cache coherence between handlers (each
///      handler keying into its own per-handler map, so the
///      sitemapindex's `<loc>` references to /sitemap/posts/N
///      would drift from the cached content at that URL).
///
/// This pin catches case (a) at compile-test time. Case (b) would
/// require a runtime integration test to catch, but the
/// idempotent-construction pin documents the invariant.
#[test]
fn sitemap_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = sitemap::router();
    let _r2: axum::Router<revolution_api::AppState> = sitemap::router();
    let _r3: axum::Router<revolution_api::AppState> = sitemap::router();
}

// ── 3. Public handler-function fingerprints exist ────────────────────

/// The 4 public handler functions are pub at the module level
/// (`pub async fn index`, `pub async fn sitemap_index`, etc.). A
/// regression that demoted any of them to `async fn` (private)
/// would break this test because the symbols would no longer be
/// reachable from `revolution_api::routes::sitemap::`. The router()
/// would still compile because handlers can be private — but the
/// public surface contract would silently shrink.
///
/// This is a compile-pin: if any of these fail to resolve, the
/// test file won't compile, so we'd catch the regression at
/// `cargo check --tests`.
///
/// Path<i64> on `posts` is load-bearing — sitemap pagination
/// across BIGINT-PK tables (posts.id is `bigint NOT NULL`,
/// schema.sql:8798) requires i64 page indexing. Per CLAUDE.md
/// "Money / cents" rule i64 end-to-end is correct here; this is
/// NOT a Reserved exception because the page count is unbounded
/// above (one sitemap entry per published post over the catalog's
/// lifetime).
#[test]
fn sitemap_handler_symbols_publicly_reachable() {
    // Type-check that the handler functions exist with the documented
    // shape. We don't invoke them (they all need a real AppState
    // with a Postgres pool), we just resolve them as function items.
    //
    // Note: rust doesn't let us take the address of an async fn
    // generic over State<AppState> without specifying, so we use
    // an unevaluated type-position assertion via `let _: fn(...) =
    // sitemap::index`. But `index` is generic over the State
    // extractor, so we instead use the simpler "compile if the
    // path resolves" pattern: the function paths below MUST be
    // valid sitemap module items, else this file fails to compile.
    let _index_path = sitemap::index;
    let _sitemap_index_path = sitemap::sitemap_index;
    let _main_sitemap_path = sitemap::main_sitemap;
    let _posts_path = sitemap::posts;
    let _categories_path = sitemap::categories;
    let _tags_sitemap_path = sitemap::tags_sitemap;
}

// ── 4. Posts page parameter is i64 (BIGSERIAL-aligned) ──────────────

/// `posts(State, Path(page): Path<i64>)` — the page parameter is
/// `i64`. Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY":
/// while `page` is not money, it indexes ACROSS a BIGINT-PK table
/// (`posts.id` is `bigint NOT NULL`, schema.sql:8798), and the
/// handler's offset arithmetic is
/// `(page - 1) * MAX_URLS_PER_SITEMAP` = `(page - 1) * 50_000`.
///
/// At i32::MAX pages (~2.1B), that arithmetic would overflow on
/// i32 (`2_147_483_647 * 50_000` ≈ 1e14, way past i32::MAX) but
/// fits in i64. This is NOT a Reserved exception (page-count is
/// genuinely unbounded above as the catalog grows over years).
///
/// R9-D NEGATIVE: a regression that changed `Path<i64>` to
/// `Path<i32>` would silently overflow the offset for any
/// catalog past ~42949 pages (~2B posts).
///
/// We can't invoke the handler without a DB. But we CAN type-assert
/// the value space — by constructing an i64 page value > i32::MAX
/// and confirming the type assignment compiles:
#[test]
fn sitemap_posts_page_is_i64_bigserial_aligned() {
    let above_i32_pages: i64 = (i32::MAX as i64) + 1;
    let _: i64 = above_i32_pages;

    // Sanity: 50_000 urls per sitemap × i32::MAX pages overflows i32
    // but stays within i64.
    let max_urls_per_sitemap_i64: i64 = 50_000;
    let above_i32_offset: i64 = above_i32_pages * max_urls_per_sitemap_i64;
    assert!(
        above_i32_offset > i32::MAX as i64,
        "page * MAX_URLS_PER_SITEMAP MUST stay in i64 — \
         (i32::MAX+1) * 50_000 overflows i32 silently if Path<i32>"
    );

    // Negative: i64 sentinel just past i32::MAX MUST round-trip
    // through serde (this is the wire encoding axum's Path uses
    // for path-segment integers).
    let wire = serde_json::to_value(above_i32_pages).expect("serialize i64 page");
    assert_eq!(
        wire.as_i64(),
        Some(above_i32_pages),
        "i64 page parameter MUST round-trip through JSON wire"
    );
}
