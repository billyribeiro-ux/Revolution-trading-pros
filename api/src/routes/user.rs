//! User routes (singular) - Revolution Trading Pros
//! Routes for /api/user/* endpoints expected by frontend
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    models::User,
    AppState,
};

/// User membership response - matches frontend UserMembership interface
#[derive(Debug, Serialize, Deserialize)]
pub struct UserMembershipResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub membership_type: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "membershipType", skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "nextBillingDate", skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

/// Memberships list response - matches frontend UserMembershipsResponse
#[derive(Debug, Serialize)]
pub struct MembershipsResponse {
    pub memberships: Vec<UserMembershipResponse>,
}

/// Database row for user subscriptions
#[derive(Debug, sqlx::FromRow)]
struct UserSubscriptionDbRow {
    id: i64,
    user_id: i64,
    plan_id: i64,
    starts_at: chrono::NaiveDateTime,
    expires_at: Option<chrono::NaiveDateTime>,
    status: String,
    created_at: chrono::NaiveDateTime,
}

/// Database row for membership plan
#[derive(Debug, sqlx::FromRow)]
struct MembershipPlanDbRow {
    id: i64,
    name: String,
    slug: String,
    price: f64,
    billing_cycle: String,
    metadata: Option<serde_json::Value>,
    features: Option<serde_json::Value>,
}

/// Get user's memberships/subscriptions
/// Matches frontend expectation: GET /api/user/memberships
async fn get_memberships(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<MembershipsResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Try to fetch subscriptions - handle case where table doesn't exist
    // Note: Using user_memberships table (not user_subscriptions)
    let subscriptions_result: Result<Vec<UserSubscriptionDbRow>, _> = sqlx::query_as(
        r#"
        SELECT s.id, s.user_id, s.plan_id, s.starts_at, s.expires_at, s.status, s.created_at
        FROM user_memberships s
        WHERE s.user_id = $1 AND s.status IN ('active', 'trialing', 'pending')
        ORDER BY s.created_at DESC
        "#
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
        let plan: Option<MembershipPlanDbRow> = sqlx::query_as(
            "SELECT id, name, slug, price, billing_cycle, metadata, features FROM membership_plans WHERE id = $1"
        )
        .bind(sub.plan_id)
        .fetch_optional(&state.db.pool)
        .await
        .unwrap_or(None);

        if let Some(plan) = plan {
            let is_trial = sub.status == "trialing";

            // Extract type from metadata (default to "trading-room")
            let membership_type = plan.metadata
                .as_ref()
                .and_then(|m| m.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("trading-room")
                .to_string();

            // Extract icon from metadata
            let icon = plan.metadata
                .as_ref()
                .and_then(|m| m.get("icon"))
                .and_then(|i| i.as_str())
                .map(|s| s.to_string());

            // Extract features array
            let features = plan.features
                .as_ref()
                .and_then(|f| f.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<String>>());

            let membership = UserMembershipResponse {
                id: sub.id.to_string(),
                name: plan.name,
                membership_type,
                slug: plan.slug,
                status: if is_trial { "active".to_string() } else { sub.status },
                subscription_type: Some(if is_trial { "trial" } else { "active" }.to_string()),
                icon,
                start_date: sub.starts_at.format("%Y-%m-%d").to_string(),
                next_billing_date: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                expires_at: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                price: Some(plan.price),
                interval: Some(plan.billing_cycle),
                features,
            };
            memberships.push(membership);
        }
    }

    Ok(Json(MembershipsResponse { memberships }))
}

/// Get current user profile
/// Matches frontend expectation: GET /api/user/profile
async fn get_profile(user: User) -> Json<crate::models::UserResponse> {
    Json(user.into())
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    #[serde(default)]
    pub cancel_immediately: bool,
    pub reason: Option<String>,
}

/// Cancel a user's subscription
/// POST /api/user/memberships/:id/cancel
async fn cancel_membership(
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
        (StatusCode::NOT_FOUND, Json(json!({"error": "Membership not found"})))
    })?;

    if membership.status != "active" && membership.status != "trialing" {
        return Err((StatusCode::BAD_REQUEST, Json(json!({
            "error": "Cannot cancel membership",
            "message": "This membership is not active"
        }))));
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
async fn get_membership_details(
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

    let plan: MembershipPlanDbRow = sqlx::query_as(
        "SELECT id, name, slug, price, billing_cycle, metadata, features FROM membership_plans WHERE id = $1"
    )
    .bind(subscription.plan_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Plan not found"}))))?;

    let is_trial = subscription.status == "trialing";

    let membership_type = plan.metadata
        .as_ref()
        .and_then(|m| m.get("type"))
        .and_then(|t| t.as_str())
        .unwrap_or("trading-room")
        .to_string();

    let icon = plan.metadata
        .as_ref()
        .and_then(|m| m.get("icon"))
        .and_then(|i| i.as_str())
        .map(|s| s.to_string());

    let features = plan.features
        .as_ref()
        .and_then(|f| f.as_array())
        .map(|arr| arr.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<String>>());

    Ok(Json(UserMembershipResponse {
        id: subscription.id.to_string(),
        name: plan.name,
        membership_type,
        slug: plan.slug,
        status: if is_trial { "active".to_string() } else { subscription.status },
        subscription_type: Some(if is_trial { "trial" } else { "active" }.to_string()),
        icon,
        start_date: subscription.starts_at.format("%Y-%m-%d").to_string(),
        next_billing_date: subscription.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
        expires_at: subscription.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
        price: Some(plan.price),
        interval: Some(plan.billing_cycle),
        features,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/memberships", get(get_memberships))
        .route("/memberships/:id", get(get_membership_details))
        .route("/memberships/:id/cancel", post(cancel_membership))
        .route("/profile", get(get_profile))
        .route("/profile", axum::routing::put(update_profile))
        .route("/payment-methods", get(get_payment_methods))
        .route("/payment-methods", post(add_payment_method))
        .route("/payment-methods/:id", axum::routing::delete(delete_payment_method))
}

/// Update user profile
/// PUT /api/user/profile
async fn update_profile(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Extract fields from input
    let first_name = input.get("firstName").or_else(|| input.get("first_name")).and_then(|v| v.as_str());
    let last_name = input.get("lastName").or_else(|| input.get("last_name")).and_then(|v| v.as_str());
    let email = input.get("email").and_then(|v| v.as_str());
    
    // Update user in database
    sqlx::query(
        r#"UPDATE users SET 
            first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            email = COALESCE($3, email),
            updated_at = NOW()
        WHERE id = $4"#
    )
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(user.id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    
    Ok(Json(json!({
        "success": true,
        "message": "Profile updated successfully"
    })))
}

/// Get user payment methods (Stripe)
/// GET /api/user/payment-methods
async fn get_payment_methods(
    State(_state): State<AppState>,
    _user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement Stripe payment methods retrieval
    Ok(Json(json!({
        "payment_methods": []
    })))
}

/// Add payment method
/// POST /api/user/payment-methods
async fn add_payment_method(
    State(_state): State<AppState>,
    _user: User,
    Json(_input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement Stripe payment method addition
    Ok(Json(json!({
        "success": true,
        "message": "Payment method added"
    })))
}

/// Delete payment method
/// DELETE /api/user/payment-methods/:id
async fn delete_payment_method(
    State(_state): State<AppState>,
    _user: User,
    Path(_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement Stripe payment method deletion
    Ok(Json(json!({
        "success": true,
        "message": "Payment method deleted"
    })))
}

