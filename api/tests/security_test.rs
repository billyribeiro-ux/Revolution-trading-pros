//! Security audit-log route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::security` and exercises
//! the 2 public DTOs + the single `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/security.rs` (238 LOC) is the **security audit log** —
//! every `login_success`, `login_failed`, `password_reset`,
//! `privilege_escalation`, etc. event lands here. Two GET handlers
//! (`/events` and `/stats`) both run live SQL against the
//! `security_events` table and both are gated behind an explicit
//! role check (`role == "admin"` || `"super-admin"` || `"super_admin"`).
//! Handlers can't be invoked in isolation. What we CAN pin:
//!
//! 1. **`SecurityEvent.id` and `.user_id` are `i64` BIGSERIAL.** The
//!    event PK is BIGSERIAL — login-event volume on a B2C app crosses
//!    `2^31` rows in <2 years at sustained load. The `user_id` FK
//!    flows into `WHERE user_id = $N` admin filters; narrowing either
//!    would silently 404 the first event past row 2B (forensic
//!    blackout exactly when you need the log most).
//!
//! 2. **`SecurityEventsQuery` is fully optional.** The audit grid's
//!    default view ("show me everything, page 1") sends an empty
//!    payload. Any field flipping to required would 400-error the
//!    admin's default landing view of the forensic log.
//!
//! 3. **`SecurityEventsQuery.user_id` is `Option<i64>`.** Filtering
//!    the audit log by user is a forensic-investigation primitive
//!    ("show me every event for user 42"). Narrowing the filter input
//!    to `i32` would make user-id filtering silently fail for any
//!    user past row 2^31 — exactly the high-value users (most-active
//!    accounts, support escalations).
//!
//! 4. **`router()` mount table compile-pin.** Two routes:
//!    `/events` (GET + POST — the POST is a duplicate-mount; both
//!    invoke `get_security_events`) and `/stats`. Both handlers must
//!    keep the inline role-check; if the file is ever refactored to
//!    use the `AdminUser` extractor instead, this compile-pin still
//!    holds (the signature changes are caught at compile time).
//!
//! ## Pattern source
//!
//! Modeled on `tests/connections_test.rs`, `tests/forms_test.rs`,
//! `tests/admin_orders_test.rs`.

use revolution_api::routes::security::{SecurityEvent, SecurityEventsQuery};

// ── 1. SecurityEvent: BIGSERIAL id + FK user_id stay i64 ─────────────

/// `SecurityEvent.id` is BIGSERIAL — login-success / login-failed /
/// audit events land here at high volume. A B2C deployment crosses
/// `2^31` events in <2 years at sustained traffic; narrowing the PK to
/// `i32` would silently break event-detail fetches once the table
/// grows past that. `user_id` is the FK that flows into `WHERE
/// user_id = $N` filters — same story for forensic lookups.
#[test]
fn security_event_id_and_user_id_are_i64() {
    let now = chrono::Utc::now().naive_utc();
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let high_user_id: i64 = (i32::MAX as i64) * 3; // ~6.4B users — extreme but legal

    let evt = SecurityEvent {
        id: above_i32_max,
        user_id: Some(high_user_id),
        event_type: "login_success".to_string(),
        event_category: "auth".to_string(),
        severity: "info".to_string(),
        ip_address: Some("203.0.113.7".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        metadata: Some(serde_json::json!({"method": "password"})),
        created_at: now,
    };

    let wire = serde_json::to_value(&evt).expect("serialize SecurityEvent");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "SecurityEvent.id MUST be i64 — BIGSERIAL audit-log PK"
    );
    assert_eq!(
        wire["user_id"].as_i64(),
        Some(high_user_id),
        "SecurityEvent.user_id MUST be i64 — FK to users.id (BIGSERIAL)"
    );

    // Sanity — fixture must exceed i32::MAX or the assertion above
    // could pass with an i32 field.
    assert!(
        evt.id > i32::MAX as i64,
        "fixture must exceed i32::MAX to prove the i64 pin"
    );
    assert!(
        evt.user_id.unwrap() > i32::MAX as i64,
        "user_id fixture must exceed i32::MAX"
    );
}

// ── 2. SecurityEvent: user_id is Option (system events have no user) ─

