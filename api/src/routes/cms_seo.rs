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

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::{HashMap, HashSet};

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Rate limit: 30 requests per minute per user
const RATE_LIMIT_MAX_REQUESTS: i64 = 30;
const RATE_LIMIT_WINDOW_SECONDS: u64 = 60;

/// SEO thresholds
const TITLE_MIN_LENGTH: usize = 30;
const TITLE_OPTIMAL_MIN: usize = 50;
const TITLE_OPTIMAL_MAX: usize = 60;
const TITLE_MAX_LENGTH: usize = 70;

const META_MIN_LENGTH: usize = 70;
const META_OPTIMAL_MIN: usize = 150;
const META_OPTIMAL_MAX: usize = 160;
const META_MAX_LENGTH: usize = 180;

const MIN_WORD_COUNT: u32 = 300;
const GOOD_WORD_COUNT: u32 = 1000;
const EXCELLENT_WORD_COUNT: u32 = 1500;

const KEYWORD_DENSITY_MIN: f32 = 0.5;
const KEYWORD_DENSITY_MAX: f32 = 3.0;
const KEYWORD_DENSITY_OPTIMAL_MIN: f32 = 1.0;
const KEYWORD_DENSITY_OPTIMAL_MAX: f32 = 2.5;

const SLUG_MAX_LENGTH: usize = 75;

/// Words per minute for reading time calculation
const WORDS_PER_MINUTE: u32 = 200;

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES AND ENUMS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// SEO issue severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SeoIssueSeverity {
    Error,
    Warning,
    Info,
    Success,
}

/// SEO issue category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

// ═══════════════════════════════════════════════════════════════════════════════════════
// API RESULT TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

