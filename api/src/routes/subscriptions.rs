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

// ICT 7 SECURITY FIX: Added AdminUser for admin-only endpoints
use crate::{middleware::admin::AdminUser, models::User, AppState};

/// Membership plan row (price in integer cents per architecture standard §1.2)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Extended membership plan with room info (price in integer cents)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct MembershipPlanExtended {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub price_cents: i64,
    pub billing_cycle: String,
    pub interval_count: Option<i32>,
    pub savings_percent: Option<i32>,
    pub is_popular: Option<bool>,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub stripe_price_id: Option<String>,
    pub stripe_product_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub sort_order: Option<i32>,
    pub room_id: Option<i64>,
    pub room_name: Option<String>,
    pub room_slug: Option<String>,
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

/// Upgrade/Downgrade subscription request (ICT 7+ Enterprise)
#[derive(Debug, Deserialize)]
pub struct ChangePlanRequest {
    pub new_plan_id: i64,
    pub prorate: Option<bool>,
}

/// Proration calculation result (all monetary in integer cents)
#[derive(Debug, serde::Serialize)]
pub struct ProrationPreview {
    pub current_plan_credit_cents: i64,
    pub new_plan_cost_cents: i64,
    pub proration_amount_cents: i64,
    pub effective_date: String,
    pub billing_cycle_days_remaining: i64,
}

/// List all membership plans (public)
async fn list_plans(
    State(state): State<AppState>,
) -> Result<Json<Vec<MembershipPlanRow>>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plans: Vec<MembershipPlanRow> = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE is_active = true ORDER BY price ASC"#,
    )
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
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE slug = $1"#,
    )
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

/// Get all subscription plan variants for a specific trading room
/// GET /subscriptions/room/:room_slug/plans
async fn get_room_plans(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let plans: Vec<MembershipPlanExtended> = sqlx::query_as(
        r#"SELECT 
            mp.id, mp.name, mp.slug, mp.display_name, mp.description,
            (mp.price * 100)::BIGINT AS price_cents, mp.billing_cycle,
            mp.interval_count, mp.savings_percent, mp.is_popular,
            mp.is_active, mp.metadata, mp.stripe_price_id, mp.stripe_product_id,
            mp.features, mp.trial_days, mp.sort_order, mp.room_id,
            tr.name as room_name, tr.slug as room_slug
        FROM membership_plans mp
        JOIN trading_rooms tr ON tr.id = mp.room_id
        WHERE tr.slug = $1 AND mp.is_active = true
        ORDER BY mp.sort_order ASC"#,
    )
    .bind(&room_slug)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if plans.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No plans found for this room"})),
        ));
    }

    // Get room info from first plan
    let room_name = plans
        .first()
        .and_then(|p| p.room_name.clone())
        .unwrap_or_default();

    Ok(Json(json!({
        "room_slug": room_slug,
        "room_name": room_name,
        "plans": plans,
        "total": plans.len()
    })))
}

/// Extended subscription row with plan details (ICT 7+ Enterprise)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct UserSubscriptionWithPlanRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub trial_ends_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub current_period_start: Option<chrono::NaiveDateTime>,
    pub current_period_end: Option<chrono::NaiveDateTime>,
    pub cancel_at_period_end: Option<bool>,
    pub grace_period_end: Option<chrono::NaiveDateTime>,
    pub failed_payment_count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    // Plan details (joined)
    pub plan_name: Option<String>,
    pub plan_price_cents: Option<i64>,
    pub plan_billing_cycle: Option<String>,
    pub plan_features: Option<serde_json::Value>,
    pub plan_trial_days: Option<i32>,
}

