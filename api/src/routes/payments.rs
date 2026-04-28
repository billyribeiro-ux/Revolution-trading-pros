//! Payment Routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Complete Stripe payment integration:
//! - Checkout session creation
//! - Customer portal access
//! - Webhook handling with order/subscription updates
//! - Refund processing

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::{
    middleware::admin::AdminUser,
    models::User,
    services::stripe::{CheckoutConfig, LineItem},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════
// Request/Response Types
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct CreateCheckoutRequest {
    pub items: Vec<CheckoutItem>,
    // success_path / cancel_path must start with "/" — URLs are built server-side from app_url
    pub success_path: Option<String>,
    pub cancel_path: Option<String>,
    pub coupon_code: Option<String>,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CheckoutItem {
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    // price is intentionally NOT accepted from the client; it is looked up from the DB
    pub quantity: i32,
    pub is_subscription: bool,
    pub interval: Option<String>,
}

#[derive(Serialize)]
pub struct CheckoutResponse {
    pub session_id: String,
    pub url: String,
    pub order_id: i64,
    pub order_number: String,
}

#[derive(Deserialize)]
pub struct CreatePortalRequest {
    pub return_url: String,
}

#[derive(Serialize)]
pub struct PortalResponse {
    pub url: String,
}

#[derive(Deserialize)]
pub struct RefundRequest {
    pub order_id: i64,
    pub amount: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub amount: i64,
    pub status: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Route Handlers
// ═══════════════════════════════════════════════════════════════════════════

/// Create Stripe checkout session
/// POST /api/payments/checkout
async fn create_checkout(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateCheckoutRequest>,
) -> Result<Json<CheckoutResponse>, (StatusCode, Json<serde_json::Value>)> {
    if input.items.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No items in checkout"})),
        ));
    }

    // Look up authoritative prices from DB — never trust client-supplied price
    #[derive(sqlx::FromRow)]
    struct PlanRow {
        name: String,
        price: f64,
    }
    #[derive(sqlx::FromRow)]
    struct ProductRow {
        name: String,
        price: f64,
    }

    // resolved_items: (product_id, plan_id, db_name, db_price, quantity, is_subscription, interval)
    struct ResolvedItem {
        product_id: Option<i64>,
        plan_id: Option<i64>,
        db_name: String,
        db_price: f64,
        quantity: i32,
        is_subscription: bool,
        interval: Option<String>,
    }

    let mut resolved_items: Vec<ResolvedItem> = Vec::new();
    let mut subtotal = 0.0_f64;
    let mut line_items: Vec<LineItem> = Vec::new();

    for item in &input.items {
        let (db_name, db_price): (String, f64) = if let Some(plan_id) = item.plan_id {
            let row: PlanRow = sqlx::query_as(
                "SELECT name, price FROM membership_plans WHERE id = $1 AND is_active = true",
            )
            .bind(plan_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
            .ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Plan {} not found or inactive", plan_id)}))))?;
            (row.name, row.price)
        } else if let Some(product_id) = item.product_id {
            let row: ProductRow = sqlx::query_as(
                "SELECT name, price FROM products WHERE id = $1 AND is_active = true",
            )
            .bind(product_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
            .ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({"error": format!("Product {} not found or inactive", product_id)}))))?;
            (row.name, row.price)
        } else {
            return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Each item must have either plan_id or product_id"}))));
        };

        subtotal += db_price * item.quantity as f64;

        let (interval, interval_count) = match item.interval.as_deref() {
            Some("monthly") => (Some("month".to_string()), Some(1)),
            Some("quarterly") => (Some("month".to_string()), Some(3)),
            Some("yearly") => (Some("year".to_string()), Some(1)),
            _ => (None, None),
        };

        line_items.push(LineItem {
            price_id: None,
            name: db_name.clone(),
            description: None,
            amount: (db_price * 100.0).round() as i64,
            currency: "usd".to_string(),
            quantity: item.quantity as i64,
            is_subscription: item.is_subscription,
            interval,
            interval_count,
        });

        resolved_items.push(ResolvedItem {
            product_id: item.product_id,
            plan_id: item.plan_id,
            db_name,
            db_price,
            quantity: item.quantity,
            is_subscription: item.is_subscription,
            interval: item.interval.clone(),
        });
    }

    // Apply coupon if provided
    let mut discount = 0.0_f64;
    let mut coupon_id: Option<i64> = None;

    if let Some(ref code) = input.coupon_code {
        #[derive(sqlx::FromRow)]
        struct CouponInfo {
            id: i64,
            discount_type: String,
            discount_value: f64,
            max_discount: Option<f64>,
        }

        let coupon: Option<CouponInfo> = sqlx::query_as(
            r#"SELECT id, discount_type, discount_value, max_discount
               FROM coupons
               WHERE UPPER(code) = UPPER($1)
               AND is_active = true
               AND (expires_at IS NULL OR expires_at > NOW())
               AND (usage_limit IS NULL OR usage_count < usage_limit)"#,
        )
        .bind(code)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        if let Some(c) = coupon {
            discount = if c.discount_type == "percent" {
                subtotal * (c.discount_value / 100.0)
            } else {
                c.discount_value
            };

            if let Some(max) = c.max_discount {
                discount = discount.min(max);
            }

            coupon_id = Some(c.id);
        }
    }

    let total = subtotal - discount;

    // Generate order number
    let order_number = format!(
        "ORD-{}-{}",
        chrono::Utc::now().format("%Y%m%d"),
        uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    // Create order in database
    let order_id: i64 = sqlx::query_scalar(
        r#"INSERT INTO orders (
            user_id, order_number, status, subtotal, discount, tax, total,
            currency, coupon_id, coupon_code, billing_name, billing_email,
            billing_address, created_at, updated_at
        ) VALUES ($1, $2, 'pending', $3, $4, 0, $5, 'USD', $6, $7, $8, $9, $10, NOW(), NOW())
        RETURNING id"#,
    )
    .bind(user.id)
    .bind(&order_number)
    .bind(subtotal)
    .bind(discount)
    .bind(total)
    .bind(coupon_id)
    .bind(&input.coupon_code)
    .bind(&input.billing_name)
    .bind(input.billing_email.as_ref().unwrap_or(&user.email))
    .bind(&input.billing_address)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Insert order items using DB-sourced prices
    for ri in &resolved_items {
        sqlx::query(
            r#"INSERT INTO order_items (
                order_id, product_id, plan_id, name, quantity, unit_price, total, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())"#,
        )
        .bind(order_id)
        .bind(ri.product_id)
        .bind(ri.plan_id)
        .bind(&ri.db_name)
        .bind(ri.quantity)
        .bind(ri.db_price)
        .bind(ri.db_price * ri.quantity as f64)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;
    }

    // Create Stripe checkout session
    let mut metadata = HashMap::new();
    metadata.insert("order_id".to_string(), order_id.to_string());
    metadata.insert("order_number".to_string(), order_number.clone());
    metadata.insert("user_id".to_string(), user.id.to_string());

    // Validate and sanitize paths — must start with "/" to prevent open redirect
    let success_path = input.success_path.as_deref().unwrap_or("/checkout/success");
    let cancel_path = input.cancel_path.as_deref().unwrap_or("/checkout/cancel");
    if !success_path.starts_with('/') || !cancel_path.starts_with('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "success_path and cancel_path must start with /"})),
        ));
    }
    let app_url = state.config.app_url.trim_end_matches('/');
    let config = CheckoutConfig {
        customer_email: input.billing_email.unwrap_or(user.email.clone()),
        customer_name: input.billing_name,
        line_items,
        success_url: format!("{}{}?order={}", app_url, success_path, order_number),
        cancel_url: format!("{}{}", app_url, cancel_path),
        metadata,
        allow_promotion_codes: input.coupon_code.is_none(),
        billing_address_collection: true,
    };

    let session = state
        .services
        .stripe
        .create_checkout_session(config)
        .await
        .map_err(|e| {
            tracing::error!("Stripe checkout error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Payment service error: {}", e)})),
            )
        })?;

    // Update order with Stripe session ID
    sqlx::query("UPDATE orders SET stripe_session_id = $1, updated_at = NOW() WHERE id = $2")
        .bind(&session.id)
        .bind(order_id)
        .execute(&state.db.pool)
        .await
        .ok();

    tracing::info!(
        target: "payments",
        event = "checkout_created",
        order_id = %order_id,
        order_number = %order_number,
        user_id = %user.id,
        amount = %total,
        "Checkout session created"
    );

    Ok(Json(CheckoutResponse {
        session_id: session.id,
        url: session.url.unwrap_or_default(),
        order_id,
        order_number,
    }))
}

