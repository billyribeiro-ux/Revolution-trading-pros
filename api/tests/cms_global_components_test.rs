//! CMS global components route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_global_components`
//! and exercises the public DTOs + the `admin_router()` /
//! `public_router()` mount tables. The Global Components Library is
//! the "Header / Footer / CTA stays in sync across every page"
//! surface. Every component instance on every page reads back through
//! these DTOs, so a rename of `usage_count` or `is_synced` silently
//! breaks the sidebar across the entire CMS.
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_global_components.rs` runs live SQL
//! against `cms_global_components` / `cms_global_component_versions`
//! / `cms_global_component_usage`, so we can't unit-test handlers.
//! Instead we pin:
//!
//! 1. **`GlobalComponentCategory` wire format** — the enum has
//!    `#[serde(rename_all = "lowercase")]` on the type AND a manual
//!    `Display` impl that returns the lowercase form. Both paths
//!    must agree; the JSON sent over the wire is lowercase and the
//!    sidebar filter UI sends lowercase back. A drift between the
//!    two is the kind of bug that compiles, tests pass, but the UI
//!    silently filters nothing.
//!
//! 2. **`CreateGlobalComponentRequest` defaults** — `is_global`
//!    defaults to `false` (private to author by default). A flipped
//!    default would silently publish every editor's drafts to every
//!    other editor's CMS sidebar.
//!
//! 3. **`TrackComponentUsageRequest` defaults** — `is_synced`
//!    defaults to `true` via `default_true()`. The "sync component
//!    instances to source on save" flag is the entire reason this
//!    feature exists; a flipped default breaks the value-prop.
//!
//! 4. **`SyncComponentRequest.force` defaults to false** — a
//!    flipped default would force-overwrite every detached instance
//!    in the system on the next sync click.
//!
//! 5. **Money pin (documenting absence)** — this is the CMS
//!    components surface; no monetary fields exist. The CLAUDE.md
//!    money rule (every `*_cents` is i64) is upheld by NOT
//!    introducing one. This serialize-and-grep test will fail loud
//!    if a future commit drifts a money field into the schema.
//!
//! 6. **Both routers mount** — `admin_router()` and `public_router()`
//!    must build with `AppState`; a refactor that breaks an extractor
//!    contract is caught here, not in prod.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/cms_delivery_test.rs`.

use revolution_api::routes::cms_global_components::{
    CategoryCount, CreateGlobalComponentRequest, GlobalComponent, GlobalComponentCategory,
    GlobalComponentSummary, SyncComponentRequest, TrackComponentUsageRequest,
};
use uuid::Uuid;

// ── Category enum wire-format (snake_case lowercase) ─────────────────

/// `GlobalComponentCategory` is tagged `#[serde(rename_all = "lowercase")]`.
/// The sidebar filter UI sends lowercase strings back; a refactor that
/// drops the rename would silently break the entire filter dropdown.
#[test]
fn global_component_category_serializes_as_lowercase() {
    let cases = [
        (GlobalComponentCategory::Header, "\"header\""),
        (GlobalComponentCategory::Footer, "\"footer\""),
        (GlobalComponentCategory::Cta, "\"cta\""),
        (GlobalComponentCategory::Form, "\"form\""),
        (GlobalComponentCategory::Navigation, "\"navigation\""),
        (GlobalComponentCategory::Sidebar, "\"sidebar\""),
        (GlobalComponentCategory::Banner, "\"banner\""),
        (GlobalComponentCategory::Modal, "\"modal\""),
        (GlobalComponentCategory::Card, "\"card\""),
        (GlobalComponentCategory::Section, "\"section\""),
    ];
    for (variant, expected) in cases {
        let wire = serde_json::to_string(&variant).expect("serialize variant");
        assert_eq!(
            wire, expected,
            "category must serialize to lowercase for sidebar-filter contract"
        );
    }
}

#[test]
fn global_component_category_default_is_header() {
    let d = GlobalComponentCategory::default();
    assert!(
        matches!(d, GlobalComponentCategory::Header),
        "Default::default() must remain Header — the create-form picks this when no category is selected"
    );
}

#[test]
fn global_component_category_display_matches_serde() {
    // Display impl is hand-written and used in slug generation /
    // logging. It MUST stay in sync with the serde lowercase rename,
    // otherwise we get two different lowercase strings depending on
    // code path. Pin both paths here.
    for (variant, expected) in [
        (GlobalComponentCategory::Header, "header"),
        (GlobalComponentCategory::Footer, "footer"),
        (GlobalComponentCategory::Cta, "cta"),
        (GlobalComponentCategory::Form, "form"),
        (GlobalComponentCategory::Navigation, "navigation"),
        (GlobalComponentCategory::Sidebar, "sidebar"),
        (GlobalComponentCategory::Banner, "banner"),
        (GlobalComponentCategory::Modal, "modal"),
        (GlobalComponentCategory::Card, "card"),
        (GlobalComponentCategory::Section, "section"),
    ] {
        assert_eq!(format!("{variant}"), expected);
        // And the Display result MUST equal what serde emits (minus
        // the surrounding quotes).
        let wire = serde_json::to_string(&variant).expect("serialize");
        assert_eq!(wire, format!("\"{expected}\""));
    }
}

// ── CreateGlobalComponentRequest defaults ────────────────────────────

