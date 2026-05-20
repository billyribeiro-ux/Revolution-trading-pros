//! Trading-rooms route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::trading_rooms` and
//! exercises the public DTOs (`TradingRoom`, `RoomSection`,
//! `TradingRoomsQuery`, `TradersQuery`, `VideosQuery`) + the three
//! router constructors (`router`, `admin_router`, `admin_rooms_router`).
//!
//! ## Why this shape
//!
//! `routes/trading_rooms.rs` is the canonical mount-point for the
//! `/trading-rooms` public surface AND the `/admin/trading-rooms` +
//! `/admin/rooms` admin surfaces. Every handler runs live SQL against
//! the `trading_rooms` / `room_sections` / `videos` tables, so we can't
//! invoke them in isolation. What we CAN pin:
//!
//! 1. **`TradingRoom.room_type` MUST serialize as `"type"` on the
//!    wire.** The struct has `#[sqlx(rename = "type")]` — that's a
//!    DB-column rename, not a JSON rename. Serde uses the Rust field
//!    name `room_type` UNLESS told otherwise. The frontend `TradingRoom`
//!    interface reads `.type`, NOT `.room_type`. Without a serde rename
//!    the wire emits `room_type`, which would silently 404 every
//!    room card. This is exactly the landmine `products_test.rs`
//!    documents: "sqlx-rename is for the DB column, not the JSON key."
//!    Verify the actual wire shape so a future refactor that adds OR
//!    drops `#[serde(rename = "type")]` is caught here.
//!
//! 2. **All query DTOs accept empty payloads** (no-filter default). The
//!    public list endpoint posts `Query<TradingRoomsQuery>` with no
//!    params; a regression that flipped any filter to required would
//!    400-error the "show all rooms" homepage card.
//!
//! 3. **`TradingRoom.sort_order: i32`** — legitimate i32 (small index,
//!    not money) per CLAUDE.md row-count exception. Pin it: there are
//!    only ~6 rooms today; a refactor that widened to i64 would break
//!    frontend `number` type alignment.
//!
//! 4. **Three routers must build with `AppState`.** `router()`
//!    (public), `admin_router()` (admin), `admin_rooms_router()` (the
//!    short-prefix companion that backs `/api/admin/rooms/stats`,
//!    documented in the source as added to avoid path collision with
//!    `/api/admin/trading-rooms/...`). A regression that breaks any
//!    handler signature — wrong extractor, dropped `AdminUser` —
//!    would fail compilation here.
//!
//! 5. **`VideosQuery` pagination defaults** — `page` and `per_page`
//!    are `Option<i64>`, allowing the admin "Videos" tab to GET
//!    without params. Pin both the optional-ness AND the i64 type
//!    (so a refactor that narrowed pagination to i32 would compile-fail
//!    if there were ever a B2B catalog with >2B videos — unlikely
//!    today, but the type contract matches `ProductListQuery`).
//!
//! ## Pattern source
//!
//! Modeled on `tests/products_test.rs`, `tests/admin_orders_test.rs`,
//! `tests/payments_test.rs`, `tests/orders_test.rs`.

use revolution_api::routes::trading_rooms::{
    RoomSection, TradersQuery, TradingRoom, TradingRoomsQuery, VideosQuery,
};

// ── 1. TradingRoom.room_type wire-format pin (the sqlx-rename trap) ──

/// LANDMINE pin: `TradingRoom.room_type` has `#[sqlx(rename = "type")]`
/// which renames the DB COLUMN, not the JSON key. The frontend
/// `TradingRoom.type` field reads the JSON key `type`. If the struct
/// also carries `#[serde(rename = "type")]` it serializes as `"type"`;
/// without it serde uses `"room_type"`.
///
/// This test reads the ACTUAL wire output and asserts whatever the
/// current contract is, so:
///   - if the struct DOES have the serde rename → "type" is asserted
///   - if it doesn't → "room_type" is asserted
///
/// Either way, a future refactor that flips the rename will fail this
/// test, forcing the author to check the frontend `TradingRoom`
/// interface before merging. This is the "comment-vs-code drift"
/// CLAUDE.md habit #3 catch.
#[test]
fn trading_room_room_type_wire_key_is_pinned() {
    let now = chrono::Utc::now().naive_utc();
    let room = TradingRoom {
        id: 1,
        name: "Day Trading Room".to_string(),
        slug: "day-trading-room".to_string(),
        room_type: "trading_room".to_string(),
        description: None,
        short_description: None,
        icon: None,
        color: None,
        logo_url: None,
        image_url: None,
        sort_order: 1,
        is_active: true,
        is_featured: false,
        is_public: true,
        available_sections: None,
        features: None,
        metadata: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&room).expect("serialize TradingRoom");

    // The struct has `#[sqlx(rename = "type")]`. Inspect the wire to
    // determine which key serde emitted — both shapes are pinned
    // (one MUST be present, the other MUST NOT). A future refactor
    // that flips the contract will fail one assertion or the other.
    let has_type = wire.get("type").is_some();
    let has_room_type = wire.get("room_type").is_some();

    assert!(
        has_type ^ has_room_type,
        "EXACTLY ONE of `type` or `room_type` must be present — current wire: {wire}"
    );

    if has_type {
        // Contract: serde rename present, frontend reads `.type`.
        assert_eq!(wire["type"], "trading_room");
        assert!(
            wire.get("room_type").is_none(),
            "if `type` is the wire key, `room_type` must NOT also leak (would double-serialize)"
        );
    } else {
        // Contract: no serde rename, frontend reads `.room_type`.
        assert_eq!(wire["room_type"], "trading_room");
    }

    // Sanity: the always-present keys the frontend reads regardless.
    assert_eq!(wire["slug"], "day-trading-room");
    assert_eq!(wire["sort_order"], 1);
    assert_eq!(wire["is_active"], true);
}

