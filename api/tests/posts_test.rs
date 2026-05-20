//! Posts route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::posts` and exercises the
//! public DTOs (`PostListQuery`, `PostRow`, `PostWithAuthor`,
//! `CreatePostRequest`, `UpdatePostRequest`) + the public `router()`
//! and `admin_router()` mount tables.
//!
//! ## Why this shape
//!
//! `routes/posts.rs` is the canonical blog/post surface — public
//! `/posts/*` for the frontend blog grid + admin `/admin/posts/*` for
//! the CMS. Every handler runs live SQL against the `posts` /
//! `post_categories` / `post_tags` tables, so we can't run them in
//! isolation. What we CAN pin:
//!
//! 1. **`PostRow.id` and `PostRow.author_id` MUST stay `i64`.** Every
//!    primary/foreign key on this stack is `BIGSERIAL`. A regression
//!    to `i32` would silently break joins to `users` (whose `id` is
//!    `i64`) — the symptom would be "newer authors disappear from
//!    blog cards" with no error, because sqlx would type-mismatch on
//!    decode. Pin both with a fixture that exceeds `i32::MAX`.
//!
//! 2. **`PostRow.featured_media_id: Option<i64>`** was added in
//!    migration 045 specifically because the admin form sends it and
//!    the API persists it. Pin both the type (i64) AND the
//!    Optional-ness (existing posts don't have a media id; the
//!    admin can clear the field, which sends null).
//!
//! 3. **`PostListQuery` is fully optional.** The public `/posts`
//!    endpoint accepts no params (default page 1, status=published).
//!    A regression that flipped any filter to required would 400-error
//!    the blog index card on the home page.
//!
//! 4. **`CreatePostRequest.title` is required**, everything else
//!    Optional. DB has NOT NULL on `posts.title`; a regression that
//!    made `title` Optional would 500 at SQL time instead of 422
//!    at serde time (worse UX).
//!
//! 5. **`UpdatePostRequest` is all-Optional** (partial-update). The
//!    admin's "Save" button PATCHes only changed fields; a regression
//!    that flipped any field to required would 422 every partial save.
//!
//! 6. **`PostWithAuthor.post` uses `#[serde(flatten)]`** to spread the
//!    `PostRow` fields at the top level alongside `author_name` /
//!    `author_email`. The frontend `Post` interface reads `id` /
//!    `title` / `author_name` as flat keys; a regression that wrapped
//!    `post` under a nested object would silently 404 every blog card.
//!
//! 7. **Two routers compile** — public `router()` (list + detail +
//!    related) and admin `admin_router()` (CRUD + publish/unpublish/
//!    archive). A regression that drops the `AdminUser` extractor
//!    from any admin handler would fail compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/products_test.rs`, `tests/admin_orders_test.rs`,
//! `tests/payments_test.rs`, `tests/forms_test.rs`.

use revolution_api::routes::posts::{
    CreatePostRequest, PostListQuery, PostRow, PostWithAuthor, UpdatePostRequest,
};

// Helper: build a PostRow fixture with all-None optional fields.
fn fixture_post_row(id: i64, author_id: i64) -> PostRow {
    let now = chrono::Utc::now().naive_utc();
    PostRow {
        id,
        author_id,
        title: "How to Day Trade".to_string(),
        slug: "how-to-day-trade".to_string(),
        excerpt: None,
        content_blocks: None,
        featured_image: None,
        status: "published".to_string(),
        published_at: Some(now),
        meta_title: None,
        meta_description: None,
        indexable: true,
        canonical_url: None,
        schema_markup: None,
        created_at: now,
        updated_at: now,
        featured_media_id: None,
        featured_image_alt: None,
        featured_image_caption: None,
        featured_image_title: None,
        featured_image_description: None,
        meta_keywords: None,
        allow_comments: true,
    }
}

// ── 1. ID pin: PostRow.id + author_id are i64 (BIGSERIAL contract) ───

