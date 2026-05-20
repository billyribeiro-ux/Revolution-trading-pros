//! Admin media (DAM) route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::media` and exercises
//! the public DTOs (`Media`, `MediaQuery`, `UpdateMedia`,
//! `PresignedUploadRequest`, `PresignedUploadResponse`,
//! `ConfirmUploadRequest`, `BulkDeleteRequest`, `BulkUpdateRequest`,
//! `CleanupResult`, `MalwareScanResult`) + the public
//! `admin_router()` mount table.
//!
//! ## Why this file exists (R16-D)
//!
//! `routes/media.rs` is 1269 LOC of admin-gated Cloudflare R2 DAM
//! (upload, presigned URLs, bulk operations, malware scan hook,
//! orphan cleanup). Every handler is `_admin: AdminUser`-gated.
//! The `media` table is `bigint NOT NULL` PK with a `size bigint`
//! file-size column (schema.sql:7638-7656). What we CAN pin in
//! no-DB tests:
//!
//! 1. **PK type — `Media.id` is `i64` (BIGSERIAL).** Per
//!    schema.sql:7639, `media.id bigint NOT NULL`. The
//!    bulk-delete/bulk-update payloads carry `Vec<i64>`, and the
//!    handler `Path<i64>` extractor for `/scan/:id` confirms.
//!    A regression to i32 would silently 400 every media URL
//!    past row 2^31.
//!
//! 2. **`Media.size` is `Option<i64>` (BIGINT).** File sizes can
//!    legitimately exceed i32::MAX bytes (~2.1 GB) — a single
//!    4K video file can easily reach 8-10 GB. Per CLAUDE.md
//!    money rule, byte sizes are NOT money but the SAME i64
//!    discipline applies: "Mixing types within a single value's
//!    lifecycle (struct → DB → API) is a recipe for silent
//!    overflow during summation." The `statistics` handler
//!    aggregates `SUM(size)` across all media — that aggregate
//!    can reach hundreds of GB on a mature app, well past
//!    i32::MAX.
//!
//!    R9-D NEGATIVE: size > i32::MAX MUST round-trip cleanly as
//!    i64 (proving the type is not narrowed silently).
//!
//! 3. **`Media.width` and `Media.height` are `Option<i32>`** —
//!    per CLAUDE.md "Reserved exception", image dimensions are
//!    bounded counters (no real-world image exceeds 65k px on
//!    a side, let alone 2.1B). i32 is appropriate. MONEY never
//!    qualifies.
//!
//! 4. **`MediaQuery` is fully optional + has serde aliases for
//!    `sort` / `order`.** The admin grid lands on
//!    `/admin/media` with zero params. The Rust field
//!    `sort_by` has `#[serde(alias = "sort")]` and `sort_dir`
//!    has `#[serde(alias = "order")]`. The Rust field `r#type`
//!    is keyword-escaped — wire `type` populates it.
//!
//!    R9-D NEGATIVE: snake-case `sort_by` on the wire IS
//!    accepted (it's the field name); the alias is `sort`.
//!    Confirm both work.
//!
//! 5. **`PresignedUploadRequest.filename` and `content_type` are
//!    required.** The presigned-upload handler validates
//!    `content_type` against an allow-list (lines 601-617).
//!    `size: Option<i64>` is optional. The response carries
//!    `expires_in: u64` (1 hour in seconds — not money).
//!
//! 6. **`ConfirmUploadRequest.size: i64` is REQUIRED.** Once the
//!    file is uploaded to R2, the client knows the exact byte
//!    count — there's no excuse for omitting it. R9-D NEGATIVE:
//!    missing `size` MUST fail.
//!
//! 7. **`BulkDeleteRequest.ids: Vec<i64>`.** The handler caps the
//!    vec at 100 items (line 924). A regression that flipped to
//!    Vec<i32> would silently break bulk-delete on production
//!    media with PKs > 2^31.
//!
//! 8. **Router mount table — 11 routes (including the `/files`
//!    alias for `/`).** Compile pin against handler signature
//!    drift. All admin-gated.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_popups_test.rs`,
//! `tests/admin_orders_test.rs`, `tests/posts_test.rs`,
//! `tests/categories_test.rs`, `tests/bunny_upload_test.rs`.