/// Get user's subscriptions with complete plan details
/// ICT 7+ Fix: Returns full plan information instead of placeholders
async fn get_my_subscriptions(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7+ Fix: Join with membership_plans to get full plan details
    let subscriptions: Vec<UserSubscriptionWithPlanRow> = sqlx::query_as(
        r#"
        SELECT
            um.id,
            um.user_id,
            um.plan_id,
            um.starts_at,
            um.expires_at,
            um.trial_ends_at,
            um.cancelled_at,
            um.status,
            um.payment_provider,
            um.stripe_subscription_id,
            um.stripe_customer_id,
            um.current_period_start,
            um.current_period_end,
            um.cancel_at_period_end,
            um.grace_period_end,
            um.payment_failure_count AS failed_payment_count,
            um.created_at,
            um.updated_at,
            mp.name as plan_name,
            (mp.price * 100)::BIGINT as plan_price_cents,
            mp.billing_cycle as plan_billing_cycle,
            mp.features as plan_features,
            mp.trial_days as plan_trial_days
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.user_id = $1
        ORDER BY um.created_at DESC
        "#,
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

    // Map to frontend format with full plan details (price in integer cents)
    let mapped: Vec<serde_json::Value> = subscriptions.iter().map(|sub| {
        let price_cents: i64 = sub.plan_price_cents.unwrap_or(0);
        let billing_cycle = sub.plan_billing_cycle.clone().unwrap_or_else(|| "monthly".to_string());
        let now = chrono::Utc::now().naive_utc();

        // Calculate if in trial period
        let in_trial = sub.trial_ends_at
            .map(|trial_end| trial_end > now)
            .unwrap_or(false);

        // Calculate if in grace period
        let in_grace_period = sub.grace_period_end
            .map(|grace_end| grace_end > now)
            .unwrap_or(false);

        // Calculate days remaining in period
        let days_remaining = sub.current_period_end
            .map(|end| {
                if end > now {
                    ((end - now).num_seconds() as f64 / 86400.0).ceil() as i64
                } else {
                    0
                }
            })
            .unwrap_or(0);

        json!({
            "id": sub.id,
            "userId": sub.user_id,
            "planId": sub.plan_id,
            "status": sub.status,
            "startDate": sub.starts_at.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "N/A".to_string()),
            "nextPayment": sub.current_period_end.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            "expiresAt": sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
            "cancelledAt": sub.cancelled_at.map(|d| d.format("%Y-%m-%d").to_string()),
            "cancelAtPeriodEnd": sub.cancel_at_period_end.unwrap_or(false),

            // Trial info
            "inTrial": in_trial,
            "trialEndsAt": sub.trial_ends_at.map(|d| d.format("%Y-%m-%d").to_string()),

            // Grace period info
            "inGracePeriod": in_grace_period,
            "gracePeriodEnd": sub.grace_period_end.map(|d| d.format("%Y-%m-%d").to_string()),
            "failedPaymentCount": sub.failed_payment_count.unwrap_or(0),

            // Payment info
            "paymentProvider": sub.payment_provider,
            "stripeSubscriptionId": sub.stripe_subscription_id,
            "stripeCustomerId": sub.stripe_customer_id,

            // Period info
            "currentPeriodStart": sub.current_period_start.map(|d| d.format("%Y-%m-%d").to_string()),
            "currentPeriodEnd": sub.current_period_end.map(|d| d.format("%Y-%m-%d").to_string()),
            "daysRemaining": days_remaining,

            // Plan details (monetary in integer cents per arch standard)
            "productName": sub.plan_name.clone().unwrap_or_else(|| "Unknown Plan".to_string()),
            "priceCents": price_cents,
            "interval": billing_cycle.clone(),
            "features": sub.plan_features.clone().unwrap_or(json!([])),

            // Items array for frontend compatibility
            "items": [{
                "name": sub.plan_name.clone().unwrap_or_else(|| "Subscription".to_string()),
                "priceCents": price_cents,
                "billingCycle": billing_cycle,
                "quantity": 1
            }],

            // Timestamps
            "createdAt": sub.created_at.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
            "updatedAt": sub.updated_at.map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string())
        })
    }).collect();

    Ok(Json(json!({
        "subscriptions": mapped,
        "total": mapped.len(),
        "hasActiveSubscription": subscriptions.iter().any(|s| s.status == "active" || s.status == "trialing")
    })))
}

/// Get active subscription
async fn get_active_subscription(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Option<UserSubscriptionRow>>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: Option<UserSubscriptionRow> = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE user_id = $1 AND status IN ('active', 'trialing') ORDER BY created_at DESC LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(subscription))
}

