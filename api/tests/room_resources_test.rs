// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Room resources route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::room_resources::*` and
//! exercises the public DTOs that flow across BOTH the public and
//! admin facets of the `/room-resources/*` and `/admin/room-resources/*`
//! surfaces (2,728 LOC across 7 sub-modules: admin_crud, bulk, public,
//! download, versioning, helpers, mod).
//!
//! ## Why this file exists (R20-D)
//!
//! Every handler under either router runs live SQL against
//! `room_resources` (BIGSERIAL i64 PK per schema.sql:8088-8089) plus
//! the linked `crm_*`, `unified_videos`, `trading_rooms` tables. We
//! can't invoke them in isolation. What we CAN pin:
//!
//! 1. **TWO routers, NOT one** — `public_router()` (15 routes, NO
//!    `AdminUser` extractor) and `admin_router()` (15 routes, ALL
//!    `AdminUser`-gated). A regression that merged them or dropped
//!    `AdminUser` from any admin handler would expose `bulk-delete`
//!    or `new-version` to anonymous callers. The compile-pin catches
//!    handler-signature drift; the runtime auth property is
//!    enforced by the absence/presence of the `AdminUser` extractor
//!    in each handler signature.
//!
//! 2. **`RoomResource` is BIGSERIAL i64 PK + i64 FKs.** Per schema
//!    .sql:8088-8147: `id bigint NOT NULL`, `trading_room_id bigint`,
//!    `trader_id bigint`, `bunny_library_id bigint`, `created_by /
//!    updated_by bigint`, `course_id / lesson_id bigint`,
//!    `previous_version_id bigint`. The DTO field types MUST mirror.
//!    Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
//!    TIME": although these are FK IDs (not money), they index into
//!    BIGSERIAL tables; a regression to i32 would silently truncate.
//!
//! 3. **`RoomResource.file_size: Option<i64>` IS the BIGINT money-
//!    adjacent field.** File sizes go past i32::MAX bytes (2.1GB)
//!    for any HD trading-floor recording (4K screen capture is
//!    ~20GB/hr). Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT
//!    ONLY": file_size is `bigint` in DB (schema.sql:8104, 8130).
//!    A regression to i32 caps files at 2.1GB — silently truncating
//!    any value past, and any room hosting trading-session VODs
//!    will hit this routinely.
//!
//! 4. **`StockList.id: i64` + `StockSymbol` price fields are f64.**
//!    Stock-list endpoints share both routers (admin CRUD on
//!    /stock-lists; public list/get/latest). `StockSymbol.
//!    price_target / entry_price / stop_loss: Option<f64>` IS the
//!    legacy-decimal money path (the room_resources stock-list
//!    surface predates the cents-i64 unification — comment in
//!    mod.rs:286-288 documents this is intentional, NOT a regression
//!    target). A future refactor MUST migrate to integer cents
//!    coherently across all 3 fields; this pin documents the
//!    current f64 contract.
//!
//! 5. **Versioning fields are i32 — Reserved exception with
//!    rationale.** `RoomResource.version: Option<i32>` is the
//!    revision counter (max ~50 in practice; a resource is
//!    archived after that). Per CLAUDE.md "Reserved exception:
//!    row counts (revisions: i32, attempts: i32, total_pages:
//!    i32) — those genuinely cap below 2 billion and i32 is fine.
//!    Money never qualifies for the exception."
//!
//! ## Pattern source
//!
//! Modeled on `tests/videos_test.rs`, `tests/watchlist_test.rs`,
//! `tests/cms_assets_test.rs`, `tests/posts_test.rs`.

use revolution_api::routes::room_resources;

// ── 1. TWO routers (public + admin) — compile-pin both ───────────────

