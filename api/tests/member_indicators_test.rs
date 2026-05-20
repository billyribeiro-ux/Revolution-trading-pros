//! Member Indicators route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::member_indicators` and
//! exercises the three router mount tables (`public_router()`,
//! `member_router()`, `download_router()`) plus the model contract
//! that flows through them: `models::indicator::{Indicator,
//! IndicatorListItem, IndicatorFile, UserIndicatorOwnership,
//! IndicatorQueryParams}`.
//!
//! ## Why this shape
//!
//! `routes/member_indicators.rs` is THE auth+money-adjacent surface:
//!   - paywall gate (`get_my_indicators`, `get_indicator_downloads`,
//!     `generate_download_url`, `get_license_key`) — every handler
//!     takes `user: User` and checks `user_indicators` ownership;
//!   - secure download tokens (`generate_download_url` builds a
//!     SHA256(user_id || file_id || expiry || secret) hash);
//!   - license-key generation + validation (16-char hex from SHA256).
//!
//! Every handler runs live SQL against `indicators`,
//! `user_indicators`, `indicator_files`, `indicator_videos`,
//! `indicator_downloads`, `trading_platforms`. What we CAN pin
//! without a DB:
//!
//! 1. **`Indicator.id` and `.price_cents` are `i64`** — the catalog
//!    surface's money + identity invariants. CLAUDE.md money rule:
//!    every `*_cents` field is `i64`. CLAUDE.md PK rule: every
//!    BIGSERIAL is `i64`. A narrowing of either is the worst class
//!    of regression on this stack.
//!
//! 2. **`IndicatorListItem.price_cents` is also `Option<i64>`** —
//!    the public catalog grid uses the lighter Summary type. Same
//!    money rule applies. `Option<>` because free indicators
//!    (price_cents = NULL) are part of the contract.
//!
//! 3. **`UserIndicatorOwnership.user_id` and `.indicator_id` are
//!    `i64`** — the ownership-check table FKs. The handler chain
//!    `Indicator.id (i64) -> UserIndicatorOwnership.indicator_id` is
//!    the paywall gate; a narrowing on either side would silently
//!    let the "do you own this?" join return false-empty results
//!    (which presents as 403 to the user — but the diagnostic is the
//!    wrong cause, harder to triage).
//!
//! 4. **`IndicatorFile.id` is `i32`** (CLAUDE.md "Reserved
//!    exception") — file IDs are bounded (an indicator has <100
//!    files across platform versions). But `IndicatorFile
//!    .indicator_id` is `i64` (BIGSERIAL FK to indicators.id). This
//!    asymmetry is load-bearing and surfaces in the URL path:
//!    `/download/indicator/:slug/:file_id` where `file_id: i32` per
//!    `Path<(String, i32)>`. A regression that flipped the path to
//!    `i64` would still parse, but a flip from i32→i64 on the
//!    IndicatorFile struct would force a wide DB migration.
//!
//! 5. **`IndicatorQueryParams` is fully optional** — public catalog
//!    GET must accept no params (default page 1, 20 per page,
//!    active-only).
//!
//! 6. **All three routers compile** — `public_router()` (catalog,
//!    no auth), `member_router()` (paywall-gated, `User` extractor),
//!    `download_router()` (token-gated, no User but token-verified).
//!    A regression that dropped the `User` extractor from any of the
//!    8 member routes would silently expose paywall content.
//!
//! 7. **Wire format is snake_case** for the indicator surface
//!    (`price_cents`, `is_active`, `download_url`, `meta_title`).
//!    R9-D NEGATIVE pin: camelCase MUST NOT appear.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_indicators_test.rs` (sister admin
//! surface), `tests/payments_test.rs`, `tests/posts_test.rs`.

use revolution_api::models::indicator::{
    Indicator, IndicatorFile, IndicatorListItem, IndicatorQueryParams, UserIndicatorOwnership,
};
use revolution_api::routes::member_indicators;

// Helper: BIGSERIAL-safe Indicator fixture with i64 price
fn fixture_indicator(id: i64, price_cents: Option<i64>) -> Indicator {
    let now = chrono::Utc::now().naive_utc();
    Indicator {
        id,
        name: "ICT Order Blocks".to_string(),
        slug: "ict-order-blocks".to_string(),
        description: Some("Detects institutional order blocks".to_string()),
        long_description: None,
        price_cents,
        is_active: Some(true),
        platform: Some("tradingview".to_string()),
        version: Some("2.1.0".to_string()),
        download_url: None,
        documentation_url: None,
        thumbnail: None,
        screenshots: None,
        features: None,
        requirements: None,
        meta_title: None,
        meta_description: None,
        created_at: Some(now),
        updated_at: Some(now),
    }
}

