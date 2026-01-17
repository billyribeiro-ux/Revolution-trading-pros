//! Content-Type Middleware
//! ICT 11+ Principal Engineer: CORB Fix
//!
//! Ensures all JSON responses have explicit Content-Type headers
//! This prevents CORB (Cross-Origin Read Blocking) when using credentials
//!
//! CRITICAL: When using CORS with credentials + X-Content-Type-Options: nosniff,
//! browsers enforce strict CORB policies. Every response MUST have an explicit
//! Content-Type header or the browser will block it.
//!
//! EXCEPTION: 204 NO_CONTENT and 304 NOT_MODIFIED must NOT have Content-Type
//! per RFC 7231, as they have no message body.

use axum::{
    body::Body,
    http::{header, Request, Response, StatusCode},
    middleware::Next,
};

/// Middleware to ensure all responses have Content-Type header
///
/// This is critical for CORB compliance when using:
/// - CORS with credentials (allow_credentials: true)
/// - X-Content-Type-Options: nosniff security header
///
/// IMPORTANT: 204 NO_CONTENT and 304 NOT_MODIFIED should NOT have Content-Type
/// per HTTP spec (RFC 7231), as they have no body.
pub async fn ensure_content_type(
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    // Process the request
    let mut response = next.run(request).await;

    let status = response.status();

    // RFC 7231: 204 NO_CONTENT and 304 NOT_MODIFIED MUST NOT contain a message body
    // Therefore they should NOT have a Content-Type header
    if status == StatusCode::NO_CONTENT || status == StatusCode::NOT_MODIFIED {
        return Ok(response);
    }

    // Check if Content-Type is already set
    if response.headers().get(header::CONTENT_TYPE).is_none() {
        // Default to application/json for API responses with bodies
        // This is safe because our API is JSON-only (except robots.txt, sitemap.xml which set their own)
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json; charset=utf-8"),
        );

        tracing::debug!(
            status = %status,
            "Added missing Content-Type header to response (CORB fix)"
        );
    }

    Ok(response)
}

/// Middleware to log response headers for debugging CORB issues
#[allow(dead_code)]
pub async fn log_response_headers(
    request: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let uri = request.uri().clone();
    let method = request.method().clone();

    let response = next.run(request).await;

    // Log headers for debugging
    if tracing::enabled!(tracing::Level::DEBUG) {
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("MISSING");

        let cors_origin = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("MISSING");

        tracing::debug!(
            method = %method,
            uri = %uri,
            content_type = %content_type,
            cors_origin = %cors_origin,
            "Response headers"
        );
    }

    Ok(response)
}
