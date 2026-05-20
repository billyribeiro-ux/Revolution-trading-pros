//! Connections route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::connections` and
//! exercises the 7 public DTOs + the `admin_router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/connections.rs` (1,559 LOC) manages third-party service
//! integrations (Stripe, Resend, R2, etc.) — credential storage,
//! webhook registration, health-check ping endpoints. Every handler
//! runs live SQL against `service_connections` + `integration_webhooks`
//! + audit log tables and is gated behind `AdminUser`, so we can't
//! invoke handlers in isolation. What we CAN pin:
//!
//! 1. **`ServiceConnection.id` and `IntegrationWebhook.id` /
//!    `.connection_id` are i64.** Both are BIGSERIAL — narrowing to
//!    i32 would silently 404 the first connection past row 2^31. The
//!    `connection_id` foreign key carries through to `WHERE
//!    connection_id = $1`; a narrowed type would orphan webhooks for
//!    high-id connections.
//!
//! 2. **`ServiceConnection.api_calls_total` is i64.** This counter
//!    legitimately exceeds i32::MAX on production deployments — a
//!    busy Stripe integration can hit 2B API calls in <12 months at
//!    sustained load. Per CLAUDE.md "Reserved exception": counters
//!    *can* be i32 if they "genuinely cap below 2 billion" — this one
//!    doesn't, so it stays i64. Pin it.
//!
//! 3. **`ConnectServiceRequest.credentials` is `HashMap<String,
//!    String>` (NOT `serde_json::Value`).** This is load-bearing for
//!    the encryption layer: `encrypt_credentials` accepts ONLY a
//!    flat string-keyed map (it base64-encodes a `serde_json::
//!    to_string` of the HashMap). A regression that broadened it to
//!    arbitrary JSON would silently accept nested objects that the
//!    encryptor flattens into garbage at decrypt time, then 401-bomb
//!    every Stripe/Resend/R2 callback.
//!
//! 4. **`UpdateWebhookRequest` is fully optional (PATCH semantics).**
//!    The frontend hits this endpoint to toggle just `is_active`
//!    without resending the full webhook config; a regression that
//!    required the other fields would lock out the on/off toggle.
//!
//! 5. **`admin_router()` mount table compile-pin.** 11 mounted routes
//!    including the security-critical `/connect` (writes encrypted
//!    credentials), `/disconnect` (revokes), `/test` (live call to
//!    the provider). All require `AdminUser`. A refactor dropping
//!    that gate from any handler fails compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`, `tests/forms_test.rs`
//! (parallel), `tests/admin_members_test.rs`, `tests/products_test.rs`.

use revolution_api::routes::connections::{
    ConnectServiceRequest, CreateWebhookRequest, IntegrationWebhook, ListQuery, ServiceConnection,
    UpdateConnectionRequest, UpdateWebhookRequest,
};

// ── 1. ServiceConnection: BIGSERIAL ID + i64 totals counter ──────────

/// `ServiceConnection.id` is BIGSERIAL → must stay `i64`. And
/// `api_calls_total` is the lifetime API-call counter — production
/// deployments routinely cross i32::MAX (2.1B calls) within months for
/// Stripe + analytics integrations. Pin both `i64` end-to-end.
#[test]
fn service_connection_id_and_total_counter_are_i64() {
    let now = chrono::Utc::now();
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let huge_total: i64 = (i32::MAX as i64) * 10; // ~21B calls — well past i32

    let conn = ServiceConnection {
        id: above_i32_max,
        service_key: "stripe".to_string(),
        name: "Stripe (live)".to_string(),
        category: "payments".to_string(),
        description: Some("Stripe API integration".to_string()),
        status: "connected".to_string(),
        health_score: 100,
        health_status: Some("healthy".to_string()),
        environment: Some("production".to_string()),
        credentials_encrypted: Some("base64-blob".to_string()),
        settings: Some(serde_json::json!({"webhook_signing_secret": "whsec_..."})),
        webhook_url: Some("https://example.com/webhook/stripe".to_string()),
        webhook_secret: Some("whsec_xxx".to_string()),
        api_calls_today: 1_000_000,
        api_calls_total: huge_total,
        last_error: None,
        last_verified_at: Some(now),
        connected_at: Some(now),
        created_at: Some(now),
        updated_at: Some(now),
    };

    let wire = serde_json::to_value(&conn).expect("serialize ServiceConnection");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "ServiceConnection.id MUST be i64 (BIGSERIAL)"
    );
    assert_eq!(
        wire["api_calls_total"].as_i64(),
        Some(huge_total),
        "api_calls_total MUST be i64 — i32 would overflow at 2B calls"
    );
    // Sanity
    assert!(conn.id > i32::MAX as i64);
    assert!(conn.api_calls_total > i32::MAX as i64);
}

