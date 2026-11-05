//! Internal-error scrubbing.
//!
//! ~600 handlers build their 500 body from the underlying error
//! (`json!({"error": e.to_string()})`), which hands sqlx / driver / upstream
//! internals (table and constraint names, SQL fragments, hostnames) to the
//! client. Rather than rely on every call site, this layer withholds the body
//! of EVERY `500 Internal Server Error` and replaces it with a fixed, generic
//! JSON body. The original body is logged server-side so nothing is lost for
//! debugging.
//!
//! Only 500 is rewritten: 4xx bodies are intentional, client-actionable
//! messages (validation, conflicts) and must reach the client.

use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::Response,
};

/// Fixed body sent in place of any 500 response body.
const GENERIC_BODY: &str = r#"{"error":"Internal server error"}"#;

/// Upper bound on how much of the original body is read for logging.
const MAX_LOGGED_BODY_BYTES: usize = 16 * 1024;

pub async fn scrub_internal_errors(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = crate::middleware::request_trace::diagnostic_path(&request).to_owned();

    let response = next.run(request).await;
    if response.status() != StatusCode::INTERNAL_SERVER_ERROR {
        return response;
    }

    let (mut parts, body) = response.into_parts();
    let original = match to_bytes(body, MAX_LOGGED_BODY_BYTES).await {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(e) => format!("<body unreadable or over {MAX_LOGGED_BODY_BYTES} bytes: {e}>"),
    };
    tracing::error!(
        target: "internal_error",
        %method,
        %path,
        body = %original,
        "500 response body withheld from client"
    );

    // The old body is gone, so its framing headers must go with it.
    parts.headers.remove(header::CONTENT_LENGTH);
    parts.headers.remove(header::CONTENT_ENCODING);
    parts.headers.remove(header::TRANSFER_ENCODING);
    parts.headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    Response::from_parts(parts, Body::from(GENERIC_BODY))
}
