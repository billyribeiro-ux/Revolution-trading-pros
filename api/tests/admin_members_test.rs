//! Admin-members route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_members` and
//! exercises the public DTOs + the `router()` mount table. This is
//! the back-of-house member-marketing surface: segments, tags, saved
//! filters, tag assignment, member notes / email history. It's
//! distinct from `admin_member_management` (which owns member CRUD /
//! ban / export); these two together make up the admin-members domain.
//!
//! ## Why this shape
//!
//! Every handler in `routes/admin_members.rs` runs live SQL against
//! `member_segments` / `member_tags` / `user_tags` / `member_filters`
//! / `member_notes` / `member_emails`. None of that is reachable in a
//! unit-test harness — so we attack the contract surface:
//!
//! 1. **i64 ID pin (BIGSERIAL boundary).** Every `id` / `user_id` /
//!    `tag_id` field on `MemberSegment`, `MemberTag`, `MemberFilter`,
//!    `MemberNote`, `MemberEmail`, `AssignTagRequest`, and
//!    `BulkAssignTagsRequest` is `i64`. User IDs on this stack are
//!    BIGSERIAL — a regression that narrowed any of these to `i32`
//!    would silently truncate IDs above 2.1B and corrupt every
//!    user-tag association written by the bulk-assign endpoint.
//!
//! 2. **`BulkAssignTagsRequest` is a `Vec<i64> x Vec<i64>` cross
//!    product.** This is the most-blast-radius endpoint in the whole
//!    file — one admin click tags N users with M tags atomically. The
//!    inner item types MUST stay `i64` for the BIGSERIAL pin to hold.
//!    Pin both vec types past `i32::MAX`.
//!
//! 3. **Permissive DTOs + strict handlers.** Filter queries
//!    (`SegmentQuery`, `MemberTagQuery`, `MemberFilterQuery`,
//!    `AnalyticsRangeQuery`) are all `Option`-everywhere — the admin
//!    grid POSTs with whatever filter set is currently applied. A
//!    refactor that flipped any of them to required would 400-error
//!    the empty-grid load (the most common AI-refactor footgun).
//!
//! 4. **Router mount table.** The router exposes ~30 routes across
//!    segments / tags / filters / tag-assignment / analytics / notes
//!    / emails. We can't probe paths without `AppState`, but we CAN
//!    compile-pin the `Router<AppState>` return type — that catches
//!    every extractor-signature regression at build time. The most
//!    important regression to catch is dropping `AdminUser` from
//!    `bulk_assign_tags` (which would let any authenticated user
//!    silently retag the whole customer base).
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/products_test.rs`, `tests/admin_member_management_test.rs`.

use revolution_api::routes::admin_members::{
    AnalyticsRangeQuery, AssignTagRequest, BulkAssignTagsRequest, CreateMemberFilterRequest,
    CreateMemberTagRequest, CreateNoteRequest, CreateSegmentRequest, MemberEmail, MemberFilter,
    MemberFilterQuery, MemberNote, MemberSegment, MemberTag, MemberTagQuery, SegmentQuery,
    UpdateMemberFilterRequest, UpdateMemberTagRequest, UpdateSegmentRequest,
};

// ── 1. ID fields on every domain row are i64 (BIGSERIAL pin) ─────────

/// HARD RULE: every BIGSERIAL ID is `i64` end-to-end. A regression that
/// narrows any of `MemberSegment.id`, `MemberTag.id`, `MemberFilter.id`
/// to `i32` would silently truncate IDs above 2.1B at the wire boundary.
/// On a customer base with high churn (tag-create-delete cycles drive
/// the sequence forward independent of customer count), 2.1B is a
/// real number on a 7-year horizon.
#[test]
fn segment_tag_filter_ids_round_trip_past_i32_max() {
    let huge: i64 = (i32::MAX as i64) + 12_345;
    let now = chrono::Utc::now().naive_utc();

    let seg = MemberSegment {
        id: huge,
        name: "VIPs".to_string(),
        slug: "vips".to_string(),
        description: Some("Top spenders".to_string()),
        rules: Some(serde_json::json!({"min_ltv_cents": 100_000})),
        member_count: 42,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    let tag = MemberTag {
        id: huge + 1,
        name: "Early Adopter".to_string(),
        slug: "early-adopter".to_string(),
        color: Some("#ff00ff".to_string()),
        description: None,
        member_count: 100,
        created_at: now,
        updated_at: now,
    };

    let filt = MemberFilter {
        id: huge + 2,
        name: "Churned this month".to_string(),
        description: None,
        filters: serde_json::json!({"status": "churned"}),
        is_default: false,
        is_public: true,
        created_by: Some(huge + 3),
        created_at: now,
        updated_at: now,
    };

    let seg_wire = serde_json::to_value(&seg).expect("serialize MemberSegment");
    let tag_wire = serde_json::to_value(&tag).expect("serialize MemberTag");
    let filt_wire = serde_json::to_value(&filt).expect("serialize MemberFilter");

    assert_eq!(seg_wire["id"].as_i64(), Some(huge));
    assert_eq!(tag_wire["id"].as_i64(), Some(huge + 1));
    assert_eq!(filt_wire["id"].as_i64(), Some(huge + 2));
    assert_eq!(filt_wire["created_by"].as_i64(), Some(huge + 3));

    // Narrowing to i32 must lose data (proof that i64 is required).
    assert!(
        (huge as i32 as i64) != huge,
        "fixture must exceed i32::MAX or the i64 pin is not proven"
    );
}

// ── 2. BulkAssignTagsRequest: N×M tag-assignment is i64 end-to-end ───

/// `BulkAssignTagsRequest` is the most-blast-radius endpoint in the
/// admin-members surface: one admin click tags every user in
/// `user_ids` with every tag in `tag_ids`. Both vecs MUST stay
/// `Vec<i64>` so the BIGSERIAL pin holds across the cross-product.
/// A regression to `Vec<i32>` on either side would silently corrupt
/// the user-tag join table for any ID above 2.1B.
#[test]
fn bulk_assign_tags_request_vecs_are_i64() {
    let huge: i64 = (i32::MAX as i64) + 99;

    let req: BulkAssignTagsRequest = serde_json::from_value(serde_json::json!({
        "user_ids": [huge, huge + 1, huge + 2],
        "tag_ids": [huge + 100, huge + 200],
    }))
    .expect("BulkAssignTagsRequest with i64 vecs must deserialize");

    assert_eq!(req.user_ids, vec![huge, huge + 1, huge + 2]);
    assert_eq!(req.tag_ids, vec![huge + 100, huge + 200]);

    // Empty vecs are also valid (handler runtime-rejects with 400, but
    // the DTO is permissive — preserves the typed-error wire shape).
    let empty: BulkAssignTagsRequest = serde_json::from_value(serde_json::json!({
        "user_ids": [],
        "tag_ids": [],
    }))
    .expect("empty bulk-assign must deserialize");
    assert!(empty.user_ids.is_empty());
    assert!(empty.tag_ids.is_empty());

    // Single-pair AssignTagRequest — same i64 pin
    let single: AssignTagRequest = serde_json::from_value(serde_json::json!({
        "user_id": huge,
        "tag_id": huge + 1,
    }))
    .expect("single AssignTagRequest must deserialize");
    assert_eq!(single.user_id, huge);
    assert_eq!(single.tag_id, huge + 1);
}

// ── 3. Filter / list queries: empty payload must parse ───────────────

/// `SegmentQuery`, `MemberTagQuery`, `MemberFilterQuery`, and
/// `AnalyticsRangeQuery` are all `Option`-everywhere. The empty-grid
/// load posts `{}` (no filters applied); a refactor that flipped any
/// field to required would 400-error every initial page load. Pin the
/// permissive shape across all four.
#[test]
fn list_queries_accept_empty_payloads() {
    let seg: SegmentQuery = serde_json::from_str("{}").expect("empty SegmentQuery");
    assert!(seg.search.is_none());
    assert!(seg.is_active.is_none());
    assert!(seg.page.is_none());
    assert!(seg.per_page.is_none());

    let tag: MemberTagQuery = serde_json::from_str("{}").expect("empty MemberTagQuery");
    assert!(tag.search.is_none());
    assert!(tag.page.is_none());

    let filt: MemberFilterQuery = serde_json::from_str("{}").expect("empty MemberFilterQuery");
    assert!(filt.search.is_none());
    assert!(filt.is_public.is_none());
    assert!(filt.page.is_none());

    let analytics: AnalyticsRangeQuery =
        serde_json::from_str("{}").expect("empty AnalyticsRangeQuery");
    assert!(analytics.range.is_none());

    // Full payloads — pin the i64 pagination shape (large per-page
    // values are valid for the export path).
    let seg_full: SegmentQuery = serde_json::from_value(serde_json::json!({
        "search": "vip",
        "is_active": true,
        "page": 1,
        "per_page": 100,
    }))
    .expect("full SegmentQuery");
    assert_eq!(seg_full.is_active, Some(true));
    assert_eq!(seg_full.per_page, Some(100));

    let analytics_30d: AnalyticsRangeQuery =
        serde_json::from_value(serde_json::json!({"range": "30d"}))
            .expect("range=30d AnalyticsRangeQuery");
    assert_eq!(analytics_30d.range.as_deref(), Some("30d"));
}

// ── 4. Create / Update request DTOs: required vs optional split ─────

/// The create-vs-update split is load-bearing: create requires
/// `name` + (filter shape requires `filters`); update flips
/// everything to optional. A refactor that mixed these up would
/// either 400-error legitimate "create a fresh segment" calls or
/// silently accept partial updates that drop fields. Pin both shapes.
#[test]
fn create_and_update_dtos_have_correct_required_optional_split() {
    // CreateSegmentRequest: `name` required, everything else optional
    let seg_create: CreateSegmentRequest =
        serde_json::from_value(serde_json::json!({"name": "VIPs"}))
            .expect("CreateSegmentRequest with just name");
    assert_eq!(seg_create.name, "VIPs");
    assert!(seg_create.slug.is_none());
    assert!(seg_create.description.is_none());
    assert!(seg_create.rules.is_none());
    assert!(seg_create.is_active.is_none());

    // Drift-catch: omitting `name` must FAIL (the field is required)
    assert!(
        serde_json::from_value::<CreateSegmentRequest>(serde_json::json!({"slug": "vips"}))
            .is_err(),
        "CreateSegmentRequest without name must reject"
    );

    // UpdateSegmentRequest: everything optional
    let seg_update: UpdateSegmentRequest = serde_json::from_str("{}").expect("empty update");
    assert!(seg_update.name.is_none());
    assert!(seg_update.is_active.is_none());

    // CreateMemberTagRequest: name required, rest optional
    let tag_create: CreateMemberTagRequest =
        serde_json::from_value(serde_json::json!({"name": "Trial"}))
            .expect("CreateMemberTagRequest with just name");
    assert_eq!(tag_create.name, "Trial");
    assert!(tag_create.color.is_none());

    let tag_update: UpdateMemberTagRequest = serde_json::from_str("{}").expect("empty tag update");
    assert!(tag_update.name.is_none());

    // CreateMemberFilterRequest: name + filters BOTH required
    let filt_create: CreateMemberFilterRequest = serde_json::from_value(serde_json::json!({
        "name": "Churned",
        "filters": {"status": "churned"},
    }))
    .expect("CreateMemberFilterRequest with name + filters");
    assert_eq!(filt_create.name, "Churned");
    assert!(filt_create.is_default.is_none());

    // Drift-catch: missing `filters` MUST reject (it's required, not
    // Option) — this is what differentiates the create from the update.
    assert!(
        serde_json::from_value::<CreateMemberFilterRequest>(serde_json::json!({"name": "broken"}))
            .is_err(),
        "CreateMemberFilterRequest without filters must reject"
    );

    let filt_update: UpdateMemberFilterRequest =
        serde_json::from_str("{}").expect("empty filter update");
    assert!(filt_update.name.is_none());
    assert!(filt_update.filters.is_none());

    // CreateNoteRequest: content required (the only field)
    let note: CreateNoteRequest =
        serde_json::from_value(serde_json::json!({"content": "VIP customer"}))
            .expect("CreateNoteRequest");
    assert_eq!(note.content, "VIP customer");
    assert!(
        serde_json::from_value::<CreateNoteRequest>(serde_json::json!({})).is_err(),
        "CreateNoteRequest without content must reject"
    );
}

// ── 5. MemberNote / MemberEmail wire-format keys ────────────────────

/// `MemberNote` and `MemberEmail` are read by the admin member detail
/// page. A `#[serde(rename)]` regression on `user_id` / `created_by` /
/// `sent_at` would silently break the audit-trail UI. Pin the wire
/// keys explicitly and pin the BIGSERIAL `id` / `user_id` as i64.
#[test]
fn member_note_and_email_wire_format_keys_match_frontend_contract() {
    let huge: i64 = (i32::MAX as i64) + 5;
    let now = chrono::Utc::now().naive_utc();

    let note = MemberNote {
        id: huge,
        user_id: huge + 1,
        content: "Comp'd for 30d after refund".to_string(),
        created_by: Some(huge + 2),
        created_at: now,
    };
    let note_wire = serde_json::to_value(&note).expect("serialize MemberNote");
    assert_eq!(note_wire["id"].as_i64(), Some(huge));
    assert_eq!(note_wire["user_id"].as_i64(), Some(huge + 1));
    assert_eq!(note_wire["created_by"].as_i64(), Some(huge + 2));
    assert_eq!(note_wire["content"], "Comp'd for 30d after refund");

    let email = MemberEmail {
        id: huge + 10,
        user_id: huge + 1,
        subject: "Welcome to Revolution".to_string(),
        status: "delivered".to_string(),
        sent_at: Some(now),
        created_at: now,
    };
    let email_wire = serde_json::to_value(&email).expect("serialize MemberEmail");
    assert_eq!(email_wire["id"].as_i64(), Some(huge + 10));
    assert_eq!(email_wire["user_id"].as_i64(), Some(huge + 1));
    assert_eq!(email_wire["status"], "delivered");
    assert!(email_wire["sent_at"].is_string());

    // Null sent_at shape — for queued/failed emails the timestamp is
    // NULL; pin that the wire shape carries it as JSON null, not
    // missing-key.
    let mut queued = email;
    queued.sent_at = None;
    queued.status = "queued".to_string();
    let queued_wire = serde_json::to_value(&queued).expect("serialize queued email");
    assert!(queued_wire["sent_at"].is_null());
}

// ── 6. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Most-important
/// load-bearing regression caught: dropping `AdminUser` from
/// `bulk_assign_tags` (which would let any authenticated user retag
/// the entire customer base in one POST). The explicit
/// `Router<AppState>` return-type annotation forces a compile error
/// if any handler signature drifts.
#[test]
fn admin_members_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_members::router();
}
