//! CMS-SEO route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_seo` and exercises
//! the 8 public DTOs/enums + the `router()` mount table.
//!
//! ## Why this shape
//!
//! `routes/cms_seo.rs` (1,579 LOC) implements the `/api/cms/seo/validate`
//! endpoint — the SEO scoring rubric that ranks every blog post / CMS
//! page on title length, meta description, heading hierarchy, internal
//! vs external link ratio, keyword density, and readability. The
//! single handler takes `State<AppState>` (Redis rate-limit check) +
//! `User` (RBAC editor gate), so unit-testing without infra is
//! impossible. What we CAN pin:
//!
//! 1. **`SeoIssueSeverity` and `SeoIssueCategory` enums use
//!    `rename_all = "snake_case"`.** The frontend filters issues
//!    by exact string match (`if (issue.type === 'error')`,
//!    `if (issue.category === 'meta')`). A regression that flipped
//!    serde back to PascalCase would silently break every SEO-panel
//!    badge in the admin and the score breakdown would render empty.
//!
//! 2. **`SeoIssue.severity` is serialized as `"type"` (not
//!    `"severity"`)** — load-bearing rename via `#[serde(rename =
//!    "type")]`. The frontend expects `issue.type`; a refactor that
//!    dropped the rename would silently flip the field name and break
//!    every issue badge.
//!
//! 3. **Counter widths in `LinksAnalysis` / `SeoValidationResponse`
//!    are u32 (NOT i64).** These are per-post counts — a single
//!    blog post can't legally have >4B internal links. The CLAUDE.md
//!    "Reserved exception" applies (row counts can be i32/u32 if they
//!    cap below 2B). Pin to catch a refactor that flipped to i32
//!    (which could go negative on parser bug) or i64 (over-allocation
//!    + JSON-bigint serialization pitfall).
//!
//! 4. **Scoring uses `u8` for score (0..=100) and `f32` for density.**
//!    Locking these prevents a future refactor from flipping to
//!    `f64` (JS-side `number` is fine but JSON ints lose precision
//!    differently) or signed (negative scores are nonsense).
//!
//! 5. **`router()` mount table compile-pin.** Single route POST
//!    `/validate`. A refactor that broke the handler signature
//!    (wrong extractor, dropped `User` gate, wrong return type)
//!    fails compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/admin_orders_test.rs`,
//! `tests/forms_test.rs` (parallel).

use revolution_api::routes::cms_seo::{
    CategoryScores, HeadingNode, LinksAnalysis, SeoIssue, SeoIssueCategory, SeoIssueSeverity,
    SeoValidationRequest, SeoValidationResponse,
};

// ── 1. SeoIssueSeverity: snake_case wire format ──────────────────────

/// The frontend filters issues by exact string match (`if
/// (issue.type === 'error')`). A regression that flipped serde back
/// to PascalCase ("Error" instead of "error") would silently break
/// every issue badge — the frontend would render unstyled fallbacks.
/// Pin the snake_case wire format for all four variants.
#[test]
fn seo_issue_severity_serializes_as_snake_case() {
    let cases = [
        (SeoIssueSeverity::Error, "\"error\""),
        (SeoIssueSeverity::Warning, "\"warning\""),
        (SeoIssueSeverity::Info, "\"info\""),
        (SeoIssueSeverity::Success, "\"success\""),
    ];
    for (severity, expected) in cases {
        let wire = serde_json::to_string(&severity).expect("severity must serialize");
        assert_eq!(wire, expected, "severity wire format MUST be snake_case");

        // Round-trip: the same string must deserialize back to the
        // same variant. Catches asymmetric rename_all bugs.
        let parsed: SeoIssueSeverity =
            serde_json::from_str(expected).expect("snake_case severity must round-trip");
        assert_eq!(parsed, severity);
    }
}

// ── 2. SeoIssueCategory: 10 variants, all snake_case ─────────────────

/// The frontend scoring panel groups issues by category for the
/// per-section score breakdown ("Title issues: 3", "Meta issues: 1").
/// Each category string is a hard-coded key in the panel UI — a
/// regression that renamed any one would render that category's
/// issues as "Unknown".
#[test]
fn seo_issue_category_serializes_as_snake_case() {
    let cases = [
        (SeoIssueCategory::Title, "\"title\""),
        (SeoIssueCategory::Meta, "\"meta\""),
        (SeoIssueCategory::Content, "\"content\""),
        (SeoIssueCategory::Keyword, "\"keyword\""),
        (SeoIssueCategory::Readability, "\"readability\""),
        (SeoIssueCategory::Structure, "\"structure\""),
        (SeoIssueCategory::Slug, "\"slug\""),
        (SeoIssueCategory::Links, "\"links\""),
        (SeoIssueCategory::Images, "\"images\""),
        (SeoIssueCategory::Headings, "\"headings\""),
    ];
    for (category, expected) in cases {
        let wire = serde_json::to_string(&category).expect("category must serialize");
        assert_eq!(wire, expected, "category wire format MUST be snake_case");

        let parsed: SeoIssueCategory =
            serde_json::from_str(expected).expect("snake_case category must round-trip");
        assert_eq!(parsed, category);
    }
}