/// Create subscription (subscribe to plan)
/// ICT 7+ Enterprise: Full trial period support, proper period calculation
async fn create_subscription(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the plan
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true"#,
    )
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
    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_memberships WHERE user_id = $1 AND status IN ('active', 'trialing')",
    )
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
            Json(
                json!({"error": "User already has an active subscription. Use upgrade/downgrade endpoint instead."}),
            ),
        ));
    }

    // Create Stripe checkout session if plan has stripe_price_id.
    // The Stripe-side subscription will fire `checkout.session.completed` →
    // `customer.subscription.created` webhooks; the webhook handler in
    // payments.rs is responsible for inserting the matching user_memberships
    // row. Do not write user_memberships here.
    if let Some(ref stripe_price_id) = plan.stripe_price_id {
        let app_url = state.config.app_url.trim_end_matches('/');
        let success_url = format!(
            "{}/account/subscriptions?status=success&plan={}",
            app_url, plan.slug
        );
        let cancel_url = format!("{}/account/subscriptions?status=cancel", app_url);

        let checkout_url = state
            .services
            .stripe
            .create_subscription_checkout(&user.email, stripe_price_id, &success_url, &cancel_url)
            .await
            .map_err(|e| {
                tracing::error!(
                    user_id = user.id,
                    plan_id = plan.id,
                    error = %e,
                    "Failed to create Stripe subscription checkout session"
                );
                (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": "Failed to create Stripe checkout session"})),
                )
            })?;

        tracing::info!(
            user_id = user.id,
            plan_id = plan.id,
            stripe_price_id = stripe_price_id.as_str(),
            "Created Stripe subscription checkout session"
        );

        return Ok(Json(json!({
            "checkout_url": checkout_url,
            "plan": plan,
            "message": "Redirect to Stripe checkout",
        })));
    }

    // ICT 7+ Enterprise: Calculate trial period and billing periods
    let now = chrono::Utc::now().naive_utc();
    let trial_days = plan.trial_days.unwrap_or(0);
    let has_trial = trial_days > 0;

    // Calculate trial end date if applicable
    let trial_ends_at = if has_trial {
        Some(now + chrono::Duration::days(trial_days as i64))
    } else {
        None
    };

    // Calculate billing period based on plan type
    let period_days = match plan.billing_cycle.as_str() {
        "monthly" => 30,
        "quarterly" => 90,
        "annual" | "yearly" => 365,
        _ => 30,
    };

    // Period starts after trial (or now if no trial)
    let period_start = trial_ends_at.unwrap_or(now);
    let period_end = period_start + chrono::Duration::days(period_days);

    // Initial status is 'trial' if trial period, otherwise 'active'
    let initial_status = if has_trial { "trial" } else { "active" };

    // For free plans or trials, create subscription directly
    let subscription: UserSubscriptionRow = sqlx::query_as(
        r#"
        INSERT INTO user_memberships (
            user_id, plan_id, starts_at, status, payment_provider,
            trial_ends_at, current_period_start, current_period_end,
            cancel_at_period_end, created_at, updated_at
        )
        VALUES ($1, $2, NOW(), $3, 'free', $4, $5, $6, false, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(user.id)
    .bind(input.plan_id)
    .bind(initial_status)
    .bind(trial_ends_at)
    .bind(period_start)
    .bind(period_end)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "subscription": subscription,
        "plan": {
            "id": plan.id,
            "name": plan.name,
            "price_cents": plan.price_cents,
            "billing_cycle": plan.billing_cycle,
            "trial_days": trial_days
        },
        "trial": {
            "has_trial": has_trial,
            "trial_days": trial_days,
            "trial_ends_at": trial_ends_at.map(|d| d.format("%Y-%m-%d").to_string())
        },
        "message": if has_trial {
            format!("Subscription created with {}-day trial", trial_days)
        } else {
            "Subscription created successfully".to_string()
        }
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
/// ICT 7+ Enterprise: Complete MRR/ARR/churn/LTV calculations
async fn get_metrics(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Basic counts
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

    let trial: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'trial'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let paused: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'paused' OR status = 'on-hold'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let pending_cancel: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE cancel_at_period_end = true AND status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    // MRR in integer cents — sum active+trialing memberships' plan prices, normalized to monthly
    let mrr_result: Option<(i64,)> = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM(
            CASE mp.billing_cycle
                WHEN 'monthly'   THEN (mp.price * 100)::BIGINT
                WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                ELSE (mp.price * 100)::BIGINT
            END
        ), 0)::BIGINT as mrr_cents
        FROM user_memberships um
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status IN ('active', 'trialing')
        "#,
    )
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let mrr_cents: i64 = mrr_result.map(|r| r.0).unwrap_or(0);
    let arr_cents: i64 = mrr_cents.saturating_mul(12);

    // New subscriptions this month
    let new_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE starts_at >= date_trunc('month', CURRENT_DATE)"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Cancelled this month
    let cancelled_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE cancelled_at >= date_trunc('month', CURRENT_DATE)"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // ICT 7+ Enterprise: Calculate churn rate
    // Churn = (Cancelled this month / Active at start of month) * 100
    let active_start_of_month: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM user_memberships
        WHERE starts_at < date_trunc('month', CURRENT_DATE)
        AND (cancelled_at IS NULL OR cancelled_at >= date_trunc('month', CURRENT_DATE))
        AND status IN ('active', 'cancelled')
        "#,
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((1,)); // Avoid division by zero

    let churn_rate = if active_start_of_month.0 > 0 {
        (cancelled_this_month.0 as f64 / active_start_of_month.0 as f64) * 100.0
    } else {
        0.0
    };

    // ICT 7+ Enterprise: Calculate Average Lifetime Value (LTV)
    // LTV = Average subscription duration * ARPU (Average Revenue Per User)
    let avg_duration_months: Option<(f64,)> = sqlx::query_as(
        r#"
        SELECT COALESCE(AVG(
            EXTRACT(EPOCH FROM (COALESCE(cancelled_at, NOW()) - starts_at)) / (30.0 * 24.0 * 3600.0)
        ), 0)
        FROM user_memberships
        WHERE starts_at IS NOT NULL
        "#,
    )
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let avg_subscription_months = avg_duration_months.map(|r| r.0).unwrap_or(0.0);
    // ARPU and LTV in integer cents (avg-months × ARPU is fractional → keep float for the
    // ARPU-derived figures, but the inputs and the final reported amounts are cents).
    let arpu_cents: i64 = if active.0 > 0 {
        mrr_cents / active.0
    } else {
        0
    };
    let ltv_cents: i64 = (avg_subscription_months * arpu_cents as f64).round() as i64;

    // Failed payments (subscriptions in grace period or past_due)
    let failed_payments: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'past_due' OR (grace_period_end IS NOT NULL AND grace_period_end > NOW())"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Upcoming renewals (next 7 days)
    let upcoming_renewals: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'active' AND current_period_end BETWEEN NOW() AND NOW() + INTERVAL '7 days'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        // Basic counts
        "total_active": active.0,
        "total_cancelled": cancelled.0,
        "total_expired": expired.0,
        "total_trial": trial.0,
        "total_paused": paused.0,
        "pending_cancel": pending_cancel.0,

        // Revenue metrics — integer cents
        "mrr_cents": mrr_cents,
        "arr_cents": arr_cents,
        "monthly_recurring_revenue_cents": mrr_cents,
        "annual_recurring_revenue_cents": arr_cents,

        // Growth metrics
        "new_this_month": new_this_month.0,
        "cancelled_this_month": cancelled_this_month.0,
        "net_growth": new_this_month.0 - cancelled_this_month.0,

        // Churn metrics
        "churn_rate": (churn_rate * 100.0).round() / 100.0,

        // LTV metrics — integer cents
        "average_lifetime_value_cents": ltv_cents,
        "average_subscription_months": (avg_subscription_months * 10.0).round() / 10.0,
        "arpu_cents": arpu_cents,

        // Health metrics
        "failed_payments": failed_payments.0,
        "upcoming_renewals_7_days": upcoming_renewals.0,

        // Totals
        "total_subscriptions": active.0 + cancelled.0 + expired.0 + trial.0 + paused.0
    })))
}

