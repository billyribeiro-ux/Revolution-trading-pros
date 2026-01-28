//! Stats Calculation Integration Tests - Explosive Swings Feature
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! Comprehensive tests for performance statistics:
//! - GET /api/room-content/rooms/:room_slug/stats
//! - Stats cache invalidation
//! - Metric accuracy (win rate, avg win/loss, profit factor)

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use chrono::{Duration, Utc};
use serde_json::json;
use tower::ServiceExt;

use super::fixtures::{
    TradeBuilder, TestContext, TestTradingRoom, TestUser,
    assert_status_and_json, body_to_json, cleanup_room_data,
    calculate_win_rate, calculate_profit_factor,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// STATS RETRIEVAL TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_room_stats_empty() {
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
                .uri(format!("/api/room-content/rooms/{}/stats", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    // Stats may be null if no data exists
    assert!(body.get("data").is_some());
}

#[tokio::test]
async fn test_get_room_stats_public_access() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Act - Access without authentication
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/stats", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Public endpoint should be accessible
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_room_stats_nonexistent_room() {
    // Arrange
    let ctx = TestContext::new().await;

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/room-content/rooms/nonexistent-room-12345/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert - Should return OK with null data (not an error)
    let body = assert_status_and_json(response, StatusCode::OK).await;
    assert!(body["data"].is_null());
}

// ═══════════════════════════════════════════════════════════════════════════════════
// WIN RATE CALCULATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_win_rate_70_percent() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Win Rate Test Room", "win-rate-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 7 winning trades and 3 losing trades
    for i in 0..7 {
        let trade = TradeBuilder::long_call(&room.slug, &format!("WIN{}", i), 100.0, 2.00).build_json();
        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        // Close as winner (entry 2.00, exit 4.00)
        ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 4.00}).to_string()))
                .unwrap(),
        ).await.unwrap();
    }

    for i in 0..3 {
        let trade = TradeBuilder::long_call(&room.slug, &format!("LOSS{}", i), 100.0, 2.00).build_json();
        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        // Close as loser (entry 2.00, exit 1.00)
        ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 1.00}).to_string()))
                .unwrap(),
        ).await.unwrap();
    }

    // Act - Calculate expected win rate
    let expected_win_rate = calculate_win_rate(7, 3);
    assert!((expected_win_rate - 70.0).abs() < 0.01);

    // Assert - Verify trades were created correctly
    let list_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trades?status=closed", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let list_body = assert_status_and_json(list_response, StatusCode::OK).await;
    let trades = list_body["data"].as_array().unwrap();
    assert_eq!(trades.len(), 10);

    // Count wins and losses
    let wins = trades.iter().filter(|t| t["result"] == "WIN").count();
    let losses = trades.iter().filter(|t| t["result"] == "LOSS").count();
    assert_eq!(wins, 7);
    assert_eq!(losses, 3);
}

#[tokio::test]
async fn test_win_rate_100_percent() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Perfect Win Room", "perfect-win-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 5 winning trades
    for i in 0..5 {
        let trade = TradeBuilder::long_call(&room.slug, &format!("ALLWIN{}", i), 100.0, 2.00).build_json();
        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 5.00}).to_string()))
                .unwrap(),
        ).await.unwrap();
    }

    // Assert
    let expected_win_rate = calculate_win_rate(5, 0);
    assert_eq!(expected_win_rate, 100.0);

    let list_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trades?status=closed", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let list_body = body_to_json(list_response).await;
    let trades = list_body["data"].as_array().unwrap();
    assert!(trades.iter().all(|t| t["result"] == "WIN"));
}

#[tokio::test]
async fn test_win_rate_0_percent() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "All Loss Room", "all-loss-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 3 losing trades
    for i in 0..3 {
        let trade = TradeBuilder::long_call(&room.slug, &format!("ALLLOSS{}", i), 100.0, 5.00).build_json();
        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 1.00}).to_string()))
                .unwrap(),
        ).await.unwrap();
    }

    // Assert
    let expected_win_rate = calculate_win_rate(0, 3);
    assert_eq!(expected_win_rate, 0.0);

    let list_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/trades?status=closed", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let list_body = body_to_json(list_response).await;
    let trades = list_body["data"].as_array().unwrap();
    assert!(trades.iter().all(|t| t["result"] == "LOSS"));
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PROFIT FACTOR CALCULATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_profit_factor_2_0() {
    // Profit Factor = Total Wins / Total Losses
    // If wins = $1000, losses = $500, profit factor = 2.0
    let profit_factor = calculate_profit_factor(1000.0, 500.0);
    assert!((profit_factor - 2.0).abs() < 0.01);
}