/// `is_global` defaults to `false` via `#[serde(default)]`. A flipped
/// default would silently publish every editor's local drafts to
/// every other editor's CMS sidebar.
#[test]
fn create_global_component_request_defaults_is_global_to_false() {
    let body = serde_json::json!({
        "name": "Main Header",
        "component_data": [],
        "category": "header",
    });
    let req: CreateGlobalComponentRequest =
        serde_json::from_value(body).expect("minimal create payload");
    assert_eq!(req.name, "Main Header");
    assert!(matches!(req.category, GlobalComponentCategory::Header));
    assert!(
        !req.is_global,
        "is_global default must remain false — author-private by default"
    );
    assert!(req.slug.is_none());
    assert!(req.description.is_none());
    assert!(req.tags.is_none());
}

// ── TrackComponentUsageRequest default is_synced = true ──────────────

/// `is_synced` defaults to `true` via `default_true()`. The "sync
/// component instances to source on save" flag is the entire reason
/// this feature exists; a flipped default breaks the value-prop.
#[test]
fn track_component_usage_request_defaults_is_synced_to_true() {
    let component_id = Uuid::new_v4();
    let content_id = Uuid::new_v4();
    let body = serde_json::json!({
        "component_id": component_id,
        "content_id": content_id,
        "instance_id": "block-abc-123",
    });
    let req: TrackComponentUsageRequest =
        serde_json::from_value(body).expect("minimal usage payload");
    assert_eq!(req.component_id, component_id);
    assert_eq!(req.content_id, content_id);
    assert_eq!(req.instance_id, "block-abc-123");
    assert!(
        req.is_synced,
        "is_synced default must be true — the whole point of the library is auto-sync"
    );
}

// ── SyncComponentRequest default force = false ───────────────────────

#[test]
fn sync_component_request_defaults_force_to_false() {
    let req: SyncComponentRequest =
        serde_json::from_str("{}").expect("empty sync payload must parse");
    assert!(
        !req.force,
        "force default must be false — force-sync clobbers detached instances"
    );
}

// ── Wire-format keys: summary contract for the sidebar UI ────────────

#[test]
fn global_component_summary_wire_keys_match_sidebar_contract() {
    let now = chrono::Utc::now();
    let summary = GlobalComponentSummary {
        id: Uuid::new_v4(),
        name: "Footer v2".to_string(),
        slug: "footer-v2".to_string(),
        description: Some("Replaces the v1 footer".to_string()),
        category: "footer".to_string(),
        tags: Some(vec!["site-wide".to_string()]),
        thumbnail_url: None,
        usage_count: 42,
        is_global: true,
        is_locked: false,
        version: 3,
        created_at: now,
        updated_at: now,
    };
    let wire = serde_json::to_value(&summary).expect("serialize summary");
    assert_eq!(wire["name"], "Footer v2");
    assert_eq!(wire["slug"], "footer-v2");
    assert_eq!(wire["category"], "footer");
    assert_eq!(
        wire["usage_count"], 42,
        "usage_count must stay snake_case — sidebar reads this verbatim"
    );
    assert_eq!(wire["is_global"], true);
    assert_eq!(wire["is_locked"], false);
    assert_eq!(wire["version"], 3);
    // Negative pin: must NOT be renamed to camelCase
    assert!(
        wire.get("usageCount").is_none(),
        "usage_count must stay snake_case to match sidebar JSON contract"
    );
}

#[test]
fn category_count_wire_keys_match_sidebar_facet_contract() {
    let cc = CategoryCount {
        category: "cta".to_string(),
        count: 7,
    };
    let wire = serde_json::to_value(&cc).expect("serialize CategoryCount");
    assert_eq!(wire["category"], "cta");
    assert_eq!(wire["count"], 7);
}

// ── Money: documenting the absence ───────────────────────────────────

/// `cms_global_components` is the **CMS reusability** surface; it
/// intentionally carries NO monetary fields. The CLAUDE.md money rule
/// (every `*_cents` is i64) is upheld by NOT introducing one. A
/// future commit that drifts a price field into a Banner / CTA block
/// would fail this test, forcing the reviewer to move the field to a
/// money-aware module (products, orders, coupons).
#[test]
fn cms_global_components_public_dtos_have_no_cents_fields() {
    let now = chrono::Utc::now();
    let component = GlobalComponent {
        id: Uuid::new_v4(),
        name: "Main CTA".to_string(),
        slug: "main-cta".to_string(),
        description: None,
        component_data: serde_json::json!([]),
        category: GlobalComponentCategory::Cta,
        tags: None,
        thumbnail_url: None,
        usage_count: 0,
        is_global: false,
        is_locked: false,
        version: 1,
        deleted_at: None,
        created_at: now,
        updated_at: now,
        created_by: None,
        updated_by: None,
    };
    let wire = serde_json::to_string(&component).expect("serialize GlobalComponent");
    assert!(
        !wire.contains("_cents"),
        "cms_global_components wire-format must never carry a *_cents field — found in: {wire}"
    );
}

// ── Router mount tables ──────────────────────────────────────────────

/// `admin_router()` mounts 13 documented endpoints (component CRUD,
/// slug lookup, duplicate, version history + restore, usage CRUD,
/// sync/detach/reattach). Building it as `Router<AppState>` proves
/// every handler signature still matches its extractor contract.
#[test]
fn cms_global_components_admin_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_global_components::admin_router();
}

/// `public_router()` mounts the read-only public surface
/// (`/slug/:slug`, `/:id`). A refactor that breaks either signature
/// would compile here but explode at server boot — pin both.
#[test]
fn cms_global_components_public_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_global_components::public_router();
}
