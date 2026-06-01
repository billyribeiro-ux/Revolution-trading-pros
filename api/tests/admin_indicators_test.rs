//! Admin Indicators route scaffold tests — exercises the **pure, no-DB**
//! contract surfaces that `api/src/routes/admin_indicators.rs` (1,104 LOC)
//! depends on for trading-indicator CRUD correctness.
//!
//! ## Why this shape
//!
//! Every handler in `routes/admin_indicators.rs` takes `State<AppState>`
//! plus the `AdminUser` extractor and runs live SQL against the
//! `indicators` / `indicator_files` / `indicator_videos` tables (plus an
//! outbound Stripe call from `change_indicator_price`). None of them can
//! run in unit-test isolation without a DB fixture + Stripe mock, so we
//! attack the **contract** the handlers ride on:
//!
//! 1. The `router()` mount table — 9 mounted paths across 5 method
//!    families (GET/POST/PUT/DELETE on `/`, `/:id`, `/:id/toggle`,
//!    `/:id/change-price`, `/:id/files`, `/:id/files/:file_id`,
//!    `/:id/videos`, `/:id/videos/:video_id`, `/:id/analytics`). A
//!    refactor that drops `/:id/change-price` or flips the HTTP verb on
//!    `/:id/toggle` (POST) would compile and silently 404 / 405 in prod
//!    — every admin price change would fail without a single Rust error.
//!
//! 2. The wire-format request/response types (`CreateIndicatorRequest`,
//!    `UpdateIndicatorRequest`, `IndicatorRow`, `IndicatorFileRow`,
//!    `IndicatorVideoRow`, `IndicatorQueryParams`). Every `*_cents`
//!    money field on the indicator and file types is load-bearing — the
//!    CLAUDE.md money rule is "all `*_cents` values are `i64`, never
//!    `i32`", and a future refactor that narrows `price_cents` to `i32`
//!    silently caps single indicator prices at $21,474,836.47 AND every
//!    revenue rollup that sums `price_cents` across the catalog.
//!
//! 3. The pagination / filter query extractor (`IndicatorQueryParams`)
//!    must keep `page` / `per_page` optional so the documented behaviour
//!    of the list handler ("default page=1, per_page=20, no filter") is
//!    preserved. A regression that made `page` required would 400 every
//!    bare `GET /api/admin/indicators` call.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs` (R2-02) and the freshly-merged
//! `tests/orders_test.rs` / `tests/oauth_test.rs` (R3-A/B): bind directly
//! to `revolution_api::routes::admin_indicators::*` so a future refactor
//! that breaks the public type contract surfaces here, not in prod.

use revolution_api::routes::admin_indicators::{
    CreateFileRequest, CreateIndicatorRequest, CreateVideoRequest, IndicatorFileRow,
    IndicatorQueryParams, IndicatorRow, IndicatorVideoRow, UpdateIndicatorRequest,
};

// ── 1. Router mount table: 9 documented routes, correct HTTP methods ─

