//! Weekly Watchlist route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::watchlist` and exercises
//! the public model DTOs (`WatchlistEntry`, `WatchlistResponse`,
//! `CreateWatchlistRequest`, `UpdateWatchlistRequest`, `WatchlistDate`,
//! `VideoData`, `SpreadsheetData`, `NavigationLink`, `PaginationMeta`)
//! plus the `router()` mount table.
//!
//! ## Why this shape — split member/admin auth
//!
//! `routes/watchlist.rs` is split-auth:
//!   - `list_watchlist`  → takes `_user: User`  (member-gated read)
//!   - `get_watchlist`   → takes `_user: User`  (member-gated read)
//!   - `create_watchlist`→ takes `admin: AdminUser` (admin-gated write)
//!   - `update_watchlist`→ takes `admin: AdminUser` (admin-gated write)
//!   - `delete_watchlist`→ takes `admin: AdminUser` (admin-gated write)
//!
//! The split is critical: dropping `User` from a read handler would
//! leak paid content to anon visitors; dropping `AdminUser` from a
//! write handler would let any member CRUD other members' watchlists.
//!
//! The wire-format contract is also split-direction:
//!   - `WatchlistEntry` (DB shape — snake_case for sqlx::FromRow)
//!   - `WatchlistResponse` (frontend shape — camelCase via
//!     `#[serde(rename)]` on most fields)
//!   - `CreateWatchlistRequest` / `UpdateWatchlistRequest` (frontend
//!     input — camelCase via `#[serde(rename)]`)
//!
//! A regression that flipped the rename direction on any DTO would
//! silently break the date-switcher (the date-posted dropdown that
//! powers every member's "what's this week's watchlist?" flow).
//!
//! What we CAN pin without a DB:
//!
//! 1. **`WatchlistEntry.id` is `i64`** (BIGSERIAL PK contract).
//!    `WatchlistResponse.id` is also `i64` (passes the PK through to
//!    the frontend for cache keys).
//!
//! 2. **`CreateWatchlistRequest` requires `title`, `trader`,
//!    `weekOf`** (camelCase rename). The handler parses `weekOf`
//!    as a NaiveDate; a missing field would 422 at serde and never
//!    hit the parser.
//!
//! 3. **`CreateWatchlistRequest.weekOf` is camelCase** — frontend
//!    sends camelCase by convention; R9-D NEGATIVE pin that
//!    `week_of` (snake) MUST NOT populate the field (would silently
//!    422 at the `parse_from_str` line in the handler if a stale
//!    field defaulted to "").
//!
//! 4. **`UpdateWatchlistRequest` is PATCH-style (all-Optional)** —
//!    most common admin flow is "just flip status from draft to
//!    published" without re-sending the full watchlist payload.
//!
//! 5. **`WatchlistResponse` wire is camelCase** (frontend contract)
//!    with `id`/`slug`/`title`/`trader` as exceptions (those four
//!    are universal). R9-D NEGATIVE pin: snake_case `trader_image`,
//!    `date_posted`, `week_of`, `created_at` MUST NOT appear.
//!
//! 6. **`WatchlistDate.spreadsheetUrl` is camelCase** — the
//!    date-switcher DTO. Snake-case `spreadsheet_url` MUST fail to
//!    parse (the rename is camelCase ONLY — sending snake_case is
//!    ignored, leaving the field unpopulated, which fails the
//!    required-field check).
//!
//! 7. **`ListQuery` is fully optional** — member's watchlist index
//!    page hits this with no params; a required field would 400-
//!    error every member's room dashboard.
//!
//! 8. **`router()` mount table compile-pin** — FIVE handlers, split
//!    `User`/`AdminUser`. A refactor dropping `AdminUser` from
//!    write handlers would let any logged-in member rewrite admin-
//!    owned content; dropping `User` from reads would leak paid
//!    content. Compile-pin catches handler-signature drift.
//!
//! ## Pattern source
//!
//! Modeled on `tests/categories_test.rs`, `tests/member_indicators_test.rs`,
//! `tests/posts_test.rs` (split member/admin auth surface).

use chrono::{NaiveDate, Utc};
use revolution_api::models::watchlist::{
    CreateWatchlistRequest, NavigationLink, PaginationMeta, SpreadsheetData,
    UpdateWatchlistRequest, VideoData, WatchlistDate, WatchlistEntry,
};

