//! CMS Assets (DAM) route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_assets` and pins
//! the public DTOs + the `router()` mount table for the Digital
//! Asset Manager surface at `/cms/assets/*`. This is the **1444 LOC**
//! enterprise DAM API that backs every image / video / file the
//! marketing site and admin console handle.
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_assets.rs` requires the `AdminUser`
//! extractor (Redis session lookup) AND live SQL against the
//! `cms_assets`, `cms_asset_folders`, `cms_asset_usage` tables.
//! None can be invoked in unit-test isolation. What we CAN pin
//! without a DB:
//!
//! 1. **Router compile-pin** — `cms_assets::router()` must build as
//!    `Router<AppState>`. The documented mount table is 16 routes
//!    across asset list/search/CRUD, folder hierarchy CRUD, bulk
//!    operations (move/delete/tag), and usage tracking. A regression
//!    in any handler signature (wrong `State<>`, missing `AdminUser`,
//!    wrong return-type) fails compilation here.
//!
//! 2. **`Asset.file_size` is i64** — file_size is BIGINT in the DB
//!    (NOT BIGSERIAL — it's a measurement, not a PK). Files up to
//!    2 GiB are common for video uploads; an i32 cap would silently
//!    truncate the size of any file > 2.1 GB to a negative number,
//!    leading to "this file is -1 bytes" UI bugs. CLAUDE.md money
//!    rule applies by analogy: measurements that *accumulate* in
//!    rollups need i64 end-to-end (sum of all asset file_sizes is
//!    the storage-cost dashboard input).
//!
//! 3. **`AssetStats.total_size` and `recent_uploads` are i64** —
//!    these are aggregate rollups. The "total assets stored" tile
//!    on the admin dashboard SUMs file_size; that aggregate easily
//!    exceeds 2^31 bytes (just 100 video assets × 25 MB each = 2.5
//!    GB; total storage across the platform is in the TB range).
//!    Narrowing to i32 would wrap negative on overflow.
//!
//! 4. **Asset / Folder PKs are UUIDs, not i64** — this is the only
//!    exception in the codebase where a primary key is NOT i64
//!    BIGSERIAL. CMS assets use UUID for the "client-generated ID
//!    works without a roundtrip" optimization (the SPA assigns the
//!    UUID locally, uploads, then commits with the same UUID). Pin
//!    this explicitly so a future refactor that "standardized" to
//!    i64 BIGSERIAL would break here AND the SPA's optimistic
//!    upload flow simultaneously.
//!
//! 5. **`AssetListQuery` filters are all Optional** — a bare
//!    `GET /cms/assets/` (no filters) must NOT 400. This is the
//!    default DAM landing page. R9-D NEGATIVE pin: a regression
//!    that flipped any filter to required would 400 every default
//!    DAM page load.
//!
//! 6. **`CreateFolderRequest.name` is required** — R9-D NEGATIVE
//!    pin. A folder without a name would land in the DB with NULL
//!    name; the SPA tree-renderer would silently skip the row,
//!    creating "ghost folders" that the user can't see but contain
//!    real assets.
//!
//! 7. **Bulk DTOs require `asset_ids`** — `BulkMoveRequest`,
//!    `BulkDeleteRequest`, `BulkTagRequest` all require an
//!    `asset_ids: Vec<Uuid>` payload. A refactor that flipped this
//!    to Optional would let an admin POST `/bulk/delete {}` and
//!    have it succeed against ZERO assets (no-op), which masks
//!    bugs in the SPA's selection state.
//!
//! ## Money contract
//!
//! `routes/cms_assets.rs` exposes NO `*_cents` fields. File-size
//! tracking is BYTES (i64), not cents. The CLAUDE.md "i64 only"
//! rule applies here by analogy because:
//!   - file_size is BIGINT in PG schema.
//!   - aggregate storage-cost rollups SUM these values across
//!     potentially millions of assets.
//!   - the storage-cost dashboard is a money-adjacent surface
//!     downstream of these values.
//!
//! "Reserved exception" (revisions / attempts as i32) DOES apply to:
//!   - `Asset.usage_count: i32` — bounded by "how many times this
//!     asset is referenced in CMS content", which caps in the low
//!     thousands per asset on any realistic site.
//!   - `Asset.version: i32` — revision count, by the explicit
//!     CLAUDE.md exception.
//!   - `AssetFolder.depth: i32` — folder hierarchy depth, capped
//!     by the schema's recursive query depth (typically 10 levels).
//!   - Pixel dimensions (`width`, `height`) — bounded by image
//!     decoders.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/cms_delivery_test.rs`,
//! `tests/media_test.rs` (if exists).

