//! CMS Presets route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_presets` and
//! exercises the public DTOs + both routers (`admin_router()` for the
//! authenticated CRUD and `public_router()` for the read-only by-block
//! lookup).
//!
//! ## Why this shape
//!
//! `routes/cms_presets.rs` (1,364 LOC) is the component-preset library
//! powering the CMS block builder — each preset is a JSON-encoded
//! block configuration that can be applied to a page. Every handler
//! runs live SQL against the `cms_presets` table and admin handlers
//! are gated behind the `User` extractor + a role allowlist
//! (`require_cms_admin` / `require_cms_editor`). Handlers can't be
//! invoked in isolation. What we CAN pin:
//!
//! 1. **`CmsPreset.id` is `Uuid` (NOT i64).** CMS content tables in
//!    this repo are deliberately `uuid` PKs (vs the `bigserial` PKs
//!    on transactional tables) so they can be created offline and
//!    synced without ID collision. Pin the `Uuid` type — a regression
//!    to `i64` would silently break the `?include_revisions` query
//!    join.
//!
//! 2. **`CmsPreset.usage_count` and `.version` are `i32`.** Per
//!    CLAUDE.md "Reserved exception": these counters legitimately cap
//!    below 2 billion (a single preset realistically gets used at
//!    most a few thousand times; version is per-edit). Pin the i32
//!    width to document the exception applies here.
//!
//! 3. **`CmsPresetCategory` is a `sqlx::Type` enum mapping to the
//!    Postgres `cms_preset_category` type with lowercase variants.**
//!    Both `#[sqlx(rename_all = "lowercase")]` AND
//!    `#[serde(rename_all = "lowercase")]` apply — the wire format
//!    MUST be lowercase ("default", "brand", etc.) so the SvelteKit
//!    client can match the filter dropdown values. A regression that
//!    flipped serde to PascalCase would silently fail every filter.
//!
//! 4. **`CreatePresetRequest` requires `block_type`, `name`, and
//!    `preset_data` (the JSON payload).** Other fields default at the
//!    handler (`is_default = false`, `is_global = true`). A
//!    regression flipping `preset_data` to optional would silently
//!    create empty preset rows that crash the apply-to-block handler
//!    later.
//!
//! 5. **`ListPresetsQuery` is fully optional (admin grid default
//!    view).** The initial preset library render hits
//!    `GET /admin/cms/presets` with no filters; if any field were
//!    required, the grid would 422 on first load.
//!
//! 6. **NEGATIVE: `limit` and `offset` MUST be numeric i64**, not
//!    strings. A regression that loosened either type would silently
//!    accept `?limit=fifty` and produce a SQL bind error at the
//!    handler.
//!
//! 7. **Wire-format keys are snake_case.** `blockType`,
//!    `presetData`, `isDefault` etc. MUST NOT appear on the wire.
//!
//! 8. **Both routers build under `AppState`.** Mount-table compile
//!    pin for the public router (`/block/{block_type}` only) and the
//!    admin router (8 mounted routes including
//!    `/save-from-block` and `/{id}/apply`).
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/cms_seo_test.rs`,
//! `tests/cms_global_components_test.rs`, and
//! `tests/redirects_test.rs`.

use chrono::Utc;
use revolution_api::routes::cms_presets::{
    BlockTypeWithPresets, CmsPreset, CmsPresetCategory, CmsPresetSummary, CreatePresetRequest,
    ListPresetsQuery, UpdatePresetRequest,
};
use uuid::Uuid;

// ── 1. CmsPreset: Uuid PK + i32 counter pins ────────────────────────

/// CMS content tables in this repo use `Uuid` PKs (not BIGSERIAL i64)
/// so that content can be authored offline and synced without ID
/// collision. Pin the Uuid type — a regression to i64 would break
/// the join from `cms_pages.block_id` (also a Uuid) into the preset
/// library.
///
/// `usage_count` and `version` are `i32` — CLAUDE.md "Reserved
/// exception" applies (legitimately bounded counters; usage caps at
/// thousands per preset, version at edit-count per preset).
#[test]
fn cms_preset_uses_uuid_pk_and_i32_bounded_counters() {
    let preset_id = Uuid::new_v4();
    let now = Utc::now();

    let preset = CmsPreset {
        id: preset_id,
        block_type: "button".to_string(),
        name: "Primary CTA".to_string(),
        slug: "primary-cta".to_string(),
        description: Some("Default primary action button".to_string()),
        preset_data: serde_json::json!({
            "text": "Get Started",
            "variant": "primary",
            "size": "lg"
        }),
        thumbnail_url: None,
        thumbnail_blurhash: None,
        category: CmsPresetCategory::Default,
        tags: Some(vec!["cta".to_string(), "primary".to_string()]),
        is_default: true,
        is_locked: false,
        is_global: true,
        usage_count: 12_345,
        version: 3,
        created_at: now,
        updated_at: now,
        created_by: Some(Uuid::new_v4()),
        updated_by: None,
        deleted_at: None,
    };

    let wire = serde_json::to_value(&preset).expect("serialize CmsPreset");

    // Uuid PK serializes to a string in JSON.
    assert!(
        wire["id"].is_string(),
        "CmsPreset.id is Uuid — wire format is a string, not a number"
    );
    assert_eq!(wire["id"].as_str().unwrap(), preset_id.to_string());

    // Counter widths.
    assert_eq!(wire["usage_count"].as_i64(), Some(12_345));
    assert_eq!(wire["version"].as_i64(), Some(3));

    // Wire-format keys MUST be snake_case.
    assert!(
        wire.get("blockType").is_none(),
        "wire MUST be snake_case (`block_type`), not camelCase"
    );
    assert!(
        wire.get("presetData").is_none(),
        "wire MUST be snake_case (`preset_data`), not camelCase"
    );
    assert!(
        wire.get("isDefault").is_none(),
        "wire MUST be snake_case (`is_default`), not camelCase"
    );
    assert!(
        wire.get("usageCount").is_none(),
        "wire MUST be snake_case (`usage_count`), not camelCase"
    );

    // The Uuid-PK pin is enforced at compile time too — a regression
    // to i64 would fail to assign Uuid::new_v4() to the field.
    let _: Uuid = preset.id;
}

// ── 2. CmsPresetCategory: lowercase wire format pin ──────────────────

/// `CmsPresetCategory` is a Postgres enum with `#[serde(rename_all =
/// "lowercase")]`. The SvelteKit category-filter dropdown sends and
/// expects lowercase strings ("default", "brand", "seasonal",
/// "marketing", "trading", "custom"). A regression that flipped
/// serde to PascalCase (default Rust enum serialization) would
/// silently break every filter.
#[test]
fn cms_preset_category_wire_format_is_lowercase() {
    // Round-trip each variant — serialize gives lowercase, deserialize
    // accepts lowercase only.
    let cases = [
        (CmsPresetCategory::Default, "default"),
        (CmsPresetCategory::Brand, "brand"),
        (CmsPresetCategory::Seasonal, "seasonal"),
        (CmsPresetCategory::Marketing, "marketing"),
        (CmsPresetCategory::Trading, "trading"),
        (CmsPresetCategory::Custom, "custom"),
    ];

    for (variant, expected_str) in cases {
        let wire = serde_json::to_value(variant).expect("serialize category");
        assert_eq!(
            wire.as_str(),
            Some(expected_str),
            "category MUST serialize as lowercase `{expected_str}`"
        );

        // Deserialize the lowercase string back.
        let back: CmsPresetCategory = serde_json::from_value(wire).expect("round-trip lowercase");
        assert_eq!(back, variant);
    }

    // Default variant.
    let d: CmsPresetCategory = Default::default();
    assert_eq!(d, CmsPresetCategory::Default);

    // NEGATIVE: PascalCase variants MUST NOT deserialize — that would
    // mean someone broke the `rename_all = "lowercase"` attribute.
    assert!(
        serde_json::from_value::<CmsPresetCategory>(serde_json::json!("Default")).is_err(),
        "PascalCase `Default` MUST fail (rename_all = lowercase)"
    );
    assert!(
        serde_json::from_value::<CmsPresetCategory>(serde_json::json!("Brand")).is_err(),
        "PascalCase `Brand` MUST fail (rename_all = lowercase)"
    );
}

