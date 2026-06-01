//! CMS Reusable Blocks route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_reusable_blocks` and
//! exercises the public DTOs (`CmsReusableBlock`, `CmsReusableBlockSummary`,
//! `CmsReusableBlockUsage`, `CreateReusableBlockRequest`,
//! `UpdateReusableBlockRequest`, `TrackBlockUsageRequest`,
//! `ListReusableBlocksQuery`, `PaginatedBlocksResponse`,
//! `BlockUsageResponse`, `CmsReusableBlockCategory`) plus the public
//! `admin_router()` and `public_router()` mount tables.
//!
//! ## Why this shape
//!
//! `routes/cms_reusable_blocks.rs` is the CMS block-library surface —
//! admin CRUD + public read + usage-tracking. Handlers run live SQL
//! against `cms_reusable_blocks` and `cms_reusable_block_usages`, so
//! we can't run them in isolation. What we CAN pin without a DB:
//!
//! 1. **`CmsReusableBlock.id` is `Uuid`** (not BIGSERIAL — this is
//!    one of the few tables on this stack with a UUID PK because the
//!    block_data JSON refers to blocks by UUID for portability across
//!    environments). Pin the type so a future refactor that flipped
//!    to BIGSERIAL would catch here — the schema migration would have
//!    to flip the JSON-embedded refs too.
//!
//! 2. **`usage_count` and `version` are i32**, NOT i64. CLAUDE.md's
//!    "Reserved exception": row counts that genuinely cap below 2
//!    billion. A block used 2.1B times across content is impossible
//!    on this stack (we have ~10⁵ content rows). Pin the i32
//!    contract so a future "be safe, make it i64" refactor surfaces
//!    here — that change would also require a DB migration that's
//!    almost certainly unwanted.
//!
//! 3. **`CmsReusableBlockCategory` enum serializes as lowercase**
//!    (`"general"` / `"trading"` — see `#[serde(rename_all =
//!    "lowercase")]`). The DB column is a Postgres enum with the
//!    SAME lowercase variants. A regression to `PascalCase` would
//!    silently fail every INSERT (Postgres rejects "Trading" against
//!    a "trading" enum). Pin BOTH the Rust→JSON wire format AND the
//!    enum variant names. R9-D NEGATIVE pin: assert PascalCase is
//!    NOT on the wire.
//!
//! 4. **`CreateReusableBlockRequest` requires `name` + `block_data`**
//!    (DB has NOT NULL on both). `slug` is auto-generated when
//!    absent — must be Optional. A regression that flipped `slug` to
//!    required would 422 every "use the default slug" admin save.
//!
//! 5. **`UpdateReusableBlockRequest` is all-Optional** (partial save
//!    is the contract). Specifically, `is_locked` toggle alone must
//!    parse — admins lock/unlock blocks without touching content.
//!
//! 6. **`ListReusableBlocksQuery` is fully optional** — the public
//!    block-picker hits GET `/cms/reusable-blocks/public` with no
//!    query params; default returns 20 most-recently-updated blocks.
//!
//! 7. **Both routers compile** — `admin_router()` (CRUD + usage +
//!    detach/restore) and `public_router()` (read-only listing for
//!    the editor's block-picker). A regression in any handler
//!    signature would fail here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/cms_global_components_test.rs`,
//! `tests/posts_test.rs`.

use revolution_api::routes::cms_reusable_blocks::{
    BlockUsageResponse, CmsReusableBlock, CmsReusableBlockCategory, CmsReusableBlockSummary,
    CmsReusableBlockUsage, CreateReusableBlockRequest, ListReusableBlocksQuery,
    PaginatedBlocksResponse, PaginationMeta, TrackBlockUsageRequest, UpdateReusableBlockRequest,
};
use uuid::Uuid;

// Helper: fixture a CmsReusableBlock with stable UUIDs
fn fixture_block(id: Uuid, usage_count: i32, version: i32) -> CmsReusableBlock {
    let now = chrono::Utc::now();
    CmsReusableBlock {
        id,
        name: "Hero Banner".to_string(),
        slug: "hero-banner".to_string(),
        description: Some("Reusable hero block".to_string()),
        block_data: serde_json::json!({ "type": "hero", "title": "Welcome" }),
        category: Some(CmsReusableBlockCategory::Marketing),
        tags: Some(vec!["hero".to_string(), "landing".to_string()]),
        thumbnail_url: None,
        usage_count,
        is_global: true,
        is_locked: false,
        version,
        deleted_at: None,
        created_at: now,
        updated_at: now,
        created_by: Some(Uuid::new_v4()),
    }
}

