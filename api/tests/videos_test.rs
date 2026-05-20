//! Public videos route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::videos` and exercises the
//! public DTOs (`UnifiedVideoRow`, `VideoListQuery`, `TrackEventRequest`,
//! `WatchHistoryQuery`) + the public `router()` mount table. Also binds
//! the `models::video` response/meta types that flow back out to the
//! frontend (`VideoResponse`, `PaginationMeta`, `TagDetail`, `TraderInfo`,
//! `RoomInfo`).
//!
//! ## Why this file exists (R16-D)
//!
//! `routes/videos.rs` is 564 LOC of public-facing video endpoints
//! (list / get / track / weekly / history / related). The list handler
//! builds a parameterised WHERE clause via `sqlx::PgArguments` after a
//! FIX-2026-04-26 P1 SQL-injection remediation — every value is bound,
//! not interpolated. The PK type is `i64` (BIGSERIAL, schema.sql:10248
//! confirms `unified_videos.id bigint`). What we CAN pin in a no-DB
//! test:
//!
//! 1. **PK type — `UnifiedVideoRow.id` is `i64` (BIGSERIAL).**
//!
//!    The `unified_videos` table is `bigint NOT NULL` (schema.sql
//!    line 10248), and the handler `Path<i64>` extractors at
//!    `routes/videos.rs:386,408` confirm. The `track_event` handler
//!    embeds `Path<i64>` directly — a regression to `i32` would
//!    silently break video tracking on any video past row 2^31.
//!    Bunny CDN library IDs (`bunny_library_id: Option<i64>`) are
//!    similarly i64 because Bunny's API returns u64-shaped IDs.
//!
//!    R9-D NEGATIVE: a payload with `id` exceeding i64::MAX must
//!    not deserialize cleanly into `UnifiedVideoRow`. (Note: the
//!    Rust type is i64, so JSON > i64::MAX overflows serde_json.)
//!
//! 2. **`VideoListQuery` is fully optional.**
//!
//!    The public `/api/videos` endpoint lands with zero query
//!    params (homepage thumbnail strip). Every field
//!    (`page`, `per_page`, `content_type`, `room_id`, `tags`,
//!    `difficulty_level`, `search`) is `Option<_>`. A regression
//!    that flipped any to required would 422 every "list videos"
//!    fetch on first page load.
//!
//!    The handler clamps `per_page` to `[1, 100]` and `page` to
//!    `max(1)` AT THE HANDLER, so the DTO accepts any i64 — even
//!    negative — and the handler normalises.
//!
//! 3. **`TrackEventRequest.event_type` is required, but
//!    `timestamp_seconds` is `Option<i32>`.**
//!
//!    The video player emits `play` / `pause` / `complete` events
//!    without timestamps and `seek` events WITH timestamps. R9-D
//!    NEGATIVE: missing `event_type` MUST fail. `timestamp_seconds`
//!    is `i32` — per CLAUDE.md "Reserved exception" for bounded
//!    counters: video duration in seconds caps far below i32::MAX
//!    (~68 years of seconds), so i32 is fine. MONEY never qualifies.
//!
//! 4. **`WatchHistoryQuery` — `user_id` and `limit` both Optional
//!    i64.** The handler defaults `user_id` to 0 (no rows) and
//!    `limit` to 20, capped at 50. R9-D NEGATIVE: a non-integer
//!    `limit` MUST fail to parse.
//!
//! 5. **`VideoResponse.views_count` is i32 (counter bounded per
//!    video).** Per "Reserved exception" — `views_count` is bounded
//!    by what a single video can realistically accumulate. A
//!    SINGLE video reaching 2.1B views is implausible (top-1
//!    YouTube videos take ~5 years to reach that). MONEY would
//!    never qualify. `VideoResponse.id` and the assignment FKs
//!    (`trader.id`, `rooms[].id`) are i64 (BIGSERIAL).
//!
//! 6. **Router mount table — 6 routes.** Compile pin against
//!    handler signature drift. The `/:id_or_slug` extractor is
//!    `Path<String>` (the handler tries `.parse::<i64>()` first and
//!    falls back to slug lookup) — a regression to `Path<i64>`
//!    would silently 404 every slug-based video URL the frontend
//!    emits.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_popups_test.rs`,
//! `tests/cms_revisions_test.rs`, `tests/posts_test.rs`,
//! `tests/categories_test.rs`.

use chrono::{NaiveDate, NaiveDateTime};
use revolution_api::models::video::{
    PaginationMeta, RoomInfo, TagDetail, TraderInfo, VideoResponse,
};
use revolution_api::routes::videos::{
    TrackEventRequest, UnifiedVideoRow, VideoListQuery, WatchHistoryQuery,
};