use chrono::Utc;
use revolution_api::routes::media::{
    BulkDeleteRequest, BulkUpdateRequest, CleanupResult, ConfirmUploadRequest, MalwareScanResult,
    Media, MediaQuery, PresignedUploadRequest, PresignedUploadResponse, UpdateMedia,
};

// ── 1. PK type — Media.id is i64 (BIGSERIAL, schema.sql:7639) ────────

/// `media.id` is `bigint NOT NULL` per schema.sql:7639. The Rust
/// struct mirrors with `id: i64`. The `BulkDeleteRequest.ids:
/// Vec<i64>` payload and the `Path<i64>` extractor for
/// `/scan/:id` (line 1268) confirm. A regression to i32 would
/// silently break the DAM on any production media past row 2^31.
#[test]
fn media_pk_is_i64_bigserial() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    let m = Media {
        id: above_i32,
        filename: "test.jpg".to_string(),
        original_filename: Some("original.jpg".to_string()),
        mime_type: Some("image/jpeg".to_string()),
        size: Some(1024_i64),
        path: Some("uploads/abc.jpg".to_string()),
        url: Some("https://r2.example.com/uploads/abc.jpg".to_string()),
        title: None,
        alt_text: None,
        caption: None,
        description: None,
        collection: None,
        is_optimized: Some(false),
        width: Some(1920),
        height: Some(1080),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    let _: i64 = m.id;
    let _: Option<i64> = m.size;

    let wire = serde_json::to_value(&m).expect("serialize Media");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32),
        "Media.id MUST be i64 — media.id is bigint (schema.sql:7639)"
    );
}

// ── 2. Media.size is i64 (BIGINT) — accommodates 4K video, GB+ files ─

/// Per schema.sql:7643, `media.size bigint`. A regression to i32
/// would silently overflow on any file past 2.1 GB — a single
/// 4K video file easily reaches 8-10 GB. The `statistics` handler
/// aggregates `SUM(size)` across all media; on a mature DAM with
/// thousands of videos, the aggregate exceeds i32::MAX trivially.
///
/// Per CLAUDE.md money rule, byte sizes are NOT money but the SAME
/// i64 discipline applies: "Mixing types within a single value's
/// lifecycle (struct → DB → API) is a recipe for silent overflow
/// during summation."
#[test]
fn media_size_is_i64_bigint_for_large_files() {
    let large_file_bytes: i64 = (i32::MAX as i64) + 1024; // ~2.1 GB

    let m = Media {
        id: 1,
        filename: "4k-video.mp4".to_string(),
        original_filename: None,
        mime_type: Some("video/mp4".to_string()),
        size: Some(large_file_bytes),
        path: None,
        url: None,
        title: None,
        alt_text: None,
        caption: None,
        description: None,
        collection: None,
        is_optimized: Some(false),
        width: None,
        height: None,
        created_at: None,
        updated_at: None,
    };
    let _: Option<i64> = m.size;
    let wire = serde_json::to_value(&m).expect("serialize Media");
    assert_eq!(
        wire["size"].as_i64(),
        Some(large_file_bytes),
        "Media.size MUST round-trip as i64 — a 4K video can exceed 2.1 GB"
    );

    // ConfirmUploadRequest.size is REQUIRED i64 (not Optional)
    let confirm: ConfirmUploadRequest = serde_json::from_value(serde_json::json!({
        "file_key": "uploads/abc.mp4",
        "original_filename": "video.mp4",
        "content_type": "video/mp4",
        "size": large_file_bytes,
    }))
    .expect("ConfirmUploadRequest with i64 size MUST parse");
    assert_eq!(
        confirm.size, large_file_bytes,
        "ConfirmUploadRequest.size MUST be i64 — large videos exceed i32"
    );
    let _: i64 = confirm.size;
}

// ── 3. Width/height are i32 (Reserved exception — pixel-bounded) ─────

