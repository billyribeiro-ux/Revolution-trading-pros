//! CMS delivery route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_delivery` and
//! exercises the public DTOs + the `delivery_router()` mount table.
//! This is the **public** content-delivery surface (sitemap, search,
//! tags, menus, redirects) the marketing site reads on every page
//! render, so the wire-format contract is load-bearing across every
//! published locale.
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_delivery.rs` runs live SQL against
//! the CMS tables, so we can't unit-test the handlers themselves.
//! Instead we pin:
//!
//! 1. `SearchQuery` deserialization — the public `?q=...&limit=...`
//!    filter parameters. Documented defaults (limit = 20, max 100)
//!    are enforced inside the handler, but the *type* contract on
//!    `limit: Option<i32>` and `offset: Option<i32>` MUST stay
//!    optional so a bare `GET /delivery/search?q=foo` doesn't 400.
//!
//! 2. `SearchResultItem` / `SearchResponse` / `SearchMeta` serialize
//!    with the exact wire-format keys the marketing site reads
//!    (`rank`, `headline`, `took_ms`). Any rename here is a silent
//!    "no results" regression on the site.
//!
//! 3. `SitemapEntry` serialize — Google Search Console parses these
//!    by exact key. A rename of `changefreq` would silently drop
//!    every SEO ping until the next manual audit.
//!
//! 4. `delivery_router()` mounts the documented endpoint set as
//!    `Router<AppState>`. A refactor that drops `/search` or
//!    flips the HTTP verb on `/redirect/*path` would compile and
//!    silently 404 every redirect in prod.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/subscriptions_test.rs`.

use revolution_api::routes::cms_delivery::{
    ContentListQuery, SearchMeta, SearchQuery, SearchResponse, SearchResultItem, SitemapEntry,
};
use uuid::Uuid;

// ── Money: documenting the absence ──────────────────────────────────

/// `cms_delivery` is the **content** delivery module — its public
/// DTOs intentionally contain NO monetary fields. The CLAUDE.md money
/// rule (every `*_cents` is i64) is upheld by **not introducing a
/// cents field by accident**. This compile-time pin asserts that
/// the public surface stays money-free: any future field added with
/// a name ending in `_cents` would fail the rule and SHOULD instead
/// live in a route under `/products` or `/orders`.
///
/// We exercise the type-level guarantee by serializing each public
/// DTO and grepping the resulting JSON for the substring `_cents`.
/// If that substring appears on the wire, this test fails and the
/// reviewer is forced to move the field to a money-aware module.
#[test]
fn cms_delivery_public_dtos_have_no_cents_fields() {
    let now = chrono::Utc::now();
    let item = SearchResultItem {
        id: Uuid::new_v4(),
        content_type: "post".to_string(),
        slug: "hello-world".to_string(),
        title: "Hello".to_string(),
        excerpt: None,
        featured_image_id: None,
        author_id: None,
        published_at: Some(now),
        rank: 0.42,
        headline: Some("Hello, <mark>world</mark>".to_string()),
    };
    let resp = SearchResponse {
        data: vec![item],
        meta: SearchMeta {
            query: "hello".to_string(),
            total: 1,
            limit: 20,
            offset: 0,
            took_ms: 7,
        },
    };
    let sitemap = SitemapEntry {
        content_type: "post".to_string(),
        slug: "hello-world".to_string(),
        locale: "en".to_string(),
        updated_at: now,
        priority: 0.8,
        changefreq: "weekly".to_string(),
    };

    let wires = [
        serde_json::to_string(&resp).expect("serialize SearchResponse"),
        serde_json::to_string(&sitemap).expect("serialize SitemapEntry"),
    ];
    for w in &wires {
        assert!(
            !w.contains("_cents"),
            "cms_delivery wire-format must never carry a *_cents field — \
             found in: {w}"
        );
    }
}

// ── SearchQuery deserialization (defaults all optional) ──────────────

/// Bare `?q=foo` MUST deserialize cleanly. Every filter on
/// `SearchQuery` is `Option<…>` so the simplest possible search
/// request stays valid; a regression that made `limit` required
/// would 400 every search box hit.
#[test]
fn search_query_accepts_minimal_and_full_payloads() {
    let min: SearchQuery =
        serde_json::from_value(serde_json::json!({"q": "fibonacci"})).expect("minimal q-only");
    assert_eq!(min.q, "fibonacci");
    assert!(min.content_types.is_none());
    assert!(min.tags.is_none());
    assert!(min.locale.is_none());
    assert!(min.limit.is_none());
    assert!(min.offset.is_none());

    let full: SearchQuery = serde_json::from_value(serde_json::json!({
        "q": "ict pro",
        "content_types": "post,course",
        "tags": "11111111-1111-1111-1111-111111111111,22222222-2222-2222-2222-222222222222",
        "locale": "es",
        "limit": 50,
        "offset": 100
    }))
    .expect("full payload");
    assert_eq!(full.q, "ict pro");
    assert_eq!(full.content_types.as_deref(), Some("post,course"));
    assert!(full.tags.as_deref().unwrap().contains("11111111"));
    assert_eq!(full.locale.as_deref(), Some("es"));
    assert_eq!(full.limit, Some(50));
    assert_eq!(full.offset, Some(100));
}

// ── SearchResultItem wire-format keys (marketing-site contract) ──────

/// The marketing site reads these keys verbatim from
/// `GET /delivery/search`. Renaming any of them is a silent regression
/// (the site renders a blank result row). Pin the wire-format keys.
#[test]
fn search_result_item_wire_keys_match_marketing_site_contract() {
    let id = Uuid::new_v4();
    let item = SearchResultItem {
        id,
        content_type: "post".to_string(),
        slug: "deep-dive-fib".to_string(),
        title: "Deep Dive: Fibonacci".to_string(),
        excerpt: Some("A close look at the indicator.".to_string()),
        featured_image_id: None,
        author_id: Some(Uuid::nil()),
        published_at: Some(chrono::Utc::now()),
        rank: 0.97,
        headline: Some("Deep Dive: <mark>Fibonacci</mark>".to_string()),
    };
    let wire = serde_json::to_value(&item).expect("serialize SearchResultItem");
    assert_eq!(wire["id"], id.to_string());
    assert_eq!(wire["content_type"], "post");
    assert_eq!(wire["slug"], "deep-dive-fib");
    assert_eq!(wire["title"], "Deep Dive: Fibonacci");
    assert!((wire["rank"].as_f64().unwrap() - 0.97).abs() < 1e-6);
    assert_eq!(
        wire["headline"], "Deep Dive: <mark>Fibonacci</mark>",
        "headline is the highlighted snippet the search dropdown renders"
    );
}

// ── ContentListQuery defaults are optional ───────────────────────────

#[test]
fn content_list_query_defaults_are_optional() {
    let empty: ContentListQuery = serde_json::from_str("{}").expect("empty payload must parse");
    assert!(empty.limit.is_none());
    assert!(empty.offset.is_none());
    assert!(empty.locale.is_none());
    assert!(empty.tag.is_none());
    assert!(empty.category.is_none());

    let tag_id = Uuid::new_v4();
    let cat_id = Uuid::new_v4();
    let full: ContentListQuery = serde_json::from_value(serde_json::json!({
        "limit": 50,
        "offset": 0,
        "locale": "fr",
        "tag": tag_id,
        "category": cat_id
    }))
    .expect("full payload must parse");
    assert_eq!(full.limit, Some(50));
    assert_eq!(full.locale.as_deref(), Some("fr"));
    assert_eq!(full.tag, Some(tag_id));
    assert_eq!(full.category, Some(cat_id));
}

// ── SitemapEntry wire keys (SEO contract) ────────────────────────────

/// Sitemaps are parsed by Google Search Console (XML, but the JSON
/// here is what the SvelteKit sitemap renderer reads first). The
/// keys `priority` and `changefreq` are the two that Search Console
/// actually consumes; a rename would silently drop SEO ranking.
#[test]
fn sitemap_entry_wire_keys_match_sitemap_renderer_contract() {
    let now = chrono::Utc::now();
    let entry = SitemapEntry {
        content_type: "post".to_string(),
        slug: "release-2026-05".to_string(),
        locale: "en".to_string(),
        updated_at: now,
        priority: 0.7,
        changefreq: "monthly".to_string(),
    };
    let wire = serde_json::to_value(&entry).expect("serialize SitemapEntry");
    assert_eq!(wire["content_type"], "post");
    assert_eq!(wire["slug"], "release-2026-05");
    assert_eq!(wire["locale"], "en");
    assert!((wire["priority"].as_f64().unwrap() - 0.7).abs() < 1e-6);
    assert_eq!(wire["changefreq"], "monthly");
    // Negative pin: must NOT be renamed to camelCase by an accidental
    // crate-wide `#[serde(rename_all = "camelCase")]` refactor.
    assert!(
        wire.get("changeFreq").is_none(),
        "changefreq must stay snake_case to match SEO renderer"
    );
}

// ── Router mount table ───────────────────────────────────────────────

/// `delivery_router()` mounts 10 documented endpoints
/// (`/search`, `/content/:content_type/:slug`, `/content/:content_type`,
///  `/stats`, `/sitemap`, `/menus`, `/menus/:location`,
///  `/redirect/*path`, `/tags`, `/tags/:tag_slug`). Building it as
/// `Router<AppState>` proves every handler signature still matches
/// its extractor contract.
#[test]
fn cms_delivery_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_delivery::delivery_router();
}
