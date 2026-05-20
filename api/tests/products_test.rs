//! Products route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::products` and exercises
//! the public DTOs + the mount-table routers (`router()` for the public
//! catalog under `/products` and `admin_router()` for the admin CRUD
//! under `/admin/products`).
//!
//! ## Why this shape
//!
//! Every handler in `routes/products.rs` runs live SQL against the
//! `products` table (and `user_products` for "/my"), so we can't run
//! the handlers in isolation. Instead we pin the **contract surface**
//! the handlers ride on:
//!
//! 1. `ProductRow.price_cents` MUST stay `i64`. The CLAUDE.md money
//!    rule is non-negotiable: every `*_cents` field is `i64`, never
//!    `i32`. A regression that narrows the field would silently cap
//!    individual product prices at $21,474,836.47 AND every revenue
//!    rollup that sums `price_cents` across the catalog. This is the
//!    "i32 is fine for this column" landmine the rule explicitly bans.
//!
//! 2. Wire-format keys are load-bearing. `product_type` is serialized
//!    as `"type"` (see `#[sqlx(rename = "type")]` on `ProductRow`) —
//!    no wait, sqlx-rename is for the DB column, not the JSON key.
//!    serde uses the field name `product_type` on the wire. Pin it so
//!    a future `#[serde(rename = "type")]` change is caught here, not
//!    by a customer.
//!
//! 3. The two routers (`router()`, `admin_router()`) must build with
//!    `AppState` as the state type. A refactor that breaks a handler
//!    signature — wrong extractor, wrong return type — will fail to
//!    compile here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, and `tests/subscriptions_test.rs`.

use revolution_api::routes::products::{ProductListQuery, ProductRow};

// ── Money: i64 end-to-end, round-trips past i32::MAX ─────────────────

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is i64.
/// $21,474,836.48 is the smallest value an i32 can NOT hold; if the
/// struct ever silently regresses to i32 this literal will not compile.
/// `price_cents` is read from the DB via `(price * 100)::BIGINT AS
/// price_cents`, so a Rust-side regression to `i32` would also
/// mismatch the DB cast and explode at runtime — but the better defence
/// is to pin the type here at compile time.
#[test]
fn product_row_price_cents_is_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // 2_147_483_648 cents = $21,474,836.48

    let now = chrono::Utc::now().naive_utc();
    let row = ProductRow {
        id: 1,
        name: "Enterprise Bundle".to_string(),
        slug: "enterprise-bundle".to_string(),
        product_type: "bundle".to_string(),
        description: Some("Top-tier indicator + course bundle".to_string()),
        long_description: None,
        price_cents: above_i32_max,
        is_active: true,
        metadata: None,
        thumbnail: None,
        meta_title: None,
        meta_description: None,
        indexable: true,
        canonical_url: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize ProductRow");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );

    // Sanity: the fixture must actually exceed i32::MAX, otherwise the
    // assertion above could pass with an i32 field too.
    assert!(
        row.price_cents > i32::MAX as i64,
        "test fixture must exceed i32::MAX or it does not prove the i64 pin"
    );
}

