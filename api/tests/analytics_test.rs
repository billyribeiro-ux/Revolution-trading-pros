//! Analytics route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::analytics` and pins the
//! public DTOs + the `router()` mount table for the analytics surface
//! at `/analytics/*`.
//!
//! ## Why this shape
//!
//! Every handler in `routes/analytics.rs` either touches Postgres
//! (`analytics_events` INSERTs, `AnalyticsService` SELECTs) or
//! requires the `AdminUser` extractor (which hits Redis for session
//! lookup). None can be invoked in unit-test isolation without a DB
//! fixture. What we CAN pin without a DB:
//!
//! 1. **Router compile-pin** — `analytics::router()` must build as
//!    `Router<AppState>`. The documented mount table (per
//!    `routes/analytics.rs:577-592`) is 9 endpoints:
//!      - POST /track            (public — sendBeacon ingest)
//!      - POST /reading          (public — reading-time beacon)
//!      - POST /performance      (public — web-vitals beacon)
//!      - GET  /overview         (admin — dashboard tile)
//!      - GET  /realtime         (admin — live counters)
//!      - GET  /room/:room_slug                       (admin)
//!      - GET  /room/:room_slug/equity-curve          (admin)
//!      - GET  /room/:room_slug/drawdowns             (admin)
//!      - GET  /room/:room_slug/ticker/:ticker        (admin)
//!    A regression in any handler signature (wrong `State<>`,
//!    missing `AdminUser`, wrong return-type) fails compilation here.
//!
//! 2. **`TrackRequest` deserialization** — the public ingest DTO.
//!    `event_type` and `event_name` are required; the other three
//!    (`page_url`, `referrer`, `properties`) are optional. R9-D
//!    NEGATIVE pin: a refactor that flipped `event_type` to Optional
//!    would let the public ingest endpoint accept `{}` and insert a
//!    row with NULL event_type, which downstream dashboards group
//!    by — silently breaking every count.
//!
//! 3. **`ReadingTrackRequest` accepts both snake_case AND camelCase**
//!    for `post_id`. The JS client (`readingAnalytics.ts`) ships
//!    `postId` via `navigator.sendBeacon`; the legacy server-side
//!    consumer expects `post_id`. The `#[serde(alias = "post_id")]`
//!    on `post_id` + `#[serde(rename_all = "camelCase")]` at the
//!    struct level is load-bearing — a regression that dropped the
//!    alias would silently drop every "reading completed" event
//!    from the marketing-site beacons.
//!
//! 4. **`FlexibleId` accepts both number AND string** for `post_id`.
//!    The client's TS type is `string | number`; the production
//!    payload is overwhelmingly numeric, but a small fraction of
//!    older clients ship strings. A refactor that hardened to
//!    number-only would silently drop those events with a 400.
//!
//! 5. **`AnalyticsQuery` filters are all Optional** — a bare
//!    `GET /analytics/overview` (no query string) must NOT 400.
//!    The admin dashboard sends no params for the default 30-day
//!    window. NEGATIVE pin.
//!
//! 6. **`CmsEvent` IDs in `RoomAnalyticsQuery` route paths are `i64`
//!    surface** — `analytics_events.id` is `BIGSERIAL` in the DB,
//!    matching the i64 contract from CLAUDE.md money rule (i64 also
//!    applies to identity columns by the "wrong-row-on-narrow" rule;
//!    a narrowing to i32 would wrap above 2,147,483,647 rows and
//!    silently overwrite older events — analytics ingest is high-
//!    volume, this is reachable in months not years).
//!
//! ## Money contract
//!
//! `routes/analytics.rs` exposes NO monetary DTOs publicly — the
//! room-analytics handlers return `serde_json::Value` from
//! `AnalyticsService` (which is tested separately in
//! `tests/utils_test.rs` if at all). The CLAUDE.md "Reserved
//! exception" (revisions/attempts as i32) does NOT apply here:
//! analytics event IDs are `BIGSERIAL` (i64) because the table is
//! high-write — see `migrations/*_analytics_events.sql`. Pin the
//! type-shape on the public DTOs and the absence of a narrowing.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_delivery_test.rs`, `tests/auth_test.rs`,
//! `tests/payments_test.rs`.