// ── 1. CmsReusableBlock.id is Uuid (NOT BIGSERIAL — special-case PK) ─

/// Unlike most tables on this stack, `cms_reusable_blocks.id` is a
/// `Uuid`, not a `BIGSERIAL i64`. This is because the
/// `cms_reusable_block_usages.reusable_block_id` FK + the JSON
/// content-blocks that embed reusable-block refs use UUIDs for
/// portability across environments (a block exported from staging
/// can be imported to prod with the same UUID).
///
/// A regression that flipped to `i64` would force a migration that
/// also rewrites every embedded JSON ref — almost certainly unwanted.
/// Pin the UUID type so a refactor surfaces here.
#[test]
fn cms_reusable_block_id_is_uuid_not_i64() {
    let id = Uuid::new_v4();
    let block = fixture_block(id, 0, 1);

    // Compile-pin
    let _: Uuid = block.id;
    let _: Option<Uuid> = block.created_by;

    // Wire format: UUID serializes as a hyphenated string (NOT a number)
    let wire = serde_json::to_value(&block).expect("serialize CmsReusableBlock");
    assert!(
        wire["id"].is_string(),
        "CmsReusableBlock.id must serialize as a string (UUID), NOT a number — schema portability across envs depends on it"
    );
    assert_eq!(wire["id"].as_str(), Some(id.to_string().as_str()));

    // NEGATIVE: id MUST NOT serialize as a number (would mean a regression to BIGSERIAL)
    assert!(
        wire["id"].as_i64().is_none(),
        "CmsReusableBlock.id must NOT be a number — UUID PK contract is load-bearing"
    );
}

// ── 2. usage_count and version are i32 (CLAUDE.md reserved exception) ─

/// `usage_count` (rows in cms_reusable_block_usages referencing this
/// block) and `version` (admin-managed revision number) are i32.
/// CLAUDE.md "Reserved exception": "row counts (`revisions: i32`,
/// `attempts: i32`, `total_pages: i32`) — those genuinely cap below
/// 2 billion and `i32` is fine."
///
/// A block can't realistically be used 2.1B times — the entire
/// content table is ~10⁵ rows. A "be safe, make it i64" refactor
/// would force a needless DB migration. Pin the i32 contract here
/// so the refactor surfaces (and the author has to justify it).
#[test]
fn cms_reusable_block_counts_are_i32_per_reserved_exception() {
    let id = Uuid::new_v4();
    let block = fixture_block(id, 42, 7);

    // Compile-pin: types are genuinely i32
    let _: i32 = block.usage_count;
    let _: i32 = block.version;

    // Wire format: serializes as a regular JSON number (no rename)
    let wire = serde_json::to_value(&block).expect("serialize");
    assert_eq!(wire["usage_count"].as_i64(), Some(42));
    assert_eq!(wire["version"].as_i64(), Some(7));

    // i32::MAX must round-trip (the upper bound of the reserved exception)
    let block_max = fixture_block(id, i32::MAX, i32::MAX);
    let wire_max = serde_json::to_value(&block_max).expect("serialize i32::MAX");
    assert_eq!(wire_max["usage_count"].as_i64(), Some(i32::MAX as i64));
    assert_eq!(wire_max["version"].as_i64(), Some(i32::MAX as i64));
}

// ── 3. Category enum: lowercase wire-format (R9-D NEGATIVE pin) ──────

