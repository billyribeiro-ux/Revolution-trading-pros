//! Shared DTOs for the cms_seo module.
//!
//! Pure data types: request body, response, issue/category enums, and the
//! nested analysis structs (`HeadingNode`, `LinksAnalysis`, `CategoryScores`).
//!
//! These stay `pub` (and are re-exported from `routes::cms_seo`) so external
//! consumers — currently `tests/cms_seo_test.rs` which pins the snake_case
//! wire format — continue to compile with unchanged import paths after the
//! R19-B structural split.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// SEO issue severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SeoIssueSeverity {
    Error,
    Warning,
    Info,
    Success,
}

/// SEO issue category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SeoIssueCategory {
    Title,
    Meta,
    Content,
    Keyword,
    Readability,
    Structure,
    Slug,
    Links,
    Images,
    Headings,
}

/// Individual SEO issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeoIssue {
    #[serde(rename = "type")]
    pub severity: SeoIssueSeverity,
    pub category: SeoIssueCategory,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
}

/// Heading node for hierarchy analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingNode {
    pub level: u8,
    pub text: String,
    pub id: String,
    #[serde(default)]
    pub children: Vec<HeadingNode>,
}

/// Links analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinksAnalysis {
    pub internal_count: u32,
    pub external_count: u32,
    pub ratio: f32,
    pub nofollow_count: u32,
    pub broken_count: u32,
}

/// Request body for SEO validation
#[derive(Debug, Clone, Deserialize)]
pub struct SeoValidationRequest {
    pub title: String,
    #[serde(default)]
    pub meta_description: Option<String>,
    pub content_blocks: Vec<JsonValue>,
    pub slug: String,
    #[serde(default)]
    pub focus_keyword: Option<String>,
}

/// Response from SEO validation endpoint
#[derive(Debug, Clone, Serialize)]
pub struct SeoValidationResponse {
    pub score: u8,
    pub grade: String,
    pub issues: Vec<SeoIssue>,
    pub suggestions: Vec<String>,
    pub keyword_density: f32,
    pub readability_score: f32,
    pub word_count: u32,
    pub reading_time_minutes: u32,
    pub heading_structure: Vec<HeadingNode>,
    pub links: LinksAnalysis,
    pub images_without_alt: u32,
    /// Individual category scores
    pub category_scores: CategoryScores,
}

/// Individual category scores
#[derive(Debug, Clone, Serialize)]
pub struct CategoryScores {
    pub title_score: u8,
    pub meta_score: u8,
    pub content_score: u8,
    pub readability_score: u8,
    pub keyword_score: u8,
    pub structure_score: u8,
}