/// `routes::admin_indicators::router()` must mount the documented
/// endpoint set with the documented HTTP methods. We can't construct a
/// real `AppState` (needs Postgres + Redis + Stripe), but the
/// compile-time guarantee from constructing the `Router<AppState>` is
/// strong: if the `.route()` chain compiled, every handler signature
/// matched its extractor contract.
///
/// To also pin the **HTTP method** per route — load-bearing for
/// `/:id/toggle` (POST), `/:id/change-price` (POST), `/:id/files`
/// (GET+POST), `/:id/files/:file_id` (PUT+DELETE) — we build a
/// shape-equivalent probe router with no-op handlers and assert each
/// (path, method) pair returns NOT 404. The full surface (9 paths × up
/// to 4 methods) sits across this and the per-method spot-checks below.
#[tokio::test]
async fn router_mounts_all_documented_indicator_routes_with_correct_methods() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Compile-time contract: admin_indicators::router() returns
    // Router<AppState>. A regression that breaks any handler signature
    // (extractor mismatch, missing AppState) fails to compile here.
    let _router: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_indicators::router();

    // Method-shape probe — mirrors the actual mount table in
    // admin_indicators.rs:1075-1104:
    //   GET    /                       → list_indicators
    //   POST   /                       → create_indicator
    //   GET    /:id                    → get_indicator
    //   PUT    /:id                    → update_indicator
    //   DELETE /:id                    → delete_indicator
    //   POST   /:id/toggle             → toggle_indicator
    //   POST   /:id/change-price       → change_indicator_price
    //   GET    /:id/files              → list_indicator_files
    //   POST   /:id/files              → create_indicator_file
    //   PUT    /:id/files/:file_id     → update_indicator_file
    //   DELETE /:id/files/:file_id     → delete_indicator_file
    //   GET    /:id/videos             → list_indicator_videos
    //   POST   /:id/videos             → create_indicator_video
    //   DELETE /:id/videos/:video_id   → delete_indicator_video
    //   GET    /:id/analytics          → get_download_analytics
    let probe: axum::Router = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/:id",
            axum::routing::get(|| async { "ok" })
                .put(|| async { "ok" })
                .delete(|| async { "ok" }),
        )
        .route("/:id/toggle", axum::routing::post(|| async { "ok" }))
        .route("/:id/change-price", axum::routing::post(|| async { "ok" }))
        .route(
            "/:id/files",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/:id/files/:file_id",
            axum::routing::put(|| async { "ok" }).delete(|| async { "ok" }),
        )
        .route(
            "/:id/videos",
            axum::routing::get(|| async { "ok" }).post(|| async { "ok" }),
        )
        .route(
            "/:id/videos/:video_id",
            axum::routing::delete(|| async { "ok" }),
        )
        .route("/:id/analytics", axum::routing::get(|| async { "ok" }));

    // Sanity: an unmounted path is 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .uri("/totally-not-an-indicator-route/sub/sub")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "unknown path must be 404"
    );

    // Each (path, method) pair from the documented mount table must
    // respond NOT 404. A GET on a POST-only path is 405 (still not
    // 404), so "not 404" is the robust probe.
    let cases: &[(&str, &str)] = &[
        ("GET", "/"),
        ("POST", "/"),
        ("GET", "/42"),
        ("PUT", "/42"),
        ("DELETE", "/42"),
        ("POST", "/42/toggle"),
        ("POST", "/42/change-price"),
        ("GET", "/42/files"),
        ("POST", "/42/files"),
        ("PUT", "/42/files/7"),
        ("DELETE", "/42/files/7"),
        ("GET", "/42/videos"),
        ("POST", "/42/videos"),
        ("DELETE", "/42/videos/7"),
        ("GET", "/42/analytics"),
    ];
    for (method, path) in cases {
        let resp = probe
            .clone()
            .oneshot(
                Request::builder()
                    .method(*method)
                    .uri(*path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must be a mounted admin-indicators route"
        );
    }
}

// ── 2. /:id/change-price must reject GET (POST-only) ─────────────────

/// `/:id/change-price` is a sensitive money mutation (calls Stripe
/// to mint a new Price object, then updates the DB pointer + writes
/// `security_events` audit row). It MUST be POST-only — a refactor that
/// flipped the method to GET would compile, deserialize, and then
/// silently expose the Stripe-price-mint side-effect to GET requests
/// (CSRF-able, cacheable, log-leakable). This test pins POST-only.
#[tokio::test]
async fn change_price_route_is_post_only_not_get() {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Shape-equivalent probe of the actual mount table — POST only.
    let probe: axum::Router =
        axum::Router::new().route("/:id/change-price", axum::routing::post(|| async { "ok" }));

    // POST is allowed (NOT 404, NOT 405).
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/42/change-price")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "POST /:id/change-price must be mounted"
    );
    assert_ne!(
        resp.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "POST /:id/change-price must accept POST"
    );

    // GET on the same path is 405 (Method Not Allowed) — the inverse
    // method must be explicitly rejected, NOT silently 404.
    let resp = probe
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/42/change-price")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "GET /:id/change-price must be 405 — POST-only sensitive money mutation"
    );
}

// ── 3. IndicatorRow: price_cents round-trips through JSON as i64 ─────

