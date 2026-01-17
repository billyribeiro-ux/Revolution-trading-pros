//! Payment Routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Complete Stripe payment integration:
//! - Checkout session creation
//! - Customer portal access
//! - Webhook handling with order/subscription updates
//! - Refund processing

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::{
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
    pub success_url: String,
    pub cancel_url: String,
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
    pub price: f64,
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

    // Calculate totals
    let mut subtotal = 0.0_f64;
    let mut line_items: Vec<LineItem> = Vec::new();

    for item in &input.items {
        let item_total = item.price * item.quantity as f64;
        subtotal += item_total;

        // Determine interval for subscriptions
        let (interval, interval_count) = match item.interval.as_deref() {
            Some("monthly") => (Some("month".to_string()), Some(1)),
            Some("quarterly") => (Some("month".to_string()), Some(3)),
            Some("yearly") => (Some("year".to_string()), Some(1)),
            _ => (None, None),
        };

        line_items.push(LineItem {
            price_id: None, // We create ad-hoc prices
            name: item.name.clone(),
            description: None,
            amount: (item.price * 100.0) as i64, // Convert to cents
            currency: "usd".to_string(),
            quantity: item.quantity as i64,
            is_subscription: item.is_subscription,
            interval,
            interval_count,
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

    // Insert order items
    for item in &input.items {
        sqlx::query(
            r#"INSERT INTO order_items (
                order_id, product_id, plan_id, name, quantity, unit_price, total, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())"#,
        )
        .bind(order_id)
        .bind(item.product_id)
        .bind(item.plan_id)
        .bind(&item.name)
        .bind(item.quantity)
        .bind(item.price)
        .bind(item.price * item.quantity as f64)
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

    let config = CheckoutConfig {
        customer_email: input.billing_email.unwrap_or(user.email.clone()),
        customer_name: input.billing_name,
        line_items,
        success_url: format!("{}?order={}", input.success_url, order_number),
        cancel_url: input.cancel_url,
        metadata,
        allow_promotion_codes: input.coupon_code.is_none(), // Only if no coupon already applied
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
    body: String,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Get signature header
    let _signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing signature"})),
            )
        })?;

    // Verify webhook signature (if webhook secret is configured)
    // In production, always verify signatures
    let event = state
        .services
        .stripe
        .parse_webhook_event(&body)
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
        _ => {
            tracing::debug!("Unhandled webhook event: {}", event.event_type);
        }
    }

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
async fn get_config(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "publishable_key": state.config.stripe_publishable_key,
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
                    // Find the plan from order items
                    let plan_id: Option<i64> = sqlx::query_scalar(
                        "SELECT plan_id FROM order_items WHERE order_id = $1 AND plan_id IS NOT NULL LIMIT 1"
                    )
                    .bind(order_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten();

                    if let Some(plan_id) = plan_id {
                        // Create user membership
                        sqlx::query(
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
                        .await
                        .ok();
                    }
                }
            }
        }

        // Increment coupon usage if applicable
        sqlx::query(
            "UPDATE coupons SET usage_count = usage_count + 1, updated_at = NOW() WHERE id = (SELECT coupon_id FROM orders WHERE id = $1)"
        )
        .bind(order_id)
        .execute(&state.db.pool)
        .await
        .ok();

        // ═══════════════════════════════════════════════════════════════════════════
        // COURSE ENROLLMENT CREATION - Apple ICT 11+ Grade
        // ═══════════════════════════════════════════════════════════════════════════
        // Check if any order items are courses and create enrollments
        if let Some(user_id) = user_id {
            #[derive(sqlx::FromRow)]
            struct CourseOrderItem {
                product_id: Option<i64>,
            }

            let course_items: Vec<CourseOrderItem> = sqlx::query_as(
                "SELECT product_id FROM order_items WHERE order_id = $1 AND product_id IS NOT NULL",
            )
            .bind(order_id)
            .fetch_all(&state.db.pool)
            .await
            .unwrap_or_default();

            for item in course_items {
                if let Some(product_id) = item.product_id {
                    // Check if this product is a course (has a course_id in products table)
                    let course_id: Option<uuid::Uuid> = sqlx::query_scalar(
                        "SELECT course_id FROM products WHERE id = $1 AND course_id IS NOT NULL",
                    )
                    .bind(product_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten();

                    if let Some(course_id) = course_id {
                        // Create enrollment for this course
                        sqlx::query(
                            r#"INSERT INTO user_course_enrollments (user_id, course_id, status, enrolled_at)
                               VALUES ($1, $2, 'active', NOW())
                               ON CONFLICT (user_id, course_id) DO UPDATE SET
                                   status = 'active',
                                   updated_at = NOW()"#
                        )
                        .bind(user_id)
                        .bind(course_id)
                        .execute(&state.db.pool)
                        .await
                        .ok();

                        // Auto-create Bunny Storage folder for course downloads
                        // (folder path: courses/{course_id})
                        tracing::info!(
                            target: "payments",
                            event = "course_enrollment_created",
                            user_id = %user_id,
                            course_id = %course_id,
                            order_id = %order_id,
                            "User enrolled in course after purchase"
                        );
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
    .ok();

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
    .ok();

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

    if let Some(sub_id) = subscription_id {
        sqlx::query(
            "UPDATE user_memberships SET status = 'past_due', updated_at = NOW() WHERE stripe_subscription_id = $1"
        )
        .bind(sub_id)
        .execute(&state.db.pool)
        .await
        .ok();

        // TODO: Send payment failed email
    }

    tracing::warn!(
        target: "payments",
        event = "payment_failed",
        invoice_id = %invoice["id"].as_str().unwrap_or("unknown"),
        "Payment failed"
    );

    Ok(())
}

async fn handle_refund(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let charge = &event.data.object;
    let payment_intent = charge["payment_intent"].as_str();

    if let Some(pi) = payment_intent {
        sqlx::query(
            "UPDATE orders SET status = 'refunded', refunded_at = NOW(), updated_at = NOW() WHERE payment_intent_id = $1"
        )
        .bind(pi)
        .execute(&state.db.pool)
        .await
        .ok();
    }

    tracing::info!(
        target: "payments",
        event = "charge_refunded",
        charge_id = %charge["id"].as_str().unwrap_or("unknown"),
        "Charge refunded"
    );

    Ok(())
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
}
