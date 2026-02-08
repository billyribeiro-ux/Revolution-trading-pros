//! Alert CRUD Integration Tests - Explosive Swings Feature
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Comprehensive tests for alert management:
//! - GET /api/room-content/rooms/:room_slug/alerts (pagination, filtering)
//! - POST /api/admin/room-content/alerts (create with validation)
//! - PUT /api/admin/room-content/alerts/:id (update)
//! - DELETE /api/admin/room-content/alerts/:id
//! - TOS format string generation
//! - Alert type validation (ENTRY, EXIT, UPDATE)
//! - Authentication/authorization

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use chrono::{Duration, NaiveDate, Utc};
use serde_json::json;
use tower::ServiceExt;

use super::fixtures::{
    assert_status_and_json, body_to_json, cleanup_room_data, AlertBuilder, TestContext,
    TestTradingRoom, TestUser, TosFormatter,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT LISTING TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_alerts_empty_room() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/alerts", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["data"].is_array());
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
    assert_eq!(body["meta"]["total"], 0);
}

#[tokio::test]
async fn test_list_alerts_with_pagination() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 25 alerts for pagination testing
    for i in 0..25 {
        let alert = AlertBuilder::entry(&room.slug, &format!("TICK{}", i))
            .with_title(format!("Alert {}", i))
            .build_json();

        ctx.app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/room-content/alerts")
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::AUTHORIZATION, admin.auth_header())
                    .body(Body::from(alert.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    // Act - Get first page
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/room-content/rooms/{}/alerts?page=1&per_page=10",
                    room.slug
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 10);
    assert_eq!(body["meta"]["current_page"], 1);
    assert_eq!(body["meta"]["per_page"], 10);
    assert!(body["meta"]["total"].as_i64().unwrap() >= 25);
    assert!(body["meta"]["total_pages"].as_i64().unwrap() >= 3);

    // Act - Get second page
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/room-content/rooms/{}/alerts?page=2&per_page=10",
                    room.slug
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 10);
    assert_eq!(body["meta"]["current_page"], 2);
}

#[tokio::test]
async fn test_list_alerts_respects_per_page_limit() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Act - Request more than max per_page (100)
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/room-content/rooms/{}/alerts?per_page=200",
                    room.slug
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be capped at 100
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["meta"]["per_page"].as_i64().unwrap() <= 100);
}

#[tokio::test]
async fn test_list_alerts_only_published() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create published alert
    let published_alert = AlertBuilder::entry(&room.slug, "AAPL")
        .with_is_published(true)
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(published_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create unpublished alert
    let unpublished_alert = AlertBuilder::entry(&room.slug, "MSFT")
        .with_is_published(false)
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(unpublished_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - Public endpoint should only return published alerts
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/alerts", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let alerts = body["data"].as_array().unwrap();

    // All returned alerts should be published
    for alert in alerts {
        assert_eq!(alert["is_published"], true);
    }
}

#[tokio::test]
async fn test_list_alerts_pinned_first() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create regular alert first
    let regular_alert = AlertBuilder::entry(&room.slug, "AAPL")
        .with_is_pinned(false)
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(regular_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create pinned alert second
    let pinned_alert = AlertBuilder::entry(&room.slug, "MSFT")
        .with_is_pinned(true)
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(pinned_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/alerts", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Pinned alert should be first
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let alerts = body["data"].as_array().unwrap();
    assert!(alerts.len() >= 2);
    assert_eq!(alerts[0]["is_pinned"], true);
    assert_eq!(alerts[0]["ticker"], "MSFT");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT CREATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_create_entry_alert() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "SPY")
        .with_strike(500.0)
        .with_option_type("CALL")
        .with_fill_price(2.50)
        .with_notes("Test entry alert")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["id"].as_i64().is_some());
    assert_eq!(body["alert_type"], "ENTRY");
    assert_eq!(body["ticker"], "SPY");
    assert_eq!(body["strike"], 500.0);
    assert_eq!(body["option_type"], "CALL");
    assert_eq!(body["fill_price"], 2.50);
    assert_eq!(body["is_new"], true);
    assert_eq!(body["is_published"], true);
}

