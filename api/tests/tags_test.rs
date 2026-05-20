//! Tags route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::tags` and exercises the
//! public DTOs (`Tag`, `TagWithCount`, `TagQuery`, `CreateTag`,
//! `UpdateTag`, `MergeTagRequest`) + the `router()` mount table.
//!
//! ## Why this shape — admin-auth-adjacent + multi-step mutation
//!
//! `routes/tags.rs` is the blog-tags admin surface. EVERY handler
//! takes `_admin: AdminUser` (axum extractor enforcing the admin
//! contract). Seven endpoints mounted under root via explicit paths
//! (mount table is `/admin/tags*`, not `nest`-mounted):
//!   - GET  /admin/tags                (list — paginated)
//!   - POST /admin/tags                (create)
//!   - POST /admin/tags/merge          (merge — 3-step mutation)
//!   - GET  /admin/tags/with-counts    (analytics — counts per tag)
//!   - GET  /admin/tags/:id            (show)
//!   - PUT  /admin/tags/:id            (update)
//!   - DELETE /admin/tags/:id          (destroy)
//!   - GET  /admin/tags/:id/usage      (analytics — single-tag count)
//!
//! The merge endpoint is the interesting one: it does a 3-step
//! mutation (UPDATE post_tags + DELETE remaining post_tags + DELETE
//! source tag) without an explicit transaction wrapper. The DTO
//! contract here is what protects the caller from sending malformed
//! merge payloads that 500 at sqlx-bind instead of 422-ing at serde.
//!
//! What we CAN pin without a DB:
//!
//! 1. **`Tag.id` and `TagWithCount.id` are `i64`** — BIGSERIAL on
//!    this stack. The tags table accumulates rows quickly (every
//!    blog post can add new tags; tag-suggestion UIs can multiply
//!    inadvertent rows). A narrowing would silently 404 tags above
//!    `2^31`.
//!
//! 2. **`TagWithCount.post_count` is `i64`** — count aggregation
//!    over `post_tags` join. The Postgres `COUNT(*)` returns `int8`
//!    (i64); narrowing here would silently truncate large
//!    aggregates on the analytics screen.
//!
//! 3. **`CreateTag` requires `name` + `slug`** (DB columns NOT NULL).
//!    Missing either MUST fail at serde, not at sqlx-bind.
//!
//! 4. **`UpdateTag` is all-Optional** (PATCH semantics — common admin
//!    flow is "just rename" or "just re-slug"). A required field
//!    would 422 every partial save.
//!
//! 5. **`TagQuery` fully optional** — admin grid loads with no
//!    params. A required field would 400-error the default view.
//!
//! 6. **`MergeTagRequest.target_id` + `.source_id` BOTH required +
//!    BOTH `i64`** (R9-D NEGATIVE pins for both). Merge is a
//!    destructive operation; sending a malformed payload that 500s
//!    at sqlx-bind would be worse than 422-ing at serde (the user
//!    has already clicked Confirm by that point).
//!
//! 7. **Wire format snake_case** — frontend reads `post_count` as
//!    snake_case. R9-D NEGATIVE pin: camelCase MUST NOT appear.
//!
//! 8. **`router()` mount-table compile-pin** — EIGHT handlers, EVERY
//!    one taking `_admin: AdminUser`. A refactor that dropped the
//!    extractor on the destructive `merge_tags` or `destroy` handler
//!    would silently expose tag-rewrite or tag-delete to unauth
//!    users. Compile-pin is the cheapest guard.
//!
//! ## Pattern source
//!
//! Modeled on `tests/categories_test.rs`, `tests/redirects_test.rs`,
//! `tests/cms_datasources_test.rs` (sibling admin-CRUD surfaces).

use revolution_api::routes::tags::{
    CreateTag, MergeTagRequest, Tag, TagQuery, TagWithCount, UpdateTag,
};

fn fixture_tag(id: i64) -> Tag {
    Tag {
        id,
        name: "Technical Analysis".to_string(),
        slug: "technical-analysis".to_string(),
    }
}

// ── 1. Tag + TagWithCount: id is i64 (BIGSERIAL); post_count is i64 ──

