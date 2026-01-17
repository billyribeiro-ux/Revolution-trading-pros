//! Subscriptions Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full admin CRUD for subscriptions and subscription plans.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// DATA TYPES - SUBSCRIPTIONS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionRow {
    pub id: i64,
    pub user_id: i64,
    pub plan_id: Option<i64>,
    pub status: String,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub cancelled_at: Option<chrono::NaiveDateTime>,
    pub payment_provider: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionPlanRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub price: f64,
    pub billing_cycle: String,
    pub is_active: bool,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub user_id: Option<i64>,
    pub plan_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PlanListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: i64,
    pub plan_id: i64,
    pub status: Option<String>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubscriptionRequest {
    pub status: Option<String>,
    pub plan_id: Option<i64>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub billing_cycle: String,
    pub is_active: Option<bool>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub billing_cycle: Option<String>,
    pub is_active: Option<bool>,
    pub stripe_price_id: Option<String>,
    pub features: Option<serde_json::Value>,
    pub trial_days: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════════════
// SUBSCRIPTION HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// List all subscriptions (admin)
async fn list_subscriptions(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<SubscriptionListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let subscriptions: Vec<SubscriptionRow> = sqlx::query_as(
        r#"
        SELECT id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
               payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        FROM user_memberships
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::bigint IS NULL OR user_id = $2)
          AND ($3::bigint IS NULL OR plan_id = $3)
        ORDER BY created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(query.status.as_deref())
    .bind(query.user_id)
    .bind(query.plan_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_subscriptions: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM user_memberships
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::bigint IS NULL OR user_id = $2)
          AND ($3::bigint IS NULL OR plan_id = $3)
        "#,
    )
    .bind(query.status.as_deref())
    .bind(query.user_id)
    .bind(query.plan_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "data": subscriptions,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single subscription (admin)
async fn get_subscription(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        SELECT id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
               payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        FROM user_memberships WHERE id = $1
        "#,
    )
    .bind(id)
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

    Ok(Json(json!({"data": subscription})))
}

/// Create subscription (admin)
async fn create_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_create",
        admin_id = %user.id,
        user_id = %input.user_id,
        plan_id = %input.plan_id,
        "Admin creating subscription"
    );

    let status = input.status.as_deref().unwrap_or("active");

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        INSERT INTO user_memberships (user_id, plan_id, status, starts_at, expires_at, created_at, updated_at)
        VALUES ($1, $2, $3, COALESCE($4::timestamp, NOW()), $5::timestamp, NOW(), NOW())
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(input.user_id)
    .bind(input.plan_id)
    .bind(status)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create subscription: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription created successfully"}),
    ))
}

/// Update subscription (admin)
async fn update_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSubscriptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_update",
        admin_id = %user.id,
        subscription_id = %id,
        "Admin updating subscription"
    );

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        UPDATE user_memberships SET
            status = COALESCE($2, status),
            plan_id = COALESCE($3, plan_id),
            starts_at = COALESCE($4::timestamp, starts_at),
            expires_at = COALESCE($5::timestamp, expires_at),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(&input.status)
    .bind(input.plan_id)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription updated successfully"}),
    ))
}

/// Delete subscription (admin)
async fn delete_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "subscription_delete",
        admin_id = %user.id,
        subscription_id = %id,
        "Admin deleting subscription"
    );

    let result = sqlx::query("DELETE FROM user_memberships WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Subscription not found"})),
        ));
    }

    Ok(Json(
        json!({"message": "Subscription deleted successfully"}),
    ))
}

/// Cancel subscription (admin)
async fn cancel_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_cancel", admin_id = %user.id, subscription_id = %id, "Cancelling subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription cancelled"}),
    ))
}

/// Pause subscription (admin)
async fn pause_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_pause", admin_id = %user.id, subscription_id = %id, "Pausing subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        UPDATE user_memberships SET status = 'paused', updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription paused"}),
    ))
}

/// Resume subscription (admin)
async fn resume_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_resume", admin_id = %user.id, subscription_id = %id, "Resuming subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        UPDATE user_memberships SET status = 'active', updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription resumed"}),
    ))
}

