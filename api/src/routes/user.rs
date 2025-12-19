//! User routes (singular) - Revolution Trading Pros
//! Routes for /api/user/* endpoints expected by frontend
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
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
}

/// Get user's memberships/subscriptions
/// Matches frontend expectation: GET /api/user/memberships
async fn get_memberships(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<MembershipsResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Try to fetch subscriptions - handle case where table doesn't exist
    let subscriptions_result: Result<Vec<UserSubscriptionDbRow>, _> = sqlx::query_as(
        r#"
        SELECT s.id, s.user_id, s.plan_id, s.starts_at, s.expires_at, s.status, s.created_at
        FROM user_subscriptions s
        WHERE s.user_id = $1 AND s.status IN ('active', 'trialing')
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
            "SELECT id, name, slug, price, billing_cycle FROM membership_plans WHERE id = $1"
        )
        .bind(sub.plan_id)
        .fetch_optional(&state.db.pool)
        .await
        .unwrap_or(None);

        if let Some(plan) = plan {
            let is_trial = sub.status == "trialing";
            let membership = UserMembershipResponse {
                id: sub.id.to_string(),
                name: plan.name,
                membership_type: "subscription".to_string(),
                slug: plan.slug,
                status: if is_trial { "active".to_string() } else { sub.status },
                subscription_type: Some(if is_trial { "trial" } else { "active" }.to_string()),
                icon: None,
                start_date: sub.starts_at.format("%Y-%m-%d").to_string(),
                next_billing_date: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                expires_at: sub.expires_at.map(|d| d.format("%Y-%m-%d").to_string()),
                price: Some(plan.price),
                interval: Some(plan.billing_cycle),
                features: None,
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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/memberships", get(get_memberships))
        .route("/profile", get(get_profile))
}

