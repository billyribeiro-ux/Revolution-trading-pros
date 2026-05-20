//! Newsletter route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::newsletter` and
//! exercises the 6 public DTOs + the 3 mounted routers
//! (`public_router()`, `admin_router()`, `router()` combined).
//!
//! ## Why this shape
//!
//! `routes/newsletter.rs` (785 LOC) is the **public newsletter
//! subscription surface** — GDPR-compliant subscribe/confirm/unsub
//! flow with HMAC-SHA256 tokens, plus admin endpoints for listing
//! subscribers, stats, and bulk-send. Every handler runs live SQL
//! against `newsletter_subscribers` (+ outbound email when wired);
//! tokens are HMAC-signed against an env-var secret. Handlers can't
//! be invoked in isolation. What we CAN pin:
//!
//! 1. **`SubscriberRow.id` is `i64` BIGSERIAL.** Newsletter
//!    subscribers grow fast on a B2C site (every blog reader, every
//!    promo signup). Crossing `2^31` is realistic over the product
//!    lifetime. The PK flows into the HMAC token payload (the token
//!    encodes `subscriber_id:email`) — narrowing to `i32` would
//!    silently 404 unsubscribe links for high-id users (lock them
//!    into the list permanently — GDPR violation).
//!
//! 2. **`SubscriberRow.gdpr_consent` is bool (NOT NULL).** GDPR
//!    audit trail requires a concrete yes/no consent record. The DB
//!    schema uses `COALESCE(gdpr_consent, false)` to surface NULL as
//!    false; the Rust type stays `bool` so consumers can't accidentally
//!    treat "not recorded" as "consented".
//!
//! 3. **`SubscribeRequest.gdpr_consent` is `Option<bool>` (NOT
//!    required).** The handler explicitly checks `unwrap_or(false)`
//!    and rejects with 400 if not `true`. A regression that flipped
//!    it to required `bool` would be MORE strict (rejecting payloads
//!    that didn't include the field at all) but would also reject
//!    legacy SDK clients silently — the current shape is "optional
//!    field that defaults to false and triggers a guard".
//!
//! 4. **`ConfirmQuery` / `UnsubscribeQuery` both require `token`.**
//!    These are the GDPR-required double-opt-in and unsubscribe
//!    flows. The token is HMAC-signed and verified at the handler;
//!    the DTO MUST require `token` (you cannot confirm or unsub
//!    without a token — silently allowing a tokenless unsub would
//!    let any party unsubscribe any user).
//!
//! 5. **`BulkEmailRequest.subject` + `html_content` required;
//!    everything else optional.** The bulk-send admin endpoint sends
//!    HTML emails to a segment of subscribers. A regression that
//!    flipped `subject` or `html_content` to optional would let an
//!    admin send empty emails (UX disaster + spam-classifier hits).
//!    Pin the required shape.
//!
//! 6. **`BulkEmailRequest.template_id` is `Option<i64>`.** Templates
//!    are stored in `email_templates` (separate module); the FK is
//!    BIGSERIAL → i64. Narrowing here would break the
//!    "send-from-template" path for high-id templates.
//!
//! 7. **Three router constructors compile-pin.** `public_router()`
//!    (unauthenticated subscribe/confirm/unsubscribe — the highest-
//!    blast-radius public mount), `admin_router()` (admin grid +
//!    bulk-send — must gate on AdminUser), and `router()` (the
//!    backwards-compatible combined mount, used by `api_router()`
//!    at the `/newsletter` prefix).
//!
//! ## Pattern source
//!
//! Modeled on `tests/forms_test.rs` (parallel — both have public +
//! admin routers), `tests/connections_test.rs`, `tests/oauth_test.rs`
//! (parallel — both manipulate HMAC tokens at the handler layer).

use revolution_api::routes::newsletter::{
    BulkEmailRequest, ConfirmQuery, SubscribeRequest, SubscriberListQuery, SubscriberRow,
    UnsubscribeQuery,
};

// ── 1. SubscriberRow: BIGSERIAL id is i64 + GDPR fields wire-shape ──