use revolution_api::routes::cms_assets;
use revolution_api::routes::cms_assets::{
    Asset, AssetFolder, AssetListQuery, AssetStats, AssetSummary, BulkDeleteRequest,
    BulkMoveRequest, BulkTagRequest, CreateFolderRequest, PaginatedAssets, PaginationMeta,
    ReplaceAssetRequest, TypeStats, UpdateAssetRequest, UpdateFolderRequest, UploadConfirmRequest,
};
use uuid::Uuid;

// ── 1. router() mount-table compile-pin ──────────────────────────────

/// `cms_assets::router()` must build as `Router<AppState>`. The
/// documented mount table is 16 routes across asset list/search,
/// folder hierarchy CRUD, asset CRUD, bulk operations, and usage
/// tracking. A regression in any handler signature (wrong State<>,
/// missing AdminUser, wrong return-type) fails compilation here.
#[test]
fn cms_assets_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = cms_assets::router();
}

// ── 2. Asset.file_size is i64 (BIGINT — not BIGSERIAL) ──────────────

/// `Asset.file_size` is BIGINT in the DB. Files up to 2 GB are
/// common for video uploads. A narrowing to i32 would silently
/// truncate the size of any file > 2.1 GB to a negative number,
/// leading to "this file is -1 bytes" UI bugs and breaking the
/// storage-cost dashboard (which SUMs file_size).
///
/// CLAUDE.md money rule applies by analogy: measurements that
/// accumulate in rollups need i64 end-to-end. This is NOT the
/// "Reserved exception" (revisions/attempts) — file_size is
/// unbounded by user intent.
#[test]
fn asset_file_size_is_i64_above_i32_max() {
    use chrono::Utc;

    let above_i32_max: i64 = (i32::MAX as i64) + 1; // ~2.15 GB + 1 byte
    let now = Utc::now();

    let asset = Asset {
        id: Uuid::new_v4(),
        folder_id: None,
        filename: "huge_video.mp4".to_string(),
        original_filename: "huge_video.mp4".to_string(),
        mime_type: "video/mp4".to_string(),
        file_size: above_i32_max,
        file_extension: "mp4".to_string(),
        storage_provider: "r2".to_string(),
        storage_key: "videos/huge_video.mp4".to_string(),
        cdn_url: "https://cdn.example.com/videos/huge_video.mp4".to_string(),
        width: None,
        height: None,
        aspect_ratio: None,
        blurhash: None,
        dominant_color: None,
        duration_seconds: None,
        video_codec: None,
        audio_codec: None,
        bunny_video_id: None,
        bunny_library_id: None,
        thumbnail_url: None,
        variants: None,
        title: None,
        alt_text: None,
        caption: None,
        description: None,
        credits: None,
        seo_title: None,
        seo_description: None,
        tags: None,
        usage_count: 0,
        last_used_at: None,
        deleted_at: None,
        version: 1,
        created_at: now,
        updated_at: now,
        created_by: None,
        updated_by: None,
    };

    // Compile-pin
    let _: i64 = asset.file_size;

    let wire = serde_json::to_value(&asset).expect("Asset must serialize");
    assert_eq!(wire["file_size"].as_i64(), Some(above_i32_max));

    // Narrowing proof — a refactor that narrowed file_size to i32
    // would silently report "file is negative bytes"
    assert!(
        (above_i32_max as i32 as i64) != above_i32_max,
        "narrowing file_size to i32 must lose data — proves i64 is required (storage-cost SUM overflow is silent)"
    );

    // AssetSummary.file_size (lightweight projection) must also be i64
    let summary = AssetSummary {
        id: asset.id,
        folder_id: None,
        filename: asset.filename.clone(),
        mime_type: asset.mime_type.clone(),
        file_size: above_i32_max,
        cdn_url: asset.cdn_url.clone(),
        width: None,
        height: None,
        blurhash: None,
        thumbnail_url: None,
        title: None,
        alt_text: None,
        tags: None,
        usage_count: 0,
        created_at: now,
    };
    let _: i64 = summary.file_size;
    let s_wire = serde_json::to_value(&summary).expect("AssetSummary must serialize");
    assert_eq!(s_wire["file_size"].as_i64(), Some(above_i32_max));
}

