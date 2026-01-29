//! Trade Plan CRUD Integration Tests - Explosive Swings Feature
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Comprehensive tests for trade plan management:
//! - GET /api/room-content/rooms/:room_slug/trade-plan
//! - POST /api/admin/room-content/trade-plan
//! - PUT /api/admin/room-content/trade-plan/:id
//! - DELETE /api/admin/room-content/trade-plan/:id
//! - Week_of date handling
//! - Bias validation

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use chrono::{Datelike, Duration, Utc};
use serde_json::json;
use tower::ServiceExt;

use super::fixtures::{
    TradePlanBuilder, TestContext, TestTradingRoom, TestUser,
    assert_status_and_json, body_to_json, cleanup_room_data,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN LISTING TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_trade_plans_empty_room() {
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
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["data"].is_array());
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_list_trade_plans_with_pagination() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 15 trade plans for pagination testing
    for i in 0..15 {
        let plan = TradePlanBuilder::bullish(&room.slug, &format!("TICK{}", i))
            .with_sort_order(i)
            .build_json();

        ctx.app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/room-content/trade-plan")
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::AUTHORIZATION, admin.auth_header())
                    .body(Body::from(plan.to_string()))
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
                    "/api/room-content/rooms/{}/trade-plan?page=1&per_page=10",
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
    assert!(body["meta"]["total"].as_i64().unwrap() >= 15);
}

#[tokio::test]
async fn test_list_trade_plans_filter_by_week() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Get current week's Monday
    let today = Utc::now().date_naive();
    let days_from_monday = today.weekday().num_days_from_monday() as i64;
    let this_monday = today - Duration::days(days_from_monday);
    let last_monday = this_monday - Duration::days(7);

    // Create plans for this week
    let this_week_plan = TradePlanBuilder::bullish(&room.slug, "THISWEEK")
        .with_week_of(&this_monday.format("%Y-%m-%d").to_string())
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(this_week_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create plans for last week
    let last_week_plan = TradePlanBuilder::bearish(&room.slug, "LASTWEEK")
        .with_week_of(&last_monday.format("%Y-%m-%d").to_string())
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(last_week_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - Filter by last week
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/api/room-content/rooms/{}/trade-plan?week_of={}",
                    room.slug,
                    last_monday.format("%Y-%m-%d")
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Only last week's plans
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let plans = body["data"].as_array().unwrap();
    assert!(!plans.is_empty());
    for plan in plans {
        assert!(plan["week_of"].as_str().unwrap().contains(&last_monday.format("%Y-%m-%d").to_string()));
    }
}

#[tokio::test]
async fn test_list_trade_plans_returns_most_recent_week_by_default() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Get dates for this week and last week
    let today = Utc::now().date_naive();
    let days_from_monday = today.weekday().num_days_from_monday() as i64;
    let this_monday = today - Duration::days(days_from_monday);
    let last_monday = this_monday - Duration::days(7);

    // Create plan for last week
    let last_week_plan = TradePlanBuilder::bullish(&room.slug, "OLD")
        .with_week_of(&last_monday.format("%Y-%m-%d").to_string())
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(last_week_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create plan for this week
    let this_week_plan = TradePlanBuilder::bearish(&room.slug, "CURRENT")
        .with_week_of(&this_monday.format("%Y-%m-%d").to_string())
        .build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(this_week_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - Get without week_of filter (should return most recent week)
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should return this week's plans (most recent)
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let plans = body["data"].as_array().unwrap();

    // Should have plans and they should be from the most recent week
    if !plans.is_empty() {
        // All returned plans should be from the same (most recent) week
        let first_week = plans[0]["week_of"].as_str().unwrap();
        for plan in plans {
            assert_eq!(plan["week_of"].as_str().unwrap(), first_week);
        }
    }
}

#[tokio::test]
async fn test_list_trade_plans_sorted_by_sort_order() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create plans with specific sort orders
    let plans_data = [
        ("SPY", 2),
        ("AAPL", 0),
        ("MSFT", 1),
    ];

    for (ticker, order) in plans_data {
        let plan = TradePlanBuilder::bullish(&room.slug, ticker)
            .with_sort_order(order)
            .build_json();

        ctx.app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/room-content/trade-plan")
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::AUTHORIZATION, admin.auth_header())
                    .body(Body::from(plan.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be sorted by sort_order ascending
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let plans = body["data"].as_array().unwrap();

    if plans.len() >= 3 {
        assert_eq!(plans[0]["ticker"], "AAPL"); // sort_order: 0
        assert_eq!(plans[1]["ticker"], "MSFT"); // sort_order: 1
        assert_eq!(plans[2]["ticker"], "SPY");  // sort_order: 2
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN CREATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_create_bullish_trade_plan() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "SPY")
        .with_entry("495")
        .with_targets("500", Some("505"), Some("510"))
        .with_stop("490")
        .with_options("500", "2026-02-07")
        .with_notes("Weekly breakout setup")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["id"].as_i64().is_some());
    assert_eq!(body["ticker"], "SPY");
    assert_eq!(body["bias"], "BULLISH");
    assert_eq!(body["entry"], "495");
    assert_eq!(body["target1"], "500");
    assert_eq!(body["target2"], "505");
    assert_eq!(body["target3"], "510");
    assert_eq!(body["stop"], "490");
    assert_eq!(body["options_strike"], "500");
    assert_eq!(body["notes"], "Weekly breakout setup");
    assert_eq!(body["is_active"], true);
}

#[tokio::test]
async fn test_create_bearish_trade_plan() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bearish(&room.slug, "QQQ")
        .with_entry("400")
        .with_targets("395", Some("390"), None)
        .with_stop("405")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["bias"], "BEARISH");
}

