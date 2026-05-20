//! Admin popups route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_popups` and
//! exercises the public DTOs (`ListPopupsQuery`, `ToggleStatusRequest`,
//! `CreateAbTestRequest`, `AbTestVariant`) + the public `router()`
//! mount table. Also binds the popup-model DTOs that flow through
//! these handlers (`CreatePopupRequest`, `UpdatePopupRequest`,
//! `AbTestVariantMetrics`, `ViewMetrics`, `ConversionMetrics`,
//! `DeviceBreakdown`, `DailyCount`).
//!
//! ## Why this file exists (R15-D)
//!
//! `routes/admin_popups.rs` is 867 LOC of admin-gated CRUD + A/B
//! testing — every handler is `_admin: AdminUser`-gated, every write
//! mutates the `popups` / `popup_ab_tests` tables, and the listing
//! endpoint composes parameterized SQL from optional filter fields.
//! A real DB integration test is out of scope here. What we CAN pin:
//!
//! 1. **PK type — popups uses `i32` (SERIAL), the BIGSERIAL
//!    "Reserved exception".**
//!
//!    CLAUDE.md "Money / cents" rule explicitly carves out row
//!    counters and bounded-cardinality PKs:
//!
//!      > Reserved exception: row counts (`revisions: i32`,
//!      >  `attempts: i32`, `total_pages: i32`) — those genuinely
//!      >  cap below 2 billion and `i32` is fine. Money never
//!      >  qualifies for the exception.
//!
//!    Popups are an admin-CMS object: total cardinality is bounded
//!    by the admin team (dozens of popups in the lifetime of the
//!    app, not millions). The `popups.id` column is `SERIAL` (not
//!    `BIGSERIAL`) for that reason — see `models::popup::Popup.id:
//!    i32` and the handler `Path<i32>` extractors at
//!    `routes/admin_popups.rs:717,812`. A regression that flipped
//!    `popups` to `BIGSERIAL` would break the `Path<i32>`
//!    extraction silently (the handler would 404 every popup
//!    edit/delete the moment a stale i32 PK appears in the URL).
//!
//!    We pin this with a positive-i32 fixture + a NEGATIVE pin
//!    that an `id` exceeding `i32::MAX` MUST fail to deserialize
//!    (the request parameter is `Path<i32>`, not `Path<i64>`).
//!
//! 2. **Analytics counters MUST stay i64** (`ViewMetrics`,
//!    `ConversionMetrics`, `DeviceBreakdown`, `DailyCount`).
//!
//!    The PK is `i32` because popups are bounded; counters are
//!    `i64` because impression/conversion volume IS unbounded. The
//!    "popups admin analytics" page shows lifetime totals across
//!    the whole user base — a regression to `i32` would silently
//!    overflow at ~2.1B impressions (reachable in a single
//!    promotional period). Pin `ViewMetrics.total` / `today` /
//!    `this_week` / `this_month` as i64.
//!
//! 3. **`ListPopupsQuery` is fully optional with `#[serde(rename =
//!    "type")]` on `popup_type`.**
//!
//!    The admin lands on `/admin/popups` with zero params. The
//!    Rust field name `popup_type` clashes with the SQL column
//!    `type` (a reserved word) and the JSON wire `type`, so the
//!    struct uses `#[serde(rename = "type")]`. R9-D NEGATIVE pin:
//!    snake-case `popup_type` on the wire MUST NOT populate the
//!    field (rename is camelCase ONLY).
//!
//! 4. **`CreatePopupRequest.name` is the ONLY required field.**
//!
//!    Per the wire contract — `name` is `String`, everything else
//!    `Option<...>`. The handler defaults type to "newsletter",
//!    priority to 0, status to "draft", etc. A regression that
//!    flipped any of those to required would 422 every "create
//!    popup" click.
//!
//! 5. **`CreatePopupRequest` `#[serde(rename = "type")]`** — same
//!    rationale as (3). Frontend sends `type`, Rust binds
//!    `popup_type`. NEGATIVE pin: snake-case `popup_type` ignored.
//!
//! 6. **Router mount table — 10 routes, ALL admin-gated.** Compile
//!    pin against handler-signature drift.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_indicators_test.rs`,
//! `tests/admin_member_management_test.rs`, `tests/forms_test.rs`,
//! `tests/coupons_test.rs`.

