// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Admin videos route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_videos::*` and
//! pins:
//!   - the dual-router mount shape (`router()` + `analytics_router()`)
//!   - the single public DTO (`UnifiedVideoRow`)
//!
//! `routes/admin_videos/` is 2,032 LOC across 6 sub-modules
//! (`helpers`, `crud`, `analytics`, `series_chapters`, `operations`,
//! `bunny_embed`) post-split (R7-B, 2026-05-20). Every handler runs
//! live SQL against `unified_videos` (BIGSERIAL i64 PK per schema
//! .sql:10247-10248) plus `video_chapters`, `video_series`,
//! `video_watch_progress`, `video_transcripts`. We can't invoke them
//! in isolation. What we CAN pin:
//!
//! ## Pins
//!
//! 1. **Dual-router compile-pin** — `router()` (7 routes mounted at
//!    `/admin/videos`) and `analytics_router()` (~30 routes mounted
//!    at `/video-advanced`). A regression in ANY sub-module's
//!    handler signature would fail compilation. Both routers MUST
//!    be `Router<AppState>` and idempotent (per CLAUDE.md habit
//!    #3: no module-cached state surviving the refactor).
//!
//! 2. **`UnifiedVideoRow` is BIGSERIAL i64 PK + i64 FK alignment.**
//!    Per schema.sql:10247-10272:
//!      - id                bigint NOT NULL      → i64
//!      - bunny_library_id  bigint               → Option<i64>
//!      - trader_id         bigint (FK→users)    → Option<i64>
//!      - room_id           bigint (FK→trading_rooms) → NOT in this
//!                                                       struct
//!                                                       directly
//!      - created_by/updated_by bigint           → Option<i64>
//!
//!    Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
//!    TIME": although these are FK IDs, they index into BIGSERIAL
//!    tables. A regression to i32 would silently truncate any FK
//!    past 2.1B.
//!
//! 3. **Counter fields are i32 — Reserved exception with rationale.**
//!    `UnifiedVideoRow.views_count / likes_count / completion_rate
//!    / duration: Option<i32>` are all i32. Per CLAUDE.md
//!    "Reserved exception: row counts (revisions: i32, attempts:
//!    i32, total_pages: i32) — those genuinely cap below 2 billion
//!    and i32 is fine. Money never qualifies for the exception":
//!      - views_count per video ≤ 2.1B (even the highest-traffic
//!        YouTube videos are ~10B views, but THIS is a B2B trading-
//!        platform: per-video views cap at ~100K for a flagship
//!        recap)
//!      - likes_count: same scale, ~10K max
//!      - completion_rate: 0-100 by handler convention
//!      - duration: seconds, capped at ~24h × 3600 = 86400 per video
//!    None of these qualify as money. The i32 choice is the correct
//!    Reserved exception.
//!
//! 4. **`video_date: NaiveDate` (NOT String).** The DTO accepts a
//!    typed date — a regression to `String` would let malformed
//!    dates (e.g., "tomorrow") flow into SQL where they'd 500 on
//!    binding. Pin the typed contract.
//!
//! 5. **Soft-delete via `deleted_at: Option<NaiveDateTime>`.** Per
//!    schema.sql:10270 the column exists; soft-delete is the only
//!    safe deletion path because `unified_videos` is referenced by
//!    `video_room_assignments`, `video_chapters`, `video_series`,
//!    `video_watch_progress`, `video_transcripts` — a hard delete
//!    would cascade and lose user watch-progress data.
//!
//! ## Pattern source
//!
//! Modeled on `tests/videos_test.rs`, `tests/admin_orders_test.rs`,
//! `tests/courses_admin_test.rs`, `tests/admin_page_layouts_test.rs`.

use revolution_api::routes::admin_videos;

// ── 1. Dual-router compile-pin (router + analytics_router) ───────────