// ── 3. AssetStats aggregate rollups are i64 ─────────────────────────

/// `AssetStats.total_size`, `total_assets`, `recent_uploads`, and
/// `unused_assets` are i64 — they're aggregate rollups across the
/// entire asset table. Total platform storage is in the TB range
/// (one TB ≈ 1.1 × 10^12 bytes, which exceeds i32::MAX × 500).
/// Narrowing any to i32 would wrap negative on overflow.
#[test]
fn asset_stats_aggregate_rollups_are_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let stats = AssetStats {
        total_assets: above_i32_max,
        total_size: above_i32_max * 1024, // multi-TB total
        total_size_formatted: "2.2 TB".to_string(),
        by_type: TypeStats {
            images: 1_000_000_000,
            videos: 500_000_000,
            audio: 100_000_000,
            documents: 50_000_000,
            other: above_i32_max,
        },
        recent_uploads: above_i32_max,
        unused_assets: above_i32_max,
    };

    // Compile-pins
    let _: i64 = stats.total_assets;
    let _: i64 = stats.total_size;
    let _: i64 = stats.recent_uploads;
    let _: i64 = stats.unused_assets;
    let _: i64 = stats.by_type.images;
    let _: i64 = stats.by_type.videos;
    let _: i64 = stats.by_type.audio;
    let _: i64 = stats.by_type.documents;
    let _: i64 = stats.by_type.other;

    let wire = serde_json::to_value(&stats).expect("AssetStats must serialize");
    assert_eq!(wire["total_assets"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["total_size"].as_i64(), Some(above_i32_max * 1024));
    assert_eq!(wire["by_type"]["other"].as_i64(), Some(above_i32_max));

    // Narrowing proof
    assert!(
        (above_i32_max as i32 as i64) != above_i32_max,
        "narrowing AssetStats rollups to i32 must lose data — storage-cost dashboard would show negative bytes"
    );
}

// ── 4. Asset / Folder PKs are UUID (not i64 BIGSERIAL) ──────────────

/// CMS assets use UUID for the primary key — the only exception in
/// the codebase where a PK is NOT i64 BIGSERIAL. The SPA's
/// optimistic upload flow generates the UUID locally and uploads
/// the file under that key BEFORE the row is committed, so the
/// asset has the same ID before and after the DB roundtrip.
///
/// Pin this explicitly so a future refactor that "standardized" to
/// i64 BIGSERIAL would break here AND the SPA's optimistic upload
/// simultaneously (the SPA would have an i64 PK on the wire but the
/// server would expect UUID, 400 every upload).
#[test]
fn asset_and_folder_primary_keys_are_uuid_not_i64() {
    use chrono::Utc;
    let now = Utc::now();

    let asset_id = Uuid::new_v4();
    let folder_id = Uuid::new_v4();

    let asset = Asset {
        id: asset_id,
        folder_id: Some(folder_id),
        filename: "x.png".to_string(),
        original_filename: "x.png".to_string(),
        mime_type: "image/png".to_string(),
        file_size: 1024,
        file_extension: "png".to_string(),
        storage_provider: "r2".to_string(),
        storage_key: "img/x.png".to_string(),
        cdn_url: "https://cdn.example.com/img/x.png".to_string(),
        width: Some(640),
        height: Some(480),
        aspect_ratio: None,
        blurhash: None,
        dominant_color: None,
        duration_seconds: None,
        video_codec: None,
        audio_codec: None,
        bunny_video_id: None,
        bunny_library_id: None,
        thumbnail_url: None,
        variants: None,
        title: None,
        alt_text: None,
        caption: None,
        description: None,
        credits: None,
        seo_title: None,
        seo_description: None,
        tags: None,
        usage_count: 0,
        last_used_at: None,
        deleted_at: None,
        version: 1,
        created_at: now,
        updated_at: now,
        created_by: None,
        updated_by: None,
    };

    // Compile-pins
    let _: Uuid = asset.id;
    let _: Option<Uuid> = asset.folder_id;

    // Wire serialization preserves Uuid as a hyphenated string
    let wire = serde_json::to_value(&asset).expect("Asset must serialize");
    assert_eq!(
        wire["id"].as_str().unwrap().len(),
        36,
        "Asset.id must serialize as a 36-char UUID string, NOT a number"
    );
    assert!(
        !wire["id"].is_number(),
        "Asset.id must NOT serialize as a number — that would break the SPA's optimistic upload (client-generated UUID)"
    );

    // AssetFolder PK is also UUID
    let folder = AssetFolder {
        id: folder_id,
        name: "screenshots".to_string(),
        slug: "screenshots".to_string(),
        parent_id: None,
        path: "/screenshots".to_string(),
        depth: 0,
        color: None,
        icon: None,
        asset_count: 0,
        created_at: now,
        updated_at: now,
        created_by: None,
    };
    let _: Uuid = folder.id;
    let f_wire = serde_json::to_value(&folder).expect("AssetFolder must serialize");
    assert!(
        !f_wire["id"].is_number(),
        "AssetFolder.id must NOT serialize as a number — UUID PK is load-bearing for the SPA tree-renderer"
    );

    // CLAUDE.md "Reserved exception" — these counters ARE i32 by
    // design (per the rule). Document the exception explicitly so a
    // future refactor doesn't incorrectly widen them.
    let _: i32 = asset.usage_count; // bounded by content references
    let _: i32 = asset.version; // revisions exception
    let _: i32 = folder.depth; // hierarchy depth (typically ≤ 10)
}