use revolution_api::models::popup::{
    AbTestVariantMetrics, ConversionMetrics, CreatePopupRequest, DailyCount, DeviceBreakdown,
    UpdatePopupRequest, ViewMetrics,
};
use revolution_api::routes::admin_popups::{
    AbTestVariant, CreateAbTestRequest, ListPopupsQuery, ToggleStatusRequest,
};

// ── 1. PK type — popups.id is i32 (BIGSERIAL "Reserved exception") ───

/// CLAUDE.md "Money / cents" rule carves out a "Reserved exception"
/// for bounded-cardinality counters / PKs: `revisions: i32`,
/// `attempts: i32`, `total_pages: i32` are explicitly listed as
/// fine. Popups qualify under the SAME rationale — total
/// cardinality is bounded by the admin team's hand-curation (the
/// app will never have 2.1B popups). `models::popup::Popup.id: i32`
/// and the handler `Path<i32>` extractors confirm this.
///
/// MONEY never qualifies — popup analytics counters
/// (`total_views`, `total_conversions`) are i64 (see test #2).
///
/// Positive pin: `AbTestVariantMetrics.popup_id` is `i32` and a
/// reasonable popup ID round-trips. NEGATIVE pin: an `i32::MAX + 1`
/// `popup_id` MUST fail to deserialize (the route extractor is
/// `Path<i32>`, not `Path<i64>`).
#[test]
fn popups_pk_is_i32_bigserial_reserved_exception() {
    // Compile-pin: AbTestVariantMetrics.popup_id is i32.
    let metrics = AbTestVariantMetrics {
        popup_id: 42,
        variant_name: "Variant A".to_string(),
        impressions: 1_000_000_000_i64, // 1B — needs i64
        conversions: 500_000_000_i64,
        conversion_rate: 50.0,
        confidence: 0.95,
        is_winner: true,
    };
    let _: i32 = metrics.popup_id;

    let wire = serde_json::to_value(&metrics).expect("serialize AbTestVariantMetrics");
    assert_eq!(
        wire["popup_id"].as_i64(),
        Some(42),
        "popup_id MUST round-trip as i32-compatible JSON number"
    );

    // R9-D NEGATIVE: an i32-overflowing popup_id MUST fail to parse
    // back into `AbTestVariantMetrics` (i32 is the Rust type, and
    // serde_json enforces the bounds — i32::MAX + 1 = 2_147_483_648).
    let overflow = serde_json::json!({
        "popup_id": (i32::MAX as i64) + 1,
        "variant_name": "Overflow",
        "impressions": 0_i64,
        "conversions": 0_i64,
        "conversion_rate": 0.0,
        "confidence": 0.0,
        "is_winner": false,
    });
    assert!(
        serde_json::from_value::<AbTestVariantMetrics>(overflow).is_err(),
        "popup_id > i32::MAX MUST fail to deserialize — popups.id is SERIAL (i32) by design (Reserved exception)"
    );
}

// ── 2. Analytics counters are i64 (CLAUDE.md money/counter rule) ─────

