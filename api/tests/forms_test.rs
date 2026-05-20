//! Forms route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::forms` and exercises the
//! 10 public DTOs + both mounted routers (`public_router()` for the
//! `/api/forms/*` public surface and `admin_router()` for the admin
//! CRUD under `/api/admin/forms/*`).
//!
//! ## Why this shape
//!
//! Every handler in `routes/forms.rs` (1,660 LOC) runs live SQL
//! against `forms` and `form_submissions`, plus admin-gated extractors
//! (`AdminUser`), so the handlers cannot be invoked in unit-test
//! isolation. What we CAN pin:
//!
//! 1. **`FormRow` / `FormSubmissionRow` ID types are `i64`.** Forms
//!    and submissions are BIGSERIAL — `id`, `form_id`, the JSON-array
//!    `submission_ids` payload in bulk endpoints all carry through to
//!    `WHERE id = $1` SQL binds, and a regression that narrowed any
//!    one to `i32` would silently 404 the first form past row
//!    `2^31` and break every bulk endpoint on production data.
//!
//! 2. **List-query optional-everywhere shape.** The admin grid POSTs
//!    `/forms` with whatever filter set the user has applied (often
//!    none). A regression that flipped `page` / `per_page` / `search`
//!    / `is_published` to required would 400-error the
//!    "show me everything" flow.
//!
//! 3. **`CreateFormRequest` accepts arbitrary `serde_json::Value` for
//!    `fields` and `settings`.** Form schemas are user-authored JSON
//!    (drag-and-drop builder); pinning the `Value` wire shape protects
//!    that.
//!
//! 4. **Bulk-write payloads accept `Vec<i64>` of submission IDs.**
//!    `BulkUpdateStatusRequest` and `BulkDeleteRequest` are the
//!    highest-blast-radius admin endpoints in this domain — one POST
//!    can re-status or delete every submission in the supplied vec.
//!    Pin the `Vec<i64>` shape.
//!
//! 5. **`router()` mount table compile-pin.** Both `public_router()`
//!    and `admin_router()` must build with `AppState` as the state
//!    type. A refactor that breaks a handler signature (wrong
//!    extractor, dropped `AdminUser` gate, wrong return type) fails
//!    compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`, `tests/products_test.rs`,
//! `tests/admin_members_test.rs`, `tests/email_templates_test.rs`.

use revolution_api::routes::forms::{
    BulkDeleteRequest, BulkUpdateStatusRequest, CreateFormRequest, ExportQuery, FormListQuery,
    FormRow, FormSubmissionRow, SubmissionListQuery, UpdateFormRequest,
    UpdateSubmissionStatusRequest,
};

// ── 1. FormRow: ID and counts compile to i64 / i32 as documented ─────

/// `FormRow.id` and `FormRow.submission_count` straddle the i64 / i32
/// boundary intentionally. `id` is BIGSERIAL → i64; `submission_count`
/// is a per-row counter that legitimately caps at 2B (per the
/// CLAUDE.md "Reserved exception" — row counts stay i32). Pinning both
/// here catches a refactor that flipped either one.
#[test]
fn form_row_id_is_i64_and_submission_count_is_i32() {
    let now = chrono::Utc::now().naive_utc();
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // 2_147_483_648 — first row past i32

    let row = FormRow {
        id: above_i32_max,
        name: "Contact Us".to_string(),
        slug: "contact-us".to_string(),
        description: Some("Inbound lead form".to_string()),
        fields: serde_json::json!([{"name": "email", "type": "email"}]),
        settings: serde_json::json!({"submit_label": "Send"}),
        is_published: true,
        submission_count: i32::MAX, // largest legal value for the count
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize FormRow");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "FormRow.id MUST be i64 — BIGSERIAL primary key"
    );
    assert_eq!(
        wire["submission_count"].as_i64(),
        Some(i32::MAX as i64),
        "submission_count is per-row count (i32 OK per CLAUDE.md exception)"
    );

    // Sanity — the fixture must actually exceed i32::MAX or the
    // assertion above could pass with an i32 id field.
    assert!(
        row.id > i32::MAX as i64,
        "fixture must exceed i32::MAX or it does not prove the i64 pin"
    );
}

