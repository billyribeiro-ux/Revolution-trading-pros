//! Per-category SEO analyzers.
//!
//! One function per analysis category — title, meta description, content,
//! readability, slug, structure (headings + images), links — plus the
//! shared `calculate_grade` helper.
//!
//! Each analyzer takes the relevant inputs plus `&mut Vec<SeoIssue>` and
//! `&mut Vec<String>` (suggestions) collectors and returns the per-category
//! score (and, where relevant, a derived metric like `keyword_density` or
//! the heading tree). The top-level handler in `handlers.rs` orchestrates
//! the calls and folds the scores into the overall response.

use serde_json::Value as JsonValue;

use super::regex_patterns::{
    EXTERNAL_LINK_RE, H2_RE, IMG_TOTAL_RE, IMG_WITH_ALT_RE, INTERNAL_LINK_RE, NOFOLLOW_LINK_RE,
    PASSIVE_VOICE_RE,
};
use super::text::{
    calculate_flesch_kincaid, calculate_keyword_density, count_sentences, count_words,
};
use super::types::{HeadingNode, LinksAnalysis, SeoIssue, SeoIssueCategory, SeoIssueSeverity};
use super::{
    EXCELLENT_WORD_COUNT, GOOD_WORD_COUNT, KEYWORD_DENSITY_MAX, KEYWORD_DENSITY_MIN,
    KEYWORD_DENSITY_OPTIMAL_MAX, KEYWORD_DENSITY_OPTIMAL_MIN, META_MAX_LENGTH, META_MIN_LENGTH,
    META_OPTIMAL_MAX, META_OPTIMAL_MIN, MIN_WORD_COUNT, SLUG_MAX_LENGTH, TITLE_MAX_LENGTH,
    TITLE_MIN_LENGTH, TITLE_OPTIMAL_MAX, TITLE_OPTIMAL_MIN,
};