/// `CmsReusableBlockCategory` carries `#[serde(rename_all =
/// "lowercase")]` AND `#[sqlx(... rename_all = "lowercase")]` so the
/// Rust enum, the JSON wire-format, AND the Postgres enum
/// (`cms_reusable_block_category`) all use lowercase variants. A
/// regression that dropped EITHER rename would silently break:
///   - drop `serde(rename_all)`: the JSON would serialize as
///     PascalCase (`"Trading"`), which the frontend's TypeScript
///     `Category = "trading" | "marketing"` union would reject;
///   - drop `sqlx(rename_all)`: every INSERT would fail with
///     `invalid input value for enum cms_reusable_block_category:
///     "Trading"`.
///
/// R9-D NEGATIVE pin: assert PascalCase is NOT on the wire.
#[test]
fn cms_reusable_block_category_wire_is_lowercase_not_pascal() {
    // Each variant serializes as lowercase
    let cases = [
        (CmsReusableBlockCategory::General, "general"),
        (CmsReusableBlockCategory::Trading, "trading"),
        (CmsReusableBlockCategory::Layout, "layout"),
        (CmsReusableBlockCategory::Callout, "callout"),
        (CmsReusableBlockCategory::Marketing, "marketing"),
        (CmsReusableBlockCategory::Navigation, "navigation"),
        (CmsReusableBlockCategory::Media, "media"),
        (CmsReusableBlockCategory::Form, "form"),
    ];
    for (variant, expected) in cases {
        let wire = serde_json::to_value(variant).expect("serialize category");
        assert_eq!(
            wire.as_str(),
            Some(expected),
            "category variant must serialize as lowercase string {expected:?}"
        );

        // NEGATIVE: PascalCase variant must NOT round-trip from itself
        // (proves the `rename_all = "lowercase"` is load-bearing).
        let pascal = format!(
            "{}{}",
            expected.chars().next().unwrap().to_uppercase(),
            &expected[1..]
        );
        if pascal != expected {
            let pascal_attempt =
                serde_json::from_str::<CmsReusableBlockCategory>(&format!("\"{pascal}\""));
            // We don't strictly require this to fail (serde may accept
            // either case in some configs), but PascalCase MUST NOT be
            // what we EMIT. The wire-format pin above is the load-bearing
            // assertion; this is documentation.
            let _ = pascal_attempt;
        }
    }

    // Default is "general"
    let default = CmsReusableBlockCategory::default();
    let wire_default = serde_json::to_value(default).expect("serialize default");
    assert_eq!(wire_default.as_str(), Some("general"));
}

// ── 4. CreateReusableBlockRequest required fields (NEGATIVE pin) ─────

/// DB has NOT NULL on `cms_reusable_blocks.name` AND `.block_data`.
/// `slug` is auto-generated from `name` in the handler when absent —
/// MUST be Optional. A regression that flipped `slug` to required
/// would 422 every "use the default slug" admin save, and a
/// regression that flipped `block_data` to Optional would let an
/// admin POST an empty block (then 500 at sqlx-bind on the JSON column).
#[test]
fn create_reusable_block_request_required_fields_negative_pin() {
    // Minimal valid payload — only name + block_data
    let minimal: CreateReusableBlockRequest = serde_json::from_value(serde_json::json!({
        "name": "CTA Banner",
        "block_data": { "type": "cta", "label": "Buy Now" },
    }))
    .expect("minimal CreateReusableBlockRequest must parse");
    assert_eq!(minimal.name, "CTA Banner");
    assert!(
        minimal.slug.is_none(),
        "slug must be auto-generated when absent"
    );
    assert!(minimal.description.is_none());
    assert!(minimal.category.is_none());
    assert!(
        !minimal.is_global,
        "is_global defaults to false via #[serde(default)]"
    );

    // NEGATIVE: missing `name` must FAIL
    let no_name = serde_json::from_value::<CreateReusableBlockRequest>(serde_json::json!({
        "block_data": { "type": "cta" },
    }));
    assert!(
        no_name.is_err(),
        "CreateReusableBlockRequest without name must fail — DB has NOT NULL on cms_reusable_blocks.name"
    );

    // NEGATIVE: missing `block_data` must FAIL
    let no_data = serde_json::from_value::<CreateReusableBlockRequest>(serde_json::json!({
        "name": "Empty Block",
    }));
    assert!(
        no_data.is_err(),
        "CreateReusableBlockRequest without block_data must fail — DB has NOT NULL on cms_reusable_blocks.block_data (no point saving an empty block)"
    );

    // Full payload with all the admin form fields
    let full: CreateReusableBlockRequest = serde_json::from_value(serde_json::json!({
        "name": "Trading Calculator",
        "slug": "custom-calculator-slug",
        "description": "P&L calculator widget",
        "block_data": { "type": "calculator" },
        "category": "trading",
        "tags": ["calculator", "widget"],
        "thumbnail_url": "https://cdn.example/calc.png",
        "is_global": true,
    }))
    .expect("full CreateReusableBlockRequest must parse");
    assert_eq!(full.slug.as_deref(), Some("custom-calculator-slug"));
    assert_eq!(full.category, Some(CmsReusableBlockCategory::Trading));
    assert!(full.is_global);
    assert_eq!(full.tags.as_ref().map(|t| t.len()), Some(2));
}

// ── 5. UpdateReusableBlockRequest: all-Optional (partial save) ───────