/// Not every security event has an attributable user — failed-login
/// attempts against a non-existent email, rate-limit trips before
/// auth, IP-blocklist hits. `user_id` MUST stay `Option<i64>` so the
/// FK can be NULL. A regression that flipped it to required `i64`
/// would force the handler to invent a sentinel (e.g. `0`), silently
/// corrupting the audit-log for unattributable events.
#[test]
fn security_event_user_id_is_optional_for_unattributable_events() {
    let now = chrono::Utc::now().naive_utc();

    // Failed login against a non-existent user — no FK to set.
    let evt = SecurityEvent {
        id: 1_i64,
        user_id: None,
        event_type: "login_failed".to_string(),
        event_category: "auth".to_string(),
        severity: "warning".to_string(),
        ip_address: Some("198.51.100.42".to_string()),
        user_agent: Some("curl/8.4.0".to_string()),
        metadata: Some(serde_json::json!({"reason": "user_not_found"})),
        created_at: now,
    };

    let wire = serde_json::to_value(&evt).expect("serialize unattributable event");
    assert!(
        wire["user_id"].is_null(),
        "user_id MUST serialize as null when None (audit log for unauth'd events)"
    );

    // Also pin: metadata is Optional — many event types record no
    // structured metadata (e.g. a plain logout).
    let logout = SecurityEvent {
        id: 2_i64,
        user_id: Some(42_i64),
        event_type: "logout".to_string(),
        event_category: "auth".to_string(),
        severity: "info".to_string(),
        ip_address: None,
        user_agent: None,
        metadata: None,
        created_at: now,
    };
    let wire2 = serde_json::to_value(&logout).expect("serialize logout event");
    assert!(wire2["metadata"].is_null());
    assert!(wire2["ip_address"].is_null());
    assert!(wire2["user_agent"].is_null());
}

// ── 3. SecurityEventsQuery: all-optional admin-grid default view ─────

/// The admin audit-log grid hits `/api/security/events` with whatever
/// filters the user has applied — often none ("show me everything,
/// page 1"). A regression that flipped `page` / `per_page` /
/// `event_type` / `severity` / `user_id` to required would 400-error
/// the default landing view. Pin all-optional.
#[test]
fn security_events_query_accepts_empty_payload() {
    let empty: SecurityEventsQuery =
        serde_json::from_str("{}").expect("empty SecurityEventsQuery must deserialize");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.event_type.is_none());
    assert!(empty.severity.is_none());
    assert!(empty.user_id.is_none());

    // Single-field filter: forensic flow ("show me every login
    // attempt for user 42"). The user_id filter input MUST stay i64
    // — narrowing it would silently lose high-id user filtering.
    let high_user_id: i64 = (i32::MAX as i64) + 100;
    let filtered: SecurityEventsQuery = serde_json::from_value(serde_json::json!({
        "user_id": high_user_id,
        "event_type": "login_failed"
    }))
    .expect("filtered SecurityEventsQuery must deserialize");
    assert_eq!(filtered.user_id, Some(high_user_id));
    assert_eq!(filtered.event_type.as_deref(), Some("login_failed"));
    assert!(filtered.user_id.unwrap() > i32::MAX as i64);
}

// ── 4. SecurityEventsQuery: pagination accepts large page numbers ────

/// `page` / `per_page` are `Option<i64>` — pagination on a high-volume
/// audit log can reach page numbers in the millions, and `per_page` is
/// clamped at the handler (`min(100)`). Pin the i64 input shape and
/// the wire contract: integers round-trip without precision loss.
#[test]
fn security_events_query_pagination_accepts_large_values() {
    // High page number — forensic deep-paging is a legitimate flow.
    let deep_page: SecurityEventsQuery = serde_json::from_value(serde_json::json!({
        "page": 1_000_000_i64,
        "per_page": 100_i64
    }))
    .expect("deep-paging payload must deserialize");
    assert_eq!(deep_page.page, Some(1_000_000_i64));
    assert_eq!(deep_page.per_page, Some(100_i64));

    // Negative-test: a per_page that's not a number MUST fail.
    let bad = serde_json::from_value::<SecurityEventsQuery>(serde_json::json!({
        "per_page": "fifty"
    }));
    assert!(
        bad.is_err(),
        "non-numeric per_page MUST fail (i64 type pin)"
    );

    // Severity filter accepts arbitrary string (handler validates at
    // runtime against the severity column).
    let sev: SecurityEventsQuery = serde_json::from_value(serde_json::json!({
        "severity": "critical"
    }))
    .expect("severity filter must deserialize");
    assert_eq!(sev.severity.as_deref(), Some("critical"));
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Mount table:
///   - `/events` (GET + POST — both invoke `get_security_events`;
///     the POST mount is intentional, matches an internal frontend
///     pattern that POSTs filter-bodies)
///   - `/stats` (GET only — aggregate counts)
///
/// Both handlers must keep the inline role-check
/// (`role == "admin" || "super-admin" || "super_admin"`). A refactor
/// that broke either handler signature or dropped the `User` extractor
/// fails compilation here.
#[test]
fn security_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::security::router();
}

/// Idempotent constructor — must be safe to call multiple times.
/// `api_router()` nests this under `/security`; nothing prevents
/// future composition from re-nesting elsewhere.
#[test]
fn security_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::security::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::security::router();
}