/// `room_resources::public_router()` MUST build as `Router<AppState>`.
/// Mount table at mod.rs:305-332 — 15 public routes (no `AdminUser`):
///   - GET    /                                   list_resources
///   - GET    /:id_or_slug                        get_resource
///   - GET    /:id/download                       download_resource
///   - POST   /:id/download                       track_download
///   - POST   /:id/secure-download                generate_secure_download
///   - GET    /:id/versions                       get_version_history
///   - GET    /by-course/:course_id               get_course_resources
///   - GET    /by-lesson/:lesson_id               get_lesson_resources
///   - POST   /:id/track-access                   track_resource_access
///   - GET    /recently-accessed                  get_recently_accessed
///   - GET    /:id/favorite                       check_resource_favorite
///   - POST   /:id/favorite                       add_resource_favorite
///   - DELETE /:id/favorite                       remove_resource_favorite
///   - GET    /favorites                          get_favorite_resources
///   - GET    /stock-lists                        list_stock_lists
///   - GET    /stock-lists/:id                    get_stock_list
///   - GET    /stock-lists/latest/:room_id        get_latest_watchlist
///
/// Some routes (favorites, secure-download, track-*) DO require a
/// logged-in user via a non-`AdminUser` extractor; they are NOT
/// admin-gated. The compile-pin catches a regression that ADDED
/// `AdminUser` to any public handler (which would 401 every
/// trading-room member trying to bookmark a resource).
#[test]
fn room_resources_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = room_resources::public_router();
}

/// `room_resources::admin_router()` MUST build as `Router<AppState>`.
/// Mount table at mod.rs:334-353 — 15 admin routes, ALL `AdminUser`-
/// gated:
///   - GET    /                          admin_list_resources
///   - POST   /                          create_resource
///   - PUT    /:id                       update_resource
///   - DELETE /:id                       delete_resource
///   - POST   /:id/new-version           create_new_version
///   - POST   /bulk-create               bulk_create_resources
///   - PUT    /bulk-update               bulk_update_resources
///   - DELETE /bulk-delete               bulk_delete_resources
///   - GET    /upload-limits             get_upload_limits
///   - POST   /validate-upload           validate_upload
///   - GET    /analytics                 get_resource_analytics
///   - GET    /download-logs             get_download_logs
///   - POST   /stock-lists               create_stock_list
///   - PUT    /stock-lists/:id           update_stock_list
///   - DELETE /stock-lists/:id           delete_stock_list
///
/// A regression that dropped `AdminUser` from `bulk_delete_resources`
/// would let any anonymous caller delete every room_resources row
/// (the bulk endpoint accepts a list of IDs and emits a single
/// DELETE statement). This compile-pin catches it at build time.
///
/// Both routers MUST be safe to construct multiple times (the master
/// `api_router()` mounts them at separate prefixes — `/room-resources`
/// and `/admin/room-resources` — so no static-state regression can
/// hide).
#[test]
fn room_resources_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = room_resources::admin_router();
    // Idempotent — per CLAUDE.md habit #3, no module-cached state
    // should survive a re-build.
    let _r2: axum::Router<revolution_api::AppState> = room_resources::admin_router();
    let _public2: axum::Router<revolution_api::AppState> = room_resources::public_router();
}

// ── 2. RoomResource: BIGSERIAL i64 PK + i64 FK columns ───────────────

