//! Checkout & Cart routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// CART
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CartItem {
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub items: Vec<CartItem>,
    pub coupon_code: Option<String>,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    // success_path / cancel_path must start with "/" — full URLs are built server-side
    pub success_path: Option<String>,
    pub cancel_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CalculateTaxRequest {
    pub items: Vec<CartItem>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
}

/// Product row for cart calculations (price in integer cents)
#[derive(Debug, sqlx::FromRow)]
struct ProductPrice {
    id: i64,
    name: String,
    price_cents: i64,
}

/// Plan row for cart calculations (price in integer cents)
#[derive(Debug, sqlx::FromRow)]
struct PlanPrice {
    id: i64,
    name: String,
    price_cents: i64,
    stripe_price_id: Option<String>,
    billing_cycle: String,
}

/// Create checkout session
async fn create_checkout(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CheckoutRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.items.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Cart is empty"})),
        ));
    }

    let mut line_items = Vec::new();
    let mut subtotal_cents: i64 = 0;

    // Calculate totals from products
    for item in &input.items {
        if let Some(product_id) = item.product_id {
            let product: ProductPrice = sqlx::query_as(
                "SELECT id, name, (price * 100)::BIGINT AS price_cents FROM products WHERE id = $1 AND is_active = true",
            )
            .bind(product_id)
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
                    Json(json!({"error": format!("Product {} not found", product_id)})),
                )
            })?;

            let qty = item.quantity.unwrap_or(1);
            let line_total_cents: i64 = product.price_cents.saturating_mul(qty as i64);
            subtotal_cents = subtotal_cents.saturating_add(line_total_cents);

            line_items.push(json!({
                "type": "product",
                "id": product.id,
                "name": product.name,
                "price_cents": product.price_cents,
                "quantity": qty,
                "total_cents": line_total_cents
            }));
        }

        if let Some(plan_id) = item.plan_id {
            let plan: PlanPrice = sqlx::query_as(
                "SELECT id, name, (price * 100)::BIGINT AS price_cents, stripe_price_id, billing_cycle FROM membership_plans WHERE id = $1 AND is_active = true",
            )
            .bind(plan_id)
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
                    Json(json!({"error": format!("Plan {} not found", plan_id)})),
                )
            })?;

            // Require a Stripe price ID — ad-hoc pricing is forbidden by PAYMENTS_ARCHITECTURE_STANDARD §1
            if plan.stripe_price_id.is_none() {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"error": format!("Plan {} has no Stripe price configured", plan_id)})),
                ));
            }

            subtotal_cents = subtotal_cents.saturating_add(plan.price_cents);

            line_items.push(json!({
                "type": "subscription",
                "id": plan.id,
                "name": plan.name,
                "price_cents": plan.price_cents,
                "stripe_price_id": plan.stripe_price_id,
                "billing_cycle": plan.billing_cycle,
                "quantity": 1,
                "total_cents": plan.price_cents
            }));
        }
    }

    // Apply coupon if provided - all math in integer cents
    let mut discount_cents: i64 = 0;
    let mut coupon_info = None;
    let mut applied_coupon_id: Option<i64> = None;

    if let Some(ref code) = input.coupon_code {
        #[derive(sqlx::FromRow)]
        struct CouponInfo {
            id: i64,
            code: String,
            #[sqlx(rename = "type")]
            coupon_type: String,
            value_cents: i64,
            max_uses: i32,
            current_uses: i32,
            expiry_date: Option<chrono::NaiveDateTime>,
            min_purchase_amount_cents: i64,
        }

        let coupon: Option<CouponInfo> = sqlx::query_as(
            r#"SELECT id, code, type,
                      (value * 100)::BIGINT                AS value_cents,
                      max_uses, current_uses, expiry_date,
                      (min_purchase_amount * 100)::BIGINT  AS min_purchase_amount_cents
               FROM coupons
               WHERE UPPER(code) = UPPER($1) AND is_active = true"#,
        )
        .bind(code)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "checkout", error = %e, "Failed to fetch coupon");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to validate coupon"})),
            )
        })?;

        if let Some(c) = coupon {
            if let Some(expiry) = c.expiry_date {
                if expiry < chrono::Utc::now().naive_utc() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Coupon has expired"})),
                    ));
                }
            }

            if c.max_uses > 0 && c.current_uses >= c.max_uses {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Coupon has reached its usage limit"})),
                ));
            }

            if c.min_purchase_amount_cents > 0 && subtotal_cents < c.min_purchase_amount_cents {
                let dollars = c.min_purchase_amount_cents as f64 / 100.0; // display only
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": format!("Minimum purchase of ${:.2} required for this coupon", dollars)}),
                    ),
                ));
            }

            // For percent coupons, value is stored as e.g. 50.00 (i.e. value_cents=5000) representing 50%
            discount_cents = if c.coupon_type == "percentage" {
                let percent = c.value_cents / 100; // 5000 → 50
                (subtotal_cents.saturating_mul(percent)) / 100
            } else {
                c.value_cents
            };

            if discount_cents > subtotal_cents {
                discount_cents = subtotal_cents;
            }

            applied_coupon_id = Some(c.id);
            coupon_info = Some(json!({
                "id": c.id,
                "code": c.code,
                "type": c.coupon_type,
                "value_cents": c.value_cents,
                "discount_cents": discount_cents
            }));
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid coupon code"})),
            ));
        }
    }

    // Tax (simplified - 0% for now, should integrate with tax service)
    let tax_cents: i64 = 0;
    let total_cents: i64 = subtotal_cents.saturating_sub(discount_cents).saturating_add(tax_cents);

    // Generate order number
    let order_number = format!(
        "ORD-{}-{}",
        chrono::Utc::now().format("%Y%m%d"),
        uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    // FIX-2026-04-26 (Priority 5, CRITICAL): wrap orders + order_items + coupon-usage
    // increment in a single transaction. Commit BEFORE the external Stripe API call so
    // a Stripe failure doesn't leave half-written rows. The post-Stripe
    // `UPDATE orders SET stripe_session_id = ...` is intentionally a single-statement
    // query that runs OUTSIDE this tx (it's a backfill, not a multi-step mutation).
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!(target: "checkout", error = %e, "Failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Create order in database - ICT 7 FIX: Include coupon_id and coupon_code
    #[derive(sqlx::FromRow)]
    struct OrderId {
        id: i64,
    }

    // Original pool reference: .fetch_one(&state.db.pool)
    // DB columns are NUMERIC(10,2); convert from cents at the SQL boundary
    let order: OrderId = sqlx::query_as(
        r#"
        INSERT INTO orders (user_id, order_number, status, subtotal, discount, tax, total, currency,
                           billing_name, billing_email, billing_address, coupon_id, coupon_code, created_at, updated_at)
        VALUES ($1, $2, 'pending',
                $3::BIGINT / 100.0, $4::BIGINT / 100.0, $5::BIGINT / 100.0, $6::BIGINT / 100.0,
                'USD', $7, $8, $9, $10, $11, NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(user.id)
    .bind(&order_number)
    .bind(subtotal_cents)
    .bind(discount_cents)
    .bind(tax_cents)
    .bind(total_cents)
    .bind(&input.billing_name)
    .bind(&input.billing_email)
    .bind(&input.billing_address)
    .bind(applied_coupon_id)
    .bind(&input.coupon_code)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "checkout", error = %e, "Failed to create order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create order"})))
    })?;

    // ICT 7 FIX: Increment coupon usage if applied
    // Original pool reference: .execute(&state.db.pool)
    if let Some(coupon_id) = applied_coupon_id {
        sqlx::query(
            "UPDATE coupons SET current_uses = current_uses + 1, updated_at = NOW() WHERE id = $1",
        )
        .bind(coupon_id)
        .execute(&mut *tx)
        .await
        .ok(); // Log but don't fail order creation
    }

    // Insert order items — convert cents → NUMERIC(10,2) at SQL boundary
    for item in &line_items {
        let price_cents = item.get("price_cents").and_then(|p| p.as_i64()).unwrap_or(0);
        let total_cents_li = item.get("total_cents").and_then(|t| t.as_i64()).unwrap_or(0);
        sqlx::query(
            r#"
            INSERT INTO order_items (order_id, product_id, plan_id, name, quantity, unit_price, total, created_at)
            VALUES ($1, $2, $3, $4, $5, $6::BIGINT / 100.0, $7::BIGINT / 100.0, NOW())
            "#
        )
        .bind(order.id)
        .bind(item.get("type").and_then(|t| if t == "product" { item.get("id").and_then(|i| i.as_i64()) } else { None }))
        .bind(item.get("type").and_then(|t| if t == "subscription" { item.get("id").and_then(|i| i.as_i64()) } else { None }))
        .bind(item.get("name").and_then(|n| n.as_str()).unwrap_or(""))
        .bind(item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(1) as i32)
        .bind(price_cents)
        .bind(total_cents_li)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    // FIX-2026-04-26 (Priority 5): COMMIT BEFORE external Stripe HTTP call.
    tx.commit().await.map_err(|e| {
        tracing::error!(target: "checkout", error = %e, "Failed to commit checkout transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Build URLs server-side — validate paths to prevent open redirect
    let success_path = input.success_path.as_deref().unwrap_or("/checkout/success");
    let cancel_path = input.cancel_path.as_deref().unwrap_or("/checkout/cancel");
    if !success_path.starts_with('/') || !cancel_path.starts_with('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "success_path and cancel_path must start with /"})),
        ));
    }
    let app_url = state.config.app_url.trim_end_matches('/');
    let success_url = format!("{}{}", app_url, success_path);
    let cancel_url = format!("{}{}", app_url, cancel_path);

    // Build Stripe line items from our cart items
    // Subscriptions use stripe_price_id (the price is Stripe-authoritative).
    // One-time products use ad-hoc pricing until products also carry stripe_price_id.
    let mut stripe_line_items: Vec<crate::services::stripe::LineItem> = Vec::new();

    for item in &line_items {
        let is_subscription = item.get("type").and_then(|t| t.as_str()) == Some("subscription");
        let stripe_price_id = item.get("stripe_price_id").and_then(|v| v.as_str()).map(|s| s.to_string());

        // Derive interval from billing_cycle for metadata purposes (Stripe already knows from price_id)
        let billing_cycle = item.get("billing_cycle").and_then(|v| v.as_str()).unwrap_or("monthly");
        let (interval, interval_count) = match billing_cycle {
            "quarterly" => ("month", 3),
            "annual"    => ("year", 1),
            _           => ("month", 1),
        };

        stripe_line_items.push(crate::services::stripe::LineItem {
            price_id: stripe_price_id, // Use Stripe price ID for subscriptions (Stripe-authoritative pricing)
            name: item
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Item")
                .to_string(),
            description: None,
            amount: item.get("price_cents").and_then(|p| p.as_i64()).unwrap_or(0),
            currency: "usd".to_string(),
            quantity: item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(1),
            is_subscription,
            interval: if is_subscription { Some(interval.to_string()) } else { None },
            interval_count: if is_subscription { Some(interval_count) } else { None },
        });
    }

    // Create Stripe checkout session with order metadata
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("order_id".to_string(), order.id.to_string());
    metadata.insert("order_number".to_string(), order_number.clone());
    metadata.insert("user_id".to_string(), user.id.to_string());

    let config = crate::services::stripe::CheckoutConfig {
        customer_email: input
            .billing_email
            .clone()
            .unwrap_or_else(|| user.email.clone()),
        customer_name: input.billing_name.clone(),
        line_items: stripe_line_items,
        success_url: format!("{}?order={}", success_url, order_number),
        cancel_url: cancel_url.clone(),
        metadata,
        allow_promotion_codes: input.coupon_code.is_none(),
        billing_address_collection: true,
        trial_period_days: None,
        trial_requires_payment_method: true,
    };

    // Create Stripe session
    let stripe_session = state
        .services
        .stripe
        .create_checkout_session(config)
        .await
        .map_err(|e| {
            tracing::error!(target: "checkout", error = %e, "Failed to create Stripe session");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Payment service error: {}", e)})),
            )
        })?;

    // Update order with Stripe session ID
    sqlx::query("UPDATE orders SET stripe_session_id = $1, updated_at = NOW() WHERE id = $2")
        .bind(&stripe_session.id)
        .bind(order.id)
        .execute(&state.db.pool)
        .await
        .ok();

    tracing::info!(
        target: "checkout",
        event = "checkout_session_created",
        order_id = %order.id,
        order_number = %order_number,
        user_id = %user.id,
        stripe_session_id = %stripe_session.id,
        total_cents = %total_cents,
        "Stripe checkout session created"
    );

    Ok(Json(json!({
        "order_id": order.id,
        "order_number": order_number,
        "line_items": line_items,
        "subtotal_cents": subtotal_cents,
        "discount_cents": discount_cents,
        "coupon": coupon_info,
        "tax_cents": tax_cents,
        "total_cents": total_cents,
        "currency": "USD",
        "stripe_session_id": stripe_session.id,
        "checkout_url": stripe_session.url,
        "success_url": format!("{}?order={}", success_url, order_number),
        "cancel_url": cancel_url
    })))
}