// ── 5. AssetListQuery filters all Optional (R9-D NEGATIVE pin) ──────

/// `AssetListQuery` must accept an empty query string. The default
/// DAM landing page hits `GET /cms/assets/` with no filters.
///
/// R9-D NEGATIVE pin: a regression that flipped any filter to
/// required would 400 every default DAM page load — the admin
/// would see an error toast and no asset grid would render.
#[test]
fn asset_list_query_all_fields_optional_negative_pin() {
    // Empty query — must parse (the default DAM landing page hit)
    let empty: AssetListQuery = serde_json::from_value(serde_json::json!({}))
        .expect("AssetListQuery must parse from empty body — default DAM landing page");
    assert!(empty.folder_id.is_none());
    assert!(empty.r#type.is_none());
    assert!(empty.search.is_none());
    assert!(empty.tags.is_none());
    assert!(empty.mime_type.is_none());
    assert!(empty.min_width.is_none());
    assert!(empty.max_width.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.sort_order.is_none());
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.include_deleted.is_none());

    // With all filters populated
    let full: AssetListQuery = serde_json::from_value(serde_json::json!({
        "folder_id": Uuid::new_v4().to_string(),
        "type": "image",
        "search": "logo",
        "tags": "brand,marketing",
        "mime_type": "image/png",
        "min_width": 100,
        "max_width": 2000,
        "sort_by": "created_at",
        "sort_order": "desc",
        "page": 1,
        "per_page": 50,
        "include_deleted": false,
    }))
    .expect("AssetListQuery with all filters must parse");
    assert_eq!(full.r#type.as_deref(), Some("image"));
    assert_eq!(full.page, Some(1));
    assert_eq!(full.per_page, Some(50));

    // UpdateAssetRequest / UpdateFolderRequest are PATCH-style — all
    // fields Optional so a partial update is legal.
    let upd_asset: UpdateAssetRequest = serde_json::from_value(serde_json::json!({}))
        .expect("UpdateAssetRequest must parse from empty body — PATCH semantics");
    assert!(upd_asset.title.is_none());

    let upd_folder: UpdateFolderRequest = serde_json::from_value(serde_json::json!({}))
        .expect("UpdateFolderRequest must parse from empty body — PATCH semantics");
    assert!(upd_folder.name.is_none());
}

// ── 6. Create + Bulk DTOs require structural fields (NEGATIVE pin) ──

/// `CreateFolderRequest.name` is required — a folder without a name
/// would land with NULL name, and the SPA tree-renderer would
/// silently skip it, creating "ghost folders" that contain real
/// assets but are invisible.
///
/// `BulkMoveRequest`, `BulkDeleteRequest`, `BulkTagRequest` all
/// require `asset_ids: Vec<Uuid>`. A refactor that flipped this
/// to Optional would let an admin POST `/bulk/delete {}` and have
/// it succeed against ZERO assets — masking bugs in the SPA's
/// selection state and giving a false "deleted 0 items" toast.
///
/// `UploadConfirmRequest` and `ReplaceAssetRequest` require
/// file_key + original_filename + mime_type + file_size.
#[test]
fn create_folder_and_bulk_dtos_required_fields_negative_pin() {
    // CreateFolderRequest — name required
    let valid: CreateFolderRequest = serde_json::from_value(serde_json::json!({
        "name": "screenshots",
    }))
    .expect("CreateFolderRequest with name must parse");
    assert_eq!(valid.name, "screenshots");

    let no_name = serde_json::from_value::<CreateFolderRequest>(serde_json::json!({}));
    assert!(
        no_name.is_err(),
        "CreateFolderRequest without name must fail — NULL name creates ghost folders"
    );

    // BulkMoveRequest — asset_ids required
    let bulk_move: BulkMoveRequest = serde_json::from_value(serde_json::json!({
        "asset_ids": [Uuid::new_v4().to_string()],
        "target_folder_id": null,
    }))
    .expect("BulkMoveRequest with asset_ids must parse");
    assert_eq!(bulk_move.asset_ids.len(), 1);

    let no_ids_move = serde_json::from_value::<BulkMoveRequest>(serde_json::json!({}));
    assert!(
        no_ids_move.is_err(),
        "BulkMoveRequest without asset_ids must fail — empty bulk op must be explicit, not accidental"
    );

    // BulkDeleteRequest — asset_ids required
    let bulk_del: BulkDeleteRequest = serde_json::from_value(serde_json::json!({
        "asset_ids": [Uuid::new_v4().to_string()],
    }))
    .expect("BulkDeleteRequest with asset_ids must parse");
    assert_eq!(bulk_del.asset_ids.len(), 1);
    assert!(bulk_del.permanent.is_none());

    let no_ids_del = serde_json::from_value::<BulkDeleteRequest>(serde_json::json!({}));
    assert!(
        no_ids_del.is_err(),
        "BulkDeleteRequest without asset_ids must fail — admins must not silently delete zero items"
    );

    // BulkTagRequest — asset_ids required, tag operations optional
    let bulk_tag: BulkTagRequest = serde_json::from_value(serde_json::json!({
        "asset_ids": [Uuid::new_v4().to_string()],
    }))
    .expect("BulkTagRequest with only asset_ids must parse (no-op tag is legal)");
    assert_eq!(bulk_tag.asset_ids.len(), 1);

    let no_ids_tag = serde_json::from_value::<BulkTagRequest>(serde_json::json!({}));
    assert!(
        no_ids_tag.is_err(),
        "BulkTagRequest without asset_ids must fail"
    );

    // UploadConfirmRequest — all four required (file_key,
    // original_filename, mime_type, file_size)
    let upload: UploadConfirmRequest = serde_json::from_value(serde_json::json!({
        "file_key": "uploads/abc.png",
        "original_filename": "logo.png",
        "mime_type": "image/png",
        "file_size": 12345_i64,
    }))
    .expect("UploadConfirmRequest with required fields must parse");
    assert_eq!(upload.file_key, "uploads/abc.png");
    // file_size compile-pin
    let _: i64 = upload.file_size;

    for missing in &["file_key", "original_filename", "mime_type", "file_size"] {
        let mut obj = serde_json::json!({
            "file_key": "uploads/abc.png",
            "original_filename": "logo.png",
            "mime_type": "image/png",
            "file_size": 12345_i64,
        });
        obj.as_object_mut().unwrap().remove(*missing);
        let r = serde_json::from_value::<UploadConfirmRequest>(obj);
        assert!(
            r.is_err(),
            "UploadConfirmRequest without {missing} must fail — upload confirmation must have all four required fields"
        );
    }

    // ReplaceAssetRequest — same four required fields as Upload
    let replace: ReplaceAssetRequest = serde_json::from_value(serde_json::json!({
        "file_key": "uploads/new.png",
        "original_filename": "logo-v2.png",
        "mime_type": "image/png",
        "file_size": 9999_i64,
    }))
    .expect("ReplaceAssetRequest with required fields must parse");
    let _: i64 = replace.file_size;

    // PaginatedAssets + PaginationMeta — wire shape pin (the DAM grid
    // uses these for pagination UI)
    let meta = PaginationMeta {
        total: (i32::MAX as i64) + 1,
        page: 1,
        per_page: 50,
        total_pages: 100,
        has_more: true,
    };
    let _: i64 = meta.total;
    let _: i64 = meta.page;
    let _: i64 = meta.per_page;
    let _: i64 = meta.total_pages;
    let m_wire = serde_json::to_value(&meta).expect("PaginationMeta must serialize");
    assert_eq!(m_wire["total"].as_i64(), Some((i32::MAX as i64) + 1));

    let page = PaginatedAssets { data: vec![], meta };
    let _ = serde_json::to_value(&page).expect("PaginatedAssets must serialize");
}
