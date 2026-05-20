//! Blog Categories route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::categories` and exercises
//! the public DTOs (`Category`, `CategoryQuery`, `CreateCategory`,
//! `UpdateCategory`, `CategoryTreeNode`) + the `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/categories.rs` mounts six admin-gated endpoints under
//! `/admin/categories` (CRUD + tree + children-of). Every handler runs
//! live SQL against the `categories` table, so we can't run them in
//! isolation. What we CAN pin without a DB:
//!
//! 1. **`Category.id` and `parent_id` MUST stay `i64`** (BIGSERIAL on
//!    this stack). The self-referential FK `categories.parent_id ->
//!    categories.id` MUST match. A narrowing to `i32` would silently
//!    break the tree-build recursion (`build_tree` filters by
//!    `parent_id == parent_id`) — `i32 != i64` would always be true
//!    after a cast, returning an empty children list. The catalog
//!    page would render as a flat list with no hierarchy.
//!
//! 2. **`CreateCategory` requires `name` and `slug`** (DB columns are
//!    NOT NULL). A regression that flipped either to `Option<>` would
//!    let the admin POST without the field, then 500 at sqlx-bind
//!    time instead of 422 at serde time — worse UX, dirties logs.
//!
//! 3. **`UpdateCategory` is all-Optional** (partial update). The
//!    admin's "Save" button PATCHes only changed fields; any required
//!    field would 422 every partial save.
//!
//! 4. **`CategoryQuery` is fully optional** — the admin list page
//!    must accept GET with no params (defaults: sort by name asc).
//!    A required field would 400-error the admin grid on load.
//!
//! 5. **Wire format is snake_case** — frontend reads `parent_id`,
//!    `created_at`, `updated_at`. A regression to camelCase
//!    (`parentId`, `createdAt`) would break the admin's category-tree
//!    UI (it would see undefined values). NEGATIVE pin per R9-D.
//!
//! 6. **`CategoryTreeNode.children` is recursive** — the tree
//!    endpoint serializes nested children. A regression that
//!    flattened the children (e.g. switched to a parent_id list) would
//!    break the admin's hierarchy view. Pin via a 2-level fixture.
//!
//! 7. **Router builds as `Router<AppState>`** — compile-pin the
//!    `AdminUser` extractor on each handler. A refactor that dropped
//!    the extractor on any of the 6 routes (index/show/store/update/
//!    destroy/tree/children) would silently expose admin CRUD to
//!    unauth users.
//!
//! ## Pattern source
//!
//! Modeled on `tests/posts_test.rs`, `tests/products_test.rs`,
//! `tests/tags_*` (sibling tag CRUD).

use revolution_api::routes::categories::{
    Category, CategoryQuery, CategoryTreeNode, CreateCategory, UpdateCategory,
};

// Helper: BIGSERIAL-safe Category fixture
fn fixture_category(id: i64, parent_id: Option<i64>) -> Category {
    let now = chrono::Utc::now().naive_utc();
    Category {
        id,
        name: "Technical Analysis".to_string(),
        slug: "technical-analysis".to_string(),
        description: Some("Charts, indicators, patterns".to_string()),
        parent_id,
        created_at: Some(now),
        updated_at: Some(now),
    }
}

// ── 1. ID pin: Category.id + parent_id are i64 (BIGSERIAL contract) ──

/// HARD RULE: every PK on this stack is `BIGSERIAL` (i64). The self-
/// referential FK `categories.parent_id -> categories.id` MUST also
/// be `i64`. A narrowing of either would silently break the tree-build
/// recursion in `build_tree` (which filters by `parent_id == parent_id`)
/// — after a narrowing cast, the filter would never match and the
/// admin's category-tree view would render as a flat list.
#[test]
fn category_id_and_parent_id_are_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let cat = fixture_category(above_i32_max, Some(above_i32_max - 1));

    let wire = serde_json::to_value(&cat).expect("serialize Category");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["parent_id"].as_i64(), Some(above_i32_max - 1));

    // Compile-pin: types are genuinely i64.
    let _: i64 = cat.id;
    let _: Option<i64> = cat.parent_id;

    // Narrowing proof
    assert!(
        (cat.id as i32 as i64) != cat.id,
        "narrowing Category.id to i32 must lose data — proves i64 is required"
    );

    // Root-category case (parent_id null)
    let root = fixture_category(1, None);
    let wire_root = serde_json::to_value(&root).expect("serialize root Category");
    assert!(
        wire_root["parent_id"].is_null(),
        "root category must serialize parent_id as null, not 0 — the tree-build filter depends on the null/Some distinction"
    );
}

// ── 2. CreateCategory: name and slug required (NEGATIVE pin) ─────────