// ── 1. Indicator.id + .price_cents are i64 (money + BIGSERIAL pins) ──

/// HARD RULE (CLAUDE.md money): every `*_cents` field is `i64`,
/// never `i32`. The DB column is `BIGINT` (per migration 061's
/// drop of legacy NUMERIC price). A regression to `i32` would cap
/// individual indicator prices at $21,474,836.47 AND overflow any
/// SUM rollup across the catalog.
///
/// HARD RULE (CLAUDE.md BIGSERIAL): every PK on this stack is `i64`.
/// `Indicator.id` JOINs to `user_indicators.indicator_id` (BIGINT)
/// for the paywall check — a narrowing on the Rust side would
/// silently mistype the bind and return zero ownership rows.
#[test]
fn indicator_id_and_price_cents_are_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let ind = fixture_indicator(above_i32_max, Some(above_i32_max));

    let wire = serde_json::to_value(&ind).expect("serialize Indicator");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["price_cents"].as_i64(), Some(above_i32_max));

    // Compile-pins
    let _: i64 = ind.id;
    let _: Option<i64> = ind.price_cents;

    // Narrowing proofs
    assert!(
        (ind.id as i32 as i64) != ind.id,
        "narrowing Indicator.id to i32 must lose data — proves i64 is required"
    );
    let pc = ind.price_cents.unwrap();
    assert!(
        (pc as i32 as i64) != pc,
        "narrowing Indicator.price_cents to i32 must lose data — proves i64 is required (CLAUDE.md money rule)"
    );

    // Free indicator (price_cents = None) must serialize as null
    let free = fixture_indicator(2, None);
    let wire_free = serde_json::to_value(&free).expect("serialize free Indicator");
    assert!(
        wire_free["price_cents"].is_null(),
        "free indicator must serialize price_cents as null, not 0 — frontend uses null to render 'Free'"
    );
}

// ── 2. IndicatorListItem.price_cents is Option<i64> (catalog summary) ─

/// `IndicatorListItem` is the lighter type used by the public catalog
/// grid (`list_public_indicators` SELECTs only the summary columns).
/// `price_cents` is `Option<i64>` — same money rule, with Optional
/// because some indicators in the catalog are free (NULL price).
#[test]
fn indicator_list_item_price_cents_is_optional_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now = chrono::Utc::now().naive_utc();
    let item = IndicatorListItem {
        id: above_i32_max,
        name: "Free Demo Indicator".to_string(),
        slug: "free-demo".to_string(),
        description: None,
        price_cents: Some(above_i32_max),
        is_active: Some(true),
        platform: Some("thinkorswim".to_string()),
        thumbnail: None,
        created_at: Some(now),
    };

    // Compile-pins
    let _: i64 = item.id;
    let _: Option<i64> = item.price_cents;

    let wire = serde_json::to_value(&item).expect("serialize IndicatorListItem");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["price_cents"].as_i64(), Some(above_i32_max));

    // Free case
    let free_item = IndicatorListItem {
        price_cents: None,
        ..item
    };
    let wire_free = serde_json::to_value(&free_item).expect("serialize");
    assert!(
        wire_free["price_cents"].is_null(),
        "Option<i64> price_cents must serialize None as null, not absent or 0"
    );
}

// ── 3. UserIndicatorOwnership: user_id + indicator_id are i64 ────────

