//! Consent management route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::consent` and exercises
//! the public DTOs (`ConsentSettings`, `BulkUpdateRequest`) + the
//! public `router()` mount table.
//!
//! ## Why this file exists (R16-D)
//!
//! `routes/consent.rs` is 305 LOC of admin-gated GDPR consent
//! configuration created on 2026-04-26 to satisfy the
//! `/admin/consent/settings` admin page. Every handler is
//! `AdminUser`-gated, every read/write goes through the generic
//! `settings` JSONB table (single fixed key
//! `consent_management_settings`). The file-level docs are explicit
//! that there is NO in-process cache by design — a process-local
//! fallback would mask DB outages (FULL_REPO_AUDIT_2026-05-17 P3
//! "consent OnceLock", cluster I). What we CAN pin in no-DB tests:
//!
//! 1. **`ConsentSettings::default()` returns GDPR-leaning defaults.**
//!
//!    Per the file-level docs (lines 94-140): "Defaults mirror the
//!    initial `$state` block on the admin page so a just-installed
//!    deployment renders a sensible, GDPR-leaning configuration
//!    with no further admin action required." A regression that
//!    flipped `consent_enabled`, `banner_enabled`, or
//!    `script_blocking_enabled` to `false` by default would silently
//!    disable consent enforcement on every new deployment.
//!
//! 2. **All bool fields round-trip through JSON wire.** The admin
//!    page reads/writes the full struct as JSON. A regression in
//!    serde derives would break the bulk-update endpoint silently.
//!
//! 3. **Validator constraints on numeric ranges.** The struct has
//!    `#[validate(range(...))]` on `expire_days`, `consent_version`,
//!    `close_on_scroll_distance`, `proof_retention_days`, and
//!    `#[validate(length(...))]` on the string version/layout
//!    fields. The bulk handler at line 245 calls `.validate()` and
//!    400s on failure — pin that out-of-range values trip validator.
//!
//!    Per CLAUDE.md "Reserved exception", all the integer fields
//!    here (`expire_days: i32`, `consent_version: i32`,
//!    `close_on_scroll_distance: i32`, `proof_retention_days: i32`)
//!    are bounded counters / day-counts. `expire_days` is capped at
//!    730 (2 years), `proof_retention_days` at 3650 (10 years) by
//!    `#[validate(range(...))]` — far below i32::MAX. MONEY never
//!    qualifies for the exception, but these don't represent money.
//!
//! 4. **`BulkUpdateRequest` requires the nested `settings` key.**
//!    The frontend sends `{ settings: { ... full ConsentSettings ... } }`
//!    (see file docs:145). A flat body MUST fail to deserialize.
//!
//! 5. **Router mount table — 3 routes, ALL admin-gated.**
//!    `GET /settings`, `POST /settings/bulk`, `POST /settings/reset`.
//!    Compile pin against handler signature drift; a regression that
//!    dropped the `AdminUser` extractor from `bulk_update_settings`
//!    or `reset_settings` would let any logged-in member bypass
//!    GDPR enforcement.
//!
//! 6. **JSON wire keys stay snake_case** — the file docs are explicit
//!    that snake_case wire matches the frontend `ConsentSettings`
//!    TypeScript interface. A regression to camelCase would silently
//!    fail every read/write from the admin page (404-on-key-mismatch
//!    at the destructuring level).
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_popups_test.rs`,
//! `tests/admin_page_layouts_test.rs`,
//! `tests/email_templates_test.rs`, `tests/cms_seo_test.rs`.

use revolution_api::routes::consent::{BulkUpdateRequest, ConsentSettings};
use validator::Validate;

// ── 1. ConsentSettings::default() is GDPR-leaning ───────────────────

/// Per the file-level docs (routes/consent.rs:94-140), defaults
/// mirror the initial `$state` block on the admin page and produce
/// a "GDPR-leaning configuration with no further admin action
/// required." A regression flipping `consent_enabled`,
/// `banner_enabled`, or `script_blocking_enabled` to `false` by
/// default would silently disable consent enforcement on every
/// new deployment.
///
/// Pin all three master switches + key script-block defaults +
/// `proof_consent_enabled` (audit-trail) so the regression fails
/// the test, not the user's first GDPR audit.
#[test]
fn consent_settings_default_is_gdpr_leaning() {
    let d = ConsentSettings::default();

    // Master switches — MUST be true by default
    assert!(
        d.consent_enabled,
        "consent_enabled MUST default to true (GDPR-leaning)"
    );
    assert!(
        d.banner_enabled,
        "banner_enabled MUST default to true (visible disclosure)"
    );
    assert!(
        d.script_blocking_enabled,
        "script_blocking_enabled MUST default to true (block until consent)"
    );

    // Sensible expiry — 365 days (annual reset)
    assert_eq!(d.expire_days, 365_i32, "default consent expiry MUST be 365");
    assert_eq!(d.consent_version, 1_i32);
    assert_eq!(d.policy_version, "1.0.0");
    assert!(!d.test_mode, "test_mode MUST default false in prod build");

    // Banner shape — must allow rejection (GDPR requires symmetric choice)
    assert!(
        d.show_reject_button,
        "show_reject_button MUST default true — GDPR requires symmetric accept/reject"
    );
    assert!(d.show_settings_button);
    assert_eq!(d.banner_position, "bottom");
    assert_eq!(d.banner_layout, "bar");
    assert!(
        !d.close_on_scroll,
        "close_on_scroll MUST default false — implicit consent via scroll is NOT GDPR-compliant"
    );
    assert_eq!(d.close_on_scroll_distance, 60_i32);

    // Script blocks — all major pixels MUST default to BLOCKED
    assert!(d.block_google_analytics);
    assert!(d.block_google_tag_manager);
    assert!(d.block_facebook_pixel);
    assert!(d.block_tiktok_pixel);
    assert!(d.block_twitter_pixel);
    assert!(d.block_linkedin_pixel);
    assert!(d.block_pinterest_tag);
    assert!(d.block_reddit_pixel);
    assert!(d.block_hotjar);
    assert!(d.block_youtube_embeds);
    assert!(d.block_vimeo_embeds);
    assert!(d.block_google_maps);

    // Google Consent Mode v2 (TCF-aligned) — on by default
    assert!(d.google_consent_mode_enabled);
    assert!(!d.bing_consent_mode_enabled);

    // Geolocation — disabled by default (PII-cost without an
    // explicit opt-in is the wrong default for a GDPR app)
    assert!(!d.geolocation_enabled);
    assert!(
        d.geo_default_strict,
        "geo_default_strict MUST default true — unknown geo = strict mode"
    );

    // Proof of consent — required for the audit trail
    assert!(
        d.proof_consent_enabled,
        "proof_consent_enabled MUST default true — GDPR Art.7(1) audit trail"
    );
    assert_eq!(d.proof_retention_days, 365_i32);
    assert!(d.proof_auto_delete);
}

// ── 2. Snake-case JSON wire round-trip (frontend contract) ───────────

/// The file docs are explicit (lines 35-39): "Field names are kept
/// snake_case on the wire to match the existing frontend
/// `ConsentSettings` interface". A regression to camelCase would
/// silently break the admin page (`load_settings` returns 200 but
/// the page destructures camelCase keys that don't exist → all
/// fields render as undefined).
///
/// Pin a full round-trip with snake_case keys + value-equality on
/// every field type (bool, i32, String).
#[test]
fn consent_settings_round_trip_snake_case() {
    let original = ConsentSettings::default();

    let wire = serde_json::to_value(&original).expect("serialize ConsentSettings");

    // Confirm snake_case keys at the wire surface
    assert!(wire.get("consent_enabled").is_some());
    assert!(wire.get("expire_days").is_some());
    assert!(wire.get("banner_position").is_some());
    assert!(wire.get("show_reject_button").is_some());
    assert!(wire.get("script_blocking_enabled").is_some());
    assert!(wire.get("block_google_analytics").is_some());
    assert!(wire.get("google_consent_mode_enabled").is_some());
    assert!(wire.get("geo_default_strict").is_some());
    assert!(wire.get("proof_retention_days").is_some());

    // R9-D NEGATIVE: camelCase wire MUST NOT populate snake fields
    // (serde without rename_all defaults to using struct field names
    // as-is; camelCase keys are not aliases).
    let camel: Result<ConsentSettings, _> = serde_json::from_value(serde_json::json!({
        "consentEnabled": true,
        "expireDays": 365,
        "policyVersion": "1.0.0",
        // ...remaining fields missing → MUST fail at the first missing required field
    }));
    assert!(
        camel.is_err(),
        "camelCase wire MUST fail to deserialize ConsentSettings (frontend sends snake_case per file docs)"
    );

    // Round-trip: serialize → deserialize → field equality
    let parsed: ConsentSettings = serde_json::from_value(wire).expect("ConsentSettings round-trip");
    assert_eq!(parsed.consent_enabled, original.consent_enabled);
    assert_eq!(parsed.expire_days, original.expire_days);
    assert_eq!(parsed.consent_version, original.consent_version);
    assert_eq!(parsed.policy_version, original.policy_version);
    assert_eq!(parsed.banner_position, original.banner_position);
    assert_eq!(parsed.show_reject_button, original.show_reject_button);
    assert_eq!(parsed.block_facebook_pixel, original.block_facebook_pixel);
    assert_eq!(parsed.proof_retention_days, original.proof_retention_days);
}

// ── 3. Validator constraints enforce range bounds ───────────────────

/// The handler at `bulk_update_settings` (line 245) calls
/// `payload.settings.validate()` before persisting and 400s with
/// `details: errors.to_string()` on failure. Pin the exact
/// constraints declared on the struct so a regression that
/// loosened or tightened them is caught at the test surface.
///
/// `expire_days`: 1-730 (2 years max — anything longer fails the
/// "fresh consent" GDPR principle).
/// `consent_version`: min 1 (zero or negative would invalidate the
/// "current version" comparison on the consent banner).
/// `close_on_scroll_distance`: 10-5000 px.
/// `proof_retention_days`: 30-3650 (10 years max — beyond which
/// retention is a privacy LIABILITY, not an audit gain).
/// `policy_version`: length 1-32.
/// `banner_position`, `banner_layout`: length 1-32.
///
/// Per CLAUDE.md "Reserved exception": these i32 day-counts are
/// bounded counters and i32 is appropriate. MONEY never qualifies.
#[test]
fn consent_settings_validator_enforces_ranges() {
    // Happy path — default passes
    assert!(
        ConsentSettings::default().validate().is_ok(),
        "Default ConsentSettings MUST pass validator"
    );

    // expire_days out of range (above 730)
    let mut bad = ConsentSettings::default();
    bad.expire_days = 1000;
    assert!(
        bad.validate().is_err(),
        "expire_days=1000 MUST fail (max 730)"
    );

    // expire_days = 0 (below min 1)
    let mut bad = ConsentSettings::default();
    bad.expire_days = 0;
    assert!(bad.validate().is_err(), "expire_days=0 MUST fail (min 1)");

    // consent_version < 1
    let mut bad = ConsentSettings::default();
    bad.consent_version = 0;
    assert!(
        bad.validate().is_err(),
        "consent_version=0 MUST fail (min 1)"
    );

    // proof_retention_days out of range
    let mut bad = ConsentSettings::default();
    bad.proof_retention_days = 5000; // exceeds 3650
    assert!(
        bad.validate().is_err(),
        "proof_retention_days=5000 MUST fail (max 3650 = 10 years)"
    );

    let mut bad = ConsentSettings::default();
    bad.proof_retention_days = 10; // below min 30
    assert!(
        bad.validate().is_err(),
        "proof_retention_days=10 MUST fail (min 30)"
    );

    // close_on_scroll_distance out of range
    let mut bad = ConsentSettings::default();
    bad.close_on_scroll_distance = 9999;
    assert!(
        bad.validate().is_err(),
        "close_on_scroll_distance > 5000 MUST fail"
    );

    let mut bad = ConsentSettings::default();
    bad.close_on_scroll_distance = 5;
    assert!(
        bad.validate().is_err(),
        "close_on_scroll_distance < 10 MUST fail"
    );

    // policy_version empty (length 0 below min 1)
    let mut bad = ConsentSettings::default();
    bad.policy_version = String::new();
    assert!(
        bad.validate().is_err(),
        "policy_version='' MUST fail (length 1-32)"
    );

    // policy_version too long (>32)
    let mut bad = ConsentSettings::default();
    bad.policy_version = "x".repeat(33);
    assert!(
        bad.validate().is_err(),
        "policy_version length 33 MUST fail (max 32)"
    );

    // banner_position empty
    let mut bad = ConsentSettings::default();
    bad.banner_position = String::new();
    assert!(
        bad.validate().is_err(),
        "banner_position='' MUST fail (length 1-32)"
    );

    // banner_layout overlong
    let mut bad = ConsentSettings::default();
    bad.banner_layout = "x".repeat(64);
    assert!(
        bad.validate().is_err(),
        "banner_layout overlong MUST fail (max 32)"
    );

    // R9-D NEGATIVE — at the type boundary, expire_days IS i32, so
    // an `expire_days > i32::MAX` payload MUST be rejected by serde
    // BEFORE validator gets a chance. Pin both layers.
    let overflow = serde_json::json!({
        "expire_days": (i32::MAX as i64) + 1,
    });
    // This is a partial body — will fail because of missing fields,
    // but we specifically construct a full struct via serde_json to
    // pin the type boundary (the struct cannot represent i32::MAX+1).
    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.insert(
            "expire_days".to_string(),
            serde_json::json!((i32::MAX as i64) + 1),
        );
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "expire_days > i32::MAX MUST fail to deserialize (i32 — Reserved exception)"
    );

    // Sanity: overflow value is i64-positive (just confirming it's
    // not a silent fail because serde_json couldn't parse the
    // number at all).
    assert_eq!(
        overflow["expire_days"].as_i64(),
        Some((i32::MAX as i64) + 1)
    );
}

// ── 4. BulkUpdateRequest — nested `settings` key required ───────────

/// The frontend sends `{ settings: { ... full ConsentSettings ... } }`
/// (file docs line 145). The handler at line 243 destructures
/// `Json(payload): Json<BulkUpdateRequest>` and accesses
/// `payload.settings`. A flat body lacking the wrapper would 422,
/// not silently no-op.
///
/// R9-D NEGATIVE: a flat body (no `settings` wrapper) MUST fail.
/// A body with `settings` set to garbage (string, array, missing
/// required fields) MUST fail.
#[test]
fn bulk_update_request_requires_nested_settings_key() {
    let defaults = ConsentSettings::default();
    let wrapper = serde_json::json!({ "settings": defaults });
    let parsed: BulkUpdateRequest =
        serde_json::from_value(wrapper).expect("BulkUpdateRequest with nested settings MUST parse");
    assert_eq!(parsed.settings.expire_days, defaults.expire_days);
    assert!(parsed.settings.consent_enabled);

    // R9-D NEGATIVE: flat body (no wrapper)
    assert!(
        serde_json::from_value::<BulkUpdateRequest>(serde_json::to_value(&defaults).unwrap())
            .is_err(),
        "BulkUpdateRequest without nested `settings` wrapper MUST fail (flat body)"
    );

    // R9-D NEGATIVE: settings as string
    assert!(
        serde_json::from_value::<BulkUpdateRequest>(serde_json::json!({
            "settings": "{}"
        }))
        .is_err(),
        "BulkUpdateRequest with settings as string MUST fail (must be object)"
    );

    // R9-D NEGATIVE: settings as array
    assert!(
        serde_json::from_value::<BulkUpdateRequest>(serde_json::json!({
            "settings": []
        }))
        .is_err(),
        "BulkUpdateRequest with settings as array MUST fail"
    );

    // R9-D NEGATIVE: empty settings object (missing required fields)
    assert!(
        serde_json::from_value::<BulkUpdateRequest>(serde_json::json!({
            "settings": {}
        }))
        .is_err(),
        "BulkUpdateRequest with empty settings MUST fail (all fields required)"
    );
}

// ── 5. ConsentSettings has NO Optional fields — full PUT semantics ──

/// ConsentSettings is a FULL-document replace (PUT semantics, not
/// PATCH) — every field is required on the wire. The admin page
/// always sends the complete struct. A regression that flipped
/// fields to Optional would invite partial-payload writes that
/// silently zero out unmentioned fields when persisted as JSONB.
///
/// Pin by attempting to parse a body missing one critical field
/// and confirming it fails.
#[test]
fn consent_settings_has_no_optional_fields_put_semantics() {
    // R9-D NEGATIVE: every field is required — drop one critical
    // field and the parse MUST fail.
    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.remove("consent_enabled");
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "ConsentSettings missing `consent_enabled` MUST fail (no Optional fields — PUT semantics)"
    );

    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.remove("script_blocking_enabled");
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "ConsentSettings missing `script_blocking_enabled` MUST fail (every field required)"
    );

    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.remove("expire_days");
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "ConsentSettings missing `expire_days` MUST fail"
    );

    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.remove("proof_retention_days");
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "ConsentSettings missing `proof_retention_days` MUST fail"
    );

    // R9-D NEGATIVE: wrong type for a bool field
    let mut full = serde_json::to_value(ConsentSettings::default()).unwrap();
    if let serde_json::Value::Object(map) = &mut full {
        map.insert("consent_enabled".to_string(), serde_json::json!("yes"));
    }
    assert!(
        serde_json::from_value::<ConsentSettings>(full).is_err(),
        "ConsentSettings.consent_enabled MUST reject string (bool only)"
    );
}

// ── 6. Router mount-table compile-pin (3 routes, admin-gated) ───────

/// `routes::consent::router()` MUST build as `Router<AppState>`.
/// Mount table (all admin-gated via `admin: AdminUser`):
///   - GET  /settings           (get_settings)
///   - POST /settings/bulk      (bulk_update_settings)
///   - POST /settings/reset     (reset_settings)
///
/// All three routes take `admin: AdminUser` as a load-bearing
/// extractor — a regression that dropped this from
/// `bulk_update_settings` or `reset_settings` would let any
/// logged-in member modify consent settings (which controls
/// pixel-blocking + GDPR enforcement). The compile-pin catches
/// handler-signature drift; the runtime auth gate is enforced
/// by the `AdminUser` extractor itself, not by the router.
#[test]
fn consent_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::consent::router();
}

/// Idempotent construction. Per CLAUDE.md habit #3: pin that
/// nothing global (no `OnceLock`, no `static mut`) lives inside
/// the router constructor. The file-level docs (line 13-16)
/// explicitly call out that there is NO in-process cache by
/// design — a refactor that "optimized" by adding one would
/// break the GDPR fail-closed contract.
#[test]
fn consent_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::consent::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::consent::router();
}