/// GET /subscriptions/export - Export subscriptions as CSV
/// ICT 7 SECURITY FIX: Added AdminUser authentication - export is admin-only
async fn export_subscriptions(
    State(state): State<AppState>,
    _admin: AdminUser,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::response::Response, (StatusCode, Json<serde_json::Value>)> {
    use axum::http::header;
    use axum::response::IntoResponse;

    let format = params.get("format").map(|s| s.as_str()).unwrap_or("csv");

    // Fetch subscriptions with user info
    #[allow(clippy::type_complexity)]
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

        // ICT 7: Surface a serialization failure as a 500 instead of returning
        // an empty 200 body — silent failure here would mask a real defect.
        let body = serde_json::to_string(&json_data).map_err(|e| {
            tracing::error!("Failed to serialize subscription export JSON: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to encode export"})),
            )
        })?;
        let response = ([(header::CONTENT_TYPE, "application/json")], body);
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

/// Upgrade or downgrade subscription to a new plan
/// ICT 7+ Enterprise: Full proration calculation
async fn change_plan(
    State(state): State<AppState>,
    user: User,
    Path(subscription_id): Path<i64>,
    Json(input): Json<ChangePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current subscription
    let subscription: UserSubscriptionRow = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE id = $1 AND user_id = $2 AND status IN ('active', 'trial')"
    )
    .bind(subscription_id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Active subscription not found"}))))?;

    // Get current plan
    let current_plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1"#,
    )
    .bind(subscription.plan_id)
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
            Json(json!({"error": "Current plan not found"})),
        )
    })?;

    // Get new plan
    let new_plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true"#,
    )
    .bind(input.new_plan_id)
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
            Json(json!({"error": "New plan not found or inactive"})),
        )
    })?;

    if current_plan.id == new_plan.id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Already on this plan"})),
        ));
    }

    let prorate = input.prorate.unwrap_or(true);
    let now = chrono::Utc::now().naive_utc();

    // Calculate proration if enabled — all integer cents
    let proration = if let (true, Some(period_end)) = (prorate, subscription.current_period_end) {
        let period_start = subscription.current_period_start.unwrap_or(now);

        // Days in billing cycle
        let total_days: i64 = (period_end - period_start).num_days().max(1);
        let days_remaining: i64 = (period_end - now).num_days().max(0);

        // credit_cents = (current_plan.price_cents * days_remaining) / total_days
        let credit_cents: i64 =
            (current_plan.price_cents.saturating_mul(days_remaining)) / total_days;
        let new_cost_cents: i64 =
            (new_plan.price_cents.saturating_mul(days_remaining)) / total_days;
        let proration_amount_cents: i64 = new_cost_cents - credit_cents;

        Some(ProrationPreview {
            current_plan_credit_cents: credit_cents,
            new_plan_cost_cents: new_cost_cents,
            proration_amount_cents,
            effective_date: now.format("%Y-%m-%d").to_string(),
            billing_cycle_days_remaining: days_remaining,
        })
    } else {
        None
    };

    let is_upgrade = new_plan.price_cents > current_plan.price_cents;
    let change_type = if is_upgrade { "upgrade" } else { "downgrade" };

    // NF-3 (P0): the previous implementation did ONLY
    // `UPDATE user_memberships SET plan_id` with NO Stripe call — any user
    // could POST their own plan change and receive a higher-tier
    // entitlement with no charge (or keep paying the old price). Money +
    // entitlement integrity break.
    //
    // Correct flow: move the real Stripe billing. The authoritative DB
    // `plan_id` write is performed by the `customer.subscription.updated`
    // webhook after Stripe confirms the change (single source of truth) —
    // we deliberately do NOT write `plan_id` here. A subscription with no
    // linked Stripe billing cannot be changed in place (fail-closed: no
    // free upgrades) and must go through checkout.
    let sub_stripe_id = subscription.stripe_subscription_id.clone();
    let new_price_id = new_plan.stripe_price_id.clone();
    match (sub_stripe_id, new_price_id) {
        (Some(sub_id), Some(price_id)) if !sub_id.is_empty() && !price_id.is_empty() => {
            let behavior = if prorate { "create_prorations" } else { "none" };
            state
                .services
                .stripe
                .migrate_subscription_to_price(&sub_id, &price_id, behavior)
                .await
                .map_err(|e| {
                    tracing::error!(
                        target: "billing",
                        event = "change_plan_stripe_failed",
                        user_id = %user.id,
                        subscription_id = %subscription_id,
                        error = %e,
                        "Stripe plan migration failed"
                    );
                    (
                        StatusCode::BAD_GATEWAY,
                        Json(json!({"error": format!("Stripe plan change failed: {}", e)})),
                    )
                })?;
        }
        _ => {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({
                    "error": "This subscription has no linked Stripe billing; complete the plan change through checkout.",
                    "code": "stripe_subscription_required"
                })),
            ));
        }
    }

    Ok(Json(json!({
        "success": true,
        "pending": true,
        "change_type": change_type,
        "previous_plan": {
            "id": current_plan.id,
            "name": current_plan.name,
            "price_cents": current_plan.price_cents,
            "billing_cycle": current_plan.billing_cycle
        },
        "new_plan": {
            "id": new_plan.id,
            "name": new_plan.name,
            "price_cents": new_plan.price_cents,
            "billing_cycle": new_plan.billing_cycle
        },
        "proration": proration,
        "message": format!(
            "Plan change to {} submitted to Stripe; it takes effect once billing is confirmed",
            new_plan.name
        )
    })))
}