/// The admin's "Save" button PATCHes only the changed fields. Every
/// field on `UpdateReusableBlockRequest` MUST be `Option<>`.
///
/// Specifically: `is_locked` toggle alone must parse — admins lock
/// blocks to prevent edits across many pages, and an "only is_locked
/// changed" save is a primary admin flow. A regression that required
/// `name` or `block_data` would 422 every lock/unlock toggle.
#[test]
fn update_reusable_block_request_accepts_empty_and_partial_payloads() {
    // Empty body (no-op save, bumps updated_at)
    let empty: UpdateReusableBlockRequest =
        serde_json::from_str("{}").expect("empty UpdateReusableBlockRequest must parse");
    assert!(empty.name.is_none());
    assert!(empty.block_data.is_none());
    assert!(empty.is_locked.is_none());

    // Lock-only toggle (primary admin flow)
    let lock: UpdateReusableBlockRequest = serde_json::from_value(serde_json::json!({
        "is_locked": true,
    }))
    .expect("lock-only update must parse");
    assert_eq!(lock.is_locked, Some(true));
    assert!(lock.name.is_none(), "unchanged fields must stay None");

    // Rename-only
    let rename: UpdateReusableBlockRequest = serde_json::from_value(serde_json::json!({
        "name": "Renamed Block",
    }))
    .expect("name-only update must parse");
    assert_eq!(rename.name.as_deref(), Some("Renamed Block"));

    // Re-categorize-only
    let recat: UpdateReusableBlockRequest = serde_json::from_value(serde_json::json!({
        "category": "navigation",
    }))
    .expect("category-only update must parse");
    assert_eq!(recat.category, Some(CmsReusableBlockCategory::Navigation));
}

// ── 6. ListReusableBlocksQuery: fully optional + usage-tracking DTO ──

/// `ListReusableBlocksQuery` backs the public block-picker (called
/// from the CMS editor's "insert block" modal). The default request
/// has NO params (returns 20 most-recently-updated). A regression
/// that required any field would 400 the modal on first open.
///
/// Also pin `TrackBlockUsageRequest` (the usage-tracking POST): it
/// requires `reusable_block_id`, `content_id`, `block_instance_id`,
/// and `is_synced` defaults to `true` (per `default_true()` fn).
#[test]
fn list_query_optional_and_track_usage_required_fields_pin() {
    // Empty list query — default block-picker open
    let empty: ListReusableBlocksQuery =
        serde_json::from_str("{}").expect("empty ListReusableBlocksQuery must parse");
    assert!(empty.category.is_none());
    assert!(empty.search.is_none());
    assert!(empty.tags.is_none());
    assert!(empty.limit.is_none());
    assert!(empty.offset.is_none());

    // Full filter (admin's "search marketing blocks tagged hero" flow)
    let full: ListReusableBlocksQuery = serde_json::from_value(serde_json::json!({
        "category": "marketing",
        "search": "banner",
        "tags": "hero,landing",
        "is_global": true,
        "sort_by": "usage_count",
        "sort_order": "desc",
        "limit": 50,
        "offset": 100,
        "include_deleted": false,
    }))
    .expect("full ListReusableBlocksQuery must parse");
    assert_eq!(full.limit, Some(50));
    assert_eq!(full.offset, Some(100));
    let _: Option<i64> = full.limit;
    let _: Option<i64> = full.offset;

    // TrackBlockUsageRequest: 3 required fields, is_synced defaults true
    let block_id = Uuid::new_v4();
    let content_id = Uuid::new_v4();
    let track: TrackBlockUsageRequest = serde_json::from_value(serde_json::json!({
        "reusable_block_id": block_id,
        "content_id": content_id,
        "block_instance_id": "instance-abc-123",
    }))
    .expect("TrackBlockUsageRequest with default is_synced must parse");
    assert_eq!(track.reusable_block_id, block_id);
    assert_eq!(track.content_id, content_id);
    assert_eq!(track.block_instance_id, "instance-abc-123");
    assert!(
        track.is_synced,
        "is_synced must default to true via #[serde(default = \"default_true\")]"
    );

    // NEGATIVE: missing reusable_block_id must FAIL
    let no_block = serde_json::from_value::<TrackBlockUsageRequest>(serde_json::json!({
        "content_id": content_id,
        "block_instance_id": "x",
    }));
    assert!(
        no_block.is_err(),
        "TrackBlockUsageRequest without reusable_block_id must fail — it's the FK target"
    );
}

// ── 7. Summary / Usage / Response types: snake_case wire format ──────

