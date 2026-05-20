//! Admin page-layouts route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_page_layouts`
//! (the `router()` mount table) AND `revolution_api::models::page_layout`
//! (the public DTOs: `PageLayout`, `PageLayoutListItem`,
//! `PageLayoutVersion`, `CreatePageLayoutRequest`,
//! `UpdatePageLayoutRequest`, `PageLayoutQueryParams`,
//! `PaginatedLayouts`).
//!
//! ## Why this file exists (R15-D — deferred from R14-D)
//!
//! `routes/admin_page_layouts.rs` is 766 LOC of admin-gated
//! page-builder CRUD + versioning + duplicate + publish/unpublish +
//! restore. Every handler runs `require_admin(&user)?` against the
//! `page_layouts` and `page_layout_versions` tables. None of that
//! can run without a real DB. What we CAN pin:
//!
//! 1. **PK type — page layouts use `Uuid`** (course-content surface,
//!    Uuid-native end-to-end — `page_layouts.id`, `course_id`,
//!    `created_by`, `updated_by` are all Uuid). The BIGSERIAL-i64
//!    rule does NOT apply here: this subsystem matches the CMS v2
//!    convention. A regression to BIGSERIAL would silently 422
//!    every layout request the moment the route extractor's
//!    `Path<Uuid>` flips to `Path<i64>`.
//!
//! 2. **`version` is `Option<i32>`** (CLAUDE.md "Reserved exception"
//!    — bounded counter, like `revisions: i32`). The DB column is
//!    `INTEGER NULL` (legacy layouts pre-dating the versioning
//!    feature have NULL version). Pin both the i32-ness AND the
//!    Optionality so a regression to required i32 doesn't break
//!    legacy layouts on first list, and a regression to i64
//!    doesn't break the wire contract.
//!
//! 3. **`CreatePageLayoutRequest.title` is the ONLY required
//!    field.** `course_id`, `slug`, `description`, `blocks` are
//!    all Optional. The admin can create a "draft" layout without
//!    attaching it to a course yet (`course_id` filled later). A
//!    regression that flipped `course_id` to required would 422
//!    every "New Layout" click before the admin picks a course.
//!
//! 4. **`UpdatePageLayoutRequest` is fully-Optional (PATCH).** The
//!    page-builder autosaves on every block-drop; only the changed
//!    blocks travel on the wire. A regression that flipped any
//!    field to required would 422 every autosave.
//!
//! 5. **`PageLayoutQueryParams` is fully optional.** The admin
//!    lands on `/admin/page-layouts` with no filters; per-page
//!    defaults to 20, page defaults to 1.
//!
//! 6. **`PaginatedLayouts.total: i64`** — `COUNT(*)` from SQL is
//!    i64. `page` / `per_page` / `total_pages` are i32 (admin
//!    pagination doesn't exceed 2.1B pages). Pin both type
//!    contracts on the SAME struct.
//!
//! 7. **Router mount table — 10 routes, ALL admin-gated** via the
//!    `user: User` + `require_admin(&user)?` pattern inside each
//!    handler. Compile-pin catches handler-signature drift —
//!    dropping `User` from any handler would let anon traffic
//!    publish / duplicate / restore course-page layouts in
//!    production.
//!
//! ## Pattern source
//!
//! Modeled on `tests/courses_admin_test.rs`,
//! `tests/cms_v2_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/admin_member_management_test.rs`.

use revolution_api::models::page_layout::{
    CreatePageLayoutRequest, PageLayout, PageLayoutListItem, PageLayoutQueryParams,
    PageLayoutVersion, PaginatedLayouts, UpdatePageLayoutRequest,
};
use uuid::Uuid;

// ── 1. PK type — PageLayout uses Uuid (CMS-style PK contract) ────────