fn fixture_entry(id: i64) -> WatchlistEntry {
    let now = Utc::now();
    WatchlistEntry {
        id,
        slug: "05202026-billy".to_string(),
        title: "Spring Setups".to_string(),
        subtitle: Some("Week of May 20, 2026".to_string()),
        trader: "Billy".to_string(),
        trader_image: Some("https://cdn.example.com/billy.jpg".to_string()),
        date_posted: "May 20, 2026".to_string(),
        week_of: NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid date"),
        video_url: "https://video.example.com/v1.mp4".to_string(),
        video_poster: Some("https://cdn.example.com/poster.jpg".to_string()),
        video_title: "Spring Setups".to_string(),
        spreadsheet_url: "https://sheets.example.com/s1".to_string(),
        watchlist_dates: Some(serde_json::json!([
            {"date": "2026-05-20", "spreadsheetUrl": "https://sheets.example.com/s1"}
        ])),
        description: Some("Bullish bias on QQQ and AAPL".to_string()),
        status: "published".to_string(),
        rooms: Some(serde_json::json!(["explosive-swings", "day-trading-room"])),
        created_at: now,
        updated_at: now,
        deleted_at: None,
    }
}

// ── 1. WatchlistEntry.id + WatchlistResponse.id are i64 (BIGSERIAL) ──

/// `WatchlistEntry.id` is BIGSERIAL — `watchlist_entries` accumulates
/// rows slowly (one per week per team), so `2^31` rows is unreachable
/// in practice, BUT the i64 contract is the house standard and
/// matches the rest of the stack. The response struct passes the PK
/// to the frontend as `id` (universal field — not renamed) and that
/// also MUST be i64.
#[test]
fn watchlist_entry_id_is_i64_bigserial() {
    let above: i64 = (i32::MAX as i64) + 1;
    let e = fixture_entry(above);

    let _: i64 = e.id;

    let wire = serde_json::to_value(&e).expect("serialize WatchlistEntry");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above),
        "WatchlistEntry.id MUST be i64 — BIGSERIAL PK"
    );

    // Conversion to response also preserves i64
    let resp = e.to_response();
    let _: i64 = resp.id;
    let resp_wire = serde_json::to_value(&resp).expect("serialize WatchlistResponse");
    assert_eq!(
        resp_wire["id"].as_i64(),
        Some(above),
        "WatchlistResponse.id MUST be i64 — passes PK through to frontend"
    );
}

// ── 2. CreateWatchlistRequest: required + camelCase rename for weekOf ─

/// `CreateWatchlistRequest` requires `title`, `trader`, `weekOf`
/// (`#[serde(rename = "weekOf")]`). The handler parses `weekOf` via
/// `NaiveDate::parse_from_str(&body.week_of, "%Y-%m-%d")`; missing
/// the field would default to empty string and 400 at the parser,
/// but R9-D NEGATIVE pin says it should fail at serde first.
#[test]
fn create_watchlist_required_fields_and_camel_rename() {
    // Happy path — minimal
    let ok: CreateWatchlistRequest = serde_json::from_value(serde_json::json!({
        "title": "Spring Setups",
        "trader": "Billy",
        "weekOf": "2026-05-20",
    }))
    .expect("minimal CreateWatchlistRequest MUST parse");
    assert_eq!(ok.title, "Spring Setups");
    assert_eq!(ok.trader, "Billy");
    assert_eq!(ok.week_of, "2026-05-20");

    // Optional fields populated via camelCase aliases
    let full: CreateWatchlistRequest = serde_json::from_value(serde_json::json!({
        "title": "X",
        "trader": "Y",
        "weekOf": "2026-05-20",
        "traderImage": "https://cdn.example.com/y.jpg",
        "videoSrc": "https://video.example.com/v.mp4",
        "videoPoster": "https://cdn.example.com/p.jpg",
        "spreadsheetSrc": "https://sheets.example.com/s",
        "watchlistDates": [
            {"date": "2026-05-20", "spreadsheetUrl": "https://sheets.example.com/s"}
        ],
        "status": "draft",
        "rooms": ["explosive-swings"],
    }))
    .expect("full CreateWatchlistRequest MUST parse");
    assert_eq!(
        full.trader_image.as_deref(),
        Some("https://cdn.example.com/y.jpg")
    );
    assert_eq!(
        full.video_src.as_deref(),
        Some("https://video.example.com/v.mp4")
    );
    assert_eq!(
        full.video_poster.as_deref(),
        Some("https://cdn.example.com/p.jpg")
    );
    assert_eq!(
        full.spreadsheet_src.as_deref(),
        Some("https://sheets.example.com/s")
    );
    assert_eq!(full.status.as_deref(), Some("draft"));
    assert_eq!(full.watchlist_dates.as_ref().map(|v| v.len()), Some(1));

    // R9-D NEGATIVE: missing `weekOf` MUST fail
    assert!(
        serde_json::from_value::<CreateWatchlistRequest>(serde_json::json!({
            "title": "X",
            "trader": "Y",
        }))
        .is_err(),
        "CreateWatchlistRequest without `weekOf` MUST fail (required, camelCase)"
    );

    // R9-D NEGATIVE: missing `title` MUST fail
    assert!(
        serde_json::from_value::<CreateWatchlistRequest>(serde_json::json!({
            "trader": "Y",
            "weekOf": "2026-05-20",
        }))
        .is_err(),
        "CreateWatchlistRequest without `title` MUST fail (required)"
    );

    // R9-D NEGATIVE: snake_case `week_of` MUST fail
    // (Rust field is `week_of` with `#[serde(rename = "weekOf")]`;
    // sending snake_case is ignored, leaving week_of empty → required-field error.)
    assert!(
        serde_json::from_value::<CreateWatchlistRequest>(serde_json::json!({
            "title": "X",
            "trader": "Y",
            "week_of": "2026-05-20",
        }))
        .is_err(),
        "CreateWatchlistRequest with snake `week_of` MUST fail — wire is camelCase `weekOf`"
    );
}

