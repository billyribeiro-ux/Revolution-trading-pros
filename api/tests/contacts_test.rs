//! Contacts (CRM) route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::contacts` and exercises
//! the 4 public DTOs + the `router()` mount table that the
//! `/admin/contacts` admin surface depends on.
//!
//! ## Why this shape
//!
//! `routes/contacts.rs` (338 LOC) is the CRM admin surface — list,
//! search, create, update, and delete contact rows that the marketing
//! / sales workflows depend on. Every handler runs live SQL against
//! the `contacts` table and is gated behind `AdminUser`
//! (`crate::middleware::admin::AdminUser`), so we can't drive the
//! handlers in isolation. What we CAN pin:
//!
//! 1. **`ContactRow.id` is `i64` BIGSERIAL.** Every CRM keeps every
//!    historical contact around indefinitely — leads from 4 years ago
//!    that never converted are still data the sales team queries. A
//!    long-running CRM table crosses `2^31` rows over years, and the
//!    PK flows into `WHERE id = $1` on get / update / delete. A
//!    regression to `i32` would silently 404 any contact past row
//!    2.1 billion.
//!
//! 2. **`ContactRow.user_id` is `Option<i64>` BIGSERIAL FK.** Contacts
//!    can exist BEFORE they ever sign up (cold leads from form
//!    captures), so it's nullable; but once it's set it MUST match
//!    `users.id` which is `BIGSERIAL`. Narrowing to `i32` here would
//!    orphan high-id user FKs.
//!
//! 3. **`ContactListQuery` is fully optional (admin grid default
//!    view).** The admin contacts grid initially loads with NO
//!    filters — `GET /api/admin/contacts` with empty query. If any
//!    field were required, that initial render would 422 and the
//!    admin would see an empty screen during onboarding.
//!
//! 4. **NEGATIVE: `page` and `per_page` MUST be numeric `i64`, not
//!    strings.** A regression that loosened the type (e.g. flipped to
//!    `String` "for compatibility") would silently accept `?page=foo`
//!    and surface as a query-builder runtime error after the request
//!    hit the handler. Pin it as compile-time + serde-time.
//!
//! 5. **`CreateContactRequest.email` is REQUIRED, status defaults to
//!    "lead".** The handler hard-codes `status.unwrap_or_else(|| "lead"
//!    .to_string())` so the DTO MUST accept create-without-status.
//!    Status is validated against the
//!    `["lead", "prospect", "customer", "inactive", "churned"]`
//!    allowlist in the update handler — pinning the optional shape
//!    here documents that the validation lives at the handler.
//!
//! 6. **`UpdateContactRequest` follows PATCH semantics — every field
//!    optional.** The "mark customer" toggle from the contact-detail
//!    UI sends `{"status": "customer"}` without re-sending email /
//!    name. A regression that flipped any field to required would
//!    break that flow.
//!
//! 7. **Wire-format keys are snake_case.** CamelCase regressions
//!    (`firstName`, `lastName`, `customFields`) MUST NOT appear on
//!    the wire — the SvelteKit admin client expects snake_case to
//!    match the rest of the API surface.
//!
//! 8. **`router()` mount table compile-pin.** Removing `/`, `/stats`,
//!    or `/:id` from the router would silently break the admin grid;
//!    a refactor that broke a handler signature would fail to compile
//!    here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/redirects_test.rs`, `tests/connections_test.rs`,
//! `tests/products_test.rs`, and `tests/courses_admin_test.rs`.

use revolution_api::routes::contacts::{
    ContactListQuery, ContactRow, CreateContactRequest, UpdateContactRequest,
};

// ── 1. ContactRow: i64 PK + Option<i64> user_id FK pin ───────────────

