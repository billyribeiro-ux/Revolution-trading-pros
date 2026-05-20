//! Search route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::search` and pins:
//!   - the public `router()` mount-table shape (3 GET endpoints)
//!   - the `SearchQuery` request DTO (query + limit with default)
//!   - the i64 `price_cents` invariant on `SearchableCourse`
//!     (R17-D, 2026-05-20: widened i32 → i64 in
//!     `services/search.rs` — pin documents the fix)
//!
//! ## Why this file exists (R21-D)
//!
//! `routes/search.rs` is 103 LOC of thin proxy handlers over
//! `state.services.search.search_*` (Meilisearch client). The route
//! file itself doesn't touch SQL, but its handlers consume the
//! `SearchableCourse` / `SearchablePost` DTOs from `services/search.rs`
//! that round-trip the index payload. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 3 GET endpoints, public surface.**
//!    Mount table at routes/search.rs:33-38:
//!      - GET /          → `search_all`     (cross-index, no limit)
//!      - GET /courses   → `search_courses` (limit via query param)
//!      - GET /posts     → `search_posts`   (limit via query param)
//!    Mounted at `/search` per routes/mod.rs:92. All three handlers
//!    are PUBLIC (no `_admin: AdminUser`, no `_user: User`
//!    extractor — search is a member-facing feature accessible
//!    without auth on the catalog browse pages).
//!
//! 2. **`SearchQuery` is `pub` and accepts `q: String` + optional
//!    `limit: usize`** (lines 12-21). The `#[serde(default =
//!    "default_limit")]` makes `limit` default to 10. A regression
//!    that flipped `limit: usize` to `limit: i32` would compile but
//!    silently change the wire contract — frontend sends
//!    `?limit=20`, backend deserializes as i32, and any frontend
//!    that ever sends a u32 value > i32::MAX (legitimate `usize`)
//!    would fail at JSON parse time on the backend.
//!
//! 3. **`SearchResponse` is `pub` and exposes the 3-arm shape.**
//!    Per lines 25-30, `SearchResponse { courses, posts, query }`
//!    is `#[allow(dead_code)]` — currently the handlers return
//!    `Json<serde_json::Value>` directly (lines 47-54), NOT
//!    `Json<SearchResponse>`. The DTO is the typed contract the
//!    frontend is supposed to consume. A regression that deleted
//!    `SearchResponse` would silently sever the typed contract;
//!    the pin keeps it alive.
//!
//! 4. **`SearchableCourse.price_cents: i64`** (per
//!    `services/search.rs:22`, widened from i32 in R17-D,
//!    2026-05-20). Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT
//!    ONLY, EVERY TIME": the search index payload MUST carry the
//!    same i64 as the DB column `courses.price_cents` (BIGINT).
//!    A regression to i32 would silently truncate any course
//!    priced above $21,474,836.47 (per CLAUDE.md: "The cap is
//!    reached on totals, not individual prices" — but corporate
//!    cohort pricing CAN hit that ceiling on a per-course basis).
//!
//! 5. **`default_limit()` returns 10.** This is the load-bearing
//!    default — if a caller omits `?limit=`, the handler must NOT
//!    return the unbounded result set (which on a heavy traffic day
//!    could pull 100K records from Meilisearch and bloat the JSON
//!    payload). The pin freezes the default; a regression that
//!    flipped to e.g. 1000 would silently 10x the per-request
//!    payload size.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_seo_test.rs` (sister proxy surface),
//! `tests/room_search_test.rs` (sister search surface),
//! `tests/courses_admin_test.rs` (i64 price_cents pin).

use revolution_api::routes::search;

// ── 1. Router mount-table compile-pin — 3 public GET endpoints ───────

/// `routes::search::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/search.rs:33-38:
///   - GET /          → `search_all`     (cross-index)
///   - GET /courses   → `search_courses` (filtered)
///   - GET /posts     → `search_posts`   (filtered)
///
/// All 3 handlers are PUBLIC. Mounted at `/search` per
/// routes/mod.rs:92. A regression that added auth (`_user: User`,
/// `_admin: AdminUser`) would 401 unauthenticated browsers on the
/// catalog search bar.
///
/// R9-D NEGATIVE: a regression that dropped any of the 3 routes
/// would fail this compile — the closures in `router()` reference
/// the handler functions by name, and a deleted handler would
/// produce a name-resolution error.
#[test]
fn search_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = search::router();
}