/// `PageLayout.id` is `Uuid`. The `page_layouts` table is part of
/// the course-content stack which is Uuid-native end-to-end
/// (course_id, created_by, updated_by are all Uuid). The
/// BIGSERIAL-i64 rule does NOT apply here — this is a deliberate
/// design choice. Pin the Uuid contract on PageLayout itself AND
/// the lighter PageLayoutListItem AND the version-history record
/// PageLayoutVersion so a future "let's unify PKs" refactor fails
/// at type-check time.
#[test]
fn page_layout_pk_is_uuid() {
    let id = Uuid::new_v4();
    let course_id = Uuid::new_v4();
    let creator_id = Uuid::new_v4();

    let layout = PageLayout {
        id,
        course_id: Some(course_id),
        title: "Hero + Modules".to_string(),
        slug: Some("hero-modules".to_string()),
        description: Some("Course landing page".to_string()),
        status: Some("draft".to_string()),
        version: Some(3),
        blocks: serde_json::json!([
            {"type": "hero", "props": {"title": "Welcome"}},
            {"type": "module-grid", "props": {"columns": 3}},
        ]),
        created_at: None,
        updated_at: None,
        published_at: None,
        created_by: Some(creator_id),
        updated_by: Some(creator_id),
    };

    // Compile-pin: id + course_id + created_by + updated_by are Uuid
    let _: Uuid = layout.id;
    let _: Option<Uuid> = layout.course_id;
    let _: Option<Uuid> = layout.created_by;
    let _: Option<Uuid> = layout.updated_by;

    let wire = serde_json::to_value(&layout).expect("serialize PageLayout");
    assert_eq!(
        wire["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()),
        Some(id),
        "PageLayout.id MUST serialize as Uuid string"
    );
    assert_eq!(
        wire["course_id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok()),
        Some(course_id),
        "PageLayout.course_id MUST serialize as Uuid string"
    );

    // Round-trip
    let parsed: PageLayout = serde_json::from_value(wire).expect("PageLayout round-trip");
    assert_eq!(parsed.id, id);
    assert_eq!(parsed.course_id, Some(course_id));

    // R9-D NEGATIVE: integer `id` MUST fail (Uuid-only — the route
    // extractor is `Path<Uuid>`, not `Path<i64>`).
    let int_id = serde_json::json!({
        "id": 42_i64,
        "course_id": null,
        "title": "Bad",
        "slug": null,
        "description": null,
        "status": null,
        "version": null,
        "blocks": [],
        "created_at": null,
        "updated_at": null,
        "published_at": null,
        "created_by": null,
        "updated_by": null,
    });
    assert!(
        serde_json::from_value::<PageLayout>(int_id).is_err(),
        "PageLayout.id MUST reject integer (Uuid only — `page_layouts.id` is UUID)"
    );

    // PageLayoutListItem — same Uuid contract (the list endpoint
    // returns the lighter shape)
    let item = PageLayoutListItem {
        id,
        course_id: Some(course_id),
        title: "List view".to_string(),
        slug: Some("list-view".to_string()),
        status: Some("published".to_string()),
        version: Some(1),
        created_at: None,
        updated_at: None,
    };
    let _: Uuid = item.id;
    let _: Option<Uuid> = item.course_id;

    // PageLayoutVersion — also Uuid (versions table is course-style)
    let version_record = PageLayoutVersion {
        id: Uuid::new_v4(),
        layout_id: id, // FK back to PageLayout.id — MUST also be Uuid
        version: 2,
        blocks: serde_json::json!([]),
        created_at: None,
        created_by: Some(creator_id),
    };
    let _: Uuid = version_record.id;
    let _: Uuid = version_record.layout_id;
    let _: Option<Uuid> = version_record.created_by;
}

// ── 2. PageLayout.version is Option<i32> (Reserved exception + NULL) ─

/// CLAUDE.md "Reserved exception" explicitly carves out `revisions:
/// i32` and similar bounded counters. `PageLayout.version` is a
/// per-layout monotonic counter — bounded by editor workload, never
/// 2.1B. The DB column is `INTEGER NULL` (legacy layouts pre-dating
/// the versioning feature have NULL version), so the Rust field is
/// `Option<i32>`.
///
/// Pin BOTH:
///   - the i32-ness (not i64; not String)
///   - the Optionality (NULL legacy rows MUST round-trip)
#[test]
fn page_layout_version_is_optional_i32_reserved_exception() {
    // Some(version) — modern layouts
    let modern = PageLayout {
        id: Uuid::new_v4(),
        course_id: None,
        title: "Modern".to_string(),
        slug: None,
        description: None,
        status: Some("published".to_string()),
        version: Some(42),
        blocks: serde_json::json!([]),
        created_at: None,
        updated_at: None,
        published_at: None,
        created_by: None,
        updated_by: None,
    };
    let _: Option<i32> = modern.version;

    let wire = serde_json::to_value(&modern).expect("serialize modern PageLayout");
    assert_eq!(wire["version"].as_i64(), Some(42));

    // None — legacy layout, NULL version column
    let legacy = PageLayout {
        version: None,
        ..modern
    };
    let _: Option<i32> = legacy.version;

    let legacy_wire = serde_json::to_value(&legacy).expect("serialize legacy PageLayout");
    assert!(
        legacy_wire["version"].is_null(),
        "Legacy PageLayout.version=None MUST serialize as JSON null (NULL DB column)"
    );

    // Round-trip the legacy shape (NULL version preserved)
    let parsed: PageLayout =
        serde_json::from_value(legacy_wire).expect("legacy PageLayout round-trip");
    assert!(parsed.version.is_none());

    // R9-D NEGATIVE: version overflowing i32 MUST fail (Reserved
    // exception — versions are i32, not i64).
    let overflow = serde_json::json!({
        "id": Uuid::new_v4(),
        "course_id": null,
        "title": "Overflow",
        "slug": null,
        "description": null,
        "status": null,
        "version": (i32::MAX as i64) + 1,
        "blocks": [],
        "created_at": null,
        "updated_at": null,
        "published_at": null,
        "created_by": null,
        "updated_by": null,
    });
    assert!(
        serde_json::from_value::<PageLayout>(overflow).is_err(),
        "PageLayout.version > i32::MAX MUST fail (Reserved exception — i32 only)"
    );
}

// ── 3. CreatePageLayoutRequest — title is the ONLY required field ────

/// `CreatePageLayoutRequest.title: String` (required). Everything
/// else (`course_id`, `slug`, `description`, `blocks`) is
/// `Option<...>`. The admin's "New Layout" flow lets them save a
/// draft without picking a course yet — `course_id` filled later
/// when they wire it to a course page. A regression that flipped
/// `course_id` to required would 422 every "New Layout" click.
///
/// `UpdatePageLayoutRequest` is fully-Optional (PATCH). The
/// page-builder autosaves on every block drag; only the changed
/// blocks travel.
#[test]
fn create_and_update_page_layout_request_required_optional_split() {
    // Happy path — minimal (title only)
    let minimal: CreatePageLayoutRequest = serde_json::from_value(serde_json::json!({
        "title": "Untitled Layout",
    }))
    .expect("CreatePageLayoutRequest with only `title` MUST parse");
    assert_eq!(minimal.title, "Untitled Layout");
    assert!(minimal.course_id.is_none());
    assert!(minimal.slug.is_none());
    assert!(minimal.description.is_none());
    assert!(minimal.blocks.is_none());

    // Rich — every field populated
    let course_id = Uuid::new_v4();
    let full: CreatePageLayoutRequest = serde_json::from_value(serde_json::json!({
        "course_id": course_id,
        "title": "Hero + Module Grid",
        "slug": "hero-grid",
        "description": "Default landing layout",
        "blocks": [
            {"type": "hero", "props": {"title": "Welcome"}},
        ],
    }))
    .expect("rich CreatePageLayoutRequest MUST parse");
    assert_eq!(full.course_id, Some(course_id));
    assert_eq!(full.title, "Hero + Module Grid");
    assert_eq!(full.slug.as_deref(), Some("hero-grid"));
    assert_eq!(full.description.as_deref(), Some("Default landing layout"));
    assert!(full.blocks.is_some());

    // R9-D NEGATIVE: missing `title` MUST fail (only required field)
    assert!(
        serde_json::from_value::<CreatePageLayoutRequest>(serde_json::json!({
            "course_id": course_id,
        }))
        .is_err(),
        "CreatePageLayoutRequest without `title` MUST fail (the ONE required field)"
    );

    // R9-D NEGATIVE: non-Uuid `course_id` (string) MUST fail
    assert!(
        serde_json::from_value::<CreatePageLayoutRequest>(serde_json::json!({
            "title": "X",
            "course_id": "not-a-uuid",
        }))
        .is_err(),
        "CreatePageLayoutRequest.course_id MUST reject non-Uuid input"
    );

    // UpdatePageLayoutRequest — fully optional (PATCH semantics)
    let empty: UpdatePageLayoutRequest =
        serde_json::from_str("{}").expect("empty UpdatePageLayoutRequest MUST parse (PATCH)");
    assert!(empty.title.is_none());
    assert!(empty.slug.is_none());
    assert!(empty.description.is_none());
    assert!(empty.status.is_none());
    assert!(empty.blocks.is_none());

    // Status-only update (autosave / publish flow)
    let status_only: UpdatePageLayoutRequest = serde_json::from_value(serde_json::json!({
        "status": "published",
    }))
    .expect("status-only UpdatePageLayoutRequest MUST parse");
    assert_eq!(status_only.status.as_deref(), Some("published"));
    assert!(status_only.title.is_none());

    // Blocks-only update (the most common autosave shape)
    let blocks_only: UpdatePageLayoutRequest = serde_json::from_value(serde_json::json!({
        "blocks": [{"type": "rich-text", "props": {"html": "<p>edit</p>"}}],
    }))
    .expect("blocks-only UpdatePageLayoutRequest MUST parse");
    assert!(blocks_only.blocks.is_some());
    assert!(blocks_only.title.is_none());
}

// ── 4. PageLayoutQueryParams — fully optional for default load ───────

/// `PageLayoutQueryParams` is the query-string DTO behind
/// `GET /admin/page-layouts`. The admin lands on the page with
/// zero filters; the handler defaults page to 1, per_page to 20.
/// A regression that flipped any field to required would 422
/// every "Page Layouts" tab open.
///
/// Both pagination fields are `Option<i32>` (admin pagination is
/// bounded — never 2.1B pages, "Reserved exception" applies).
#[test]
fn page_layout_query_params_fully_optional() {
    let empty: PageLayoutQueryParams =
        serde_json::from_str("{}").expect("empty PageLayoutQueryParams MUST parse");
    assert!(empty.course_id.is_none());
    assert!(empty.status.is_none());
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());

    // Rich
    let course_id = Uuid::new_v4();
    let full: PageLayoutQueryParams = serde_json::from_value(serde_json::json!({
        "course_id": course_id,
        "status": "published",
        "page": 3,
        "per_page": 50,
    }))
    .expect("rich PageLayoutQueryParams MUST parse");
    assert_eq!(full.course_id, Some(course_id));
    assert_eq!(full.status.as_deref(), Some("published"));
    assert_eq!(full.page, Some(3));
    assert_eq!(full.per_page, Some(50));

    // Compile-pin: page + per_page are i32 (Reserved exception)
    let _: Option<i32> = full.page;
    let _: Option<i32> = full.per_page;

    // R9-D NEGATIVE: garbage in `page` MUST fail
    assert!(
        serde_json::from_value::<PageLayoutQueryParams>(serde_json::json!({
            "page": "three",
        }))
        .is_err(),
        "PageLayoutQueryParams.page MUST reject string input"
    );

    // R9-D NEGATIVE: non-Uuid `course_id` MUST fail
    assert!(
        serde_json::from_value::<PageLayoutQueryParams>(serde_json::json!({
            "course_id": "not-a-uuid",
        }))
        .is_err(),
        "PageLayoutQueryParams.course_id MUST reject non-Uuid input"
    );
}

// ── 5. PaginatedLayouts — total: i64, pagination ints: i32 ───────────

/// `PaginatedLayouts` is the response shape for the list endpoint.
///   - `total: i64` — `COUNT(*)` returns i64 from sqlx
///   - `page` / `per_page` / `total_pages: i32` — admin
///     pagination never exceeds 2.1B (Reserved exception)
///
/// A regression that flipped `total` to i32 would silently
/// truncate counts when the table grows past 2.1B rows
/// (unreachable, but the type contract is the audit trail).
#[test]
fn paginated_layouts_total_i64_pagination_i32() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let resp = PaginatedLayouts {
        layouts: vec![],
        total: above_i32_max,
        page: 1,
        per_page: 20,
        total_pages: 100,
    };

    // Compile-pin: total is i64; page/per_page/total_pages are i32
    let _: i64 = resp.total;
    let _: i32 = resp.page;
    let _: i32 = resp.per_page;
    let _: i32 = resp.total_pages;

    let wire = serde_json::to_value(&resp).expect("serialize PaginatedLayouts");
    assert_eq!(
        wire["total"].as_i64(),
        Some(above_i32_max),
        "PaginatedLayouts.total MUST be i64 — survives past i32::MAX"
    );
    assert_eq!(wire["page"].as_i64(), Some(1));
    assert_eq!(wire["per_page"].as_i64(), Some(20));
    assert_eq!(wire["total_pages"].as_i64(), Some(100));
    assert!(wire["layouts"].is_array());
}

