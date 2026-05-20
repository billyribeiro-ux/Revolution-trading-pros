//! Popups (public-facing) route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::popups` and exercises
//! the public DTOs (`PublicPopup`, `DisplayRulesCompat`,
//! `ActivePopupsQuery`, `TrackEventBody`, `TrackConversionBody`,
//! `PopupFormSubmission`, `BatchEventsBody`, `BatchEvent`) + the
//! `router()` mount table.
//!
//! ## Why this shape — HIGH TRAFFIC surface
//!
//! `routes/popups.rs` is the public-facing popups surface mounted at
//! `/api/popups/*`. Five endpoints:
//!   - GET  /active                (no auth — every public page)
//!   - POST /:id/impression        (no auth — fire-and-forget tracking)
//!   - POST /:id/conversion        (no auth — fire-and-forget tracking)
//!   - POST /:id/form-submit       (no auth — lead-capture flow)
//!   - POST /events                (no auth — batched analytics)
//!
//! `/active` is invoked by the popup loader on EVERY public page —
//! one of the highest-traffic endpoints on the site. Any wire-format
//! drift breaks the popup display layer immediately and visibly.
//!
//! What we CAN pin without a DB:
//!
//! 1. **`PublicPopup.id` is `i32`** — CLAUDE.md "Reserved exception"
//!    for legitimately bounded counts. The `popups` table is admin-
//!    curated (every popup is hand-built by marketing); a site
//!    crossing 2B popup rows is not plausible. The existing schema
//!    uses `INTEGER` and the route's `PopupRow.id: i32` matches.
//!    NOT to be widened to i64 without a coordinated migration.
//!
//! 2. **`PublicPopup.impressions` and `.conversions` are `i64`** —
//!    these are running totals across the LIFETIME of the popup
//!    (incremented on every page view that matches). A
//!    successful exit-intent popup on a high-traffic landing page
//!    can legitimately accumulate 100M+ views over years. `i32`
//!    would overflow silently around 2.1B. The DTO pin matches
//!    the `PopupRow.total_views: i64`/`total_conversions: i64`
//!    DB columns.
//!
//! 3. **Mixed serde rename: snake_case + camelCase aliases**.
//!    `PublicPopup` is a backward-compat layer for the popup
//!    frontend — some keys are snake_case (`cta_text`, `position`,
//!    `trigger_rules`) and SOME are `#[serde(rename = "isActive")]`
//!    / `"closeButton"` / `"closeOnOverlayClick"` / `"displayRules"`
//!    (camelCase) for the legacy popup loader. A regression that
//!    flipped EVERY key to camelCase would break the new admin
//!    surface; one that flipped EVERY key to snake_case would break
//!    the legacy popup loader. The pins below catch BOTH drifts.
//!
//! 4. **`ActivePopupsQuery` is fully optional** — the popup loader
//!    on `/` typically sends `?page=/&device=desktop` but MUST
//!    accept GET with no params (defaults: `page="/"`,
//!    `device="desktop"`). A required field would 400-error every
//!    public page load.
//!
//! 5. **`PopupFormSubmission.form_data` is required, snake_case is
//!    NOT** — the legacy popup loader serializes form data as
//!    `formData` (camelCase). NEGATIVE pin per R9-D: a snake_case
//!    rename would silently 422 every popup form submit (lead
//!    capture broken).
//!
//! 6. **`BatchEventsBody.events` is required** — the analytics
//!    buffer fires once per batch interval; an empty array is
//!    legal (no-op), but the `events` key MUST be present.
//!    Missing key MUST fail (catches a frontend regression that
//!    drops the wrapper).
//!
//! 7. **`router()` mount table compile-pin** — high-traffic
//!    endpoint group. A refactor that accidentally swapped GET
//!    for POST on `/active` would break the popup loader on
//!    every public page; the compile-pin catches handler-result-
//!    type drift, the runtime pin doesn't help.
//!
//! ## Pattern source
//!
//! Modeled on `tests/redirects_test.rs`, `tests/contacts_test.rs`,
//! `tests/forms_test.rs` (sibling public + analytics surfaces).

use revolution_api::models::popup::{DisplayRules, FrequencyRules, PopupDesign, TriggerRules};
use revolution_api::routes::popups::{
    ActivePopupsQuery, BatchEvent, BatchEventsBody, DisplayRulesCompat, PopupFormSubmission,
    PublicPopup, TrackConversionBody, TrackEventBody,
};