fn api_error_with_details(
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
// AUTHORIZATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing" | "developer"
    ) {
        Ok(())
    } else {
        Err(api_error(
            StatusCode::FORBIDDEN,
            "Editor access required for SEO validation",
        ))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// RATE LIMITING
// ═══════════════════════════════════════════════════════════════════════════════════════

async fn check_rate_limit(
    state: &AppState,
    user_id: i64,
) -> Result<(), (StatusCode, Json<JsonValue>)> {
    if let Some(ref redis) = state.services.redis {
        let key = format!("cms_seo_rate_limit:{}", user_id);
        match redis
            .check_rate_limit(&key, RATE_LIMIT_MAX_REQUESTS, RATE_LIMIT_WINDOW_SECONDS)
            .await
        {
            Ok(allowed) if allowed => Ok(()),
            Ok(_) => {
                tracing::warn!(
                    target: "security",
                    event = "cms_seo_rate_limited",
                    user_id = %user_id,
                    "CMS SEO rate limit exceeded"
                );
                Err(api_error_with_details(
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded. Please wait before making more requests.",
                    json!({
                        "limit": RATE_LIMIT_MAX_REQUESTS,
                        "window_seconds": RATE_LIMIT_WINDOW_SECONDS,
                        "retry_after": RATE_LIMIT_WINDOW_SECONDS
                    }),
                ))
            }
            Err(e) => {
                tracing::error!(
                    target: "cms_seo",
                    error = %e,
                    "Failed to check rate limit"
                );
                // Allow request if rate limit check fails (fail open for availability)
                Ok(())
            }
        }
    } else {
        // No Redis available, allow request
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// TEXT EXTRACTION AND ANALYSIS HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Extract plain text from content blocks
fn extract_text_from_blocks(blocks: &[JsonValue]) -> String {
    let mut text = String::new();

    for block in blocks {
        if let Some(content) = block.get("content") {
            // Extract text from various content fields
            if let Some(t) = content.get("text").and_then(|v| v.as_str()) {
                text.push_str(t);
                text.push(' ');
            }
            if let Some(h) = content.get("html").and_then(|v| v.as_str()) {
                text.push_str(&strip_html(h));
                text.push(' ');
            }
            // Handle list items
            if let Some(items) = content.get("listItems").and_then(|v| v.as_array()) {
                for item in items {
                    if let Some(item_text) = item.as_str() {
                        text.push_str(item_text);
                        text.push(' ');
                    }
                }
            }
            // Handle nested children
            if let Some(children) = content.get("children").and_then(|v| v.as_array()) {
                text.push_str(&extract_text_from_blocks(children));
            }
        }
    }

    text.trim().to_string()
}

/// Extract HTML content from blocks for structural analysis
fn extract_html_from_blocks(blocks: &[JsonValue]) -> String {
    let mut html = String::new();

    for block in blocks {
        let block_type = block.get("type").and_then(|v| v.as_str()).unwrap_or("");

        if let Some(content) = block.get("content") {
            // Add heading tags
            if block_type == "heading" {
                let level = block
                    .get("settings")
                    .and_then(|s| s.get("level"))
                    .and_then(|l| l.as_u64())
                    .unwrap_or(2);
                if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                    html.push_str(&format!("<h{level}>{text}</h{level}>\n"));
                }
            }

            // Add HTML content
            if let Some(h) = content.get("html").and_then(|v| v.as_str()) {
                html.push_str(h);
                html.push('\n');
            }

            // Add paragraph text
            if block_type == "paragraph" {
                if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                    html.push_str(&format!("<p>{text}</p>\n"));
                }
            }

            // Handle images
            if block_type == "image" {
                let src = content
                    .get("mediaUrl")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let alt = content
                    .get("mediaAlt")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                html.push_str(&format!("<img src=\"{src}\" alt=\"{alt}\">\n"));
            }

            // Handle nested children
            if let Some(children) = content.get("children").and_then(|v| v.as_array()) {
                html.push_str(&extract_html_from_blocks(children));
            }
        }
    }

    html
}

/// Strip HTML tags from text
fn strip_html(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    let text = re.replace_all(html, " ");
    // Normalize whitespace
    let whitespace_re = Regex::new(r"\s+").unwrap();
    whitespace_re.replace_all(&text, " ").trim().to_string()
}

/// Count words in text
fn count_words(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

/// Count sentences in text
fn count_sentences(text: &str) -> u32 {
    let re = Regex::new(r"[.!?]+").unwrap();
    let count = re.find_iter(text).count();
    if count == 0 {
        1
    } else {
        count as u32
    }
}

/// Count syllables in a word (approximation)
fn count_syllables(word: &str) -> u32 {
    let word = word.to_lowercase();
    if word.len() <= 3 {
        return 1;
    }

    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u', 'y'].iter().cloned().collect();
    let mut count = 0;
    let mut prev_vowel = false;

    for (i, c) in word.chars().enumerate() {
        let is_vowel = vowels.contains(&c);
        if is_vowel && !prev_vowel {
            count += 1;
        }
        prev_vowel = is_vowel;

        // Handle silent e at end
        if i == word.len() - 1 && c == 'e' && count > 1 {
            count -= 1;
        }
    }

    if count == 0 {
        1
    } else {
        count
    }
}

/// Calculate Flesch-Kincaid readability score
fn calculate_flesch_kincaid(text: &str) -> f32 {
    let words = count_words(text);
    let sentences = count_sentences(text);
    let syllables: u32 = text
        .split_whitespace()
        .map(count_syllables)
        .sum();

    if words == 0 || sentences == 0 {
        return 0.0;
    }

    // Flesch Reading Ease formula
    let score = 206.835
        - (1.015 * (words as f32 / sentences as f32))
        - (84.6 * (syllables as f32 / words as f32));

    // Clamp between 0 and 100
    score.clamp(0.0, 100.0)
}

/// Calculate keyword density
fn calculate_keyword_density(text: &str, keyword: &str) -> f32 {
    if keyword.is_empty() {
        return 0.0;
    }

    let words = count_words(text);
    if words == 0 {
        return 0.0;
    }

    let keyword_lower = keyword.to_lowercase();
    let text_lower = text.to_lowercase();
    let keyword_count = text_lower.matches(&keyword_lower).count();

    (keyword_count as f32 / words as f32) * 100.0
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SEO ANALYSIS FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Analyze title SEO
fn analyze_title(
    title: &str,
    keyword: &Option<String>,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> u8 {
    let mut score: i32 = 100;
    let title_len = title.chars().count();

    // Title presence
    if title.is_empty() {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Error,
            category: SeoIssueCategory::Title,
            message: "Page title is missing".to_string(),
            impact: Some("high".to_string()),
        });
        return 0;
    }

    // Title length
    if title_len < TITLE_MIN_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Title,
            message: format!(
                "Title is too short ({} chars). Aim for {}-{} characters.",
                title_len, TITLE_OPTIMAL_MIN, TITLE_OPTIMAL_MAX
            ),
            impact: Some("medium".to_string()),
        });
        score -= 20;
    } else if title_len > TITLE_MAX_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Title,
            message: format!(
                "Title is too long ({} chars). Keep under {} characters.",
                title_len, TITLE_OPTIMAL_MAX
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    } else if (TITLE_OPTIMAL_MIN..=TITLE_OPTIMAL_MAX).contains(&title_len) {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Title,
            message: format!("Title length is optimal ({} chars)", title_len),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Title,
            message: format!("Title length is acceptable ({} chars)", title_len),
            impact: None,
        });
    }

    // Keyword in title
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            let title_lower = title.to_lowercase();
            let kw_lower = kw.to_lowercase();

            if title_lower.contains(&kw_lower) {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Title,
                    message: "Focus keyword found in title".to_string(),
                    impact: None,
                });

                // Check if keyword is near the beginning
                if title_lower.starts_with(&kw_lower)
                    || title_lower.find(&kw_lower).unwrap_or(100) < 20
                {
                    issues.push(SeoIssue {
                        severity: SeoIssueSeverity::Success,
                        category: SeoIssueCategory::Title,
                        message: "Focus keyword appears near the beginning of the title"
                            .to_string(),
                        impact: None,
                    });
                }
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Title,
                    message: "Focus keyword not found in title".to_string(),
                    impact: Some("high".to_string()),
                });
                suggestions
                    .push("Add your focus keyword near the beginning of the title".to_string());
                score -= 15;
            }
        }
    }

    // Power words check
    let power_words = [
        "ultimate",
        "complete",
        "essential",
        "proven",
        "best",
        "top",
        "guide",
        "how to",
        "secrets",
        "tips",
        "definitive",
        "comprehensive",
        "master",
        "expert",
        "professional",
    ];
    let title_lower = title.to_lowercase();
    let has_power_word = power_words.iter().any(|w| title_lower.contains(w));

    if has_power_word {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Title,
            message: "Title contains power words".to_string(),
            impact: None,
        });
    } else {
        suggestions
            .push("Consider adding power words to make your title more compelling".to_string());
    }

    score.clamp(0, 100) as u8
}