// ── 2. SearchQuery DTO: q + default-10 limit ─────────────────────────

/// `SearchQuery { q: String, limit: usize (default 10) }` MUST stay
/// the public contract for the query-string deserialization.
///
/// Per routes/search.rs:14-21:
///   - `q: String` — the search term (required; no `Option<String>`)
///   - `limit: usize` — `#[serde(default = "default_limit")]` → 10
///
/// R9-D NEGATIVE: a regression that flipped `limit: usize` to
/// `limit: Option<i32>` would compile but silently change the wire
/// shape — `?limit=20` arrives as `Some(20)` but a missing param
/// arrives as `None` instead of defaulting to 10, and any handler
/// reading `params.limit` would have to `.unwrap_or(10)` — easily
/// missed in code review.
///
/// The pin builds a `SearchQuery` via JSON (mimicking axum's
/// `Query<T>` URL-decode → serde flow) with both default and
/// explicit limit, asserting both shapes round-trip.
#[test]
fn search_query_deserializes_with_default_limit_10() {
    use revolution_api::routes::search::SearchQuery;

    // Default limit when caller omits ?limit=
    let q: SearchQuery = serde_json::from_value(serde_json::json!({
        "q": "options"
    }))
    .expect("SearchQuery deserializes with just q");
    assert_eq!(q.q, "options");
    assert_eq!(q.limit, 10, "default limit MUST be 10 per default_limit()");

    // Explicit limit when caller sends ?limit=N
    let q: SearchQuery = serde_json::from_value(serde_json::json!({
        "q": "futures",
        "limit": 25
    }))
    .expect("SearchQuery deserializes with q + limit");
    assert_eq!(q.q, "futures");
    assert_eq!(q.limit, 25);

    // R9-D NEGATIVE: missing q MUST fail (q is required, not Option).
    let err = serde_json::from_value::<SearchQuery>(serde_json::json!({
        "limit": 5
    }));
    assert!(err.is_err(), "missing q MUST fail — q is required");
}

// ── 3. SearchResponse public shape — `pub courses + posts + query` ───

/// `SearchResponse { courses, posts, query }` MUST stay `pub`. Per
/// routes/search.rs:25-30, the struct is `#[allow(dead_code)]`
/// because the current handlers (lines 41-103) return
/// `Json<serde_json::Value>` ad-hoc; `SearchResponse` is the typed
/// contract reserved for the next refactor that tightens the
/// handler return types.
///
/// Per CLAUDE.md memory (`feedback_create_not_delete.md`:
/// "orphan = build the missing side. Never `git rm`"), the DTO
/// MUST NOT be deleted to "clean up dead code" — it's the future
/// typed surface. The pin keeps it alive via a compile reference.
///
/// The pin constructs a `SearchResponse` and asserts its JSON
/// shape matches the documented 3-arm contract.
#[test]
fn search_response_public_shape_has_courses_posts_query() {
    use revolution_api::routes::search::SearchResponse;

    let resp = SearchResponse {
        courses: vec![],
        posts: vec![],
        query: "options".to_string(),
    };

    let wire = serde_json::to_value(&resp).expect("serialize SearchResponse");
    assert!(wire["courses"].is_array(), "courses MUST be array");
    assert!(wire["posts"].is_array(), "posts MUST be array");
    assert_eq!(wire["query"].as_str(), Some("options"));
}

// ── 4. SearchableCourse.price_cents is i64 — R17-D money pin ─────────