/// The paywall gate joins `user_indicators` on
/// `user_id = user.id AND indicator_id = indicator.id`. Both sides
/// are `BIGINT` in Postgres (BIGSERIAL FKs). A narrowing on either
/// would silently break the join — sqlx type-mismatches on bind and
/// returns zero rows. Symptom: "you do not own this indicator" 403
/// for legitimately-purchased indicators above i32::MAX user IDs.
#[test]
fn user_indicator_ownership_user_id_and_indicator_id_are_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let ownership = UserIndicatorOwnership {
        id: 1,
        user_id: above_i32_max,
        indicator_id: above_i32_max + 100,
        order_id: None,
        order_item_id: None,
        price_paid_cents: Some(4999),
        currency: Some("usd".to_string()),
        access_granted_at: None,
        access_expires_at: None,
        is_lifetime_access: Some(true),
        source: Some("purchase".to_string()),
        granted_by: None,
        notes: None,
        is_active: Some(true),
        revoked_at: None,
        revoked_reason: None,
        created_at: None,
        updated_at: None,
    };

    // Compile-pins
    let _: i64 = ownership.user_id;
    let _: i64 = ownership.indicator_id;

    let wire = serde_json::to_value(&ownership).expect("serialize UserIndicatorOwnership");
    assert_eq!(wire["user_id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["indicator_id"].as_i64(), Some(above_i32_max + 100));

    // Narrowing proofs
    assert!(
        (ownership.user_id as i32 as i64) != ownership.user_id,
        "narrowing UserIndicatorOwnership.user_id to i32 must lose data"
    );
    assert!(
        (ownership.indicator_id as i32 as i64) != ownership.indicator_id,
        "narrowing UserIndicatorOwnership.indicator_id to i32 must lose data"
    );
}

// ── 4. IndicatorFile: id is i32 (reserved), indicator_id is i64 ──────

/// CLAUDE.md "Reserved exception" applies to `IndicatorFile.id`:
/// file IDs are bounded (<100 platform versions per indicator, ~10⁴
/// indicators = ~10⁶ files << 2.1B). `i32` is fine.
///
/// `IndicatorFile.indicator_id` is NOT a reserved exception — it's a
/// BIGSERIAL FK to `indicators.id` and must be `i64`. The handler
/// path `/download/indicator/:slug/:file_id` reads `file_id` as
/// `Path<(String, i32)>` — also `i32`, matching the file ID type.
///
/// This asymmetry (i32 file ID, i64 indicator ID) is load-bearing.
/// Pin both sides.
#[test]
fn indicator_file_id_is_i32_indicator_id_is_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let file = IndicatorFile {
        id: 12345,
        indicator_id: above_i32_max,
        file_name: "ict-orderblocks.pine".to_string(),
        original_name: Some("ICT Order Blocks v2.1.pine".to_string()),
        file_path: "/storage/indicators/ict-ob.pine".to_string(),
        file_size_bytes: Some(45_678),
        file_type: Some("pine".to_string()),
        mime_type: Some("text/plain".to_string()),
        checksum_sha256: None,
        platform: "tradingview".to_string(),
        platform_version: Some("v5".to_string()),
        storage_provider: Some("r2".to_string()),
        storage_bucket: None,
        storage_key: None,
        cdn_url: Some("https://cdn.example/ict-ob.pine".to_string()),
        download_token: None,
        token_expires_at: None,
        version: Some("2.1.0".to_string()),
        is_current_version: Some(true),
        changelog: None,
        display_name: None,
        display_order: Some(1),
        icon_url: None,
        is_active: Some(true),
        download_count: Some(42),
        uploaded_at: None,
        updated_at: None,
    };

    // Compile-pin: id is genuinely i32 (CLAUDE.md reserved exception
    // — files are bounded). The FK `indicator_id` is i64 (BIGSERIAL).
    let _: i32 = file.id;
    let _: i64 = file.indicator_id;

    // file_size_bytes is Option<i64> — files can be large (>2GB
    // theoretically). NOT a reserved exception; size is unbounded
    // by row-count semantics.
    let _: Option<i64> = file.file_size_bytes;

    let wire = serde_json::to_value(&file).expect("serialize IndicatorFile");
    assert_eq!(wire["id"].as_i64(), Some(12345));
    assert_eq!(wire["indicator_id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["file_size_bytes"].as_i64(), Some(45_678));

    // Narrowing proof: indicator_id MUST be i64
    assert!(
        (file.indicator_id as i32 as i64) != file.indicator_id,
        "narrowing IndicatorFile.indicator_id to i32 must lose data — it's a BIGSERIAL FK"
    );
}

// ── 5. IndicatorQueryParams: fully optional (catalog grid loads) ─────

/// The public `/indicators` catalog grid hits GET with NO query
/// params (defaults page 1, per_page 20, is_active=true). Every
/// field on `IndicatorQueryParams` MUST be `Option<>`. A regression
/// that flipped any to required would 400-error the public catalog
/// on the marketing site (zero click-through to paid indicators).
#[test]
fn indicator_query_params_accepts_empty_and_filtered_payloads() {
    // Empty (default marketing-grid render)
    let empty: IndicatorQueryParams =
        serde_json::from_str("{}").expect("empty IndicatorQueryParams must parse");
    assert!(empty.status.is_none());
    assert!(empty.platform.is_none());
    assert!(empty.is_free.is_none());
    assert!(empty.search.is_none());
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.sort_order.is_none());

    // Full payload — every documented filter
    let full: IndicatorQueryParams = serde_json::from_value(serde_json::json!({
        "status": "published",
        "platform": "thinkorswim",
        "is_free": false,
        "search": "order blocks",
        "page": 2,
        "per_page": 25,
        "sort_by": "created_at",
        "sort_order": "desc",
    }))
    .expect("full IndicatorQueryParams must parse");
    assert_eq!(full.platform.as_deref(), Some("thinkorswim"));
    assert_eq!(full.is_free, Some(false));
    assert_eq!(full.page, Some(2));
    assert_eq!(full.per_page, Some(25));
}

// ── 6. Wire format: snake_case ONLY (R9-D NEGATIVE pin) ──────────────

/// The frontend's `indicators` store reads `price_cents`, `is_active`,
/// `download_url`, `documentation_url`, `meta_title`, `meta_description`,
/// `created_at`, `updated_at`, `long_description`. A regression that
/// added `#[serde(rename_all = "camelCase")]` to the Indicator
/// model would silently break every consumer (the frontend would see
/// undefined values everywhere and render empty cards with blank
/// prices — which is worse than a 500, because the page LOADS).
#[test]
fn indicator_wire_is_snake_case_no_camel_case() {
    let ind = fixture_indicator(1, Some(4999));
    let wire = serde_json::to_value(&ind).expect("serialize Indicator");

    // POSITIVE: snake_case keys present
    assert!(wire.get("price_cents").is_some(), "price_cents on wire");
    assert!(wire.get("is_active").is_some(), "is_active on wire");
    assert!(
        wire.get("long_description").is_some(),
        "long_description on wire"
    );
    assert!(wire.get("download_url").is_some(), "download_url on wire");
    assert!(
        wire.get("documentation_url").is_some(),
        "documentation_url on wire"
    );
    assert!(wire.get("meta_title").is_some(), "meta_title on wire");
    assert!(
        wire.get("meta_description").is_some(),
        "meta_description on wire"
    );
    assert!(wire.get("created_at").is_some(), "created_at on wire");
    assert!(wire.get("updated_at").is_some(), "updated_at on wire");

    // NEGATIVE: camelCase MUST NOT appear (R9-D regression guard)
    assert!(
        wire.get("priceCents").is_none(),
        "priceCents (camelCase) must NOT appear — wire format is snake_case"
    );
    assert!(
        wire.get("isActive").is_none(),
        "isActive (camelCase) must NOT appear"
    );
    assert!(
        wire.get("longDescription").is_none(),
        "longDescription (camelCase) must NOT appear"
    );
    assert!(
        wire.get("downloadUrl").is_none(),
        "downloadUrl (camelCase) must NOT appear"
    );
    assert!(
        wire.get("metaTitle").is_none(),
        "metaTitle (camelCase) must NOT appear"
    );
    assert!(
        wire.get("createdAt").is_none(),
        "createdAt (camelCase) must NOT appear"
    );
}

// ── 7. Three routers compile (public + member + download) ────────────

/// `member_indicators::public_router()` is the unauth catalog — no
/// `User` extractor. A refactor that ADDED a User extractor to a
/// public route would silently 401 the marketing-site catalog grid
/// (which calls without an Authorization header).
#[test]
fn public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = member_indicators::public_router();
}