/// Get Stripe customer portal URL
/// POST /api/payments/portal
async fn create_portal(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreatePortalRequest>,
) -> Result<Json<PortalResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Get user's Stripe customer ID
    let customer_id: Option<String> = sqlx::query_scalar(
        "SELECT stripe_customer_id FROM user_memberships WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .flatten();

    let customer_id = customer_id.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No payment method on file"})),
        )
    })?;

    let url = state
        .services
        .stripe
        .create_portal_session(&customer_id, &input.return_url)
        .await
        .map_err(|e| {
            tracing::error!("Stripe portal error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Failed to create portal session"})),
            )
        })?;

    Ok(Json(PortalResponse { url }))
}

/// Handle Stripe webhooks
/// POST /api/payments/webhook
async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Get signature header
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing signature"})),
            )
        })?;

    // PE7 invariant 2A: prefer DB-stored webhook secret over env, so admin
    // rotation in /admin/settings takes effect without a redeploy.
    let env_scope = state.config.environment.clone();
    let stripe_for_webhook = state
        .services
        .credentials
        .stripe_client(&state.db.pool, &env_scope)
        .await;

    // ICT 7 Fix: Actually verify webhook signature for production security
    match stripe_for_webhook.verify_webhook(&body, signature) {
        Ok(true) => {
            tracing::debug!(target: "payments", "Webhook signature verified successfully");
        }
        Ok(false) => {
            tracing::error!(target: "payments", "Webhook signature verification failed");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid webhook signature"})),
            ));
        }
        Err(e) => {
            // If webhook secret not configured, reject in production, warn in dev
            if e.to_string().contains("not configured") {
                // FIX-2026-04-26 (Priority 8): read environment from state.config (parsed
                // once at startup) instead of std::env::var per-request. Eliminates
                // race conditions where the env-var name diverges from runtime config and
                // ensures webhook dev-mode bypass never accidentally triggers in prod due
                // to per-request env mutation.
                // Original lines:
                // let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
                // if environment != "development" && environment != "dev" {
                let env_name = state.config.environment.as_str();
                let is_dev = env_name.starts_with("dev");
                if !is_dev {
                    tracing::error!(
                        target: "payments",
                        environment = %env_name,
                        "Webhook secret not configured in production - rejecting webhook"
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Webhook secret not configured"})),
                    ));
                }
                tracing::warn!(
                    target: "payments",
                    environment = %env_name,
                    "Webhook secret not configured - skipping signature verification (dev mode)"
                );
            } else {
                tracing::error!(target: "payments", "Webhook verification error: {}", e);
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Webhook verification failed"})),
                ));
            }
        }
    }

    // Parse the webhook event — require valid UTF-8 (Stripe always sends UTF-8 JSON)
    let body_str = std::str::from_utf8(&body).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Webhook payload is not valid UTF-8"})),
        )
    })?;
    let event = state
        .services
        .stripe
        .parse_webhook_event(body_str)
        .map_err(|e| {
            tracing::error!("Webhook parse error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid webhook payload"})),
            )
        })?;

    tracing::info!(
        target: "payments",
        event = "webhook_received",
        event_type = %event.event_type,
        event_id = %event.id,
        "Processing Stripe webhook"
    );

    // Idempotency: record the event; skip processing if already seen
    let payload_value: serde_json::Value =
        serde_json::from_str(body_str).unwrap_or(serde_json::Value::Null);
    let inserted: Option<i64> = sqlx::query_scalar(
        r#"INSERT INTO webhook_events (event_id, event_type, payload)
           VALUES ($1, $2, $3)
           ON CONFLICT (event_id) DO NOTHING
           RETURNING 1"#,
    )
    .bind(&event.id)
    .bind(&event.event_type)
    .bind(&payload_value)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "Failed to record webhook event: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error recording webhook"})),
        )
    })?
    .flatten();

    if inserted.is_none() {
        tracing::info!(
            target: "payments",
            event_id = %event.id,
            "Duplicate webhook event — skipping"
        );
        return Ok(StatusCode::OK);
    }

    // Handle different event types
    match event.event_type.as_str() {
        "checkout.session.completed" => {
            handle_checkout_completed(&state, &event).await?;
        }
        "customer.subscription.created" => {
            handle_subscription_created(&state, &event).await?;
        }
        "customer.subscription.updated" => {
            handle_subscription_updated(&state, &event).await?;
        }
        "customer.subscription.deleted" => {
            handle_subscription_deleted(&state, &event).await?;
        }
        "invoice.paid" => {
            handle_invoice_paid(&state, &event).await?;
        }
        "invoice.payment_failed" => {
            handle_payment_failed(&state, &event).await?;
        }
        "charge.refunded" => {
            handle_refund(&state, &event).await?;
        }
        "charge.dispute.created" => {
            handle_dispute_created(&state, &event).await?;
        }
        _ => {
            tracing::debug!("Unhandled webhook event: {}", event.event_type);
        }
    }

    // Mark event as processed
    sqlx::query(
        "UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1",
    )
    .bind(&event.id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "Failed to mark webhook processed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to finalize webhook processing"})),
        )
    })?;

    Ok(StatusCode::OK)
}

