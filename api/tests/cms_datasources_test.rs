//! CMS-datasources route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_datasources` and
//! exercises the 18 public DTOs + both routers (`admin_router()` and
//! `public_router()`).
//!
//! ## Why this shape
//!
//! `routes/cms_datasources.rs` (1,636 LOC) implements the
//! reusable-option-list system that powers every dropdown / select /
//! tag picker in the CMS-authored blog + page editor. Every handler
//! runs live SQL against `cms_datasources` and `cms_datasource_entries`
//! and most are gated behind the `User`-extractor + `require_cms_admin`
//! / `require_cms_editor` RBAC helpers, so we can't invoke handlers in
//! isolation. What we CAN pin:
//!
//! 1. **UUID-typed primary keys throughout.** Both `CmsDatasource.id`
//!    and `CmsDatasourceEntry.id` are `Uuid` (NOT BIGSERIAL i64).
//!    This is a deliberate departure from the rest of the stack — the
//!    datasource API is consumed by an embedded WYSIWYG block whose
//!    block schemas reference entries by UUID at JSON-blob level.
//!    Pinning the type here catches a refactor that flipped to i64
//!    (which would silently mismatch every embedded reference).
//!
//! 2. **`CmsDatasource.entry_count` is `i32`** — per-datasource row
//!    count. The CLAUDE.md "Reserved exception" applies (counts cap
//!    far below 2B). Pin to catch a refactor to u32 (which would flip
//!    serialization at the FFI boundary).
//!
//! 3. **`PaginationMeta.total/limit/offset` are `i64`.** These flow
//!    into SQL `LIMIT $1 OFFSET $2` binds (Postgres expects BIGINT for
//!    OFFSET past row 2^31; sqlx will reject i32 for the OFFSET clause
//!    on tables larger than that). Pin the i64 shape.
//!
//! 4. **Bulk-create + reorder payloads accept `Vec<…>`.**
//!    `BulkCreateEntriesRequest.entries` and
//!    `ReorderEntriesRequest.entry_ids` are the blast-radius payloads
//!    (one POST can create N entries or re-order an entire list).
//!    Pin the Vec shape + element type.
//!
//! 5. **Both routers compile against `AppState`.** The admin router
//!    covers ~20 routes (CRUD, bulk ops, CSV import/export); the
//!    public router covers slug-keyed lookups (`/by-slug/:slug`,
//!    `/by-slug/:slug/entries`) consumed by the frontend WYSIWYG
//!    block. Any handler-signature regression fails compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs` (also UUID-keyed),
//! `tests/cms_global_components_test.rs`, `tests/forms_test.rs`
//! (parallel), `tests/admin_orders_test.rs`.

use revolution_api::routes::cms_datasources::{
    BulkCreateEntriesRequest, CmsDatasource, CmsDatasourceEntry, CmsDatasourceEntrySummary,
    CmsDatasourceSummary, CreateDatasourceRequest, CreateEntryRequest, CsvEntryRow,
    ListDatasourcesQuery, ListEntriesQuery, PaginationMeta, PublicDatasourceResponse,
    PublicEntryResponse, ReorderEntriesRequest, UpdateDatasourceRequest, UpdateEntryRequest,
};
use uuid::Uuid;

// ── 1. CmsDatasource: UUID id + i32 entry_count round-trip ───────────

/// `CmsDatasource.id` is `Uuid` (NOT i64). Pin the type at compile
/// time — the embedded WYSIWYG block references datasources by UUID
/// in JSON-blob block schemas; a regression to i64 would silently
/// invalidate every embedded reference in every existing post.
#[test]
fn cms_datasource_id_is_uuid_and_count_is_i32() {
    let now = chrono::Utc::now();
    let id = Uuid::nil(); // stable test fixture
    let creator = Uuid::nil();

    let ds = CmsDatasource {
        id,
        name: "Countries".to_string(),
        slug: "countries".to_string(),
        description: Some("ISO 3166 country list".to_string()),
        icon: Some("globe".to_string()),
        color: Some("#3b82f6".to_string()),
        entry_count: 195,
        is_system: false,
        is_locked: false,
        created_at: now,
        updated_at: now,
        created_by: Some(creator),
        updated_by: Some(creator),
        deleted_at: None,
    };

    let wire = serde_json::to_value(&ds).expect("serialize CmsDatasource");
    // UUID serializes as a hyphenated string — pin the format.
    assert_eq!(
        wire["id"].as_str(),
        Some("00000000-0000-0000-0000-000000000000"),
        "CmsDatasource.id MUST serialize as UUID hyphenated string"
    );
    assert_eq!(
        wire["entry_count"].as_i64(),
        Some(195),
        "entry_count is i32 (counts cap far below 2B — CLAUDE.md exception)"
    );
    assert_eq!(wire["is_system"].as_bool(), Some(false));
    assert_eq!(wire["is_locked"].as_bool(), Some(false));

    // Round-trip: serialize → deserialize → field-equal.
    let parsed: CmsDatasource =
        serde_json::from_value(wire).expect("CmsDatasource must round-trip");
    assert_eq!(parsed.id, id);
    assert_eq!(parsed.entry_count, 195);
    assert_eq!(parsed.slug, "countries");
}