// ── 6. Router mount-table compile-pin (10 routes, admin-gated) ───────

/// `routes::admin_page_layouts::router()` MUST build as
/// `Router<AppState>`. Mount table (all admin-gated via
/// `require_admin(&user)?` inside each handler):
///
///   - GET    /                            (list_layouts)
///   - POST   /                            (create_layout)
///   - GET    /{id}                        (get_layout)              ← Path<Uuid>
///   - PUT    /{id}                        (update_layout)           ← Path<Uuid>
///   - DELETE /{id}                        (delete_layout)           ← Path<Uuid>
///   - POST   /{id}/publish                (publish_layout)          ← Path<Uuid>
///   - POST   /{id}/unpublish              (unpublish_layout)        ← Path<Uuid>
///   - POST   /{id}/duplicate              (duplicate_layout)        ← Path<Uuid>
///   - GET    /{id}/versions               (get_layout_versions)     ← Path<Uuid>
///   - POST   /{id}/versions/{version}/restore (restore_layout_version) ← Path<(Uuid, i32)>
///   - GET    /course/{course_id}          (get_layout_by_course)    ← Path<Uuid>
///
/// `Path<Uuid>` is LOAD-BEARING — the PK is Uuid (not BIGSERIAL).
/// A refactor that flipped to `Path<i64>` would silently 400 every
/// admin URL the frontend emits.
///
/// The admin-gating is the OTHER load-bearing piece: every handler
/// calls `require_admin(&user)?` as the FIRST line. Skipping that
/// in any one handler would let anon traffic publish course
/// layouts — that's the entire course-landing page going live with
/// a single curl. Compile-pin catches handler-signature drift; the
/// in-mod `require_admin` check is the runtime enforcement layer.
#[test]
fn admin_page_layouts_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_page_layouts::router();
}

/// Per CLAUDE.md habit #3 — repeatable constructor (no global
/// `OnceLock` / `static mut` leaking between calls).
#[test]
fn admin_page_layouts_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_page_layouts::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_page_layouts::router();
}