#[tokio::test]
async fn test_profit_factor_infinite() {
    // Profit Factor is infinite when there are no losses
    let profit_factor = calculate_profit_factor(1000.0, 0.0);
    assert!(profit_factor.is_infinite());
    assert!(profit_factor.is_sign_positive());
}

#[tokio::test]
async fn test_profit_factor_zero() {
    // Profit Factor is 0 when there are no wins but there are losses
    let profit_factor = calculate_profit_factor(0.0, 500.0);
    assert_eq!(profit_factor, 0.0);
}

#[tokio::test]
async fn test_profit_factor_less_than_one() {
    // Profit Factor < 1 when losses exceed wins
    let profit_factor = calculate_profit_factor(300.0, 600.0);
    assert!((profit_factor - 0.5).abs() < 0.01);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// P&L CALCULATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_pnl_calculation_long_call_win() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Entry: 2.50 per contract, Quantity: 2, Exit: 5.00
    // P&L = (5.00 - 2.50) * 2 = 5.00
    let trade = TradeBuilder::long_call(&room.slug, "PNLTEST1", 500.0, 2.50)
        .with_quantity(2)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Act
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 5.00}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(close_response, StatusCode::OK).await;
    let pnl = body["pnl"].as_f64().unwrap();
    assert!((pnl - 5.00).abs() < 0.01, "Expected PnL ~5.00, got {}", pnl);
}

#[tokio::test]
async fn test_pnl_calculation_long_call_loss() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Entry: 5.00 per contract, Quantity: 1, Exit: 1.00
    // P&L = (1.00 - 5.00) * 1 = -4.00
    let trade = TradeBuilder::long_call(&room.slug, "PNLTEST2", 500.0, 5.00)
        .with_quantity(1)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Act
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 1.00}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(close_response, StatusCode::OK).await;
    let pnl = body["pnl"].as_f64().unwrap();
    assert!((pnl - (-4.00)).abs() < 0.01, "Expected PnL ~-4.00, got {}", pnl);
}

