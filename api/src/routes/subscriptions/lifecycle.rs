//! Subscription lifecycle handlers — create, cancel, reactivate.
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs` as a pure structural move. All SQL, error
//! mapping, Stripe call sites, money handling, and the NF-3 (P0)
//! free-upgrade fix are preserved byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dtos::{
    CancelSubscriptionRequest, CreateSubscriptionRequest, MembershipPlanRow, UserSubscriptionRow,
};
use crate::{models::User, AppState};

/// Create subscription (subscribe to plan)
/// ICT 7+ Enterprise: Full trial period support, proper period calculation
pub(super) async fn create_subscription(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get the plan
    // ICT 11+ Fix: Cast DECIMAL price to FLOAT8 for SQLx f64 compatibility
    let plan: MembershipPlanRow = sqlx::query_as(
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true",
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
        let cancel_url = format!("{app_url}/account/subscriptions?status=cancel");

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
        r"
        INSERT INTO user_memberships (
            user_id, plan_id, starts_at, status, payment_provider,
            trial_ends_at, current_period_start, current_period_end,
            cancel_at_period_end, created_at, updated_at
        )
        VALUES ($1, $2, NOW(), $3, 'free', $4, $5, $6, false, NOW(), NOW())
        RETURNING *
        ",
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
            format!("Subscription created with {trial_days}-day trial")
        } else {
            "Subscription created successfully".to_string()
        }
    })))
}

/// Cancel subscription
pub(super) async fn cancel_subscription(
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

/// Reactivate a cancelled subscription
pub(super) async fn reactivate_subscription(
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