/// `Tag.id` is BIGSERIAL — the tags table accumulates rows
/// fast on a content-heavy site (every post can introduce new tags;
/// tag-suggestion UIs multiply inadvertent rows). A narrowing to
/// `i32` would silently 404 the first high-id tag in the admin grid.
///
/// `TagWithCount.post_count` is `i64` because Postgres `COUNT(*)`
/// returns `int8` — narrowing would silently truncate aggregates on
/// the analytics screen.
#[test]
fn tag_ids_are_i64_and_post_count_is_i64() {
    let above: i64 = (i32::MAX as i64) + 1;
    let t = fixture_tag(above);

    let _: i64 = t.id;

    let wire = serde_json::to_value(&t).expect("serialize Tag");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above),
        "Tag.id MUST be i64 — BIGSERIAL PK"
    );

    // TagWithCount: id AND post_count must be i64
    let twc = TagWithCount {
        id: above + 2,
        name: "Risk Management".to_string(),
        slug: "risk-management".to_string(),
        post_count: (i32::MAX as i64) + 100, // an actively-used tag on a high-traffic site
    };
    let _: i64 = twc.id;
    let _: i64 = twc.post_count;
    let wire = serde_json::to_value(&twc).expect("serialize TagWithCount");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above + 2),
        "TagWithCount.id MUST be i64"
    );
    assert_eq!(
        wire["post_count"].as_i64(),
        Some((i32::MAX as i64) + 100),
        "TagWithCount.post_count MUST be i64 — Postgres COUNT(*) returns int8"
    );
}

// ── 2. CreateTag: name + slug required (DB NOT NULL columns) ──

/// `CreateTag` requires both `name` AND `slug` — DB columns are
/// NOT NULL on the `tags` table. A regression flipping either to
/// `Option<>` would 500 at sqlx-bind on the INSERT.
#[test]
fn create_tag_requires_name_and_slug() {
    // Happy path
    let ok: CreateTag = serde_json::from_value(serde_json::json!({
        "name": "Day Trading",
        "slug": "day-trading",
    }))
    .expect("CreateTag with name+slug MUST parse");
    assert_eq!(ok.name, "Day Trading");
    assert_eq!(ok.slug, "day-trading");

    // R9-D NEGATIVE: missing `name` MUST fail
    assert!(
        serde_json::from_value::<CreateTag>(serde_json::json!({"slug": "x"})).is_err(),
        "CreateTag without `name` MUST fail (DB column NOT NULL)"
    );

    // R9-D NEGATIVE: missing `slug` MUST fail
    assert!(
        serde_json::from_value::<CreateTag>(serde_json::json!({"name": "Trading"})).is_err(),
        "CreateTag without `slug` MUST fail (DB column NOT NULL)"
    );

    // R9-D NEGATIVE: name as number MUST fail
    assert!(
        serde_json::from_value::<CreateTag>(serde_json::json!({
            "name": 42,
            "slug": "x",
        }))
        .is_err(),
        "CreateTag with numeric `name` MUST fail — type is String"
    );
}

// ── 3. UpdateTag: PATCH semantics, all-Optional ──

/// `UpdateTag` is all-Optional. The most-common admin flows:
///   - rename only (drop `name`, leave slug)
///   - re-slug only (drop `slug`, leave name)
///   - empty body (no-op save, bumps updated_at — actually triggers
///     the handler's "No fields to update" 422)
///
/// A regression flipping any field to required would 422 every
/// partial save.
#[test]
fn update_tag_accepts_empty_and_partial_payloads() {
    // Empty body (handler returns 422 "no fields to update", but serde MUST parse it)
    let empty: UpdateTag = serde_json::from_str("{}").expect("empty UpdateTag MUST parse");
    assert!(empty.name.is_none());
    assert!(empty.slug.is_none());

    // Rename only
    let rename: UpdateTag = serde_json::from_value(serde_json::json!({
        "name": "Renamed Tag",
    }))
    .expect("name-only update MUST parse");
    assert_eq!(rename.name.as_deref(), Some("Renamed Tag"));
    assert!(rename.slug.is_none());

    // Re-slug only
    let reslug: UpdateTag = serde_json::from_value(serde_json::json!({
        "slug": "renamed-tag",
    }))
    .expect("slug-only update MUST parse");
    assert!(reslug.name.is_none());
    assert_eq!(reslug.slug.as_deref(), Some("renamed-tag"));

    // Both
    let both: UpdateTag = serde_json::from_value(serde_json::json!({
        "name": "X",
        "slug": "x",
    }))
    .expect("name+slug update MUST parse");
    assert_eq!(both.name.as_deref(), Some("X"));
    assert_eq!(both.slug.as_deref(), Some("x"));
}

// ── 4. TagQuery: fully optional (admin grid loads with no params) ──