use revolution_api::routes::analytics;
use revolution_api::routes::analytics::{
    AnalyticsQuery, FlexibleId, ReadingTrackRequest, RoomAnalyticsQuery, TrackRequest,
};

/// Test helper — `FlexibleId::as_i64` is private to the route
/// module (the handler uses it internally to coerce the
/// number-or-string payload). Replicate the same coercion here so
/// the test asserts the SAME conversion the handler will perform.
/// If the route module changes the coercion rule (e.g. trims
/// whitespace before parsing), this helper must mirror it.
fn flexible_id_as_i64(f: &FlexibleId) -> Option<i64> {
    match f {
        FlexibleId::Number(n) => Some(*n),
        FlexibleId::Stringy(s) => s.parse().ok(),
    }
}

// ── 1. router() mount-table compile-pin ──────────────────────────────

/// `analytics::router()` must build as `Router<AppState>`. The
/// documented mount table is 9 endpoints (3 public ingest, 1 admin
/// overview, 1 admin realtime, 4 admin room-analytics). A regression
/// in any handler signature fails compilation here.
#[test]
fn analytics_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = analytics::router();
}

// ── 2. TrackRequest — required vs optional fields (NEGATIVE pin) ────

/// `TrackRequest` requires `event_type` and `event_name`. The other
/// three fields (`page_url`, `referrer`, `properties`) are optional.
///
/// R9-D NEGATIVE pin: a refactor that flipped `event_type` to
/// Optional would let `POST /analytics/track` accept `{}` and INSERT
/// a row with NULL event_type. Every analytics dashboard groups by
/// event_type — a NULL group would silently appear and skew counts.
#[test]
fn track_request_required_event_type_and_event_name_negative_pin() {
    let valid: TrackRequest = serde_json::from_value(serde_json::json!({
        "event_type": "pageview",
        "event_name": "post_view",
    }))
    .expect("TrackRequest with required fields must parse");
    assert_eq!(valid.event_type, "pageview");
    assert_eq!(valid.event_name, "post_view");
    assert!(valid.page_url.is_none());
    assert!(valid.referrer.is_none());
    assert!(valid.properties.is_none());

    // Optional fields still work
    let full: TrackRequest = serde_json::from_value(serde_json::json!({
        "event_type": "interaction",
        "event_name": "click",
        "page_url": "/posts/welcome",
        "referrer": "https://google.com",
        "properties": { "target": "cta_button" },
    }))
    .expect("TrackRequest with all fields must parse");
    assert_eq!(full.page_url.as_deref(), Some("/posts/welcome"));

    // NEGATIVE: missing event_type must fail
    let no_type = serde_json::from_value::<TrackRequest>(serde_json::json!({
        "event_name": "ghost",
    }));
    assert!(
        no_type.is_err(),
        "TrackRequest without event_type must fail — NULL group would skew dashboards"
    );

    // NEGATIVE: missing event_name must fail
    let no_name = serde_json::from_value::<TrackRequest>(serde_json::json!({
        "event_type": "pageview",
    }));
    assert!(
        no_name.is_err(),
        "TrackRequest without event_name must fail — event_name is the grouping key"
    );

    // NEGATIVE: empty body must fail
    let empty = serde_json::from_value::<TrackRequest>(serde_json::json!({}));
    assert!(
        empty.is_err(),
        "TrackRequest empty body must fail — the public ingest endpoint must reject pingbacks"
    );
}

// ── 3. ReadingTrackRequest — snake_case + camelCase alias ───────────

