//! CMS SEO Validation Routes - Apple ICT 11+ Principal Engineer Grade
//! January 2026
//!
//! Server-side SEO validation endpoint with comprehensive content analysis:
//! - Title and meta description validation
//! - Keyword density and placement analysis
//! - Heading hierarchy validation
//! - Readability scoring (Flesch-Kincaid)
//! - Link analysis (internal/external ratio)
//! - Image alt text presence
//! - Content length validation
//! - Duplicate content detection
//! - URL/slug optimization
//!
//! Features:
//! - Rate limiting (30 requests/minute per user)
//! - Comprehensive error handling
//! - SQL injection prevention via parameterized queries
//! - Caching support for duplicate detection
//!
//! ## Layout (R19-B structural split, May 2026)
//!
//! The original `routes/cms_seo.rs` (1,579 LOC) was carved into a
//! directory of focused sub-modules. The public API
//! (`routes::cms_seo::router()` and every previously-`pub` DTO consumed
//! by `tests/cms_seo_test.rs`) is preserved exactly via re-exports.
//!
//! - `types.rs` — DTOs and enums (`SeoIssue`, `SeoValidationRequest`,
//!   `SeoValidationResponse`, etc.)
//! - `regex_patterns.rs` — `lazy_static!` regex blocks
//! - `text.rs` — block→text/HTML extraction + word / sentence /
//!   syllable counts + Flesch-Kincaid + keyword density
//! - `auth.rs` — `require_cms_editor` RBAC + Redis-backed rate limit
//! - `analyzers.rs` — per-category analyzers (title, meta, content,
//!   readability, slug, structure, links) + `calculate_grade`
//! - `handlers.rs` — the `POST /validate` handler that orchestrates
//!   the analyzers and folds scores into the response

use axum::{http::StatusCode, routing::post, Json, Router};
use serde_json::{json, Value as JsonValue};

use crate::AppState;

mod analyzers;
mod auth;
mod handlers;
mod regex_patterns;
mod text;
mod types;

// Re-export the public DTO surface so external consumers (currently
// `tests/cms_seo_test.rs`) keep the same import paths.
pub use types::{
    CategoryScores, HeadingNode, LinksAnalysis, SeoIssue, SeoIssueCategory, SeoIssueSeverity,
    SeoValidationRequest, SeoValidationResponse,
};

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Rate limit: 30 requests per minute per user
pub(crate) const RATE_LIMIT_MAX_REQUESTS: i64 = 30;
pub(crate) const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;

/// SEO thresholds
pub(crate) const TITLE_MIN_LENGTH: usize = 30;
pub(crate) const TITLE_OPTIMAL_MIN: usize = 50;
pub(crate) const TITLE_OPTIMAL_MAX: usize = 60;
pub(crate) const TITLE_MAX_LENGTH: usize = 70;

pub(crate) const META_MIN_LENGTH: usize = 70;
pub(crate) const META_OPTIMAL_MIN: usize = 150;
pub(crate) const META_OPTIMAL_MAX: usize = 160;
pub(crate) const META_MAX_LENGTH: usize = 180;

pub(crate) const MIN_WORD_COUNT: u32 = 300;
pub(crate) const GOOD_WORD_COUNT: u32 = 1000;
pub(crate) const EXCELLENT_WORD_COUNT: u32 = 1500;

pub(crate) const KEYWORD_DENSITY_MIN: f32 = 0.5;
pub(crate) const KEYWORD_DENSITY_MAX: f32 = 3.0;
pub(crate) const KEYWORD_DENSITY_OPTIMAL_MIN: f32 = 1.0;
pub(crate) const KEYWORD_DENSITY_OPTIMAL_MAX: f32 = 2.5;

pub(crate) const SLUG_MAX_LENGTH: usize = 75;

/// Words per minute for reading time calculation
pub(crate) const WORDS_PER_MINUTE: u32 = 200;

// ═══════════════════════════════════════════════════════════════════════════════════════
// API RESULT TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

pub(crate) type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

pub(crate) fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

pub(crate) fn api_error_with_details(
    status: StatusCode,
    message: &str,
    details: JsonValue,
) -> (StatusCode, Json<JsonValue>) {
    (
        status,
        Json(json!({
            "error": message,
            "details": details
        })),
    )
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS SEO validation routes (requires authentication)
pub fn router() -> Router<AppState> {
    Router::new().route("/validate", post(handlers::validate_seo))
}