#[tokio::test]
async fn test_create_exit_alert() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // First create an entry alert
    let entry_alert = AlertBuilder::entry(&room.slug, "AAPL")
        .with_strike(175.0)
        .build_json();

    let entry_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(entry_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let entry_body = body_to_json(entry_response).await;
    let entry_id = entry_body["id"].as_i64().unwrap();

    // Create exit alert linked to entry
    let exit_alert = AlertBuilder::exit(&room.slug, "AAPL", entry_id)
        .with_fill_price(5.00)
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(exit_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["alert_type"], "EXIT");
    assert_eq!(body["ticker"], "AAPL");
    assert_eq!(body["entry_alert_id"], entry_id);
    assert_eq!(body["action"], "SELL");
}

#[tokio::test]
async fn test_create_update_alert() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // First create an entry alert
    let entry_alert = AlertBuilder::entry(&room.slug, "TSLA").build_json();

    let entry_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(entry_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let entry_body = body_to_json(entry_response).await;
    let entry_id = entry_body["id"].as_i64().unwrap();

    // Create update alert
    let update_alert = AlertBuilder::update(&room.slug, "TSLA", entry_id)
        .with_notes("Adjusting stop loss to breakeven")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["alert_type"], "UPDATE");
    assert_eq!(body["entry_alert_id"], entry_id);
}

#[tokio::test]
async fn test_create_alert_with_tos_string() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let expiration = Utc::now().date_naive() + Duration::days(7);
    let tos_string = TosFormatter::options(
        "BUY",
        1,
        "SPY",
        100,
        "Weeklys",
        &expiration,
        500.0,
        "CALL",
        "MKT",
        None,
    );

    let alert = AlertBuilder::entry(&room.slug, "SPY")
        .with_tos_string(&tos_string)
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["tos_string"], tos_string);
}

#[tokio::test]
async fn test_create_alert_ticker_uppercase() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "aapl") // lowercase input
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Ticker should be uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["ticker"], "AAPL");
}

#[tokio::test]
async fn test_create_alert_with_limit_order() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "NVDA")
        .with_order_type("LMT")
        .with_limit_price(3.50)
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["order_type"], "LMT");
    assert_eq!(body["limit_price"], 3.50);
}

#[tokio::test]
async fn test_create_alert_shares_trade() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::new(&room.slug)
        .with_alert_type("ENTRY")
        .with_ticker("GOOG")
        .with_trade_type("shares")
        .with_action("BUY")
        .with_quantity(100)
        .with_fill_price(140.50)
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["trade_type"], "shares");
    assert_eq!(body["quantity"], 100);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT UPDATE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_update_alert() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create initial alert
    let alert = AlertBuilder::entry(&room.slug, "AMD").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Update alert
    let update_payload = json!({
        "ticker": "INTC",
        "notes": "Updated notes",
        "is_pinned": true
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["id"], alert_id);
    assert_eq!(body["ticker"], "INTC");
    assert_eq!(body["notes"], "Updated notes");
    assert_eq!(body["is_pinned"], true);
}

#[tokio::test]
async fn test_update_alert_partial() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert with specific values
    let alert = AlertBuilder::entry(&room.slug, "META")
        .with_strike(500.0)
        .with_fill_price(5.00)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Partial update - only change notes
    let update_payload = json!({
        "notes": "New notes only"
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Other fields should remain unchanged
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["ticker"], "META");
    assert_eq!(body["strike"], 500.0);
    assert_eq!(body["fill_price"], 5.00);
    assert_eq!(body["notes"], "New notes only");
}

#[tokio::test]
async fn test_update_alert_mark_as_read() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert as new
    let alert = AlertBuilder::entry(&room.slug, "AMZN")
        .with_is_new(true)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();
    assert_eq!(create_body["is_new"], true);

    // Mark as read
    let update_payload = json!({
        "is_new": false
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["is_new"], false);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT DELETE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_delete_alert() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert
    let alert = AlertBuilder::entry(&room.slug, "NFLX").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Act - Delete alert
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["success"], true);
}