// ── 1. PK type — UnifiedVideoRow.id is i64 (BIGSERIAL, schema:10248) ─

/// `unified_videos.id` is `bigint NOT NULL` in PostgreSQL
/// (schema.sql:10248). The Rust struct mirrors with `id: i64`.
/// `bunny_library_id` is also i64 because Bunny's API returns
/// 64-bit IDs. The route extractors `Path<i64>` for
/// `:id/related` and `:id/track` would silently 400 on any URL
/// containing a row past 2^31 if the PK regressed to i32.
///
/// Positive pin: build a row with id near i64::MAX and confirm
/// the value round-trips as JSON (Serialize-only — the type
/// derives `Serialize` but NOT `Deserialize`, see the source).
#[test]
fn unified_video_row_pk_is_i64_bigserial() {
    let row = UnifiedVideoRow {
        id: (i32::MAX as i64) + 1, // exceeds i32 — proves i64
        title: "Test".to_string(),
        slug: "test".to_string(),
        description: None,
        video_url: "https://example.com/v.mp4".to_string(),
        video_platform: "bunny".to_string(),
        video_id: None,
        bunny_video_guid: Some("abc-123".to_string()),
        thumbnail_url: None,
        duration: Some(180),
        content_type: "daily_video".to_string(),
        difficulty_level: None,
        category: None,
        video_date: NaiveDate::from_ymd_opt(2026, 5, 20).expect("valid date"),
        is_published: true,
        is_featured: false,
        tags: None,
        views_count: 42,
        trader_id: Some(7_i64),
        bunny_library_id: Some(123_i64),
        created_at: NaiveDateTime::default(),
    };
    let _: i64 = row.id;
    let _: Option<i64> = row.trader_id;
    let _: Option<i64> = row.bunny_library_id;

    let wire = serde_json::to_value(&row).expect("serialize UnifiedVideoRow");
    assert_eq!(
        wire["id"].as_i64(),
        Some((i32::MAX as i64) + 1),
        "id MUST round-trip as i64 — unified_videos.id is bigint (schema.sql:10248)"
    );
    assert_eq!(wire["bunny_library_id"].as_i64(), Some(123));
    assert_eq!(wire["trader_id"].as_i64(), Some(7));

    // Compile-pin: duration is i32 (seconds — Reserved exception,
    // 2.1B seconds is ~68 years per video, unreachable).
    let _: Option<i32> = row.duration;
    // Compile-pin: views_count is i32 (Reserved exception, see test #5).
    let _: i32 = row.views_count;
}

// ── 2. VideoListQuery — fully optional with negative i64 accepted ───

/// The public `/api/videos` endpoint must work on first page load
/// with NO query params (the homepage thumbnail strip hits this).
/// A regression that flipped any field to required would 422 the
/// landing page. The handler clamps `per_page` to `[1, 100]` and
/// `page` to `max(1)`, so the DTO accepts any i64 and the handler
/// does the clamping.
///
/// `room_id: Option<i64>` (rooms.id is BIGSERIAL).
#[test]
fn video_list_query_fully_optional() {
    let empty: VideoListQuery =
        serde_json::from_str("{}").expect("empty VideoListQuery MUST parse (homepage default)");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.content_type.is_none());
    assert!(empty.room_id.is_none());
    assert!(empty.tags.is_none());
    assert!(empty.difficulty_level.is_none());
    assert!(empty.search.is_none());

    // Full params with i64 room_id past i32::MAX
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let full: VideoListQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 50,
        "content_type": "weekly_watchlist",
        "room_id": above_i32,
        "tags": "swing,momentum",
        "difficulty_level": "intermediate",
        "search": "NVDA breakout",
    }))
    .expect("full VideoListQuery MUST parse");
    assert_eq!(full.page, Some(2));
    assert_eq!(full.per_page, Some(50));
    assert_eq!(full.content_type.as_deref(), Some("weekly_watchlist"));
    assert_eq!(
        full.room_id,
        Some(above_i32),
        "room_id MUST be i64 — trading_rooms.id is BIGSERIAL"
    );
    assert_eq!(full.tags.as_deref(), Some("swing,momentum"));
    assert_eq!(full.difficulty_level.as_deref(), Some("intermediate"));
    assert_eq!(full.search.as_deref(), Some("NVDA breakout"));

    // R9-D NEGATIVE: garbage in `per_page` MUST fail
    assert!(
        serde_json::from_value::<VideoListQuery>(serde_json::json!({
            "per_page": "twenty",
        }))
        .is_err(),
        "VideoListQuery.per_page MUST reject string input"
    );

    // R9-D NEGATIVE: garbage in `room_id` MUST fail
    assert!(
        serde_json::from_value::<VideoListQuery>(serde_json::json!({
            "room_id": "abc",
        }))
        .is_err(),
        "VideoListQuery.room_id MUST reject string input (i64 only)"
    );
}