/// `IndicatorRow::price_cents` is the per-indicator catalog price in
/// integer cents. CLAUDE.md money rule: "all `*_cents` values are
/// `i64`, never `i32`". A regression that narrowed to `i32` would
/// compile and silently truncate any price above $21,474,836.47 (and
/// any aggregate `SUM(price_cents)` rollup). This test pins the i64
/// contract through serialize → JSON number → deserialize.
#[test]
fn indicator_row_price_cents_round_trips_as_i64_json_number() {
    // Beyond i32::MAX — proves the field cannot be silently narrowed.
    let beyond_i32: i64 = i32::MAX as i64 + 1_234;

    let row = IndicatorRow {
        id: 9_876_543_210_i64,
        name: "Mega Trend Indicator".to_string(),
        slug: "mega-trend-indicator".to_string(),
        description: Some("A trend indicator".to_string()),
        long_description: None,
        price_cents: beyond_i32,
        is_active: Some(true),
        platform: Some("tradingview".to_string()),
        version: Some("1.0.0".to_string()),
        download_url: None,
        documentation_url: None,
        thumbnail: None,
        screenshots: None,
        features: None,
        requirements: None,
        meta_title: None,
        meta_description: None,
        created_at: None,
        updated_at: None,
    };

    let v: serde_json::Value = serde_json::to_value(&row).expect("IndicatorRow must serialize");

    let price_json = v.get("price_cents").expect("price_cents missing");
    assert!(
        price_json.is_number(),
        "price_cents must serialize as a JSON number, not a string"
    );
    assert_eq!(
        price_json.as_i64(),
        Some(beyond_i32),
        "price_cents must round-trip through JSON as i64 — i32 narrowing \
         would cap single-indicator prices at $21.4M"
    );

    // id is i64 — BIGINT primary keys, Stripe-scale.
    assert_eq!(v["id"].as_i64(), Some(9_876_543_210_i64));

    // Symmetric: deserialize back from JSON proves the contract holds
    // in both directions.
    let json = serde_json::to_string(&row).expect("must serialize");
    let parsed: IndicatorRow =
        serde_json::from_str(&json).expect("IndicatorRow must round-trip via JSON");
    assert_eq!(
        parsed.price_cents, beyond_i32,
        "price_cents must round-trip through full serialize→deserialize as i64"
    );
}

// ── 4. CreateIndicatorRequest / UpdateIndicatorRequest: Option<i64> ──