/// Process refund request
/// POST /api/payments/refund
async fn create_refund(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<RefundRequest>,
) -> Result<Json<RefundResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Get order and verify ownership (or admin)
    #[derive(sqlx::FromRow)]
    struct OrderInfo {
        user_id: i64,
        payment_intent_id: Option<String>,
        status: String,
    }

    let order: OrderInfo =
        sqlx::query_as("SELECT user_id, payment_intent_id, status FROM orders WHERE id = $1")
            .bind(input.order_id)
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
                    Json(json!({"error": "Order not found"})),
                )
            })?;

    // Check authorization (must be owner or admin)
    let is_admin =
        user.role.as_deref() == Some("admin") || user.role.as_deref() == Some("super_admin");
    if order.user_id != user.id && !is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not authorized"})),
        ));
    }

    // Check order status
    if order.status != "completed" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Order cannot be refunded"})),
        ));
    }

    let payment_intent_id = order.payment_intent_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No payment found for this order"})),
        )
    })?;

    // Create refund via Stripe
    let refund = state
        .services
        .stripe
        .create_refund(&payment_intent_id, input.amount, input.reason.as_deref())
        .await
        .map_err(|e| {
            tracing::error!("Stripe refund error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Refund failed: {}", e)})),
            )
        })?;

    // Update order status
    sqlx::query("UPDATE orders SET status = 'refunded', refunded_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(input.order_id)
        .execute(&state.db.pool)
        .await
        .ok();

    tracing::info!(
        target: "payments",
        event = "refund_processed",
        order_id = %input.order_id,
        refund_id = %refund.id,
        amount = %refund.amount,
        "Refund processed"
    );

    Ok(Json(RefundResponse {
        refund_id: refund.id,
        amount: refund.amount,
        status: refund.status,
    }))
}