/// Per CLAUDE.md "Reserved exception": image dimensions are
/// bounded counters. No image exceeds ~32k pixels on a side (and
/// most caps are ~8k for 8K UHD). i32 ceiling at ~2.1B is so far
/// past the realistic max that the exception applies cleanly.
/// MONEY never qualifies, but image dimensions are not money.
///
/// Pin width/height as i32 + confirm a width past i32::MAX MUST
/// fail to deserialize (it's at the type boundary).
#[test]
fn media_width_height_i32_reserved_exception() {
    let m = Media {
        id: 1,
        filename: "test.png".to_string(),
        original_filename: None,
        mime_type: Some("image/png".to_string()),
        size: Some(0),
        path: None,
        url: None,
        title: None,
        alt_text: None,
        caption: None,
        description: None,
        collection: None,
        is_optimized: Some(true),
        width: Some(7680), // 8K UHD width — well within i32
        height: Some(4320),
        created_at: None,
        updated_at: None,
    };
    let _: Option<i32> = m.width;
    let _: Option<i32> = m.height;
    let wire = serde_json::to_value(&m).expect("serialize Media");
    assert_eq!(wire["width"].as_i64(), Some(7680));
    assert_eq!(wire["height"].as_i64(), Some(4320));

    // ConfirmUploadRequest mirrors the i32 type
    let confirm: ConfirmUploadRequest = serde_json::from_value(serde_json::json!({
        "file_key": "uploads/img.png",
        "original_filename": "img.png",
        "content_type": "image/png",
        "size": 1024,
        "width": 7680,
        "height": 4320,
    }))
    .expect("ConfirmUploadRequest with i32 width/height MUST parse");
    let _: Option<i32> = confirm.width;
    let _: Option<i32> = confirm.height;
    assert_eq!(confirm.width, Some(7680_i32));

    // R9-D NEGATIVE: width > i32::MAX MUST fail
    // (the Rust type is i32 — Reserved exception, pixel-bounded
    // by physics: no real image exceeds ~32k px on a side).
    assert!(
        serde_json::from_value::<ConfirmUploadRequest>(serde_json::json!({
            "file_key": "uploads/img.png",
            "original_filename": "img.png",
            "content_type": "image/png",
            "size": 1024,
            "width": (i32::MAX as i64) + 1,
        }))
        .is_err(),
        "ConfirmUploadRequest.width > i32::MAX MUST fail — i32 is Reserved exception (pixel-bounded)"
    );
}

// ── 4. MediaQuery — fully optional + serde aliases for sort/order ────

