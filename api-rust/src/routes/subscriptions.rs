//! Subscription routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::subscription::{
    UserSubscription, SubscriptionWithPlan, CreateSubscriptionRequest, CancelSubscriptionRequest
};
use crate::services::JwtService;

/// GET /api/my/subscriptions - List user's subscriptions
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;

    let subscriptions: Vec<SubscriptionWithPlan> = ctx.data.db.query(
        r#"
        SELECT 
            us.*,
            json_build_object(
                'id', sp.id,
                'name', sp.name,
                'price', sp.price,
                'interval', sp.interval,
                'features', sp.features
            ) as plan,
            p.name as product_name
        FROM user_subscriptions us
        JOIN subscription_plans sp ON sp.id = us.plan_id
        JOIN products p ON p.id = us.product_id
        WHERE us.user_id = $1
        ORDER BY us.created_at DESC
        "#,
        vec![serde_json::json!(user.id.to_string())]
    ).await?;

    Response::from_json(&subscriptions)
}

/// POST /api/my/subscriptions - Create a new subscription
pub async fn create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    
    let body: CreateSubscriptionRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    // Get the plan
    let plan: crate::models::subscription::SubscriptionPlan = ctx.data.db.query_one(
        "SELECT * FROM subscription_plans WHERE id = $1 AND is_active = true",
        vec![serde_json::json!(body.plan_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::NotFound("Subscription plan not found".to_string()))?;

    // Check if user already has an active subscription for this product
    let existing: Option<UserSubscription> = ctx.data.db.query_one(
        r#"
        SELECT * FROM user_subscriptions 
        WHERE user_id = $1 AND product_id = $2 AND status IN ('active', 'trialing')
        "#,
        vec![
            serde_json::json!(user.id.to_string()),
            serde_json::json!(plan.product_id.to_string()),
        ]
    ).await?;

    if existing.is_some() {
        return Err(ApiError::Conflict("You already have an active subscription for this product".to_string()).into());
    }

    // Create Stripe subscription if payment method provided
    let stripe_subscription = if let Some(_payment_method_id) = &body.payment_method_id {
        // Get or create Stripe customer
        let customer_id = get_or_create_stripe_customer(&ctx, &user).await?;
        
        // Create subscription in Stripe
        let stripe = &ctx.data.services.stripe;
        Some(stripe.create_subscription(
            &customer_id,
            plan.stripe_price_id.as_deref().unwrap_or(""),
            plan.trial_days.map(|d| d as i32),
        ).await?)
    } else {
        None
    };

    let now = crate::utils::now();
    let subscription_id = uuid::Uuid::new_v4();
    
    let period_end = if let Some(trial_days) = plan.trial_days {
        now + chrono::Duration::days(trial_days as i64)
    } else {
        match plan.interval {
            crate::models::subscription::BillingInterval::Day => now + chrono::Duration::days(plan.interval_count as i64),
            crate::models::subscription::BillingInterval::Week => now + chrono::Duration::weeks(plan.interval_count as i64),
            crate::models::subscription::BillingInterval::Month => now + chrono::Duration::days(30 * plan.interval_count as i64),
            crate::models::subscription::BillingInterval::Year => now + chrono::Duration::days(365 * plan.interval_count as i64),
        }
    };

    // Create subscription in database
    ctx.data.db.execute(
        r#"
        INSERT INTO user_subscriptions (
            id, user_id, plan_id, product_id, status,
            stripe_subscription_id, stripe_customer_id,
            current_period_start, current_period_end,
            trial_ends_at, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
        "#,
        vec![
            serde_json::json!(subscription_id.to_string()),
            serde_json::json!(user.id.to_string()),
            serde_json::json!(plan.id.to_string()),
            serde_json::json!(plan.product_id.to_string()),
            serde_json::json!(if plan.trial_days.is_some() { "trialing" } else { "active" }),
            serde_json::json!(stripe_subscription.as_ref().map(|s| &s.id)),
            serde_json::json!(stripe_subscription.as_ref().map(|s| &s.customer)),
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(period_end.to_rfc3339()),
            serde_json::json!(plan.trial_days.map(|d| (now + chrono::Duration::days(d as i64)).to_rfc3339())),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Grant product access
    ctx.data.db.execute(
        r#"
        INSERT INTO product_user (id, user_id, product_id, status, starts_at, created_at)
        VALUES ($1, $2, $3, 'active', $4, $4)
        ON CONFLICT (user_id, product_id) DO UPDATE SET status = 'active', updated_at = $4
        "#,
        vec![
            serde_json::json!(uuid::Uuid::new_v4().to_string()),
            serde_json::json!(user.id.to_string()),
            serde_json::json!(plan.product_id.to_string()),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Return created subscription
    let subscription: SubscriptionWithPlan = ctx.data.db.query_one(
        r#"
        SELECT us.*, 
               json_build_object('id', sp.id, 'name', sp.name, 'price', sp.price) as plan,
               p.name as product_name
        FROM user_subscriptions us
        JOIN subscription_plans sp ON sp.id = us.plan_id
        JOIN products p ON p.id = us.product_id
        WHERE us.id = $1
        "#,
        vec![serde_json::json!(subscription_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created subscription".to_string()))?;

    Response::from_json(&subscription).map(|r| r.with_status(201))
}

/// GET /api/my/subscriptions/:id - Get subscription details
pub async fn show(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing subscription id".to_string()))?;

    let subscription: Option<SubscriptionWithPlan> = ctx.data.db.query_one(
        r#"
        SELECT us.*, 
               json_build_object('id', sp.id, 'name', sp.name, 'price', sp.price, 'features', sp.features) as plan,
               p.name as product_name
        FROM user_subscriptions us
        JOIN subscription_plans sp ON sp.id = us.plan_id
        JOIN products p ON p.id = us.product_id
        WHERE us.id = $1 AND us.user_id = $2
        "#,
        vec![serde_json::json!(id), serde_json::json!(user.id.to_string())]
    ).await?;

    match subscription {
        Some(s) => Response::from_json(&s),
        None => Err(ApiError::NotFound("Subscription not found".to_string()).into())
    }
}

/// POST /api/my/subscriptions/:id/cancel - Cancel subscription
pub async fn cancel(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing subscription id".to_string()))?;

    let body: CancelSubscriptionRequest = req.json().await.unwrap_or(CancelSubscriptionRequest {
        reason: None,
        feedback: None,
        cancel_immediately: None,
    });

    // Get subscription
    let subscription: UserSubscription = ctx.data.db.query_one(
        "SELECT * FROM user_subscriptions WHERE id = $1 AND user_id = $2",
        vec![serde_json::json!(id), serde_json::json!(user.id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::NotFound("Subscription not found".to_string()))?;

    let now = crate::utils::now();
    let cancel_immediately = body.cancel_immediately.unwrap_or(false);

    // Cancel in Stripe if applicable
    if let Some(stripe_sub_id) = &subscription.stripe_subscription_id {
        let stripe = &ctx.data.services.stripe;
        stripe.cancel_subscription(stripe_sub_id, !cancel_immediately).await?;
    }

    // Update subscription
    if cancel_immediately {
        ctx.data.db.execute(
            r#"
            UPDATE user_subscriptions 
            SET status = 'cancelled', cancelled_at = $1, ended_at = $1, updated_at = $1
            WHERE id = $2
            "#,
            vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(id)]
        ).await?;

        // Revoke product access
        ctx.data.db.execute(
            "UPDATE product_user SET status = 'cancelled', updated_at = $1 WHERE user_id = $2 AND product_id = $3",
            vec![
                serde_json::json!(now.to_rfc3339()),
                serde_json::json!(user.id.to_string()),
                serde_json::json!(subscription.product_id.to_string()),
            ]
        ).await?;
    } else {
        ctx.data.db.execute(
            r#"
            UPDATE user_subscriptions 
            SET cancel_at_period_end = true, cancelled_at = $1, updated_at = $1
            WHERE id = $2
            "#,
            vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(id)]
        ).await?;
    }

    Response::from_json(&serde_json::json!({
        "message": if cancel_immediately { 
            "Subscription cancelled immediately" 
        } else { 
            "Subscription will be cancelled at the end of the billing period" 
        }
    }))
}

/// POST /api/my/subscriptions/:id/pause - Pause subscription
pub async fn pause(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing subscription id".to_string()))?;

    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        UPDATE user_subscriptions 
        SET status = 'paused', paused_at = $1, updated_at = $1
        WHERE id = $2 AND user_id = $3 AND status = 'active'
        "#,
        vec![
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(id),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "Subscription paused"
    }))
}

/// POST /api/my/subscriptions/:id/resume - Resume paused subscription
pub async fn resume(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing subscription id".to_string()))?;

    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        UPDATE user_subscriptions 
        SET status = 'active', paused_at = NULL, resume_at = NULL, updated_at = $1
        WHERE id = $2 AND user_id = $3 AND status = 'paused'
        "#,
        vec![
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(id),
            serde_json::json!(user.id.to_string()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "Subscription resumed"
    }))
}

/// Helper to get or create Stripe customer
async fn get_or_create_stripe_customer(
    ctx: &RouteContext<AppState>,
    user: &crate::models::User,
) -> Result<String, ApiError> {
    // Check if user already has a Stripe customer ID
    if let Some(customer_id) = ctx.data.db.query_one::<CustomerIdResult>(
        "SELECT stripe_customer_id FROM user_subscriptions WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1",
        vec![serde_json::json!(user.id.to_string())]
    ).await?.map(|r| r.stripe_customer_id) {
        return Ok(customer_id);
    }

    // Create new Stripe customer
    let stripe = &ctx.data.services.stripe;
    let customer = stripe.create_customer(
        &user.email,
        Some(&user.name),
        Some(serde_json::json!({"user_id": user.id.to_string()})),
    ).await?;

    Ok(customer.id)
}

#[derive(serde::Deserialize)]
struct CustomerIdResult {
    stripe_customer_id: String,
}

/// Helper to get authenticated user
async fn get_authenticated_user(
    req: &Request,
    ctx: &RouteContext<AppState>,
) -> Result<crate::models::User, ApiError> {
    let auth_header = req.headers().get("Authorization")
        .map_err(|_| ApiError::Unauthorized("Missing authorization header".to_string()))?
        .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?;

    let token = JwtService::extract_token(&auth_header)
        .ok_or_else(|| ApiError::Unauthorized("Invalid authorization header".to_string()))?;

    let jwt = JwtService::new(
        &ctx.data.config.jwt_secret,
        &ctx.data.config.jwt_issuer,
        &ctx.data.config.jwt_audience,
    );

    let claims = jwt.validate_access_token(token)?;

    ctx.data.db.query_one::<crate::models::User>(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(claims.sub)]
    ).await
    .map_err(|e| ApiError::Database(e.to_string()))?
    .ok_or_else(|| ApiError::Unauthorized("User not found".to_string()))
}