// ── 1. PublicPopup: id is i32 (CLAUDE.md "Reserved exception"); ──
//      impressions + conversions are i64 (lifetime totals).

/// `PublicPopup.id` is `i32` — CLAUDE.md "Reserved exception" for
/// legitimately bounded counts. The `popups` table is admin-curated
/// (every row is hand-built by marketing); crossing `2^31` rows is
/// not plausible for this surface. The existing schema and
/// `PopupRow.id: i32` both match — a widening here would be a
/// coordinated migration, not a silent type change.
///
/// `PublicPopup.impressions` and `.conversions` are `i64` because
/// they're lifetime running totals (every view of a popup increments
/// the counter). A successful exit-intent popup on a high-traffic
/// landing page can legitimately accumulate 100M+ views over years;
/// `i32` would overflow silently around 2.1B (i32::MAX).
#[test]
fn public_popup_id_is_i32_impressions_conversions_are_i64() {
    let p = fixture_public_popup(/* id */ 42);

    // Compile-time pins (load-bearing — catches type narrowing).
    let _: i32 = p.id;
    let _: i64 = p.impressions;
    let _: i64 = p.conversions;

    // Lifetime-totals must accept values above i32::MAX without overflow.
    let p_big = PublicPopup {
        impressions: (i32::MAX as i64) + 100_000_000,
        conversions: (i32::MAX as i64) + 1,
        ..fixture_public_popup(1)
    };
    let wire = serde_json::to_value(&p_big).expect("serialize PublicPopup");
    assert_eq!(
        wire["impressions"].as_i64(),
        Some((i32::MAX as i64) + 100_000_000),
        "impressions MUST be i64 — lifetime view totals can exceed 2^31"
    );
    assert_eq!(
        wire["conversions"].as_i64(),
        Some((i32::MAX as i64) + 1),
        "conversions MUST be i64 — lifetime conversion totals can exceed 2^31"
    );
}

// ── 2. PublicPopup wire format: mixed snake_case + camelCase ─────
//      backward-compat layer for legacy popup loader.

/// `PublicPopup` is a backward-compat DTO that emits SOME keys as
/// snake_case (new admin) and SOME keys as camelCase (legacy popup
/// loader). The split is deliberate — flipping the entire struct to
/// either format would break one consumer or the other.
///
/// NEGATIVE pins per R9-D — both directions:
///   - snake_case keys MUST be present (`cta_text`, `popup_type`).
///   - camelCase aliases MUST be present (`isActive`, `closeButton`,
///     `closeOnOverlayClick`, `displayRules`).
#[test]
fn public_popup_wire_is_mixed_snake_and_camel_per_field() {
    let p = fixture_public_popup(7);
    let wire = serde_json::to_value(&p).expect("serialize PublicPopup");

    // POSITIVE snake_case (new admin surface)
    assert!(
        wire.get("cta_text").is_some(),
        "cta_text MUST be snake_case (admin surface)"
    );
    assert!(
        wire.get("cta_url").is_some(),
        "cta_url MUST be snake_case (admin surface)"
    );
    assert!(
        wire.get("close_on_overlay_click").is_some(),
        "close_on_overlay_click MUST be snake_case (admin surface)"
    );
    assert!(
        wire.get("trigger_rules").is_some(),
        "trigger_rules MUST be snake_case"
    );

    // POSITIVE camelCase (legacy popup loader consumers)
    assert!(
        wire.get("isActive").is_some(),
        "isActive MUST be present (camelCase legacy alias for is_active)"
    );
    assert!(
        wire.get("closeButton").is_some(),
        "closeButton MUST be present (camelCase legacy alias)"
    );
    assert!(
        wire.get("closeOnOverlayClick").is_some(),
        "closeOnOverlayClick MUST be present (camelCase legacy alias for the compat field)"
    );
    assert!(
        wire.get("displayRules").is_some(),
        "displayRules MUST be present (camelCase legacy alias for the compat block)"
    );

    // The `type` field is renamed via `#[serde(rename = "type")]` — verify it.
    assert!(
        wire.get("type").is_some(),
        "popup_type serializes as `type` per #[serde(rename = \"type\")]"
    );
    assert!(
        wire.get("popup_type").is_none(),
        "raw `popup_type` MUST NOT appear on the wire — Rust field name is hidden by rename"
    );

    // NEGATIVE: full snake_case takeover would lose the legacy keys.
    assert!(
        wire.get("is_active").is_none(),
        "is_active (snake) MUST NOT appear — would break legacy loader. Rename target is `isActive`."
    );
    assert!(
        wire.get("close_button").is_none(),
        "close_button (snake) MUST NOT appear — would break legacy loader. Rename target is `closeButton`."
    );

    // NEGATIVE: DisplayRulesCompat fields are camelCase.
    let compat = &wire["displayRules"];
    assert!(
        compat.get("delaySeconds").is_some(),
        "delaySeconds MUST be camelCase on the compat sub-object"
    );
    assert!(
        compat.get("delay_seconds").is_none(),
        "delay_seconds (snake) MUST NOT appear on the compat sub-object"
    );
    assert!(
        compat.get("showOnScroll").is_some(),
        "showOnScroll MUST be camelCase on the compat sub-object"
    );
    assert!(
        compat.get("deviceTargeting").is_some(),
        "deviceTargeting MUST be camelCase on the compat sub-object"
    );
}

