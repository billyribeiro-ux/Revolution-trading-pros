//! HTML sanitization at the public read boundary.
//!
//! Admin- and editor-authored HTML (blog content blocks, popups, form copy) is
//! stored as written and served to anonymous visitors. Everything that leaves
//! through a public endpoint passes through here first, so the API never hands
//! a browser script — independent of how, or whether, a client sanitizes it
//! again before rendering (the SvelteKit frontend does, as defence in depth).
//!
//! Sanitizing on read rather than on write keeps the stored content exactly as
//! its author wrote it, covers every row already in the database, and lets the
//! policy evolve without a data migration. `ammonia` (html5ever, the WHATWG
//! parser Servo uses) parses, allowlists and re-serializes the markup.
//!
//! The policy mirrors the frontend's `rich` profile
//! (`frontend/src/lib/utils/html-sanitizer/policy.ts`): the most permissive
//! profile any of these fields is rendered with. Clients may apply a stricter
//! one for display.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use ammonia::{url::ParseError, Builder, Url, UrlRelative};
use serde_json::{Map, Value};

/// Elements kept; anything else is unwrapped (its children survive).
const ALLOWED_TAGS: &[&str] = &[
    "a",
    "audio",
    "b",
    "blockquote",
    "br",
    "caption",
    "code",
    "col",
    "colgroup",
    "div",
    "em",
    "figcaption",
    "figure",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "i",
    "iframe",
    "img",
    "li",
    "ol",
    "p",
    "pre",
    "s",
    "source",
    "span",
    "strong",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "u",
    "ul",
    "video",
];

/// Elements removed together with their content: script and style bodies,
/// foreign (SVG/MathML) content, raw-text and template-like elements, plugin
/// fallback content, and `select` (whose content model browsers are changing).
const DROP_WITH_CONTENT: &[&str] = &[
    "applet",
    "embed",
    "head",
    "math",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "plaintext",
    "script",
    "select",
    "style",
    "svg",
    "template",
    "title",
    "xmp",
];

/// Attributes allowed on every kept element. `rel` is not listed: every link
/// gets `rel="noopener noreferrer"` instead.
const ALLOWED_ATTRIBUTES: &[&str] = &[
    "allow",
    "allowfullscreen",
    "alt",
    "autoplay",
    "class",
    "controls",
    "frameborder",
    "height",
    "href",
    "id",
    "loading",
    "loop",
    "muted",
    "poster",
    "src",
    "target",
    "title",
    "width",
];

/// Schemes a URL attribute (`href`, `src`, `poster`) may use. Relative URLs
/// pass through. `data:` is admitted here and narrowed to media sources by
/// [`filter_attribute`].
const URL_SCHEMES: &[&str] = &["http", "https", "mailto", "tel", "data"];

/// Schemes a bare URL field (a block's `url`, a CTA link) may use.
const LINK_SCHEMES: &[&str] = &["http", "https", "mailto", "tel"];

/// Hosts an `<iframe src>` may embed, over https only.
const TRUSTED_EMBED_HOSTS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "youtube-nocookie.com",
    "www.youtube-nocookie.com",
    "player.vimeo.com",
    "vimeo.com",
];

/// Resolves scheme-relative (`//host/x`) URLs to https and plain relative
/// paths to a host that is never trusted.
static RELATIVE_BASE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://relative.invalid/").expect("static base URL must parse"));

static SANITIZER: LazyLock<Builder<'static>> = LazyLock::new(|| {
    let mut builder = Builder::empty();
    builder
        .tags(ALLOWED_TAGS.iter().copied().collect())
        .clean_content_tags(DROP_WITH_CONTENT.iter().copied().collect())
        .tag_attributes(HashMap::new())
        .tag_attribute_values(HashMap::new())
        .generic_attributes(ALLOWED_ATTRIBUTES.iter().copied().collect())
        .generic_attribute_prefixes(HashSet::from(["data-", "aria-"]))
        .url_schemes(URL_SCHEMES.iter().copied().collect())
        .url_relative(UrlRelative::PassThrough)
        .link_rel(Some("noopener noreferrer"))
        .strip_comments(true)
        .attribute_filter(filter_attribute);
    builder
});

/// Second pass over each surviving attribute, after ammonia's scheme check.
fn filter_attribute<'u>(element: &str, attribute: &str, value: &'u str) -> Option<Cow<'u, str>> {
    let allowed = match (element, attribute) {
        ("iframe", "src") => is_trusted_embed(value),
        (_, "href" | "src" | "poster") => match data_url_media_type(value) {
            Some(media_type) => attribute == "src" && is_media_for(element, &media_type),
            None => true,
        },
        _ => true,
    };
    allowed.then_some(Cow::Borrowed(value))
}

/// The lowercased media type of a `data:` URL, or `None` for any other URL.
fn data_url_media_type(value: &str) -> Option<String> {
    let url = Url::parse(value).ok()?;
    (url.scheme() == "data").then(|| url.path().to_ascii_lowercase())
}

/// `data:` sources are allowed only for media of the element's own kind.
fn is_media_for(element: &str, media_type: &str) -> bool {
    let prefixes: &[&str] = match element {
        "img" => &["image/"],
        "video" => &["video/"],
        "audio" => &["audio/"],
        "source" => &["image/", "video/", "audio/"],
        _ => &[],
    };
    prefixes.iter().any(|prefix| media_type.starts_with(prefix))
}