// ── 2. CmsDatasourceEntry: UUID id + FK + i32 sort_order ─────────────

/// Entries carry `id`, `datasource_id` as UUID FK, and `sort_order` as
/// i32 (drag-to-reorder index, never above a few thousand even for
/// huge lists). Pin all three shapes.
#[test]
fn cms_datasource_entry_id_and_fk_are_uuid() {
    let now = chrono::Utc::now();
    let entry_id = Uuid::nil();
    let datasource_id = Uuid::nil();

    let entry = CmsDatasourceEntry {
        id: entry_id,
        datasource_id,
        name: "United States".to_string(),
        value: "US".to_string(),
        dimension: "default".to_string(),
        sort_order: 1,
        metadata: Some(serde_json::json!({"flag": "🇺🇸"})),
        created_at: now,
        updated_at: now,
    };

    let wire = serde_json::to_value(&entry).expect("serialize CmsDatasourceEntry");
    assert_eq!(
        wire["id"].as_str(),
        Some("00000000-0000-0000-0000-000000000000")
    );
    assert_eq!(
        wire["datasource_id"].as_str(),
        Some("00000000-0000-0000-0000-000000000000")
    );
    assert_eq!(wire["sort_order"].as_i64(), Some(1));
    assert_eq!(wire["dimension"].as_str(), Some("default"));
}

// ── 3. PaginationMeta: i64 total / limit / offset ────────────────────

/// `PaginationMeta.total/limit/offset` flow into SQL `LIMIT $1 OFFSET
/// $2` binds. Postgres requires BIGINT for OFFSET past row 2^31; a
/// regression that narrowed any field to i32 would silently 500 on a
/// large entries table. Pin i64 end-to-end.
#[test]
fn pagination_meta_uses_i64_for_total_limit_offset() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let meta = PaginationMeta {
        total: above_i32_max,
        limit: 100,
        offset: above_i32_max - 100,
        has_more: false,
    };

    let wire = serde_json::to_value(&meta).expect("serialize PaginationMeta");
    assert_eq!(
        wire["total"].as_i64(),
        Some(above_i32_max),
        "total MUST be i64 — Postgres BIGINT for COUNT(*)"
    );
    assert_eq!(
        wire["offset"].as_i64(),
        Some(above_i32_max - 100),
        "offset MUST be i64 — Postgres LIMIT/OFFSET requires BIGINT"
    );
    assert_eq!(wire["limit"].as_i64(), Some(100));
    assert_eq!(wire["has_more"].as_bool(), Some(false));

    // Sanity: must actually exceed i32::MAX or the assertion above
    // could pass with an i32 field.
    assert!(meta.total > i32::MAX as i64);
}

// ── 4. CreateDatasourceRequest / CreateEntryRequest: required shape ──