// ── 3. ActivePopupsQuery: fully optional (every public page load) ──

/// The popup loader hits `/api/popups/active` on EVERY public page
/// load. The query MUST tolerate an empty payload (no params) AND a
/// fully-specified payload. A required field would 400-error every
/// public page on the site — the highest-impact wire regression in
/// this file.
#[test]
fn active_popups_query_accepts_empty_and_full() {
    // Empty (no targeting hints — fall back to defaults at the handler).
    // Use serde_json shape; the JSON {} maps to "all fields None" because
    // every field is `Option<>` — the same property `serde_urlencoded`
    // exploits when the axum Query extractor sees an empty query string.
    let empty: ActivePopupsQuery =
        serde_json::from_str("{}").expect("empty ActivePopupsQuery MUST parse");
    assert!(empty.page.is_none());
    assert!(empty.device.is_none());
    assert!(empty.utm_source.is_none());
    assert!(empty.utm_campaign.is_none());
    assert!(empty.is_returning.is_none());

    // Full (popup loader's max-info call)
    let full: ActivePopupsQuery = serde_json::from_value(serde_json::json!({
        "page": "/checkout",
        "device": "mobile",
        "utm_source": "google",
        "utm_campaign": "spring",
        "is_returning": true,
    }))
    .expect("full ActivePopupsQuery MUST parse");
    assert_eq!(full.page.as_deref(), Some("/checkout"));
    assert_eq!(full.device.as_deref(), Some("mobile"));
    assert_eq!(full.utm_source.as_deref(), Some("google"));
    assert_eq!(full.utm_campaign.as_deref(), Some("spring"));
    assert_eq!(full.is_returning, Some(true));

    // Boolean toggling — pin both true and false roundtrip.
    let returning_false: ActivePopupsQuery = serde_json::from_value(serde_json::json!({
        "page": "/",
        "device": "desktop",
        "is_returning": false,
    }))
    .expect("is_returning=false MUST parse");
    assert_eq!(returning_false.is_returning, Some(false));
}

// ── 4. TrackEventBody + TrackConversionBody: all-optional shape ──
//      (fire-and-forget tracking — never block the popup UI)

/// Both `TrackEventBody` (impression beacon) and `TrackConversionBody`
/// (conversion beacon) MUST accept an empty body. The popup loader
/// fires these as `navigator.sendBeacon(...)` from `pagehide`/`unload`
/// where serializing the full session context may fail; the beacon
/// MUST still reach the server with a 200 (the row insert will just
/// have NULLs for missing fields).
///
/// Also: `TrackConversionBody.conversion_time` is camelCase-aliased
/// per the existing serde rename. R9-D NEGATIVE pin — snake_case
/// `conversion_time` MUST fail to populate the field.
#[test]
fn track_event_bodies_accept_empty_and_pin_camel_alias() {
    // Empty impression beacon (most common in the wild)
    let empty_view: TrackEventBody =
        serde_json::from_str("{}").expect("empty TrackEventBody MUST parse");
    assert!(empty_view.session_id.is_none());
    assert!(empty_view.page_url.is_none());
    assert!(empty_view.device_type.is_none());

    // Empty conversion beacon
    let empty_conv: TrackConversionBody =
        serde_json::from_str("{}").expect("empty TrackConversionBody MUST parse");
    assert!(empty_conv.session_id.is_none());
    assert!(empty_conv.conversion_time.is_none());

    // Conversion with camelCase `conversionTime` alias (the legacy loader's shape)
    let with_alias: TrackConversionBody = serde_json::from_value(serde_json::json!({
        "session_id": "sess_abc",
        "conversionTime": 1700000000_i64,
    }))
    .expect("camelCase alias `conversionTime` MUST populate conversion_time");
    assert_eq!(with_alias.conversion_time, Some(1_700_000_000));
    assert_eq!(with_alias.session_id.as_deref(), Some("sess_abc"));

    // R9-D NEGATIVE: snake_case `conversion_time` is NOT renamed, so it
    // would NOT populate the field (Rust field is `conversion_time` with
    // `#[serde(rename = "conversionTime")]` — sending raw snake_case is
    // ignored, NOT an error, but the field stays None).
    let no_alias: TrackConversionBody = serde_json::from_value(serde_json::json!({
        "conversion_time": 1_700_000_000_i64,
    }))
    .expect("ignored-unknown-key is fine; field stays None");
    assert!(
        no_alias.conversion_time.is_none(),
        "snake_case `conversion_time` MUST NOT populate the field — rename target is `conversionTime`"
    );
}

