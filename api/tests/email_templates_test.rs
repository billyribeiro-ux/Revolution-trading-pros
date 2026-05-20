//! Email templates route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::email_templates` and
//! exercises the public DTOs + the `admin_router()` mount table.
//! This is the **admin** email-template CRUD surface — every
//! transactional email the platform sends (welcome, receipt,
//! password reset) is rendered from a template stored through these
//! routes, so a drift in the wire-format keys silently breaks the
//! marketing team's editing UI.
//!
//! The task header asked for `email_test.rs` against `routes/email.rs`,
//! but no `email.rs` module exists in this repo — `email_templates`
//! is the closest match and the next 0-test high-value module.
//!
//! ## Why this shape
//!
//! Every handler runs live SQL against `email_templates`, plus an
//! outbound SMTP call from `send_test_email` (TODO-stub today). None
//! of that runs in unit-test isolation, so we attack the contract:
//!
//! 1. **`EmailTemplateRow.id` is `i64`** — same rule the rest of the
//!    backend uses for primary keys. Round-trips past `i32::MAX` to
//!    prove the i64 pin. (Email-template rows are low-volume in
//!    practice, but the pin matters because the rest of the schema
//!    consistently uses i64; a stray i32 here is a cross-table-join
//!    landmine.)
//!
//! 2. **Request DTO shapes** — `CreateTemplateRequest` requires
//!    `name`, `subject`, `body` (no defaults); `UpdateTemplateRequest`
//!    is entirely Optional for partial-update PATCH semantics; the
//!    `is_active.unwrap_or(true)` default in `create_template` is
//!    operator-visible — new templates are live by default. The
//!    contract: `is_active: Option<bool>` on the request DTO so
//!    "live by default" stays in handler land.
//!
//! 3. **`SendTestRequest.email` is a String** — and the
//!    `is_valid_email` private helper rejects malformed addresses
//!    before the handler attempts an SMTP send. We can't reach
//!    `is_valid_email` directly, but we DO exercise the request DTO
//!    contract.
//!
//! 4. **`PreviewRequest.data` is `serde_json::Value`** — arbitrary
//!    JSON for variable substitution. The `html_escape` step in the
//!    handler is the security-critical bit (XSS); the DTO contract
//!    is that any JSON shape parses.
//!
//! 5. **Money pin (documenting absence)** — `email_templates` is an
//!    admin CMS surface for email content; no monetary fields exist.
//!    The CLAUDE.md money rule (every `*_cents` is i64) is upheld
//!    by NOT introducing one. A future commit that drifts a price
//!    field into the email-template row would fail this test and
//!    force a move to a money-aware module.
//!
//! 6. **`admin_router()` mount table** — 7 documented endpoints
//!    (`POST /`, `GET /`, `GET /:id`, `PUT /:id`, `DELETE /:id`,
//!    `POST /:id/preview`, `POST /:id/test`, `GET /settings`).
//!    Building it as `Router<AppState>` proves every handler
//!    signature still matches its extractor contract.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/cms_delivery_test.rs`.

use revolution_api::routes::email_templates::{
    CreateTemplateRequest, EmailTemplateRow, PreviewRequest, SendTestRequest, TemplateListQuery,
    UpdateTemplateRequest,
};

// ── i64 primary key round-trip ───────────────────────────────────────