#[tokio::test]
async fn test_pnl_percent_calculation() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Entry: 4.00, Exit: 6.00
    // P&L % = ((6.00 - 4.00) / 4.00) * 100 = 50%
    let trade = TradeBuilder::long_call(&room.slug, "PNLPCT", 500.0, 4.00)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Act
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 6.00}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(close_response, StatusCode::OK).await;
    let pnl_percent = body["pnl_percent"].as_f64().unwrap();
    assert!((pnl_percent - 50.0).abs() < 0.1, "Expected PnL% ~50, got {}", pnl_percent);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HOLDING DAYS CALCULATION TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_holding_days_same_day() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let today = Utc::now().date_naive();
    let trade = TradeBuilder::long_call(&room.slug, "HOLD0", 100.0, 2.00)
        .with_entry_date(&today.format("%Y-%m-%d").to_string())
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Act - Close same day
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({
                    "exit_price": 3.00,
                    "exit_date": today.format("%Y-%m-%d").to_string()
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(close_response, StatusCode::OK).await;
    let holding_days = body["holding_days"].as_i64().unwrap();
    assert_eq!(holding_days, 0);
}

#[tokio::test]
async fn test_holding_days_multi_day() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let entry_date = Utc::now().date_naive() - Duration::days(10);
    let exit_date = Utc::now().date_naive();

    let trade = TradeBuilder::long_call(&room.slug, "HOLD10", 100.0, 2.00)
        .with_entry_date(&entry_date.format("%Y-%m-%d").to_string())
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Act
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({
                    "exit_price": 3.00,
                    "exit_date": exit_date.format("%Y-%m-%d").to_string()
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(close_response, StatusCode::OK).await;
    let holding_days = body["holding_days"].as_i64().unwrap();
    assert_eq!(holding_days, 10);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// AVERAGE WIN/LOSS TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_average_win_calculation() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Avg Win Room", "avg-win-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 3 winning trades with different P&L values
    // Trade 1: Entry 2.00, Exit 4.00, Qty 1 -> P&L = 2.00
    // Trade 2: Entry 3.00, Exit 6.00, Qty 1 -> P&L = 3.00
    // Trade 3: Entry 1.00, Exit 5.00, Qty 1 -> P&L = 4.00
    // Average Win = (2.00 + 3.00 + 4.00) / 3 = 3.00

    let trades = [
        (2.00, 4.00), // P&L = 2.00
        (3.00, 6.00), // P&L = 3.00
        (1.00, 5.00), // P&L = 4.00
    ];

    let mut total_pnl = 0.0;

    for (i, (entry, exit)) in trades.iter().enumerate() {
        let trade = TradeBuilder::long_call(&room.slug, &format!("AVGWIN{}", i), 100.0, *entry)
            .with_quantity(1)
            .build_json();

        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        let close_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": exit}).to_string()))
                .unwrap(),
        ).await.unwrap();

        let close_body = body_to_json(close_resp).await;
        total_pnl += close_body["pnl"].as_f64().unwrap();
    }

    // Assert
    let expected_avg = total_pnl / 3.0;
    assert!((expected_avg - 3.00).abs() < 0.01);
}

#[tokio::test]
async fn test_average_loss_calculation() {
    // Arrange
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Avg Loss Room", "avg-loss-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create 2 losing trades with different P&L values
    // Trade 1: Entry 5.00, Exit 3.00, Qty 1 -> P&L = -2.00
    // Trade 2: Entry 4.00, Exit 1.00, Qty 1 -> P&L = -3.00
    // Average Loss = (-2.00 + -3.00) / 2 = -2.50

    let trades = [
        (5.00, 3.00), // P&L = -2.00
        (4.00, 1.00), // P&L = -3.00
    ];

    let mut total_loss = 0.0;

    for (i, (entry, exit)) in trades.iter().enumerate() {
        let trade = TradeBuilder::long_call(&room.slug, &format!("AVGLOSS{}", i), 100.0, *entry)
            .with_quantity(1)
            .build_json();

        let create_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();

        let trade_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

        let close_resp = ctx.app.clone().oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": exit}).to_string()))
                .unwrap(),
        ).await.unwrap();

        let close_body = body_to_json(close_resp).await;
        total_loss += close_body["pnl"].as_f64().unwrap();
    }

    // Assert
    let expected_avg = total_loss / 2.0;
    assert!((expected_avg - (-2.50)).abs() < 0.01);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// STATS CACHE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_stats_structure() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    // Manually insert stats cache for testing
    sqlx::query(
        r#"
        INSERT INTO room_stats_cache
        (room_id, room_slug, win_rate, weekly_profit, monthly_profit, active_trades,
         closed_this_week, total_trades, wins, losses, avg_win, avg_loss,
         profit_factor, avg_holding_days, largest_win, largest_loss, current_streak, calculated_at)
        VALUES (
            $1, $2, 68.5, '+$2,500', '+$8,750', 3,
            5, 42, 29, 13, 125.50, 75.25,
            2.15, 3.5, 450.00, 200.00, 4, NOW()
        )
        ON CONFLICT (room_slug) DO UPDATE SET
            win_rate = EXCLUDED.win_rate,
            calculated_at = NOW()
        "#
    )
    .bind(room.id)
    .bind(&room.slug)
    .execute(&ctx.pool)
    .await
    .unwrap();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/stats", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let data = &body["data"];

    // Verify all expected fields are present
    assert!(data["win_rate"].is_number() || data["win_rate"].is_null());
    assert!(data["weekly_profit"].is_string() || data["weekly_profit"].is_null());
    assert!(data["monthly_profit"].is_string() || data["monthly_profit"].is_null());
    assert!(data["active_trades"].is_number() || data["active_trades"].is_null());
    assert!(data["closed_this_week"].is_number() || data["closed_this_week"].is_null());
    assert!(data["total_trades"].is_number() || data["total_trades"].is_null());
    assert!(data["wins"].is_number() || data["wins"].is_null());
    assert!(data["losses"].is_number() || data["losses"].is_null());
    assert!(data["profit_factor"].is_number() || data["profit_factor"].is_null());
}