// ── 5. PopupFormSubmission: form_data REQUIRED, camelCase rename ──

/// Lead-capture flow. The popup-form submit endpoint MUST receive a
/// `formData` (camelCase) field; the renamed serde target is the
/// existing house contract for the legacy popup loader.
///
/// R9-D NEGATIVE pin: snake_case `form_data` MUST fail to parse
/// (the field is `#[serde(rename = "formData")]` and required —
/// missing key fails deserialization).
#[test]
fn popup_form_submission_requires_camel_form_data() {
    // Happy path: camelCase formData with payload
    let ok: PopupFormSubmission = serde_json::from_value(serde_json::json!({
        "formData": {"email": "a@b.com", "name": "Alice"},
        "session_id": "sess_xyz",
    }))
    .expect("camelCase formData MUST parse");
    assert_eq!(ok.form_data["email"], "a@b.com");
    assert_eq!(ok.session_id.as_deref(), Some("sess_xyz"));

    // R9-D NEGATIVE: missing formData entirely MUST fail.
    let missing = serde_json::from_value::<PopupFormSubmission>(serde_json::json!({
        "session_id": "sess_xyz",
    }));
    assert!(
        missing.is_err(),
        "PopupFormSubmission without `formData` MUST fail — field is required"
    );

    // R9-D NEGATIVE: snake_case `form_data` MUST fail (rename target is camelCase ONLY).
    let snake = serde_json::from_value::<PopupFormSubmission>(serde_json::json!({
        "form_data": {"email": "a@b.com"},
    }));
    assert!(
        snake.is_err(),
        "PopupFormSubmission with snake `form_data` MUST fail — wire contract is `formData` camelCase"
    );
}

// ── 6. BatchEventsBody: events key required (analytics buffer wrapper) ─

