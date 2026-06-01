//! User-scoped subscription read endpoints (`/my`, `/my/active`,
//! `/my/subscriptions/:id`).
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs` as a pure structural move. All SQL, error
//! mapping, and money handling preserved byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dtos::{UserSubscriptionRow, UserSubscriptionWithPlanRow};
use crate::{models::User, AppState};

/// Get user's subscriptions with complete plan details
/// ICT 7+ Fix: Returns full plan information instead of placeholders
pub(super) async fn get_my_subscriptions(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7+ Fix: Join with membership_plans to get full plan details
    let subscriptions: Vec<UserSubscriptionWithPlanRow> = sqlx::query_as(
        r"
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
        ",
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
pub(super) async fn get_active_subscription(
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

/// Get single subscription by ID
pub(super) async fn get_subscription_by_id(
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