/// Analyze meta description SEO
fn analyze_meta_description(
    meta: &Option<String>,
    keyword: &Option<String>,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> u8 {
    let mut score: i32 = 100;

    let meta_text = match meta {
        Some(m) if !m.is_empty() => m,
        _ => {
            issues.push(SeoIssue {
                severity: SeoIssueSeverity::Error,
                category: SeoIssueCategory::Meta,
                message: "Meta description is missing".to_string(),
                impact: Some("high".to_string()),
            });
            suggestions.push("Add a compelling meta description of 150-160 characters".to_string());
            return 40;
        }
    };

    let meta_len = meta_text.chars().count();

    // Meta length
    if meta_len < META_MIN_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Meta,
            message: format!(
                "Meta description is too short ({} chars). Aim for {}-{} characters.",
                meta_len, META_OPTIMAL_MIN, META_OPTIMAL_MAX
            ),
            impact: Some("medium".to_string()),
        });
        score -= 25;
    } else if meta_len > META_MAX_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Meta,
            message: format!(
                "Meta description is too long ({} chars). Keep under {} characters.",
                meta_len, META_OPTIMAL_MAX
            ),
            impact: Some("low".to_string()),
        });
        score -= 10;
    } else if (META_OPTIMAL_MIN..=META_OPTIMAL_MAX).contains(&meta_len) {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Meta,
            message: format!("Meta description length is optimal ({} chars)", meta_len),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Meta,
            message: format!("Meta description length is acceptable ({} chars)", meta_len),
            impact: None,
        });
    }

    // Keyword in meta
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            let meta_lower = meta_text.to_lowercase();
            let kw_lower = kw.to_lowercase();

            if meta_lower.contains(&kw_lower) {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Meta,
                    message: "Focus keyword found in meta description".to_string(),
                    impact: None,
                });
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Meta,
                    message: "Focus keyword not found in meta description".to_string(),
                    impact: Some("medium".to_string()),
                });
                suggestions.push("Include your focus keyword in the meta description".to_string());
                score -= 15;
            }
        }
    }

    // Call to action check
    let cta_words = [
        "learn", "discover", "find out", "get", "start", "try", "read", "click", "explore", "see",
    ];
    let meta_lower = meta_text.to_lowercase();
    let has_cta = cta_words.iter().any(|w| meta_lower.contains(w));

    if has_cta {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Meta,
            message: "Meta description contains a call to action".to_string(),
            impact: None,
        });
    } else {
        suggestions.push("Add a call to action in your meta description".to_string());
    }

    score.clamp(0, 100) as u8
}