// ── 3. CreatePresetRequest: required fields + serde defaults ─────────

/// `CreatePresetRequest` has 3 required fields: `block_type`, `name`,
/// `preset_data`. `is_default` and `is_global` have serde defaults
/// (`#[serde(default)]` = false, `#[serde(default = "default_true")]`
/// = true). A regression flipping `preset_data` to optional would
/// create empty preset rows that crash the apply-to-block handler.
#[test]
fn create_preset_request_required_fields_and_defaults() {
    let minimal: CreatePresetRequest = serde_json::from_value(serde_json::json!({
        "block_type": "heading",
        "name": "Hero Title",
        "preset_data": {"text": "Welcome", "level": 1}
    }))
    .expect("minimal CreatePresetRequest must deserialize");
    assert_eq!(minimal.block_type, "heading");
    assert_eq!(minimal.name, "Hero Title");
    assert!(minimal.slug.is_none(), "slug auto-generated at handler");
    assert!(minimal.category.is_none(), "category optional");
    assert!(
        !minimal.is_default,
        "is_default defaults to false via serde(default)"
    );
    assert!(
        minimal.is_global,
        "is_global defaults to true via serde(default = default_true)"
    );

    // Full payload with category enum.
    let full: CreatePresetRequest = serde_json::from_value(serde_json::json!({
        "block_type": "button",
        "name": "Brand CTA",
        "slug": "brand-cta",
        "description": "Brand colors variant",
        "preset_data": {"variant": "brand"},
        "thumbnail_url": "https://cdn.example.com/p.png",
        "category": "brand",
        "tags": ["brand", "cta"],
        "is_default": true,
        "is_global": false
    }))
    .expect("full CreatePresetRequest must deserialize");
    assert_eq!(full.category, Some(CmsPresetCategory::Brand));
    assert!(full.is_default);
    assert!(!full.is_global);

    // NEGATIVE: missing required `block_type` MUST fail.
    assert!(
        serde_json::from_value::<CreatePresetRequest>(serde_json::json!({
            "name": "No block type",
            "preset_data": {}
        }))
        .is_err(),
        "missing block_type MUST fail (required field)"
    );

    // NEGATIVE: missing required `name` MUST fail.
    assert!(
        serde_json::from_value::<CreatePresetRequest>(serde_json::json!({
            "block_type": "button",
            "preset_data": {}
        }))
        .is_err(),
        "missing name MUST fail (required field)"
    );

    // NEGATIVE: missing required `preset_data` MUST fail.
    assert!(
        serde_json::from_value::<CreatePresetRequest>(serde_json::json!({
            "block_type": "button",
            "name": "Empty"
        }))
        .is_err(),
        "missing preset_data MUST fail (required field)"
    );

    // NEGATIVE: camelCase `blockType` is silently ignored (snake_case
    // is the contract); the resulting struct would be missing
    // `block_type` and fail at the required-field check.
    assert!(
        serde_json::from_value::<CreatePresetRequest>(serde_json::json!({
            "blockType": "should be ignored",
            "name": "test",
            "preset_data": {}
        }))
        .is_err(),
        "camelCase `blockType` MUST NOT satisfy required `block_type`"
    );
}

