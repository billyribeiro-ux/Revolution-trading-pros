//! Text extraction and analysis helpers.
//!
//! Pure functions: take JSON blocks / strings, return derived strings or
//! scalar counts. No I/O, no state. Used by both the analyzers and the
//! top-level handler to produce the plain-text + HTML views of the
//! content.

use serde_json::Value as JsonValue;
use std::collections::HashSet;

use super::regex_patterns::{HTML_TAG_RE, SENTENCE_END_RE, WHITESPACE_RE};

/// Extract plain text from content blocks
pub(super) fn extract_text_from_blocks(blocks: &[JsonValue]) -> String {
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
pub(super) fn extract_html_from_blocks(blocks: &[JsonValue]) -> String {
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
pub(super) fn strip_html(html: &str) -> String {
    let text = HTML_TAG_RE.replace_all(html, " ");
    // Normalize whitespace
    WHITESPACE_RE.replace_all(&text, " ").trim().to_string()
}

/// Count words in text
pub(super) fn count_words(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

/// Count sentences in text
pub(super) fn count_sentences(text: &str) -> u32 {
    let count = SENTENCE_END_RE.find_iter(text).count();
    if count == 0 {
        1
    } else {
        count as u32
    }
}

/// Count syllables in a word (approximation)
pub(super) fn count_syllables(word: &str) -> u32 {
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
pub(super) fn calculate_flesch_kincaid(text: &str) -> f32 {
    let words = count_words(text);
    let sentences = count_sentences(text);
    let syllables: u32 = text.split_whitespace().map(count_syllables).sum();

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
pub(super) fn calculate_keyword_density(text: &str, keyword: &str) -> f32 {
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