/// Calculate tax for cart
async fn calculate_tax(
    State(_state): State<AppState>,
    Json(input): Json<CalculateTaxRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Simplified tax calculation - in production, integrate with tax service like TaxJar
    let tax_rate = match input.country.as_deref() {
        Some("US") => match input.state.as_deref() {
            Some("CA") => 0.0725,
            Some("NY") => 0.08,
            Some("TX") => 0.0625,
            _ => 0.0,
        },
        Some("GB") => 0.20,
        Some("DE") => 0.19,
        _ => 0.0,
    };

    Ok(Json(json!({
        "tax_rate": tax_rate,
        "tax_rate_percent": format!("{:.2}%", tax_rate * 100.0),
        "country": input.country,
        "state": input.state
    })))
}

/// Get order by number
async fn get_order(
    State(state): State<AppState>,
    user: User,
    Path(order_number): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderRow {
        id: i64,
        user_id: i64,
        order_number: String,
        status: String,
        subtotal_cents: i64,
        discount_cents: i64,
        tax_cents: i64,
        total_cents: i64,
        currency: String,
        coupon_code: Option<String>,
        billing_name: Option<String>,
        billing_email: Option<String>,
        completed_at: Option<chrono::NaiveDateTime>,
        created_at: chrono::NaiveDateTime,
    }

    let order: OrderRow = sqlx::query_as(
        r#"SELECT id, user_id, order_number, status,
                  (subtotal * 100)::BIGINT AS subtotal_cents,
                  (discount * 100)::BIGINT AS discount_cents,
                  (tax * 100)::BIGINT      AS tax_cents,
                  (total * 100)::BIGINT    AS total_cents,
                  currency, coupon_code, billing_name, billing_email,
                  completed_at, created_at
           FROM orders WHERE order_number = $1 AND user_id = $2"#
    )
    .bind(&order_number)
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Order not found"}))))?;

    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderItemRow {
        id: i64,
        product_id: Option<i64>,
        plan_id: Option<i64>,
        name: String,
        quantity: i32,
        unit_price_cents: i64,
        total_cents: i64,
    }

    let items: Vec<OrderItemRow> = sqlx::query_as(
        r#"SELECT id, product_id, plan_id, name, quantity,
                  (unit_price * 100)::BIGINT AS unit_price_cents,
                  (total      * 100)::BIGINT AS total_cents
           FROM order_items WHERE order_id = $1"#
    )
    .bind(order.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "order": order,
        "items": items
    })))
}

/// Get user's order history
async fn get_orders(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderSummary {
        id: i64,
        order_number: String,
        status: String,
        total_cents: i64,
        currency: String,
        created_at: chrono::NaiveDateTime,
    }

    let orders: Vec<OrderSummary> = sqlx::query_as(
        r#"SELECT id, order_number, status,
                  (total * 100)::BIGINT AS total_cents,
                  currency, created_at
           FROM orders WHERE user_id = $1 ORDER BY created_at DESC"#
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"orders": orders})))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_checkout))
        .route("/calculate-tax", post(calculate_tax))
        .route("/orders", get(get_orders))
        .route("/orders/:order_number", get(get_order))
}
