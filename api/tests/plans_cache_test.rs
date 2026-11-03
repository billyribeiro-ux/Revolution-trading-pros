//! Public membership-plan cache — freshness contract (needs the test DB).
//!
//! `GET /api/subscriptions/plans` and `/plans/{slug}` are read-through cached
//! (60s TTL). The risk with caching a price list is showing a stale price
//! after an admin edit, so this pins that every admin write path is visible
//! to the very next public read, not one TTL later.

// Shared harness; this file uses only the admin-app helper.
mod common;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use common::{body_to_json, setup_test_app_with_admin};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(
    app: &axum::Router,
    method: Method,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(t) = token {
        req = req.header(header::AUTHORIZATION, format!("Bearer {t}"));
    }
    let req = match body {
        Some(b) => req
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(b.to_string())),
        None => req.body(Body::empty()),
    }
    .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    (status, body_to_json(res).await)
}

fn plan_in_list(list: &Value, id: i64) -> Option<&Value> {
    list.as_array()?.iter().find(|p| p["id"] == id)
}

#[tokio::test]
async fn admin_plan_writes_are_visible_to_next_public_read() {
    let (app, token) = setup_test_app_with_admin().await;
    let name = format!("Cache Contract {}", uuid::Uuid::new_v4());

    // Create: a previously cached list must not hide the new plan.
    let (status, _) = call(&app, Method::GET, "/api/subscriptions/plans", None, None).await;
    assert_eq!(status, StatusCode::OK);
    let (status, created) = call(
        &app,
        Method::POST,
        "/api/admin/subscriptions/plans",
        Some(&token),
        Some(json!({"name": name, "price_cents": 4_900, "billing_cycle": "monthly"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "create failed: {created}");
    let id = created["data"]["id"].as_i64().expect("created plan id");
    let slug = created["data"]["slug"].as_str().unwrap().to_owned();

    let (_, list) = call(&app, Method::GET, "/api/subscriptions/plans", None, None).await;
    assert_eq!(plan_in_list(&list, id).unwrap()["price_cents"], 4_900);
    // Prime the by-slug entry too.
    let by_slug = format!("/api/subscriptions/plans/{slug}");
    let (_, one) = call(&app, Method::GET, &by_slug, None, None).await;
    assert_eq!(one["price_cents"], 4_900);

    // Update: both cached entries must reflect the new price immediately.
    let (status, _) = call(
        &app,
        Method::PUT,
        &format!("/api/admin/subscriptions/plans/{id}"),
        Some(&token),
        Some(json!({"price_cents": 5_900})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, list) = call(&app, Method::GET, "/api/subscriptions/plans", None, None).await;
    assert_eq!(plan_in_list(&list, id).unwrap()["price_cents"], 5_900);
    let (_, one) = call(&app, Method::GET, &by_slug, None, None).await;
    assert_eq!(one["price_cents"], 5_900);

    // Delete: a cached entry must not outlive the row.
    let (status, _) = call(
        &app,
        Method::DELETE,
        &format!("/api/admin/subscriptions/plans/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, list) = call(&app, Method::GET, "/api/subscriptions/plans", None, None).await;
    assert!(plan_in_list(&list, id).is_none());
    let (status, _) = call(&app, Method::GET, &by_slug, None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