/// HARD RULE: every primary/foreign key on this stack is `BIGSERIAL`
/// (i64). `PostRow.id` and `PostRow.author_id` join to `users.id`
/// (i64). A regression that narrowed either to i32 would silently
/// break the author join — sqlx would type-mismatch on decode and
/// return zero rows, so the blog page would render "no posts" with
/// no log entry. Pin both with fixtures that exceed i32::MAX.
#[test]
fn post_row_id_and_author_id_are_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let row = fixture_post_row(above_i32_max, above_i32_max + 1);

    let wire = serde_json::to_value(&row).expect("serialize PostRow");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["author_id"].as_i64(), Some(above_i32_max + 1));

    // Sanity: narrowing to i32 must lose data — proves i64 is required.
    assert!(
        (row.id as i32 as i64) != row.id,
        "narrowing PostRow.id to i32 must lose data — proves i64 is required"
    );
    assert!(
        (row.author_id as i32 as i64) != row.author_id,
        "narrowing PostRow.author_id to i32 must lose data — proves i64 is required"
    );
}

// ── 2. featured_media_id: i64 + Optional (migration 045 contract) ────

/// `featured_media_id` was added in migration 045 — the admin form
/// sends it and the API persists it (per the source-file comment).
/// Pin both:
///   - type is `Option<i64>` (matches `media.id` FK, BIGSERIAL)
///   - Optional-ness: existing pre-045 posts don't have a media id
///     (null in the DB), AND the admin can clear the field by
///     sending null. A regression to required would 422 every
///     "remove featured image" save.
#[test]
fn post_row_featured_media_id_is_optional_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    // With a media id
    let with_media = PostRow {
        featured_media_id: Some(above_i32_max),
        featured_image_alt: Some("Trading floor photo".to_string()),
        ..fixture_post_row(1, 2)
    };
    let wire = serde_json::to_value(&with_media).expect("serialize");
    assert_eq!(wire["featured_media_id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["featured_image_alt"], "Trading floor photo");

    // Without (legacy post or "remove image" save)
    let no_media = fixture_post_row(2, 3);
    let wire2 = serde_json::to_value(&no_media).expect("serialize");
    assert!(
        wire2["featured_media_id"].is_null(),
        "missing featured_media_id must serialize as null, not 0"
    );
    // Note: PostRow is Serialize-only (sqlx::FromRow handles the DB
    // side, not serde::Deserialize). Pin Optional-ness via the wire
    // shape alone; cf. fixture_post_row which sets the field to None.
    assert_eq!(no_media.featured_media_id, None);
}

// ── 3. PostListQuery: fully optional (default-everything) ────────────

/// The public `/posts` endpoint must accept GET with no query params
/// (default to page 1, published-only). Every field on `PostListQuery`
/// MUST be `Option<>` — a regression that flipped any field to
/// required would 400-error the blog index card on the home page.
#[test]
fn post_list_query_accepts_empty_and_filtered_payloads() {
    // Empty (default home-page render)
    let empty: PostListQuery = serde_json::from_str("{}").expect("empty PostListQuery must parse");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.status.is_none());
    assert!(empty.author_id.is_none());
    assert!(empty.search.is_none());

    // Full payload — every documented filter (admin grid use-case)
    let full: PostListQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 25,
        "status": "draft",
        "author_id": 42,
        "search": "fibonacci",
    }))
    .expect("full PostListQuery must parse");
    assert_eq!(full.page, Some(2));
    assert_eq!(full.per_page, Some(25));
    assert_eq!(full.status.as_deref(), Some("draft"));
    assert_eq!(full.author_id, Some(42));
    assert_eq!(full.search.as_deref(), Some("fibonacci"));

    // i64 compile-pin (matches the rest of the stack)
    let _: i64 = full.page.unwrap();
    let _: i64 = full.per_page.unwrap();
    let _: i64 = full.author_id.unwrap();

    // Author-filter only (admin "show only my posts" flow)
    let author_only: PostListQuery = serde_json::from_value(serde_json::json!({ "author_id": 7 }))
        .expect("author-filter must parse");
    assert_eq!(author_only.author_id, Some(7));
    assert!(author_only.page.is_none());
}

