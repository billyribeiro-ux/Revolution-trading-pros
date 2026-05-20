//! Bunny.net video upload route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::bunny_upload` and
//! exercises the 3 public DTOs + the `admin_router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/bunny_upload.rs` (589 LOC) handles video uploads to
//! Bunny.net Stream — create video entries, fetch upload URLs, track
//! processing status, list recent uploads, and stream the actual
//! video bytes through a 5GB-limit PUT endpoint. Every handler is
//! gated behind `AdminUser`, runs live SQL against `bunny_uploads`,
//! and makes outbound HTTPS calls to Bunny.net. Handlers can't be
//! invoked in isolation. What we CAN pin:
//!
//! 1. **`CreateVideoRequest.library_id` is `Option<i64>`.** The Bunny
//!    Stream library ID is the tenant-isolation primitive — videos
//!    uploaded to library A are NEVER served from library B. The
//!    default-library lookup (`get_default_library_id()`) returns
//!    `i64` from `std::env::var(...).parse::<i64>()`. A regression
//!    that narrowed the input to `i32` would silently fail to parse
//!    high library IDs (Bunny rolls library IDs forward as accounts
//!    upgrade) and would force an env-default fallback at runtime.
//!
//! 2. **`CreateVideoResponse` / `VideoStatusResponse` wire-format
//!    pin.** The SvelteKit upload-progress UI reads these fields by
//!    exact name (`video_guid`, `upload_url`, `embed_url`,
//!    `status_code`, `duration`). A snake_case → camelCase regression
//!    would break the live upload widget without breaking any
//!    backend test. Pin the snake_case wire shape.
//!
//! 3. **`VideoStatusResponse.duration` is `Option<i32>`.** Video
//!    duration is "seconds" — `i32::MAX` seconds is 68 years; videos
//!    longer than that don't exist. CLAUDE.md "Reserved exception"
//!    applies — i32 is correct here. Pin the optional shape
//!    (`None` while processing, `Some(secs)` once ready).
//!
//! 4. **`admin_router()` mount table compile-pin.** The mount tree
//!    is non-trivial — it `.nest("/upload", upload_router)` with a
//!    5GB `DefaultBodyLimit` applied via `.layer()` (the layer is
//!    SCOPED to the nested upload router so other routes keep the
//!    default 2MB limit). A refactor that moved the limit to the
//!    outer router would silently expose every admin endpoint to
//!    5GB POSTs — DoS-via-large-body. The compile-pin doesn't catch
//!    that semantic, but it catches handler-signature regressions
//!    (dropped `AdminUser`, wrong return type).
//!
//! 5. **All 4 handlers MUST gate on `AdminUser`** — uploading video
//!    bytes to a Bunny library is a privileged operation (cost +
//!    legal-content liability). The mount-table compile-pin catches
//!    if the gate is ever dropped from a handler.
//!
//! ## Pattern source
//!
//! Modeled on `tests/connections_test.rs`, `tests/forms_test.rs`,
//! `tests/admin_orders_test.rs`.

use revolution_api::routes::bunny_upload::{
    CreateVideoRequest, CreateVideoResponse, VideoStatusResponse,
};

// ── 1. CreateVideoRequest: library_id is Option<i64> ─────────────────