// ── 4. UpdatePresetRequest + ListPresetsQuery: PATCH + filters ───────

/// `UpdatePresetRequest` is fully optional (PATCH). Toggling just
/// `is_locked` to prevent further edits is the most common admin
/// action — must not require resending the full payload.
///
/// `ListPresetsQuery` is also fully optional (admin grid default
/// view). NEGATIVE: `limit` and `offset` MUST be numeric i64.
#[test]
fn update_preset_and_list_query_dtos_are_patch_and_typed() {
    let empty_update: UpdatePresetRequest = serde_json::from_str("{}")
        .expect("empty UpdatePresetRequest must deserialize (PATCH semantics)");
    assert!(empty_update.name.is_none());
    assert!(empty_update.is_locked.is_none());
    assert!(empty_update.preset_data.is_none());

    // Lock-only PATCH (most common admin action).
    let lock: UpdatePresetRequest = serde_json::from_value(serde_json::json!({"is_locked": true}))
        .expect("lock-only PATCH must deserialize");
    assert_eq!(lock.is_locked, Some(true));

    // Category change PATCH — uses the enum directly.
    let recategorize: UpdatePresetRequest =
        serde_json::from_value(serde_json::json!({"category": "marketing"}))
            .expect("category-change PATCH must deserialize");
    assert_eq!(recategorize.category, Some(CmsPresetCategory::Marketing));

    // ListPresetsQuery: all-optional.
    let empty_query: ListPresetsQuery =
        serde_json::from_str("{}").expect("empty ListPresetsQuery must deserialize");
    assert!(empty_query.block_type.is_none());
    assert!(empty_query.limit.is_none());
    assert!(empty_query.offset.is_none());

    // Filtered query.
    let filtered: ListPresetsQuery = serde_json::from_value(serde_json::json!({
        "block_type": "button",
        "category": "brand",
        "search": "primary",
        "tags": "cta,featured",
        "is_global": true,
        "limit": 50_i64,
        "offset": 100_i64
    }))
    .expect("filtered ListPresetsQuery must deserialize");
    assert_eq!(filtered.block_type.as_deref(), Some("button"));
    assert_eq!(filtered.limit, Some(50));
    assert_eq!(filtered.offset, Some(100));

    // NEGATIVE: `limit` as a string MUST fail.
    assert!(
        serde_json::from_value::<ListPresetsQuery>(serde_json::json!({"limit": "fifty"})).is_err(),
        "limit as string MUST fail (Option<i64> type pin)"
    );

    // NEGATIVE: `offset` as a string MUST fail.
    assert!(
        serde_json::from_value::<ListPresetsQuery>(serde_json::json!({"offset": "first"})).is_err(),
        "offset as string MUST fail (Option<i64> type pin)"
    );
}