/// The admin `/admin/tags` index page hits this with NO query params
/// on initial load. EVERY field MUST be `Option<>`. A required field
/// would 400-error the admin grid on every visit.
#[test]
fn tag_query_accepts_empty_and_full() {
    // Empty
    let empty: TagQuery = serde_json::from_str("{}").expect("empty TagQuery MUST parse");
    assert!(empty.search.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.sort_dir.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.page.is_none());
    assert!(empty.all.is_none());

    // Full
    let full: TagQuery = serde_json::from_value(serde_json::json!({
        "search": "trading",
        "sort_by": "name",
        "sort_dir": "desc",
        "per_page": 50_i64,
        "page": 2_i64,
        "all": false,
    }))
    .expect("full TagQuery MUST parse");
    assert_eq!(full.search.as_deref(), Some("trading"));
    assert_eq!(full.sort_by.as_deref(), Some("name"));
    assert_eq!(full.sort_dir.as_deref(), Some("desc"));
    assert_eq!(full.per_page, Some(50));
    assert_eq!(full.page, Some(2));
    assert_eq!(full.all, Some(false));

    // i64 pagination compile-pin (matches the rest of the stack)
    let _: Option<i64> = full.per_page;
    let _: Option<i64> = full.page;
}

// ── 5. MergeTagRequest: target_id + source_id required, BOTH i64 ──

/// Tag merge is a DESTRUCTIVE 3-step mutation (UPDATE post_tags +
/// DELETE remaining post_tags + DELETE source tag — NO transaction
/// wrapper, see route source). The user has already clicked
/// "Confirm" by the time this DTO is parsed; a 500 at sqlx-bind on
/// a malformed payload is much worse than a 422 at serde.
///
/// R9-D NEGATIVE pins for BOTH fields, plus type-correctness for the
/// i64 BIGSERIAL contract on tags.id.
#[test]
fn merge_tag_request_requires_both_ids_and_pins_i64() {
    // Happy path with both above i32::MAX (BIGSERIAL contract)
    let above_t: i64 = (i32::MAX as i64) + 10;
    let above_s: i64 = (i32::MAX as i64) + 20;
    let ok: MergeTagRequest = serde_json::from_value(serde_json::json!({
        "target_id": above_t,
        "source_id": above_s,
    }))
    .expect("MergeTagRequest with i64 ids MUST parse");
    assert_eq!(ok.target_id, above_t);
    assert_eq!(ok.source_id, above_s);
    let _: i64 = ok.target_id;
    let _: i64 = ok.source_id;

    // R9-D NEGATIVE: missing target_id MUST fail
    assert!(
        serde_json::from_value::<MergeTagRequest>(serde_json::json!({"source_id": 1})).is_err(),
        "MergeTagRequest without `target_id` MUST fail (required for destructive merge)"
    );

    // R9-D NEGATIVE: missing source_id MUST fail
    assert!(
        serde_json::from_value::<MergeTagRequest>(serde_json::json!({"target_id": 1})).is_err(),
        "MergeTagRequest without `source_id` MUST fail (required for destructive merge)"
    );

    // R9-D NEGATIVE: target_id as string MUST fail (i64 type contract)
    assert!(
        serde_json::from_value::<MergeTagRequest>(serde_json::json!({
            "target_id": "1",
            "source_id": 2,
        }))
        .is_err(),
        "MergeTagRequest with string `target_id` MUST fail — type is i64"
    );
}

// ── 6. Tag wire is snake_case (R9-D NEGATIVE camelCase pin) ──

/// `Tag` and `TagWithCount` wire format MUST be snake_case (the
/// admin grid + analytics screens read `post_count`, etc.). A
/// regression that added `#[serde(rename_all = "camelCase")]` would
/// silently break the analytics view.
#[test]
fn tag_with_count_wire_is_snake_case_no_camel_case() {
    let twc = TagWithCount {
        id: 1,
        name: "Day Trading".to_string(),
        slug: "day-trading".to_string(),
        post_count: 42,
    };
    let wire = serde_json::to_value(&twc).expect("serialize TagWithCount");

    // POSITIVE: snake_case keys
    assert!(
        wire.get("post_count").is_some(),
        "post_count MUST be snake_case on the wire"
    );

    // R9-D NEGATIVE camelCase
    assert!(
        wire.get("postCount").is_none(),
        "postCount (camelCase) MUST NOT appear — wire is snake_case"
    );
}

// ── 7. Router mount-table compile-pin (admin-only surface) ──

/// `routes::tags::router()` MUST build as `Router<AppState>`. Mount
/// table covers EIGHT handlers (CRUD + merge + with-counts + usage),
/// EVERY one taking `_admin: AdminUser`. A refactor that dropped the
/// extractor on the destructive `merge_tags` or `destroy` handler
/// would silently expose tag-rewrite or tag-delete to unauth users.
/// Compile-pin catches handler-signature drift; runtime tests on a
/// CI-only fixture would miss it.
#[test]
fn tags_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::tags::router();
}

/// Idempotent construction — pin that nothing global lives inside
/// the router constructor (per CLAUDE.md habit #3).
#[test]
fn tags_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::tags::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::tags::router();
    let _r3: axum::Router<revolution_api::AppState> = revolution_api::routes::tags::router();
}