/// Get payment configuration (publishable key)
/// GET /api/payments/config
/// ICT 7 Fix: Returns both snake_case and camelCase for frontend compatibility
async fn get_config(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        // Snake_case (original)
        "publishable_key": state.config.stripe_publishable_key,
        // CamelCase (frontend compatibility)
        "publishableKey": state.config.stripe_publishable_key,
        "currency": "usd",
        "country": "US"
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Webhook Handlers
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_checkout_completed(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let session = event.as_checkout_session().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid checkout session data"})),
        )
    })?;

    let order_id = event.get_order_id();
    let user_id = event.get_user_id();

    if let Some(order_id) = order_id {
        // Update order status to completed
        sqlx::query(
            r#"UPDATE orders SET
                status = 'completed',
                payment_provider = 'stripe',
                payment_intent_id = $1,
                completed_at = NOW(),
                updated_at = NOW()
            WHERE id = $2"#,
        )
        .bind(&session.payment_intent)
        .bind(order_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        // If subscription, create user membership
        if let Some(ref subscription_id) = session.subscription {
            if let Some(user_id) = user_id {
                // Get subscription details
                let stripe_sub = state
                    .services
                    .stripe
                    .get_subscription(subscription_id)
                    .await
                    .ok();

                if let Some(sub) = stripe_sub {
                    // FIX-2026-04-26: was `.ok().flatten()` which silently dropped DB errors
                    // on the plan-id lookup. Now logs the error and proceeds without
                    // creating the membership (rather than 500'ing the webhook).
                    // Old: .fetch_optional(&state.db.pool).await.ok().flatten()
                    let plan_id: Option<i64> = match sqlx::query_scalar::<_, i64>(
                        "SELECT plan_id FROM order_items WHERE order_id = $1 AND plan_id IS NOT NULL LIMIT 1"
                    )
                    .bind(order_id)
                    .fetch_optional(&state.db.pool)
                    .await {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::error!(target: "payments", order_id = %order_id, "plan_id lookup failed in webhook: {}", e);
                            None
                        }
                    };

                    if let Some(plan_id) = plan_id {
                        // FIX-2026-04-26: was `.execute(&state.db.pool).await.ok();` — silently
                        // dropping membership-creation failures meant the user paid Stripe
                        // but got no membership and no error logged. Now logs failure
                        // explicitly. Future: wrap in transaction with the access-grant
                        // loop below for true atomicity.
                        // Old: .execute(&state.db.pool).await.ok();
                        if let Err(e) = sqlx::query(
                            r#"INSERT INTO user_memberships (
                                user_id, plan_id, starts_at, status,
                                payment_provider, stripe_subscription_id, stripe_customer_id,
                                current_period_start, current_period_end,
                                cancel_at_period_end, created_at, updated_at
                            ) VALUES ($1, $2, NOW(), 'active', 'stripe', $3, $4, $5, $6, false, NOW(), NOW())
                            ON CONFLICT (user_id, plan_id)
                            DO UPDATE SET
                                status = 'active',
                                stripe_subscription_id = $3,
                                stripe_customer_id = $4,
                                current_period_start = $5,
                                current_period_end = $6,
                                updated_at = NOW()"#
                        )
                        .bind(user_id)
                        .bind(plan_id)
                        .bind(&sub.id)
                        .bind(&sub.customer)
                        .bind(chrono::DateTime::from_timestamp(sub.current_period_start, 0).map(|d| d.naive_utc()))
                        .bind(chrono::DateTime::from_timestamp(sub.current_period_end, 0).map(|d| d.naive_utc()))
                        .execute(&state.db.pool)
                        .await {
                            tracing::error!(target: "payments", user_id = %user_id, plan_id = %plan_id, "Failed to create user_membership: {}", e);
                        }
                    }
                }
            }
        }

        // FIX-2026-04-26: was `.execute(&state.db.pool).await.ok();` — silently
        // swallowed coupon-usage update failures. Logged now.
        // Old: .execute(&state.db.pool).await.ok();
        if let Err(e) = sqlx::query(
            "UPDATE coupons SET usage_count = usage_count + 1, updated_at = NOW() WHERE id = (SELECT coupon_id FROM orders WHERE id = $1)"
        )
        .bind(order_id)
        .execute(&state.db.pool)
        .await {
            tracing::error!(target: "payments", order_id = %order_id, "Failed to increment coupon usage: {}", e);
        }

        // ═══════════════════════════════════════════════════════════════════════════
        // ACCESS GRANTING - Apple ICT 11+ Principal Engineer Grade
        // ═══════════════════════════════════════════════════════════════════════════
        // Grant access for all purchased items: courses, indicators, and products
        if let Some(user_id) = user_id {
            #[derive(sqlx::FromRow)]
            struct ProductOrderItem {
                product_id: Option<i64>,
                name: String,
            }

            let product_items: Vec<ProductOrderItem> = sqlx::query_as(
                "SELECT product_id, name FROM order_items WHERE order_id = $1 AND product_id IS NOT NULL",
            )
            .bind(order_id)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "payments",
                    event = "access_grant_query_failed",
                    order_id = %order_id,
                    "Failed to fetch order items for access grant: {}",
                    e
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to fetch order items for access grant"})),
                )
            })?;

            for item in product_items {
                if let Some(product_id) = item.product_id {
                    // Get product details to determine type
                    #[derive(sqlx::FromRow)]
                    struct ProductInfo {
                        product_type: Option<String>,
                        course_id: Option<uuid::Uuid>,
                        indicator_id: Option<i64>,
                    }

                    // FIX-2026-04-26: was `.ok().flatten()` — silently dropped product
                    // lookup errors. Now logged.
                    // Old: .fetch_optional(&state.db.pool).await.ok().flatten();
                    let product_info: Option<ProductInfo> = match sqlx::query_as::<_, ProductInfo>(
                        "SELECT type as product_type, course_id, indicator_id FROM products WHERE id = $1",
                    )
                    .bind(product_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::error!(target: "payments", product_id = %product_id, "products lookup failed: {}", e);
                            None
                        }
                    };

                    if let Some(info) = product_info {
                        // ─────────────────────────────────────────────────────────
                        // 1. Create user_products record for ownership tracking
                        // ─────────────────────────────────────────────────────────
                        // FIX-2026-04-26: was `.ok();` — silently dropped user_products
                        // INSERT failures (paid customer, no ownership row). Now logged.
                        // Old: .execute(&state.db.pool).await.ok();
                        if let Err(e) = sqlx::query(
                            r#"INSERT INTO user_products (user_id, product_id, purchased_at, order_id)
                               VALUES ($1, $2, NOW(), $3)
                               ON CONFLICT (user_id, product_id) DO UPDATE SET
                                   purchased_at = COALESCE(user_products.purchased_at, NOW()),
                                   order_id = COALESCE(user_products.order_id, $3)"#
                        )
                        .bind(user_id)
                        .bind(product_id)
                        .bind(order_id)
                        .execute(&state.db.pool)
                        .await {
                            tracing::error!(target: "payments", user_id = %user_id, product_id = %product_id, order_id = %order_id, "user_products INSERT failed: {}", e);
                        }

                        tracing::info!(
                            target: "payments",
                            event = "user_product_created",
                            user_id = %user_id,
                            product_id = %product_id,
                            order_id = %order_id,
                            "Product ownership recorded"
                        );

                        // ─────────────────────────────────────────────────────────
                        // 2. Course enrollment
                        // ─────────────────────────────────────────────────────────
                        if let Some(course_id) = info.course_id {
                            // FIX-2026-04-26: was `.ok();` — silently dropped course
                            // enrollment failures. Now logged.
                            // Old: .execute(&state.db.pool).await.ok();
                            if let Err(e) = sqlx::query(
                                r#"INSERT INTO user_course_enrollments (user_id, course_id, status, enrolled_at)
                                   VALUES ($1, $2, 'active', NOW())
                                   ON CONFLICT (user_id, course_id) DO UPDATE SET
                                       status = 'active',
                                       updated_at = NOW()"#
                            )
                            .bind(user_id)
                            .bind(course_id)
                            .execute(&state.db.pool)
                            .await {
                                tracing::error!(target: "payments", user_id = %user_id, course_id = %course_id, "user_course_enrollments INSERT failed: {}", e);
                            }

                            tracing::info!(
                                target: "payments",
                                event = "course_enrollment_created",
                                user_id = %user_id,
                                course_id = %course_id,
                                order_id = %order_id,
                                "User enrolled in course after purchase"
                            );
                        }

                        // ─────────────────────────────────────────────────────────
                        // 3. Indicator access
                        // ─────────────────────────────────────────────────────────
                        if let Some(indicator_id) = info.indicator_id {
                            // FIX-2026-04-26: was `.ok();` — silently dropped indicator
                            // access INSERT failures. Now logged.
                            // Old: .execute(&state.db.pool).await.ok();
                            if let Err(e) = sqlx::query(
                                r#"INSERT INTO user_indicator_access (user_id, indicator_id, is_active, granted_at)
                                   VALUES ($1, $2, true, NOW())
                                   ON CONFLICT (user_id, indicator_id) DO UPDATE SET
                                       is_active = true,
                                       granted_at = COALESCE(user_indicator_access.granted_at, NOW())"#
                            )
                            .bind(user_id)
                            .bind(indicator_id)
                            .execute(&state.db.pool)
                            .await {
                                tracing::error!(target: "payments", user_id = %user_id, indicator_id = %indicator_id, "user_indicator_access INSERT failed: {}", e);
                            }

                            tracing::info!(
                                target: "payments",
                                event = "indicator_access_granted",
                                user_id = %user_id,
                                indicator_id = %indicator_id,
                                order_id = %order_id,
                                "Indicator access granted after purchase"
                            );
                        }

                        // ─────────────────────────────────────────────────────────
                        // 4. Handle indicator type products without indicator_id FK
                        // ─────────────────────────────────────────────────────────
                        if info.product_type.as_deref() == Some("indicator")
                            && info.indicator_id.is_none()
                        {
                            // Try to find indicator by matching product name/slug
                            let indicator_id: Option<i64> = sqlx::query_scalar(
                                r#"SELECT i.id FROM indicators i
                                   JOIN products p ON LOWER(p.name) LIKE CONCAT('%', LOWER(i.name), '%')
                                   WHERE p.id = $1
                                   LIMIT 1"#
                            )
                            .bind(product_id)
                            // FIX-2026-04-26: was `.ok().flatten()` — silently dropped
                            // indicator-by-name lookup errors. Now logged.
                            // Old: .fetch_optional(&state.db.pool).await.ok().flatten();
                            .fetch_optional(&state.db.pool)
                            .await
                            .unwrap_or_else(|e| {
                                tracing::error!(target: "payments", product_id = %product_id, "indicator-by-name lookup failed: {}", e);
                                None
                            });

                            if let Some(ind_id) = indicator_id {
                                // FIX-2026-04-26: was `.ok();` — silently dropped indicator
                                // access (by-type) INSERT failures. Now logged.
                                // Old: .execute(&state.db.pool).await.ok();
                                if let Err(e) = sqlx::query(
                                    r#"INSERT INTO user_indicator_access (user_id, indicator_id, is_active, granted_at)
                                       VALUES ($1, $2, true, NOW())
                                       ON CONFLICT (user_id, indicator_id) DO UPDATE SET
                                           is_active = true,
                                           granted_at = COALESCE(user_indicator_access.granted_at, NOW())"#
                                )
                                .bind(user_id)
                                .bind(ind_id)
                                .execute(&state.db.pool)
                                .await {
                                    tracing::error!(target: "payments", user_id = %user_id, indicator_id = %ind_id, "user_indicator_access (by-type) INSERT failed: {}", e);
                                }

                                tracing::info!(
                                    target: "payments",
                                    event = "indicator_access_granted_by_type",
                                    user_id = %user_id,
                                    indicator_id = %ind_id,
                                    product_id = %product_id,
                                    order_id = %order_id,
                                    "Indicator access granted via product type matching"
                                );
                            }
                        }
                    }
                }
            }
        }

        // Send order confirmation email
        if let Some(user_id) = user_id {
            if let Some(ref email_service) = state.services.email {
                let user_email: Option<String> =
                    sqlx::query_scalar("SELECT email FROM users WHERE id = $1")
                        .bind(user_id)
                        .fetch_optional(&state.db.pool)
                        .await
                        .ok()
                        .flatten();

                if let Some(email) = user_email {
                    let order_number: Option<String> =
                        sqlx::query_scalar("SELECT order_number FROM orders WHERE id = $1")
                            .bind(order_id)
                            .fetch_optional(&state.db.pool)
                            .await
                            .ok()
                            .flatten();

                    if let Some(order_num) = order_number {
                        let _ = email_service
                            .send_order_confirmation(&email, &order_num)
                            .await;
                    }
                }
            }
        }

        tracing::info!(
            target: "payments",
            event = "order_completed",
            order_id = %order_id,
            subscription_id = ?session.subscription,
            "Order completed via Stripe"
        );
    }

    Ok(())
}