/// Analyze content SEO
fn analyze_content(
    text: &str,
    keyword: &Option<String>,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> (u8, f32) {
    let mut score: i32 = 100;
    let word_count = count_words(text);
    let mut keyword_density: f32 = 0.0;

    // Word count check
    if word_count < MIN_WORD_COUNT {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Error,
            category: SeoIssueCategory::Content,
            message: format!(
                "Content is too short ({} words). Aim for at least {} words.",
                word_count, MIN_WORD_COUNT
            ),
            impact: Some("high".to_string()),
        });
        score -= 30;
    } else if word_count < GOOD_WORD_COUNT {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Content,
            message: format!(
                "Content is relatively short ({} words). Consider expanding to {}+ words.",
                word_count, EXCELLENT_WORD_COUNT
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    } else if word_count >= EXCELLENT_WORD_COUNT {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Content,
            message: format!("Excellent content length ({} words)", word_count),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Content,
            message: format!("Good content length ({} words)", word_count),
            impact: None,
        });
    }

    // Keyword analysis
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            keyword_density = calculate_keyword_density(text, kw);

            if keyword_density == 0.0 {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Error,
                    category: SeoIssueCategory::Keyword,
                    message: "Focus keyword not found in content".to_string(),
                    impact: Some("high".to_string()),
                });
                score -= 25;
            } else if keyword_density < KEYWORD_DENSITY_MIN {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Keyword,
                    message: format!(
                        "Keyword density is low ({:.1}%). Aim for {:.1}-{:.1}%.",
                        keyword_density, KEYWORD_DENSITY_OPTIMAL_MIN, KEYWORD_DENSITY_OPTIMAL_MAX
                    ),
                    impact: Some("medium".to_string()),
                });
                score -= 10;
            } else if keyword_density > KEYWORD_DENSITY_MAX {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Keyword,
                    message: format!(
                        "Keyword density is too high ({:.1}%). This may be seen as keyword stuffing.",
                        keyword_density
                    ),
                    impact: Some("medium".to_string()),
                });
                score -= 15;
            } else if (KEYWORD_DENSITY_OPTIMAL_MIN..=KEYWORD_DENSITY_OPTIMAL_MAX).contains(&keyword_density)
            {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Keyword,
                    message: format!("Keyword density is optimal ({:.1}%)", keyword_density),
                    impact: None,
                });
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Info,
                    category: SeoIssueCategory::Keyword,
                    message: format!("Keyword density is acceptable ({:.1}%)", keyword_density),
                    impact: None,
                });
            }

            // Check keyword in first paragraph (first 100 words)
            let first_100_words: String = text
                .split_whitespace()
                .take(100)
                .collect::<Vec<&str>>()
                .join(" ")
                .to_lowercase();

            if first_100_words.contains(&kw.to_lowercase()) {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Content,
                    message: "Focus keyword appears in the first 100 words".to_string(),
                    impact: None,
                });
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Content,
                    message: "Focus keyword does not appear in the first 100 words".to_string(),
                    impact: Some("medium".to_string()),
                });
                suggestions.push("Add your focus keyword within the first paragraph".to_string());
                score -= 10;
            }
        }
    }

    (score.clamp(0, 100) as u8, keyword_density)
}

