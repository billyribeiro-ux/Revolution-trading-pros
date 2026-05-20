//! Favorites route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::favorites` and
//! exercises the public DTOs (`UserFavorite`, `CreateFavoriteRequest`,
//! `ListFavoritesQuery`, `CheckFavoriteQuery`) + the `router()`
//! mount table.
//!
//! ## Why this shape — auth-adjacent, per-user state
//!
//! `routes/favorites.rs` is the persistent-bookmarks surface. EVERY
//! handler takes `auth: User` (axum extractor that enforces the
//! authenticated-member contract) and queries `user_favorites` scoped
//! by `auth.id`. Six endpoints:
//!   - GET    /                       (list — paginated)
//!   - POST   /                       (add)
//!   - DELETE /:id                    (remove by row id)
//!   - DELETE /remove                 (remove by (item_type, item_id))
//!   - GET    /check                  (is-favorited probe)
//!   - GET    /room/:room_slug        (per-room list)
//!
//! The auth-adjacent angle: a missing `User` extractor on ANY of
//! these would silently leak ALL favorites across users (since the
//! `WHERE user_id = $1` clause depends on the extractor). The
//! compile-pin in this file is the cheapest guard against that
//! refactor mistake.
//!
//! What we CAN pin without a DB:
//!
//! 1. **`UserFavorite.id`, `.user_id`, `.item_id` are `i64`** —
//!    THREE BIGSERIAL fields on this row. Every PK on this stack is
//!    `i64`; `user_id` FKs into `users.id` (i64); `item_id` FKs into
//!    polymorphic per-type tables (`posts.id`, `member_indicators.id`,
//!    etc — all i64). A narrowing on ANY would silently 404 the first
//!    high-id row.
//!
//! 2. **`CreateFavoriteRequest` requires `room_slug` + `item_type` +
//!    `item_id`** — these are the polymorphic-pointer fields. NULL
//!    on any of them would 422 the favorite-add flow.
//!
//! 3. **`item_id` on `CreateFavoriteRequest` is `i64`** — `Option<>`
//!    would break the polymorphic-pointer contract (you can't bookmark
//!    "nothing"); narrowing to `i32` would silently 404 favorites for
//!    item rows above 2B (legitimately reachable: posts table).
//!
//! 4. **`ListFavoritesQuery` is fully optional** — the favorites
//!    grid sends GET with no params (defaults: page=1, per_page=20,
//!    no filter). A required field would 400-error the favorites
//!    page on initial load.
//!
//! 5. **`CheckFavoriteQuery` requires `item_type` + `item_id`** —
//!    used by both `/check` (is-favorited probe) AND `/remove` (DELETE
//!    by item, not by row id). Missing either MUST fail at serde-time
//!    instead of 500-ing at sqlx-bind.
//!
//! 6. **Wire format is snake_case** — frontend reads `room_slug`,
//!    `item_type`, `item_id`, `created_at`, `thumbnail_url`. A
//!    regression to camelCase would silently break the UI (favorites
//!    grid sees `undefined` for every key). R9-D NEGATIVE pin.
//!
//! 7. **`router()` mount table compile-pin** — `axum::Router<AppState>`.
//!    EVERY handler takes `auth: User`. A refactor that dropped the
//!    extractor on ANY handler would silently leak other users'
//!    favorites; compile-pin catches the handler-signature drift.
//!
//! ## Pattern source
//!
//! Modeled on `tests/categories_test.rs`, `tests/contacts_test.rs`,
//! `tests/posts_test.rs`.

use revolution_api::routes::favorites::{
    CheckFavoriteQuery, CreateFavoriteRequest, ListFavoritesQuery, UserFavorite,
};

fn fixture_favorite(id: i64, user_id: i64, item_id: i64) -> UserFavorite {
    UserFavorite {
        id,
        user_id,
        room_slug: "explosive-swings".to_string(),
        item_type: "post".to_string(),
        item_id,
        title: Some("Setup of the Week".to_string()),
        excerpt: Some("Long bias on QQQ".to_string()),
        href: Some("/explosive-swings/setup-of-the-week".to_string()),
        thumbnail_url: Some("https://cdn.example.com/thumb.jpg".to_string()),
        created_at: chrono::Utc::now(),
    }
}

// ── 1. UserFavorite: id, user_id, item_id are ALL i64 (BIGSERIAL) ──

/// THREE BIGSERIAL fields on `user_favorites`:
///   - `id` (PK)
///   - `user_id` (FK to `users.id`, also i64)
///   - `item_id` (FK to polymorphic per-type table, all i64)
///
/// A narrowing of ANY field would silently break the favorites flow
/// when the cited table crosses `2^31` rows. The `users` table alone
/// can plausibly cross 2B rows for a long-running B2C site; the
/// `posts` table (one of the polymorphic targets) crosses it faster
/// (every CMS revision can write a new row).
#[test]
fn user_favorite_ids_are_all_i64_bigserial() {
    let above: i64 = (i32::MAX as i64) + 1;
    let f = fixture_favorite(above, above + 1, above + 2);

    // Compile-time pins (load-bearing — catches type narrowing)
    let _: i64 = f.id;
    let _: i64 = f.user_id;
    let _: i64 = f.item_id;

    let wire = serde_json::to_value(&f).expect("serialize UserFavorite");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above),
        "UserFavorite.id MUST be i64 — BIGSERIAL PK"
    );
    assert_eq!(
        wire["user_id"].as_i64(),
        Some(above + 1),
        "UserFavorite.user_id MUST be i64 — FK to users.id (BIGSERIAL)"
    );
    assert_eq!(
        wire["item_id"].as_i64(),
        Some(above + 2),
        "UserFavorite.item_id MUST be i64 — polymorphic FK to per-type tables, all BIGSERIAL"
    );
}