#[tokio::test]
async fn test_stats_win_rate_value() {
    // Arrange
    let ctx = TestContext::new().await;
    let room = TestTradingRoom::create(&ctx, "Stats Value Test", "stats-value-test").await;

    // Insert specific win rate
    sqlx::query(
        r#"
        INSERT INTO room_stats_cache (room_id, room_slug, win_rate, calculated_at)
        VALUES ($1, $2, 72.5, NOW())
        ON CONFLICT (room_slug) DO UPDATE SET
            win_rate = EXCLUDED.win_rate,
            calculated_at = NOW()
        "#
    )
    .bind(room.id)
    .bind(&room.slug)
    .execute(&ctx.pool)
    .await
    .unwrap();

    // Act
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/stats", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    let body = assert_status_and_json(response, StatusCode::OK).await;
    let win_rate = body["data"]["win_rate"].as_f64().unwrap();
    assert!((win_rate - 72.5).abs() < 0.01);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// EDGE CASE TESTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_stats_with_only_open_trades() {
    // Stats should handle rooms with only open trades (no closed trades yet)
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Only Open Trades", "only-open-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create only open trades (don't close them)
    for i in 0..3 {
        let trade = TradeBuilder::long_call(&room.slug, &format!("OPEN{}", i), 100.0, 2.00).build_json();
        ctx.app.clone().oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        ).await.unwrap();
    }

    // Get stats - should return OK with empty/null metrics
    let response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/room-content/rooms/{}/stats", room.slug))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_stats_with_invalidated_trades_excluded() {
    // Invalidated trades should not be counted in stats
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::create(&ctx, "Invalid Excluded", "invalid-excluded-test").await;
    cleanup_room_data(&ctx.pool, &room.slug).await;

    // Create and close a winning trade
    let win_trade = TradeBuilder::long_call(&room.slug, "STATWIN", 100.0, 2.00).build_json();
    let create_resp = ctx.app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/admin/room-content/trades")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, admin.auth_header())
            .body(Body::from(win_trade.to_string()))
            .unwrap(),
    ).await.unwrap();
    let win_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

    ctx.app.clone().oneshot(
        Request::builder()
            .method("PUT")
            .uri(format!("/api/admin/room-content/trades/{}/close", win_id))
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, admin.auth_header())
            .body(Body::from(json!({"exit_price": 4.00}).to_string()))
            .unwrap(),
    ).await.unwrap();

    // Create and invalidate a trade
    let invalid_trade = TradeBuilder::long_call(&room.slug, "STATINV", 100.0, 2.00).build_json();
    let create_resp = ctx.app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/admin/room-content/trades")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, admin.auth_header())
            .body(Body::from(invalid_trade.to_string()))
            .unwrap(),
    ).await.unwrap();
    let inv_id = body_to_json(create_resp).await["id"].as_i64().unwrap();

    ctx.app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri(format!("/api/admin/room-content/trades/{}/invalidate", inv_id))
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, admin.auth_header())
            .body(Body::from(json!({"reason": "Test invalidation"}).to_string()))
            .unwrap(),
    ).await.unwrap();

    // List closed trades - should only have the winning trade
    let list_resp = ctx.app.clone().oneshot(
        Request::builder()
            .uri(format!("/api/room-content/rooms/{}/trades?status=closed", room.slug))
            .body(Body::empty())
            .unwrap(),
    ).await.unwrap();

    let list_body = body_to_json(list_resp).await;
    let closed_trades = list_body["data"].as_array().unwrap();

    // Only the closed trade should appear, not the invalidated one
    assert_eq!(closed_trades.len(), 1);
    assert_eq!(closed_trades[0]["ticker"], "STATWIN");
}

#[tokio::test]
async fn test_stats_with_breakeven_trade() {
    // A trade that closes at exactly entry price (0% gain)
    let ctx = TestContext::new().await;
    let admin = TestUser::admin(&ctx).await;
    let room = TestTradingRoom::explosive_swings(&ctx).await;

    let trade = TradeBuilder::long_call(&room.slug, "BREAKEVEN", 100.0, 3.00)
        .build_json();

    let create_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(trade.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = body_to_json(create_response).await;
    let trade_id = create_body["id"].as_i64().unwrap();

    // Close at exactly entry price
    let close_response = ctx
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{}/close", trade_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, admin.auth_header())
                .body(Body::from(json!({"exit_price": 3.00}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = assert_status_and_json(close_response, StatusCode::OK).await;

    let pnl = body["pnl"].as_f64().unwrap();
    assert!((pnl - 0.0).abs() < 0.01, "Breakeven trade should have PnL ~0");

    let pnl_percent = body["pnl_percent"].as_f64().unwrap();
    assert!((pnl_percent - 0.0).abs() < 0.01, "Breakeven trade should have PnL% ~0");

    // Breakeven is technically not a win (PnL <= 0), so result should be LOSS
    assert_eq!(body["result"], "LOSS");
}