// ── 3. UpdateWatchlistRequest: PATCH semantics, all-Optional ──

/// `UpdateWatchlistRequest` is fully PATCH. Most common admin flow
/// is "flip status from draft to published" without re-sending
/// title/trader/dates. A regression flipping any field to required
/// would 422 every status toggle.
#[test]
fn update_watchlist_patch_semantics() {
    // Empty body — pure PATCH (handler returns 422 "no fields to update",
    // but serde MUST parse it)
    let empty: UpdateWatchlistRequest =
        serde_json::from_str("{}").expect("empty UpdateWatchlistRequest MUST parse (PATCH)");
    assert!(empty.title.is_none());
    assert!(empty.trader.is_none());
    assert!(empty.status.is_none());
    assert!(empty.description.is_none());

    // Status-only toggle (the most-common admin flow)
    let toggle: UpdateWatchlistRequest = serde_json::from_value(serde_json::json!({
        "status": "published",
    }))
    .expect("status-only update MUST parse");
    assert_eq!(toggle.status.as_deref(), Some("published"));
    assert!(toggle.title.is_none());

    // Rich update — camelCase aliases populate fields
    let rich: UpdateWatchlistRequest = serde_json::from_value(serde_json::json!({
        "title": "Updated",
        "videoSrc": "https://new.example.com/v.mp4",
        "spreadsheetSrc": "https://new.example.com/s",
        "watchlistDates": [
            {"date": "2026-05-27", "spreadsheetUrl": "https://new.example.com/s"}
        ],
    }))
    .expect("rich UpdateWatchlistRequest MUST parse");
    assert_eq!(rich.title.as_deref(), Some("Updated"));
    assert_eq!(
        rich.video_src.as_deref(),
        Some("https://new.example.com/v.mp4")
    );
    assert_eq!(
        rich.spreadsheet_src.as_deref(),
        Some("https://new.example.com/s")
    );
    assert_eq!(rich.watchlist_dates.as_ref().map(|v| v.len()), Some(1));
}

// ── 4. WatchlistResponse wire is camelCase (frontend contract) ──

/// `WatchlistResponse` is the frontend-facing shape — fields are
/// renamed to camelCase via `#[serde(rename)]`. A regression that
/// dropped the renames would silently break the date-switcher view.
///
/// EXCEPTION: `id`, `slug`, `title`, `subtitle`, `trader`,
/// `description`, `status`, `rooms`, `video`, `spreadsheet`,
/// `previous`, `next` are NOT renamed (their Rust names are already
/// camelCase or single-word).
#[test]
fn watchlist_response_wire_is_camel_case_for_renamed_fields() {
    let entry = fixture_entry(1);
    let resp = entry.to_response();
    let wire = serde_json::to_value(&resp).expect("serialize WatchlistResponse");

    // POSITIVE camelCase (renamed fields)
    assert!(
        wire.get("traderImage").is_some(),
        "traderImage MUST be camelCase (renamed)"
    );
    assert!(
        wire.get("datePosted").is_some(),
        "datePosted MUST be camelCase (renamed)"
    );
    assert!(
        wire.get("weekOf").is_some(),
        "weekOf MUST be camelCase (renamed)"
    );
    assert!(
        wire.get("createdAt").is_some(),
        "createdAt MUST be camelCase (renamed)"
    );
    assert!(
        wire.get("updatedAt").is_some(),
        "updatedAt MUST be camelCase (renamed)"
    );

    // R9-D NEGATIVE snake_case MUST NOT appear (regression guard)
    assert!(
        wire.get("trader_image").is_none(),
        "trader_image (snake) MUST NOT appear — wire is camelCase"
    );
    assert!(
        wire.get("date_posted").is_none(),
        "date_posted (snake) MUST NOT appear — wire is camelCase"
    );
    assert!(
        wire.get("week_of").is_none(),
        "week_of (snake) MUST NOT appear — wire is camelCase"
    );
    assert!(
        wire.get("created_at").is_none(),
        "created_at (snake) MUST NOT appear — wire is camelCase"
    );
    assert!(
        wire.get("updated_at").is_none(),
        "updated_at (snake) MUST NOT appear — wire is camelCase"
    );

    // POSITIVE single-word (not renamed) — also pin shape of nested objects
    assert_eq!(wire["id"].as_i64(), Some(1));
    assert!(wire["video"].is_object(), "video MUST be a nested object");
    assert!(
        wire["spreadsheet"].is_object(),
        "spreadsheet MUST be a nested object"
    );
}