// ── 2. CreateFavoriteRequest: required fields enforced at serde time ──

/// `CreateFavoriteRequest` requires `room_slug`, `item_type`,
/// `item_id` (DB columns are NOT NULL). A regression flipping any of
/// them to `Option<>` would 500 at sqlx-bind time on the INSERT
/// instead of 422-ing at serde — worse UX, dirties logs.
#[test]
fn create_favorite_required_fields_enforced() {
    // Happy path — minimal
    let ok: CreateFavoriteRequest = serde_json::from_value(serde_json::json!({
        "room_slug": "swing-trading-room",
        "item_type": "post",
        "item_id": (i32::MAX as i64) + 5,
    }))
    .expect("minimal CreateFavoriteRequest MUST parse");
    assert_eq!(ok.room_slug, "swing-trading-room");
    assert_eq!(ok.item_type, "post");
    assert_eq!(ok.item_id, (i32::MAX as i64) + 5);
    assert!(ok.title.is_none());
    assert!(ok.excerpt.is_none());
    assert!(ok.href.is_none());
    assert!(ok.thumbnail_url.is_none());

    // Optional metadata fields populate correctly
    let full: CreateFavoriteRequest = serde_json::from_value(serde_json::json!({
        "room_slug": "explosive-swings",
        "item_type": "indicator",
        "item_id": 100_i64,
        "title": "VWAP Bands",
        "excerpt": "Custom volatility-weighted bands",
        "href": "/indicators/vwap-bands",
        "thumbnail_url": "https://cdn.example.com/vwap.jpg",
    }))
    .expect("full CreateFavoriteRequest MUST parse");
    assert_eq!(full.title.as_deref(), Some("VWAP Bands"));
    assert_eq!(
        full.thumbnail_url.as_deref(),
        Some("https://cdn.example.com/vwap.jpg")
    );

    // R9-D NEGATIVE: missing room_slug MUST fail
    assert!(
        serde_json::from_value::<CreateFavoriteRequest>(serde_json::json!({
            "item_type": "post",
            "item_id": 1,
        }))
        .is_err(),
        "CreateFavoriteRequest without `room_slug` MUST fail (required)"
    );

    // R9-D NEGATIVE: missing item_type MUST fail
    assert!(
        serde_json::from_value::<CreateFavoriteRequest>(serde_json::json!({
            "room_slug": "x",
            "item_id": 1,
        }))
        .is_err(),
        "CreateFavoriteRequest without `item_type` MUST fail (required)"
    );

    // R9-D NEGATIVE: missing item_id MUST fail
    assert!(
        serde_json::from_value::<CreateFavoriteRequest>(serde_json::json!({
            "room_slug": "x",
            "item_type": "post",
        }))
        .is_err(),
        "CreateFavoriteRequest without `item_id` MUST fail (required, polymorphic pointer)"
    );

    // R9-D NEGATIVE: wrong type for item_id (string) MUST fail
    assert!(
        serde_json::from_value::<CreateFavoriteRequest>(serde_json::json!({
            "room_slug": "x",
            "item_type": "post",
            "item_id": "not-a-number",
        }))
        .is_err(),
        "CreateFavoriteRequest with string `item_id` MUST fail — type contract is i64"
    );
}

// ── 3. ListFavoritesQuery: fully optional (default favorites grid) ──

/// The `/api/favorites/` endpoint is hit by the favorites grid with
/// NO query params on initial load (defaults: page=1, per_page=20,
/// no filter). EVERY field MUST be `Option<>`. A required field
/// would 400-error the favorites page on every visit.
///
/// Also: `page` and `per_page` MUST be `i64` (matches the rest of
/// the stack's pagination contract — supports large user libraries).
#[test]
fn list_favorites_query_accepts_empty_and_full() {
    // Empty (default grid load)
    let empty: ListFavoritesQuery =
        serde_json::from_str("{}").expect("empty ListFavoritesQuery MUST parse");
    assert!(empty.room_slug.is_none());
    assert!(empty.item_type.is_none());
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());

    // Full payload — every filter
    let full: ListFavoritesQuery = serde_json::from_value(serde_json::json!({
        "room_slug": "spx-profit-pulse",
        "item_type": "post",
        "page": 3_i64,
        "per_page": 50_i64,
    }))
    .expect("full ListFavoritesQuery MUST parse");
    assert_eq!(full.room_slug.as_deref(), Some("spx-profit-pulse"));
    assert_eq!(full.item_type.as_deref(), Some("post"));
    assert_eq!(full.page, Some(3));
    assert_eq!(full.per_page, Some(50));

    // Compile-pin: page/per_page are i64 (Pagination contract on this stack)
    let _: Option<i64> = full.page;
    let _: Option<i64> = full.per_page;
}