async fn handle_subscription_created(
    _state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_created",
        subscription_id = %subscription.id,
        customer = %subscription.customer,
        "Subscription created"
    );

    Ok(())
}

async fn handle_subscription_updated(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    // Update user membership
    let status = match subscription.status.as_str() {
        "active" => "active",
        "past_due" => "past_due",
        "canceled" => "cancelled",
        "unpaid" => "unpaid",
        _ => "active",
    };

    sqlx::query(
        r#"UPDATE user_memberships SET
            status = $1,
            current_period_start = $2,
            current_period_end = $3,
            cancel_at_period_end = $4,
            cancelled_at = $5,
            updated_at = NOW()
        WHERE stripe_subscription_id = $6"#,
    )
    .bind(status)
    .bind(
        chrono::DateTime::from_timestamp(subscription.current_period_start, 0)
            .map(|d| d.naive_utc()),
    )
    .bind(
        chrono::DateTime::from_timestamp(subscription.current_period_end, 0).map(|d| d.naive_utc()),
    )
    .bind(subscription.cancel_at_period_end)
    .bind(
        subscription
            .canceled_at
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|d| d.naive_utc())),
    )
    .bind(&subscription.id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "payments",
            subscription_id = %subscription.id,
            error = %e,
            "DB write failed in subscription_updated — Stripe will retry"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update subscription"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_updated",
        subscription_id = %subscription.id,
        status = %status,
        "Subscription updated"
    );

    Ok(())
}

async fn handle_subscription_deleted(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    sqlx::query(
        "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW() WHERE stripe_subscription_id = $1"
    )
    .bind(&subscription.id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "payments",
            subscription_id = %subscription.id,
            error = %e,
            "DB write failed in subscription_deleted — Stripe will retry"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to cancel subscription"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_cancelled",
        subscription_id = %subscription.id,
        "Subscription cancelled"
    );

    Ok(())
}