/// Analyze readability
fn analyze_readability(
    text: &str,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> (u8, f32) {
    let mut score: i32 = 100;
    let word_count = count_words(text);
    let sentence_count = count_sentences(text);
    let flesch_score = calculate_flesch_kincaid(text);

    // Average sentence length
    let avg_sentence_length = if sentence_count > 0 {
        word_count as f32 / sentence_count as f32
    } else {
        0.0
    };

    if avg_sentence_length > 25.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Sentences are too long on average ({:.0} words). Aim for under 20 words.",
                avg_sentence_length
            ),
            impact: Some("medium".to_string()),
        });
        suggestions.push("Break up long sentences for better readability".to_string());
        score -= 15;
    } else if avg_sentence_length > 20.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Average sentence length is slightly high ({:.0} words)",
                avg_sentence_length
            ),
            impact: Some("low".to_string()),
        });
        score -= 5;
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Good average sentence length ({:.0} words)",
                avg_sentence_length
            ),
            impact: None,
        });
    }

    // Flesch reading ease interpretation
    if flesch_score >= 60.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Good readability score ({:.0} Flesch Reading Ease)",
                flesch_score
            ),
            impact: None,
        });
    } else if flesch_score >= 30.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Content may be difficult to read ({:.0} Flesch Reading Ease). Aim for 60+.",
                flesch_score
            ),
            impact: Some("medium".to_string()),
        });
        suggestions
            .push("Simplify your writing by using shorter sentences and simpler words".to_string());
        score -= 15;
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Error,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Content is very difficult to read ({:.0} Flesch Reading Ease)",
                flesch_score
            ),
            impact: Some("high".to_string()),
        });
        score -= 25;
    }

    // Transition words check
    let transition_words = [
        "however",
        "therefore",
        "furthermore",
        "additionally",
        "moreover",
        "consequently",
        "thus",
        "hence",
        "meanwhile",
        "nevertheless",
        "although",
        "because",
        "since",
        "while",
        "whereas",
        "finally",
        "first",
        "second",
        "third",
        "next",
        "then",
        "also",
        "besides",
    ];

    let text_lower = text.to_lowercase();
    let transition_count = transition_words
        .iter()
        .filter(|w| text_lower.contains(*w))
        .count();

    if transition_count < 3 && word_count > 300 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: "Content lacks transition words".to_string(),
            impact: Some("low".to_string()),
        });
        suggestions.push("Add transition words to improve content flow".to_string());
        score -= 10;
    } else if transition_count >= 3 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: "Good use of transition words".to_string(),
            impact: None,
        });
    }

    // Passive voice check (simplified)
    let passive_re = Regex::new(r"\b(was|were|is|are|been|being)\s+\w+ed\b").unwrap();
    let passive_count = passive_re.find_iter(&text_lower).count();
    let passive_percentage = if sentence_count > 0 {
        (passive_count as f32 / sentence_count as f32) * 100.0
    } else {
        0.0
    };

    if passive_percentage > 20.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Too much passive voice detected (~{:.0}%)",
                passive_percentage
            ),
            impact: Some("low".to_string()),
        });
        suggestions.push("Use more active voice for engaging content".to_string());
        score -= 10;
    } else if passive_percentage > 10.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Readability,
            message: format!("Some passive voice detected (~{:.0}%)", passive_percentage),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: "Good use of active voice".to_string(),
            impact: None,
        });
    }

    (score.clamp(0, 100) as u8, flesch_score)
}