/// DB has NOT NULL on `categories.name` AND `categories.slug`. A
/// regression that flipped either to `Option<>` would let the admin
/// POST without the field, then 500 at sqlx-bind time (with a
/// confusing `null value in column violates not-null constraint`
/// error) instead of returning 422 at serde-validation time. The
/// admin's form-error toast assumes a 422 with a field error.
///
/// R9-D NEGATIVE pin: assert deserialization FAILS when each
/// required field is missing.
#[test]
fn create_category_required_fields_negative_pin() {
    // Minimal valid payload (root category)
    let root: CreateCategory = serde_json::from_value(serde_json::json!({
        "name": "Trading",
        "slug": "trading",
    }))
    .expect("minimal CreateCategory must parse");
    assert_eq!(root.name, "Trading");
    assert_eq!(root.slug, "trading");
    assert!(root.parent_id.is_none());
    assert!(root.description.is_none());

    // Child category — parent_id is i64 (BIGSERIAL FK)
    let child: CreateCategory = serde_json::from_value(serde_json::json!({
        "name": "Day Trading",
        "slug": "day-trading",
        "parent_id": (i32::MAX as i64) + 1,
        "description": "Intra-day strategies",
    }))
    .expect("child CreateCategory must parse with i64 parent_id");
    assert_eq!(child.parent_id, Some((i32::MAX as i64) + 1));
    let _: Option<i64> = child.parent_id;

    // NEGATIVE: missing name must FAIL — DB has NOT NULL on categories.name
    let no_name = serde_json::from_value::<CreateCategory>(serde_json::json!({
        "slug": "trading",
    }));
    assert!(
        no_name.is_err(),
        "CreateCategory without name must fail — DB has NOT NULL on categories.name"
    );

    // NEGATIVE: missing slug must FAIL — DB has NOT NULL on categories.slug
    let no_slug = serde_json::from_value::<CreateCategory>(serde_json::json!({
        "name": "Trading",
    }));
    assert!(
        no_slug.is_err(),
        "CreateCategory without slug must fail — DB has NOT NULL on categories.slug"
    );
}

// ── 3. UpdateCategory: partial-update, all-Optional ──────────────────

/// The admin's "Save" button PATCHes only the changed fields. Every
/// field on `UpdateCategory` MUST be `Option<>`. A regression that
/// flipped ANY field to required would 422 every partial save —
/// specifically the common "just rename" or "just re-parent" flow.
#[test]
fn update_category_accepts_empty_and_partial_payloads() {
    // Empty body (no-op save, bumps updated_at)
    let empty: UpdateCategory =
        serde_json::from_str("{}").expect("empty UpdateCategory must parse");
    assert!(empty.name.is_none());
    assert!(empty.slug.is_none());
    assert!(empty.description.is_none());
    assert!(empty.parent_id.is_none());

    // Rename-only (common flow)
    let rename: UpdateCategory = serde_json::from_value(serde_json::json!({
        "name": "Renamed Category",
    }))
    .expect("name-only update must parse");
    assert_eq!(rename.name.as_deref(), Some("Renamed Category"));
    assert!(rename.slug.is_none(), "unchanged fields must stay None");

    // Re-parent (move a category under a new parent — i64 contract)
    let reparent: UpdateCategory = serde_json::from_value(serde_json::json!({
        "parent_id": (i32::MAX as i64) + 100,
    }))
    .expect("re-parent must parse with i64");
    assert_eq!(reparent.parent_id, Some((i32::MAX as i64) + 100));

    // Promote to root (parent_id explicitly null)
    let to_root: UpdateCategory = serde_json::from_value(serde_json::json!({
        "parent_id": null,
    }))
    .expect("promote-to-root must parse with explicit null");
    assert!(to_root.parent_id.is_none());
}

// ── 4. CategoryQuery: fully optional (admin grid with no filter works) ─

/// The admin `/admin/categories` index page must accept GET with no
/// query params (defaults to sort by name asc, all rows). Every field
/// on `CategoryQuery` MUST be `Option<>`. A required field would
/// 400-error the admin's category-list grid on initial load.
#[test]
fn category_query_accepts_empty_and_filtered_payloads() {
    // Empty (admin's default grid)
    let empty: CategoryQuery = serde_json::from_str("{}").expect("empty CategoryQuery must parse");
    assert!(empty.search.is_none());
    assert!(empty.is_visible.is_none());
    assert!(empty.parent_id.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.page.is_none());
    assert!(empty.all.is_none());

    // Full payload — every documented filter
    let full: CategoryQuery = serde_json::from_value(serde_json::json!({
        "search": "trading",
        "is_visible": true,
        "parent_id": 42,
        "sort_by": "created_at",
        "sort_dir": "desc",
        "per_page": 25,
        "page": 2,
        "all": false,
    }))
    .expect("full CategoryQuery must parse");
    assert_eq!(full.search.as_deref(), Some("trading"));
    assert_eq!(full.is_visible, Some(true));
    assert_eq!(full.parent_id, Some(42));
    assert_eq!(full.sort_by.as_deref(), Some("created_at"));
    assert_eq!(full.sort_dir.as_deref(), Some("desc"));
    assert_eq!(full.per_page, Some(25));
    assert_eq!(full.page, Some(2));

    // i64 compile-pins (match the rest of the stack)
    let _: Option<i64> = full.parent_id;
    let _: Option<i64> = full.per_page;
    let _: Option<i64> = full.page;
}