// ── 3. TrackEventRequest — event_type required, timestamp Optional ──

/// `TrackEventRequest` is the body of `POST /:id/track`. The video
/// player emits `play` / `pause` / `complete` events without a
/// timestamp, and `seek` events WITH `timestamp_seconds`. Only
/// `event_type` is required.
///
/// `timestamp_seconds: Option<i32>` — per CLAUDE.md "Reserved
/// exception" for bounded counters. Video duration in seconds caps
/// far below i32::MAX (`i32::MAX` ≈ 2.1B seconds ≈ 68 years), so
/// i32 is fine for an offset into a single video. MONEY would
/// never qualify.
#[test]
fn track_event_request_event_type_required_timestamp_optional() {
    // Minimal — only event_type
    let minimal: TrackEventRequest = serde_json::from_value(serde_json::json!({
        "event_type": "play",
    }))
    .expect("TrackEventRequest with only event_type MUST parse");
    assert_eq!(minimal.event_type, "play");
    assert!(minimal.timestamp_seconds.is_none());

    // With timestamp (seek event)
    let with_ts: TrackEventRequest = serde_json::from_value(serde_json::json!({
        "event_type": "seek",
        "timestamp_seconds": 3600,
    }))
    .expect("TrackEventRequest with timestamp_seconds MUST parse");
    assert_eq!(with_ts.event_type, "seek");
    assert_eq!(with_ts.timestamp_seconds, Some(3600_i32));

    // R9-D NEGATIVE: missing event_type MUST fail (the ONE required)
    assert!(
        serde_json::from_value::<TrackEventRequest>(serde_json::json!({
            "timestamp_seconds": 30,
        }))
        .is_err(),
        "TrackEventRequest without event_type MUST fail (required string)"
    );

    // R9-D NEGATIVE: timestamp_seconds > i32::MAX MUST fail
    // (the Rust type is i32 — Reserved exception, 68 years per video)
    assert!(
        serde_json::from_value::<TrackEventRequest>(serde_json::json!({
            "event_type": "seek",
            "timestamp_seconds": (i32::MAX as i64) + 1,
        }))
        .is_err(),
        "timestamp_seconds > i32::MAX MUST fail — i32 is Reserved exception (per-video seconds bounded)"
    );

    // R9-D NEGATIVE: non-string event_type MUST fail
    assert!(
        serde_json::from_value::<TrackEventRequest>(serde_json::json!({
            "event_type": 42,
        }))
        .is_err(),
        "TrackEventRequest.event_type MUST reject non-string"
    );
}

// ── 4. WatchHistoryQuery — both Optional i64, handler defaults ─────

/// `WatchHistoryQuery` powers `GET /history`. The handler defaults
/// `user_id` to 0 (returns no rows) and `limit` to 20, capping at
/// 50 via `.min(50)`. The DTO is fully Optional so the empty
/// query-string case parses.
///
/// `user_id: Option<i64>` — users.id is BIGSERIAL. Pin i64.
#[test]
fn watch_history_query_optional_i64_fields() {
    let empty: WatchHistoryQuery =
        serde_json::from_str("{}").expect("empty WatchHistoryQuery MUST parse (handler defaults)");
    assert!(empty.user_id.is_none());
    assert!(empty.limit.is_none());

    let above_i32: i64 = (i32::MAX as i64) + 1;
    let full: WatchHistoryQuery = serde_json::from_value(serde_json::json!({
        "user_id": above_i32,
        "limit": 50,
    }))
    .expect("WatchHistoryQuery with values MUST parse");
    assert_eq!(
        full.user_id,
        Some(above_i32),
        "user_id MUST be i64 — users.id is BIGSERIAL"
    );
    assert_eq!(full.limit, Some(50_i64));

    // R9-D NEGATIVE: limit non-integer MUST fail
    assert!(
        serde_json::from_value::<WatchHistoryQuery>(serde_json::json!({
            "limit": "fifty",
        }))
        .is_err(),
        "WatchHistoryQuery.limit MUST reject string input"
    );

    // R9-D NEGATIVE: user_id non-integer MUST fail
    assert!(
        serde_json::from_value::<WatchHistoryQuery>(serde_json::json!({
            "user_id": 3.14,
        }))
        .is_err(),
        "WatchHistoryQuery.user_id MUST reject float input (i64 only)"
    );
}

// ── 5. VideoResponse — id i64, views_count i32 (Reserved exception) ─