/// `EmailTemplateRow.id` is `i64`. Email-template rows are
/// low-volume, but the cross-table-join landmine of mixing i32 / i64
/// primary keys means we pin every row's id to i64 explicitly. Above
/// `i32::MAX` must survive serialize round-trip.
#[test]
fn email_template_row_id_is_i64_round_trips_past_i32_max() {
    let big: i64 = (i32::MAX as i64) + 1;
    let now = chrono::NaiveDateTime::default();
    let row = EmailTemplateRow {
        id: big,
        name: "Welcome Email".to_string(),
        slug: "welcome-email".to_string(),
        subject: "Welcome to Revolution".to_string(),
        body: "<p>Hello {{name}}.</p>".to_string(),
        variables: serde_json::json!([{"key": "name", "type": "string"}]),
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize EmailTemplateRow");
    assert_eq!(
        wire["id"].as_i64(),
        Some(big),
        "EmailTemplateRow.id must round-trip past i32::MAX"
    );
    assert!(
        (big as i32 as i64) != big,
        "narrowing id to i32 must lose data — proves i64 is required"
    );
}

// ── CreateTemplateRequest contract ───────────────────────────────────

/// The minimum "create a welcome email" payload is `{ name, subject,
/// body }`. All optional fields default to None.
#[test]
fn create_template_request_accepts_minimal_payload() {
    let body = serde_json::json!({
        "name": "Welcome",
        "subject": "Welcome aboard!",
        "body": "<p>Hello!</p>",
    });
    let req: CreateTemplateRequest = serde_json::from_value(body).expect("minimal create payload");
    assert_eq!(req.name, "Welcome");
    assert_eq!(req.subject, "Welcome aboard!");
    assert_eq!(req.body, "<p>Hello!</p>");
    assert!(
        req.variables.is_none(),
        "variables must stay Optional — handler defaults to []"
    );
    assert!(
        req.is_active.is_none(),
        "is_active must stay Optional — handler defaults to true (new templates live by default)"
    );
}

/// Negative pin: a payload missing the required `body` field must
/// not parse. If a refactor accidentally makes `body` optional, the
/// list of templates would silently include blank-body rows.
#[test]
fn create_template_request_rejects_missing_required_body() {
    let bad = serde_json::json!({
        "name": "Welcome",
        "subject": "Welcome aboard!",
    });
    assert!(
        serde_json::from_value::<CreateTemplateRequest>(bad).is_err(),
        "CreateTemplateRequest must reject payloads missing body — a blank body is a UX regression"
    );
}

// ── UpdateTemplateRequest is all-optional for PATCH semantics ────────

#[test]
fn update_template_request_accepts_empty_and_partial_payloads() {
    let empty: UpdateTemplateRequest =
        serde_json::from_str("{}").expect("empty update must parse — every field is optional");
    assert!(empty.name.is_none());
    assert!(empty.subject.is_none());
    assert!(empty.body.is_none());
    assert!(empty.variables.is_none());
    assert!(empty.is_active.is_none());

    let toggle: UpdateTemplateRequest =
        serde_json::from_value(serde_json::json!({"is_active": false}))
            .expect("toggle-only update");
    assert_eq!(toggle.is_active, Some(false));
    assert!(toggle.name.is_none());
}

// ── TemplateListQuery defaults ───────────────────────────────────────

#[test]
fn template_list_query_defaults_are_all_optional() {
    let q: TemplateListQuery = serde_json::from_str("{}").expect("empty list query must parse");
    assert!(q.page.is_none());
    assert!(q.per_page.is_none());
    assert!(q.search.is_none());

    let full: TemplateListQuery =
        serde_json::from_value(serde_json::json!({"page": 2, "per_page": 25, "search": "welcome"}))
            .expect("full query");
    assert_eq!(full.page, Some(2));
    assert_eq!(full.per_page, Some(25));
    assert_eq!(full.search.as_deref(), Some("welcome"));
}

// ── PreviewRequest accepts arbitrary JSON ────────────────────────────

#[test]
fn preview_request_accepts_arbitrary_data() {
    let req: PreviewRequest = serde_json::from_value(serde_json::json!({
        "data": {"name": "Billy", "trial_days": 14}
    }))
    .expect("preview payload must parse");
    let data_obj = req.data.as_object().expect("data must be object");
    assert_eq!(data_obj.get("name").and_then(|v| v.as_str()), Some("Billy"));
    assert_eq!(
        data_obj.get("trial_days").and_then(|v| v.as_i64()),
        Some(14)
    );
}

// ── SendTestRequest captures the recipient email ─────────────────────

#[test]
fn send_test_request_round_trips_email_field() {
    let req: SendTestRequest =
        serde_json::from_value(serde_json::json!({"email": "billy@example.com"}))
            .expect("send-test payload must parse");
    assert_eq!(req.email, "billy@example.com");
}

// ── Money: documenting the absence ───────────────────────────────────

/// `email_templates` is an admin CMS surface for email content; no
/// monetary fields exist. The CLAUDE.md money rule (every `*_cents`
/// is i64) is upheld by NOT introducing one. If a future commit
/// drifts a price field into the email-template schema, this test
/// fails and forces the reviewer to move the field to a money-aware
/// module.
#[test]
fn email_templates_public_dtos_have_no_cents_fields() {
    let now = chrono::NaiveDateTime::default();
    let row = EmailTemplateRow {
        id: 1,
        name: "Receipt".to_string(),
        slug: "receipt".to_string(),
        subject: "Your receipt".to_string(),
        body: "<p>{{order_total}}</p>".to_string(),
        variables: serde_json::json!([]),
        is_active: true,
        created_at: now,
        updated_at: now,
    };
    let wire = serde_json::to_string(&row).expect("serialize EmailTemplateRow");
    assert!(
        !wire.contains("_cents"),
        "email_templates wire-format must never carry a *_cents field — found in: {wire}"
    );
}

// ── Router mount table ───────────────────────────────────────────────

/// `admin_router()` mounts 7 documented endpoints
/// (`GET/POST /`, `GET/PUT/DELETE /:id`, `POST /:id/preview`,
///  `POST /:id/test`, `GET /settings`). Building it as
/// `Router<AppState>` proves every handler signature still matches
/// its extractor contract.
#[test]
fn email_templates_admin_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::email_templates::admin_router();
}