/// `ReadingTrackRequest` accepts both snake_case AND camelCase for
/// `post_id`. The JS client (`readingAnalytics.ts`) ships `postId`
/// via `sendBeacon`. The `#[serde(alias = "post_id")]` + struct-level
/// `#[serde(rename_all = "camelCase")]` is load-bearing — a refactor
/// that dropped the alias would silently drop every "reading
/// completed" event from the marketing site.
#[test]
fn reading_track_request_accepts_camelcase_and_snake_case() {
    // camelCase (the JS client's wire format)
    let camel: ReadingTrackRequest = serde_json::from_value(serde_json::json!({
        "postId": 12345i64,
        "scrollDepth": 87,
        "timeOnPage": 240,
        "event": "reading_completion",
    }))
    .expect("ReadingTrackRequest must accept camelCase from sendBeacon");
    assert_eq!(flexible_id_as_i64(&camel.post_id), Some(12345));
    assert_eq!(camel.scroll_depth, Some(87));
    assert_eq!(camel.time_on_page, Some(240));
    // `event` flows through `extras` (flatten)
    assert!(camel.extras.contains_key("event"));

    // snake_case alias path
    let snake: ReadingTrackRequest = serde_json::from_value(serde_json::json!({
        "post_id": 999i64,
        "scroll_depth": 50,
        "time_on_page": 120,
    }))
    .expect("ReadingTrackRequest must accept snake_case via alias");
    assert_eq!(flexible_id_as_i64(&snake.post_id), Some(999));
    assert_eq!(snake.scroll_depth, Some(50));

    // NEGATIVE: no post_id / postId — must fail (the only required field)
    let no_id = serde_json::from_value::<ReadingTrackRequest>(serde_json::json!({
        "scrollDepth": 50,
    }));
    assert!(
        no_id.is_err(),
        "ReadingTrackRequest without post_id/postId must fail — the row has no post to associate with"
    );
}

// ── 4. FlexibleId — accepts both number and string for post_id ──────

/// `FlexibleId` is `#[serde(untagged)]` accepting either i64 OR
/// String. The client's TS type is `string | number`; production
/// payloads are overwhelmingly numeric, but a small fraction of
/// older clients ship strings. A refactor that hardened to
/// number-only would silently drop those events with a 400 BAD_REQUEST.
#[test]
fn flexible_id_accepts_both_number_and_string_variants() {
    // Number variant
    let req_num: ReadingTrackRequest =
        serde_json::from_value(serde_json::json!({ "postId": 42i64 }))
            .expect("FlexibleId::Number must parse from JSON number");
    assert_eq!(flexible_id_as_i64(&req_num.post_id), Some(42));

    // String variant — older clients ship `post_id` as a JSON string
    let req_str: ReadingTrackRequest =
        serde_json::from_value(serde_json::json!({ "postId": "42" }))
            .expect("FlexibleId::Stringy must parse from JSON string");
    assert_eq!(flexible_id_as_i64(&req_str.post_id), Some(42));

    // String variant with unparseable content — as_i64 returns None
    // (the handler then skips inserting post_id into properties; the
    // row is still recorded with the rest of the payload).
    let req_bad: ReadingTrackRequest =
        serde_json::from_value(serde_json::json!({ "postId": "not-a-number" }))
            .expect("FlexibleId::Stringy must parse even if as_i64 returns None");
    assert_eq!(flexible_id_as_i64(&req_bad.post_id), None);

    // i64 above i32::MAX — analytics_events.id is BIGSERIAL; if a
    // refactor narrowed FlexibleId::Number to i32, post_id values
    // for the (future) high-volume post table would silently truncate.
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let req_big: ReadingTrackRequest =
        serde_json::from_value(serde_json::json!({ "postId": above_i32 }))
            .expect("FlexibleId::Number must accept values above i32::MAX");
    assert_eq!(flexible_id_as_i64(&req_big.post_id), Some(above_i32));
    // Compile-pin: the variant holds i64, not i32
    let _: i64 = match &req_big.post_id {
        FlexibleId::Number(n) => *n,
        FlexibleId::Stringy(_) => unreachable!(),
    };
}