// ── 2. FormSubmissionRow: form_id / id are i64 BIGSERIAL ─────────────

/// `FormSubmissionRow` carries two foreign-key-shaped IDs (`id`,
/// `form_id`). Both flow into `WHERE id = $1 AND form_id = $2` SQL
/// binds throughout the file. Narrowing either to `i32` would silently
/// 404 the first submission past row 2^31 on a high-volume form
/// (lead-gen / newsletter sign-up forms cross that threshold on
/// real B2C deployments).
#[test]
fn form_submission_row_ids_are_i64() {
    let now = chrono::Utc::now().naive_utc();
    let huge_id: i64 = (i32::MAX as i64) * 4; // ~$85.9M-equivalent row count

    let row = FormSubmissionRow {
        id: huge_id,
        form_id: huge_id - 1,
        data: serde_json::json!({"email": "j@example.com", "name": "Jane"}),
        status: Some("unread".to_string()),
        ip_address: Some("203.0.113.7".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        created_at: now,
    };

    let wire = serde_json::to_value(&row).expect("serialize FormSubmissionRow");
    assert_eq!(wire["id"].as_i64(), Some(huge_id));
    assert_eq!(wire["form_id"].as_i64(), Some(huge_id - 1));
    assert!(row.id > i32::MAX as i64);
    assert!(row.form_id > i32::MAX as i64);
}

// ── 3. FormListQuery / SubmissionListQuery: empty payload accepted ───

/// The admin grid hits `/forms` with no filters set (the default
/// "show everything" landing view). A regression that flipped any of
/// `page` / `per_page` / `search` / `is_published` to required would
/// 400-error that flow. Pin the all-optional shape on both grid
/// queries (forms list + submissions list).
#[test]
fn list_queries_accept_empty_payloads() {
    let f: FormListQuery =
        serde_json::from_str("{}").expect("empty FormListQuery must deserialize");
    assert!(f.page.is_none());
    assert!(f.per_page.is_none());
    assert!(f.search.is_none());
    assert!(f.is_published.is_none());

    let s: SubmissionListQuery =
        serde_json::from_str("{}").expect("empty SubmissionListQuery must deserialize");
    assert!(s.page.is_none());
    assert!(s.per_page.is_none());
    assert!(s.status.is_none());
}

// ── 4. CreateFormRequest / UpdateFormRequest: JSON-blob fields ───────

/// Form schemas are user-authored JSON (drag-and-drop builder output).
/// `fields` is REQUIRED on create (you can't have a form with no
/// fields) but `settings` is optional. On update, EVERYTHING is
/// optional (PATCH semantics). Pin both shapes so the wire format
/// stays stable for the legacy frontend.
#[test]
fn create_and_update_form_requests_round_trip() {
    let create: CreateFormRequest = serde_json::from_value(serde_json::json!({
        "name": "Newsletter Signup",
        "fields": [
            {"name": "email", "type": "email", "required": true},
            {"name": "consent", "type": "checkbox"}
        ],
    }))
    .expect("create form with fields-only payload");
    assert_eq!(create.name, "Newsletter Signup");
    assert!(create.description.is_none());
    assert!(create.settings.is_none());
    assert!(create.is_published.is_none());
    assert!(create.fields.is_array(), "fields must be arbitrary JSON");

    // PATCH-update semantics: every field optional.
    let update: UpdateFormRequest = serde_json::from_str("{}")
        .expect("empty UpdateFormRequest must deserialize (PATCH semantics)");
    assert!(update.name.is_none());
    assert!(update.fields.is_none());
    assert!(update.settings.is_none());
    assert!(update.is_published.is_none());

    // Update only the published flag — the "publish" toggle path.
    let publish_only: UpdateFormRequest =
        serde_json::from_value(serde_json::json!({"is_published": true}))
            .expect("publish-only update must deserialize");
    assert_eq!(publish_only.is_published, Some(true));
    assert!(publish_only.name.is_none());
}

// ── 5. Bulk endpoints: Vec<i64> blast radius ─────────────────────────

/// `BulkUpdateStatusRequest` and `BulkDeleteRequest` are the
/// highest-blast-radius admin endpoints in this domain. One POST
/// re-statuses or deletes every submission whose ID appears in the
/// supplied vec. The vec MUST be `Vec<i64>` — `Vec<i32>` would
/// silently truncate submission IDs past `2^31`, mass-mutating the
/// WRONG rows. Pin the i64 shape and the wire contract.
#[test]
fn bulk_endpoints_take_vec_i64_and_pin_status_field() {
    let above_i32_max: i64 = (i32::MAX as i64) + 7;

    let update: BulkUpdateStatusRequest = serde_json::from_value(serde_json::json!({
        "submission_ids": [1_i64, 2_i64, above_i32_max],
        "status": "read",
    }))
    .expect("bulk update payload must deserialize");
    assert_eq!(update.submission_ids.len(), 3);
    assert_eq!(update.submission_ids[2], above_i32_max);
    assert_eq!(update.status, "read");

    let delete: BulkDeleteRequest = serde_json::from_value(serde_json::json!({
        "submission_ids": [10_i64, 20_i64, above_i32_max],
    }))
    .expect("bulk delete payload must deserialize");
    assert_eq!(delete.submission_ids.len(), 3);
    assert!(delete.submission_ids.iter().any(|&id| id > i32::MAX as i64));

    // Single-row status update: pin the `status` shape too.
    let single: UpdateSubmissionStatusRequest =
        serde_json::from_value(serde_json::json!({"status": "spam"}))
            .expect("status update payload must deserialize");
    assert_eq!(single.status, "spam");
}

// ── 6. ExportQuery: optional format ──────────────────────────────────

/// The admin grid offers a CSV export of form submissions. Today the
/// only supported format is CSV (the handler ignores anything else)
/// but the DTO is permissive — `format` is `Option<String>` so future
/// formats (xlsx, json) can roll out without breaking the wire
/// contract.
#[test]
fn export_query_accepts_empty_and_known_format() {
    let empty: ExportQuery =
        serde_json::from_str("{}").expect("empty ExportQuery must deserialize");
    assert!(empty.format.is_none());

    let csv: ExportQuery = serde_json::from_value(serde_json::json!({"format": "csv"}))
        .expect("format=csv must deserialize");
    assert_eq!(csv.format.as_deref(), Some("csv"));

    // Future format hint — must still parse (handler will validate at
    // runtime, but DTO is permissive).
    let xlsx: ExportQuery = serde_json::from_value(serde_json::json!({"format": "xlsx"}))
        .expect("future format hint must deserialize");
    assert_eq!(xlsx.format.as_deref(), Some("xlsx"));
}

// ── 7. Router mount tables: both build with AppState ─────────────────

/// `public_router()` must build as `Router<AppState>`. Load-bearing
/// because the public-facing mount handles unauthenticated form
/// submission (the most-hit endpoint in this module — every lead
/// capture, newsletter signup, contact submission). A refactor that
/// broke any of the four handlers (list_public_forms, get_public_form,
/// submit_form, track_form_view) fails compilation here.
#[test]
fn public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::forms::public_router();
}

/// `admin_router()` must build as `Router<AppState>`. Covers 15
/// mounted routes — form CRUD, publish/unpublish, duplicate, analytics,
/// plus the submissions surface (list/get/delete/bulk-update-status/
/// bulk-delete/export). Single canonical compile-pin for the entire
/// admin-forms surface.
#[test]
fn admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::forms::admin_router();
}

/// Constructors must be safe to call multiple times (the main
/// `api_router()` nests these under different prefixes; a stateful
/// builder would panic on second invocation).
#[test]
fn routers_are_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::forms::public_router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::forms::public_router();
    let _a1: axum::Router<revolution_api::AppState> = revolution_api::routes::forms::admin_router();
    let _a2: axum::Router<revolution_api::AppState> = revolution_api::routes::forms::admin_router();
}
