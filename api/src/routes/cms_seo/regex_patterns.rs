//! Compiled regex patterns for SEO analysis (hoisted to avoid per-request
//! recompilation).
//!
//! House style: this repo uses `lazy_static!` (see `api/src/routes/sitemap.rs`
//! and `robots.rs`). The `.expect("static regex must compile")` runs once at
//! startup, so a panic there is a build-time bug, not a per-request hazard.

use regex::Regex;

lazy_static::lazy_static! {
    /// Strip HTML tags (e.g. `<p>`, `<a href="...">`).
    pub(super) static ref HTML_TAG_RE: Regex =
        Regex::new(r"<[^>]*>").expect("static regex must compile");

    /// Collapse runs of whitespace to a single space.
    pub(super) static ref WHITESPACE_RE: Regex =
        Regex::new(r"\s+").expect("static regex must compile");

    /// Sentence-ending punctuation.
    pub(super) static ref SENTENCE_END_RE: Regex =
        Regex::new(r"[.!?]+").expect("static regex must compile");

    /// Naive passive-voice detector (e.g. "was written", "is loved").
    pub(super) static ref PASSIVE_VOICE_RE: Regex =
        Regex::new(r"\b(was|were|is|are|been|being)\s+\w+ed\b")
            .expect("static regex must compile");

    /// Match `<h2 ...>...</h2>` blocks.
    pub(super) static ref H2_RE: Regex =
        Regex::new(r"<h2[^>]*>(.*?)</h2>").expect("static regex must compile");

    /// Match all `<img ...>` tags.
    pub(super) static ref IMG_TOTAL_RE: Regex =
        Regex::new(r"<img[^>]*>").expect("static regex must compile");

    /// Match `<img>` tags that have a non-empty `alt="..."`.
    pub(super) static ref IMG_WITH_ALT_RE: Regex =
        Regex::new(r#"<img[^>]*alt="[^"]+""#).expect("static regex must compile");

    /// Match anchors with internal hrefs (`/...` or `#...`).
    pub(super) static ref INTERNAL_LINK_RE: Regex =
        Regex::new(r#"<a[^>]*href="(/[^"]*|#[^"]*)""#)
            .expect("static regex must compile");

    /// Match anchors with external `http(s)://` hrefs.
    pub(super) static ref EXTERNAL_LINK_RE: Regex =
        Regex::new(r#"<a[^>]*href="https?://[^"]*""#)
            .expect("static regex must compile");

    /// Match anchors carrying a `rel="...nofollow..."`.
    pub(super) static ref NOFOLLOW_LINK_RE: Regex =
        Regex::new(r#"<a[^>]*rel="[^"]*nofollow[^"]*""#)
            .expect("static regex must compile");
}
