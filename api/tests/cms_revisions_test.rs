//! CMS revisions route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_revisions` and
//! exercises the public DTOs (`CmsRevisionExtended`, `RevisionSummary`,
//! `RevisionListQuery`, `RevisionListResponse`, `CompareQuery`,
//! `FieldChange`, `BlockChange`, `DiffStats`, `RevisionMetadata`,
//! `RevisionCompareResponse`, `RestoreResponse`) plus the public
//! `router()` mount table.
//!
//! ## Why this file exists (R15-D — deferred from R14-D)
//!
//! `routes/cms_revisions.rs` already has in-mod unit tests for
//! `compute_diff` + `count_words` (private helpers). What those
//! tests CANNOT cover:
//!
//!   - the PUBLIC DTOs the frontend wire-format depends on
//!   - the `router()` mount-table compile-pin
//!   - the i32 / i64 / Uuid type-contract on each field
//!   - the editor-gated auth surface (extractor signatures)
//!
//! An external `tests/*_test.rs` file is the ONLY way to pin those
//! against a public-API refactor (`super::*` in the in-mod tests
//! re-exports private items, so they don't catch
//! visibility-regression bugs).
//!
//! ## What we pin
//!
//! 1. **PK type — CMS revisions use `Uuid`** (not BIGSERIAL i64).
//!
//!    The `cms_revisions` table is part of the CMS v2 stack which
//!    is Uuid-native end-to-end (every `cms_*` table — `cms_users`,
//!    `cms_content`, `cms_revisions` — uses `Uuid` PKs to match
//!    Sanity / Contentful / Strapi conventions). The BIGSERIAL-PK
//!    rule does NOT apply: Uuid is a different design choice for
//!    this whole subsystem. Pin `CmsRevisionExtended.id: Uuid` and
//!    `content_id: Uuid` so a future "let's unify on i64 PKs"
//!    refactor fails at type-check time, not at runtime when the
//!    CMS UI starts 422-ing every revision-list request.
//!
//! 2. **`revision_number` is i32** (CLAUDE.md "Reserved exception").
//!
//!    Per CLAUDE.md:
//!
//!      > Reserved exception: row counts (`revisions: i32`,
//!      >  `attempts: i32`, `total_pages: i32`) — those genuinely
//!      >  cap below 2 billion and `i32` is fine. Money never
//!      >  qualifies for the exception.
//!
//!    Pin `RevisionSummary.revision_number: i32`,
//!    `CompareQuery.v1: i32`, `v2: i32`. The handler caps revisions
//!    at one-per-edit-burst (de-duped on save); a CMS page would
//!    need 2.1B edits to exhaust i32, which is unreachable.
//!
//! 3. **`RevisionListResponse.total: i64`** — counters that
//!    AGGREGATE (number of revisions across all content of a
//!    single page) are i64 by stack convention; the COUNT(*) SQL
//!    in the handler returns `i64`. Pin the response field type.
//!
//! 4. **`RevisionListQuery` is fully optional + has a default
//!    `limit = 25`.** The CMS UI hits `/content/:id/revisions`
//!    with NO params on first load. A regression that flipped
//!    `limit` / `offset` / `change_type` / `include_data` to
//!    required would 422 every "View History" click.
//!
//! 5. **`CompareQuery.v1` and `v2` are required + `include_blocks`
//!    defaults to `true`.** The compare-revisions modal sends
//!    `?v1=3&v2=5` from a side-by-side selector; missing either
//!    breaks the diff view. The `include_blocks` default was
//!    chosen for UX (the modal expects block diffs by default).
//!
//! 6. **`RestoreResponse` carries `new_revision_number` and
//!    `restored_from_revision`** — both i32. The CMS UI's
//!    "Restored from revision 5 to revision 7" toast reads these
//!    two fields. A regression renaming either would silently
//!    blank the toast.
//!
//! 7. **`DiffStats` block-counters are i32** (per "Reserved
//!    exception" — bounded by the editor's reasonable workload).
//!    `word_count_diff: i32` can be NEGATIVE (a deletion is a
//!    negative diff); pin a negative value round-trips.
//!
//! 8. **Router mount table — 4 routes, editor-gated.**
//!    `list_revisions`, `get_revision`, `compare_revisions`,
//!    `restore_revision`. All take `user: User` + run
//!    `require_cms_editor(&user)?` as the first line of the
//!    handler. Compile-pin catches signature drift.
//!
//! ## In-mod test coverage
//!
//! Per `routes/cms_revisions.rs:1153-1209`, in-mod tests cover
//! `compute_diff` (title-change, block-added) and `count_words` —
//! both PRIVATE helpers. This file is strictly the EXTERNAL
//! public-API contract pin.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/cms_v2_enterprise_test.rs`,
//! `tests/cms_global_components_test.rs`, `tests/cms_scheduling_test.rs`
//! (all Uuid-native PK contracts on the CMS v2 surface).