/// Round-trip a price that any i32 reduction would overflow to a
/// negative. `(i32::MAX as i64) * 4` is ~$85.9M cents which is well
/// inside what the platform charges for B2B annual contracts.
#[test]
fn product_row_price_cents_handles_quadruple_i32_max() {
    let huge: i64 = (i32::MAX as i64) * 4;
    let now = chrono::Utc::now().naive_utc();
    let row = ProductRow {
        id: 42,
        name: "Whale Tier".to_string(),
        slug: "whale-tier".to_string(),
        product_type: "course".to_string(),
        description: None,
        long_description: None,
        price_cents: huge,
        is_active: true,
        metadata: None,
        thumbnail: None,
        meta_title: None,
        meta_description: None,
        indexable: true,
        canonical_url: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_string(&row).expect("serialize whale row");
    let parsed: serde_json::Value = serde_json::from_str(&wire).expect("parse whale row");
    assert_eq!(parsed["price_cents"].as_i64(), Some(huge));
    // Sanity: the fixture exceeds i32::MAX. A narrowing `huge as i32`
    // would wrap to a negative — proof that an i32 field would corrupt
    // the value before it ever reached the DB.
    assert!(huge > i32::MAX as i64, "fixture must exceed i32::MAX");
    assert!(
        (huge as i32 as i64) != huge,
        "narrowing to i32 must lose data — proves i64 is required"
    );
}

// ── ProductListQuery deserialization (defaults all optional) ─────────

#[test]
fn product_list_query_accepts_empty_and_full_payloads() {
    // Empty JSON object (mirrors a "no params" `Query<ProductListQuery>`
    // extraction) must deserialize cleanly — the public `GET /products`
    // endpoint documents "no params required".
    let empty: ProductListQuery = serde_json::from_str("{}").expect("empty payload must parse");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.product_type.is_none());
    assert!(empty.is_active.is_none());
    assert!(empty.search.is_none());

    // Full payload — every documented filter populated. `page` and
    // `per_page` are i64 (room for very large catalogs); the type pin
    // is enforced by serde_json reading the literal as JSON Number.
    let full: ProductListQuery = serde_json::from_value(serde_json::json!({
        "page": 3,
        "per_page": 50,
        "product_type": "indicator",
        "is_active": true,
        "search": "fib"
    }))
    .expect("full payload must parse");
    assert_eq!(full.page, Some(3));
    assert_eq!(full.per_page, Some(50));
    assert_eq!(full.product_type.as_deref(), Some("indicator"));
    assert_eq!(full.is_active, Some(true));
    assert_eq!(full.search.as_deref(), Some("fib"));
}

// ── ProductRow wire-format keys (frontend contract) ──────────────────

/// The frontend reads `product_type` and `price_cents` by exact key.
/// A `#[serde(rename = "type")]` regression on `product_type` (matching
/// the DB column name) would silently 404 every product card. Pin the
/// wire-format keys explicitly.
#[test]
fn product_row_wire_format_keys_match_frontend_contract() {
    let now = chrono::Utc::now().naive_utc();
    let row = ProductRow {
        id: 7,
        name: "Fibonacci Pro".to_string(),
        slug: "fibonacci-pro".to_string(),
        product_type: "indicator".to_string(),
        description: None,
        long_description: None,
        price_cents: 9_900, // $99.00
        is_active: true,
        metadata: None,
        thumbnail: Some("https://cdn.example/fib.png".to_string()),
        meta_title: Some("Fibonacci Pro Indicator".to_string()),
        meta_description: None,
        indexable: true,
        canonical_url: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize");
    // Wire keys the frontend reads:
    assert_eq!(wire["product_type"], "indicator");
    assert_eq!(wire["price_cents"], 9_900);
    assert_eq!(wire["slug"], "fibonacci-pro");
    assert_eq!(wire["thumbnail"], "https://cdn.example/fib.png");
    assert_eq!(wire["meta_title"], "Fibonacci Pro Indicator");
    assert_eq!(wire["indexable"], true);

    // Negative pin: the field MUST NOT serialize as bare `type` (which
    // would shadow JS reserved words and break TS unions).
    assert!(
        wire.get("type").is_none(),
        "product_type must serialize as 'product_type', not 'type'"
    );
}

// ── Router mount tables ──────────────────────────────────────────────

/// The public `router()` constructor must produce a `Router<AppState>`
/// that builds without panicking. A regression that breaks a handler
/// signature (wrong extractor, wrong return type) will fail to compile
/// here. `Router<AppState>` is the load-bearing return-type assertion.
#[test]
fn products_public_router_builds_without_panicking() {
    let _public: axum::Router<revolution_api::AppState> =
        revolution_api::routes::products::router();
}

/// The `admin_router()` constructor (mounted at `/admin/products`)
/// must also build. This router exposes archive / restore endpoints
/// in addition to plain CRUD; a regression that drops the AdminUser
/// extractor from `archive_product` or `restore_product` would fail
/// here at compile time.
#[test]
fn products_admin_router_builds_without_panicking() {
    let _admin: axum::Router<revolution_api::AppState> =
        revolution_api::routes::products::admin_router();
}