/// `SubscriberRow.id` is BIGSERIAL — the PK flows into the HMAC token
/// payload (`generate_secure_token(subscriber_id: i64, ...)`).
/// Narrowing to `i32` would silently 404 unsubscribe links for high-id
/// subscribers — they couldn't unsubscribe, which is a GDPR
/// compliance violation. Pin i64 end-to-end.
#[test]
fn subscriber_row_id_is_i64_and_gdpr_fields_serialize() {
    let now = chrono::Utc::now().naive_utc();
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let row = SubscriberRow {
        id: above_i32_max,
        email: "jane@example.com".to_string(),
        name: Some("Jane Doe".to_string()),
        status: "confirmed".to_string(),
        source: Some("blog-footer".to_string()),
        ip_address: Some("203.0.113.7".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        tags: Some(serde_json::json!(["weekly", "promo"])),
        metadata: Some(serde_json::json!({"signup_page": "/blog/intro"})),
        gdpr_consent: true,
        consent_ip: Some("203.0.113.7".to_string()),
        consent_source: Some("blog-footer".to_string()),
        confirmed_at: Some(now),
        unsubscribed_at: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize SubscriberRow");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "SubscriberRow.id MUST be i64 — feeds the HMAC unsubscribe-token payload"
    );
    assert_eq!(
        wire["gdpr_consent"],
        serde_json::json!(true),
        "gdpr_consent MUST be bool (NOT NULL — audit-trail requires concrete y/n)"
    );
    assert_eq!(
        wire["consent_ip"],
        serde_json::json!("203.0.113.7"),
        "consent_ip is optional but populated when consent was given"
    );

    // Sanity — fixture must exceed i32::MAX or the i64 pin above
    // could pass with an i32 field.
    assert!(row.id > i32::MAX as i64);
}

// ── 2. SubscriberRow: unsubscribed timestamp pattern ─────────────────

/// Unsubscribed subscribers stay in the table (GDPR audit trail —
/// "we honored their unsub request, here's when") with
/// `unsubscribed_at: Some(when)` and `status: "unsubscribed"`.
/// `confirmed_at` may also be populated (they previously confirmed,
/// then later unsubscribed). Pin both Optional timestamps so the
/// admin grid can show the full history.
#[test]
fn subscriber_row_unsubscribe_state_serializes_full_history() {
    let confirmed_at = chrono::Utc::now().naive_utc() - chrono::Duration::days(30);
    let unsubscribed_at = chrono::Utc::now().naive_utc();

    let row = SubscriberRow {
        id: 42_i64,
        email: "former@example.com".to_string(),
        name: None,
        status: "unsubscribed".to_string(),
        source: Some("promo-popup".to_string()),
        ip_address: None,
        user_agent: None,
        tags: None,
        metadata: None,
        gdpr_consent: true,
        consent_ip: None,
        consent_source: None,
        confirmed_at: Some(confirmed_at),
        unsubscribed_at: Some(unsubscribed_at),
        created_at: confirmed_at,
        updated_at: unsubscribed_at,
    };

    let wire = serde_json::to_value(&row).expect("serialize unsubscribed row");
    assert!(
        wire["confirmed_at"].is_string(),
        "confirmed_at populated for previously-confirmed subscribers"
    );
    assert!(
        wire["unsubscribed_at"].is_string(),
        "unsubscribed_at populated post-unsubscribe (audit trail)"
    );
    assert_eq!(wire["status"], serde_json::json!("unsubscribed"));

    // A never-confirmed pending subscriber: both timestamps null.
    let pending = SubscriberRow {
        id: 43_i64,
        email: "pending@example.com".to_string(),
        name: None,
        status: "pending".to_string(),
        source: None,
        ip_address: None,
        user_agent: None,
        tags: None,
        metadata: None,
        gdpr_consent: true,
        consent_ip: None,
        consent_source: None,
        confirmed_at: None,
        unsubscribed_at: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    let wire2 = serde_json::to_value(&pending).expect("serialize pending row");
    assert!(wire2["confirmed_at"].is_null());
    assert!(wire2["unsubscribed_at"].is_null());
}

// ── 3. SubscribeRequest: email required, gdpr_consent optional bool ──

/// `SubscribeRequest.email` is the only REQUIRED field — the
/// handler then validates format (RFC 5322-ish) and lowercases.
/// `gdpr_consent` is `Option<bool>` — the handler checks
/// `unwrap_or(false)` and rejects with 400 if not `true`. The
/// optional-vs-required-vs-required-true distinction matters:
///
///   - REQUIRED bool: legacy SDK clients (no consent field) fail at
///     deserialize time — 400 with "missing field"
///   - OPTIONAL bool (current): legacy clients pass deserialize then
///     fail at the explicit guard — 400 with the proper
///     "GDPR consent is required" message + field hint
///
/// The current shape is more user-friendly. Pin it.
#[test]
fn subscribe_request_email_required_consent_optional() {
    // Minimal: just email (handler will then reject for missing consent).
    let minimal: SubscribeRequest = serde_json::from_value(serde_json::json!({
        "email": "new@example.com"
    }))
    .expect("minimal SubscribeRequest must deserialize");
    assert_eq!(minimal.email, "new@example.com");
    assert!(minimal.name.is_none());
    assert!(minimal.source.is_none());
    assert!(minimal.tags.is_none());
    assert!(
        minimal.gdpr_consent.is_none(),
        "gdpr_consent absent — handler will reject with 400 + helpful message"
    );

    // Full happy-path payload.
    let full: SubscribeRequest = serde_json::from_value(serde_json::json!({
        "email": "jane@example.com",
        "name": "Jane Doe",
        "source": "blog-footer",
        "tags": ["weekly", "promo"],
        "gdpr_consent": true
    }))
    .expect("full SubscribeRequest must deserialize");
    assert_eq!(full.email, "jane@example.com");
    assert_eq!(full.name.as_deref(), Some("Jane Doe"));
    assert_eq!(full.gdpr_consent, Some(true));
    assert_eq!(full.tags.as_ref().map(|t| t.len()), Some(2));

    // Missing `email` MUST fail (required).
    assert!(
        serde_json::from_value::<SubscribeRequest>(serde_json::json!({"name": "x"})).is_err(),
        "SubscribeRequest without `email` MUST fail (required)"
    );

    // `gdpr_consent: false` MUST deserialize cleanly — handler enforces
    // the truthiness check, not the DTO.
    let no_consent: SubscribeRequest = serde_json::from_value(serde_json::json!({
        "email": "x@example.com",
        "gdpr_consent": false
    }))
    .expect("explicit false consent must deserialize (handler rejects at runtime)");
    assert_eq!(no_consent.gdpr_consent, Some(false));
}

// ── 4. ConfirmQuery + UnsubscribeQuery: token required ───────────────

/// Both confirm and unsubscribe flows require an HMAC-signed token.
/// The DTO MUST require the `token` field — a missing-token payload
/// landing at the handler with `Option<String>` would risk a future
/// refactor accidentally allowing tokenless unsubscribe (lets any
/// party unsubscribe any user — security incident).
#[test]
fn confirm_and_unsubscribe_queries_require_token() {
    let confirm: ConfirmQuery = serde_json::from_value(serde_json::json!({
        "token": "abc.signature"
    }))
    .expect("ConfirmQuery with token must deserialize");
    assert_eq!(confirm.token, "abc.signature");

    let unsub: UnsubscribeQuery = serde_json::from_value(serde_json::json!({
        "token": "def.signature",
        "reason": "Too many emails"
    }))
    .expect("UnsubscribeQuery with token + reason must deserialize");
    assert_eq!(unsub.token, "def.signature");
    assert_eq!(unsub.reason.as_deref(), Some("Too many emails"));

    // Missing token MUST fail for confirm.
    assert!(
        serde_json::from_value::<ConfirmQuery>(serde_json::json!({})).is_err(),
        "ConfirmQuery without `token` MUST fail (security gate)"
    );

    // Missing token MUST fail for unsubscribe.
    assert!(
        serde_json::from_value::<UnsubscribeQuery>(serde_json::json!({"reason": "x"})).is_err(),
        "UnsubscribeQuery without `token` MUST fail (security gate)"
    );

    // Reason is optional on unsubscribe.
    let no_reason: UnsubscribeQuery = serde_json::from_value(serde_json::json!({
        "token": "xyz.sig"
    }))
    .expect("UnsubscribeQuery without reason must deserialize");
    assert!(no_reason.reason.is_none());
}

// ── 5. BulkEmailRequest: subject + html_content required, template_id i64 ──

/// `BulkEmailRequest` is the admin's bulk-send endpoint. `subject`
/// and `html_content` MUST stay required — letting an admin send
/// empty emails is a UX + spam-classifier disaster. `template_id` is
/// `Option<i64>` — the FK to `email_templates.id` (BIGSERIAL). A
/// regression narrowing it to `i32` would break the
/// "send-from-template" path for high-id templates (long-running
/// products accumulate template versions fast).
#[test]
fn bulk_email_request_required_fields_and_template_id_is_i64() {
    // Minimal required-only payload.
    let minimal: BulkEmailRequest = serde_json::from_value(serde_json::json!({
        "subject": "Weekly digest 2026-05-20",
        "html_content": "<h1>This week in markets</h1>"
    }))
    .expect("minimal BulkEmailRequest must deserialize");
    assert_eq!(minimal.subject, "Weekly digest 2026-05-20");
    assert!(minimal.template_id.is_none());
    assert!(minimal.text_content.is_none());
    assert!(minimal.segment.is_none());
    assert!(minimal.rate_limit.is_none());

    // With template (FK to email_templates.id) past i32::MAX.
    let high_template_id: i64 = (i32::MAX as i64) + 11;
    let with_template: BulkEmailRequest = serde_json::from_value(serde_json::json!({
        "template_id": high_template_id,
        "subject": "Templated send",
        "html_content": "<p>From template</p>",
        "text_content": "From template",
        "segment": "confirmed",
        "rate_limit": 200
    }))
    .expect("BulkEmailRequest with high template_id must deserialize");
    assert_eq!(with_template.template_id, Some(high_template_id));
    assert!(
        with_template.template_id.unwrap() > i32::MAX as i64,
        "template_id MUST be i64 — i32 would silently truncate high template IDs"
    );
    assert_eq!(with_template.segment.as_deref(), Some("confirmed"));
    assert_eq!(with_template.rate_limit, Some(200));

    // Missing `subject` MUST fail.
    assert!(
        serde_json::from_value::<BulkEmailRequest>(serde_json::json!({"html_content": "<p>x</p>"}))
            .is_err(),
        "BulkEmailRequest without `subject` MUST fail (no empty-subject sends)"
    );

    // Missing `html_content` MUST fail.
    assert!(
        serde_json::from_value::<BulkEmailRequest>(serde_json::json!({"subject": "x"})).is_err(),
        "BulkEmailRequest without `html_content` MUST fail (no empty-body sends)"
    );
}

// ── 6. SubscriberListQuery: admin grid default view all-optional ─────

/// The admin subscribers grid lands on `/admin/newsletter/subscribers`
/// with no filters set. All-optional shape required so the default
/// "show me everything" landing view works. Pin it.
#[test]
fn subscriber_list_query_accepts_empty_and_filtered_payloads() {
    let empty: SubscriberListQuery =
        serde_json::from_str("{}").expect("empty SubscriberListQuery must deserialize");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.status.is_none());
    assert!(empty.search.is_none());

    let filtered: SubscriberListQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 100,
        "status": "confirmed",
        "search": "@example.com"
    }))
    .expect("filtered SubscriberListQuery must deserialize");
    assert_eq!(filtered.page, Some(2));
    assert_eq!(filtered.per_page, Some(100));
    assert_eq!(filtered.status.as_deref(), Some("confirmed"));
    assert_eq!(filtered.search.as_deref(), Some("@example.com"));
}