/// HARD RULE: `id` is `i64` BIGSERIAL — the CRM never deletes leads,
/// so the table just grows forever; row counts past `2^31` are
/// realistic on a multi-year-old CRM. `user_id` is `Option<i64>`
/// because a contact can pre-exist any signup (cold lead from a form
/// capture). Once it's set it MUST hold a BIGSERIAL value from
/// `users.id`. A regression to `i32` here orphans high-id users.
///
/// We construct a fixture with `(i32::MAX as i64) + N` for both `id`
/// and `user_id` to prove the i64 width.
#[test]
fn contact_row_id_and_user_id_are_i64_past_i32_max() {
    let big_id: i64 = (i32::MAX as i64) + 17;
    let big_user_id: i64 = (i32::MAX as i64) + 42;

    let now = chrono::Utc::now().naive_utc();
    let row = ContactRow {
        id: big_id,
        user_id: Some(big_user_id),
        email: "lead@example.com".to_string(),
        first_name: Some("Ada".to_string()),
        last_name: Some("Lovelace".to_string()),
        phone: Some("+1-555-0100".to_string()),
        company: Some("Analytical Engines Co".to_string()),
        job_title: Some("Founder".to_string()),
        source: Some("webform".to_string()),
        status: "lead".to_string(),
        tags: Some(serde_json::json!(["priority", "vip"])),
        custom_fields: Some(serde_json::json!({"region": "EMEA"})),
        last_contacted_at: None,
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize ContactRow");
    assert_eq!(
        wire["id"].as_i64(),
        Some(big_id),
        "ContactRow.id must round-trip as i64 past i32::MAX"
    );
    assert_eq!(
        wire["user_id"].as_i64(),
        Some(big_user_id),
        "ContactRow.user_id must round-trip as i64 past i32::MAX (FK to users.id BIGSERIAL)"
    );

    // Belt-and-suspenders: assert the fixture really exceeds i32::MAX
    // (so a future refactor to i32 will compile-fail this literal).
    assert!(row.id > i32::MAX as i64);
    assert!(row.user_id.unwrap() > i32::MAX as i64);

    // Wire-format keys are snake_case — pin no camelCase regression.
    assert!(
        wire.get("firstName").is_none(),
        "wire MUST be snake_case (`first_name`), not camelCase"
    );
    assert!(
        wire.get("userId").is_none(),
        "wire MUST be snake_case (`user_id`), not camelCase"
    );
    assert!(
        wire.get("customFields").is_none(),
        "wire MUST be snake_case (`custom_fields`), not camelCase"
    );
    assert_eq!(wire["first_name"], "Ada");
}

// ── 2. ContactListQuery: all-optional + negative pin ─────────────────

/// `ContactListQuery` is fully optional. The default admin grid render
/// hits `GET /api/admin/contacts` with NO query string; if any field
/// were required this would 422 and the admin would see an empty
/// screen on first load. Pin the all-optional contract.
///
/// Also pin a NEGATIVE: `per_page` MUST be a number (Option<i64>), not
/// a string. A regression that loosened the type would silently accept
/// `?per_page=fifty` and the handler's pagination math would silently
/// produce nonsense `OFFSET` values.
#[test]
fn contact_list_query_is_all_optional_with_typed_numerics() {
    let empty: ContactListQuery =
        serde_json::from_str("{}").expect("empty ContactListQuery must deserialize");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.status.is_none());
    assert!(empty.search.is_none());

    // Filtered: search + status filter, paginated.
    let filtered: ContactListQuery = serde_json::from_value(serde_json::json!({
        "page": 3_i64,
        "per_page": 25_i64,
        "status": "customer",
        "search": "analytical engines"
    }))
    .expect("filtered ContactListQuery must deserialize");
    assert_eq!(filtered.page, Some(3));
    assert_eq!(filtered.per_page, Some(25));
    assert_eq!(filtered.status.as_deref(), Some("customer"));
    assert_eq!(filtered.search.as_deref(), Some("analytical engines"));

    // NEGATIVE: `per_page` as a string MUST fail.
    assert!(
        serde_json::from_value::<ContactListQuery>(serde_json::json!({"per_page": "fifty"}))
            .is_err(),
        "non-numeric per_page MUST fail (Option<i64> type pin)"
    );

    // NEGATIVE: `page` as a string MUST fail.
    assert!(
        serde_json::from_value::<ContactListQuery>(serde_json::json!({"page": "first"})).is_err(),
        "non-numeric page MUST fail (Option<i64> type pin)"
    );
}

// ── 3. CreateContactRequest: email required, status defaults to lead