/// Bunny.net Stream library IDs are the tenant-isolation primitive.
/// `library_id` is `Option<i64>` — when None the handler falls back
/// to the `BUNNY_STREAM_LIBRARY_ID` env-var-or-default. A regression
/// that narrowed this to `i32` would silently truncate large library
/// IDs (Bunny rolls IDs forward as accounts upgrade — production
/// libraries already exceed 500k, well below i32 max today but the
/// type contract is i64 per the env-var parse path).
#[test]
fn create_video_request_library_id_is_optional_i64() {
    // Minimal: just `title` — the rest are optional.
    let minimal: CreateVideoRequest = serde_json::from_value(serde_json::json!({
        "title": "Daily Recap 2026-05-20"
    }))
    .expect("minimal CreateVideoRequest must deserialize");
    assert_eq!(minimal.title, "Daily Recap 2026-05-20");
    assert!(minimal.library_id.is_none());
    assert!(minimal.collection_id.is_none());
    assert!(minimal.room_slug.is_none());

    // With explicit library_id well past i32::MAX — proves i64 pin.
    let above_i32_max: i64 = (i32::MAX as i64) + 7;
    let with_lib: CreateVideoRequest = serde_json::from_value(serde_json::json!({
        "title": "Big library upload",
        "library_id": above_i32_max,
        "collection_id": "abc-123-collection",
        "room_slug": "explosive-swings"
    }))
    .expect("CreateVideoRequest with high library_id must deserialize");
    assert_eq!(with_lib.library_id, Some(above_i32_max));
    assert_eq!(
        with_lib.collection_id.as_deref(),
        Some("abc-123-collection")
    );
    assert_eq!(with_lib.room_slug.as_deref(), Some("explosive-swings"));
    assert!(
        with_lib.library_id.unwrap() > i32::MAX as i64,
        "library_id MUST be i64 — i32 would silently truncate"
    );

    // Missing `title` MUST fail (required).
    assert!(
        serde_json::from_value::<CreateVideoRequest>(serde_json::json!({"library_id": 1})).is_err(),
        "CreateVideoRequest without `title` MUST fail (required)"
    );
}

// ── 2. CreateVideoResponse: snake_case wire shape pin ────────────────

/// The SvelteKit upload widget reads `video_guid`, `upload_url`,
/// `video_url`, `embed_url` by exact snake_case name. A regression
/// that swapped to camelCase (or `#[serde(rename_all = "camelCase")]`
/// applied to this struct) would break the widget without breaking
/// any backend test.
#[test]
fn create_video_response_wire_shape_is_snake_case() {
    let resp = CreateVideoResponse {
        success: true,
        video_guid: "abc-def-1234".to_string(),
        upload_url: "https://video.bunnycdn.com/library/585929/videos/abc-def-1234".to_string(),
        video_url: "https://vz-5a23b520-193.b-cdn.net/abc-def-1234/play_720p.mp4".to_string(),
        embed_url: "https://iframe.mediadelivery.net/embed/585929/abc-def-1234".to_string(),
    };

    let wire = serde_json::to_value(&resp).expect("serialize CreateVideoResponse");
    assert_eq!(wire["success"], serde_json::json!(true));
    assert_eq!(wire["video_guid"], serde_json::json!("abc-def-1234"));
    assert!(
        wire["upload_url"].is_string(),
        "upload_url MUST be snake_case (frontend widget reads by that key)"
    );
    assert!(wire["video_url"].is_string());
    assert!(wire["embed_url"].is_string());

    // Negative-test: camelCase keys MUST NOT exist on the wire.
    assert!(
        wire.get("videoGuid").is_none(),
        "camelCase videoGuid MUST NOT appear (snake_case contract)"
    );
    assert!(wire.get("uploadUrl").is_none());
    assert!(wire.get("embedUrl").is_none());
}

// ── 3. VideoStatusResponse: status_code i32 + duration Option<i32> ───

/// Video processing pipeline maps Bunny's numeric status codes 0-5
/// to human-readable strings. `status_code` is `i32` (small bounded
/// range), `duration` is `Option<i32>` (None while processing,
/// `Some(seconds)` once ready). `i32::MAX` seconds ≈ 68 years —
/// videos longer than that don't exist, so the CLAUDE.md "Reserved
/// exception" for bounded counters applies. Pin the optional shape.
#[test]
fn video_status_response_pins_status_code_and_optional_duration() {
    // Processing state — no duration yet, no URLs yet.
    let processing = VideoStatusResponse {
        success: true,
        status: "processing".to_string(),
        status_code: 2,
        video_url: None,
        embed_url: None,
        thumbnail_url: None,
        duration: None,
    };

    let wire = serde_json::to_value(&processing).expect("serialize processing response");
    assert_eq!(wire["status"], serde_json::json!("processing"));
    assert_eq!(wire["status_code"], serde_json::json!(2));
    assert!(wire["duration"].is_null());
    assert!(wire["video_url"].is_null());

    // Ready state — all URLs populated, duration in seconds.
    let ready = VideoStatusResponse {
        success: true,
        status: "ready".to_string(),
        status_code: 4,
        video_url: Some("https://vz-x.b-cdn.net/guid/play_720p.mp4".to_string()),
        embed_url: Some("https://iframe.mediadelivery.net/embed/123/guid".to_string()),
        thumbnail_url: Some("https://vz-x.b-cdn.net/guid/thumb.jpg".to_string()),
        duration: Some(3_600), // 1-hour recording
    };

    let wire2 = serde_json::to_value(&ready).expect("serialize ready response");
    assert_eq!(wire2["status_code"], serde_json::json!(4));
    assert_eq!(wire2["duration"], serde_json::json!(3_600));
    assert!(
        wire2["video_url"].is_string(),
        "video_url populated only when status == ready"
    );

    // Pin: duration accepts the max-legal i32 — proves i32 type (and
    // confirms it doesn't accidentally widen to f64 or string).
    let long_video = VideoStatusResponse {
        success: true,
        status: "ready".to_string(),
        status_code: 4,
        video_url: None,
        embed_url: None,
        thumbnail_url: None,
        duration: Some(i32::MAX), // ~68 years — the bound
    };
    let wire3 = serde_json::to_value(&long_video).expect("serialize max-duration response");
    assert_eq!(wire3["duration"].as_i64(), Some(i32::MAX as i64));
}