/// Renew subscription (admin)
async fn renew_subscription(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "subscription_renew", admin_id = %user.id, subscription_id = %id, "Renewing subscription");

    let subscription: SubscriptionRow = sqlx::query_as(
        r#"
        UPDATE user_memberships SET 
            status = 'active', 
            starts_at = NOW(),
            expires_at = NOW() + INTERVAL '1 month',
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, plan_id, status, starts_at, expires_at, cancelled_at,
                  payment_provider, stripe_subscription_id, stripe_customer_id, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Subscription not found"}))))?;

    Ok(Json(
        json!({"data": subscription, "message": "Subscription renewed"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// SUBSCRIPTION PLAN HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// List all subscription plans (admin)
async fn list_plans(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<PlanListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let plans: Vec<SubscriptionPlanRow> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, price, billing_cycle, is_active,
               stripe_price_id, features, trial_days, created_at, updated_at
        FROM membership_plans
        WHERE ($1::boolean IS NULL OR is_active = $1)
        ORDER BY price ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(query.is_active)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_plans: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM membership_plans WHERE ($1::boolean IS NULL OR is_active = $1)",
    )
    .bind(query.is_active)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "data": plans,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single plan (admin)
async fn get_plan(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let plan: SubscriptionPlanRow = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, price, billing_cycle, is_active,
               stripe_price_id, features, trial_days, created_at, updated_at
        FROM membership_plans WHERE id = $1
        "#,
    )
    .bind(id)
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

    Ok(Json(json!({"data": plan})))
}

/// Create subscription plan (admin)
async fn create_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreatePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_create",
        admin_id = %user.id,
        plan_name = %input.name,
        "Admin creating subscription plan"
    );

    let slug = slug::slugify(&input.name);
    let features = input.features.unwrap_or(json!([]));

    let plan: SubscriptionPlanRow = sqlx::query_as(
        r#"
        INSERT INTO membership_plans (name, slug, description, price, billing_cycle, is_active, stripe_price_id, features, trial_days, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
        RETURNING id, name, slug, description, price, billing_cycle, is_active, stripe_price_id, features, trial_days, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(input.price)
    .bind(&input.billing_cycle)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.stripe_price_id)
    .bind(&features)
    .bind(input.trial_days)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create plan: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(
        json!({"data": plan, "message": "Plan created successfully"}),
    ))
}

/// Update subscription plan (admin)
async fn update_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePlanRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_update",
        admin_id = %user.id,
        plan_id = %id,
        "Admin updating subscription plan"
    );

    let plan: SubscriptionPlanRow = sqlx::query_as(
        r#"
        UPDATE membership_plans SET
            name = COALESCE($2, name),
            slug = CASE WHEN $2 IS NOT NULL THEN LOWER(REPLACE($2, ' ', '-')) ELSE slug END,
            description = COALESCE($3, description),
            price = COALESCE($4, price),
            billing_cycle = COALESCE($5, billing_cycle),
            is_active = COALESCE($6, is_active),
            stripe_price_id = COALESCE($7, stripe_price_id),
            features = COALESCE($8, features),
            trial_days = COALESCE($9, trial_days),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, price, billing_cycle, is_active, stripe_price_id, features, trial_days, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.price)
    .bind(&input.billing_cycle)
    .bind(input.is_active)
    .bind(&input.stripe_price_id)
    .bind(&input.features)
    .bind(input.trial_days)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Plan not found"}))))?;

    Ok(Json(
        json!({"data": plan, "message": "Plan updated successfully"}),
    ))
}

/// Delete subscription plan (admin)
async fn delete_plan(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "plan_delete",
        admin_id = %user.id,
        plan_id = %id,
        "Admin deleting subscription plan"
    );

    let result = sqlx::query("DELETE FROM membership_plans WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Plan not found"})),
        ));
    }

    Ok(Json(json!({"message": "Plan deleted successfully"})))
}

/// Get plan stats (admin)
async fn plan_stats(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_plans: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM membership_plans")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let active_plans: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM membership_plans WHERE is_active = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_subscriptions: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "data": {
            "total_plans": total_plans.0,
            "active_plans": active_plans.0,
            "total_active_subscriptions": total_subscriptions.0
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

/// Subscriptions admin router - mounted at /admin/subscriptions
pub fn subscriptions_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_subscriptions).post(create_subscription))
        .route(
            "/:id",
            get(get_subscription)
                .put(update_subscription)
                .delete(delete_subscription),
        )
        .route("/:id/cancel", post(cancel_subscription))
        .route("/:id/pause", post(pause_subscription))
        .route("/:id/resume", post(resume_subscription))
        .route("/:id/renew", post(renew_subscription))
}

/// Subscription plans admin router - mounted at /admin/subscriptions/plans
pub fn plans_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_plans).post(create_plan))
        .route("/stats", get(plan_stats))
        .route("/:id", get(get_plan).put(update_plan).delete(delete_plan))
}