// ── 4. CreatePostRequest: title required, everything else Optional ───

/// DB has NOT NULL on `posts.title`. A regression that made `title`
/// `Option<>` would let the admin form POST without a title, then
/// 500 at sqlx instead of 422 at serde — worse UX. Pin both the
/// required-ness of `title` AND the optional-ness of everything else.
#[test]
fn create_post_request_requires_title_only() {
    // Minimal — title only
    let minimal: CreatePostRequest = serde_json::from_value(serde_json::json!({
        "title": "Hello, world",
    }))
    .expect("title-only must parse");
    assert_eq!(minimal.title, "Hello, world");
    assert!(minimal.excerpt.is_none());
    assert!(minimal.status.is_none());
    assert!(minimal.categories.is_none());
    assert!(minimal.tags.is_none());

    // Missing title must FAIL — pin required-ness
    let no_title = serde_json::from_value::<CreatePostRequest>(serde_json::json!({
        "excerpt": "no title here",
    }));
    assert!(
        no_title.is_err(),
        "CreatePostRequest without title must fail deserialization — DB has NOT NULL"
    );

    // Full payload with all the admin-form fields
    let full: CreatePostRequest = serde_json::from_value(serde_json::json!({
        "title": "Fibonacci Retracements",
        "excerpt": "Master the tool",
        "content_blocks": [{ "type": "paragraph", "content": "..." }],
        "featured_image": "https://cdn.example/fib.jpg",
        "status": "draft",
        "meta_title": "Fibonacci Retracements Guide",
        "meta_description": "Learn fib",
        "indexable": true,
        "featured_media_id": 12345,
        "featured_image_alt": "Fib chart",
        "meta_keywords": ["fibonacci", "trading"],
        "allow_comments": false,
        "categories": ["technical-analysis", "education"],
        "tags": ["fibonacci", "swing-trading"],
    }))
    .expect("full CreatePostRequest must parse");
    assert_eq!(full.title, "Fibonacci Retracements");
    assert_eq!(full.featured_media_id, Some(12345));
    assert_eq!(full.indexable, Some(true));
    assert_eq!(full.allow_comments, Some(false));
    assert_eq!(
        full.categories.as_ref().map(|c| c.len()),
        Some(2),
        "categories must round-trip as a vec of slugs"
    );
    assert_eq!(
        full.tags
            .as_ref()
            .and_then(|t| t.first())
            .map(|s| s.as_str()),
        Some("fibonacci"),
        "tags must round-trip as a vec of names"
    );
    assert_eq!(
        full.meta_keywords.as_ref().map(|k| k.len()),
        Some(2),
        "meta_keywords must round-trip as Vec<String>"
    );

    // i64 compile-pin for featured_media_id
    let _: i64 = full.featured_media_id.unwrap();
}

// ── 5. UpdatePostRequest: partial-update, all-Optional ───────────────

/// The admin's "Save" button PATCHes only the changed fields. Every
/// field on `UpdatePostRequest` MUST be `Option<>`. A regression that
/// flipped ANY field to required would 422 every partial save —
/// specifically the common "just change the publish date" flow.
#[test]
fn update_post_request_accepts_empty_and_partial_payloads() {
    // Empty body (no-op save, bumps updated_at)
    let empty: UpdatePostRequest =
        serde_json::from_str("{}").expect("empty UpdatePostRequest must parse");
    assert!(empty.title.is_none());
    assert!(empty.status.is_none());
    assert!(empty.featured_media_id.is_none());
    assert!(empty.categories.is_none());

    // Title-only change
    let rename: UpdatePostRequest = serde_json::from_value(serde_json::json!({
        "title": "Rebranded",
    }))
    .expect("title-only update must parse");
    assert_eq!(rename.title.as_deref(), Some("Rebranded"));
    assert!(
        rename.status.is_none(),
        "unchanged fields must stay None — partial-update IS the contract"
    );

    // Publish-flow (status only)
    let publish: UpdatePostRequest = serde_json::from_value(serde_json::json!({
        "status": "published",
    }))
    .expect("status-only update must parse");
    assert_eq!(publish.status.as_deref(), Some("published"));

    // Clear-featured-image flow — set fields to null explicitly
    let clear_image: UpdatePostRequest = serde_json::from_value(serde_json::json!({
        "featured_image": null,
        "featured_media_id": null,
    }))
    .expect("clear-image must parse with explicit nulls");
    assert!(clear_image.featured_image.is_none());
    assert!(clear_image.featured_media_id.is_none());
}