/// Analyze title SEO
pub(super) fn analyze_title(
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
                "Title is too short ({title_len} chars). Aim for {TITLE_OPTIMAL_MIN}-{TITLE_OPTIMAL_MAX} characters."
            ),
            impact: Some("medium".to_string()),
        });
        score -= 20;
    } else if title_len > TITLE_MAX_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Title,
            message: format!(
                "Title is too long ({title_len} chars). Keep under {TITLE_OPTIMAL_MAX} characters."
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    } else if (TITLE_OPTIMAL_MIN..=TITLE_OPTIMAL_MAX).contains(&title_len) {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Title,
            message: format!("Title length is optimal ({title_len} chars)"),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Title,
            message: format!("Title length is acceptable ({title_len} chars)"),
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
pub(super) fn analyze_meta_description(
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
                "Meta description is too short ({meta_len} chars). Aim for {META_OPTIMAL_MIN}-{META_OPTIMAL_MAX} characters."
            ),
            impact: Some("medium".to_string()),
        });
        score -= 25;
    } else if meta_len > META_MAX_LENGTH {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Meta,
            message: format!(
                "Meta description is too long ({meta_len} chars). Keep under {META_OPTIMAL_MAX} characters."
            ),
            impact: Some("low".to_string()),
        });
        score -= 10;
    } else if (META_OPTIMAL_MIN..=META_OPTIMAL_MAX).contains(&meta_len) {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Meta,
            message: format!("Meta description length is optimal ({meta_len} chars)"),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Meta,
            message: format!("Meta description length is acceptable ({meta_len} chars)"),
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
pub(super) fn analyze_content(
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
                "Content is too short ({word_count} words). Aim for at least {MIN_WORD_COUNT} words."
            ),
            impact: Some("high".to_string()),
        });
        score -= 30;
    } else if word_count < GOOD_WORD_COUNT {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Content,
            message: format!(
                "Content is relatively short ({word_count} words). Consider expanding to {EXCELLENT_WORD_COUNT}+ words."
            ),
            impact: Some("medium".to_string()),
        });
        score -= 15;
    } else if word_count >= EXCELLENT_WORD_COUNT {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Content,
            message: format!("Excellent content length ({word_count} words)"),
            impact: None,
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Content,
            message: format!("Good content length ({word_count} words)"),
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
                        "Keyword density is low ({keyword_density:.1}%). Aim for {KEYWORD_DENSITY_OPTIMAL_MIN:.1}-{KEYWORD_DENSITY_OPTIMAL_MAX:.1}%."
                    ),
                    impact: Some("medium".to_string()),
                });
                score -= 10;
            } else if keyword_density > KEYWORD_DENSITY_MAX {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Warning,
                    category: SeoIssueCategory::Keyword,
                    message: format!(
                        "Keyword density is too high ({keyword_density:.1}%). This may be seen as keyword stuffing."
                    ),
                    impact: Some("medium".to_string()),
                });
                score -= 15;
            } else if (KEYWORD_DENSITY_OPTIMAL_MIN..=KEYWORD_DENSITY_OPTIMAL_MAX)
                .contains(&keyword_density)
            {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Success,
                    category: SeoIssueCategory::Keyword,
                    message: format!("Keyword density is optimal ({keyword_density:.1}%)"),
                    impact: None,
                });
            } else {
                issues.push(SeoIssue {
                    severity: SeoIssueSeverity::Info,
                    category: SeoIssueCategory::Keyword,
                    message: format!("Keyword density is acceptable ({keyword_density:.1}%)"),
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
pub(super) fn analyze_readability(
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
                "Sentences are too long on average ({avg_sentence_length:.0} words). Aim for under 20 words."
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
                "Average sentence length is slightly high ({avg_sentence_length:.0} words)"
            ),
            impact: Some("low".to_string()),
        });
        score -= 5;
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: format!("Good average sentence length ({avg_sentence_length:.0} words)"),
            impact: None,
        });
    }

    // Flesch reading ease interpretation
    if flesch_score >= 60.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Readability,
            message: format!("Good readability score ({flesch_score:.0} Flesch Reading Ease)"),
            impact: None,
        });
    } else if flesch_score >= 30.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: format!(
                "Content may be difficult to read ({flesch_score:.0} Flesch Reading Ease). Aim for 60+."
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
                "Content is very difficult to read ({flesch_score:.0} Flesch Reading Ease)"
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
    let passive_count = PASSIVE_VOICE_RE.find_iter(&text_lower).count();
    let passive_percentage = if sentence_count > 0 {
        (passive_count as f32 / sentence_count as f32) * 100.0
    } else {
        0.0
    };

    if passive_percentage > 20.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Readability,
            message: format!("Too much passive voice detected (~{passive_percentage:.0}%)"),
            impact: Some("low".to_string()),
        });
        suggestions.push("Use more active voice for engaging content".to_string());
        score -= 10;
    } else if passive_percentage > 10.0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Info,
            category: SeoIssueCategory::Readability,
            message: format!("Some passive voice detected (~{passive_percentage:.0}%)"),
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
pub(super) fn analyze_slug(
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
pub(super) fn analyze_structure(
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
    h2_count += H2_RE.find_iter(html).count() as u32;

    // H1 check (for blog posts, usually title handles H1)
    if h1_count > 1 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Headings,
            message: format!("Multiple H1 headings found ({h1_count}). Use only one H1 per page."),
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
            message: format!("Only {h2_count} H2 subheading(s) found. Consider adding more."),
            impact: Some("low".to_string()),
        });
    } else {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Success,
            category: SeoIssueCategory::Structure,
            message: format!("Good number of H2 subheadings ({h2_count})"),
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
    let total_img = IMG_TOTAL_RE.find_iter(html).count() as u32;
    let img_with_alt = IMG_WITH_ALT_RE.find_iter(html).count() as u32;
    images_without_alt += total_img.saturating_sub(img_with_alt);

    if images_without_alt > 0 {
        issues.push(SeoIssue {
            severity: SeoIssueSeverity::Warning,
            category: SeoIssueCategory::Images,
            message: format!("{images_without_alt} image(s) missing alt text"),
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
pub(super) fn analyze_links(
    html: &str,
    issues: &mut Vec<SeoIssue>,
    suggestions: &mut Vec<String>,
) -> LinksAnalysis {
    let internal_count = INTERNAL_LINK_RE.find_iter(html).count() as u32;
    let external_count = EXTERNAL_LINK_RE.find_iter(html).count() as u32;
    let nofollow_count = NOFOLLOW_LINK_RE.find_iter(html).count() as u32;

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
            message: format!("{internal_count} internal link(s) found"),
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
            message: format!("{external_count} external link(s) found"),
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
pub(super) fn calculate_grade(score: u8) -> String {
    match score {
        90..=100 => "A".to_string(),
        80..=89 => "B".to_string(),
        70..=79 => "C".to_string(),
        60..=69 => "D".to_string(),
        _ => "F".to_string(),
    }
}