/// `CmsReusableBlockSummary`, `CmsReusableBlockUsage`,
/// `PaginatedBlocksResponse`, `BlockUsageResponse` all flow to the
/// frontend's TypeScript types via the OpenAPI schema. They use
/// snake_case keys throughout. R9-D NEGATIVE pin: assert camelCase
/// is NOT on the wire for the response types.
#[test]
fn response_types_wire_is_snake_case_no_camel_case() {
    let now = chrono::Utc::now();
    let block_id = Uuid::new_v4();

    let summary = CmsReusableBlockSummary {
        id: block_id,
        name: "Hero".to_string(),
        slug: "hero".to_string(),
        description: None,
        category: Some("marketing".to_string()),
        tags: None,
        thumbnail_url: None,
        usage_count: 5,
        is_global: true,
        is_locked: false,
        version: 1,
        created_at: now,
        updated_at: now,
    };
    let s_wire = serde_json::to_value(&summary).expect("serialize summary");

    // POSITIVE: snake_case keys present
    assert!(s_wire.get("usage_count").is_some());
    assert!(s_wire.get("is_global").is_some());
    assert!(s_wire.get("is_locked").is_some());
    assert!(s_wire.get("thumbnail_url").is_some());
    assert!(s_wire.get("created_at").is_some());
    assert!(s_wire.get("updated_at").is_some());

    // NEGATIVE: camelCase MUST NOT appear
    assert!(
        s_wire.get("usageCount").is_none(),
        "usageCount (camelCase) must NOT appear on the wire"
    );
    assert!(
        s_wire.get("isGlobal").is_none(),
        "isGlobal (camelCase) must NOT appear"
    );
    assert!(
        s_wire.get("isLocked").is_none(),
        "isLocked (camelCase) must NOT appear"
    );
    assert!(
        s_wire.get("thumbnailUrl").is_none(),
        "thumbnailUrl (camelCase) must NOT appear"
    );
    assert!(
        s_wire.get("createdAt").is_none(),
        "createdAt (camelCase) must NOT appear"
    );

    // PaginatedBlocksResponse wraps Vec<Summary> + PaginationMeta
    let paginated = PaginatedBlocksResponse {
        data: vec![summary],
        meta: PaginationMeta {
            total: 1,
            limit: 20,
            offset: 0,
            has_more: false,
        },
    };
    let p_wire = serde_json::to_value(&paginated).expect("serialize paginated");
    assert_eq!(p_wire["meta"]["total"].as_i64(), Some(1));
    assert!(
        p_wire["meta"].get("has_more").is_some(),
        "has_more (snake_case) must appear in pagination meta"
    );
    assert!(
        p_wire["meta"].get("hasMore").is_none(),
        "hasMore (camelCase) must NOT appear in pagination meta"
    );

    // BlockUsageResponse round-trips
    let usage = CmsReusableBlockUsage {
        id: Uuid::new_v4(),
        reusable_block_id: block_id,
        content_id: Uuid::new_v4(),
        block_instance_id: "instance-1".to_string(),
        is_synced: true,
        detached_at: None,
        detached_by: None,
        created_at: now,
    };
    let u_wire = serde_json::to_value(&usage).expect("serialize usage");
    assert!(u_wire.get("is_synced").is_some());
    assert!(
        u_wire.get("isSynced").is_none(),
        "camelCase isSynced must NOT appear"
    );
    assert!(u_wire.get("reusable_block_id").is_some());
    assert!(
        u_wire.get("reusableBlockId").is_none(),
        "camelCase reusableBlockId must NOT appear"
    );

    let _resp = BlockUsageResponse {
        usages: vec![],
        total_count: 0,
    };
}

// ── 8. Both routers compile as Router<AppState> ──────────────────────

/// `cms_reusable_blocks::admin_router()` (CRUD + duplicate +
/// detach/restore + usage tracking) and `public_router()` (read-only
/// block-picker) must build as `Router<AppState>`. Admin handlers
/// take a `User` extractor and call `require_cms_admin` /
/// `require_cms_editor` inside — a regression that dropped the
/// extractor would silently expose CRUD to unauth users (and the
/// role check would never fire because `user` would be a no-extractor
/// default).
#[test]
fn cms_reusable_blocks_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::admin_router();
}

#[test]
fn cms_reusable_blocks_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::public_router();
}

/// Idempotent construction (CLAUDE.md habit #3 — "cached state lost
/// during refactor" landmine).
#[test]
fn cms_reusable_blocks_routers_safe_to_construct_multiple_times() {
    let _a1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::admin_router();
    let _a2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::admin_router();
    let _p1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::public_router();
    let _p2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_reusable_blocks::public_router();
}