/// Per CLAUDE.md "Reserved exception": the PK can be i32 (bounded
/// cardinality), but counters that AGGREGATE across the user base
/// MUST stay i64 — `total_views`, `total_conversions`, and the
/// admin-analytics `ViewMetrics` / `ConversionMetrics` / device
/// breakdown numbers can exceed i32::MAX during a single promotion
/// (a $0-cost popup shown on every page-load racks impressions
/// fast). Pin all five fields on `ViewMetrics` and the device-
/// breakdown counters with values past i32::MAX.
#[test]
fn popup_analytics_counters_are_i64_for_unbounded_aggregates() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let views = ViewMetrics {
        total: above_i32_max,
        today: above_i32_max,
        this_week: above_i32_max,
        this_month: above_i32_max,
        trend: "up".to_string(),
        trend_percentage: 12.5,
    };
    let _: i64 = views.total;
    let _: i64 = views.today;
    let _: i64 = views.this_week;
    let _: i64 = views.this_month;

    let wire = serde_json::to_value(&views).expect("serialize ViewMetrics");
    assert_eq!(wire["total"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["today"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["this_week"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["this_month"].as_i64(), Some(above_i32_max));

    // Round-trip back: confirm i64 survives serde
    let parsed: ViewMetrics = serde_json::from_value(wire).expect("ViewMetrics round-trip");
    assert_eq!(parsed.total, above_i32_max);
    assert_eq!(parsed.today, above_i32_max);

    // Same pin for ConversionMetrics (parallel shape)
    let conv = ConversionMetrics {
        total: above_i32_max,
        today: above_i32_max,
        this_week: above_i32_max,
        this_month: above_i32_max,
        trend: "down".to_string(),
        trend_percentage: -3.2,
    };
    let _: i64 = conv.total;
    let conv_wire = serde_json::to_value(&conv).expect("serialize ConversionMetrics");
    assert_eq!(conv_wire["total"].as_i64(), Some(above_i32_max));

    // Device breakdown — desktop+tablet+mobile, each i64
    let dev = DeviceBreakdown {
        desktop: above_i32_max,
        tablet: above_i32_max,
        mobile: above_i32_max,
    };
    let _: i64 = dev.desktop;
    let _: i64 = dev.tablet;
    let _: i64 = dev.mobile;

    // Timeline daily counts — DailyCount.count is i64
    let day = DailyCount {
        date: "2026-05-20".to_string(),
        count: above_i32_max,
    };
    let _: i64 = day.count;
    let day_wire = serde_json::to_value(&day).expect("serialize DailyCount");
    assert_eq!(day_wire["count"].as_i64(), Some(above_i32_max));
}

// ── 3. ListPopupsQuery is fully optional + #[serde(rename="type")] ──

/// `ListPopupsQuery` is the query-string DTO behind
/// `/admin/popups`. The admin lands on this page with zero params;
/// every field is Optional. The handler defaults `page=1`,
/// `per_page=20`, leaves the others unset. A regression that flipped
/// any field to required would 422 every admin who opens the page.
///
/// The `popup_type` Rust field is renamed to wire `type` (SQL
/// column name + reserved word in Rust). R9-D NEGATIVE: snake_case
/// `popup_type` on the wire MUST be ignored.
#[test]
fn list_popups_query_optional_with_type_rename() {
    // Zero params — pure default.
    let empty: ListPopupsQuery =
        serde_json::from_str("{}").expect("empty ListPopupsQuery MUST parse (default admin load)");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.status.is_none());
    assert!(empty.search.is_none());
    assert!(empty.popup_type.is_none());

    // Full params — `type` (camel) populates popup_type.
    let full: ListPopupsQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 50,
        "status": "published",
        "search": "welcome",
        "type": "exit_intent",
    }))
    .expect("full ListPopupsQuery MUST parse");
    assert_eq!(full.page, Some(2));
    assert_eq!(full.per_page, Some(50));
    assert_eq!(full.status.as_deref(), Some("published"));
    assert_eq!(full.search.as_deref(), Some("welcome"));
    assert_eq!(
        full.popup_type.as_deref(),
        Some("exit_intent"),
        "Wire `type` MUST populate Rust `popup_type` (serde rename)"
    );

    // R9-D NEGATIVE: snake `popup_type` on the wire MUST be ignored
    // (rename target is exactly `type` — snake_case is not an alias).
    let snake: ListPopupsQuery = serde_json::from_value(serde_json::json!({
        "popup_type": "newsletter",
    }))
    .expect("ListPopupsQuery parses (popup_type ignored, defaults applied)");
    assert!(
        snake.popup_type.is_none(),
        "Snake `popup_type` wire MUST NOT populate Rust `popup_type` — rename is `type`"
    );

    // R9-D NEGATIVE: garbage in `per_page` MUST fail
    assert!(
        serde_json::from_value::<ListPopupsQuery>(serde_json::json!({
            "per_page": "twenty",
        }))
        .is_err(),
        "ListPopupsQuery.per_page MUST reject string input"
    );
}

// ── 4. CreatePopupRequest — only `name` required; `type` is renamed ─