async fn handle_invoice_paid(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invoice = &event.data.object;
    let subscription_id = invoice["subscription"].as_str();

    if let Some(sub_id) = subscription_id {
        // Extend subscription period
        sqlx::query(
            "UPDATE user_memberships SET status = 'active', updated_at = NOW() WHERE stripe_subscription_id = $1"
        )
        .bind(sub_id)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    tracing::info!(
        target: "payments",
        event = "invoice_paid",
        invoice_id = %invoice["id"].as_str().unwrap_or("unknown"),
        "Invoice paid"
    );

    Ok(())
}

async fn handle_payment_failed(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invoice = &event.data.object;
    let subscription_id = invoice["subscription"].as_str();
    let customer_email = invoice["customer_email"].as_str();

    // Get payment attempt count and amount from invoice
    let attempt_count: i32 = invoice["attempt_count"].as_i64().unwrap_or(1) as i32;
    let amount_due: f64 = invoice["amount_due"].as_i64().unwrap_or(0) as f64 / 100.0;

    // ICT 7 Fix: Calculate grace period end (7 days from now for standard grace period)
    // Stripe typically retries 3-4 times over ~3 weeks, so 7 days is a reasonable initial grace period
    let grace_period_days = 7;
    let grace_period_end = chrono::Utc::now() + chrono::Duration::days(grace_period_days);

    if let Some(sub_id) = subscription_id {
        // ICT 7 Fix: Update membership status AND set grace_period_end
        sqlx::query(
            r#"UPDATE user_memberships SET
               status = 'past_due',
               grace_period_end = CASE
                   WHEN grace_period_end IS NULL OR grace_period_end < NOW()
                   THEN $1
                   ELSE grace_period_end
               END,
               payment_failure_count = $2,
               last_payment_failure = NOW(),
               updated_at = NOW()
               WHERE stripe_subscription_id = $3"#,
        )
        .bind(grace_period_end.naive_utc())
        .bind(attempt_count)
        .bind(sub_id)
        .execute(&state.db.pool)
        .await
        .ok();

        // ICT 7 Fix: Send payment failed with grace period email notification
        if let Some(ref email_service) = state.services.email {
            // Get user details from membership including plan price
            #[derive(sqlx::FromRow)]
            struct UserSubscription {
                email: String,
                name: String,
                plan_name: String,
                price: Option<f64>,
            }

            let user_info: Option<UserSubscription> = sqlx::query_as(
                r#"SELECT u.email, COALESCE(u.name, u.email) as name,
                   COALESCE(mp.name, 'Subscription') as plan_name,
                   mp.price::FLOAT8 as price
                   FROM user_memberships um
                   JOIN users u ON um.user_id = u.id
                   LEFT JOIN membership_plans mp ON um.plan_id = mp.id
                   WHERE um.stripe_subscription_id = $1
                   LIMIT 1"#,
            )
            .bind(sub_id)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();

            if let Some(info) = user_info {
                // Use amount from invoice, fallback to plan price
                let payment_amount = if amount_due > 0.0 {
                    amount_due
                } else {
                    info.price.unwrap_or(0.0)
                };
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();

                let _ = email_service
                    .send_payment_failed_with_grace(
                        &info.email,
                        &info.name,
                        &info.plan_name,
                        payment_amount,
                        &grace_end_str,
                        attempt_count,
                    )
                    .await;

                tracing::info!(
                    target: "payments",
                    event = "payment_failed_email_sent",
                    email = %info.email,
                    subscription_id = %sub_id,
                    grace_period_end = %grace_end_str,
                    attempt_count = %attempt_count,
                    "Payment failed notification email sent with grace period"
                );
            } else if let Some(email) = customer_email {
                // Fallback: use customer email from invoice if we can't find membership
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();
                let _ = email_service
                    .send_payment_failed_with_grace(
                        email,
                        "Valued Customer",
                        "your subscription",
                        amount_due,
                        &grace_end_str,
                        attempt_count,
                    )
                    .await;
            }
        }
    }

    tracing::warn!(
        target: "payments",
        event = "payment_failed",
        invoice_id = %invoice["id"].as_str().unwrap_or("unknown"),
        subscription_id = ?subscription_id,
        attempt_count = %attempt_count,
        amount_due = %amount_due,
        grace_period_end = %grace_period_end.format("%Y-%m-%d"),
        "Payment failed - grace period initialized"
    );

    Ok(())
}

async fn handle_refund(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let charge = &event.data.object;
    let payment_intent = charge["payment_intent"].as_str();

    let refund_amount = charge["amount_refunded"].as_i64().unwrap_or(0);
    let total_amount = charge["amount"].as_i64().unwrap_or(0);
    let is_full_refund = refund_amount > 0 && refund_amount >= total_amount;

    if let Some(pi) = payment_intent {
        let refund_status = if is_full_refund {
            "refunded"
        } else {
            "partial_refund"
        };

        let refund_amount_dollars = refund_amount as f64 / 100.0;

        sqlx::query(
            r#"UPDATE orders SET
               status = $1,
               refund_amount = COALESCE(refund_amount, 0) + $2,
               refunded_at = CASE WHEN $1 = 'refunded' THEN NOW() ELSE refunded_at END,
               updated_at = NOW()
               WHERE payment_intent_id = $3"#,
        )
        .bind(refund_status)
        .bind(refund_amount_dollars)
        .bind(pi)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", error = %e, "Failed to update order on refund");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record refund"})),
            )
        })?;

        // Full refund: revoke access for subscriptions, courses, and indicators
        if is_full_refund {
            // Revoke subscription membership
            sqlx::query(
                r#"UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW()
                   WHERE id IN (
                       SELECT um.id FROM user_memberships um
                       JOIN orders o ON o.user_id = um.user_id
                       WHERE o.payment_intent_id = $1
                         AND um.status != 'cancelled'
                   )"#,
            )
            .bind(pi)
            .execute(&state.db.pool)
            .await
            .ok(); // Non-fatal — best-effort revocation

            // Revoke course enrollment access
            sqlx::query(
                r#"UPDATE user_course_enrollments SET is_active = false
                   WHERE user_id = (SELECT user_id FROM orders WHERE payment_intent_id = $1 LIMIT 1)
                     AND is_active = true"#,
            )
            .bind(pi)
            .execute(&state.db.pool)
            .await
            .ok();

            // Revoke indicator access
            sqlx::query(
                r#"UPDATE user_indicator_access SET is_active = false
                   WHERE user_id = (SELECT user_id FROM orders WHERE payment_intent_id = $1 LIMIT 1)
                     AND is_active = true"#,
            )
            .bind(pi)
            .execute(&state.db.pool)
            .await
            .ok();

            tracing::info!(
                target: "payments",
                event = "access_revoked_on_refund",
                payment_intent = %pi,
                "Access revoked following full refund"
            );
        }

        tracing::info!(
            target: "payments",
            event = "charge_refunded",
            charge_id = %charge["id"].as_str().unwrap_or("unknown"),
            payment_intent = %pi,
            refund_amount_cents = %refund_amount,
            total_amount_cents = %total_amount,
            refund_status = %refund_status,
            "Refund processed"
        );
    }

    Ok(())
}