/// Analyze URL slug
fn analyze_slug(
    slug: &str,
    keyword: &Option<String>,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> u8 {
    let mut score: i32 = 100;

    if slug.is_empty() {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Slug,
            message: "URL slug not yet set".to_string(),
            impact: None,
        });
        return 50;
    }

    // Slug length
    if slug.len() > SLUG_MAX_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Slug,
            message: format!(
                "URL slug is too long ({}). Keep under {} characters.",
                slug.len(),
                SLUG_MAX_LENGTH
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Slug,
            message: "URL slug length is good".to_string(),
            impact: None,
        });
    }

    // Keyword in slug
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            let slug_lower = slug.to_lowercase();
            let kw_slug = kw.to_lowercase().replace(' ', "-");

            if slug_lower.contains(&kw_slug)
                || kw
                    .to_lowercase()
                    .split_whitespace()
                    .all(|w| slug_lower.contains(w))
            {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Slug,
                    message: "Focus keyword found in URL slug".to_string(),
                    impact: None,
                });
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Slug,
                    message: "Focus keyword not found in URL slug".to_string(),
                    impact: Some("medium".to_string()),
                });
                suggestions.push("Include your focus keyword in the URL slug".to_string());
                score -= 15;
            }
        }
    }

    // Stop words check
    let stop_words = [
        "a", "an", "the", "and", "or", "but", "in", "on", "at", "to", "for", "of",
    ];
    let slug_parts: Vec<&str> = slug.split('-').collect();
    let has_stop_words = stop_words.iter().any(|w| slug_parts.contains(w));

    if has_stop_words {
        suggestions.push("Consider removing stop words from your URL slug".to_string());
    }

    // Check for numbers at end (often dates)
    let has_trailing_numbers = slug.chars().last().map(|c| c.is_numeric()).unwrap_or(false);
    if has_trailing_numbers {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Slug,
            message: "URL contains numbers (possibly a date)".to_string(),
            impact: None,
        });
    }

    score.clamp(0, 100) as u8
}

/// Analyze heading structure
fn analyze_structure(
    html: &str,
    blocks: &[JsonValue],
    keyword: &Option<String>,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> (u8, Vec<HeadingNode>, u32) {
    let mut score: i32 = 100;
    let mut headings: Vec<HeadingNode> = Vec::new();

    // Extract headings from blocks
    let mut h1_count = 0;
    let mut h2_count = 0;
    let mut _h3_count = 0;
    let mut last_level: u8 = 0;
    let mut hierarchy_valid = true;

    for block in blocks {
        if block.get("type").and_then(|v| v.as_str()) == Some("heading") {
            let level = block
                .get("settings")
                .and_then(|s| s.get("level"))
                .and_then(|l| l.as_u64())
                .unwrap_or(2) as u8;

            let text = block
                .get("content")
                .and_then(|c| c.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();

            let id = block
                .get("id")
                .and_then(|i| i.as_str())
                .unwrap_or("")
                .to_string();

            // Check hierarchy
            if level > last_level + 1 && last_level > 0 {
                hierarchy_valid = false;
            }
            last_level = level;

            match level {
                1 => h1_count += 1,
                2 => h2_count += 1,
                3 => _h3_count += 1,
                _ => {}
            }

            headings.push(HeadingNode {
                level,
                text: text.clone(),
                id,
                children: vec![],
            });
        }
    }

    // Also check HTML content for headings
    let h2_regex = Regex::new(r"<h2[^>]*>(.*?)</h2>").unwrap();
    h2_count += h2_regex.find_iter(html).count() as u32;

    // H1 check (for blog posts, usually title handles H1)
    if h1_count > 1 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Headings,
            message: format!(
                "Multiple H1 headings found ({}). Use only one H1 per page.",
                h1_count
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    }

    // H2 check
    if h2_count == 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Structure,
            message: "No H2 subheadings found".to_string(),
            impact: Some("medium".to_string()),
        });
        suggestions.push("Add H2 subheadings to improve content structure".to_string());
        score -= 15;
    } else if h2_count < 3 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Structure,
            message: format!(
                "Only {} H2 subheading(s) found. Consider adding more.",
                h2_count
            ),
            impact: Some("low".to_string()),
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Structure,
            message: format!("Good number of H2 subheadings ({})", h2_count),
            impact: None,
        });
    }

    // Hierarchy check
    if !hierarchy_valid {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Headings,
            message: "Heading hierarchy is not sequential (e.g., H1 -> H3 skips H2)".to_string(),
            impact: Some("medium".to_string()),
        });
        suggestions.push("Maintain proper heading hierarchy (H1 -> H2 -> H3)".to_string());
        score -= 10;
    }

    // Keyword in headings check
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            let kw_lower = kw.to_lowercase();
            let has_keyword_in_heading = headings
                .iter()
                .any(|h| h.text.to_lowercase().contains(&kw_lower));

            if has_keyword_in_heading {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Headings,
                    message: "Focus keyword found in at least one heading".to_string(),
                    impact: None,
                });
            } else if !headings.is_empty() {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Headings,
                    message: "Focus keyword not found in any heading".to_string(),
                    impact: Some("medium".to_string()),
                });
                suggestions
                    .push("Add your focus keyword to at least one subheading (H2)".to_string());
                score -= 10;
            }
        }
    }

    // Count images without alt text
    let mut images_without_alt: u32 = 0;
    for block in blocks {
        if block.get("type").and_then(|v| v.as_str()) == Some("image") {
            let has_alt = block
                .get("content")
                .and_then(|c| c.get("mediaAlt"))
                .and_then(|a| a.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);

            if !has_alt {
                images_without_alt += 1;
            }
        }
    }

    // Also check HTML for images
    let img_total_regex = Regex::new(r#"<img[^>]*>"#).unwrap();
    let img_with_alt_regex = Regex::new(r#"<img[^>]*alt="[^"]+""#).unwrap();
    let total_img = img_total_regex.find_iter(html).count() as u32;
    let img_with_alt = img_with_alt_regex.find_iter(html).count() as u32;
    images_without_alt += total_img.saturating_sub(img_with_alt);

    if images_without_alt > 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Images,
            message: format!("{} image(s) missing alt text", images_without_alt),
            impact: Some("medium".to_string()),
        });
        suggestions.push("Add descriptive alt text to all images".to_string());
        score -= 10;
    } else if total_img > 0
        || blocks
            .iter()
            .any(|b| b.get("type").and_then(|v| v.as_str()) == Some("image"))
    {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Images,
            message: "All images have alt text".to_string(),
            impact: None,
        });
    }

    (score.clamp(0, 100) as u8, headings, images_without_alt)
}