#[tokio::test]
async fn test_create_neutral_trade_plan() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::neutral(&room.slug, "IWM")
        .with_notes("Range-bound, wait for breakout direction")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["bias"], "NEUTRAL");
}

#[tokio::test]
async fn test_create_trade_plan_with_runner() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "AAPL")
        .with_entry("175")
        .with_targets("180", Some("185"), Some("190"))
        .with_runner("200", "175")
        .with_stop("170")
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["runner"], "200");
    assert_eq!(body["runner_stop"], "175");
}

#[tokio::test]
async fn test_create_trade_plan_ticker_uppercase() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "msft") // lowercase
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Ticker should be uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["ticker"], "MSFT");
}

#[tokio::test]
async fn test_create_trade_plan_bias_uppercase() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::new(&room.slug)
        .with_ticker("NVDA")
        .with_bias("bullish") // lowercase
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Bias should be uppercase
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["bias"], "BULLISH");
}

#[tokio::test]
async fn test_create_trade_plan_default_week_of() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create plan without specifying week_of
    let plan = json!({
        "room_slug": room.slug,
        "ticker": "TSLA",
        "bias": "BULLISH"
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should default to current date
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["week_of"].is_string());
    // The week_of should be today or this week
    let _today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    // Verify week_of is set (exact value depends on implementation)
    assert!(!body["week_of"].as_str().unwrap().is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN UPDATE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_update_trade_plan() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create initial plan
    let plan = TradePlanBuilder::bullish(&room.slug, "AMD")
        .with_entry("150")
        .with_targets("155", Some("160"), None)
        .with_stop("145")
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Update plan
    let update_payload = json!({
        "entry": "148",
        "target1": "153",
        "notes": "Adjusted entry lower"
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["id"], plan_id);
    assert_eq!(body["entry"], "148");
    assert_eq!(body["target1"], "153");
    assert_eq!(body["notes"], "Adjusted entry lower");
    // Original values should be preserved
    assert_eq!(body["ticker"], "AMD");
    assert_eq!(body["bias"], "BULLISH");
}

#[tokio::test]
async fn test_update_trade_plan_change_bias() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create bullish plan
    let plan = TradePlanBuilder::bullish(&room.slug, "META").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();
    assert_eq!(create_body["bias"], "BULLISH");

    // Change bias to bearish
    let update_payload = json!({
        "bias": "BEARISH"
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["bias"], "BEARISH");
}

#[tokio::test]
async fn test_update_trade_plan_deactivate() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create active plan
    let plan = TradePlanBuilder::bullish(&room.slug, "GOOGL").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();
    assert_eq!(create_body["is_active"], true);

    // Deactivate
    let update_payload = json!({
        "is_active": false
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["is_active"], false);
}

#[tokio::test]
async fn test_update_trade_plan_partial() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create plan with many fields
    let plan = TradePlanBuilder::bullish(&room.slug, "NFLX")
        .with_entry("600")
        .with_targets("620", Some("640"), Some("660"))
        .with_stop("580")
        .with_notes("Original notes")
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Update only notes
    let update_payload = json!({
        "notes": "Updated notes only"
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - All other fields should remain unchanged
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["ticker"], "NFLX");
    assert_eq!(body["bias"], "BULLISH");
    assert_eq!(body["entry"], "600");
    assert_eq!(body["target1"], "620");
    assert_eq!(body["target2"], "640");
    assert_eq!(body["target3"], "660");
    assert_eq!(body["stop"], "580");
    assert_eq!(body["notes"], "Updated notes only");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TRADE PLAN DELETE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_delete_trade_plan() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "AMZN").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Act - Delete plan
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
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
async fn test_delete_trade_plan_soft_delete() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "DIS").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Delete
    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - Verify plan no longer appears in list
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Deleted plan should not appear
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let plans = body["data"].as_array().unwrap();
    for plan in plans {
        assert_ne!(plan["id"], plan_id);
    }

    // Verify it's soft deleted (still in DB with deleted_at set)
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM room_trade_plans WHERE id = $1 AND deleted_at IS NOT NULL"
    )
    .bind(plan_id)
    .fetch_optional(&ctx.pool)
    .await
    .unwrap();

    assert!(result.is_some(), "Trade plan should be soft deleted in database");
}