// ── 7. Router mount tables: all three constructors compile ───────────

/// `public_router()` must build as `Router<AppState>`. Mount table:
///   - `/subscribe` (POST → subscribe — UNAUTHENTICATED entry point)
///   - `/confirm` (GET → confirm — token-verified)
///   - `/unsubscribe` (GET → unsubscribe — token-verified)
///
/// The public mount is high-blast-radius — `/subscribe` is hit by
/// every newsletter signup widget on the site, no auth required.
/// A handler-signature regression here breaks every signup flow.
#[test]
fn newsletter_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::public_router();
}

/// `admin_router()` must build as `Router<AppState>`. Mount table:
///   - `/subscribers` (GET → list_subscribers)
///   - `/subscribers/:id` (DELETE → delete_subscriber)
///   - `/subscribers/:id/export` (GET → export_subscriber)
///   - `/stats` (GET → get_stats)
///   - `/bulk-send` (POST → send_bulk_email — HIGH BLAST RADIUS)
///
/// All MUST gate on `AdminUser`. The `bulk-send` endpoint is the
/// highest-blast-radius admin action in this domain — one POST sends
/// HTML email to every subscriber in the selected segment. Compile-pin
/// catches signature regressions.
#[test]
fn newsletter_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::admin_router();
}

/// `router()` is the backwards-compatible combined mount — both
/// public + admin routes under one prefix (`/newsletter` in
/// `api_router()`). Used to keep legacy URL shapes working during
/// the split-routers migration. MUST stay compatible with both
/// surfaces.
#[test]
fn newsletter_combined_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::newsletter::router();
}

/// Idempotent constructors — all three must be safe to call multiple
/// times. `api_router()` calls `newsletter::router()` once; nothing
/// prevents future composition from also calling `public_router()` /
/// `admin_router()` separately for a split-prefix migration.
#[test]
fn newsletter_routers_are_safe_to_construct_multiple_times() {
    let _p1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::public_router();
    let _p2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::public_router();
    let _a1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::admin_router();
    let _a2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::newsletter::admin_router();
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::newsletter::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::newsletter::router();
}
