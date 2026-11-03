//! Tier 1 Follow-Up Regression Tests (`fix/audit-followup-2026-11`).
//!
//! Each test exercises the real router via `.oneshot()` against the test
//! database and Redis (`setup_test_app_with_admin` / `setup_test_app_with_user_token`).

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use common::{
    body_to_json, mint_access_token, seed_user, setup_test_app, setup_test_app_with_admin,
    setup_test_app_with_user_token,
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

async fn test_pool() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("connect to test db")
}

/// Tier 1 #1: `GET /api/admin/cms-v2/settings`, `/menus`, `/menus/{id}` must
/// reject anonymous callers with 401 and non-admin members with 403.
#[tokio::test]
async fn tier1_1_cms_v2_admin_gets_require_admin() {
    let (app, member_token) = setup_test_app_with_user_token().await;

    for path in [
        "/api/admin/cms-v2/settings",
        "/api/admin/cms-v2/menus",
        "/api/admin/cms-v2/menus/00000000-0000-0000-0000-000000000001",
    ] {
        let anon_res = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(
            anon_res.status(),
            StatusCode::UNAUTHORIZED,
            "Anonymous GET {path} must return 401, got {}",
            anon_res.status()
        );

        let member_res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("Authorization", format!("Bearer {member_token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            member_res.status(),
            StatusCode::FORBIDDEN,
            "Member GET {path} must return 403, got {}",
            member_res.status()
        );
    }
}

/// Tier 1 #2: Six `/api/admin/room-content/rooms/{slug}/*` GET routes must
/// require `AdminUser` (rejecting even a paying member with 403).
#[tokio::test]
async fn tier1_2_admin_room_content_requires_admin() {
    let app = setup_test_app().await;
    let pool = test_pool().await;
    let member_id = seed_user(
        &pool,
        "tier1-paying-member@revolutiontradingpros.test",
        "Paying Member",
        "user",
    )
    .await;
    let member_token = mint_access_token(member_id);

    for suffix in [
        "trade-plan",
        "alerts",
        "trades",
        "weekly-video",
        "weekly-videos",
        "stats",
    ] {
        let path = format!("/api/admin/room-content/rooms/explosive-swings/{suffix}");
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&path)
                    .header("Authorization", format!("Bearer {member_token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            res.status(),
            StatusCode::FORBIDDEN,
            "Member GET {path} must be rejected by AdminUser with 403, got {}",
            res.status()
        );
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8_lossy(&bytes);
        assert_eq!(
            body_str, "Admin access required",
            "Admin route {path} must fail in AdminUser extractor, got: {body_str}"
        );
    }
}

/// Tier 1 #3: `POST /api/admin/member-management` must actually commit the
/// created user row and activity log entry to PostgreSQL.
#[tokio::test]
async fn tier1_3_create_member_persists_user_and_activity_log() {
    let (app, admin_token) = setup_test_app_with_admin().await;
    let pool = test_pool().await;
    let email = format!("audit2-tier1-member-{}@example.com", Uuid::new_v4());

    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/member-management")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Audit Tier1 Member",
                        "email": email,
                        "role": "user"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap();

    // Clean up test fixture
    let _ = sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await;

    assert_eq!(
        count, 1,
        "POST /api/admin/member-management returned 200 but user row count in DB was {count}"
    );
}

/// Tier 1 #4: Refresh tokens must embed a unique `jti` so `login -> refresh -> refresh`
/// within the same wall-clock second does not blacklist the newly minted token.
#[tokio::test]
async fn tier1_4_refresh_token_embeds_jti_and_survives_same_second_rotation() {
    let app = setup_test_app().await;
    let pool = test_pool().await;
    let email = format!("audit2-refresh-{}@example.com", Uuid::new_v4());
    let _user_id = seed_user(&pool, &email, "Refresh User", "user").await;

    let login_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "email": email,
                        "password": "TestPassword123!@#"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(login_res.status(), StatusCode::OK);
    let login_json = body_to_json(login_res).await;
    let r1 = login_json["refresh_token"]
        .as_str()
        .expect("refresh_token in login response")
        .to_string();

    // First refresh in the same second as login
    let refresh1_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/refresh")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "refresh_token": r1 }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(refresh1_res.status(), StatusCode::OK);
    let refresh1_json = body_to_json(refresh1_res).await;
    let r2 = refresh1_json["refresh_token"]
        .as_str()
        .expect("refresh_token in refresh1 response")
        .to_string();

    assert_ne!(
        r1, r2,
        "Rotated refresh token r2 must differ from r1 even within the same second"
    );

    // Second refresh using r2 must succeed (not 401 REFRESH_TOKEN_REUSE)
    let refresh2_res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/refresh")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "refresh_token": r2 }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status2 = refresh2_res.status();
    let body2 = body_to_json(refresh2_res).await;

    let _ = sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await;

    assert_eq!(
        status2,
        StatusCode::OK,
        "Second refresh failed with {status2}: {body2}"
    );
}

/// Tier 1 #5: Pagination with huge `page` or negative `page`/`per_page` must
/// clamp cleanly and never panic or return 500.
#[tokio::test]
async fn tier1_5_pagination_clamps_overflow_and_negative_params() {
    let (app, admin_token) = setup_test_app_with_admin().await;

    for path in [
        "/api/admin/courses?page=999999999&per_page=100",
        "/api/admin/indicators?page=999999999&per_page=100",
        "/api/admin/popups?page=999999999&per_page=100",
        "/api/admin/popups?page=-1&per_page=-5",
    ] {
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("Authorization", format!("Bearer {admin_token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            res.status(),
            StatusCode::OK,
            "GET {path} must return 200 OK, got {}",
            res.status()
        );
    }
}

/// Tier 1 #6: `POST /api/admin/subscriptions/plans` must reject negative
/// `price_cents` and blank `name` with 422, and return 409 on duplicate slug.
#[tokio::test]
async fn tier1_6_subscription_plan_rejects_negative_price_blank_name_and_409_on_duplicate() {
    let (app, admin_token) = setup_test_app_with_admin().await;
    let pool = test_pool().await;

    // 1. Negative price_cents -> 422
    let neg_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/subscriptions/plans")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Audit2 Negative Plan",
                        "price_cents": -5000,
                        "billing_cycle": "monthly"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        neg_res.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "Negative price_cents must return 422, got {}",
        neg_res.status()
    );

    // 2. Blank name -> 422
    let blank_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/subscriptions/plans")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "   ",
                        "price_cents": 4900,
                        "billing_cycle": "monthly"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        blank_res.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "Blank plan name must return 422, got {}",
        blank_res.status()
    );

    // 3. Valid plan + duplicate slug -> 409
    let unique_name = format!("Audit2 Plan {}", Uuid::new_v4());
    let ok_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/subscriptions/plans")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": unique_name,
                        "price_cents": 9900,
                        "billing_cycle": "monthly"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ok_res.status(), StatusCode::OK);

    let dup_res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/subscriptions/plans")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "name": unique_name,
                        "price_cents": 9900,
                        "billing_cycle": "monthly"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let dup_status = dup_res.status();

    let _ = sqlx::query("DELETE FROM membership_plans WHERE name = $1")
        .bind(&unique_name)
        .execute(&pool)
        .await;

    assert_eq!(
        dup_status,
        StatusCode::CONFLICT,
        "Duplicate plan slug must return 409 Conflict, got {dup_status}"
    );
}