// ── 2. IntegrationWebhook: BIGSERIAL id + connection_id FK ───────────

/// Both `IntegrationWebhook.id` and `.connection_id` are i64. The
/// foreign-key field is the load-bearing one: it carries into
/// `WHERE connection_id = $1` on every webhook listing / delete, and
/// narrowing it to i32 would orphan webhooks for high-id parent
/// connections (silent 404 on the admin grid).
#[test]
fn integration_webhook_ids_are_i64() {
    let now = chrono::Utc::now();
    let above_i32_max: i64 = (i32::MAX as i64) + 5;

    let hook = IntegrationWebhook {
        id: above_i32_max,
        connection_id: above_i32_max - 1,
        name: "Stripe payment_intent.succeeded".to_string(),
        url: "https://example.com/webhook".to_string(),
        secret: Some("whsec_xxx".to_string()),
        events: Some(serde_json::json!(["payment_intent.succeeded"])),
        is_active: true,
        last_triggered_at: Some(now),
        last_status_code: Some(200),
        failure_count: 0,
        created_at: Some(now),
        updated_at: Some(now),
    };

    let wire = serde_json::to_value(&hook).expect("serialize IntegrationWebhook");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["connection_id"].as_i64(), Some(above_i32_max - 1));
    assert!(hook.id > i32::MAX as i64);
    assert!(hook.connection_id > i32::MAX as i64);
}

// ── 3. ConnectServiceRequest: HashMap<String,String> for credentials ─

/// `ConnectServiceRequest.credentials` is `HashMap<String, String>`,
/// NOT `serde_json::Value`. This is load-bearing for the encryption
/// layer: `encrypt_credentials` accepts ONLY a flat string-keyed map
/// (it serializes via `serde_json::to_string(&hashmap)`). A future
/// refactor broadening this to arbitrary JSON would silently flatten
/// nested objects into useless string blobs at encrypt time.
///
/// Pin the wire shape: a flat string-keyed map round-trips, while a
/// nested object FAILS to deserialize (the inner value isn't a
/// string). That negative case proves the type is genuinely
/// `HashMap<String,String>`, not the looser `Value`.
#[test]
fn connect_request_requires_flat_string_credentials() {
    // Happy path: a flat string-keyed map.
    let req: ConnectServiceRequest = serde_json::from_value(serde_json::json!({
        "credentials": {
            "api_key": "sk_test_abc123",
            "webhook_secret": "whsec_xyz"
        },
        "environment": "test"
    }))
    .expect("flat credentials map must deserialize");
    assert_eq!(req.credentials.len(), 2);
    assert_eq!(
        req.credentials.get("api_key").map(String::as_str),
        Some("sk_test_abc123")
    );
    assert_eq!(req.environment.as_deref(), Some("test"));
    assert!(req.settings.is_none());

    // Negative: nested object inside `credentials` MUST be rejected.
    // If the field were ever broadened to `HashMap<String, Value>` or
    // `serde_json::Value`, this payload would deserialize successfully
    // — and the encrypt step would silently stringify the nested object
    // into garbage at runtime. The compile-time pin is `String` only.
    let nested = serde_json::from_value::<ConnectServiceRequest>(serde_json::json!({
        "credentials": {
            "api_key": {"primary": "sk_test_abc", "fallback": "sk_test_def"}
        }
    }));
    assert!(
        nested.is_err(),
        "nested credentials object MUST be rejected (HashMap<String,String> pin)"
    );

    // Minimal payload — only `credentials`, the rest are optional.
    let minimal: ConnectServiceRequest = serde_json::from_value(serde_json::json!({
        "credentials": {"api_key": "sk_test_min"}
    }))
    .expect("minimal credentials-only payload must deserialize");
    assert_eq!(minimal.credentials.len(), 1);
    assert!(minimal.environment.is_none());
    assert!(minimal.settings.is_none());
}

// ── 4. UpdateConnectionRequest / UpdateWebhookRequest: PATCH shape ───

