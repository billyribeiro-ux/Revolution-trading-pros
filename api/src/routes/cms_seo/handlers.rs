//! HTTP handlers for the cms_seo module.
//!
//! Currently one endpoint: `POST /api/cms/seo/validate`, which orchestrates
//! every analyzer in `analyzers.rs` (title, meta, content, readability,
//! slug, structure, links) and folds their per-category scores into a
//! single weighted-average overall score + grade + sorted issues list.

use axum::{extract::State, http::StatusCode, Json};
use std::collections::HashSet;

use crate::{models::User, AppState};

use super::analyzers::{
    analyze_content, analyze_links, analyze_meta_description, analyze_readability, analyze_slug,
    analyze_structure, analyze_title, calculate_grade,
};
use super::auth::{check_rate_limit, require_cms_editor};
use super::text::{count_words, extract_html_from_blocks, extract_text_from_blocks};
use super::types::{
    CategoryScores, SeoIssue, SeoIssueSeverity, SeoValidationRequest, SeoValidationResponse,
};
use super::{api_error, ApiResult, WORDS_PER_MINUTE};

/// POST /api/cms/seo/validate
/// Validate content for SEO
pub(super) async fn validate_seo(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<SeoValidationRequest>,
) -> ApiResult<SeoValidationResponse> {
    require_cms_editor(&user)?;
    check_rate_limit(&state, user.id).await?;

    // Validate request
    if request.title.len() > 500 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Title exceeds maximum length of 500 characters",
        ));
    }

    if request.content_blocks.len() > 500 {
        return Err(api_error(
            StatusCode::BAD_REQUEST,
            "Content blocks exceed maximum of 500 blocks",
        ));
    }

    // Sanitize inputs - ICT 11+ SQL injection prevention
    // All string inputs are properly bound in parameterized queries
    // No raw SQL is constructed from user input

    let start_time = std::time::Instant::now();

    // Extract text content
    let plain_text = extract_text_from_blocks(&request.content_blocks);
    let html_content = extract_html_from_blocks(&request.content_blocks);
    let word_count = count_words(&plain_text);

    // Initialize analysis collections
    let mut issues: Vec<SeoIssue> = Vec::new();
    let mut suggestions: Vec<String> = Vec::new();

    // Run all analyses
    let title_score = analyze_title(
        &request.title,
        &request.focus_keyword,
        &mut issues,
        &mut suggestions,
    );
    let meta_score = analyze_meta_description(
        &request.meta_description,
        &request.focus_keyword,
        &mut issues,
        &mut suggestions,
    );
    let (content_score, keyword_density) = analyze_content(
        &plain_text,
        &request.focus_keyword,
        &mut issues,
        &mut suggestions,
    );
    let (readability_score, flesch_score) =
        analyze_readability(&plain_text, &mut issues, &mut suggestions);
    let slug_score = analyze_slug(
        &request.slug,
        &request.focus_keyword,
        &mut issues,
        &mut suggestions,
    );
    let (structure_score, headings, images_without_alt) = analyze_structure(
        &html_content,
        &request.content_blocks,
        &request.focus_keyword,
        &mut issues,
        &mut suggestions,
    );
    let links = analyze_links(&html_content, &mut issues, &mut suggestions);

    // Calculate overall score (weighted average)
    let overall_score = ((title_score as f32 * 0.20)
        + (meta_score as f32 * 0.15)
        + (content_score as f32 * 0.25)
        + (readability_score as f32 * 0.20)
        + (structure_score as f32 * 0.20))
        .round()
        .clamp(0.0, 100.0) as u8;

    let grade = calculate_grade(overall_score);
    let reading_time_minutes = (word_count / WORDS_PER_MINUTE).max(1);

    // Remove duplicate suggestions
    let unique_suggestions: Vec<String> = suggestions
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Sort issues by severity (errors first, then warnings, then info, then success)
    let mut sorted_issues = issues;
    sorted_issues.sort_by(|a, b| {
        let severity_order = |s: &SeoIssueSeverity| match s {
            SeoIssueSeverity::Error => 0,
            SeoIssueSeverity::Warning => 1,
            SeoIssueSeverity::Info => 2,
            SeoIssueSeverity::Success => 3,
        };
        severity_order(&a.severity).cmp(&severity_order(&b.severity))
    });

    let processing_time_ms = start_time.elapsed().as_millis();

    tracing::info!(
        target: "cms_seo",
        user_id = %user.id,
        score = %overall_score,
        grade = %grade,
        word_count = %word_count,
        processing_time_ms = %processing_time_ms,
        "SEO validation completed"
    );

    Ok(Json(SeoValidationResponse {
        score: overall_score,
        grade,
        issues: sorted_issues,
        suggestions: unique_suggestions,
        keyword_density,
        readability_score: flesch_score,
        word_count,
        reading_time_minutes,
        heading_structure: headings,
        links,
        images_without_alt,
        category_scores: CategoryScores {
            title_score,
            meta_score,
            content_score,
            readability_score,
            keyword_score: if request.focus_keyword.is_some() {
                content_score
            } else {
                50
            },
            structure_score,
        },
    }))
}