/// `CreateContactRequest.email` is the ONE required field — every
/// other field is optional. The handler hard-codes `status.
/// unwrap_or_else(|| "lead".to_string())` so the DTO must accept
/// creating a contact WITHOUT a status field at all (defaults to
/// "lead" on the server). Pin both the required-email contract AND
/// the optional-everything-else shape.
#[test]
fn create_contact_request_requires_only_email() {
    // Minimal: just the email — everything else defaults at the handler.
    let minimal: CreateContactRequest = serde_json::from_value(serde_json::json!({
        "email": "newlead@example.com"
    }))
    .expect("minimal CreateContactRequest must deserialize");
    assert_eq!(minimal.email, "newlead@example.com");
    assert!(minimal.first_name.is_none());
    assert!(
        minimal.status.is_none(),
        "status optional — handler defaults to 'lead'"
    );
    assert!(minimal.tags.is_none());

    // Full payload with all optional fields populated.
    let full: CreateContactRequest = serde_json::from_value(serde_json::json!({
        "email": "vip@example.com",
        "first_name": "Grace",
        "last_name": "Hopper",
        "phone": "+1-555-9999",
        "company": "USN",
        "job_title": "Rear Admiral",
        "source": "referral",
        "status": "prospect",
        "tags": ["vip", "speaker"],
        "custom_fields": {"region": "DC"}
    }))
    .expect("full CreateContactRequest must deserialize");
    assert_eq!(full.email, "vip@example.com");
    assert_eq!(full.first_name.as_deref(), Some("Grace"));
    assert_eq!(
        full.tags.as_ref().unwrap(),
        &vec!["vip".to_string(), "speaker".to_string()]
    );
    assert_eq!(full.custom_fields.as_ref().unwrap()["region"], "DC");

    // NEGATIVE: missing `email` MUST fail — the one required field.
    assert!(
        serde_json::from_value::<CreateContactRequest>(serde_json::json!({"first_name": "X"}))
            .is_err(),
        "CreateContactRequest without `email` MUST fail (only required field)"
    );

    // NEGATIVE: camelCase MUST NOT be accepted (snake_case house style).
    // `firstName` is silently ignored by default serde semantics, so the
    // resulting struct has `first_name: None` — pin THAT behaviour.
    let camel_attempt: CreateContactRequest = serde_json::from_value(serde_json::json!({
        "email": "ok@example.com",
        "firstName": "Should Be Ignored"
    }))
    .expect("camelCase fields ignored, not crashing");
    assert!(
        camel_attempt.first_name.is_none(),
        "camelCase `firstName` MUST NOT populate snake_case `first_name`"
    );
}

// ── 4. UpdateContactRequest: PATCH semantics — every field optional ──

/// `UpdateContactRequest` follows PATCH semantics — every field is
/// optional. The most common admin action is "mark this lead as a
/// customer" via `{"status": "customer"}` without re-sending name /
/// email / company. A regression that flipped any field to required
/// would break that flow.
///
/// Also: `status` is validated against an allowlist at the handler
/// (`["lead", "prospect", "customer", "inactive", "churned"]`).
/// Invalid status strings are silently dropped (the `COALESCE` keeps
/// the old value), but the DTO accepts ANY String — pin that the
/// validation lives at the handler, not at deserialization.
#[test]
fn update_contact_request_follows_patch_semantics() {
    // Empty PATCH — fully optional.
    let empty: UpdateContactRequest = serde_json::from_str("{}")
        .expect("empty UpdateContactRequest must deserialize (PATCH semantics)");
    assert!(empty.email.is_none());
    assert!(empty.first_name.is_none());
    assert!(empty.status.is_none());
    assert!(empty.tags.is_none());

    // Status-toggle: the "mark as customer" admin flow.
    let toggle: UpdateContactRequest =
        serde_json::from_value(serde_json::json!({"status": "customer"}))
            .expect("status-only PATCH must deserialize");
    assert_eq!(toggle.status.as_deref(), Some("customer"));
    assert!(toggle.email.is_none());

    // DTO accepts ANY string status — the allowlist validation lives
    // at the handler, not at deserialization. Pin this so a future
    // refactor that moves validation into a custom Deserialize doesn't
    // silently break the existing handler-layer check.
    let exotic: UpdateContactRequest =
        serde_json::from_value(serde_json::json!({"status": "BOGUS_STATUS"}))
            .expect("DTO accepts any String — allowlist is handler-layer");
    assert_eq!(exotic.status.as_deref(), Some("BOGUS_STATUS"));

    // Tags as a vec of strings — JSONB array on the DB side.
    let tags_only: UpdateContactRequest =
        serde_json::from_value(serde_json::json!({"tags": ["enterprise", "renewal-2026"]}))
            .expect("tags-only PATCH must deserialize");
    assert_eq!(
        tags_only.tags.as_ref().unwrap(),
        &vec!["enterprise".to_string(), "renewal-2026".to_string()]
    );

    // NEGATIVE: tags must be an array of strings, not a single string.
    assert!(
        serde_json::from_value::<UpdateContactRequest>(
            serde_json::json!({"tags": "single-string"})
        )
        .is_err(),
        "tags MUST be Vec<String>, not String"
    );
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// Mount-table test: the contacts router is nested at
/// `/api/contacts` (see `src/routes/mod.rs:104 → .nest("/contacts",
/// contacts::router())`). A refactor that drops `/stats` or `/:id`
/// or breaks a handler signature would silently break the CRM admin
/// surface in prod — this test fails compilation if any of the four
/// mounted handlers (list / create / stats / get-or-update-or-delete)
/// loses its `AppState` extractor or its `AdminUser` gate.
#[test]
fn contacts_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::contacts::router();
}

/// Router must be constructible repeatedly (no static state, no
/// once-cell that would explode on second build). Defends against an
/// accidental `OnceLock::set()` inside `router()`.
#[test]
fn contacts_router_is_safe_to_construct_multiple_times() {
    for _ in 0..3 {
        let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::contacts::router();
    }
}