/// `SearchableCourse.price_cents: i64` MUST round-trip past
/// `i32::MAX`. Per `services/search.rs:13-25`, the field was
/// widened from i32 → i64 in audit R17-D (2026-05-20) — the source
/// DB column `courses.price_cents` is BIGINT, and carrying i32 in
/// the search-index payload was a silent narrowing on the read side.
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
/// every layer (Rust struct → JSON wire → DB column → Stripe API)
/// MUST be i64. The search index is one of those layers — even
/// though Meilisearch internally uses JSON numbers (which JS handles
/// up to 2^53), the Rust struct that serializes/deserializes the
/// payload MUST be i64 so any silent overflow is caught at the
/// type boundary.
///
/// R9-D NEGATIVE: a regression to i32 would let a price above
/// $21,474,836.47 silently truncate or fail to deserialize from the
/// index. The pin proves the field accepts and round-trips an
/// `(i32::MAX as i64) + 1` value.
#[test]
fn searchable_course_price_cents_is_i64_round_trips_past_i32_max() {
    use revolution_api::services::search::SearchableCourse;

    let above_i32_max: i64 = (i32::MAX as i64) + 12_345;

    let course = SearchableCourse {
        id: "course-abc-123".to_string(),
        title: "Corporate Options Mastery (50-seat cohort)".to_string(),
        description: "Multi-seat enterprise course".to_string(),
        slug: "corporate-options-mastery".to_string(),
        instructor_name: "Billy".to_string(),
        // R17-D widening pin: must accept value > i32::MAX
        price_cents: above_i32_max,
        is_published: true,
    };

    let wire = serde_json::to_value(&course).expect("serialize SearchableCourse");
    assert_eq!(wire["price_cents"].as_i64(), Some(above_i32_max));
    assert!(
        wire["price_cents"].as_i64().unwrap() > i32::MAX as i64,
        "price_cents MUST round-trip past i32::MAX (CLAUDE.md money rule)"
    );

    // Round-trip back via the index payload — proves both ser AND de
    // accept i64.
    let round: SearchableCourse =
        serde_json::from_value(wire).expect("deserialize SearchableCourse");
    assert_eq!(round.price_cents, above_i32_max);
}

// ── 5. R9-D NEGATIVE: SearchableCourse.id is String, NOT i64 PK ──────

/// `SearchableCourse.id: String` is the Meilisearch document key.
/// Although `courses.id` is BIGSERIAL i64 in Postgres (per
/// schema.sql), the search index payload uses the string form of
/// the ID — Meilisearch requires document IDs to be alphanumeric.
///
/// Per CLAUDE.md "Reserved exception" — this is NOT money, NOT a
/// row-counter, NOT a PK extractor on the Rust side. The String
/// type IS the correct shape for a Meilisearch document key
/// (which the indexer in `services/search.rs` builds by stringifying
/// the BIGSERIAL i64 PK from `courses.id`).
///
/// R9-D NEGATIVE: a regression that flipped `id: String` to
/// `id: i64` would break the Meilisearch indexer (which expects
/// the document key to be the JSON-string form of the PK).
/// Conversely, a regression that flipped the SQL-side PK to i32
/// would NOT show up here — but the PK pin lives in admin_courses
/// and courses_admin tests already.
///
/// The pin proves both shapes: `id` is String (NOT i64), and the
/// String value round-trips intact.
#[test]
fn searchable_course_id_is_string_meilisearch_doc_key() {
    use revolution_api::services::search::SearchableCourse;

    let above_i32_max_as_string = ((i32::MAX as i64) + 999_999_999).to_string();

    let course = SearchableCourse {
        // The id is the Meilisearch document key — stringified BIGINT PK.
        // A 12-digit course ID (well above i32::MAX) MUST round-trip as
        // its String form WITHOUT silent conversion to a JSON number.
        id: above_i32_max_as_string.clone(),
        title: "x".to_string(),
        description: "y".to_string(),
        slug: "x".to_string(),
        instructor_name: "z".to_string(),
        price_cents: 0,
        is_published: false,
    };

    let wire = serde_json::to_value(&course).expect("serialize");
    // R9-D NEGATIVE: id MUST be a JSON STRING, not a JSON NUMBER.
    assert!(
        wire["id"].is_string(),
        "id MUST be JSON string (Meilisearch doc key), not number"
    );
    assert_eq!(wire["id"].as_str(), Some(above_i32_max_as_string.as_str()));
    // And the value round-trips intact past i32::MAX.
    let round: SearchableCourse = serde_json::from_value(wire).expect("deserialize");
    assert_eq!(round.id, above_i32_max_as_string);
}