/// Tier 1 #7: `PUT /api/admin/room-content/trades/{id}/close` must handle
/// uppercase `"LONG"` direction without inverting P&L sign and return 409 when
/// closing an already-closed trade.
#[tokio::test]
async fn tier1_7_close_trade_case_insensitive_direction_and_409_on_repeat_close() {
    let (app, admin_token) = setup_test_app_with_admin().await;
    let pool = test_pool().await;

    let create_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/admin/room-content/trades")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "room_slug": "explosive-swings",
                        "ticker": "NVDA",
                        "trade_type": "stock",
                        "direction": "LONG",
                        "quantity": 10,
                        "entry_price": 100.0,
                        "entry_date": "2026-05-01"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_res.status(), StatusCode::OK);
    let created = body_to_json(create_res).await;
    let trade_id = created["id"].as_i64().expect("trade id");

    // First close at higher price (110.0) on LONG trade -> positive P&L (+100.0) & WIN
    let close1_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{trade_id}/close"))
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "exit_price": 110.0,
                        "exit_date": "2026-05-02"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(close1_res.status(), StatusCode::OK);
    let closed1 = body_to_json(close1_res).await;
    let pnl = closed1["pnl"].as_f64().unwrap_or(0.0);
    let result = closed1["result"].as_str().unwrap_or("");

    // Second close on already-closed trade -> 409 Conflict
    let close2_res = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/admin/room-content/trades/{trade_id}/close"))
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "exit_price": 120.0,
                        "exit_date": "2026-05-03"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let close2_status = close2_res.status();

    let _ = sqlx::query("DELETE FROM room_trades WHERE id = $1")
        .bind(trade_id)
        .execute(&pool)
        .await;

    assert!(
        pnl > 0.0,
        "Winning LONG trade must have positive P&L, got {pnl}"
    );
    assert_eq!(result, "WIN");
    assert_eq!(
        close2_status,
        StatusCode::CONFLICT,
        "Re-closing an already closed trade must return 409 Conflict, got {close2_status}"
    );
}

/// Tier 1 #8: `routes/health.rs` `setup_db` must declare `password_resets.id`
/// as `BIGSERIAL PRIMARY KEY` (matching `routes/auth/password.rs:183` and
/// `schema.sql:8511`) and must not leak raw SQL errors in `SetupResponse`.
#[test]
fn tier1_8_setup_db_uses_bigserial_password_resets_and_scrubs_errors() {
    let source = include_str!("../src/routes/health.rs");
    assert!(
        source.contains("CREATE TABLE password_resets (\n            id BIGSERIAL PRIMARY KEY,"),
        "setup_db must create password_resets with `id BIGSERIAL PRIMARY KEY` to match routes/auth/password.rs:183"
    );
    assert!(
        !source.contains("Failed to create table: {e}"),
        "setup_db must not leak raw SQL error strings in SetupResponse"
    );
}