/// Preview proration for a plan change (without making the change)
async fn preview_plan_change(
    State(state): State<AppState>,
    user: User,
    Path(subscription_id): Path<i64>,
    Json(input): Json<ChangePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current subscription
    let subscription: UserSubscriptionRow = sqlx::query_as(
        "SELECT * FROM user_memberships WHERE id = $1 AND user_id = $2 AND status IN ('active', 'trialing')",
    )
    .bind(subscription_id)
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
            Json(json!({"error": "Active subscription not found"})),
        )
    })?;

    // Get current plan
    let current_plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1"#,
    )
    .bind(subscription.plan_id)
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
            Json(json!({"error": "Current plan not found"})),
        )
    })?;

    // Get new plan
    let new_plan: MembershipPlanRow = sqlx::query_as(
        r#"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true"#,
    )
    .bind(input.new_plan_id)
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
            Json(json!({"error": "New plan not found or inactive"})),
        )
    })?;

    let now = chrono::Utc::now().naive_utc();

    let proration = if let Some(period_end) = subscription.current_period_end {
        let period_start = subscription.current_period_start.unwrap_or(now);
        let total_days: i64 = (period_end - period_start).num_days().max(1);
        let days_remaining: i64 = (period_end - now).num_days().max(0);

        let credit_cents: i64 =
            (current_plan.price_cents.saturating_mul(days_remaining)) / total_days;
        let new_cost_cents: i64 =
            (new_plan.price_cents.saturating_mul(days_remaining)) / total_days;
        let proration_amount_cents: i64 = new_cost_cents - credit_cents;

        ProrationPreview {
            current_plan_credit_cents: credit_cents,
            new_plan_cost_cents: new_cost_cents,
            proration_amount_cents,
            effective_date: now.format("%Y-%m-%d").to_string(),
            billing_cycle_days_remaining: days_remaining,
        }
    } else {
        ProrationPreview {
            current_plan_credit_cents: 0,
            new_plan_cost_cents: new_plan.price_cents,
            proration_amount_cents: new_plan.price_cents,
            effective_date: now.format("%Y-%m-%d").to_string(),
            billing_cycle_days_remaining: 0,
        }
    };

    let is_upgrade = new_plan.price_cents > current_plan.price_cents;
    let proration_amount_cents = proration.proration_amount_cents;
    let proration_amount_dollars = proration_amount_cents as f64 / 100.0; // display only

    Ok(Json(json!({
        "current_plan": {
            "id": current_plan.id,
            "name": current_plan.name,
            "price_cents": current_plan.price_cents,
            "billing_cycle": current_plan.billing_cycle
        },
        "new_plan": {
            "id": new_plan.id,
            "name": new_plan.name,
            "price_cents": new_plan.price_cents,
            "billing_cycle": new_plan.billing_cycle
        },
        "is_upgrade": is_upgrade,
        "proration": proration,
        "summary": if proration_amount_cents >= 0 {
            format!("You will be charged ${:.2} today", proration_amount_dollars)
        } else {
            format!("You will receive a ${:.2} credit", proration_amount_dollars.abs())
        }
    })))
}