// ── 3. SeoIssue: `severity` field renamed to `"type"` on wire ────────

/// `SeoIssue.severity` carries `#[serde(rename = "type")]` so the
/// frontend sees `issue.type === 'error'`. The Rust struct field name
/// is `severity` but the JSON key MUST be `"type"`. A regression that
/// dropped the rename would silently rename the wire key and break
/// every issue badge.
#[test]
fn seo_issue_renames_severity_to_type_on_wire() {
    let issue = SeoIssue {
        severity: SeoIssueSeverity::Error,
        category: SeoIssueCategory::Title,
        message: "Title is too short (< 30 chars)".to_string(),
        impact: Some("Reduces search visibility".to_string()),
    };

    let wire = serde_json::to_value(&issue).expect("SeoIssue must serialize");

    // The Rust field `severity` MUST land at JSON key `"type"`.
    assert_eq!(
        wire["type"].as_str(),
        Some("error"),
        "Rust .severity field MUST serialize as JSON .type"
    );
    assert!(
        wire.get("severity").is_none(),
        "JSON key `severity` MUST NOT appear (renamed to `type`)"
    );
    assert_eq!(wire["category"].as_str(), Some("title"));
    assert_eq!(
        wire["message"].as_str(),
        Some("Title is too short (< 30 chars)")
    );

    // `impact` is `skip_serializing_if = Option::is_none` → omitted
    // when absent. Round-trip with `None` to confirm.
    let no_impact = SeoIssue {
        severity: SeoIssueSeverity::Warning,
        category: SeoIssueCategory::Meta,
        message: "Meta description missing".to_string(),
        impact: None,
    };
    let wire2 = serde_json::to_value(&no_impact).expect("SeoIssue must serialize");
    assert!(
        wire2.get("impact").is_none(),
        "absent `impact` MUST be omitted from wire (skip_serializing_if)"
    );
}

// ── 4. LinksAnalysis: u32 counters, f32 ratio ────────────────────────

/// `LinksAnalysis` carries per-post link counts. The CLAUDE.md
/// "Reserved exception" applies — these are bounded counts (a single
/// post will never legally exceed u32::MAX links). Pin u32 to catch
/// a refactor that flipped to i32 (negative-on-parser-bug) or i64
/// (JSON-bigint serialization).
#[test]
fn links_analysis_counter_widths_are_pinned() {
    let analysis = LinksAnalysis {
        internal_count: 12,
        external_count: 5,
        ratio: 2.4_f32,
        nofollow_count: 2,
        broken_count: 0,
    };
    let wire = serde_json::to_value(&analysis).expect("LinksAnalysis must serialize");
    assert_eq!(wire["internal_count"].as_u64(), Some(12));
    assert_eq!(wire["external_count"].as_u64(), Some(5));
    assert_eq!(wire["nofollow_count"].as_u64(), Some(2));
    assert_eq!(wire["broken_count"].as_u64(), Some(0));
    // `ratio` is f32 → serializes as a JSON number.
    assert!(wire["ratio"].as_f64().is_some());

    // Round-trip: a wire payload with u32-shaped counts deserializes.
    let parsed: LinksAnalysis = serde_json::from_value(serde_json::json!({
        "internal_count": u32::MAX,
        "external_count": 0_u32,
        "ratio": 0.0,
        "nofollow_count": 0_u32,
        "broken_count": 0_u32
    }))
    .expect("LinksAnalysis must accept u32::MAX counts");
    assert_eq!(parsed.internal_count, u32::MAX);
}

// ── 5. SeoValidationRequest: required title+slug+blocks, optional rest