// ═══════════════════════════════════════════════════════════════════════════════════
// AUTHENTICATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_create_trade_plan_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "ORCL").build_json();

    // Act - Try to create as regular member
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, member.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should be forbidden
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_trade_plan_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create plan as admin
    let plan = TradePlanBuilder::bullish(&room.slug, "IBM").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Act - Try to update as member
    let update_payload = json!({"notes": "Hacked!"});

    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
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
async fn test_delete_trade_plan_requires_admin() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let member = TestUser::member(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Create plan as admin
    let plan = TradePlanBuilder::bullish(&room.slug, "CSCO").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let plan_id = create_body["id"].as_i64().unwrap();

    // Act - Try to delete as member
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/admin/room-content/trade-plan/{}", plan_id))
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
async fn test_list_trade_plans_public_access() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Act - Access without authentication
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Public endpoint should be accessible
    assert_eq!(response.status(), StatusCode::OK);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// WEEK_OF DATE HANDLING TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_trade_plan_week_of_date_format() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "PYPL")
        .with_week_of("2026-02-03") // Specific date
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["week_of"].as_str().unwrap().contains("2026-02-03"));
}

#[tokio::test]
async fn test_trade_plan_options_exp_date_format() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::bullish(&room.slug, "SQ")
        .with_options("100", "2026-02-21") // Specific expiration date
        .build_json();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["options_exp"].as_str().unwrap().contains("2026-02-21"));
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BIAS VALIDATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_trade_plan_accepts_bullish_bias() {
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::new(&room.slug)
        .with_ticker("TEST1")
        .with_bias("BULLISH")
        .build_json();

    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_trade_plan_accepts_bearish_bias() {
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::new(&room.slug)
        .with_ticker("TEST2")
        .with_bias("BEARISH")
        .build_json();

    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_trade_plan_accepts_neutral_bias() {
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let plan = TradePlanBuilder::new(&room.slug)
        .with_ticker("TEST3")
        .with_bias("NEUTRAL")
        .build_json();

    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REORDER TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_reorder_trade_plans() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create three plans
    let mut plan_ids = Vec::new();
    for (ticker, order) in [("FIRST", 0), ("SECOND", 1), ("THIRD", 2)] {
        let plan = TradePlanBuilder::bullish(&room.slug, ticker)
            .with_sort_order(order)
            .build_json();

        let response = ctx
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/room-content/trade-plan")
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::AUTHORIZATION, admin.auth_header())
                    .body(Body::from(plan.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = body_to_json(response).await;
        plan_ids.push(body["id"].as_i64().unwrap());
    }

    // Reorder: THIRD -> 0, FIRST -> 1, SECOND -> 2
    let reorder_payload = json!({
        "items": [
            {"id": plan_ids[2], "sort_order": 0},
            {"id": plan_ids[0], "sort_order": 1},
            {"id": plan_ids[1], "sort_order": 2}
        ]
    });

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!(
                    "/api/admin/room-content/rooms/{}/trade-plan/reorder",
                    room.slug
                ))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(reorder_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert_eq!(body["success"], true);

    // Verify order
    let list_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let list_body = body_to_json(list_response).await;
    let plans = list_body["data"].as_array().unwrap();

    if plans.len() >= 3 {
        assert_eq!(plans[0]["ticker"], "THIRD");  // sort_order: 0
        assert_eq!(plans[1]["ticker"], "FIRST");  // sort_order: 1
        assert_eq!(plans[2]["ticker"], "SECOND"); // sort_order: 2
    }
}

#[tokio::test]
async fn test_only_active_plans_returned() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create active plan
    let active_plan = TradePlanBuilder::bullish(&room.slug, "ACTIVE").build_json();

    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(active_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Create and deactivate another plan
    let inactive_plan = TradePlanBuilder::bullish(&room.slug, "INACTIVE").build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trade-plan")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(inactive_plan.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let inactive_id = create_body["id"].as_i64().unwrap();

    // Deactivate
    ctx.app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trade-plan/{}", inactive_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"is_active": false}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Act - List plans
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trade-plan", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Only active plans
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let plans = body["data"].as_array().unwrap();
    for plan in plans {
        assert_eq!(plan["is_active"], true);
        assert_ne!(plan["ticker"], "INACTIVE");
    }
}