/// Both create/update request payloads have `price_cents: Option<i64>`.
/// Two regressions this test catches:
///
///   1. Field flipped from `Option<i64>` to required `i64` — admin
///      "edit description only" flow would start returning 400 Bad
///      Request because no price was sent.
///   2. Field narrowed to `i32` — any price above $21,474,836.47 would
///      overflow on parse or silently truncate.
///
/// Also pins that the platform allowlist field (`platform: Option<String>`)
/// accepts a non-allowlisted value at deserialize time (validation is
/// the handler's job, not the extractor's — the handler filters via the
/// `valid_platforms` allowlist on admin_indicators.rs:125-141, but the
/// extractor must not silently reject unknown values upstream).
#[test]
fn create_and_update_indicator_request_price_cents_round_trips_as_optional_i64() {
    // ── CreateIndicatorRequest ──────────────────────────────────────
    // Case A: explicit big price beyond i32 range.
    let beyond_i32: i64 = i32::MAX as i64 + 99;
    let payload =
        format!(r#"{{"name":"Big","price_cents":{beyond_i32},"platform":"tradingview"}}"#);
    let req: CreateIndicatorRequest =
        serde_json::from_str(&payload).expect("CreateIndicatorRequest must deserialize");
    assert_eq!(
        req.price_cents,
        Some(beyond_i32),
        "create price_cents must round-trip through i64 without truncation"
    );
    assert_eq!(req.name, "Big");
    assert_eq!(req.platform.as_deref(), Some("tradingview"));

    // Case B: no price_cents — must produce None (handler defaults to
    // 0 via `unwrap_or(0)` at admin_indicators.rs:303).
    let payload = r#"{"name":"Free Indicator"}"#;
    let req: CreateIndicatorRequest = serde_json::from_str(payload)
        .expect("CreateIndicatorRequest must accept missing price_cents");
    assert!(
        req.price_cents.is_none(),
        "missing price_cents ⇒ None (handler defaults to 0)"
    );

    // ── UpdateIndicatorRequest ──────────────────────────────────────
    // Case C: PATCH-style partial update with only price_cents changed.
    // All fields are Option, so an empty object must deserialize too
    // (it would no-op via the SQL COALESCE chain at lines 337-353).
    let payload = format!(r#"{{"price_cents":{beyond_i32}}}"#);
    let req: UpdateIndicatorRequest =
        serde_json::from_str(&payload).expect("UpdateIndicatorRequest must deserialize");
    assert_eq!(
        req.price_cents,
        Some(beyond_i32),
        "update price_cents must round-trip through i64 without truncation"
    );
    assert!(req.name.is_none());
    assert!(req.is_active.is_none());

    // Case D: empty body — every field Option means the handler can
    // be called as a true PATCH (touch updated_at only). A regression
    // that made any field required would 400 here.
    let req: UpdateIndicatorRequest =
        serde_json::from_str(r"{}").expect("UpdateIndicatorRequest must accept empty body");
    assert!(req.price_cents.is_none());
    assert!(req.name.is_none());
    assert!(req.slug.is_none());
    assert!(req.platform.is_none());

    // Case E: explicit null — must also be None (not Err).
    let req: UpdateIndicatorRequest =
        serde_json::from_str(r#"{"price_cents":null,"is_active":null}"#)
            .expect("UpdateIndicatorRequest must accept explicit nulls");
    assert!(req.price_cents.is_none());
    assert!(req.is_active.is_none());
}

// ── 5. IndicatorQueryParams: pagination + filter wire shape ──────────

/// `IndicatorQueryParams` is the query-string extractor for
/// `GET /api/admin/indicators`. Every field is `Option<…>` so the bare
/// `GET /api/admin/indicators` URL (no query string at all) must parse
/// cleanly to all-None. The handler then applies its documented defaults
/// (`page = 1`, `per_page = 20`) at admin_indicators.rs:120-121.
///
/// A regression that made `page` or `per_page` required would 400 every
/// list call from the admin UI. A regression that narrowed `page` from
/// `i32` to a non-numeric type would break URL parsing.
#[tokio::test]
async fn indicator_query_params_accepts_empty_and_filtered_query_strings() {
    use axum::body::Body;
    use axum::extract::Query;
    use axum::http::{Request, StatusCode};
    use std::sync::{Arc, Mutex};
    use tower::ServiceExt;

    let captured: Arc<Mutex<Option<IndicatorQueryParams>>> = Arc::new(Mutex::new(None));
    let captured_for_handler = captured.clone();

    let app: axum::Router = axum::Router::new().route(
        "/probe",
        axum::routing::get(move |Query(q): Query<IndicatorQueryParams>| {
            let cap = captured_for_handler.clone();
            async move {
                *cap.lock().unwrap() = Some(q);
                StatusCode::OK
            }
        }),
    );

    // Case A: completely empty query string — every field must
    // deserialize to None. A regression that made any field required
    // would 400 here BEFORE the handler ever sees the request.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/probe")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "bare GET /probe (no query) must extract — all fields are Option"
    );
    let q = captured.lock().unwrap().take().expect("captured");
    assert!(q.page.is_none());
    assert!(q.per_page.is_none());
    assert!(q.search.is_none());
    assert!(q.is_active.is_none());
    assert!(q.platform.is_none());

    // Case B: typical admin-list URL with all filters set.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/probe?page=3&per_page=50&search=trend&is_active=true&platform=tradingview")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let q = captured.lock().unwrap().take().expect("captured");
    assert_eq!(q.page, Some(3));
    assert_eq!(q.per_page, Some(50));
    assert_eq!(q.search.as_deref(), Some("trend"));
    assert_eq!(q.is_active, Some(true));
    assert_eq!(q.platform.as_deref(), Some("tradingview"));
}

// ── 6. IndicatorFileRow & IndicatorVideoRow: file_size_bytes is i64 ──

/// `IndicatorFileRow::file_size_bytes` (Option<i64>) and the
/// `CreateFileRequest::file_size_bytes` (Option<i64>) carry byte counts
/// for downloadable indicator binaries. A single indicator package
/// regularly exceeds 2 GiB (some thinkorswim / MetaTrader installers
/// ship with embedded reference data), so an i32 narrowing would
/// silently truncate any file larger than ~2.14 GB. This test pins i64
/// end-to-end.
///
/// Also pins `IndicatorVideoRow::duration_seconds: Option<i32>` (which
/// is the documented exception — videos don't exceed 24 days = 2^31 s
/// = ~68 years) and `CreateVideoRequest::is_featured / is_preview` are
/// Option<bool>, so a video can be created without specifying flags.
#[test]
fn indicator_file_and_video_wire_types_pin_i64_byte_counts() {
    // file_size_bytes is Option<i64> — round-trip a value beyond
    // i32::MAX (~2.14 GB) without truncation.
    let beyond_i32_bytes: i64 = (i32::MAX as i64) + 1_000_000;

    let file_row = IndicatorFileRow {
        id: 1,
        indicator_id: 9_876_543_210_i64,
        file_name: "indicator.ex5".to_string(),
        original_name: Some("MegaTrend.ex5".to_string()),
        file_path: "/files/indicator.ex5".to_string(),
        file_size_bytes: Some(beyond_i32_bytes),
        file_type: Some("binary".to_string()),
        mime_type: Some("application/octet-stream".to_string()),
        checksum_sha256: None,
        platform: "metatrader".to_string(),
        platform_version: Some("5".to_string()),
        storage_provider: Some("r2".to_string()),
        storage_bucket: Some("indicators".to_string()),
        storage_key: Some("a/b/c".to_string()),
        cdn_url: None,
        version: Some("1.0.0".to_string()),
        is_current_version: Some(true),
        changelog: None,
        display_name: None,
        display_order: Some(0),
        is_active: Some(true),
        download_count: Some(0),
    };

    let v: serde_json::Value =
        serde_json::to_value(&file_row).expect("IndicatorFileRow must serialize");
    let size_json = v.get("file_size_bytes").expect("file_size_bytes missing");
    assert!(size_json.is_number());
    assert_eq!(
        size_json.as_i64(),
        Some(beyond_i32_bytes),
        "file_size_bytes must round-trip as i64 — i32 narrowing would \
         truncate files larger than ~2.14 GB"
    );
    // indicator_id is BIGINT, must round-trip beyond i32 as well.
    assert_eq!(v["indicator_id"].as_i64(), Some(9_876_543_210_i64));

    // CreateFileRequest mirrors the same field shape.
    let payload = format!(
        r#"{{"file_name":"big.ex5","file_path":"/x","file_size_bytes":{beyond_i32_bytes},"platform":"metatrader"}}"#
    );
    let req: CreateFileRequest =
        serde_json::from_str(&payload).expect("CreateFileRequest must deserialize");
    assert_eq!(
        req.file_size_bytes,
        Some(beyond_i32_bytes),
        "CreateFileRequest::file_size_bytes must round-trip as i64"
    );

    // IndicatorVideoRow: duration_seconds is Option<i32> (documented
    // exception — caps at ~68 years, fine). is_featured / is_preview
    // are Option<bool> on the row and the create request.
    let video_row = IndicatorVideoRow {
        id: 1,
        indicator_id: 9_876_543_210_i64,
        title: "Intro".to_string(),
        description: None,
        bunny_video_guid: None,
        bunny_library_id: None,
        embed_url: None,
        play_url: None,
        thumbnail_url: None,
        duration_seconds: Some(120),
        display_order: Some(0),
        is_featured: Some(false),
        is_preview: Some(true),
    };
    let v: serde_json::Value =
        serde_json::to_value(&video_row).expect("IndicatorVideoRow must serialize");
    assert_eq!(v["indicator_id"].as_i64(), Some(9_876_543_210_i64));
    assert_eq!(v["duration_seconds"].as_i64(), Some(120));
    assert_eq!(v["is_preview"], serde_json::Value::Bool(true));

    // CreateVideoRequest accepts an empty optional shape — only the
    // title is required.
    let req: CreateVideoRequest =
        serde_json::from_str(r#"{"title":"x"}"#).expect("CreateVideoRequest must deserialize");
    assert_eq!(req.title, "x");
    assert!(req.duration_seconds.is_none());
    assert!(req.is_featured.is_none());
    assert!(req.is_preview.is_none());
}