// ── 5. AnalyticsQuery + RoomAnalyticsQuery — all fields optional ────

/// `AnalyticsQuery` and `RoomAnalyticsQuery` must accept an empty
/// query string. The admin dashboard hits `GET /analytics/overview`
/// with no params for the default 30-day window. R9-D NEGATIVE pin:
/// a regression that flipped `start_date` to required would 400 the
/// dashboard load.
#[test]
fn analytics_query_all_fields_optional_negative_pin() {
    // Empty body — must parse
    let empty: AnalyticsQuery = serde_json::from_value(serde_json::json!({}))
        .expect("AnalyticsQuery must parse from empty body — the default dashboard hit");
    assert!(empty.start_date.is_none());
    assert!(empty.end_date.is_none());
    assert!(empty.event_type.is_none());

    // With all fields populated
    let full: AnalyticsQuery = serde_json::from_value(serde_json::json!({
        "start_date": "2026-01-01",
        "end_date": "2026-04-30",
        "event_type": "pageview",
    }))
    .expect("AnalyticsQuery with all fields must parse");
    assert_eq!(full.event_type.as_deref(), Some("pageview"));

    // Same for RoomAnalyticsQuery — both `from` and `to` optional
    let room_empty: RoomAnalyticsQuery = serde_json::from_value(serde_json::json!({}))
        .expect("RoomAnalyticsQuery must parse from empty body");
    assert!(room_empty.from.is_none());
    assert!(room_empty.to.is_none());

    let room_full: RoomAnalyticsQuery = serde_json::from_value(serde_json::json!({
        "from": "2026-04-01",
        "to": "2026-04-30",
    }))
    .expect("RoomAnalyticsQuery with all fields must parse");
    assert_eq!(room_full.from.as_deref(), Some("2026-04-01"));
}

// ── 6. CmsEvent IDs / FlexibleId — i64 BIGSERIAL contract ──────────

/// CLAUDE.md "i64 only, BIGINT only, every time" applies here even
/// though analytics IDs aren't money. The argument from CLAUDE.md
/// is: "narrowing identity columns is the worst-row-on-narrow bug"
/// — analytics_events is high-volume (one row per pageview), so
/// `i32` would overflow at 2.15 billion rows, which is months at
/// production traffic, not decades.
///
/// "Reserved exception" from CLAUDE.md (`revisions`, `attempts` may
/// be i32) does NOT apply here because:
///   - `analytics_events.id` is BIGSERIAL (i64) in the schema.
///   - Pageview counters are not bounded the way "revisions per
///     page" or "attempts per password reset" are.
///   - A narrowed `FlexibleId::Number(i32)` would lose data on a
///     simple cast from the client's number payload.
///
/// This test pins the i64-end-to-end contract explicitly with a
/// value that exceeds i32::MAX, so a future narrowing breaks here.
#[test]
fn analytics_post_id_is_i64_above_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let req: ReadingTrackRequest = serde_json::from_value(serde_json::json!({
        "postId": above_i32_max,
    }))
    .expect("FlexibleId::Number must accept i64 above i32::MAX");

    let id = flexible_id_as_i64(&req.post_id).expect("post_id must parse as i64");
    assert_eq!(id, above_i32_max);

    // Narrowing proof — wrap-on-cast would lose data
    assert!(
        (id as i32 as i64) != id,
        "narrowing post_id to i32 must lose data — proves i64 is required for analytics IDs"
    );

    // Compile-pin: the FlexibleId variant holds i64, not i32 / u32
    let v = FlexibleId::Number(above_i32_max);
    let _: i64 = match v {
        FlexibleId::Number(n) => n,
        FlexibleId::Stringy(_) => unreachable!(),
    };
}