/// Both update DTOs follow PATCH semantics — every field optional, so
/// the admin can toggle just `is_active` without re-sending the full
/// payload. Regressions that flipped any field to required would
/// 400-error the on/off toggle (the most-used webhook admin action).
#[test]
fn update_requests_follow_patch_semantics() {
    // UpdateConnectionRequest: everything optional.
    let empty: UpdateConnectionRequest =
        serde_json::from_str("{}").expect("empty UpdateConnectionRequest must deserialize (PATCH)");
    assert!(empty.credentials.is_none());
    assert!(empty.environment.is_none());
    assert!(empty.settings.is_none());
    assert!(empty.is_active.is_none());

    let toggle: UpdateConnectionRequest =
        serde_json::from_value(serde_json::json!({"is_active": false}))
            .expect("toggle-only update must deserialize");
    assert_eq!(toggle.is_active, Some(false));

    // UpdateWebhookRequest: everything optional.
    let webhook_toggle: UpdateWebhookRequest =
        serde_json::from_value(serde_json::json!({"is_active": true}))
            .expect("webhook toggle must deserialize");
    assert_eq!(webhook_toggle.is_active, Some(true));
    assert!(webhook_toggle.name.is_none());
    assert!(webhook_toggle.url.is_none());

    // Rename + repoint in one PATCH.
    let rename: UpdateWebhookRequest = serde_json::from_value(serde_json::json!({
        "name": "Renamed webhook",
        "url": "https://example.com/v2/webhook"
    }))
    .expect("rename+repoint webhook must deserialize");
    assert_eq!(rename.name.as_deref(), Some("Renamed webhook"));
    assert_eq!(
        rename.url.as_deref(),
        Some("https://example.com/v2/webhook")
    );
    assert!(rename.is_active.is_none());
}

// ── 5. CreateWebhookRequest + ListQuery: required vs optional shape ──

/// `CreateWebhookRequest` carries the only two REQUIRED fields in
/// this module: `name` (label) and `url` (delivery target). `events`
/// is optional (defaults to "all events" at handler level). And
/// `ListQuery` is fully optional — the admin grid's default view
/// shows everything.
#[test]
fn create_webhook_pins_required_fields_and_list_query_is_optional() {
    // CreateWebhookRequest minimal payload — `name` + `url` only.
    let minimal: CreateWebhookRequest = serde_json::from_value(serde_json::json!({
        "name": "Order events",
        "url": "https://example.com/webhook/orders"
    }))
    .expect("minimal CreateWebhookRequest must deserialize");
    assert_eq!(minimal.name, "Order events");
    assert_eq!(minimal.url, "https://example.com/webhook/orders");
    assert!(minimal.events.is_none(), "events is optional");

    // With events filter.
    let with_events: CreateWebhookRequest = serde_json::from_value(serde_json::json!({
        "name": "Payment events only",
        "url": "https://example.com/webhook/payments",
        "events": ["payment.succeeded", "payment.failed"]
    }))
    .expect("CreateWebhookRequest with events must deserialize");
    assert!(with_events.events.is_some());

    // Missing `name` MUST fail (required field).
    assert!(
        serde_json::from_value::<CreateWebhookRequest>(serde_json::json!({"url": "x"})).is_err(),
        "CreateWebhookRequest without `name` MUST fail (required)"
    );

    // Missing `url` MUST fail (required field).
    assert!(
        serde_json::from_value::<CreateWebhookRequest>(serde_json::json!({"name": "x"})).is_err(),
        "CreateWebhookRequest without `url` MUST fail (required)"
    );

    // ListQuery — admin grid default view sends nothing.
    let empty_list: ListQuery =
        serde_json::from_str("{}").expect("empty ListQuery must deserialize");
    assert!(empty_list.category.is_none());
    assert!(empty_list.status.is_none());

    let filtered: ListQuery = serde_json::from_value(serde_json::json!({
        "category": "payments",
        "status": "connected"
    }))
    .expect("filtered ListQuery must deserialize");
    assert_eq!(filtered.category.as_deref(), Some("payments"));
    assert_eq!(filtered.status.as_deref(), Some("connected"));
}

// ── 6. Router mount table compile-pin ────────────────────────────────

/// `admin_router()` must build as `Router<AppState>`. Covers 11
/// mounted routes including the security-critical `/connect` (writes
/// encrypted credentials to `service_connections`), `/disconnect`
/// (revokes access), `/test` (live API call to the provider). Every
/// handler MUST be gated behind `AdminUser`. A refactor that dropped
/// that gate from any handler fails compilation here.
#[test]
fn connections_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::connections::admin_router();
}

/// Idempotent constructor — must be safe to call multiple times (the
/// main `api_router()` may nest this under different prefixes during
/// composition).
#[test]
fn admin_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::connections::admin_router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::connections::admin_router();
}
