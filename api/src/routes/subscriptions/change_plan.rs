//! Plan-change handlers — upgrade/downgrade + proration preview.
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

use super::dtos::{ChangePlanRequest, MembershipPlanRow, ProrationPreview, UserSubscriptionRow};
use crate::{models::User, AppState};

/// Upgrade or downgrade subscription to a new plan
/// ICT 7+ Enterprise: Full proration calculation
pub(super) async fn change_plan(
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
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1",
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
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true",
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
pub(super) async fn preview_plan_change(
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
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1",
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
        r"SELECT id, name, slug, description, (price * 100)::BIGINT AS price_cents, billing_cycle,
           is_active, metadata, stripe_price_id, features, trial_days, created_at, updated_at
           FROM membership_plans WHERE id = $1 AND is_active = true",
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
            format!("You will be charged ${proration_amount_dollars:.2} today")
        } else {
            format!("You will receive a ${:.2} credit", proration_amount_dollars.abs())
        }
    })))
}
