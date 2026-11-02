//! `scrub_internal_errors` contract — pure, no DB.
//!
//! Handlers build 500 bodies from raw sqlx/upstream errors. The layer must
//! withhold every 500 body from the client while leaving intentional 4xx
//! messages (and successes) untouched.

use axum::{
    Json, Router,
    body::Body,
    http::{Request, StatusCode, header},
    middleware::from_fn,
    routing::get,
};
use http_body_util::BodyExt;
use revolution_api::middleware::scrub_internal_errors;
use serde_json::{Value, json};
use tower::ServiceExt;

const LEAK: &str = r#"error returned from database: relation "users" does not exist"#;

fn app() -> Router {
    Router::new()
        .route(
            "/leaky-500",
            get(|| async {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": LEAK })),
                )
            }),
        )
        .route(
            "/plain-500",
            get(|| async { (StatusCode::INTERNAL_SERVER_ERROR, LEAK) }),
        )
        .route(
            "/bad-request",
            get(|| async {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "email is invalid" })),
                )
            }),
        )
        .route("/ok", get(|| async { Json(json!({ "ok": true })) }))
        .layer(from_fn(scrub_internal_errors))
}

async fn get_path(path: &str) -> (StatusCode, axum::http::HeaderMap, Vec<u8>) {
    let res = app()
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = res.into_body().collect().await.unwrap().to_bytes().to_vec();
    (status, headers, bytes)
}

#[tokio::test]
async fn json_500_body_is_withheld() {
    let (status, headers, body) = get_path("/leaky-500").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let text = String::from_utf8(body.clone()).unwrap();
    assert!(!text.contains("relation"), "leaked: {text}");
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap(),
        json!({ "error": "Internal server error" })
    );
    assert_eq!(headers[header::CONTENT_TYPE], "application/json");
    // The old body's framing must not survive the rewrite.
    let len = headers
        .get(header::CONTENT_LENGTH)
        .map(|v| v.to_str().unwrap().parse::<usize>().unwrap());
    assert!(len.is_none() || len == Some(body.len()));
}

#[tokio::test]
async fn plain_text_500_body_is_withheld() {
    let (status, headers, body) = get_path("/plain-500").await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(!String::from_utf8(body).unwrap().contains("relation"));
    assert_eq!(headers[header::CONTENT_TYPE], "application/json");
}

#[tokio::test]
async fn client_errors_and_successes_pass_through() {
    let (status, _, body) = get_path("/bad-request").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap(),
        json!({ "error": "email is invalid" })
    );

    let (status, _, body) = get_path("/ok").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        serde_json::from_slice::<Value>(&body).unwrap(),
        json!({ "ok": true })
    );
}