/// `/api/popups/events` is the buffered-analytics endpoint. The
/// frontend buffers up impressions/conversions and POSTs them in a
/// `{events: [...]}` wrapper at intervals. The `events` key MUST be
/// present (the wrapper is load-bearing — strips a missing-key bug
/// in the frontend buffer).
///
/// Each `BatchEvent.event_type` is required (`#[serde(rename = "type")]`).
/// R9-D NEGATIVE: missing `type` MUST fail.
#[test]
fn batch_events_body_requires_events_and_event_type() {
    // Empty events array is legal — no-op batch (handler just returns processed=0)
    let empty: BatchEventsBody =
        serde_json::from_str(r#"{"events":[]}"#).expect("empty events array MUST parse");
    assert_eq!(empty.events.len(), 0);

    // Full batch with one event
    let full: BatchEventsBody = serde_json::from_value(serde_json::json!({
        "events": [{
            "popup_id": 42,
            "type": "view",
            "popupId": "42",
            "session_id": "sess_a",
            "page_url": "/landing",
            "device_type": "mobile",
            "data": {"variant": "A"},
            "timestamp": "2026-05-20T12:00:00Z",
        }],
    }))
    .expect("full BatchEventsBody MUST parse");
    assert_eq!(full.events.len(), 1);
    let ev: &BatchEvent = &full.events[0];
    assert_eq!(ev.event_type, "view"); // came via `type` rename
    assert_eq!(ev.popup_id, Some(42));
    assert_eq!(ev.popup_id_alt.as_deref(), Some("42")); // camelCase alias for popupId

    // R9-D NEGATIVE: missing `events` wrapper MUST fail.
    assert!(
        serde_json::from_value::<BatchEventsBody>(serde_json::json!({})).is_err(),
        "BatchEventsBody without `events` MUST fail — wrapper key is required"
    );

    // R9-D NEGATIVE: BatchEvent without `type` MUST fail.
    let event_missing_type = serde_json::from_value::<BatchEventsBody>(serde_json::json!({
        "events": [{"session_id": "x"}],
    }));
    assert!(
        event_missing_type.is_err(),
        "BatchEvent without `type` MUST fail — event_type is required"
    );

    // R9-D NEGATIVE: BatchEvent with `event_type` (snake, no rename) MUST also fail.
    // The Rust field is `event_type` but the serde rename target is `type`. Sending
    // snake_case `event_type` would be ignored and the required `type` key would still
    // be missing — deserialization fails.
    let snake_event_type = serde_json::from_value::<BatchEventsBody>(serde_json::json!({
        "events": [{"event_type": "view"}],
    }));
    assert!(
        snake_event_type.is_err(),
        "BatchEvent with snake `event_type` MUST fail — wire contract is `type`"
    );
}

// ── 7. Router mount-table compile-pin (high-traffic surface) ────

/// `router()` MUST build as `Router<AppState>`. Mount table:
///   - GET  `/active`             (no auth, public)
///   - POST `/:id/impression`     (no auth, tracking beacon)
///   - POST `/:id/conversion`     (no auth, tracking beacon)
///   - POST `/:id/form-submit`    (no auth, lead capture)
///   - POST `/events`             (no auth, batched analytics)
///
/// This is one of the highest-traffic groups on the site — `/active`
/// is invoked on every public page load. A refactor that broke the
/// mount table (e.g. swapped GET for POST on `/active`, or
/// accidentally added an `AdminUser`/`User` extractor that blocked
/// anon traffic) would fail compilation here. The compile-pin
/// catches handler-signature drift that runtime tests on a CI-only
/// fixture might miss.
#[test]
fn popups_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::popups::router();
}

/// Idempotent construction — building twice in the same process must
/// succeed. Per CLAUDE.md habit #3 ("cached state lost during
/// refactor" landmine), pin that the router constructor holds no
/// hidden global state.
#[test]
fn popups_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::popups::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::popups::router();
    let _r3: axum::Router<revolution_api::AppState> = revolution_api::routes::popups::router();
}

// ────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────

/// Build a `PublicPopup` fixture with all required fields populated.
/// The DisplayRulesCompat sub-object pins the legacy popup-loader
/// camelCase contract; the outer struct pins the new admin surface's
/// mixed snake/camel contract.
fn fixture_public_popup(id: i32) -> PublicPopup {
    PublicPopup {
        id,
        name: "Spring Promo".to_string(),
        popup_type: "modal".to_string(),
        title: Some("20% Off".to_string()),
        content: Some("This week only.".to_string()),
        cta_text: Some("Shop Now".to_string()),
        cta_url: Some("/promo/spring".to_string()),
        cta_new_tab: false,
        position: "center".to_string(),
        size: "md".to_string(),
        animation: "fade".to_string(),
        show_close_button: true,
        close_on_overlay_click: true,
        close_on_escape: true,
        auto_close_after: Some(15),
        has_form: false,
        form_id: None,
        trigger_rules: TriggerRules::default(),
        frequency_rules: FrequencyRules::default(),
        display_rules: DisplayRules::default(),
        design: PopupDesign {
            background_color: "#ffffff".to_string(),
            title_color: "#1f2937".to_string(),
            text_color: "#374151".to_string(),
            button_color: "#3b82f6".to_string(),
            button_text_color: "#ffffff".to_string(),
            button_border_radius: 8,
            button_shadow: None,
            button_padding: None,
            secondary_button_color: None,
            secondary_button_text_color: None,
            overlay_color: "#000000".to_string(),
            overlay_opacity: 50,
            overlay_blur: None,
            custom_css: None,
            background_image: None,
            header_image: None,
            video_url: None,
            video_autoplay: false,
            border_radius: 12,
            template_id: None,
        },
        priority: 10,
        is_active: true,
        close_button: true,
        close_on_overlay_click_compat: true,
        display_rules_compat: DisplayRulesCompat {
            frequency: "once_per_session".to_string(),
            delay_seconds: 0,
            show_on_scroll: false,
            scroll_percentage: 50,
            show_on_exit: false,
            device_targeting: "all".to_string(),
            pages: "".to_string(),
            exclude_pages: "".to_string(),
        },
        impressions: 0,
        conversions: 0,
    }
}