fn is_trusted_embed(src: &str) -> bool {
    Url::options()
        .base_url(Some(&RELATIVE_BASE))
        .parse(src)
        .is_ok_and(|url| {
            url.scheme() == "https"
                && url
                    .host_str()
                    .is_some_and(|host| TRUSTED_EMBED_HOSTS.contains(&host))
        })
}

/// Sanitizes an HTML fragment for rendering in a browser.
pub fn sanitize_html(input: &str) -> String {
    SANITIZER.clean(input).to_string()
}

/// Whether a bare URL is safe to place in an `href`/`src`: relative, or
/// http(s)/mailto/tel. Parsing follows the WHATWG URL standard, so
/// `java\tscript:` and ` JAVASCRIPT:` are recognized for what they are.
pub fn is_safe_url(value: &str) -> bool {
    match Url::parse(value) {
        Ok(url) => LINK_SCHEMES.contains(&url.scheme()),
        Err(ParseError::RelativeUrlWithoutBase) => true,
        Err(_) => false,
    }
}

// ── Structured content ──────────────────────────────────────────────────

/// Block types whose `text` the blog renders as HTML. Other types (code,
/// button, …) render `text` as plain text, which escaping would corrupt.
const HTML_TEXT_BLOCKS: &[&str] = &["paragraph", "heading", "quote", "pullquote", "callout"];

/// URL-valued keys in a block's data or settings.
const URL_KEYS: &[&str] = &[
    "url",
    "href",
    "mediaUrl",
    "embedUrl",
    "linkUrl",
    "backgroundImage",
];

/// Sanitizes post content blocks in place: an array of blocks, or an object
/// holding one under `blocks`. Accepts both block shapes the editor has
/// written — `{type, data: {…}}` and `{type, content: {…}, settings: {…}}` —
/// and recurses into nested blocks (columns, groups).
pub fn sanitize_content_blocks(blocks: &mut Value) {
    match blocks {
        Value::Array(items) => items.iter_mut().for_each(sanitize_block),
        Value::Object(object) => {
            if let Some(nested) = object.get_mut("blocks") {
                sanitize_content_blocks(nested);
            }
        }
        _ => {}
    }
}

fn sanitize_block(block: &mut Value) {
    let Some(block) = block.as_object_mut() else {
        return;
    };
    let block_type = block
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    for key in ["data", "content"] {
        if let Some(Value::Object(data)) = block.get_mut(key) {
            sanitize_block_data(&block_type, data);
        }
    }
    if let Some(Value::Object(settings)) = block.get_mut("settings") {
        drop_unsafe_urls(settings);
    }
}

fn sanitize_block_data(block_type: &str, data: &mut Map<String, Value>) {
    if HTML_TEXT_BLOCKS.contains(&block_type) {
        sanitize_html_field(data, "text");
    }
    if block_type == "callout" {
        sanitize_html_field(data, "content");
    }
    // `html` holds markup by definition, whatever the block type.
    sanitize_html_field(data, "html");

    match block_type {
        "list" => sanitize_list_items(data.get_mut("items")),
        "checklist" => {
            for key in ["checklistItems", "items"] {
                if let Some(Value::Array(items)) = data.get_mut(key) {
                    for item in items.iter_mut().filter_map(Value::as_object_mut) {
                        sanitize_html_field(item, "text");
                    }
                }
            }
        }
        _ => {}
    }

    drop_unsafe_urls(data);

    for key in ["blocks", "children"] {
        if let Some(nested) = data.get_mut(key) {
            sanitize_content_blocks(nested);
        }
    }
    if let Some(Value::Array(columns)) = data.get_mut("columns") {
        for column in columns.iter_mut().filter_map(Value::as_object_mut) {
            if let Some(nested) = column.get_mut("blocks") {
                sanitize_content_blocks(nested);
            }
        }
    }
}

/// List items are HTML strings (legacy shape) or `{content|text, children}`
/// objects (editor shape).
fn sanitize_list_items(items: Option<&mut Value>) {
    let Some(Value::Array(items)) = items else {
        return;
    };
    for item in items {
        match item {
            Value::String(html) => *html = sanitize_html(html),
            Value::Object(object) => {
                sanitize_html_field(object, "content");
                sanitize_html_field(object, "text");
                sanitize_list_items(object.get_mut("children"));
            }
            _ => {}
        }
    }
}

fn sanitize_html_field(object: &mut Map<String, Value>, key: &str) {
    if let Some(Value::String(html)) = object.get_mut(key) {
        *html = sanitize_html(html);
    }
}

fn drop_unsafe_urls(object: &mut Map<String, Value>) {
    for key in URL_KEYS {
        if object
            .get(*key)
            .and_then(Value::as_str)
            .is_some_and(|url| !is_safe_url(url))
        {
            object.remove(*key);
        }
    }
}

/// Sanitizes a form's field definitions in place: the markup of `html`
/// fields (carried in `placeholder`), consent copy, and policy links.
pub fn sanitize_form_fields(fields: &mut Value) {
    let Value::Array(fields) = fields else {
        return;
    };
    for field in fields.iter_mut().filter_map(Value::as_object_mut) {
        if field.get("field_type").and_then(Value::as_str) == Some("html") {
            sanitize_html_field(field, "placeholder");
        }
        if let Some(Value::Object(attributes)) = field.get_mut("attributes") {
            sanitize_html_field(attributes, "consent_text");
            for key in ["privacy_url", "terms_url"] {
                if attributes
                    .get(key)
                    .and_then(Value::as_str)
                    .is_some_and(|url| !is_safe_url(url))
                {
                    attributes.remove(key);
                }
            }
        }
    }
}