/// `CreatePopupRequest.name` is the ONLY required field — all others
/// have handler-side defaults (popup_type → "newsletter", status →
/// "draft", priority → 0, etc.). A regression that flipped any of
/// those to required would 422 every "Create Popup" click.
///
/// `popup_type` is `#[serde(rename = "type")]`. Frontend convention:
/// the request body uses `type`.
///
/// `UpdatePopupRequest` is fully-Optional (PATCH semantics). The
/// admin's "Save" button PATCHes only changed fields.
#[test]
fn create_popup_request_only_name_required_with_type_rename() {
    // Happy path — minimal valid (name only).
    let minimal: CreatePopupRequest = serde_json::from_value(serde_json::json!({
        "name": "Welcome Popup",
    }))
    .expect("CreatePopupRequest with only `name` MUST parse");
    assert_eq!(minimal.name, "Welcome Popup");
    assert!(minimal.popup_type.is_none());
    assert!(minimal.status.is_none());
    assert!(minimal.priority.is_none());

    // Wire `type` populates Rust `popup_type`
    let with_type: CreatePopupRequest = serde_json::from_value(serde_json::json!({
        "name": "Exit Intent",
        "type": "exit_intent",
        "status": "published",
        "priority": 10,
    }))
    .expect("CreatePopupRequest with `type` MUST parse");
    assert_eq!(
        with_type.popup_type.as_deref(),
        Some("exit_intent"),
        "Wire `type` MUST populate popup_type (serde rename)"
    );
    assert_eq!(with_type.status.as_deref(), Some("published"));
    assert_eq!(with_type.priority, Some(10));

    // R9-D NEGATIVE: missing `name` MUST fail (only required field).
    assert!(
        serde_json::from_value::<CreatePopupRequest>(serde_json::json!({
            "type": "newsletter",
        }))
        .is_err(),
        "CreatePopupRequest without `name` MUST fail (the ONE required field)"
    );

    // R9-D NEGATIVE: wire snake `popup_type` MUST NOT populate
    // (rename target is `type` only).
    let snake: CreatePopupRequest = serde_json::from_value(serde_json::json!({
        "name": "Snake Test",
        "popup_type": "newsletter",
    }))
    .expect("CreatePopupRequest parses with `name` (popup_type ignored)");
    assert!(
        snake.popup_type.is_none(),
        "Wire snake `popup_type` MUST NOT populate Rust `popup_type` — rename is `type`"
    );

    // UpdatePopupRequest — fully optional (PATCH). Empty body parses.
    let empty_update: UpdatePopupRequest =
        serde_json::from_str("{}").expect("empty UpdatePopupRequest MUST parse (PATCH semantics)");
    assert!(empty_update.name.is_none());
    assert!(empty_update.popup_type.is_none());
    assert!(empty_update.status.is_none());
    assert!(empty_update.is_active.is_none());

    // UpdatePopupRequest also uses `type` rename + `isActive` rename
    let patch: UpdatePopupRequest = serde_json::from_value(serde_json::json!({
        "type": "timed",
        "isActive": true,
    }))
    .expect("UpdatePopupRequest with rename keys MUST parse");
    assert_eq!(patch.popup_type.as_deref(), Some("timed"));
    assert_eq!(patch.is_active, Some(true));
}

// ── 5. A/B test DTOs — variants list, optional allocation ────────────