async fn handle_dispute_created(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let dispute = &event.data.object;

    let dispute_id = dispute["id"].as_str().unwrap_or("unknown");
    let charge_id = dispute["charge"].as_str().unwrap_or("unknown");
    let reason = dispute["reason"].as_str().unwrap_or("unknown");
    let status = dispute["status"].as_str().unwrap_or("unknown");
    let amount_cents = dispute["amount"].as_i64().unwrap_or(0);
    let currency = dispute["currency"].as_str().unwrap_or("usd");
    let response_deadline = dispute["evidence_details"]["due_by"]
        .as_i64()
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0));

    // Insert dispute record
    sqlx::query(
        r#"INSERT INTO payment_disputes
           (stripe_dispute_id, stripe_charge_id, reason, status, amount_cents, currency, response_deadline)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (stripe_dispute_id) DO NOTHING"#,
    )
    .bind(dispute_id)
    .bind(charge_id)
    .bind(reason)
    .bind(status)
    .bind(amount_cents)
    .bind(currency)
    .bind(response_deadline.map(|d| d.naive_utc()))
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "Failed to insert dispute record");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to record dispute"})),
        )
    })?;

    // Log to security_events
    sqlx::query(
        r#"INSERT INTO security_events (event_type, details, created_at)
           VALUES ('chargeback_dispute', $1, NOW())"#,
    )
    .bind(json!({
        "dispute_id": dispute_id,
        "charge_id": charge_id,
        "reason": reason,
        "amount_cents": amount_cents,
        "currency": currency,
        "status": status,
        "severity": "high"
    }))
    .execute(&state.db.pool)
    .await
    .ok(); // Log failure is non-fatal

    tracing::warn!(
        target: "payments",
        event = "dispute_created",
        dispute_id = %dispute_id,
        charge_id = %charge_id,
        reason = %reason,
        amount_cents = %amount_cents,
        "Chargeback dispute created — admin notification required"
    );

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// INVOICE GENERATION - ICT 7 Fix
// ═══════════════════════════════════════════════════════════════════════════

/// Invoice generation request
#[derive(Debug, serde::Deserialize)]
pub struct GenerateInvoiceRequest {
    pub order_id: i64,
}