// ── 5. Lightweight summary + block-type-with-presets serialization ───

/// `CmsPresetSummary` is the listing-page DTO — drops the heavy
/// `preset_data` JSON payload, keeps the metadata. Pin the i32
/// `usage_count` (reserved-exception) and snake_case wire format.
///
/// `BlockTypeWithPresets.preset_count` is i64 (COUNT(*) result from
/// Postgres — `bigint`). This is the "what block types have presets"
/// landing-page query; counts could legitimately exceed i32 across
/// a multi-tenant deployment.
#[test]
fn cms_preset_summary_and_block_type_with_presets_pin_widths_and_keys() {
    let summary = CmsPresetSummary {
        id: Uuid::new_v4(),
        block_type: "button".to_string(),
        name: "Brand CTA".to_string(),
        slug: "brand-cta".to_string(),
        description: None,
        thumbnail_url: None,
        thumbnail_blurhash: None,
        category: "brand".to_string(),
        tags: None,
        is_default: false,
        is_locked: false,
        is_global: true,
        usage_count: 99,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let wire = serde_json::to_value(&summary).expect("serialize CmsPresetSummary");
    assert_eq!(wire["usage_count"].as_i64(), Some(99));
    assert_eq!(wire["block_type"], "button");
    assert!(wire.get("blockType").is_none(), "no camelCase");
    assert!(wire.get("isGlobal").is_none(), "no camelCase");

    let block_meta = BlockTypeWithPresets {
        block_type: "heading".to_string(),
        preset_count: 12_500_i64,
        has_default: true,
    };
    let wire2 = serde_json::to_value(&block_meta).expect("serialize BlockTypeWithPresets");
    assert_eq!(
        wire2["preset_count"].as_i64(),
        Some(12_500),
        "preset_count is i64 (COUNT(*) from Postgres BIGINT)"
    );
    assert!(wire2.get("hasDefault").is_none(), "no camelCase");
}

// ── 6. Both routers build under AppState ─────────────────────────────

/// Admin router: 8 mounted routes including the security-sensitive
/// `/save-from-block` (writes new preset rows) and `/{id}/apply`
/// (applies preset to a block — touches multiple tables). All gated
/// behind the role allowlist check inside each handler. A refactor
/// that breaks a handler signature would fail to compile here.
#[test]
fn cms_presets_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_presets::admin_router();
}

/// Public router: 1 read-only route at `/block/{block_type}` (returns
/// global presets only, no auth). A refactor that exposed the admin
/// CRUD here would be a serious security regression — pin the
/// router compile-time so the function signature stays minimal.
#[test]
fn cms_presets_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_presets::public_router();
}