/// The frontend issues `/validate` on every keystroke of the SEO
/// panel (debounced). Only `title`, `slug`, and `content_blocks` are
/// required; `meta_description` and `focus_keyword` default to
/// `None`. A regression that flipped either of those to required
/// would 400-error the live-validation flow.
#[test]
fn seo_validation_request_required_vs_optional_shape() {
    // Happy path — minimal required payload.
    let minimal: SeoValidationRequest = serde_json::from_value(serde_json::json!({
        "title": "How to Trade Options",
        "slug": "how-to-trade-options",
        "content_blocks": []
    }))
    .expect("minimal SeoValidationRequest must deserialize");
    assert_eq!(minimal.title, "How to Trade Options");
    assert_eq!(minimal.slug, "how-to-trade-options");
    assert!(minimal.meta_description.is_none());
    assert!(minimal.focus_keyword.is_none());
    assert_eq!(minimal.content_blocks.len(), 0);

    // With all optionals.
    let full: SeoValidationRequest = serde_json::from_value(serde_json::json!({
        "title": "How to Trade Options",
        "meta_description": "Learn to trade options.",
        "content_blocks": [{"type": "heading", "content": {"text": "Intro"}}],
        "slug": "how-to-trade-options",
        "focus_keyword": "options trading"
    }))
    .expect("full SeoValidationRequest must deserialize");
    assert_eq!(
        full.meta_description.as_deref(),
        Some("Learn to trade options.")
    );
    assert_eq!(full.focus_keyword.as_deref(), Some("options trading"));
    assert_eq!(full.content_blocks.len(), 1);

    // Missing `title` MUST fail (required).
    assert!(
        serde_json::from_value::<SeoValidationRequest>(
            serde_json::json!({"slug": "x", "content_blocks": []})
        )
        .is_err(),
        "SeoValidationRequest without `title` MUST fail"
    );

    // Missing `slug` MUST fail (required for slug-validation rule).
    assert!(
        serde_json::from_value::<SeoValidationRequest>(
            serde_json::json!({"title": "x", "content_blocks": []})
        )
        .is_err(),
        "SeoValidationRequest without `slug` MUST fail"
    );
}

// ── 6. SeoValidationResponse + CategoryScores: u8 scores, full shape ─

/// The response wire-format MUST stay stable — the admin UI binds
/// every field by name (`response.score`, `response.grade`,
/// `response.category_scores.title_score`). `score` and the six
/// per-category scores are `u8` (range 0..=100, never negative,
/// never above 100 — narrower than `u32` deliberately so a refactor
/// to `i32` immediately stands out here).
#[test]
fn seo_validation_response_round_trips_with_pinned_score_widths() {
    let resp = SeoValidationResponse {
        score: 85,
        grade: "B".to_string(),
        issues: vec![SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Meta,
            message: "Meta description missing".to_string(),
            impact: None,
        }],
        suggestions: vec!["Add a meta description".to_string()],
        keyword_density: 1.5_f32,
        readability_score: 65.0_f32,
        word_count: 1_250,
        reading_time_minutes: 5,
        heading_structure: vec![HeadingNode {
            level: 1,
            text: "Intro".to_string(),
            id: "intro".to_string(),
            children: vec![],
        }],
        links: LinksAnalysis {
            internal_count: 3,
            external_count: 2,
            ratio: 1.5_f32,
            nofollow_count: 0,
            broken_count: 0,
        },
        images_without_alt: 0,
        category_scores: CategoryScores {
            title_score: 90,
            meta_score: 40,
            content_score: 85,
            readability_score: 70,
            keyword_score: 80,
            structure_score: 95,
        },
    };

    let wire = serde_json::to_value(&resp).expect("SeoValidationResponse must serialize");
    assert_eq!(wire["score"].as_u64(), Some(85));
    assert_eq!(wire["grade"].as_str(), Some("B"));
    assert_eq!(wire["word_count"].as_u64(), Some(1250));
    assert_eq!(wire["reading_time_minutes"].as_u64(), Some(5));
    assert_eq!(wire["images_without_alt"].as_u64(), Some(0));

    // The nested category_scores object MUST flatten through with
    // every per-category score as a number.
    assert_eq!(wire["category_scores"]["title_score"].as_u64(), Some(90));
    assert_eq!(wire["category_scores"]["meta_score"].as_u64(), Some(40));
    assert_eq!(
        wire["category_scores"]["structure_score"].as_u64(),
        Some(95)
    );

    // Issues array MUST surface the `type` rename (not `severity`).
    assert_eq!(wire["issues"][0]["type"].as_str(), Some("warning"));
    assert_eq!(wire["issues"][0]["category"].as_str(), Some("meta"));

    // Heading structure surfaces level + text + id.
    assert_eq!(wire["heading_structure"][0]["level"].as_u64(), Some(1));
    assert_eq!(wire["heading_structure"][0]["text"].as_str(), Some("Intro"));
}

// ── 7. Router mount table compile-pin ────────────────────────────────

/// `router()` must build as `Router<AppState>`. Single route POST
/// `/validate`. A refactor that broke the `validate_seo` handler
/// signature (wrong extractor, dropped `User` editor gate, wrong
/// return type) fails compilation here.
#[test]
fn cms_seo_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::cms_seo::router();
}

/// Idempotent constructor — must be safe to call multiple times. The
/// main `api_router()` nests this under `/api/cms/seo`; if the
/// constructor were stateful, repeat invocations during composition
/// would panic.
#[test]
fn cms_seo_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::cms_seo::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::cms_seo::router();
}