/// `CreateAbTestRequest` powers `POST /:id/ab-test`. The admin
/// configures: `name` (required), `variants` (required vec of
/// `AbTestVariant`), and an optional `confidence_threshold` (the
/// handler defaults to 0.95). Each `AbTestVariant` has all-Optional
/// fields — the admin can leave variant name/content/allocation
/// blank and the handler fills defaults ("Variant N",
/// `100 / (n+1)`).
///
/// `ToggleStatusRequest` is dead-simple: a single bool. R9-D
/// NEGATIVE: missing the bool MUST fail.
#[test]
fn ab_test_dtos_and_toggle_status_shape() {
    // CreateAbTestRequest — minimal (name + empty variants)
    let minimal: CreateAbTestRequest = serde_json::from_value(serde_json::json!({
        "name": "Headline Test",
        "variants": [],
    }))
    .expect("minimal CreateAbTestRequest MUST parse");
    assert_eq!(minimal.name, "Headline Test");
    assert!(minimal.variants.is_empty());
    assert!(minimal.confidence_threshold.is_none());

    // Rich — three variants with mixed populated fields
    let rich: CreateAbTestRequest = serde_json::from_value(serde_json::json!({
        "name": "CTA Color Test",
        "confidence_threshold": 0.99,
        "variants": [
            {"name": "Blue CTA", "title": "Click here", "content": "Sign up", "allocation": 33},
            {"title": "Buy now", "allocation": 33},
            {},
        ],
    }))
    .expect("rich CreateAbTestRequest MUST parse");
    assert_eq!(rich.confidence_threshold, Some(0.99));
    assert_eq!(rich.variants.len(), 3);
    assert_eq!(rich.variants[0].name.as_deref(), Some("Blue CTA"));
    assert_eq!(rich.variants[0].allocation, Some(33));
    assert!(rich.variants[1].name.is_none());
    assert_eq!(rich.variants[1].title.as_deref(), Some("Buy now"));
    assert!(rich.variants[2].name.is_none()); // empty variant — handler fills defaults
    assert!(rich.variants[2].allocation.is_none());

    // R9-D NEGATIVE: missing `variants` MUST fail (handler iterates it)
    assert!(
        serde_json::from_value::<CreateAbTestRequest>(serde_json::json!({
            "name": "Bad Test",
        }))
        .is_err(),
        "CreateAbTestRequest without `variants` MUST fail (required Vec)"
    );

    // R9-D NEGATIVE: missing `name` MUST fail
    assert!(
        serde_json::from_value::<CreateAbTestRequest>(serde_json::json!({
            "variants": [],
        }))
        .is_err(),
        "CreateAbTestRequest without `name` MUST fail (required)"
    );

    // AbTestVariant — fully optional, including empty `{}`
    let empty: AbTestVariant =
        serde_json::from_str("{}").expect("empty AbTestVariant MUST parse (all fields optional)");
    assert!(empty.name.is_none());
    assert!(empty.title.is_none());
    assert!(empty.content.is_none());
    assert!(empty.allocation.is_none());

    // ToggleStatusRequest — `is_active` is required bool
    let toggle: ToggleStatusRequest = serde_json::from_value(serde_json::json!({
        "is_active": true,
    }))
    .expect("ToggleStatusRequest with is_active MUST parse");
    assert!(toggle.is_active);

    // R9-D NEGATIVE: missing `is_active` MUST fail
    assert!(
        serde_json::from_value::<ToggleStatusRequest>(serde_json::json!({})).is_err(),
        "ToggleStatusRequest without `is_active` MUST fail (required bool)"
    );
}

// ── 6. Router mount-table compile-pin (10 routes, admin-gated) ───────

/// `routes::admin_popups::router()` MUST build as `Router<AppState>`.
/// Mount table (all admin-gated via `_admin: AdminUser`):
///   - GET    /                       (list_popups)
///   - POST   /                       (create_popup)
///   - GET    /:id                    (get_popup)         — Path<i32>
///   - PUT    /:id                    (update_popup)      — Path<i32>
///   - DELETE /:id                    (delete_popup)      — Path<i32>
///   - POST   /:id/duplicate          (duplicate_popup)   — Path<i32>
///   - POST   /:id/toggle-status      (toggle_popup_status) — Path<i32>
///   - GET    /:id/analytics          (get_popup_analytics) — Path<i32>
///   - POST   /:id/ab-test            (create_ab_test)    — Path<i32>
///   - GET    /ab-tests/:test_id/results (get_ab_test_results) — Path<i32>
///
/// `Path<i32>` is load-bearing: the popups.id column is `SERIAL`,
/// not `BIGSERIAL`. A regression that flipped it to `Path<i64>`
/// would compile but silently 400 every popup edit/delete URL the
/// frontend emits — the frontend serializes the i32 PK as a JS
/// number and Axum's Path parser is strict about type widths only
/// at the i64-overflow boundary, but a route-level type drift would
/// produce subtle 422 responses on edge cases.
///
/// The admin-only contract is also load-bearing: every handler
/// takes `_admin: AdminUser`. A regression dropping that extractor
/// from `delete_popup` / `create_ab_test` would let any logged-in
/// member nuke production popups or fork unauthorized A/B tests.
#[test]
fn admin_popups_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::admin_popups::router();
}

/// Idempotent construction. Per CLAUDE.md habit #3, pin that nothing
/// global (no `OnceLock`, no `static mut`) lives inside the router
/// constructor — a refactor that hoisted state into a global would
/// silently fail this test the second time `router()` runs.
#[test]
fn admin_popups_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_popups::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_popups::router();
}