/// Reactivate a cancelled subscription
async fn reactivate_subscription(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
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

    if subscription.status == "active" && !subscription.cancel_at_period_end.unwrap_or(false) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Subscription is already active"})),
        ));
    }

    // Legitimate path: scheduled cancellation — un-flag via Stripe, then update DB
    if subscription.cancel_at_period_end.unwrap_or(false) {
        let stripe_sub_id = subscription
            .stripe_subscription_id
            .as_deref()
            .ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Subscription has no Stripe ID"})),
                )
            })?;

        let env_scope = state.config.environment.clone();
        let stripe = state
            .services
            .credentials
            .stripe_client(&state.db.pool, &env_scope)
            .await;

        stripe
            .update_subscription(stripe_sub_id, &[("cancel_at_period_end", "false")])
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Stripe error: {}", e)})),
                )
            })?;

        sqlx::query(
            "UPDATE user_memberships SET cancel_at_period_end = false, updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        return Ok(Json(
            json!({"success": true, "message": "Subscription reactivated"}),
        ));
    }

    // Fully cancelled subscription — must re-subscribe through checkout; do NOT grant free access
    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "Subscription has fully ended. Please re-subscribe to restart.",
            "resubscribe": true,
            "plan_id": subscription.plan_id
        })),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// NOTIFICATION SCHEDULING ENDPOINTS - ICT 7 Fix