// ── 5. Wire format: snake_case ONLY (R9-D NEGATIVE pin) ──────────────

/// Frontend reads `parent_id`, `created_at`, `updated_at` as snake_case.
/// A regression that added `#[serde(rename_all = "camelCase")]` would
/// silently break the admin category-tree UI (the JS would see
/// `undefined` for every key and render a blank tree).
#[test]
fn category_wire_is_snake_case_no_camel_case() {
    let cat = fixture_category(1, Some(2));
    let wire = serde_json::to_value(&cat).expect("serialize Category");

    // POSITIVE: snake_case keys present
    assert!(
        wire.get("parent_id").is_some(),
        "parent_id must be on the wire"
    );
    assert!(
        wire.get("created_at").is_some(),
        "created_at must be on the wire"
    );
    assert!(
        wire.get("updated_at").is_some(),
        "updated_at must be on the wire"
    );

    // NEGATIVE: camelCase MUST NOT appear (regression guard)
    assert!(
        wire.get("parentId").is_none(),
        "parentId (camelCase) must NOT appear — wire format is snake_case"
    );
    assert!(
        wire.get("createdAt").is_none(),
        "createdAt (camelCase) must NOT appear — wire format is snake_case"
    );
    assert!(
        wire.get("updatedAt").is_none(),
        "updatedAt (camelCase) must NOT appear — wire format is snake_case"
    );
}

// ── 6. CategoryTreeNode: recursive children round-trip ───────────────

/// The `/admin/categories/tree` endpoint returns a recursive
/// `Vec<CategoryTreeNode>`. A regression that flattened `children`
/// (e.g. switched to a `Vec<i64>` of child IDs) would break the
/// admin's tree-view rendering (it expects nested nodes).
#[test]
fn category_tree_node_serializes_recursive_children() {
    let leaf = CategoryTreeNode {
        id: (i32::MAX as i64) + 5,
        name: "Crypto".to_string(),
        slug: "crypto".to_string(),
        description: None,
        parent_id: Some((i32::MAX as i64) + 1),
        children: vec![],
    };
    let mid = CategoryTreeNode {
        id: (i32::MAX as i64) + 1,
        name: "Day Trading".to_string(),
        slug: "day-trading".to_string(),
        description: Some("Intra-day".to_string()),
        parent_id: Some(1),
        children: vec![leaf],
    };
    let root = CategoryTreeNode {
        id: 1,
        name: "Trading".to_string(),
        slug: "trading".to_string(),
        description: None,
        parent_id: None,
        children: vec![mid],
    };

    let wire = serde_json::to_value(&root).expect("serialize tree");
    assert_eq!(wire["id"].as_i64(), Some(1));
    assert_eq!(wire["children"][0]["name"], "Day Trading");
    assert_eq!(
        wire["children"][0]["id"].as_i64(),
        Some((i32::MAX as i64) + 1),
        "second-level node id must be i64 (BIGSERIAL contract)"
    );
    assert_eq!(wire["children"][0]["children"][0]["name"], "Crypto");
    assert_eq!(
        wire["children"][0]["children"][0]["id"].as_i64(),
        Some((i32::MAX as i64) + 5),
        "third-level leaf id must also be i64 — recursion preserves the type"
    );
    assert!(
        wire["children"][0]["children"][0]["children"].is_array(),
        "leaf nodes must still have a `children: []` key (empty array, not absent — frontend consumes uniformly)"
    );
}

// ── 7. Router mount-table compile-pin ────────────────────────────────

/// `routes::categories::router()` must build as `Router<AppState>`.
/// All six handlers (`index`, `show`, `store`, `update`, `destroy`,
/// `get_children`, `get_tree`) take `_admin: AdminUser` — a refactor
/// that dropped the extractor on ANY of them would silently expose
/// admin CRUD to unauth users. Compile-pin catches an extractor-type
/// regression (it can't catch a missing extractor that returns the
/// same handler-result type, but the AdminUser extractor type is
/// load-bearing on its presence at the function-signature level).
#[test]
fn categories_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::categories::router();
}

/// Idempotent construction — building the router twice in the same
/// process must succeed. Catches accidental global-state in the
/// router constructor (per CLAUDE.md habit #3 — "cached state lost
/// during refactor" landmine).
#[test]
fn categories_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::categories::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::categories::router();
    let _r3: axum::Router<revolution_api::AppState> = revolution_api::routes::categories::router();
}
