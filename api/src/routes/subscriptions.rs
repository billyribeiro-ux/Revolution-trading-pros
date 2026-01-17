//! Subscription routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

/// Membership plan row
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: f64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// User subscription row - flexible schema for production compatibility
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UserSubscriptionRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: Option<chrono::NaiveDateTime>,
    pub current_period_end: Option<chrono::NaiveDateTime>,
    pub cancel_at_period_end: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Create subscription request
#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: i64,
    pub payment_method_id: Option<String>,
    pub coupon_code: Option<String>,
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    pub cancel_immediately: Option<bool>,
    pub reason: Option<String>,
}

/// List all membership plans (public)
async fn list_plans(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembershipPlanRow>>, (StatusCode, Json<serde_json::Value>)> {
    let plans: Vec<MembershipPlanRow> =
        sqlx::query_as("SELECT * FROM membership_plans WHERE is_active = true ORDER BY price ASC")
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(plans))
}

/// Get plan by slug (public)
async fn get_plan(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<MembershipPlanRow>, (StatusCode, Json<serde_json::Value>)> {
    let plan: MembershipPlanRow = sqlx::query_as("SELECT * FROM membership_plans WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Plan not found"})),
            )
        })?;

    Ok(Json(plan))
}

/// Get user's subscriptions
async fn get_my_subscriptions(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let subscriptions: Vec<UserSubscriptionRow> = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Map to frontend format: { id, status, startDate, nextPayment, total, items }
    let mapped: Vec<serde_json::Value> = subscriptions.into_iter().map(|sub| {
        json!({
            "id": sub.id,
            "status": sub.status,
            "startDate": sub.starts_at.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "N/A".to_string()),
            "nextPayment": sub.current_period_end.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            "total": "$0.00", // TODO: Calculate from plan price
            "items": [] // TODO: Get plan details
        })
    }).collect();

    Ok(Json(json!({
        "subscriptions": mapped
    })))
}

/// Get active subscription
async fn get_active_subscription(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Option<UserSubscriptionRow>>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: Option<UserSubscriptionRow> = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE user_id = $1 AND status = 'active' ORDER BY created_at DESC LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(subscription))
}

/// Create subscription (subscribe to plan)
async fn create_subscription(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the plan
    let plan: MembershipPlanRow =
        sqlx::query_as("SELECT * FROM membership_plans WHERE id = $1 AND is_active = true")
            .bind(input.plan_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Plan not found or inactive"})),
                )
            })?;

    // Check if user already has active subscription
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT id FROM user_memberships WHERE user_id = $1 AND status = 'active'")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "User already has an active subscription"})),
        ));
    }

    // Create Stripe checkout session if plan has stripe_price_id
    if let Some(ref stripe_price_id) = plan.stripe_price_id {
        // TODO: Create Stripe checkout session
        // For now, return a placeholder
        return Ok(Json(json!({
            "checkout_url": format!("https://checkout.stripe.com/placeholder?price={}", stripe_price_id),
            "plan": plan,
            "message": "Redirect to Stripe checkout"
        })));
    }

    // For free plans, create subscription directly
    let subscription: UserSubscriptionRow = sqlx::query_as(
        r#"
        INSERT INTO user_memberships (user_id, plan_id, starts_at, status, payment_provider, cancel_at_period_end, created_at, updated_at)
        VALUES ($1, $2, NOW(), 'active', 'free', false, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(user.id)
    .bind(input.plan_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "subscription": subscription,
        "message": "Subscription created successfully"
    })))
}

/// Cancel subscription
async fn cancel_subscription(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<CancelSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get subscription
    let subscription: UserSubscriptionRow =
        sqlx::query_as("SELECT * FROM user_memberships WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Subscription not found"})),
                )
            })?;

    if subscription.status != "active" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Subscription is not active"})),
        ));
    }

    let cancel_immediately = input.cancel_immediately.unwrap_or(false);

    if cancel_immediately {
        // Cancel immediately
        sqlx::query(
            "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    } else {
        // Cancel at period end
        sqlx::query(
            "UPDATE user_memberships SET cancel_at_period_end = true, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    Ok(Json(json!({
        "message": if cancel_immediately { "Subscription cancelled" } else { "Subscription will be cancelled at period end" }
    })))
}

/// Subscription metrics (admin)
async fn get_metrics(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &user; // TODO: Admin check

    let active: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let cancelled: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'cancelled'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let expired: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'expired'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "total_active": active.0,
        "total_cancelled": cancelled.0,
        "total_expired": expired.0
    })))
}

/// GET /subscriptions/export - Export subscriptions as CSV
/// ICT 7 FIX: Added missing endpoint that frontend expects
async fn export_subscriptions(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::response::Response, (StatusCode, Json<serde_json::Value>)> {
    use axum::http::header;
    use axum::response::IntoResponse;

    let format = params.get("format").map(|s| s.as_str()).unwrap_or("csv");

    // Fetch subscriptions with user info
    let subscriptions: Vec<(
        i64,
        i64,
        Option<String>,
        String,
        Option<chrono::NaiveDateTime>,
        Option<chrono::NaiveDateTime>,
        Option<f64>,
        Option<String>,
        Option<String>,
    )> = sqlx::query_as(
        r#"
        SELECT
            us.id,
            us.user_id,
            u.email,
            us.status,
            us.starts_at,
            us.expires_at,
            us.price,
            us.product_name,
            us.billing_period
        FROM user_memberships us
        LEFT JOIN users u ON us.user_id = u.id
        ORDER BY us.created_at DESC
        LIMIT 10000
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if format == "json" {
        let json_data: Vec<serde_json::Value> = subscriptions
            .iter()
            .map(
                |(id, user_id, email, status, starts, expires, price, name, period)| {
                    json!({
                        "id": id,
                        "user_id": user_id,
                        "email": email,
                        "status": status,
                        "starts_at": starts,
                        "expires_at": expires,
                        "price": price,
                        "product_name": name,
                        "billing_period": period
                    })
                },
            )
            .collect();

        let response = (
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_string(&json_data).unwrap_or_default(),
        );
        return Ok(response.into_response());
    }

    // CSV format
    let mut csv = String::from(
        "id,user_id,email,status,starts_at,expires_at,price,product_name,billing_period\n",
    );
    for (id, user_id, email, status, starts, expires, price, name, period) in subscriptions {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            id,
            user_id,
            email.as_deref().unwrap_or("").replace(',', ";"),
            status,
            starts.map(|d| d.to_string()).unwrap_or_default(),
            expires.map(|d| d.to_string()).unwrap_or_default(),
            price.unwrap_or(0.0),
            name.as_deref().unwrap_or("").replace(',', ";"),
            period.as_deref().unwrap_or("")
        ));
    }

    let response = (
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"subscriptions.csv\"",
            ),
        ],
        csv,
    );

    Ok(response.into_response())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/plans", get(list_plans))
        .route("/plans/:slug", get(get_plan))
        .route("/my", get(get_my_subscriptions))
        .route("/my/active", get(get_active_subscription))
        .route("/export", get(export_subscriptions))
        .route("/", post(create_subscription))
        .route("/:id/cancel", post(cancel_subscription))
        .route("/metrics", get(get_metrics))
}

/// Router for /my/subscriptions path (frontend compatibility)
pub fn my_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_subscriptions))
        .route("/:id", get(get_subscription_by_id))
}

/// Get single subscription by ID
async fn get_subscription_by_id(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<UserSubscriptionRow>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: UserSubscriptionRow =
        sqlx::query_as("SELECT * FROM user_memberships WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Subscription not found"})),
                )
            })?;

    Ok(Json(subscription))
}