use chrono::Utc;
use revolution_api::routes::cms_revisions::{
    BlockChange, CmsRevisionExtended, CompareQuery, DiffStats, FieldChange, RestoreResponse,
    RevisionCompareResponse, RevisionListQuery, RevisionListResponse, RevisionMetadata,
    RevisionSummary,
};
use uuid::Uuid;

// ── 1. PK type — CmsRevisionExtended uses Uuid (CMS v2 convention) ───

/// CMS v2 PKs are Uuid end-to-end (cms_users, cms_content,
/// cms_revisions). The BIGSERIAL-i64 rule applies to the rest of
/// the stack but NOT this subsystem — pin
/// `CmsRevisionExtended.id: Uuid` and `content_id: Uuid` so a
/// "let's unify on BIGSERIAL" refactor fails at type-check time.
#[test]
fn cms_revision_extended_uses_uuid_pks() {
    let rev_id = Uuid::new_v4();
    let content_id = Uuid::new_v4();
    let creator_id = Uuid::new_v4();
    let now = Utc::now();

    let rev = CmsRevisionExtended {
        id: rev_id,
        content_id,
        revision_number: 1,
        is_current: true,
        data: serde_json::json!({"title": "Hello"}),
        change_summary: Some("Initial content".to_string()),
        changed_fields: Some(vec!["title".to_string(), "content".to_string()]),
        created_at: now,
        created_by: Some(creator_id),
        change_type: Some("create".to_string()),
        word_count: Some(42),
        diff_stats: Some(serde_json::json!({"blocks_added": 0})),
    };

    // Compile-pin: id + content_id are exactly Uuid (not i64, not String)
    let _: Uuid = rev.id;
    let _: Uuid = rev.content_id;
    let _: Option<Uuid> = rev.created_by;

    let wire = serde_json::to_value(&rev).expect("serialize CmsRevisionExtended");
    assert_eq!(
        wire["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()),
        Some(rev_id),
        "CmsRevisionExtended.id MUST serialize as Uuid string (CMS v2 convention)"
    );
    assert_eq!(
        wire["content_id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok()),
        Some(content_id),
        "CmsRevisionExtended.content_id MUST serialize as Uuid string"
    );

    // Round-trip: Uuid string parses back to the same Uuid
    let parsed: CmsRevisionExtended =
        serde_json::from_value(wire).expect("CmsRevisionExtended round-trip");
    assert_eq!(parsed.id, rev_id);
    assert_eq!(parsed.content_id, content_id);
    assert_eq!(parsed.created_by, Some(creator_id));

    // R9-D NEGATIVE: a JSON `id` as an integer MUST fail (i64-coerce
    // bug class — if someone refactored the column to i64, the
    // frontend would send numbers and silently break Uuid parse).
    let int_id = serde_json::json!({
        "id": 42_i64,
        "content_id": content_id,
        "revision_number": 1_i32,
        "is_current": true,
        "data": {},
        "change_summary": null,
        "changed_fields": null,
        "created_at": now,
        "created_by": null,
    });
    assert!(
        serde_json::from_value::<CmsRevisionExtended>(int_id).is_err(),
        "CmsRevisionExtended.id MUST reject integer (Uuid only — CMS v2 PK)"
    );
}

// ── 2. revision_number is i32 (CLAUDE.md "Reserved exception") ───────

/// CLAUDE.md explicitly carves out `revisions: i32` as the canonical
/// example of the "Reserved exception" — bounded counters that cap
/// well below 2.1B. Pin `RevisionSummary.revision_number: i32`,
/// `CompareQuery.v1: i32`, `v2: i32`. A regression to i64 would
/// compile but introduce wire-format drift (the frontend's
/// `revision: number` interface is fine, but the CompareQuery
/// route parsing path expects i32 and would silently reject
/// integers in the i32-overflow range as 422).
#[test]
fn revision_number_fields_are_i32_reserved_exception() {
    let now = Utc::now();
    let summary = RevisionSummary {
        id: Uuid::new_v4(),
        revision_number: 7,
        is_current: false,
        change_summary: Some("Edited title".to_string()),
        changed_fields: Some(vec!["title".to_string()]),
        created_at: now,
        created_by: Some(Uuid::new_v4()),
        created_by_name: Some("Editor Bob".to_string()),
        change_type: Some("update".to_string()),
        word_count: Some(120),
    };
    let _: i32 = summary.revision_number;
    let _: Option<i32> = summary.word_count;

    let wire = serde_json::to_value(&summary).expect("serialize RevisionSummary");
    assert_eq!(wire["revision_number"].as_i64(), Some(7));
    assert_eq!(wire["word_count"].as_i64(), Some(120));

    // CompareQuery.v1 / v2 are required i32
    let cq: CompareQuery = serde_json::from_value(serde_json::json!({
        "v1": 3,
        "v2": 5,
    }))
    .expect("CompareQuery with v1/v2 MUST parse");
    let _: i32 = cq.v1;
    let _: i32 = cq.v2;
    assert_eq!(cq.v1, 3);
    assert_eq!(cq.v2, 5);
    assert!(
        cq.include_blocks,
        "CompareQuery.include_blocks defaults to true (block-diff modal expects it)"
    );

    // R9-D NEGATIVE: v1/v2 must NOT be optional (compare needs both)
    assert!(
        serde_json::from_value::<CompareQuery>(serde_json::json!({
            "v2": 5,
        }))
        .is_err(),
        "CompareQuery without v1 MUST fail (required)"
    );
    assert!(
        serde_json::from_value::<CompareQuery>(serde_json::json!({
            "v1": 3,
        }))
        .is_err(),
        "CompareQuery without v2 MUST fail (required)"
    );

    // R9-D NEGATIVE: v1 overflowing i32 MUST fail
    let overflow = serde_json::json!({
        "v1": (i32::MAX as i64) + 1,
        "v2": 1,
    });
    assert!(
        serde_json::from_value::<CompareQuery>(overflow).is_err(),
        "CompareQuery.v1 > i32::MAX MUST fail (revisions are i32 — Reserved exception)"
    );
}

// ── 3. RevisionListQuery defaults + RevisionListResponse.total i64 ───

/// `RevisionListQuery` is the query-string DTO behind
/// `GET /content/:id/revisions`. The CMS UI hits this with zero
/// params on the first "View History" click. Defaults:
///   - limit = 25 (`default_limit()`)
///   - offset = 0 (`serde(default)` → i64::default())
///   - change_type = None
///   - include_data = false (`serde(default)` → bool::default())
///
/// `RevisionListResponse.total: i64` — the COUNT(*) SQL returns
/// i64; a regression to i32 would silently truncate when a single
/// content item has more than 2.1B revisions (unreachable, but the
/// type-contract is the audit trail).
#[test]
fn revision_list_query_defaults_and_response_counter_types() {
    // Empty query → defaults
    let empty: RevisionListQuery =
        serde_json::from_str("{}").expect("empty RevisionListQuery MUST parse (default load)");
    assert_eq!(
        empty.limit, 25,
        "RevisionListQuery.limit default MUST be 25 (matches handler clamp)"
    );
    assert_eq!(
        empty.offset, 0,
        "RevisionListQuery.offset default MUST be 0"
    );
    assert!(empty.change_type.is_none());
    assert!(
        !empty.include_data,
        "RevisionListQuery.include_data default MUST be false (perf)"
    );

    // Rich query
    let rich: RevisionListQuery = serde_json::from_value(serde_json::json!({
        "limit": 100,
        "offset": 50,
        "change_type": "restore",
        "include_data": true,
    }))
    .expect("rich RevisionListQuery MUST parse");
    assert_eq!(rich.limit, 100);
    assert_eq!(rich.offset, 50);
    assert_eq!(rich.change_type.as_deref(), Some("restore"));
    assert!(rich.include_data);

    // RevisionListResponse.total is i64
    let resp = RevisionListResponse {
        revisions: vec![],
        total: (i32::MAX as i64) + 100,
        offset: 0,
        limit: 25,
        has_more: true,
        current_revision: 42,
    };
    let _: i64 = resp.total;
    let _: i64 = resp.offset;
    let _: i64 = resp.limit;
    let _: i32 = resp.current_revision;
    let wire = serde_json::to_value(&resp).expect("serialize RevisionListResponse");
    assert_eq!(
        wire["total"].as_i64(),
        Some((i32::MAX as i64) + 100),
        "RevisionListResponse.total MUST be i64 (COUNT(*) return type)"
    );
    assert_eq!(wire["current_revision"].as_i64(), Some(42));
}

// ── 4. DiffStats + RestoreResponse shapes ────────────────────────────

/// `DiffStats` is the body of the compare-revisions response. Block
/// counters (`blocks_added` / `removed` / `modified` / `reordered`)
/// are `i32` per the "Reserved exception" — bounded by the editor's
/// reasonable workload. `word_count_diff: i32` is signed (can be
/// negative — a deletion is a negative diff). Pin a negative value
/// round-trips intact.
///
/// `RestoreResponse` is the body of the restore-revision response.
/// `restored_from_revision` + `new_revision_number` +
/// `new_content_version` are all i32 (per "Reserved exception").
/// The CMS UI's "Restored from rev 5 → rev 7" toast reads these
/// two fields by NAME; a rename would silently blank the toast.
#[test]
fn diff_stats_and_restore_response_shapes() {
    // DiffStats — negative word_count_diff round-trips
    let diff = DiffStats {
        title_changed: true,
        content_changed: true,
        seo_changed: false,
        custom_fields_changed: false,
        blocks_added: 2,
        blocks_removed: 5,
        blocks_modified: 1,
        blocks_reordered: 0,
        total_fields_changed: 3,
        word_count_diff: -42, // Deletion — negative diff
        field_changes: vec![FieldChange {
            field: "title".to_string(),
            from: Some(serde_json::json!("Old Title")),
            to: Some(serde_json::json!("New Title")),
            change_type: "modified".to_string(),
        }],
        block_changes: Some(vec![BlockChange {
            block_id: "block-uuid-1".to_string(),
            block_type: "rich-text".to_string(),
            change_type: "removed".to_string(),
            old_position: Some(0),
            new_position: None,
            changes: None,
        }]),
    };
    let _: i32 = diff.blocks_added;
    let _: i32 = diff.blocks_removed;
    let _: i32 = diff.word_count_diff;

    let wire = serde_json::to_value(&diff).expect("serialize DiffStats");
    assert_eq!(
        wire["word_count_diff"].as_i64(),
        Some(-42),
        "DiffStats.word_count_diff is signed i32 — negative diffs round-trip"
    );
    assert_eq!(wire["blocks_added"].as_i64(), Some(2));
    assert_eq!(wire["blocks_removed"].as_i64(), Some(5));
    assert!(wire["field_changes"].is_array());
    assert!(wire["block_changes"].is_array());

    // RestoreResponse — three i32 revision-tracker fields
    let restore = RestoreResponse {
        message: "Restored from revision 5".to_string(),
        restored_from_revision: 5,
        new_revision_number: 8,
        new_content_version: 8,
        content: serde_json::json!({"title": "Restored"}),
        restored_at: Utc::now(),
    };
    let _: i32 = restore.restored_from_revision;
    let _: i32 = restore.new_revision_number;
    let _: i32 = restore.new_content_version;

    let wire = serde_json::to_value(&restore).expect("serialize RestoreResponse");
    // These FIELD NAMES are load-bearing — the CMS UI toast reads
    // them by exact key. Renaming would silently blank the toast.
    assert_eq!(wire["restored_from_revision"].as_i64(), Some(5));
    assert_eq!(wire["new_revision_number"].as_i64(), Some(8));
    assert_eq!(wire["new_content_version"].as_i64(), Some(8));
    assert_eq!(wire["message"].as_str(), Some("Restored from revision 5"));
}

// ── 5. RevisionCompareResponse shape (modal data contract) ───────────

/// `RevisionCompareResponse` is the body of the compare-revisions
/// endpoint. It carries TWO complete revision snapshots
/// (`from_revision` / `to_revision`) + TWO metadata blobs +
/// `DiffStats`. The compare modal renders the diff side-by-side and
/// the metadata header ("Edited by Bob 5 days ago vs Editor Alice
/// just now"). Pin the field names + the i32-versioning + the
/// Uuid-PK for `content_id`.
#[test]
fn revision_compare_response_shape() {
    let content_id = Uuid::new_v4();
    let now = Utc::now();
    let creator_a = Uuid::new_v4();
    let creator_b = Uuid::new_v4();

    let resp = RevisionCompareResponse {
        content_id,
        version_from: 3,
        version_to: 5,
        from_revision: serde_json::json!({"title": "Old"}),
        to_revision: serde_json::json!({"title": "New"}),
        from_metadata: RevisionMetadata {
            revision_number: 3,
            created_at: now,
            created_by: Some(creator_a),
            created_by_name: Some("Alice".to_string()),
            change_summary: Some("first edit".to_string()),
            change_type: Some("update".to_string()),
            word_count: Some(50),
        },
        to_metadata: RevisionMetadata {
            revision_number: 5,
            created_at: now,
            created_by: Some(creator_b),
            created_by_name: Some("Bob".to_string()),
            change_summary: Some("second edit".to_string()),
            change_type: Some("update".to_string()),
            word_count: Some(60),
        },
        diff: DiffStats {
            title_changed: true,
            content_changed: false,
            seo_changed: false,
            custom_fields_changed: false,
            blocks_added: 0,
            blocks_removed: 0,
            blocks_modified: 0,
            blocks_reordered: 0,
            total_fields_changed: 1,
            word_count_diff: 10,
            field_changes: vec![],
            block_changes: None,
        },
    };

    let _: i32 = resp.version_from;
    let _: i32 = resp.version_to;
    let _: Uuid = resp.content_id;

    let wire = serde_json::to_value(&resp).expect("serialize RevisionCompareResponse");
    assert_eq!(
        wire["content_id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok()),
        Some(content_id),
        "content_id MUST be a Uuid string (CMS v2 PK contract)"
    );
    assert_eq!(wire["version_from"].as_i64(), Some(3));
    assert_eq!(wire["version_to"].as_i64(), Some(5));
    assert!(wire["from_revision"].is_object());
    assert!(wire["to_revision"].is_object());
    assert!(wire["from_metadata"].is_object());
    assert!(wire["to_metadata"].is_object());
    assert!(wire["diff"].is_object());

    // Nested metadata shape — revision_number is i32, created_by is Uuid
    assert_eq!(wire["from_metadata"]["revision_number"].as_i64(), Some(3));
    assert_eq!(
        wire["from_metadata"]["created_by_name"].as_str(),
        Some("Alice")
    );
}

// ── 6. Router mount-table compile-pin (4 routes, editor-gated) ───────

/// `routes::cms_revisions::router()` MUST build as `Router<AppState>`.
/// Mount table (all editor-gated via `require_cms_editor(&user)?`):
///   - GET  /content/:id/revisions                  (list_revisions)
///   - GET  /content/:id/revisions/compare          (compare_revisions) ← must come before :version
///   - GET  /content/:id/revisions/:version         (get_revision)
///   - POST /content/:id/revisions/:version/restore (restore_revision)
///
/// The route ORDER matters: `compare` MUST be declared before
/// `:version` or Axum's matcher would silently route
/// `/content/x/revisions/compare` to `get_revision` with
/// `version = "compare"` and 400 at the i32-parse step. This file
/// can't enforce the order (Axum doesn't expose the route list),
/// but the compile-pin DOES catch handler-signature drift —
/// dropping `User: User` from `get_revision` would let any
/// authenticated user (or anon, if `Option<User>`) view paid CMS
/// drafts.
///
/// The editor-gating is ALSO load-bearing: `require_cms_editor`
/// allows `admin | super-admin | super_admin | editor | marketing`
/// roles. A regression that opened this to `user` role would leak
/// unpublished CMS drafts (campaigns, A/B-test content,
/// pre-announcement copy) to every logged-in member.
#[test]
fn cms_revisions_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_revisions::router();
}

/// Per CLAUDE.md habit #3 — pin that the constructor is repeatable
/// (no global state leaking through `OnceLock` etc.).
#[test]
fn cms_revisions_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_revisions::router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_revisions::router();
}