/// `admin_videos::router()` MUST build as `Router<AppState>`.
/// Mounted at `/admin/videos` per routes/mod.rs:144. Mount table at
/// mod.rs:84-98 — 7 routes, ALL `AdminUser`-gated (the crud
/// handlers' first positional extractor is `AdminUser`):
///   - GET    /                  list_videos
///   - POST   /                  create_video
///   - GET    /:id               get_video
///   - PUT    /:id               update_video
///   - DELETE /:id               delete_video
///   - GET    /stats             get_stats
///   - GET    /options           get_options
///   - POST   /bulk/publish      bulk_publish
///   - POST   /bulk/delete       bulk_delete (load-bearing — admin-
///                                            only; a regression that
///                                            dropped AdminUser would
///                                            let anonymous callers
///                                            delete every video)
///   - POST   /bulk/assign       bulk_assign
///
/// A regression in any handler signature fails the compile here.
#[test]
fn admin_videos_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = admin_videos::router();
}

/// `admin_videos::analytics_router()` MUST build as `Router<AppState>`.
/// Mounted at `/video-advanced` per routes/mod.rs:146. ~30-route
/// surface covering analytics, series, chapters, scheduled jobs,
/// bulk upload, CDN purge, Bunny webhooks. ALL `AdminUser`-gated
/// except `/bunny/webhook` (which uses a webhook-signature middleware
/// — NOT an unauthenticated path).
///
/// Per CLAUDE.md habit #3 ("does any `static` / `OnceLock` / lazy
/// init survive the refactor?"), the analytics router MUST be safe
/// to construct multiple times. The Bunny webhook handler verifies
/// signatures via shared HMAC state pulled from `state.config`, NOT
/// from a module-cached `OnceLock` — pin documents this invariant.
#[test]
fn admin_videos_analytics_router_builds_and_idempotent() {
    let _r1: axum::Router<revolution_api::AppState> = admin_videos::analytics_router();
    let _r2: axum::Router<revolution_api::AppState> = admin_videos::analytics_router();

    // Both routers can co-exist — they nest at different prefixes
    // (`/admin/videos` and `/video-advanced`) so no route collision.
    let _crud: axum::Router<revolution_api::AppState> = admin_videos::router();
    let _analytics: axum::Router<revolution_api::AppState> = admin_videos::analytics_router();
}

// ── 2. UnifiedVideoRow: BIGSERIAL i64 PK + i64 FK alignment ──────────

