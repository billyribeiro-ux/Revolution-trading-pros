//! Request spans deliberately exclude queries, credentials and request headers.
//!
//! Verification/reset URLs carry bearer capabilities. A DEBUG deployment must
//! retain useful request diagnostics without persisting those capabilities.
use axum::{extract::MatchedPath, http::Request};

/// Never fall back to the raw path: unmatched paths can contain capabilities too.
pub fn diagnostic_path<B>(request: &Request<B>) -> &str {
    request
        .extensions()
        .get::<MatchedPath>()
        .map_or("<unmatched>", MatchedPath::as_str)
}

pub fn request_span<B>(request: &Request<B>) -> tracing::Span {
    tracing::debug_span!(
        "request",
        method = %request.method(),
        path = %diagnostic_path(request),
        version = ?request.version(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::StatusCode,
        middleware,
        routing::get,
    };
    use std::{
        io::{self, Write},
        sync::{Arc, Mutex},
    };
    use tower::ServiceExt;
    use tower_http::trace::TraceLayer;
    use tracing::instrument::WithSubscriber;

    #[derive(Clone)]
    struct Captured(Arc<Mutex<Vec<u8>>>);
    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Captured {
        type Writer = Self;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }
    impl Write for Captured {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .expect("captured log mutex")
                .extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn debug_http_trace_excludes_capability_queries_and_keeps_outcomes() {
        let captured = Captured(Arc::new(Mutex::new(Vec::new())));
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .without_time()
            .with_ansi(false)
            .with_writer(captured.clone())
            .finish();
        let app = Router::new()
            .route(
                "/api/auth/verify-email",
                get(|| async {
                    tracing::debug!("verification handler reached");
                    StatusCode::OK
                }),
            )
            .route(
                "/api/auth/reset-password",
                get(|| async {
                    tracing::error!("bounded capability failure");
                    StatusCode::SERVICE_UNAVAILABLE
                }),
            )
            .route("/preview/{token}", get(|| async { StatusCode::OK }))
            .nest(
                "/api/tracking",
                Router::new()
                    .route(
                        "/click/{token}",
                        get(|| async { StatusCode::SERVICE_UNAVAILABLE }),
                    )
                    .route(
                        "/open/{token}",
                        get(|| async {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "bounded internal failure",
                            )
                        }),
                    ),
            )
            .layer(middleware::from_fn(
                crate::middleware::internal_errors::scrub_internal_errors,
            ))
            // The dormant diagnostic helper is exercised too, so enabling it
            // later cannot silently reintroduce the same capability leak.
            .layer(middleware::from_fn(
                crate::middleware::content_type::log_response_headers,
            ))
            .layer(TraceLayer::new_for_http().make_span_with(request_span));
        async {
            for (path, expected) in [
                ("/api/auth/verify-email", StatusCode::OK),
                ("/api/auth/reset-password", StatusCode::SERVICE_UNAVAILABLE),
                ("/preview/PREVIEW_PATH_SENTINEL_e03c", StatusCode::OK),
                ("/api/tracking/click/CLICK_PATH_SENTINEL_60fb", StatusCode::SERVICE_UNAVAILABLE),
                ("/api/tracking/open/OPEN_PATH_SENTINEL_2e5a", StatusCode::INTERNAL_SERVER_ERROR),
                ("/UNKNOWN_PATH_SENTINEL_d12a", StatusCode::NOT_FOUND),
            ] {
                // HTTP parsers strip actual fragments before transport. This
                // includes both a raw fragment and a fragment encoded in a
                // query value; neither may enter application diagnostics.
                let uri = format!(
                    "{path}?token=CAPABILITY_SENTINEL_6be9&next=%23FRAGMENT_SENTINEL_72ed#RAW_FRAGMENT_SENTINEL_ea18"
                );
                let response = app
                    .clone()
                    .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
                    .await
                    .unwrap();
                assert_eq!(response.status(), expected);
                if expected == StatusCode::INTERNAL_SERVER_ERROR {
                    let body=to_bytes(response.into_body(),4096).await.unwrap();
                    assert_eq!(&body[..],br#"{"error":"Internal server error"}"#);
                }
            }
        }
        .with_subscriber(subscriber)
        .await;
        let output = {
            let bytes = captured.0.lock().unwrap();
            String::from_utf8(bytes.clone()).unwrap()
        };
        for sentinel in [
            "CAPABILITY_SENTINEL_6be9",
            "FRAGMENT_SENTINEL_72ed",
            "RAW_FRAGMENT_SENTINEL_ea18",
            "PREVIEW_PATH_SENTINEL_e03c",
            "CLICK_PATH_SENTINEL_60fb",
            "OPEN_PATH_SENTINEL_2e5a",
            "UNKNOWN_PATH_SENTINEL_d12a",
            "token=",
            "next=",
            "%23",
            "?",
        ] {
            assert!(
                !output.contains(sentinel),
                "sensitive query marker leaked: {sentinel}"
            );
        }
        for retained in [
            "method=GET",
            "path=/api/auth/verify-email",
            "path=/api/auth/reset-password",
            "verification handler reached",
            "bounded capability failure",
            "path=/preview/{token}",
            "path=/api/tracking/click/{token}",
            "path=/api/tracking/open/{token}",
            "path=<unmatched>",
            "500 response body withheld from client",
            "status=404",
            "status=500",
            "status=200",
            "status=503",
            "Response headers",
        ] {
            assert!(
                output.contains(retained),
                "expected diagnostic absent: {retained}; output: {output}"
            );
        }
    }
}
