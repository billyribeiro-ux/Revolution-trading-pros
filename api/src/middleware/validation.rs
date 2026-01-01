//! Input Validation Middleware
//! ICT 11+ Principal Engineer: Request validation and sanitization
//!
//! Provides:
//! - Request body size limits
//! - Content-Type validation
//! - Input sanitization helpers
//! - JSON schema validation support

use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

/// Maximum request body size (10MB default)
pub const MAX_BODY_SIZE: usize = 10 * 1024 * 1024;

/// Middleware to validate request content type and size
pub async fn validate_request(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    // Check Content-Length if present
    if let Some(content_length) = request.headers().get(header::CONTENT_LENGTH) {
        if let Ok(length) = content_length.to_str().unwrap_or("0").parse::<usize>() {
            if length > MAX_BODY_SIZE {
                return Err((StatusCode::PAYLOAD_TOO_LARGE, "Request body too large"));
            }
        }
    }

    // For POST/PUT/PATCH, validate Content-Type
    let method = request.method().as_str();
    if matches!(method, "POST" | "PUT" | "PATCH") {
        // Allow requests without body (e.g., logout)
        let has_body = request
            .headers()
            .get(header::CONTENT_LENGTH)
            .map(|v| v != "0")
            .unwrap_or(false);

        if has_body {
            let content_type = request
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            // Allow JSON and form data
            if !content_type.contains("application/json")
                && !content_type.contains("multipart/form-data")
                && !content_type.contains("application/x-www-form-urlencoded")
            {
                return Err((
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "Content-Type must be application/json or form data",
                ));
            }
        }
    }

    Ok(next.run(request).await)
}

/// Sanitize string input (remove potential XSS vectors)
pub fn sanitize_string(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('/', "&#x2F;")
}

/// Sanitize string for SQL LIKE patterns (escape wildcards)
pub fn sanitize_like_pattern(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// Validate that a string contains only alphanumeric characters and common punctuation
pub fn is_safe_string(input: &str) -> bool {
    input.chars().all(|c| {
        c.is_alphanumeric()
            || c.is_whitespace()
            || matches!(c, '-' | '_' | '.' | ',' | '!' | '?' | '@' | '#' | '$' | '&' | '*' | '(' | ')' | '+' | '=' | ':' | ';' | '\'' | '"')
    })
}

/// Validate slug format (lowercase alphanumeric with hyphens)
pub fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 200
        && slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && !slug.contains("--")
}

/// Validate UUID format
pub fn is_valid_uuid(input: &str) -> bool {
    uuid::Uuid::parse_str(input).is_ok()
}

/// Validate positive integer
pub fn is_positive_integer(input: &str) -> bool {
    input.parse::<u64>().is_ok()
}

/// Truncate string to maximum length with ellipsis
pub fn truncate(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else if max_length <= 3 {
        input[..max_length].to_string()
    } else {
        format!("{}...", &input[..max_length - 3])
    }
}

/// Normalize whitespace (collapse multiple spaces, trim)
pub fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("<script>"), "&lt;script&gt;");
        assert_eq!(sanitize_string("Hello \"World\""), "Hello &quot;World&quot;");
    }

    #[test]
    fn test_is_valid_slug() {
        assert!(is_valid_slug("hello-world"));
        assert!(is_valid_slug("test123"));
        assert!(!is_valid_slug("-invalid"));
        assert!(!is_valid_slug("invalid-"));
        assert!(!is_valid_slug("in--valid"));
        assert!(!is_valid_slug("UPPERCASE"));
        assert!(!is_valid_slug(""));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 8), "hello...");
        assert_eq!(truncate("hi", 2), "hi");
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("  hello   world  "), "hello world");
    }
}