/// `UnifiedVideoRow` MUST declare `id: i64` and FK columns as
/// `Option<i64>` to match BIGSERIAL schema. Per schema.sql:10247-10272:
///   - id                bigint NOT NULL  → i64
///   - bunny_library_id  bigint           → Option<i64>
///   - trader_id         bigint           → Option<i64>
///   - created_by        bigint           → Option<i64>
///   - updated_by        bigint           → Option<i64>
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
/// although these are FK IDs (not money), they index into BIGSERIAL
/// tables. A regression to i32 would silently truncate any FK past
/// i32::MAX (2.1B).
///
/// `views_count: i32` IS a Reserved exception per CLAUDE.md ("row
/// counts ... genuinely cap below 2 billion") — per-video views on
/// a B2B trading platform cap at ~100K for a flagship recap; even
/// the highest-traffic YouTube videos are ~10B views which exceeds
/// i32::MAX, BUT this is NOT YouTube and the analytics dashboard
/// rolls into bigger counters at the room/category level (which are
/// computed views, not stored).
///
/// `duration: Option<i32>` — Reserved exception (seconds; capped
/// at ~24h × 3600 = 86400 per video by the upload UI).
///
/// `video_date: NaiveDate` — typed date. R9-D NEGATIVE: a regression
/// to `String` would let malformed dates flow to SQL where they'd
/// 500 on binding.
///
/// `deleted_at: Option<NaiveDateTime>` — soft-delete contract;
/// hard-deleting would cascade to `video_watch_progress` and lose
/// user data.
#[test]
fn unified_video_row_bigserial_and_reserved_i32_counters() {
    use revolution_api::routes::admin_videos::UnifiedVideoRow;

    let above_i32: i64 = (i32::MAX as i64) + 1;

    let row = UnifiedVideoRow {
        id: above_i32,
        title: "ES weekly recap — 2026-05-20".to_string(),
        slug: "es-weekly-2026-05-20".to_string(),
        description: None,
        video_url: "https://video.bunnycdn.com/.../play.mp4".to_string(),
        video_platform: "bunny".to_string(),
        video_id: Some("vid_abc123".to_string()),
        bunny_video_guid: Some("guid-abc".to_string()),
        thumbnail_url: Some("https://cdn.example/thumb.jpg".to_string()),
        thumbnail_path: None,
        // Reserved exception per CLAUDE.md: seconds capped at ~86400 (24h).
        duration: Some(7200),
        quality: Some("4k".to_string()),
        content_type: "daily_video".to_string(),
        difficulty_level: Some("advanced".to_string()),
        category: Some("recap".to_string()),
        session_type: Some("post_close".to_string()),
        chapter_timestamps: None,
        trader_id: Some(above_i32),
        video_date: chrono::NaiveDate::from_ymd_opt(2026, 5, 20).unwrap(),
        is_published: true,
        is_featured: false,
        published_at: None,
        scheduled_at: None,
        tags: None,
        // Reserved exception: per-video ≤ 2B views on a B2B platform.
        views_count: 0,
        likes_count: 0,
        completion_rate: 0,
        bunny_library_id: Some(above_i32),
        bunny_encoding_status: Some("ready".to_string()),
        bunny_thumbnail_url: None,
        metadata: None,
        created_by: Some(above_i32),
        updated_by: Some(above_i32),
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
        // Soft-delete contract — hard delete would cascade to
        // video_watch_progress and lose user data.
        deleted_at: None,
    };

    let wire = serde_json::to_value(&row).expect("serialize UnifiedVideoRow");
    assert_eq!(wire["id"].as_i64(), Some(above_i32));
    assert!(
        wire["id"].as_i64().unwrap() > i32::MAX as i64,
        "id MUST round-trip past i32::MAX"
    );
    assert_eq!(wire["bunny_library_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["trader_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["created_by"].as_i64(), Some(above_i32));
    assert_eq!(wire["updated_by"].as_i64(), Some(above_i32));
    assert_eq!(wire["views_count"].as_i64(), Some(0));
    assert_eq!(wire["duration"].as_i64(), Some(7200));
}

// ── 3. UnifiedVideoRow deserialize: typed video_date enforced ────────

/// `UnifiedVideoRow.video_date: NaiveDate` MUST stay typed. A
/// regression to `String` would let malformed dates flow through
/// serde to SQL where they'd 500 on binding (sqlx will reject
/// non-ISO-8601 dates at the parameter binding step, but the
/// error surfaces only at runtime). The typed field rejects
/// invalid dates at serde time — a much cleaner failure mode.
///
/// R9-D NEGATIVE: serializing a deliberately malformed date string
/// MUST fail to deserialize as `UnifiedVideoRow`.
#[test]
fn unified_video_row_video_date_is_typed_naive_date() {
    use revolution_api::routes::admin_videos::UnifiedVideoRow;

    // Valid ISO-8601 date deserializes.
    let row: UnifiedVideoRow = serde_json::from_value(serde_json::json!({
        "id": 1i64,
        "title": "x",
        "slug": "x",
        "video_url": "https://...",
        "video_platform": "bunny",
        "content_type": "daily_video",
        "video_date": "2026-05-20",
        "is_published": true,
        "is_featured": false,
        "views_count": 0,
        "likes_count": 0,
        "completion_rate": 0,
        "created_at": "2026-05-20T12:00:00",
        "updated_at": "2026-05-20T12:00:00",
    }))
    .expect("valid UnifiedVideoRow must deserialize");
    assert_eq!(
        row.video_date,
        chrono::NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
    );

    // R9-D NEGATIVE: malformed date MUST fail at the DTO boundary
    // (NOT silently degrade to None / default — the field is required).
    let err = serde_json::from_value::<UnifiedVideoRow>(serde_json::json!({
        "id": 1i64,
        "title": "x",
        "slug": "x",
        "video_url": "https://...",
        "video_platform": "bunny",
        "content_type": "daily_video",
        "video_date": "tomorrow",  // ← malformed
        "is_published": true,
        "is_featured": false,
        "views_count": 0,
        "likes_count": 0,
        "completion_rate": 0,
        "created_at": "2026-05-20T12:00:00",
        "updated_at": "2026-05-20T12:00:00",
    }));
    assert!(
        err.is_err(),
        "malformed video_date MUST fail at serde — NaiveDate enforced"
    );
}

// ── 4. UnifiedVideoRow soft-delete contract (deleted_at Optional) ────

/// `UnifiedVideoRow.deleted_at: Option<NaiveDateTime>` IS the soft-
/// delete sentinel. Per schema.sql:10270, the column exists; the
/// admin `delete_video` handler (referenced in mod.rs:91) uses
/// `UPDATE unified_videos SET deleted_at = now() WHERE id = $1`
/// instead of `DELETE FROM unified_videos`.
///
/// A regression that removed `deleted_at` from the struct would
/// either:
///   (a) Fail sqlx FromRow decoding on `SELECT * FROM unified_videos`
///       (caught at runtime, not compile)
///   (b) Force the admin UI to hard-delete, cascading to
///       `video_watch_progress` and losing every member's watch
///       progress history.
///
/// The compile-pin proves `deleted_at` is Optional (NULL = active
/// row, NOT-NULL = soft-deleted).
#[test]
fn unified_video_row_soft_delete_deleted_at_optional() {
    use revolution_api::routes::admin_videos::UnifiedVideoRow;

    // Active row — deleted_at is None.
    let active: UnifiedVideoRow = serde_json::from_value(serde_json::json!({
        "id": 1i64,
        "title": "x",
        "slug": "x",
        "video_url": "https://...",
        "video_platform": "bunny",
        "content_type": "daily_video",
        "video_date": "2026-05-20",
        "is_published": true,
        "is_featured": false,
        "views_count": 0,
        "likes_count": 0,
        "completion_rate": 0,
        "created_at": "2026-05-20T12:00:00",
        "updated_at": "2026-05-20T12:00:00",
        // deleted_at omitted — Optional defaults to None
    }))
    .expect("UnifiedVideoRow without deleted_at must deserialize");
    assert!(
        active.deleted_at.is_none(),
        "active rows MUST have deleted_at = None"
    );

    // Soft-deleted row — deleted_at is Some(NaiveDateTime).
    let soft_deleted: UnifiedVideoRow = serde_json::from_value(serde_json::json!({
        "id": 1i64,
        "title": "x",
        "slug": "x",
        "video_url": "https://...",
        "video_platform": "bunny",
        "content_type": "daily_video",
        "video_date": "2026-05-20",
        "is_published": true,
        "is_featured": false,
        "views_count": 0,
        "likes_count": 0,
        "completion_rate": 0,
        "created_at": "2026-05-20T12:00:00",
        "updated_at": "2026-05-20T12:00:00",
        "deleted_at": "2026-05-21T09:30:00",
    }))
    .expect("soft-deleted UnifiedVideoRow must deserialize");
    assert!(
        soft_deleted.deleted_at.is_some(),
        "soft-deleted rows MUST have deleted_at = Some(_)"
    );
    assert_eq!(
        soft_deleted.deleted_at.unwrap(),
        chrono::NaiveDateTime::parse_from_str("2026-05-21T09:30:00", "%Y-%m-%dT%H:%M:%S").unwrap()
    );
}