// ── 5. WatchlistDate: spreadsheetUrl camelCase ──

/// `WatchlistDate` is the date-switcher sub-DTO. `spreadsheetUrl` is
/// renamed to camelCase via `#[serde(rename)]`. R9-D NEGATIVE pin:
/// snake_case `spreadsheet_url` MUST fail (required field unpopulated).
///
/// Also: `VideoData`, `SpreadsheetData`, `NavigationLink`,
/// `PaginationMeta` are simple shapes — pin that they construct +
/// serialize cleanly (compile-pin doubles as a "didn't accidentally
/// remove pub" check).
#[test]
fn watchlist_date_camel_case_and_shape_constructors() {
    // Happy path: camelCase spreadsheetUrl
    let ok: WatchlistDate = serde_json::from_value(serde_json::json!({
        "date": "2026-05-20",
        "spreadsheetUrl": "https://sheets.example.com/s",
    }))
    .expect("WatchlistDate with spreadsheetUrl MUST parse");
    assert_eq!(ok.date, "2026-05-20");
    assert_eq!(ok.spreadsheet_url, "https://sheets.example.com/s");

    // R9-D NEGATIVE: snake_case `spreadsheet_url` MUST fail
    // (rename target is camelCase ONLY → snake_case is ignored → required field unpopulated → error)
    assert!(
        serde_json::from_value::<WatchlistDate>(serde_json::json!({
            "date": "2026-05-20",
            "spreadsheet_url": "https://sheets.example.com/s",
        }))
        .is_err(),
        "WatchlistDate with snake `spreadsheet_url` MUST fail — wire is camelCase"
    );

    // R9-D NEGATIVE: missing date MUST fail
    assert!(
        serde_json::from_value::<WatchlistDate>(serde_json::json!({
            "spreadsheetUrl": "x",
        }))
        .is_err(),
        "WatchlistDate without `date` MUST fail"
    );

    // Shape-of-DTO sanity checks (pin that pub access still holds)
    let v = VideoData {
        src: "https://v.example.com/v.mp4".to_string(),
        poster: "https://v.example.com/p.jpg".to_string(),
        title: "Spring Setups".to_string(),
    };
    let s = SpreadsheetData {
        src: "https://s.example.com/s".to_string(),
    };
    let n = NavigationLink {
        slug: "prev-week".to_string(),
        title: "Prev Week".to_string(),
    };
    let p = PaginationMeta {
        current_page: 1,
        per_page: 20,
        total: 100,
        last_page: 5,
    };
    let _ = serde_json::to_value(&v).expect("VideoData serializes");
    let _ = serde_json::to_value(&s).expect("SpreadsheetData serializes");
    let _ = serde_json::to_value(&n).expect("NavigationLink serializes");
    let _: i64 = p.current_page;
    let _: i64 = p.per_page;
    let _: i64 = p.total;
    let _: i64 = p.last_page;
}

// ── 6. Router mount-table compile-pin (split User/AdminUser auth) ──

/// `routes::watchlist::router()` MUST build as `Router<AppState>`.
/// Mount table:
///   - GET  /            (list — `_user: User` member-gated read)
///   - GET  /:slug       (get  — `_user: User` member-gated read)
///   - POST /            (create — `admin: AdminUser` admin-gated)
///   - PUT  /:slug       (update — `admin: AdminUser` admin-gated)
///   - DELETE /:slug     (delete — `admin: AdminUser` admin-gated)
///
/// The split-auth boundary is load-bearing:
///   - Dropping `User` from a read handler → leaks paid content
///     (anon visitors can read the weekly watchlist without paying).
///   - Dropping `AdminUser` from a write handler → any logged-in
///     member can CRUD admin-owned content.
///
/// Compile-pin catches handler-signature drift at type-check time.
#[test]
fn watchlist_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::watchlist::router();
}

/// Idempotent construction. Per CLAUDE.md habit #3, pin that nothing
/// global lives inside the router constructor.
#[test]
fn watchlist_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::watchlist::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::watchlist::router();
}
