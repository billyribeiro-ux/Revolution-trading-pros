//! User membership handlers — list, detail, cancel.
//!
//! R26-B split: handlers moved verbatim from `routes/user.rs`. SQL,
//! tracing, error mappings, and the `(price * 100)::BIGINT` cents
//! conversion at the SQL edge are preserved byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{models::User, AppState};

use super::dtos::{
    CancelSubscriptionRequest, MembershipPlanDbRow, MembershipsResponse, UserMembershipResponse,
    UserSubscriptionDbRow,
};

/// Get user's memberships/subscriptions
/// Matches frontend expectation: GET /api/user/memberships
pub(super) async fn get_memberships(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<MembershipsResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Try to fetch subscriptions - handle case where table doesn't exist
    // Note: Using user_memberships table (not user_subscriptions)
    let subscriptions_result: Result<Vec<UserSubscriptionDbRow>, _> = sqlx::query_as(
        r"
        SELECT s.id, s.user_id, s.plan_id, s.starts_at, s.expires_at, s.status, s.created_at
        FROM user_memberships s
        WHERE s.user_id = $1 AND s.status IN ('active', 'trialing', 'pending')
        ORDER BY s.created_at DESC
        ",
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await;

    // If table doesn't exist or query fails, return empty list
    let subscriptions = match subscriptions_result {
        Ok(subs) => subs,
        Err(e) => {
            tracing::warn!("Failed to fetch subscriptions (table may not exist): {}", e);
            Vec::new()
        }
    };

    // Fetch plan details for each subscription
    let mut memberships = Vec::new();
    for sub in subscriptions {
        // Money is integer cents at the Rust boundary; convert at the SQL edge.
        let plan: Option<MembershipPlanDbRow> = sqlx::query_as(
            "SELECT id, name, slug, (price * 100)::BIGINT AS price_cents, billing_cycle, metadata, features FROM membership_plans WHERE id = $1"
        )
        .bind(sub.plan_id)
        .fetch_optional(&state.db.pool)
        .await
        .unwrap_or(None);

        if let Some(plan) = plan {
            let is_trial = sub.status == "trialing";

            // Extract type from metadata (default to "trading-room")
            let membership_type = plan
                .metadata
                .as_ref()
                .and_then(|m| m.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("trading-room")
                .to_string();

            // Extract icon from metadata
            let icon = plan
                .metadata
                .as_ref()
                .and_then(|m| m.get("icon"))
                .and_then(|i| i.as_str())
                .map(|s| s.to_string());

            // Extract features array
            let features = plan
                .features
                .as_ref()
                .and_then(|f| f.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<String>>()
                });

            let membership = UserMembershipResponse {
                id: sub.id.to_string(),
                name: plan.name,
                membership_type,
                slug: plan.slug,
                status: if is_trial {
                    "active".to_string()
                } else {
                    sub.status
                },
                subscription_type: Some(if is_trial { "trial" } else { "active" }.to_string()),
                icon,
                start_date: sub.starts_at.format("%Y-%m-%d").to_string(),
                next_billing_date: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                expires_at: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                price_cents: Some(plan.price_cents),
                interval: Some(plan.billing_cycle),
                features,
            };
            memberships.push(membership);
        }
    }

    Ok(Json(MembershipsResponse { memberships }))
}

/// Cancel a user's subscription
/// POST /api/user/memberships/:id/cancel
pub(super) async fn cancel_membership(
    State(state): State<AppState>,
    user: User,
    Path(membership_id): Path<i64>,
    Json(input): Json<CancelSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify user owns this membership
    let membership: Option<UserSubscriptionDbRow> = sqlx::query_as(
        "SELECT id, user_id, plan_id, starts_at, expires_at, status, created_at FROM user_memberships WHERE id = $1 AND user_id = $2"
    )
    .bind(membership_id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let membership = membership.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Membership not found"})),
        )
    })?;

    if membership.status != "active" && membership.status != "trialing" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Cannot cancel membership",
                "message": "This membership is not active"
            })),
        ));
    }

    if input.cancel_immediately {
        // Cancel immediately
        sqlx::query(
            "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(membership_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        Ok(Json(json!({
            "success": true,
            "message": "Membership cancelled immediately",
            "status": "cancelled"
        })))
    } else {
        // Cancel at period end
        sqlx::query(
            "UPDATE user_memberships SET cancel_at_period_end = true, updated_at = NOW() WHERE id = $1"
        )
        .bind(membership_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        Ok(Json(json!({
            "success": true,
            "message": "Membership will be cancelled at the end of the billing period",
            "status": "pending_cancellation",
            "cancel_at": membership.expires_at.map(|d| d.format("%Y-%m-%d").to_string())
        })))
    }
}

/// Get single membership details
/// GET /api/user/memberships/:id
pub(super) async fn get_membership_details(
    State(state): State<AppState>,
    user: User,
    Path(membership_id): Path<i64>,
) -> Result<Json<UserMembershipResponse>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: UserSubscriptionDbRow = sqlx::query_as(
        "SELECT id, user_id, plan_id, starts_at, expires_at, status, created_at FROM user_memberships WHERE id = $1 AND user_id = $2"
    )
    .bind(membership_id)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Membership not found"}))))?;

    // Money is integer cents at the Rust boundary; convert at the SQL edge.
    let plan: MembershipPlanDbRow = sqlx::query_as(
        "SELECT id, name, slug, (price * 100)::BIGINT AS price_cents, billing_cycle, metadata, features FROM membership_plans WHERE id = $1"
    )
    .bind(subscription.plan_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Plan not found"}))))?;

    let is_trial = subscription.status == "trialing";

    let membership_type = plan
        .metadata
        .as_ref()
        .and_then(|m| m.get("type"))
        .and_then(|t| t.as_str())
        .unwrap_or("trading-room")
        .to_string();

    let icon = plan
        .metadata
        .as_ref()
        .and_then(|m| m.get("icon"))
        .and_then(|i| i.as_str())
        .map(|s| s.to_string());

    let features = plan
        .features
        .as_ref()
        .and_then(|f| f.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<String>>()
        });

    Ok(Json(UserMembershipResponse {
        id: subscription.id.to_string(),
        name: plan.name,
        membership_type,
        slug: plan.slug,
        status: if is_trial {
            "active".to_string()
        } else {
            subscription.status
        },
        subscription_type: Some(if is_trial { "trial" } else { "active" }.to_string()),
        icon,
        start_date: subscription.starts_at.format("%Y-%m-%d").to_string(),
        next_billing_date: subscription
            .expires_at
            .map(|d| d.format("%Y-%m-%d").to_string()),
        expires_at: subscription
            .expires_at
            .map(|d| d.format("%Y-%m-%d").to_string()),
        price_cents: Some(plan.price_cents),
        interval: Some(plan.billing_cycle),
        features,
    }))
}