#[tokio::test]
async fn test_delete_alert_soft_delete() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create alert
    let alert = AlertBuilder::entry(&room.slug, "DIS").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Delete alert
    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - Verify alert no longer appears in public list
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/alerts", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Deleted alert should not appear
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let alerts = body["data"].as_array().unwrap();
    for alert in alerts {
        assert_ne!(alert["id"], alert_id);
    }

    // Verify it's soft deleted (still in DB with deleted_at set)
    let result: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM room_alerts WHERE id = $1 AND deleted_at IS NOT NULL")
            .bind(alert_id)
            .fetch_optional(&ctx.pool)
            .await
            .unwrap();

    assert!(result.is_some(), "Alert should be soft deleted in database");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// AUTHENTICATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_create_alert_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "ORCL").build_json();

    // Act - Try to create as regular member
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, member.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be forbidden
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_create_alert_no_auth() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "IBM").build_json();

    // Act - Try to create without authentication
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_alert_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert as admin
    let alert = AlertBuilder::entry(&room.slug, "CSCO").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Act - Try to update as member
    let update_payload = json!({"notes": "Hacked!"});

    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, member.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_alert_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert as admin
    let alert = AlertBuilder::entry(&room.slug, "QCOM").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Act - Try to delete as member
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/alerts/{}", alert_id))
                .header(header::AUTHORIZATION, member.auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_list_alerts_public_access() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Act - Access without authentication
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/alerts", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Public endpoint should be accessible
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_super_admin_can_manage_alerts() {
    // Arrange
    let ctx = TestContext::new().await;
    let super_admin = TestUser::super_admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "PYPL").build_json();

    // Act - Create as super admin
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, super_admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_developer_can_manage_alerts() {
    // Arrange
    let ctx = TestContext::new().await;
    let developer = TestUser::developer(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::entry(&room.slug, "SQ").build_json();

    // Act - Create as developer
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, developer.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TOS FORMAT STRING TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_tos_formatter_options_market_order() {
    let expiration = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
    let tos_string = TosFormatter::options(
        "BUY",
        1,
        "SPY",
        100,
        "Weeklys",
        &expiration,
        500.0,
        "CALL",
        "MKT",
        None,
    );

    assert!(tos_string.contains("BUY +1 SPY"));
    assert!(tos_string.contains("Weeklys"));
    assert!(tos_string.contains("500"));
    assert!(tos_string.contains("CALL"));
    assert!(tos_string.contains("@MKT"));
}

#[tokio::test]
async fn test_tos_formatter_options_limit_order() {
    let expiration = NaiveDate::from_ymd_opt(2026, 2, 21).unwrap();
    let tos_string = TosFormatter::options(
        "SELL",
        5,
        "AAPL",
        100,
        "Monthly",
        &expiration,
        175.0,
        "PUT",
        "LMT",
        Some(2.50),
    );

    assert!(tos_string.contains("SELL -5 AAPL"));
    assert!(tos_string.contains("Monthly"));
    assert!(tos_string.contains("175"));
    assert!(tos_string.contains("PUT"));
    assert!(tos_string.contains("@2.50 LMT"));
}

#[tokio::test]
async fn test_tos_formatter_shares() {
    let tos_string = TosFormatter::shares("BUY", 100, "GOOG", "MKT", None);

    assert_eq!(tos_string, "BUY +100 GOOG @MKT");
}

#[tokio::test]
async fn test_tos_formatter_shares_limit() {
    let tos_string = TosFormatter::shares("SELL", 50, "MSFT", "LMT", Some(400.00));

    assert_eq!(tos_string, "SELL -50 MSFT @400.00 LMT");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ALERT TYPE VALIDATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_alert_type_uppercase_normalization() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert with lowercase type
    let alert = AlertBuilder::new(&room.slug)
        .with_alert_type("entry") // lowercase
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be normalized to uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["alert_type"], "ENTRY");
}

#[tokio::test]
async fn test_action_uppercase_normalization() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::new(&room.slug)
        .with_action("buy") // lowercase
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be normalized to uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["action"], "BUY");
}

#[tokio::test]
async fn test_option_type_uppercase_normalization() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let alert = AlertBuilder::new(&room.slug)
        .with_option_type("call") // lowercase
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be normalized to uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["option_type"], "CALL");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// MARK ALERT AS READ TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_mark_alert_read_endpoint() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create alert as new
    let alert = AlertBuilder::entry(&room.slug, "UBER")
        .with_is_new(true)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/alerts")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(alert.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let alert_id = create_body["id"].as_i64().unwrap();

    // Act - Mark as read via public endpoint
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/room-content/rooms/{}/alerts/{}/read",
                    room.slug, alert_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["success"], true);
}