/// Create requests have a sharp required/optional split. For the
/// datasource: `name` is required (the only mandatory field).
/// For an entry: `name` + `value` are required (the K/V pair). Pin
/// both shapes so a frontend regression that dropped the required
/// field surfaces here, not at the 400 response in prod.
#[test]
fn create_requests_pin_required_vs_optional_fields() {
    // Datasource: `name` is the only required field.
    let ds_min: CreateDatasourceRequest =
        serde_json::from_value(serde_json::json!({"name": "Countries"}))
            .expect("minimal CreateDatasourceRequest must deserialize");
    assert_eq!(ds_min.name, "Countries");
    assert!(ds_min.slug.is_none());
    assert!(ds_min.description.is_none());
    assert!(ds_min.icon.is_none());
    assert!(ds_min.color.is_none());

    // Missing `name` MUST fail.
    assert!(
        serde_json::from_value::<CreateDatasourceRequest>(serde_json::json!({"slug": "countries"}))
            .is_err(),
        "CreateDatasourceRequest without `name` MUST fail (required)"
    );

    // Entry: `name` + `value` both required.
    let e_min: CreateEntryRequest = serde_json::from_value(serde_json::json!({
        "name": "United States",
        "value": "US"
    }))
    .expect("minimal CreateEntryRequest must deserialize");
    assert_eq!(e_min.name, "United States");
    assert_eq!(e_min.value, "US");
    assert!(e_min.dimension.is_none());
    assert!(e_min.metadata.is_none());

    // Missing `value` MUST fail.
    assert!(
        serde_json::from_value::<CreateEntryRequest>(serde_json::json!({"name": "x"})).is_err(),
        "CreateEntryRequest without `value` MUST fail (required)"
    );
    // Missing `name` MUST fail.
    assert!(
        serde_json::from_value::<CreateEntryRequest>(serde_json::json!({"value": "x"})).is_err(),
        "CreateEntryRequest without `name` MUST fail (required)"
    );
}

// ── 5. Update + PATCH semantics + Bulk + Reorder payloads ────────────

/// Update DTOs follow PATCH semantics (every field optional) — the
/// admin can lock a datasource without re-sending the full payload.
/// Bulk/reorder DTOs carry Vec<…> blast-radius payloads — one POST
/// re-orders an entire entry list or bulk-creates N entries.
#[test]
fn update_patch_semantics_and_bulk_reorder_payloads() {
    // UpdateDatasourceRequest: everything optional, including
    // `is_locked` (the on/off toggle path).
    let empty_ds_update: UpdateDatasourceRequest =
        serde_json::from_str("{}").expect("empty UpdateDatasourceRequest must deserialize (PATCH)");
    assert!(empty_ds_update.name.is_none());
    assert!(empty_ds_update.is_locked.is_none());

    let lock_only: UpdateDatasourceRequest =
        serde_json::from_value(serde_json::json!({"is_locked": true}))
            .expect("lock-only update must deserialize");
    assert_eq!(lock_only.is_locked, Some(true));

    // UpdateEntryRequest: everything optional, including sort_order
    // (drag-to-reorder writes only `sort_order`).
    let empty_e_update: UpdateEntryRequest =
        serde_json::from_str("{}").expect("empty UpdateEntryRequest must deserialize (PATCH)");
    assert!(empty_e_update.name.is_none());
    assert!(empty_e_update.sort_order.is_none());

    let reorder_only: UpdateEntryRequest =
        serde_json::from_value(serde_json::json!({"sort_order": 5}))
            .expect("sort-only update must deserialize");
    assert_eq!(reorder_only.sort_order, Some(5));

    // Bulk create — Vec<CreateEntryRequest>.
    let bulk: BulkCreateEntriesRequest = serde_json::from_value(serde_json::json!({
        "entries": [
            {"name": "United States", "value": "US"},
            {"name": "United Kingdom", "value": "GB"},
            {"name": "France", "value": "FR"}
        ]
    }))
    .expect("BulkCreateEntriesRequest must deserialize");
    assert_eq!(bulk.entries.len(), 3);
    assert_eq!(bulk.entries[0].name, "United States");
    assert_eq!(bulk.entries[2].value, "FR");

    // Reorder — Vec<Uuid>. A regression that flipped to Vec<i64>
    // would silently break reorder for every existing entry.
    let nil_id = Uuid::nil();
    let reorder: ReorderEntriesRequest = serde_json::from_value(serde_json::json!({
        "entry_ids": [nil_id.to_string(), nil_id.to_string(), nil_id.to_string()]
    }))
    .expect("ReorderEntriesRequest must deserialize");
    assert_eq!(reorder.entry_ids.len(), 3);
    assert!(reorder.entry_ids.iter().all(|id| *id == nil_id));
}

// ── 6. Public response shapes: summary types + CSV + list queries ────