// ═══════════════════════════════════════════════════════════════════════════

/// Send renewal reminder notifications
/// POST /subscriptions/notifications/renewal-reminders
/// Called by cron job/scheduled task - sends reminders 3 days before renewal
async fn send_renewal_reminders(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Find subscriptions renewing in 3 days that haven't been reminded yet
    #[derive(sqlx::FromRow)]
    struct RenewalSubscription {
        id: i64,
        user_email: String,
        user_name: String,
        plan_name: String,
        plan_price_cents: i64,
        current_period_end: chrono::NaiveDateTime,
    }

    let subscriptions: Vec<RenewalSubscription> = sqlx::query_as(
        r#"
        SELECT
            um.id,
            u.email as user_email,
            COALESCE(u.name, u.email) as user_name,
            mp.name as plan_name,
            (mp.price * 100)::BIGINT as plan_price_cents,
            um.current_period_end
        FROM user_memberships um
        JOIN users u ON um.user_id = u.id
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status = 'active'
        AND um.cancel_at_period_end = false
        AND um.current_period_end BETWEEN NOW() + INTERVAL '2 days' AND NOW() + INTERVAL '4 days'
        AND um.renewal_reminder_sent_at IS NULL
        ORDER BY um.current_period_end ASC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let mut sent_count = 0;
    let mut failed_count = 0;

    for sub in &subscriptions {
        let renewal_date = sub.current_period_end.format("%B %d, %Y").to_string();

        match email_service
            .send_renewal_reminder(
                &sub.user_email,
                &sub.user_name,
                &sub.plan_name,
                sub.plan_price_cents,
                &renewal_date,
            )
            .await
        {
            Ok(_) => {
                // Mark as reminded
                sqlx::query(
                    "UPDATE user_memberships SET renewal_reminder_sent_at = NOW() WHERE id = $1",
                )
                .bind(sub.id)
                .execute(&state.db.pool)
                .await
                .ok();
                sent_count += 1;
            }
            Err(e) => {
                tracing::error!(
                    target: "subscriptions",
                    subscription_id = %sub.id,
                    error = %e,
                    "Failed to send renewal reminder"
                );
                failed_count += 1;
            }
        }
    }

    tracing::info!(
        target: "subscriptions",
        event = "renewal_reminders_sent",
        sent = %sent_count,
        failed = %failed_count,
        total = %subscriptions.len(),
        "Renewal reminder batch completed"
    );

    Ok(Json(json!({
        "success": true,
        "sent": sent_count,
        "failed": failed_count,
        "total_processed": subscriptions.len()
    })))
}

/// Send trial ending notifications
/// POST /subscriptions/notifications/trial-ending
/// Called by cron job/scheduled task - sends notifications 1 day before trial ends
async fn send_trial_ending_notifications(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Find trial subscriptions ending in 1-2 days that haven't been notified
    #[derive(sqlx::FromRow)]
    struct TrialSubscription {
        id: i64,
        user_email: String,
        user_name: String,
        plan_name: String,
        plan_price_cents: i64,
        billing_cycle: String,
        trial_ends_at: chrono::NaiveDateTime,
    }

    let subscriptions: Vec<TrialSubscription> = sqlx::query_as(
        r#"
        SELECT
            um.id,
            u.email as user_email,
            COALESCE(u.name, u.email) as user_name,
            mp.name as plan_name,
            (mp.price * 100)::BIGINT as plan_price_cents,
            mp.billing_cycle,
            um.trial_ends_at
        FROM user_memberships um
        JOIN users u ON um.user_id = u.id
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status IN ('trial', 'trialing')
        AND um.trial_ends_at BETWEEN NOW() AND NOW() + INTERVAL '2 days'
        AND um.trial_ending_reminder_sent_at IS NULL
        ORDER BY um.trial_ends_at ASC
        LIMIT 100
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let mut sent_count = 0;
    let mut failed_count = 0;

    for sub in &subscriptions {
        let trial_end_date = sub.trial_ends_at.format("%B %d, %Y").to_string();

        match email_service
            .send_trial_ending_soon(
                &sub.user_email,
                &sub.user_name,
                &sub.plan_name,
                &trial_end_date,
                sub.plan_price_cents,
                &sub.billing_cycle,
            )
            .await
        {
            Ok(_) => {
                // Mark as notified
                sqlx::query(
                    "UPDATE user_memberships SET trial_ending_reminder_sent_at = NOW() WHERE id = $1"
                )
                .bind(sub.id)
                .execute(&state.db.pool)
                .await
                .ok();
                sent_count += 1;
            }
            Err(e) => {
                tracing::error!(
                    target: "subscriptions",
                    subscription_id = %sub.id,
                    error = %e,
                    "Failed to send trial ending notification"
                );
                failed_count += 1;
            }
        }
    }

    tracing::info!(
        target: "subscriptions",
        event = "trial_ending_notifications_sent",
        sent = %sent_count,
        failed = %failed_count,
        total = %subscriptions.len(),
        "Trial ending notification batch completed"
    );

    Ok(Json(json!({
        "success": true,
        "sent": sent_count,
        "failed": failed_count,
        "total_processed": subscriptions.len()
    })))
}

/// Send cancellation confirmation email
/// POST /subscriptions/:id/send-cancellation-email
async fn send_cancellation_email(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Get subscription details
    #[derive(sqlx::FromRow)]
    struct CancelledSubscription {
        plan_name: String,
        current_period_end: Option<chrono::NaiveDateTime>,
        cancel_at_period_end: Option<bool>,
        status: String,
    }

    let sub: CancelledSubscription = sqlx::query_as(
        r#"
        SELECT mp.name as plan_name, um.current_period_end, um.cancel_at_period_end, um.status
        FROM user_memberships um
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.id = $1 AND um.user_id = $2
        "#,
    )
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

    let user_name = if user.name.is_empty() {
        user.email.clone()
    } else {
        user.name.clone()
    };

    // Determine if cancelled immediately or at period end
    let cancel_immediately =
        sub.status == "cancelled" && !sub.cancel_at_period_end.unwrap_or(false);

    let access_end = if cancel_immediately {
        "immediately".to_string()
    } else {
        sub.current_period_end
            .map(|d| d.format("%B %d, %Y").to_string())
            .unwrap_or_else(|| "immediately".to_string())
    };

    email_service
        .send_subscription_cancelled(
            &user.email,
            &user_name,
            &sub.plan_name,
            &access_end,
            cancel_immediately,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Cancellation email sent"
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/plans", get(list_plans))
        .route("/plans/:slug", get(get_plan))
        .route("/room/:room_slug/plans", get(get_room_plans))
        .route("/my", get(get_my_subscriptions))
        .route("/my/active", get(get_active_subscription))
        .route("/export", get(export_subscriptions))
        .route("/", post(create_subscription))
        .route("/:id/cancel", post(cancel_subscription))
        .route("/:id/change-plan", post(change_plan))
        .route("/:id/preview-change", post(preview_plan_change))
        .route("/:id/reactivate", post(reactivate_subscription))
        .route(
            "/:id/send-cancellation-email",
            post(send_cancellation_email),
        )
        .route("/metrics", get(get_metrics))
        // ICT 7 Fix: Notification scheduling endpoints (admin-only, for cron jobs)
        .route(
            "/notifications/renewal-reminders",
            post(send_renewal_reminders),
        )
        .route(
            "/notifications/trial-ending",
            post(send_trial_ending_notifications),
        )
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