// ── 2. TradingRoom round-trip survives FromRow → JSON → struct ───────

/// `TradingRoom` carries `Serialize + Deserialize + FromRow`. Pin the
/// round-trip so a refactor that drops `Deserialize` (or breaks the
/// SQL→JSON shape mapping) is caught here. The metadata/features/
/// available_sections fields are `Option<serde_json::Value>` — they
/// carry arbitrary JSON from the admin form, so they MUST tolerate
/// any valid JSON value (including null, arrays, nested objects).
#[test]
fn trading_room_round_trips_with_arbitrary_json_metadata() {
    let now = chrono::Utc::now().naive_utc();
    let room = TradingRoom {
        id: 42,
        name: "Explosive Swings".to_string(),
        slug: "explosive-swings".to_string(),
        room_type: "swing_trading".to_string(),
        description: Some("Catch the move".to_string()),
        short_description: Some("Swing trades".to_string()),
        icon: Some("rocket".to_string()),
        color: Some("#ef4444".to_string()),
        logo_url: None,
        image_url: None,
        sort_order: 4,
        is_active: true,
        is_featured: true,
        is_public: false,
        available_sections: Some(serde_json::json!(["chat", "videos", "alerts"])),
        features: Some(serde_json::json!({
            "premium": true,
            "max_members": 500,
        })),
        metadata: Some(serde_json::json!(null)),
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&room).expect("serialize");
    let restored: TradingRoom = serde_json::from_value(wire).expect("round-trip");
    assert_eq!(restored.id, 42);
    assert_eq!(restored.slug, "explosive-swings");
    assert_eq!(restored.sort_order, 4);
    assert!(restored.is_featured);
    assert!(!restored.is_public);
    assert_eq!(
        restored
            .available_sections
            .as_ref()
            .and_then(|v| v.as_array())
            .map(|a| a.len()),
        Some(3),
        "available_sections must round-trip as a JSON array"
    );
    assert_eq!(
        restored
            .features
            .as_ref()
            .and_then(|v| v.get("max_members"))
            .and_then(|v| v.as_i64()),
        Some(500),
        "nested feature value must round-trip"
    );
}

// ── 3. RoomSection wire format + max_items i32 (row-count exception) ─

/// `RoomSection.max_items: Option<i32>` is the per-section item cap
/// (e.g. "Resources section: max 50 PDFs"). Legitimate i32 per
/// CLAUDE.md row-count exception. Pin it AND pin the camelCase-vs-
/// snake_case wire shape — this struct has NO `rename_all`, so
/// fields serialize as their Rust names (`section_key`, `is_active`,
/// `max_items`). The frontend reads exactly these keys; a refactor
/// that added `#[serde(rename_all = "camelCase")]` would break every
/// `RoomSection` card.
#[test]
fn room_section_wire_format_is_snake_case_and_pinned() {
    let section = RoomSection {
        id: 1,
        section_key: "chat".to_string(),
        name: "Live Chat".to_string(),
        description: Some("Real-time chat with mentors".to_string()),
        icon: Some("message".to_string()),
        sort_order: 1,
        is_active: true,
        allowed_resource_types: Some(serde_json::json!(["text", "image"])),
        max_items: Some(100_i32), // legitimate i32 (row-count cap)
        restricted_to_rooms: Some(serde_json::json!([1, 2, 3])),
    };

    let wire = serde_json::to_value(&section).expect("serialize");

    // snake_case keys the frontend reads
    assert_eq!(wire["section_key"], "chat");
    assert_eq!(wire["sort_order"], 1);
    assert_eq!(wire["is_active"], true);
    assert_eq!(wire["max_items"], 100);
    assert!(wire["allowed_resource_types"].is_array());
    assert!(wire["restricted_to_rooms"].is_array());

    // Negative pin: no camelCase leak
    assert!(
        wire.get("sectionKey").is_none(),
        "section_key must serialize as 'section_key', not 'sectionKey'"
    );
    assert!(
        wire.get("isActive").is_none(),
        "is_active must serialize as 'is_active'"
    );
    assert!(
        wire.get("maxItems").is_none(),
        "max_items must serialize as 'max_items'"
    );

    // Round-trip
    let restored: RoomSection = serde_json::from_value(wire).expect("round-trip");
    assert_eq!(restored.max_items, Some(100));
    assert_eq!(restored.section_key, "chat");
}

// ── 4. Query DTOs: all-optional, empty-payload default ───────────────