/// Generate invoice for an order
/// POST /api/payments/invoice
async fn generate_invoice(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<GenerateInvoiceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify order belongs to user
    #[derive(sqlx::FromRow)]
    struct OrderInvoiceData {
        id: i64,
        order_number: String,
        user_id: i64,
        status: String,
        subtotal: f64,
        discount: f64,
        tax: f64,
        total: f64,
        currency: String,
        billing_name: Option<String>,
        billing_email: Option<String>,
        billing_address: Option<serde_json::Value>,
        coupon_code: Option<String>,
        created_at: chrono::NaiveDateTime,
        completed_at: Option<chrono::NaiveDateTime>,
    }

    let order: OrderInvoiceData = sqlx::query_as(
        r#"SELECT id, order_number, user_id, status, subtotal::FLOAT8 as subtotal, discount::FLOAT8 as discount,
                  tax::FLOAT8 as tax, total::FLOAT8 as total, currency, billing_name, billing_email,
                  billing_address, coupon_code, created_at, completed_at
           FROM orders WHERE id = $1"#
    )
    .bind(input.order_id)
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
            Json(json!({"error": "Order not found"})),
        )
    })?;

    // Check ownership unless admin
    let is_admin =
        user.role.as_deref() == Some("admin") || user.role.as_deref() == Some("super_admin");
    if order.user_id != user.id && !is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Access denied"})),
        ));
    }

    // Fetch order items
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderItem {
        name: String,
        quantity: i32,
        unit_price: f64,
        total: f64,
    }

    let items: Vec<OrderItem> = sqlx::query_as(
        "SELECT name, quantity, unit_price::FLOAT8 as unit_price, total::FLOAT8 as total FROM order_items WHERE order_id = $1"
    )
    .bind(order.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Generate invoice number
    let invoice_number = format!(
        "INV-{}-{}",
        order.order_number,
        chrono::Utc::now().format("%Y%m%d")
    );

    // Build invoice response
    let invoice = json!({
        "invoice_number": invoice_number,
        "order_number": order.order_number,
        "status": order.status,
        "date": order.created_at.and_utc().to_rfc3339(),
        "completed_at": order.completed_at.map(|d| d.and_utc().to_rfc3339()),
        "billing": {
            "name": order.billing_name,
            "email": order.billing_email,
            "address": order.billing_address
        },
        "items": items,
        "subtotal": order.subtotal,
        "discount": order.discount,
        "coupon_code": order.coupon_code,
        "tax": order.tax,
        "total": order.total,
        "currency": order.currency,
        "company": {
            "name": "Revolution Trading Pros",
            "address": "Trading Professional Services",
            "email": "billing@revolutiontradingpros.com"
        }
    });

    tracing::info!(
        target: "payments",
        event = "invoice_generated",
        invoice_number = %invoice_number,
        order_id = %order.id,
        user_id = %user.id,
        "Invoice generated"
    );

    Ok(Json(json!({
        "success": true,
        "invoice": invoice
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// PAYMENT RETRY - ICT 7 Fix
// ═══════════════════════════════════════════════════════════════════════════

/// Retry payment request
#[derive(Debug, serde::Deserialize)]
pub struct RetryPaymentRequest {
    pub subscription_id: String,
    pub payment_method_id: Option<String>,
}

/// Retry a failed subscription payment
/// POST /api/payments/retry
async fn retry_payment(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<RetryPaymentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify subscription belongs to user
    #[derive(sqlx::FromRow)]
    struct MembershipRow {
        id: i64,
        user_id: i64,
        stripe_subscription_id: Option<String>,
        stripe_customer_id: Option<String>,
        status: String,
    }

    let membership: MembershipRow = sqlx::query_as(
        "SELECT id, user_id, stripe_subscription_id, stripe_customer_id, status FROM user_memberships WHERE stripe_subscription_id = $1"
    )
    .bind(&input.subscription_id)
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

    // Check ownership
    if membership.user_id != user.id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Access denied"})),
        ));
    }

    // Check if subscription is in past_due status
    if membership.status != "past_due" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Subscription is not in past_due status"})),
        ));
    }

    // Update payment method if provided
    if let Some(pm_id) = &input.payment_method_id {
        if let Some(customer_id) = &membership.stripe_customer_id {
            state
                .services
                .stripe
                .update_default_payment_method(customer_id, pm_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::BAD_GATEWAY,
                        Json(json!({"error": format!("Failed to update payment method: {}", e)})),
                    )
                })?;
        }
    }

    // Retry the latest invoice
    let sub_id = membership.stripe_subscription_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No Stripe subscription ID found"})),
        )
    })?;

    let result = state
        .services
        .stripe
        .retry_subscription_payment(&sub_id)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", error = %e, "Payment retry failed");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Payment retry failed: {}", e)})),
            )
        })?;

    // Update membership status if successful
    if result.success {
        sqlx::query(
            "UPDATE user_memberships SET status = 'active', payment_failure_count = 0, updated_at = NOW() WHERE id = $1"
        )
        .bind(membership.id)
        .execute(&state.db.pool)
        .await
        .ok();

        tracing::info!(
            target: "payments",
            event = "payment_retry_success",
            subscription_id = %input.subscription_id,
            user_id = %user.id,
            "Payment retry successful"
        );
    } else {
        tracing::warn!(
            target: "payments",
            event = "payment_retry_failed",
            subscription_id = %input.subscription_id,
            user_id = %user.id,
            error = ?result.error,
            "Payment retry failed"
        );
    }

    Ok(Json(json!({
        "success": result.success,
        "message": if result.success { "Payment successful" } else { "Payment failed" },
        "error": result.error
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// PAYMENT SUMMARY (Admin Dashboard)
// ═══════════════════════════════════════════════════════════════════════════

/// Aggregated payment metrics returned to the admin dashboard.
///
/// FIX-2026-04-26: shape is hand-tuned for `/admin/dashboard/+page.svelte`
/// which reads `data.revenue` and `data.mrr`. Both are returned as decimal
/// dollar amounts (not cents) so the frontend's `formatCurrency` helper
/// renders them directly. Counts (`pending_count`, `failed_count`) are
/// included for future widgets — additive fields are tolerated by the
/// page's `...data` spread.
#[derive(Debug, Serialize)]
struct PaymentSummary {
    /// Sum of completed `orders.total` for the current calendar month (USD).
    /// The page treats this as "Revenue This Month".
    revenue: f64,
    /// Revenue today (USD), for future use by the dashboard.
    revenue_today: f64,
    /// Revenue over the last 7 days (USD), for future use.
    revenue_week: f64,
    /// Revenue over the current calendar month (USD) — alias of `revenue`.
    revenue_month: f64,
    /// Estimated monthly recurring revenue (USD) summed from active
    /// memberships' plan prices, normalized by `billing_cycle`.
    mrr: f64,
    /// Count of `orders.status = 'pending'`.
    pending_count: i64,
    /// Count of `orders.status = 'failed'`.
    failed_count: i64,
    /// Server timestamp the snapshot was generated at.
    last_updated: chrono::DateTime<chrono::Utc>,
}

/// Get aggregate payment metrics for the admin dashboard.
/// GET /api/payments/summary
async fn get_summary(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<PaymentSummary>, (StatusCode, Json<serde_json::Value>)> {
    // Revenue today (UTC).
    let revenue_today: f64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(total), 0)::FLOAT8
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= DATE_TRUNC('day', NOW())"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_today query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_today query failed: {}", e)})),
        )
    })?;

    // Revenue last 7 days.
    let revenue_week: f64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(total), 0)::FLOAT8
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= NOW() - INTERVAL '7 days'"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_week query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_week query failed: {}", e)})),
        )
    })?;

    // Revenue current calendar month.
    let revenue_month: f64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(total), 0)::FLOAT8
           FROM orders
           WHERE status = 'completed'
             AND completed_at >= DATE_TRUNC('month', NOW())"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "revenue_month query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("revenue_month query failed: {}", e)})),
        )
    })?;

    // MRR: sum active membership plan prices, normalized by billing cycle.
    // monthly => *1, quarterly => /3, annual/yearly => /12.
    let mrr: f64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(
                CASE LOWER(mp.billing_cycle)
                    WHEN 'monthly'   THEN mp.price
                    WHEN 'quarterly' THEN mp.price / 3.0
                    WHEN 'annual'    THEN mp.price / 12.0
                    WHEN 'yearly'    THEN mp.price / 12.0
                    ELSE mp.price
                END
           ), 0)::FLOAT8
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status = 'active'"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "mrr query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("mrr query failed: {}", e)})),
        )
    })?;

    // Pending and failed order counts.
    let pending_count: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM orders
           WHERE status = 'pending'"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "pending_count query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("pending_count query failed: {}", e)})),
        )
    })?;

    let failed_count: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(COUNT(*), 0)::BIGINT
           FROM orders
           WHERE status = 'failed'"#,
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "failed_count query failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("failed_count query failed: {}", e)})),
        )
    })?;

    Ok(Json(PaymentSummary {
        revenue: revenue_month,
        revenue_today,
        revenue_week,
        revenue_month,
        mrr,
        pending_count,
        failed_count,
        last_updated: chrono::Utc::now(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Router
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/checkout", post(create_checkout))
        .route("/portal", post(create_portal))
        .route("/webhook", post(webhook))
        .route("/refund", post(create_refund))
        .route("/config", get(get_config))
        .route("/invoice", post(generate_invoice))
        .route("/retry", post(retry_payment))
        // FIX-2026-04-26: payment summary for /admin/dashboard — replaces 404
        // when the page calls GET /api/payments/summary.
        .route("/summary", get(get_summary))
}