/// `VideoResponse` is the wire DTO the frontend receives. The PK
/// `id` is i64 (BIGSERIAL), but `views_count` is i32 — per CLAUDE.md
/// "Reserved exception", per-video view counters are bounded by
/// what a single video can plausibly accumulate (the top-1 YouTube
/// video has ~16B views over a decade, but the realistic ceiling
/// for a single trading-room video is in the millions). MONEY would
/// never qualify.
///
/// `PaginationMeta.total` is i64 (COUNT(*) across all videos —
/// unbounded by user activity).
#[test]
fn video_response_id_i64_views_i32_reserved_exception() {
    let resp = VideoResponse {
        id: (i32::MAX as i64) + 1,
        title: "Daily Watch".to_string(),
        slug: "daily-watch".to_string(),
        description: None,
        video_url: "https://example.com/v.mp4".to_string(),
        embed_url: "https://iframe.mediadelivery.net/embed/1/abc".to_string(),
        video_platform: "bunny".to_string(),
        thumbnail_url: None,
        duration: Some(180),
        formatted_duration: "3:00".to_string(),
        content_type: "daily_video".to_string(),
        video_date: "2026-05-20".to_string(),
        formatted_date: "May 20, 2026".to_string(),
        is_published: true,
        is_featured: false,
        tags: vec![],
        tag_details: vec![],
        views_count: i32::MAX, // pin: max i32 — at the ceiling
        trader: Some(TraderInfo {
            id: (i32::MAX as i64) + 1,
            name: "T".to_string(),
            slug: "t".to_string(),
        }),
        rooms: vec![RoomInfo {
            id: (i32::MAX as i64) + 1,
            name: "R".to_string(),
            slug: "r".to_string(),
        }],
        created_at: "2026-05-20T00:00:00".to_string(),
    };
    let _: i64 = resp.id;
    let _: i32 = resp.views_count;
    let _: i64 = resp.trader.as_ref().expect("trader present").id;
    let _: i64 = resp.rooms[0].id;

    let wire = serde_json::to_value(&resp).expect("serialize VideoResponse");
    assert_eq!(wire["id"].as_i64(), Some((i32::MAX as i64) + 1));
    assert_eq!(
        wire["views_count"].as_i64(),
        Some(i32::MAX as i64),
        "views_count MUST round-trip as i32 (Reserved exception — per-video counter bounded)"
    );

    // PaginationMeta — total is i64 (cross-video COUNT(*) unbounded)
    let meta = PaginationMeta {
        current_page: 1,
        per_page: 20,
        total: (i32::MAX as i64) + 100,
        last_page: 1,
    };
    let _: i64 = meta.total;
    let _: i64 = meta.current_page;
    let _: i64 = meta.per_page;
    let _: i64 = meta.last_page;
    let meta_wire = serde_json::to_value(&meta).expect("serialize PaginationMeta");
    assert_eq!(
        meta_wire["total"].as_i64(),
        Some((i32::MAX as i64) + 100),
        "PaginationMeta.total MUST be i64 — aggregates across all videos"
    );

    // TagDetail — slug/name/color pure strings
    let tag = TagDetail {
        slug: "swing".to_string(),
        name: "Swing Trading".to_string(),
        color: "#3b82f6".to_string(),
    };
    assert_eq!(tag.slug, "swing");
    let tag_wire = serde_json::to_value(&tag).expect("serialize TagDetail");
    assert_eq!(tag_wire["color"].as_str(), Some("#3b82f6"));
}

// ── 6. Router mount-table compile-pin (6 routes) ────────────────────

/// `routes::videos::router()` MUST build as `Router<AppState>`.
/// Mount table:
///   - GET  /                 (list_videos)         — public
///   - GET  /weekly           (get_weekly_videos)   — public
///   - GET  /history          (get_watch_history)   — public + user_id query
///   - GET  /:id_or_slug      (get_video)           — Path<String>
///   - GET  /:id/related      (get_related_videos)  — Path<i64>
///   - POST /:id/track        (track_event)         — Path<i64>
///
/// The `Path<String>` on `:id_or_slug` is LOAD-BEARING — the handler
/// at routes/videos.rs:351 tries `id_or_slug.parse::<i64>()` first
/// and falls back to a slug-keyed query. A regression to `Path<i64>`
/// would 400 every slug URL the frontend emits
/// (`/videos/nvda-breakout`).
///
/// The `Path<i64>` on `:id/related` and `:id/track` is similarly
/// load-bearing — the handlers bind directly to BIGSERIAL i64 PK
/// queries against `unified_videos.id`. A regression to `Path<i32>`
/// would silently break tracking on any video past row 2^31.
#[test]
fn videos_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::videos::router();
}

/// Idempotent construction. Per CLAUDE.md habit #3: pin that
/// nothing global (no `OnceLock`, no `static mut`) lives inside
/// the router constructor — a refactor that hoisted state into
/// a global would silently fail this test the second time
/// `router()` runs.
#[test]
fn videos_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::videos::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::videos::router();
}