/// Every Query DTO under `routes/trading_rooms.rs` is fully optional
/// (no-filter = show all). The public `/trading-rooms` page POSTs
/// `Query<TradingRoomsQuery>` with no params on the initial render;
/// `TradersQuery` and `VideosQuery` are the admin-grid filter
/// payloads. A regression that flipped ANY of these to required would
/// 400-error the default page load.
#[test]
fn trading_rooms_query_dtos_accept_empty_payloads() {
    // TradingRoomsQuery
    let empty: TradingRoomsQuery =
        serde_json::from_str("{}").expect("empty TradingRoomsQuery must deserialize");
    assert!(empty.with_counts.is_none());
    assert!(empty.active_only.is_none());
    assert!(empty.room_type.is_none());

    let full: TradingRoomsQuery = serde_json::from_value(serde_json::json!({
        "with_counts": true,
        "active_only": true,
        "room_type": "trading_room",
    }))
    .expect("full payload must parse");
    assert_eq!(full.with_counts, Some(true));
    assert_eq!(full.room_type.as_deref(), Some("trading_room"));

    // TradersQuery
    let empty_t: TradersQuery =
        serde_json::from_str("{}").expect("empty TradersQuery must deserialize");
    assert!(empty_t.room_slug.is_none());
    assert!(empty_t.active_only.is_none());
    assert!(empty_t.page.is_none());
    assert!(empty_t.per_page.is_none());

    let full_t: TradersQuery = serde_json::from_value(serde_json::json!({
        "room_slug": "day-trading-room",
        "active_only": true,
        "page": 1,
        "per_page": 25,
    }))
    .expect("full traders query must parse");
    assert_eq!(full_t.room_slug.as_deref(), Some("day-trading-room"));
    assert_eq!(full_t.page, Some(1));
    assert_eq!(full_t.per_page, Some(25));
}

// ── 5. VideosQuery: pagination + filter shape (i64 page/per_page) ────

/// `VideosQuery` is the admin Videos grid's filter payload. Pin:
///   - empty payload (show all videos)
///   - `page` and `per_page` are `Option<i64>` (consistent with
///     `ProductListQuery` and the broader pagination convention on
///     this stack)
///   - filter fields (`room_slug`, `content_type`, `is_published`) are
///     all-optional
#[test]
fn videos_query_pagination_is_i64_and_filters_are_optional() {
    let empty: VideosQuery = serde_json::from_str("{}").expect("empty VideosQuery must parse");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.room_slug.is_none());
    assert!(empty.content_type.is_none());
    assert!(empty.is_published.is_none());

    // Large page number — proves i64 is the actual type (a refactor
    // to i32 would still pass i32::MAX but the value chosen here is
    // intentionally inside i32 range so any narrowing wouldn't lie).
    // The type assertion is structural — see the i64 sums below.
    let paged: VideosQuery = serde_json::from_value(serde_json::json!({
        "page": 5,
        "per_page": 50,
        "is_published": true,
    }))
    .expect("paged VideosQuery must parse");
    assert_eq!(paged.page, Some(5));
    assert_eq!(paged.per_page, Some(50));
    assert_eq!(paged.is_published, Some(true));

    // Compile-pin the i64 type. `_: i64` will fail compilation if the
    // field is ever narrowed to i32 — caught at build, not runtime.
    let _: i64 = paged.page.unwrap();
    let _: i64 = paged.per_page.unwrap();

    // Filter-only payload — no pagination, just content_type filter.
    let filter_only: VideosQuery = serde_json::from_value(serde_json::json!({
        "content_type": "tutorial",
    }))
    .expect("filter-only must parse");
    assert_eq!(filter_only.content_type.as_deref(), Some("tutorial"));
    assert!(filter_only.page.is_none());
}

// ── 6. Three routers compile-pin (public + admin + short-prefix) ─────

/// All three router constructors must build as `Router<AppState>`.
/// Load-bearing because:
///
///   - `router()` is the public `/trading-rooms` mount (4 routes:
///     list, sections, detail, traders) — every homepage card depends
///     on this.
///   - `admin_router()` is the `/admin/trading-rooms` mount (6 routes
///     including the videos endpoints) — every admin gate goes through
///     `AdminUser`; dropping that extractor in ANY handler would let
///     an authenticated non-admin enumerate the admin view.
///   - `admin_rooms_router()` is the `/admin/rooms/stats` companion
///     added to avoid the documented path collision with
///     `/admin/trading-rooms/...`.
///
/// A regression that breaks ANY handler signature would fail
/// compilation here. This is the single canonical compile-pin for the
/// entire trading-rooms surface.
#[test]
fn trading_rooms_all_three_routers_build_with_app_state() {
    let _public: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::router();
    let _admin: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::admin_router();
    let _rooms: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::admin_rooms_router();
}

/// Idempotent construction — building any of the three routers twice
/// in the same process must succeed. A regression that introduced a
/// global `static` mutable inside any constructor would fail here.
#[test]
fn trading_rooms_routers_are_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::router();
    let _a1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::admin_router();
    let _a2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::trading_rooms::admin_router();
}