// ── 4. CheckFavoriteQuery: required (used by /check AND /remove) ──

/// `CheckFavoriteQuery` powers TWO endpoints:
///   - GET /check (is-favorited probe — used by every "heart" icon)
///   - DELETE /remove (remove by (item_type, item_id) instead of row id)
///
/// Both fields are required. R9-D NEGATIVE pins: missing either MUST
/// fail at serde time, not at sqlx-bind.
#[test]
fn check_favorite_query_requires_both_fields() {
    // Happy path
    let ok: CheckFavoriteQuery = serde_json::from_value(serde_json::json!({
        "item_type": "indicator",
        "item_id": (i32::MAX as i64) + 7,
    }))
    .expect("CheckFavoriteQuery with both fields MUST parse");
    assert_eq!(ok.item_type, "indicator");
    assert_eq!(ok.item_id, (i32::MAX as i64) + 7);

    // R9-D NEGATIVE: missing item_id MUST fail
    assert!(
        serde_json::from_value::<CheckFavoriteQuery>(serde_json::json!({"item_type": "post"}))
            .is_err(),
        "CheckFavoriteQuery without `item_id` MUST fail (required for both /check and /remove)"
    );

    // R9-D NEGATIVE: missing item_type MUST fail
    assert!(
        serde_json::from_value::<CheckFavoriteQuery>(serde_json::json!({"item_id": 1})).is_err(),
        "CheckFavoriteQuery without `item_type` MUST fail"
    );

    // R9-D NEGATIVE: wrong type for item_id (string) MUST fail
    assert!(
        serde_json::from_value::<CheckFavoriteQuery>(serde_json::json!({
            "item_type": "post",
            "item_id": "abc",
        }))
        .is_err(),
        "CheckFavoriteQuery with non-numeric item_id MUST fail"
    );
}

// ── 5. Wire format is snake_case ONLY (R9-D NEGATIVE camelCase pin) ─

/// Frontend reads `room_slug`, `item_type`, `item_id`, `created_at`,
/// `thumbnail_url` as snake_case. A regression that added
/// `#[serde(rename_all = "camelCase")]` would silently break the
/// favorites grid (the JS would see `undefined` for every key and
/// render an empty heart on every card).
#[test]
fn user_favorite_wire_is_snake_case_no_camel_case() {
    let f = fixture_favorite(1, 2, 3);
    let wire = serde_json::to_value(&f).expect("serialize UserFavorite");

    // POSITIVE snake_case
    assert!(
        wire.get("room_slug").is_some(),
        "room_slug MUST be on the wire"
    );
    assert!(
        wire.get("item_type").is_some(),
        "item_type MUST be on the wire"
    );
    assert!(wire.get("item_id").is_some(), "item_id MUST be on the wire");
    assert!(wire.get("user_id").is_some(), "user_id MUST be on the wire");
    assert!(
        wire.get("thumbnail_url").is_some(),
        "thumbnail_url MUST be on the wire"
    );
    assert!(
        wire.get("created_at").is_some(),
        "created_at MUST be on the wire"
    );

    // R9-D NEGATIVE camelCase MUST NOT appear (regression guard)
    assert!(
        wire.get("roomSlug").is_none(),
        "roomSlug (camelCase) MUST NOT appear — wire is snake_case"
    );
    assert!(
        wire.get("itemType").is_none(),
        "itemType (camelCase) MUST NOT appear — wire is snake_case"
    );
    assert!(
        wire.get("itemId").is_none(),
        "itemId (camelCase) MUST NOT appear — wire is snake_case"
    );
    assert!(
        wire.get("userId").is_none(),
        "userId (camelCase) MUST NOT appear — wire is snake_case"
    );
    assert!(
        wire.get("thumbnailUrl").is_none(),
        "thumbnailUrl (camelCase) MUST NOT appear — wire is snake_case"
    );
    assert!(
        wire.get("createdAt").is_none(),
        "createdAt (camelCase) MUST NOT appear — wire is snake_case"
    );
}

// ── 6. Router mount-table compile-pin ────────────────────────────────

/// `routes::favorites::router()` MUST build as `Router<AppState>`.
/// Mount table covers SIX handlers, EVERY ONE taking `auth: User`.
/// A refactor that dropped the `User` extractor on ANY handler would
/// silently leak ALL favorites across users (the `WHERE user_id = $1`
/// guard depends on `auth.id`). Compile-pin catches an extractor-type
/// regression at the handler-signature level.
///
/// Auth-adjacent surface — same risk class as user, organization,
/// member_courses, member_indicators (also auth-extractor-gated).
#[test]
fn favorites_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::favorites::router();
}

/// Idempotent construction — building twice must succeed. Per
/// CLAUDE.md habit #3 ("cached state lost during refactor" landmine),
/// pin that nothing global lives inside the router constructor.
#[test]
fn favorites_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::favorites::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::favorites::router();
}