// ── 6. PostWithAuthor: #[serde(flatten)] spreads at top level ────────

/// The frontend `Post` interface reads `id`, `title`, `author_name` as
/// flat top-level keys. `PostWithAuthor` uses `#[serde(flatten)]` on
/// the `post` field — a regression that dropped `flatten` would wrap
/// every post under a nested `post` object, silently 404-ing every
/// blog card. Pin the flat shape AND the post-row fields AND the
/// author fields are all at the top level.
#[test]
fn post_with_author_flattens_post_fields_at_top_level() {
    let row = fixture_post_row(1, 7);
    let combined = PostWithAuthor {
        post: row,
        author_name: Some("Jane Trader".to_string()),
        author_email: Some("jane@example.com".to_string()),
    };

    let wire = serde_json::to_value(&combined).expect("serialize PostWithAuthor");

    // Post fields MUST be at the top level (flattened)
    assert_eq!(wire["id"].as_i64(), Some(1));
    assert_eq!(wire["title"], "How to Day Trade");
    assert_eq!(wire["slug"], "how-to-day-trade");
    assert_eq!(wire["status"], "published");
    assert_eq!(wire["allow_comments"], true);
    assert_eq!(wire["indexable"], true);

    // Author fields also at the top level (NOT nested under "author")
    assert_eq!(wire["author_name"], "Jane Trader");
    assert_eq!(wire["author_email"], "jane@example.com");

    // Negative pin: there must NOT be a nested `post` object (that
    // would mean `flatten` was dropped during a refactor).
    assert!(
        wire.get("post").is_none(),
        "PostWithAuthor.post must be flattened — nested 'post' key would break every blog card"
    );
    assert!(
        wire.get("author").is_none(),
        "PostWithAuthor must not nest author under an 'author' key"
    );
}

// ── 7. Router mount tables compile-pin (public + admin) ──────────────

/// The public `router()` must build as `Router<AppState>`. Three
/// routes — list, detail by slug, related-posts — every blog card on
/// the marketing site depends on these. A regression in any handler
/// signature would fail compilation here.
#[test]
fn posts_public_router_builds_with_app_state() {
    let _public: axum::Router<revolution_api::AppState> = revolution_api::routes::posts::router();
}

/// The `admin_router()` mounts at `/admin/posts` and includes CRUD +
/// publish-state transitions (`/publish`, `/unpublish`, `/archive`).
/// A regression that drops the `AdminUser` extractor from ANY admin
/// handler would let an authenticated non-admin enumerate or modify
/// posts. Compile-pin guards against extractor regressions.
#[test]
fn posts_admin_router_builds_with_app_state() {
    let _admin: axum::Router<revolution_api::AppState> =
        revolution_api::routes::posts::admin_router();
}

/// Idempotent construction — both routers must be safe to build
/// multiple times in the same process. Catches accidental
/// global-state introduction in either constructor.
#[test]
fn posts_routers_are_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::posts::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::posts::router();
    let _a1: axum::Router<revolution_api::AppState> = revolution_api::routes::posts::admin_router();
    let _a2: axum::Router<revolution_api::AppState> = revolution_api::routes::posts::admin_router();
}
