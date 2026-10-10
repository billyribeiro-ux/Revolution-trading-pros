//! Compiled regex patterns for SEO analysis (hoisted to avoid per-request
//! recompilation).
//!
//! Lazily compiled with `std::sync::LazyLock` (stable since Rust 1.80), which
//! replaced the `lazy_static!` crate repo-wide on 2026-10-09 — upstream marks
//! `lazy_static` as superseded by `LazyLock` and no longer updated. The
//! `.expect("static regex must compile")` runs once, on first use, so a panic
//! there is a build-time bug, not a per-request hazard.

use std::sync::LazyLock;

use regex::Regex;

/// Strip HTML tags (e.g. `<p>`, `<a href="...">`).
pub(super) static HTML_TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<[^>]*>").expect("static regex must compile"));

/// Collapse runs of whitespace to a single space.
pub(super) static WHITESPACE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("static regex must compile"));

/// Sentence-ending punctuation.
pub(super) static SENTENCE_END_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[.!?]+").expect("static regex must compile"));

/// Naive passive-voice detector (e.g. "was written", "is loved").
pub(super) static PASSIVE_VOICE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(was|were|is|are|been|being)\s+\w+ed\b").expect("static regex must compile")
});

/// Match `<h2 ...>...</h2>` blocks.
pub(super) static H2_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<h2[^>]*>(.*?)</h2>").expect("static regex must compile"));

/// Match all `<img ...>` tags.
pub(super) static IMG_TOTAL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<img[^>]*>").expect("static regex must compile"));

/// Match `<img>` tags that have a non-empty `alt="..."`.
pub(super) static IMG_WITH_ALT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"<img[^>]*alt="[^"]+""#).expect("static regex must compile"));

/// Match anchors with internal hrefs (`/...` or `#...`).
pub(super) static INTERNAL_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<a[^>]*href="(/[^"]*|#[^"]*)""#).expect("static regex must compile")
});

/// Match anchors with external `http(s)://` hrefs.
pub(super) static EXTERNAL_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<a[^>]*href="https?://[^"]*""#).expect("static regex must compile")
});

/// Match anchors carrying a `rel="...nofollow..."`.
pub(super) static NOFOLLOW_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<a[^>]*rel="[^"]*nofollow[^"]*""#).expect("static regex must compile")
});