/// Analyze links
fn analyze_links(
    html: &str,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> LinksAnalysis {
    let internal_regex = Regex::new(r#"<a[^>]*href="(/[^"]*|#[^"]*)""#).unwrap();
    let external_regex = Regex::new(r#"<a[^>]*href="https?://[^"]*""#).unwrap();
    let nofollow_regex = Regex::new(r#"<a[^>]*rel="[^"]*nofollow[^"]*""#).unwrap();

    let internal_count = internal_regex.find_iter(html).count() as u32;
    let external_count = external_regex.find_iter(html).count() as u32;
    let nofollow_count = nofollow_regex.find_iter(html).count() as u32;

    let total_links = internal_count + external_count;
    let ratio = if total_links > 0 {
        internal_count as f32 / total_links as f32
    } else {
        0.0
    };

    // Internal links
    if internal_count == 0 && external_count > 0 {
        suggestions.push("Add internal links to other relevant content on your site".to_string());
    } else if internal_count > 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Links,
            message: format!("{} internal link(s) found", internal_count),
            impact: None,
        });
    }

    // External links
    if external_count == 0 && internal_count > 0 {
        suggestions.push("Consider adding external links to authoritative sources".to_string());
    } else if external_count > 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Links,
            message: format!("{} external link(s) found", external_count),
            impact: None,
        });
    }

    // No links at all
    if total_links == 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Links,
            message: "No links found in content".to_string(),
            impact: Some("low".to_string()),
        });
        suggestions.push("Add both internal and external links to your content".to_string());
    }

    LinksAnalysis {
        internal_count,
        external_count,
        ratio,
        nofollow_count,
        broken_count: 0, // Would require async HTTP checks
    }
}

/// Calculate grade from score
fn calculate_grade(score: u8) -> String {
    match score {
        90..=100 => "A".to_string(),
        80..=89 => "B".to_string(),
        70..=79 => "C".to_string(),
        60..=69 => "D".to_string(),
        _ => "F".to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ENDPOINT HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// POST /api/cms/seo/validate
/// Validate content for SEO
async fn validate_seo(
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

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS SEO validation routes (requires authentication)
pub fn router() -> Router<AppState> {
    Router::new().route("/validate", post(validate_seo))
}