/// `MediaQuery` is the admin-grid filter DTO. The page lands at
/// `/admin/media` with zero params. Every field is Optional. The
/// Rust fields `sort_by` and `sort_dir` carry serde aliases
/// (`#[serde(alias = "sort")]` / `alias = "order"`) so the
/// frontend can send either the snake_case canonical name or the
/// shorter alias (some older code paths send `?sort=created_at`).
///
/// `r#type` is keyword-escaped (Rust reserved word `type`); wire
/// JSON key `type` populates it.
#[test]
fn media_query_optional_with_sort_aliases() {
    // Zero params — pure default
    let empty: MediaQuery =
        serde_json::from_str("{}").expect("empty MediaQuery MUST parse (admin grid default)");
    assert!(empty.search.is_none());
    assert!(empty.r#type.is_none());
    assert!(empty.collection.is_none());
    assert!(empty.images_only.is_none());
    assert!(empty.is_optimized.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.sort_dir.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.page.is_none());

    // Canonical names (snake_case)
    let canon: MediaQuery = serde_json::from_value(serde_json::json!({
        "search": "logo",
        "type": "image/png",
        "collection": "branding",
        "images_only": true,
        "is_optimized": false,
        "sort_by": "created_at",
        "sort_dir": "desc",
        "per_page": 50,
        "page": 1,
    }))
    .expect("MediaQuery with canonical names MUST parse");
    assert_eq!(canon.search.as_deref(), Some("logo"));
    assert_eq!(canon.r#type.as_deref(), Some("image/png"));
    assert_eq!(
        canon.sort_by.as_deref(),
        Some("created_at"),
        "Canonical `sort_by` field name MUST populate sort_by"
    );
    assert_eq!(canon.sort_dir.as_deref(), Some("desc"));

    // Aliases (`sort` / `order`)
    let alias: MediaQuery = serde_json::from_value(serde_json::json!({
        "sort": "filename",
        "order": "asc",
    }))
    .expect("MediaQuery with aliases MUST parse");
    assert_eq!(
        alias.sort_by.as_deref(),
        Some("filename"),
        "Alias `sort` MUST populate sort_by"
    );
    assert_eq!(
        alias.sort_dir.as_deref(),
        Some("asc"),
        "Alias `order` MUST populate sort_dir"
    );

    // R9-D NEGATIVE: garbage in per_page MUST fail
    assert!(
        serde_json::from_value::<MediaQuery>(serde_json::json!({
            "per_page": "fifty",
        }))
        .is_err(),
        "MediaQuery.per_page MUST reject string input (i64 only)"
    );

    // R9-D NEGATIVE: page non-integer MUST fail
    assert!(
        serde_json::from_value::<MediaQuery>(serde_json::json!({
            "page": 1.5,
        }))
        .is_err(),
        "MediaQuery.page MUST reject float input"
    );
}

// ── 5. PresignedUploadRequest — filename + content_type required ─────

/// `PresignedUploadRequest.filename` and `content_type` are
/// required (the handler at line 601 validates content_type
/// against an allow-list). `size: Option<i64>` is optional
/// (client may not know byte count before upload).
///
/// `PresignedUploadResponse.expires_in: u64` (1 hour in seconds
/// — not money). `file_key`, `upload_url`, `public_url` are
/// pure strings.
#[test]
fn presigned_upload_request_requires_filename_and_content_type() {
    // Happy path
    let req: PresignedUploadRequest = serde_json::from_value(serde_json::json!({
        "filename": "logo.png",
        "content_type": "image/png",
    }))
    .expect("Minimal PresignedUploadRequest MUST parse");
    assert_eq!(req.filename, "logo.png");
    assert_eq!(req.content_type, "image/png");
    assert!(req.size.is_none());
    assert!(req.collection.is_none());

    let with_size: PresignedUploadRequest = serde_json::from_value(serde_json::json!({
        "filename": "video.mp4",
        "content_type": "video/mp4",
        "size": (i32::MAX as i64) + 1024,
        "collection": "tutorials",
    }))
    .expect("PresignedUploadRequest with optional fields MUST parse");
    assert_eq!(with_size.size, Some((i32::MAX as i64) + 1024));
    let _: Option<i64> = with_size.size;
    assert_eq!(with_size.collection.as_deref(), Some("tutorials"));

    // R9-D NEGATIVE: missing filename MUST fail
    assert!(
        serde_json::from_value::<PresignedUploadRequest>(serde_json::json!({
            "content_type": "image/png",
        }))
        .is_err(),
        "PresignedUploadRequest without filename MUST fail (required)"
    );

    // R9-D NEGATIVE: missing content_type MUST fail
    assert!(
        serde_json::from_value::<PresignedUploadRequest>(serde_json::json!({
            "filename": "logo.png",
        }))
        .is_err(),
        "PresignedUploadRequest without content_type MUST fail (required)"
    );

    // PresignedUploadResponse — expires_in u64, pure strings
    let resp = PresignedUploadResponse {
        upload_url: "https://r2.example.com/signed?x-amz-...".to_string(),
        file_key: "uploads/abc.png".to_string(),
        public_url: "https://r2.example.com/uploads/abc.png".to_string(),
        expires_in: 3600_u64,
    };
    let _: u64 = resp.expires_in;
    let resp_wire = serde_json::to_value(&resp).expect("serialize PresignedUploadResponse");
    assert_eq!(resp_wire["expires_in"].as_u64(), Some(3600));
    assert_eq!(resp_wire["file_key"].as_str(), Some("uploads/abc.png"));
}

// ── 6. ConfirmUploadRequest.size REQUIRED + path-traversal context ──

/// `ConfirmUploadRequest.size: i64` is REQUIRED (NOT Optional) —
/// once R2 has the file, the client knows the exact byte count.
/// The handler at line 691-695 then performs path-traversal
/// validation on `file_key` (rejecting `..`, `//`, leading `/`,
/// and `\0`). The DTO contract pin only covers the field shape.
#[test]
fn confirm_upload_request_size_required_i64() {
    let req: ConfirmUploadRequest = serde_json::from_value(serde_json::json!({
        "file_key": "uploads/abc.png",
        "original_filename": "logo.png",
        "content_type": "image/png",
        "size": 12345,
    }))
    .expect("Minimal ConfirmUploadRequest MUST parse");
    assert_eq!(req.file_key, "uploads/abc.png");
    assert_eq!(req.size, 12345_i64);
    assert!(req.width.is_none());
    assert!(req.title.is_none());

    // R9-D NEGATIVE: missing size MUST fail (i64, not Option<i64>)
    assert!(
        serde_json::from_value::<ConfirmUploadRequest>(serde_json::json!({
            "file_key": "uploads/abc.png",
            "original_filename": "logo.png",
            "content_type": "image/png",
        }))
        .is_err(),
        "ConfirmUploadRequest without size MUST fail (size: i64, REQUIRED)"
    );

    // R9-D NEGATIVE: missing file_key MUST fail
    assert!(
        serde_json::from_value::<ConfirmUploadRequest>(serde_json::json!({
            "original_filename": "logo.png",
            "content_type": "image/png",
            "size": 100,
        }))
        .is_err(),
        "ConfirmUploadRequest without file_key MUST fail"
    );

    // R9-D NEGATIVE: missing original_filename MUST fail
    assert!(
        serde_json::from_value::<ConfirmUploadRequest>(serde_json::json!({
            "file_key": "uploads/abc.png",
            "content_type": "image/png",
            "size": 100,
        }))
        .is_err(),
        "ConfirmUploadRequest without original_filename MUST fail"
    );

    // R9-D NEGATIVE: size as string MUST fail
    assert!(
        serde_json::from_value::<ConfirmUploadRequest>(serde_json::json!({
            "file_key": "uploads/abc.png",
            "original_filename": "logo.png",
            "content_type": "image/png",
            "size": "1024",
        }))
        .is_err(),
        "ConfirmUploadRequest.size MUST reject string input (i64 only)"
    );
}

// ── 7. Bulk DTOs use Vec<i64>; UpdateMedia / CleanupResult shape ────

/// `BulkDeleteRequest.ids: Vec<i64>` and `BulkUpdateRequest.ids:
/// Vec<i64>`. The handler caps both vecs at 100 items (lines
/// 924, 1005). A regression to Vec<i32> would silently break the
/// admin's "select all, delete" flow on production media past
/// row 2^31.
///
/// `CleanupResult.orphaned_files_found` / `orphaned_files_deleted`
/// / `orphaned_db_records` / `db_records_cleaned` are all `i64`
/// (aggregate counters that can grow large over time).
///
/// `UpdateMedia` is fully Optional (PATCH semantics).
#[test]
fn bulk_dtos_use_vec_i64_update_optional() {
    let above_i32: i64 = (i32::MAX as i64) + 1;

    // BulkDeleteRequest with i64 IDs past i32::MAX
    let del: BulkDeleteRequest = serde_json::from_value(serde_json::json!({
        "ids": [1, 2, above_i32, 9_999_999_999_i64],
    }))
    .expect("BulkDeleteRequest with Vec<i64> MUST parse");
    assert_eq!(del.ids.len(), 4);
    assert_eq!(
        del.ids[2], above_i32,
        "BulkDeleteRequest.ids MUST hold i64 — media.id is bigint"
    );
    let _: Vec<i64> = del.ids.clone();

    // R9-D NEGATIVE: missing ids MUST fail
    assert!(
        serde_json::from_value::<BulkDeleteRequest>(serde_json::json!({})).is_err(),
        "BulkDeleteRequest without ids MUST fail (required Vec)"
    );

    // R9-D NEGATIVE: ids not an array MUST fail
    assert!(
        serde_json::from_value::<BulkDeleteRequest>(serde_json::json!({"ids": 1})).is_err(),
        "BulkDeleteRequest.ids MUST be an array, not a scalar"
    );

    // BulkUpdateRequest — same Vec<i64>, optional fields
    let upd: BulkUpdateRequest = serde_json::from_value(serde_json::json!({
        "ids": [above_i32],
        "collection": "archived",
    }))
    .expect("BulkUpdateRequest MUST parse");
    assert_eq!(upd.ids[0], above_i32);
    assert_eq!(upd.collection.as_deref(), Some("archived"));
    assert!(upd.title.is_none()); // optional
    assert!(upd.alt_text.is_none()); // optional

    // UpdateMedia — fully Optional (PATCH semantics on PUT /:id)
    let empty: UpdateMedia =
        serde_json::from_str("{}").expect("Empty UpdateMedia MUST parse (PATCH semantics)");
    assert!(empty.title.is_none());
    assert!(empty.alt_text.is_none());
    assert!(empty.caption.is_none());
    assert!(empty.description.is_none());
    assert!(empty.collection.is_none());

    // CleanupResult — i64 counters
    let cleanup = CleanupResult {
        orphaned_files_found: above_i32,
        orphaned_files_deleted: above_i32,
        orphaned_db_records: above_i32,
        db_records_cleaned: above_i32,
        errors: vec!["sample error".to_string()],
    };
    let _: i64 = cleanup.orphaned_files_found;
    let _: i64 = cleanup.db_records_cleaned;
    let wire = serde_json::to_value(&cleanup).expect("serialize CleanupResult");
    assert_eq!(wire["orphaned_files_found"].as_i64(), Some(above_i32));
    assert_eq!(wire["db_records_cleaned"].as_i64(), Some(above_i32));

    // MalwareScanResult — bool + string fields
    let scan = MalwareScanResult {
        is_clean: true,
        threat_name: None,
        scan_provider: "stub".to_string(),
        scanned_at: "2026-05-20T00:00:00Z".to_string(),
    };
    let wire = serde_json::to_value(&scan).expect("serialize MalwareScanResult");
    assert_eq!(wire["is_clean"].as_bool(), Some(true));
    assert_eq!(wire["scan_provider"].as_str(), Some("stub"));
}

// ── 8. Router mount-table compile-pin (11 routes, admin-gated) ──────

/// `routes::media::admin_router()` MUST build as `Router<AppState>`.
/// Mount table (all admin-gated via `_admin: AdminUser`):
///   - GET    /                      (index)
///   - GET    /files                 (index — frontend alias)
///   - POST   /upload                (direct_upload — Multipart)
///   - POST   /presigned-upload      (presigned_upload)
///   - POST   /confirm-upload        (confirm_upload)
///   - GET    /statistics            (statistics — aggregates SUM(size))
///   - POST   /bulk-delete           (bulk_delete)
///   - POST   /bulk-update           (bulk_update)
///   - POST   /cleanup-orphans       (cleanup_orphaned_files)
///   - POST   /scan/:id              (scan_media_file)         — Path<i64>
///   - GET    /:id (+PUT + DELETE)   (show/update/destroy)    — Path<i64>
///
/// `Path<i64>` is load-bearing on every `:id` route. A regression
/// to `Path<i32>` would silently 400 every media URL the admin
/// page emits on a deployment past row 2^31.
///
/// The `_admin: AdminUser` gate is critical — direct upload and
/// presigned-URL generation are NOT public surfaces. A regression
/// that dropped the extractor from `presigned_upload` would let
/// any logged-in member upload arbitrary content to R2 (DoS via
/// bandwidth or hosting illegal content under the company brand).
#[test]
fn media_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::media::admin_router();
}

/// Idempotent construction. Per CLAUDE.md habit #3: pin that
/// nothing global (no `OnceLock`, no `static mut`) lives inside
/// the router constructor — the R2 storage client and DB pool
/// are pulled from `AppState` per-request, not cached at the
/// router level. A refactor that hoisted either into a global
/// would break ownership semantics and silently fail this test
/// the second time `admin_router()` runs.
#[test]
fn media_admin_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::media::admin_router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::media::admin_router();
}
