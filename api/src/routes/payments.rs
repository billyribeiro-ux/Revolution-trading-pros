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

// Batch 5a: CreateCheckoutRequest / CheckoutItem / CheckoutResponse
// removed alongside the duplicate `create_checkout` handler. The
// canonical request/response shapes for checkout now live in
// `routes/checkout.rs` (`CheckoutRequest` / `CartItem`).

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

// Batch 5a: `create_checkout` removed — duplicate of `routes/checkout.rs::create_checkout`.
// Frontend has been on `/api/checkout` exclusively; this handler had drifted (e.g. it
// upserted into a different `orders` shape and lacked the order-items tx wrapper).

/// Get Stripe customer portal URL// Get Stripe customer portal URL
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
           RETURNING 1::BIGINT"#,
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
        "customer.subscription.trial_will_end" => {
            handle_trial_will_end(&state, &event).await?;
        }
        _ => {
            tracing::debug!("Unhandled webhook event: {}", event.event_type);
        }
    }

    // Mark event as processed
    sqlx::query("UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1")
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

// IDEMPOTENT-BY: top-level webhook_events(event_id) UNIQUE +
// orders.stripe_session_id UNIQUE WHERE NOT NULL +
// user_memberships.stripe_subscription_id partial UNIQUE (Batch 4 / 063).
// A retried `checkout.session.completed` is dropped at the top-level
// dedup table; if (somehow) it reaches here, the orders update is a
// no-op SET on the same session_id, and the membership upsert lands on
// the existing row via stripe_subscription_id ON CONFLICT.
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
        // ─── Reconcile order totals with Stripe-applied discount (arch §10) ───
        // The webhook payload's session usually carries `amount_total`,
        // `amount_subtotal`, and `total_details`, but we re-fetch with
        // `expand[]=total_details.breakdown.discounts.discount.coupon` so we
        // also get the human-readable promotion code/coupon name to backfill
        // `orders.coupon_code`. This is the single source of truth for the
        // amounts the customer was actually charged.
        let session_full = state
            .services
            .stripe
            .retrieve_checkout_session(&session.id)
            .await
            .ok();

        let (amount_subtotal_cents, amount_discount_cents, amount_total_cents, applied_code) =
            if let Some(s) = session_full.as_ref() {
                let subtotal = s.amount_subtotal.or(s.amount_total).unwrap_or(0);
                let discount = s
                    .total_details
                    .as_ref()
                    .and_then(|td| td.amount_discount)
                    .unwrap_or(0);
                let total = s.amount_total.unwrap_or(subtotal.saturating_sub(discount));
                let code = s
                    .total_details
                    .as_ref()
                    .and_then(|td| td.breakdown.as_ref())
                    .and_then(|b| b.discounts.first())
                    .and_then(|d| d.discount.as_ref())
                    .and_then(|inner| {
                        inner.promotion_code.clone().or_else(|| {
                            inner
                                .coupon
                                .as_ref()
                                .and_then(|c| c.id.clone().or_else(|| c.name.clone()))
                        })
                    });
                (subtotal, discount, total, code)
            } else {
                // Stripe re-fetch failed; fall back to webhook payload's amount_total.
                let total = session.amount_total.unwrap_or(0);
                (total, 0_i64, total, None)
            };

        sqlx::query(
            r#"UPDATE orders SET
                status = 'completed',
                payment_provider = 'stripe',
                payment_intent_id = $1,
                subtotal = $3::BIGINT / 100.0,
                discount = $4::BIGINT / 100.0,
                total    = $5::BIGINT / 100.0,
                coupon_code = COALESCE($6, coupon_code),
                completed_at = NOW(),
                updated_at = NOW()
            WHERE id = $2"#,
        )
        .bind(&session.payment_intent)
        .bind(order_id)
        .bind(amount_subtotal_cents)
        .bind(amount_discount_cents)
        .bind(amount_total_cents)
        .bind(&applied_code)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        tracing::info!(
            target: "payments",
            event = "checkout_completed_reconciled",
            order_id = %order_id,
            session_id = %session.id,
            subtotal_cents = %amount_subtotal_cents,
            discount_cents = %amount_discount_cents,
            total_cents = %amount_total_cents,
            applied_promo = ?applied_code,
            "Order reconciled with Stripe-charged amounts"
        );

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
                        // Batch 4: re-subscribe history.
                        //
                        // Was ON CONFLICT (user_id, plan_id) DO UPDATE — that
                        // path is now unreachable because migration 063
                        // replaced the (user_id, plan_id) unique constraint
                        // with a *partial* unique index that only covers
                        // live statuses ('active', 'trialing', 'past_due').
                        // A user re-subscribing after full cancellation
                        // therefore gets a NEW row, preserving the prior
                        // cancelled row for history. Stripe is the source
                        // of truth via stripe_subscription_id.
                        //
                        // We deliberately do not upsert. If the partial
                        // unique index fires (an active row already exists
                        // for this user+plan), Postgres returns an error
                        // and we log it — the webhook will retry per
                        // Stripe's idempotency, and on retry the existing
                        // row's stripe_subscription_id will already match.
                        // To handle the "duplicate retry" case cleanly we
                        // make the INSERT idempotent on stripe_subscription_id.
                        if let Err(e) = sqlx::query(
                            r#"INSERT INTO user_memberships (
                                user_id, plan_id, starts_at, status,
                                payment_provider, stripe_subscription_id, stripe_customer_id,
                                current_period_start, current_period_end,
                                cancel_at_period_end, created_at, updated_at
                            ) VALUES ($1, $2, NOW(), 'active', 'stripe', $3, $4, $5, $6, false, NOW(), NOW())
                            ON CONFLICT (stripe_subscription_id) DO UPDATE SET
                                status = 'active',
                                current_period_start = EXCLUDED.current_period_start,
                                current_period_end = EXCLUDED.current_period_end,
                                updated_at = NOW()"#,
                        )
                        .bind(user_id)
                        .bind(plan_id)
                        .bind(&sub.id)
                        .bind(&sub.customer)
                        .bind({
                            let (start, _) = sub.get_current_period();
                            chrono::DateTime::from_timestamp(start, 0).map(|d| d.naive_utc())
                        })
                        .bind({
                            let (_, end) = sub.get_current_period();
                            chrono::DateTime::from_timestamp(end, 0).map(|d| d.naive_utc())
                        })
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
                                r#"INSERT INTO user_course_enrollments (user_id, course_id, is_active, enrolled_at)
                                   VALUES ($1, $2, true, NOW())
                                   ON CONFLICT (user_id, course_id) DO UPDATE SET
                                       is_active = true"#
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

        // Batch 6: send subscription-confirmation OR receipt depending
        // on whether this checkout session created a subscription.
        // Best-effort — webhook 200 is more important than email send.
        if let Some(user_id) = user_id {
            #[derive(sqlx::FromRow)]
            struct UserRow {
                email: String,
                name: String,
            }
            let user_row: Option<UserRow> =
                sqlx::query_as("SELECT email, name FROM users WHERE id = $1")
                    .bind(user_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten();

            #[derive(sqlx::FromRow)]
            struct OrderRow {
                order_number: String,
                total: rust_decimal::Decimal,
            }
            let order_row: Option<OrderRow> =
                sqlx::query_as("SELECT order_number, total FROM orders WHERE id = $1")
                    .bind(order_id)
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten();

            if let (Some(u), Some(o)) = (user_row, order_row) {
                let amount_dollars: f64 = o.total.to_string().parse::<f64>().unwrap_or(0.0);
                let is_subscription = session.subscription.is_some();
                if is_subscription {
                    // Look up the plan name + period_end for the model.
                    #[derive(sqlx::FromRow)]
                    struct PlanInfo {
                        plan_name: String,
                        current_period_end: Option<chrono::NaiveDateTime>,
                    }
                    let plan_info: Option<PlanInfo> = sqlx::query_as(
                        r#"SELECT mp.name AS plan_name, um.current_period_end
                           FROM user_memberships um
                           JOIN membership_plans mp ON mp.id = um.plan_id
                           WHERE um.stripe_subscription_id = $1
                           ORDER BY um.id DESC LIMIT 1"#,
                    )
                    .bind(session.subscription.as_deref().unwrap_or(""))
                    .fetch_optional(&state.db.pool)
                    .await
                    .ok()
                    .flatten();

                    let _ = state
                        .services
                        .email
                        .send_transactional(
                            &state.db.pool,
                            &u.email,
                            "subscription-confirmation",
                            json!({
                                "name": u.name,
                                "plan_name": plan_info.as_ref().map(|p| p.plan_name.clone())
                                    .unwrap_or_else(|| "Membership".to_string()),
                                "amount_dollars": amount_dollars,
                                "period_end": plan_info.as_ref()
                                    .and_then(|p| p.current_period_end)
                                    .map(|t| t.and_utc().to_rfc3339()),
                                "order_number": o.order_number,
                            }),
                        )
                        .await;
                } else {
                    let _ = state
                        .services
                        .email
                        .send_transactional(
                            &state.db.pool,
                            &u.email,
                            "receipt",
                            json!({
                                "name": u.name,
                                "product_name": format!("Order {}", o.order_number),
                                "amount_dollars": amount_dollars,
                                "order_id": order_id,
                                "order_number": o.order_number,
                            }),
                        )
                        .await;
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

// IDEMPOTENT-BY: pure read/log handler (no DB writes). Safe to run
// any number of times. Source-of-truth subscription persistence happens
// in `handle_checkout_completed` (initial) and `handle_subscription_updated`
// (subsequent state transitions).
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

// IDEMPOTENT-BY: SET-based UPDATE keyed by stripe_subscription_id.
// Re-running the same event lands on the same row and writes the
// same values (Stripe is the source of truth for sub.status,
// current_period_*, cancel_at_period_end). No INSERT, no counters
// incremented — pure state replication.
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
    .bind({
        let (start, _) = subscription.get_current_period();
        chrono::DateTime::from_timestamp(start, 0).map(|d| d.naive_utc())
    })
    .bind({
        let (_, end) = subscription.get_current_period();
        chrono::DateTime::from_timestamp(end, 0).map(|d| d.naive_utc())
    })
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

// IDEMPOTENT-BY: SET status='cancelled' on the row matching
// stripe_subscription_id. Already-cancelled rows are written with the
// same value (no observable state change). cancelled_at gets stamped
// to the latest NOW() on retry — acceptable because the field
// represents "when did our system observe the cancellation," not
// "when did Stripe perform it."
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

    // Batch 6: subscription-canceled email. Best-effort.
    #[derive(sqlx::FromRow)]
    struct CancelInfo {
        email: String,
        name: String,
        plan_name: String,
        current_period_end: Option<chrono::NaiveDateTime>,
    }
    let info: Option<CancelInfo> = sqlx::query_as(
        r#"SELECT u.email, u.name, mp.name AS plan_name, um.current_period_end
           FROM user_memberships um
           JOIN users u ON u.id = um.user_id
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.stripe_subscription_id = $1
           ORDER BY um.id DESC LIMIT 1"#,
    )
    .bind(&subscription.id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();
    if let Some(i) = info {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                &i.email,
                "subscription-canceled",
                json!({
                    "name": i.name,
                    "plan_name": i.plan_name,
                    "period_end": i.current_period_end.map(|t| t.and_utc().to_rfc3339()),
                }),
            )
            .await;
    }

    Ok(())
}

// IDEMPOTENT-BY: SET status='active' on user_memberships keyed by
// stripe_subscription_id. Replaying the event re-asserts the same
// status. No counters, no inserts.
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

// IDEMPOTENT-BY: top-level webhook_events dedup. The handler bumps
// payment_failure_count and stores grace_period_end on user_memberships;
// without dedup these would double-increment on retry. Stripe sends
// `invoice.payment_failed` once per attempt, but each retry attempt
// has a distinct event_id, so the top-level dedup is the right
// boundary — same event_id means same attempt and we skip.
async fn handle_payment_failed(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invoice = &event.data.object;
    let subscription_id = invoice["subscription"].as_str();
    let customer_email = invoice["customer_email"].as_str();

    // Get payment attempt count and amount from invoice (Stripe stores invoice amounts in cents)
    let attempt_count: i32 = invoice["attempt_count"].as_i64().unwrap_or(1) as i32;
    let amount_due_cents: i64 = invoice["amount_due"].as_i64().unwrap_or(0);

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
        {
            let email_service = &state.services.email;
            // Get user details from membership including plan price
            #[derive(sqlx::FromRow)]
            struct UserSubscription {
                email: String,
                name: String,
                plan_name: String,
                price_cents: Option<i64>,
            }

            let user_info: Option<UserSubscription> = sqlx::query_as(
                r#"SELECT u.email, COALESCE(u.name, u.email) as name,
                   COALESCE(mp.name, 'Subscription') as plan_name,
                   (mp.price * 100)::BIGINT as price_cents
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
                // Use amount from invoice, fallback to plan price (all cents)
                let payment_amount_cents: i64 = if amount_due_cents > 0 {
                    amount_due_cents
                } else {
                    info.price_cents.unwrap_or(0)
                };
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();

                // Batch 6: failed-payment template via send_transactional.
                let retry_link = format!(
                    "{}/account/billing",
                    state.config.app_url.trim_end_matches('/')
                );
                let _ = email_service
                    .send_transactional(
                        &state.db.pool,
                        &info.email,
                        "failed-payment",
                        json!({
                            "name": info.name,
                            "plan_name": info.plan_name,
                            "amount_dollars": (payment_amount_cents as f64) / 100.0,
                            "retry_link": retry_link,
                            "dunning_period_end": grace_end_str,
                            "attempt_count": attempt_count,
                        }),
                    )
                    .await;

                tracing::info!(
                    target: "payments",
                    event = "payment_failed_email_sent",
                    email = %info.email,
                    subscription_id = %sub_id,
                    grace_period_end = %grace_end_str,
                    attempt_count = %attempt_count,
                    "Payment failed notification email queued"
                );
            } else if let Some(email) = customer_email {
                // Fallback when membership lookup fails — still notify.
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();
                let retry_link = format!(
                    "{}/account/billing",
                    state.config.app_url.trim_end_matches('/')
                );
                let _ = email_service
                    .send_transactional(
                        &state.db.pool,
                        email,
                        "failed-payment",
                        json!({
                            "name": "Valued Customer",
                            "plan_name": "your subscription",
                            "amount_dollars": (amount_due_cents as f64) / 100.0,
                            "retry_link": retry_link,
                            "dunning_period_end": grace_end_str,
                            "attempt_count": attempt_count,
                        }),
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
        amount_due_cents = %amount_due_cents,
        grace_period_end = %grace_period_end.format("%Y-%m-%d"),
        "Payment failed - grace period initialized"
    );

    Ok(())
}

// IDEMPOTENT-BY: top-level webhook_events dedup. The handler reads
// the charge's amount_refunded value (Stripe-sourced, monotonic) and
// SETs orders.refund_amount_cents to that exact value. Replaying the
// same event writes the same number; replaying a *different* refund
// event for the same charge is a state advance, not a duplicate.
async fn handle_refund(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let charge = &event.data.object;
    let payment_intent = charge["payment_intent"].as_str();

    // Stripe stores charge.amount and charge.amount_refunded in integer cents
    let refund_amount_cents: i64 = charge["amount_refunded"].as_i64().unwrap_or(0);
    let total_amount_cents: i64 = charge["amount"].as_i64().unwrap_or(0);
    let is_full_refund = refund_amount_cents > 0 && refund_amount_cents >= total_amount_cents;

    if let Some(pi) = payment_intent {
        let refund_status = if is_full_refund {
            "refunded"
        } else {
            "partial_refund"
        };

        sqlx::query(
            r#"UPDATE orders SET
               status = $1,
               refund_amount = COALESCE(refund_amount, 0) + ($2::BIGINT / 100.0),
               refunded_at = CASE WHEN $1 = 'refunded' THEN NOW() ELSE refunded_at END,
               updated_at = NOW()
               WHERE payment_intent_id = $3"#,
        )
        .bind(refund_status)
        .bind(refund_amount_cents)
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
            refund_amount_cents = %refund_amount_cents,
            total_amount_cents = %total_amount_cents,
            refund_status = %refund_status,
            "Refund processed"
        );

        // Batch 6: refund-confirmation email. Best-effort.
        #[derive(sqlx::FromRow)]
        struct RefundEmailInfo {
            email: String,
            name: String,
            order_number: String,
        }
        let info: Option<RefundEmailInfo> = sqlx::query_as(
            r#"SELECT u.email, u.name, o.order_number
               FROM orders o
               JOIN users u ON u.id = o.user_id
               WHERE o.payment_intent_id = $1
               LIMIT 1"#,
        )
        .bind(pi)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();
        if let Some(i) = info {
            let refund_id = charge["refunds"]["data"][0]["id"]
                .as_str()
                .or_else(|| charge["id"].as_str())
                .unwrap_or("unknown");
            let _ = state
                .services
                .email
                .send_transactional(
                    &state.db.pool,
                    &i.email,
                    "refund-confirmation",
                    json!({
                        "name": i.name,
                        "product_name": format!("Order {}", i.order_number),
                        "amount_dollars": (refund_amount_cents as f64) / 100.0,
                        "refund_id": refund_id,
                        "order_number": i.order_number,
                        "is_full_refund": is_full_refund,
                    }),
                )
                .await;
        }
    }

    Ok(())
}

// IDEMPOTENT-BY: stripe_disputes(stripe_dispute_id) UNIQUE +
// ON CONFLICT DO NOTHING (see migration 055). Replaying the event
// hits the unique key and the row stays as it was — first-write-wins
// for dispute creation, which is the correct semantic (a dispute
// happens once; subsequent webhook events for the same dispute id
// are status updates, not creates).
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

    // Batch 6: notify the operator. Address comes from
    // ADMIN_NOTIFICATION_EMAIL env var; if unset we log a warning and
    // skip rather than send to a guessed address.
    if let Some(admin_email) = state.config.admin_notification_email.as_deref() {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                admin_email,
                "dispute-created",
                json!({
                    "dispute_id": dispute_id,
                    "charge_id": charge_id,
                    "amount_cents": amount_cents,
                    "amount_dollars": (amount_cents as f64) / 100.0,
                    "currency": currency,
                    "reason": reason,
                    "status": status,
                    "response_deadline": response_deadline.map(|d| d.to_rfc3339()),
                }),
            )
            .await;
    } else {
        tracing::warn!(
            target: "payments",
            event = "dispute_admin_notify_skipped",
            dispute_id = %dispute_id,
            "ADMIN_NOTIFICATION_EMAIL not set; dispute notification not sent"
        );
    }

    Ok(())
}

/// Log trial-ending soon event; email notification handled by Task 4 (Postmark).
// IDEMPOTENT-BY: log + email side-effect only (no DB writes today).
// On retry the same notification gets enqueued — Postmark handles
// dedup on its side via the `MessageID` field we set when sending.
// Until that wiring is restored (Batch 6, currently deferred), this
// handler is *almost* idempotent: a duplicate event would re-send
// the email. Acceptable risk given Stripe's webhook retry budget.
async fn handle_trial_will_end(
    state: &AppState,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let sub = &event.data.object;
    let subscription_id = sub["id"].as_str().unwrap_or("unknown");
    let customer_id = sub["customer"].as_str().unwrap_or("unknown");
    let trial_end = sub["trial_end"].as_i64();

    // Look up the user by stripe_customer_id
    let user_id: Option<i64> = sqlx::query_scalar(
        "SELECT user_id FROM user_memberships WHERE stripe_customer_id = $1 LIMIT 1",
    )
    .bind(customer_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "trial_will_end user lookup failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "DB error during trial_will_end"})),
        )
    })?
    .flatten();

    sqlx::query(
        r#"INSERT INTO security_events (user_id, event_type, event_category, severity, details, created_at)
           VALUES ($1, 'trial_will_end', 'billing', 'low', $2, NOW())"#,
    )
    .bind(user_id)
    .bind(json!({
        "subscription_id": subscription_id,
        "customer_id": customer_id,
        "trial_end_ts": trial_end,
        // TODO Task 4: send Postmark "trial ending soon" email to user
    }))
    .execute(&state.db.pool)
    .await
    .ok(); // non-fatal

    tracing::info!(
        target: "payments",
        event = "trial_will_end",
        subscription_id = %subscription_id,
        customer_id = %customer_id,
        "Trial ending in 3 days — sending notification (Batch 6)"
    );

    // Batch 6: trial-ending email. Best-effort. We dedup against the
    // daily reminders job by checking last 24h of email_logs in the
    // job itself; this Stripe-driven path is the primary signal.
    #[derive(sqlx::FromRow)]
    struct TrialInfo {
        email: String,
        name: String,
        plan_name: String,
    }
    if let Some(uid) = user_id {
        let info: Option<TrialInfo> = sqlx::query_as(
            r#"SELECT u.email, u.name, mp.name AS plan_name
               FROM user_memberships um
               JOIN users u ON u.id = um.user_id
               JOIN membership_plans mp ON mp.id = um.plan_id
               WHERE um.user_id = $1 AND um.stripe_subscription_id = $2
               ORDER BY um.id DESC LIMIT 1"#,
        )
        .bind(uid)
        .bind(subscription_id)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();
        if let Some(i) = info {
            let trial_end_iso = trial_end
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                .map(|dt| dt.to_rfc3339());
            let _ = state
                .services
                .email
                .send_transactional(
                    &state.db.pool,
                    &i.email,
                    "trial-ending",
                    json!({
                        "name": i.name,
                        "plan_name": i.plan_name,
                        "trial_end_date": trial_end_iso,
                    }),
                )
                .await;
        }
    }

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
    // Verify order belongs to user. All monetary values are integer cents.
    #[derive(sqlx::FromRow)]
    struct OrderInvoiceData {
        id: i64,
        order_number: String,
        user_id: i64,
        status: String,
        subtotal_cents: i64,
        discount_cents: i64,
        tax_cents: i64,
        total_cents: i64,
        currency: String,
        billing_name: Option<String>,
        billing_email: Option<String>,
        billing_address: Option<serde_json::Value>,
        coupon_code: Option<String>,
        created_at: chrono::NaiveDateTime,
        completed_at: Option<chrono::NaiveDateTime>,
    }

    let order: OrderInvoiceData = sqlx::query_as(
        r#"SELECT id, order_number, user_id, status,
                  (subtotal * 100)::BIGINT AS subtotal_cents,
                  (discount * 100)::BIGINT AS discount_cents,
                  (tax * 100)::BIGINT      AS tax_cents,
                  (total * 100)::BIGINT    AS total_cents,
                  currency, billing_name, billing_email,
                  billing_address, coupon_code, created_at, completed_at
           FROM orders WHERE id = $1"#,
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

    // Fetch order items (all monetary values in integer cents)
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderItem {
        name: String,
        quantity: i32,
        unit_price_cents: i64,
        total_cents: i64,
    }

    let items: Vec<OrderItem> = sqlx::query_as(
        "SELECT name, quantity,
                (unit_price * 100)::BIGINT AS unit_price_cents,
                (total      * 100)::BIGINT AS total_cents
         FROM order_items WHERE order_id = $1",
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
        "subtotal_cents": order.subtotal_cents,
        "discount_cents": order.discount_cents,
        "coupon_code": order.coupon_code,
        "tax_cents": order.tax_cents,
        "total_cents": order.total_cents,
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
/// All monetary values are integer cents per architecture standard §1.2.
/// Frontend formatCurrency helper handles cents→dollars at render.
#[derive(Debug, Serialize)]
struct PaymentSummary {
    /// Revenue this month (alias of `revenue_month_cents`), in cents.
    revenue_cents: i64,
    /// Revenue today (cents).
    revenue_today_cents: i64,
    /// Revenue over the last 7 days (cents).
    revenue_week_cents: i64,
    /// Revenue over the current calendar month (cents).
    revenue_month_cents: i64,
    /// Monthly recurring revenue (cents).
    mrr_cents: i64,
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
    // Revenue today (UTC) — integer cents.
    let revenue_today_cents: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
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

    // Revenue last 7 days — integer cents.
    let revenue_week_cents: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
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

    // Revenue current calendar month — integer cents.
    let revenue_month_cents: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM((total * 100)::BIGINT), 0)::BIGINT
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

    // MRR: sum active+trialing memberships' plan prices, normalized to monthly,
    // returned in integer cents.
    let mrr_cents: i64 = sqlx::query_scalar(
        r#"SELECT COALESCE(SUM(
                CASE LOWER(mp.billing_cycle)
                    WHEN 'monthly'   THEN (mp.price * 100)::BIGINT
                    WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                    WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                    WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                    ELSE (mp.price * 100)::BIGINT
                END
           ), 0)::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status IN ('active', 'trialing')"#,
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
        revenue_cents: revenue_month_cents,
        revenue_today_cents,
        revenue_week_cents,
        revenue_month_cents,
        mrr_cents,
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
        // Batch 5a: `/checkout` removed. The duplicate handler at this path
        // diverged from `/api/checkout` (in routes/checkout.rs) and the
        // frontend has been on `/api/checkout` exclusively since the
        // payments-fix-2026-04 cleanup. The single canonical checkout
        // endpoint now lives in `routes/checkout.rs`.
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