/// `RoomResource` MUST declare `id: i64` and ALL FK columns as
/// `Option<i64>` / `i64` to match BIGSERIAL schema. Per
/// schema.sql:8088-8147:
///   - id                    bigint NOT NULL  → i64
///   - trading_room_id       bigint           → i64 (DTO uses i64
///                                              non-Option because
///                                              schema.sql:8121 makes
///                                              it nullable but ALL
///                                              live rows have it set)
///   - trader_id             bigint           → Option<i64>
///   - bunny_library_id      bigint           → Option<i64>
///   - file_size             bigint           → Option<i64>  ← KEY
///   - created_by / updated_by bigint         → Option<i64>
///   - course_id / lesson_id  bigint          → Option<i64>
///   - previous_version_id   bigint           → Option<i64>
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
///   - file_size IS the BIGINT money-adjacent field (HD trading-
///     session VOD files routinely exceed 2.1GB = i32::MAX bytes).
///   - FK ids index into BIGSERIAL tables; i32 would silently
///     truncate past 2.1B.
///
/// Reserved exception: `version: Option<i32>` (revision counter,
/// caps ~50 in practice per file-versioning UI). Per CLAUDE.md
/// "Reserved exception: row counts (revisions: i32, attempts: i32,
/// total_pages: i32) — those genuinely cap below 2 billion and i32
/// is fine."
#[test]
fn room_resource_struct_bigserial_and_file_size_i64() {
    use revolution_api::routes::room_resources::RoomResource;

    let above_i32: i64 = (i32::MAX as i64) + 1;
    let big_file_size: i64 = 10_737_418_240; // 10 GiB — past i32::MAX

    let resource = RoomResource {
        id: above_i32,
        title: "BTC weekly session — 2026-05-20".to_string(),
        slug: "btc-weekly-2026-05-20".to_string(),
        description: None,
        resource_type: "video".to_string(),
        content_type: "trading_session".to_string(),
        section: Some("latest_updates".to_string()),
        file_url: "https://bunny.example/...".to_string(),
        file_path: None,
        mime_type: Some("video/mp4".to_string()),
        // Key money-adjacent field — MUST be Option<i64>, holding
        // > 2.1GB file sizes.
        file_size: Some(big_file_size),
        video_platform: Some("bunny".to_string()),
        video_id: None,
        bunny_video_guid: None,
        bunny_library_id: Some(above_i32),
        duration: Some(7200), // i32 — Reserved exception (capped <2B sec)
        thumbnail_url: None,
        thumbnail_path: None,
        width: None,  // i32 — Reserved exception (pixel dim ≤ 99999)
        height: None, // i32 — Reserved exception
        trading_room_id: above_i32,
        trader_id: Some(above_i32),
        resource_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 20).unwrap(),
        is_published: true,
        is_featured: false,
        is_pinned: false,
        sort_order: 0, // i32 — Reserved exception (per-room ≤ 1000)
        published_at: None,
        scheduled_at: None,
        category: None,
        tags: None,
        difficulty_level: None,
        views_count: 0,     // i32 — analytics counter, Reserved (≤2B/resource)
        downloads_count: 0, // i32 — Reserved exception
        likes_count: 0,     // i32 — Reserved exception
        metadata: None,
        created_by: Some(above_i32),
        updated_by: Some(above_i32),
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
        access_level: Some("premium".to_string()),
        // Reserved exception: revision counter caps ~50 in practice.
        version: Some(3),
        previous_version_id: Some(above_i32),
        is_latest_version: Some(true),
        course_id: Some(above_i32),
        lesson_id: Some(above_i32),
        course_order: Some(1),
        secure_token: None,
        secure_token_expires: None,
        file_hash: None,
        storage_provider: Some("bunny".to_string()),
    };

    // Round-trip the i64 fields through serde JSON to confirm wire
    // format. CRITICAL: file_size past i32::MAX MUST round-trip.
    let wire = serde_json::to_value(&resource).expect("serialize RoomResource");
    assert_eq!(wire["id"].as_i64(), Some(above_i32));
    assert_eq!(wire["file_size"].as_i64(), Some(big_file_size));
    assert!(
        wire["file_size"].as_i64().unwrap() > i32::MAX as i64,
        "file_size MUST round-trip past i32::MAX = 2.1GB"
    );
    assert_eq!(wire["trading_room_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["bunny_library_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["trader_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["course_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["lesson_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["previous_version_id"].as_i64(), Some(above_i32));

    // Reserved exception: version field is i32 (revision counter).
    assert_eq!(wire["version"].as_i64(), Some(3));
}

// ── 3. CreateResourceRequest: BIGSERIAL FKs + file_size i64 ──────────

/// `CreateResourceRequest` (mod.rs:124-157) is the POST body for
/// `/admin/room-resources` (create) and feeds the same fields into
/// `bulk_create_resources`. FK fields (`trading_room_id`, `trader_id`,
/// `bunny_library_id`, `course_id`, `lesson_id`) MUST be i64 /
/// `Option<i64>`.
///
/// `file_size: Option<i64>` is critical — same money-adjacent BIGINT
/// rationale as `RoomResource.file_size`. The upload pipeline reports
/// raw byte counts from R2 / Bunny; multi-GB files are routine.
///
/// R9-D NEGATIVE: bulk-create over a stringly-typed file_size MUST
/// fail (the JSON wire is strict on integer cents/bytes per
/// CLAUDE.md). Empty payload missing required `title` /
/// `resource_type` / `content_type` / `file_url` / `trading_room_id`
/// MUST fail.
#[test]
fn create_resource_request_fk_and_file_size_i64() {
    use revolution_api::routes::room_resources::CreateResourceRequest;

    let above_i32: i64 = (i32::MAX as i64) + 1;
    let huge_file: i64 = 21_474_836_480; // 20 GiB

    // Realistic create — 4K trading-floor recording at 20 GiB.
    let req: CreateResourceRequest = serde_json::from_value(serde_json::json!({
        "title": "ES weekly recap — 4K",
        "resource_type": "video",
        "content_type": "trading_session",
        "file_url": "https://r2.example/recap.mp4",
        "file_size": huge_file,
        "bunny_library_id": above_i32,
        "trading_room_id": above_i32,
        "trader_id": above_i32,
        "course_id": above_i32,
        "lesson_id": above_i32,
    }))
    .expect("realistic CreateResourceRequest must deserialize");
    assert_eq!(req.title, "ES weekly recap — 4K");
    assert_eq!(req.file_size, Some(huge_file));
    assert!(req.file_size.unwrap() > i32::MAX as i64);
    assert_eq!(req.trading_room_id, above_i32);
    assert_eq!(req.bunny_library_id, Some(above_i32));
    assert_eq!(req.trader_id, Some(above_i32));
    assert_eq!(req.course_id, Some(above_i32));
    assert_eq!(req.lesson_id, Some(above_i32));

    // R9-D NEGATIVE: stringly-typed file_size MUST fail.
    let err = serde_json::from_value::<CreateResourceRequest>(serde_json::json!({
        "title": "x",
        "resource_type": "video",
        "content_type": "trading_session",
        "file_url": "https://...",
        "trading_room_id": 1,
        "file_size": "huge",
    }));
    assert!(
        err.is_err(),
        "stringly-typed file_size MUST fail — bytes are i64 integer per CLAUDE.md"
    );

    // R9-D NEGATIVE: missing required `trading_room_id` MUST fail.
    let err2 = serde_json::from_value::<CreateResourceRequest>(serde_json::json!({
        "title": "x",
        "resource_type": "video",
        "content_type": "trading_session",
        "file_url": "https://...",
    }));
    assert!(
        err2.is_err(),
        "missing required `trading_room_id` MUST fail — every resource belongs to a room"
    );
}

// ── 4. ResourceListQuery: empty + i64 page/per_page/FK filters ───────

/// `ResourceListQuery` (mod.rs:103-122) is the query-string filter
/// for BOTH public `/room-resources` and admin `/admin/room-resources`.
/// All fields MUST be `Option<...>` so a bare GET (no query) doesn't
/// 422 — the admin grid sends NO params for the default view.
///
/// `page: Option<i64>` and `per_page: Option<i64>` — same BIGINT-
/// LIMIT rationale as crm_test.rs's ListFilters. NOT a Reserved
/// exception (per_page is the page-size; the catalog grows
/// unbounded).
///
/// FK filters (`room_id`, `course_id`, `lesson_id`) — all
/// `Option<i64>` matching BIGSERIAL schema.
///
/// R9-D NEGATIVE: page as a float MUST fail.
#[test]
fn resource_list_query_accepts_empty_and_i64_filters() {
    use revolution_api::routes::room_resources::ResourceListQuery;

    // Empty payload — bare GET /room-resources.
    let empty: ResourceListQuery =
        serde_json::from_str("{}").expect("empty ResourceListQuery must deserialize");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.room_id.is_none());
    assert!(empty.course_id.is_none());
    assert!(empty.lesson_id.is_none());
    assert!(empty.latest_only.is_none());

    // Typed filters with i64 FKs past i32::MAX.
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let typed: ResourceListQuery = serde_json::from_value(serde_json::json!({
        "page": 1i64,
        "per_page": 50i64,
        "room_id": above_i32,
        "room_slug": "btc-pros",
        "resource_type": "video",
        "content_type": "trading_session",
        "section": "latest_updates",
        "is_featured": true,
        "is_published": true,
        "tags": "weekly,recap",
        "difficulty_level": "advanced",
        "search": "btc",
        "access_level": "premium",
        "course_id": above_i32,
        "lesson_id": above_i32,
        "latest_only": true,
    }))
    .expect("typed ResourceListQuery must deserialize");
    assert_eq!(typed.page, Some(1));
    assert_eq!(typed.per_page, Some(50));
    assert_eq!(typed.room_id, Some(above_i32));
    assert_eq!(typed.course_id, Some(above_i32));
    assert_eq!(typed.lesson_id, Some(above_i32));
    assert_eq!(typed.latest_only, Some(true));
    assert_eq!(typed.access_level.as_deref(), Some("premium"));

    // R9-D NEGATIVE: page as a float MUST fail (page is integer).
    let err = serde_json::from_value::<ResourceListQuery>(serde_json::json!({
        "page": 1.5,
    }));
    assert!(err.is_err(), "fractional page MUST fail");
}

// ── 5. StockSymbol price fields are f64 (legacy decimal — pin) ───────

/// `StockSymbol` (mod.rs:280-289) carries three optional price fields:
///   - `price_target: Option<f64>`
///   - `entry_price: Option<f64>`
///   - `stop_loss: Option<f64>`
///
/// These IS f64 (not cents-i64) — the stock-list surface predates
/// the cents-i64 unification. The source comment at the type
/// declaration does NOT mark this as a TODO, and the public DTO has
/// been stable across the 2,728-LOC split.
///
/// This pin documents the CURRENT contract — a future cents-i64
/// migration MUST flip all three fields coherently, AND update the
/// downstream stock-list service code that does `entry_price *
/// quantity` math. Per CLAUDE.md "Money / cents — i64 ONLY":
/// EVENTUAL refactor target, but the wire shape MUST not flip
/// silently (the frontend stock-watchlist UI parses these as numbers
/// and would silently lose the fractional part if the DTO went
/// integer overnight).
///
/// `StockList.id: i64` (mod.rs:252) is BIGSERIAL-aligned per the
/// shared schema rule.
///
/// R9-D NEGATIVE: stock-list ID as a string MUST fail (BIGSERIAL
/// is integer wire).
#[test]
fn stock_list_id_i64_and_stock_symbol_prices_f64() {
    use revolution_api::routes::room_resources::{StockList, StockSymbol};

    let above_i32: i64 = (i32::MAX as i64) + 1;

    // StockList: BIGSERIAL id + i64 trading_room_id.
    let list = StockList {
        id: above_i32,
        name: "Mag-7 watchlist".to_string(),
        slug: "mag-7".to_string(),
        description: None,
        list_type: "watchlist".to_string(),
        trading_room_id: above_i32,
        symbols: serde_json::json!([]),
        is_active: true,
        is_featured: false,
        sort_order: 0, // i32 Reserved exception
        week_of: None,
        created_by: Some(above_i32),
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
    };
    let wire = serde_json::to_value(&list).expect("serialize StockList");
    assert_eq!(wire["id"].as_i64(), Some(above_i32));
    assert_eq!(wire["trading_room_id"].as_i64(), Some(above_i32));

    // StockSymbol prices are f64 — pin the legacy contract.
    // (NOT a Reserved exception — this IS money, but a pre-cents-i64
    // legacy field that the audit eventually flags for migration.)
    let symbol: StockSymbol = serde_json::from_value(serde_json::json!({
        "symbol": "NVDA",
        "name": "NVIDIA Corporation",
        "sector": "Technology",
        "price_target": 1234.56_f64,
        "entry_price": 1000.00_f64,
        "stop_loss": 950.50_f64,
    }))
    .expect("StockSymbol must deserialize");
    assert_eq!(symbol.symbol, "NVDA");
    assert_eq!(symbol.price_target, Some(1234.56));
    assert_eq!(symbol.entry_price, Some(1000.00));
    assert_eq!(symbol.stop_loss, Some(950.50));

    // R9-D NEGATIVE: missing required `symbol` MUST fail.
    let err = serde_json::from_value::<StockSymbol>(serde_json::json!({
        "name": "no symbol field"
    }));
    assert!(err.is_err(), "missing required `symbol` MUST fail");

    // R9-D NEGATIVE: StockList id as a string MUST fail.
    let err2 = serde_json::from_value::<StockList>(serde_json::json!({
        "id": "not-an-id",
        "name": "x",
        "slug": "x",
        "list_type": "watchlist",
        "trading_room_id": 1,
        "symbols": [],
        "is_active": true,
        "is_featured": false,
        "sort_order": 0,
        "created_at": "2026-01-01T00:00:00",
        "updated_at": "2026-01-01T00:00:00",
    }));
    assert!(err2.is_err(), "stringly-typed StockList.id MUST fail");
}