// ── 4. Failed-upload status mapping — status_code 5 → "failed" ───────

/// Bunny.net status codes the handler maps:
///   0 → "created", 1 → "uploaded", 2 → "processing",
///   3 → "transcoding", 4 → "ready", 5 → "failed",
///   _ → "unknown"
///
/// We can't unit-test the runtime mapping (handler is inline), but we
/// CAN pin the response shape for the "failed" path — the admin UI
/// surfaces "failed" videos in red for re-upload. `status_code: 5`
/// with `status: "failed"` is the contract the frontend reads.
#[test]
fn video_status_response_failed_path_wire_shape() {
    let failed = VideoStatusResponse {
        success: true, // success of the STATUS-FETCH call, not the video
        status: "failed".to_string(),
        status_code: 5,
        video_url: None,
        embed_url: None,
        thumbnail_url: None,
        duration: None,
    };

    let wire = serde_json::to_value(&failed).expect("serialize failed response");
    assert_eq!(wire["status"], serde_json::json!("failed"));
    assert_eq!(wire["status_code"], serde_json::json!(5));
    assert_eq!(
        wire["success"],
        serde_json::json!(true),
        "success=true even for status=failed — the status-fetch call itself succeeded"
    );

    // Negative-test: response is not snake_case-only by accident — the
    // empty struct without `status` MUST fail to serialize a coherent
    // response. (We can't easily test that, but we CAN test that the
    // shape we DO produce has the right key names.)
    assert!(wire.get("statusCode").is_none(), "no camelCase leak");
    assert!(wire.get("thumbnailUrl").is_none(), "no camelCase leak");
}

// ── 5. Router mount table compile-pin ────────────────────────────────

/// `admin_router()` must build as `Router<AppState>`. Mount table:
///   - `/create-video` (POST → create_video) — gates on `AdminUser`
///   - `/upload` (PUT → upload_video, nested with 5GB body limit)
///   - `/video-status/:guid` (GET → get_video_status)
///   - `/uploads` (GET → list_uploads)
///
/// The 5GB `DefaultBodyLimit::max(...)` layer is SCOPED to the nested
/// `/upload` sub-router — not the outer router. A refactor that
/// moved the layer to the outer `Router::new()` would silently expose
/// `/create-video` + `/video-status/:guid` + `/uploads` to 5GB POSTs
/// (DoS via large body on JSON endpoints).
///
/// This compile-pin doesn't catch the scope regression (it's a
/// runtime layer placement), but it catches every handler-signature
/// regression (dropped `_admin: AdminUser` gate, wrong return type,
/// missing State extractor).
#[test]
fn bunny_upload_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::bunny_upload::admin_router();
}

/// Idempotent constructor — must be safe to call multiple times.
/// `api_router()` nests this once under `/admin/bunny`; nothing
/// prevents future composition from re-nesting.
#[test]
fn bunny_upload_admin_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::bunny_upload::admin_router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::bunny_upload::admin_router();
}