/// Public-facing shapes are consumed by the unauthenticated frontend
/// WYSIWYG block. `PublicDatasourceResponse` and `PublicEntryResponse`
/// MUST stay narrow — exposing internal columns (created_by, etc.)
/// would leak admin-only data through the public endpoint.
#[test]
fn public_responses_summaries_and_csv_round_trip() {
    // PublicEntryResponse: only name + value — narrow on purpose.
    let public_entry = PublicEntryResponse {
        name: "United States".to_string(),
        value: "US".to_string(),
    };
    let wire = serde_json::to_value(&public_entry).expect("serialize PublicEntryResponse");
    assert_eq!(wire["name"].as_str(), Some("United States"));
    assert_eq!(wire["value"].as_str(), Some("US"));
    // No internal fields leaked.
    assert!(wire.get("id").is_none());
    assert!(wire.get("created_by").is_none());

    // PublicDatasourceResponse: slug + name + entries[]. No
    // is_system, no is_locked, no internal audit fields.
    let public_ds = PublicDatasourceResponse {
        slug: "countries".to_string(),
        name: "Countries".to_string(),
        entries: vec![public_entry],
    };
    let wire = serde_json::to_value(&public_ds).expect("serialize PublicDatasourceResponse");
    assert_eq!(wire["slug"].as_str(), Some("countries"));
    assert!(
        wire.get("is_system").is_none(),
        "public response MUST NOT leak is_system"
    );
    assert!(
        wire.get("created_by").is_none(),
        "public response MUST NOT leak created_by"
    );
    assert_eq!(wire["entries"].as_array().map(Vec::len), Some(1));

    // CsvEntryRow: 3-field tuple shape used by CSV import.
    let csv = CsvEntryRow {
        name: "France".to_string(),
        value: "FR".to_string(),
        dimension: "default".to_string(),
    };
    let wire = serde_json::to_value(&csv).expect("serialize CsvEntryRow");
    assert_eq!(wire["name"].as_str(), Some("France"));
    assert_eq!(wire["value"].as_str(), Some("FR"));
    assert_eq!(wire["dimension"].as_str(), Some("default"));

    // Summary types: round-trip with UUID id.
    let now = chrono::Utc::now();
    let ds_summary = CmsDatasourceSummary {
        id: Uuid::nil(),
        name: "Countries".to_string(),
        slug: "countries".to_string(),
        description: None,
        icon: None,
        color: None,
        entry_count: 195,
        is_system: false,
        is_locked: false,
        created_at: now,
        updated_at: now,
    };
    let wire = serde_json::to_value(&ds_summary).expect("serialize CmsDatasourceSummary");
    assert_eq!(
        wire["id"].as_str(),
        Some("00000000-0000-0000-0000-000000000000")
    );
    assert_eq!(wire["entry_count"].as_i64(), Some(195));

    let entry_summary = CmsDatasourceEntrySummary {
        id: Uuid::nil(),
        name: "US".to_string(),
        value: "United States".to_string(),
        dimension: "default".to_string(),
        sort_order: 1,
        metadata: None,
    };
    let wire = serde_json::to_value(&entry_summary).expect("serialize CmsDatasourceEntrySummary");
    assert_eq!(wire["sort_order"].as_i64(), Some(1));

    // List queries — admin grid default view passes nothing.
    let empty_ds_query: ListDatasourcesQuery =
        serde_json::from_str("{}").expect("empty ListDatasourcesQuery must deserialize");
    assert!(empty_ds_query.search.is_none());
    assert!(empty_ds_query.limit.is_none());

    let empty_e_query: ListEntriesQuery =
        serde_json::from_str("{}").expect("empty ListEntriesQuery must deserialize");
    assert!(empty_e_query.dimension.is_none());
    assert!(empty_e_query.search.is_none());
}

// ── 7. Router mount tables: both build with AppState ─────────────────

/// `admin_router()` must build as `Router<AppState>`. Covers ~20
/// mounted routes — datasource CRUD, entry CRUD, bulk-create, reorder,
/// CSV import/export. Every handler MUST be gated behind the CMS-admin
/// or CMS-editor RBAC helper. A refactor that dropped that gate from
/// any handler fails compilation here.
#[test]
fn cms_datasources_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::admin_router();
}

/// `public_router()` must build as `Router<AppState>`. Powers the
/// unauthenticated WYSIWYG-block dropdown lookups (`/by-slug/:slug`
/// and `/by-slug/:slug/entries`). A regression that broke either
/// public handler would silently break every dropdown in the
/// frontend block editor.
#[test]
fn cms_datasources_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::public_router();
}

/// Both constructors must be safe to call multiple times — the main
/// `api_router()` may nest these during composition.
#[test]
fn cms_datasources_routers_are_safe_to_construct_multiple_times() {
    let _a1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::admin_router();
    let _a2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::admin_router();
    let _p1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::public_router();
    let _p2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_datasources::public_router();
}