/// `member_indicators::member_router()` is the paywall-gated surface.
/// Every handler takes `user: User` extractor:
///   - `get_my_indicators` (`/`)
///   - `get_download_history` (`/history`)
///   - `check_updates` (`/updates`)
///   - `get_indicator_downloads` (`/:slug`)
///   - `generate_download_url` (`/:slug/download/:file_id`)
///   - `get_license_key` (`/:slug/license`)
///   - `validate_license_key` (`/:slug/validate`) — note: this one
///     takes State only, no User (license validation is by-key, not
///     by-session — that's a deliberate choice for sharing licenses
///     across devices).
///   - `get_installation_guide` (`/:slug/guide/:platform`) — public.
///
/// A refactor that dropped the User extractor from any of the
/// 6-of-8 user-gated handlers would silently expose paywall content.
/// The Router-build catches the extractor-type mismatch at compile.
#[test]
fn member_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = member_indicators::member_router();
}

/// `member_indicators::download_router()` is the token-gated CDN
/// redirect surface. The single handler (`download_file`) verifies
/// the SHA256(user_id || file_id || expiry || secret) token instead
/// of using a session — that's deliberate, because download links
/// are emailed/copied and must work without a cookie.
///
/// A refactor that ADDED a `User` extractor here would break every
/// email-shared download link (the link works in a fresh browser
/// without auth).
#[test]
fn download_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = member_indicators::download_router();
}

/// Idempotent construction across all three routers (CLAUDE.md
/// habit #3 — "cached state lost during refactor" landmine).
#[test]
fn all_three_routers_safe_to_construct_multiple_times() {
    for _ in 0..3 {
        let _p: axum::Router<revolution_api::AppState> = member_indicators::public_router();
        let _m: axum::Router<revolution_api::AppState> = member_indicators::member_router();
        let _d: axum::Router<revolution_api::AppState> = member_indicators::download_router();
    }
}
