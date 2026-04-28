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

/// Product row for cart calculations
#[derive(Debug, sqlx::FromRow)]
struct ProductPrice {
    id: i64,
    name: String,
    price: f64,
}

/// Plan row for cart calculations
#[derive(Debug, sqlx::FromRow)]
struct PlanPrice {
    id: i64,
    name: String,
    price: f64,
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
    let mut subtotal = 0.0_f64;

    // Calculate totals from products
    for item in &input.items {
        if let Some(product_id) = item.product_id {
            let product: ProductPrice = sqlx::query_as(
                "SELECT id, name, price FROM products WHERE id = $1 AND is_active = true",
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
            let item_total = product.price * qty as f64;
            subtotal += item_total;

            line_items.push(json!({
                "type": "product",
                "id": product.id,
                "name": product.name,
                "price": product.price,
                "quantity": qty,
                "total": item_total
            }));
        }

        if let Some(plan_id) = item.plan_id {
            let plan: PlanPrice = sqlx::query_as(
                "SELECT id, name, price::FLOAT8 as price, stripe_price_id, billing_cycle FROM membership_plans WHERE id = $1 AND is_active = true",
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

            subtotal += plan.price;

            line_items.push(json!({
                "type": "subscription",
                "id": plan.id,
                "name": plan.name,
                "price": plan.price,
                "stripe_price_id": plan.stripe_price_id,
                "billing_cycle": plan.billing_cycle,
                "quantity": 1,
                "total": plan.price
            }));
        }
    }

    // Apply coupon if provided - ICT 7 FIX: Complete coupon validation
    let mut discount = 0.0_f64;
    let mut coupon_info = None;
    let mut applied_coupon_id: Option<i64> = None;

    if let Some(ref code) = input.coupon_code {
        #[derive(sqlx::FromRow)]
        struct CouponInfo {
            id: i64,
            code: String,
            #[sqlx(rename = "type")]
            coupon_type: String,
            value: f64,
            max_uses: i32,
            current_uses: i32,
            expiry_date: Option<chrono::NaiveDateTime>,
            min_purchase_amount: f64,
        }

        // ICT 7 FIX: Use Laravel production schema column names
        let coupon: Option<CouponInfo> = sqlx::query_as(
            r#"SELECT id, code, type, value::FLOAT8 as value, max_uses, current_uses,
                      expiry_date, min_purchase_amount::FLOAT8 as min_purchase_amount
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
            // Validate expiry
            if let Some(expiry) = c.expiry_date {
                if expiry < chrono::Utc::now().naive_utc() {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Coupon has expired"})),
                    ));
                }
            }

            // Validate usage limits
            if c.max_uses > 0 && c.current_uses >= c.max_uses {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Coupon has reached its usage limit"})),
                ));
            }

            // Validate minimum purchase
            if c.min_purchase_amount > 0.0 && subtotal < c.min_purchase_amount {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": format!("Minimum purchase of ${:.2} required for this coupon", c.min_purchase_amount)}),
                    ),
                ));
            }

            // Calculate discount
            discount = if c.coupon_type == "percentage" {
                subtotal * (c.value / 100.0)
            } else {
                c.value
            };

            // Ensure discount doesn't exceed subtotal
            if discount > subtotal {
                discount = subtotal;
            }

            applied_coupon_id = Some(c.id);
            coupon_info = Some(json!({
                "id": c.id,
                "code": c.code,
                "type": c.coupon_type,
                "value": c.value,
                "discount": discount
            }));
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid coupon code"})),
            ));
        }
    }

    // Calculate tax (simplified - 0% for now, should integrate with tax service)
    let tax = 0.0_f64;
    let total = subtotal - discount + tax;

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
    let order: OrderId = sqlx::query_as(
        r#"
        INSERT INTO orders (user_id, order_number, status, subtotal, discount, tax, total, currency,
                           billing_name, billing_email, billing_address, coupon_id, coupon_code, created_at, updated_at)
        VALUES ($1, $2, 'pending', $3, $4, $5, $6, 'USD', $7, $8, $9, $10, $11, NOW(), NOW())
        RETURNING id
        "#
    )
    .bind(user.id)
    .bind(&order_number)
    .bind(subtotal)
    .bind(discount)
    .bind(tax)
    .bind(total)
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

    // Insert order items
    // Original pool reference: .execute(&state.db.pool)
    for item in &line_items {
        sqlx::query(
            r#"
            INSERT INTO order_items (order_id, product_id, plan_id, name, quantity, unit_price, total, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            "#
        )
        .bind(order.id)
        .bind(item.get("type").and_then(|t| if t == "product" { item.get("id").and_then(|i| i.as_i64()) } else { None }))
        .bind(item.get("type").and_then(|t| if t == "subscription" { item.get("id").and_then(|i| i.as_i64()) } else { None }))
        .bind(item.get("name").and_then(|n| n.as_str()).unwrap_or(""))
        .bind(item.get("quantity").and_then(|q| q.as_i64()).unwrap_or(1) as i32)
        .bind(item.get("price").and_then(|p| p.as_f64()).unwrap_or(0.0))
        .bind(item.get("total").and_then(|t| t.as_f64()).unwrap_or(0.0))
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
            amount: (item.get("price").and_then(|p| p.as_f64()).unwrap_or(0.0) * 100.0) as i64,
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
        total = %total,
        "Stripe checkout session created"
    );

    Ok(Json(json!({
        "order_id": order.id,
        "order_number": order_number,
        "line_items": line_items,
        "subtotal": subtotal,
        "discount": discount,
        "coupon": coupon_info,
        "tax": tax,
        "total": total,
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
        subtotal: f64,
        discount: f64,
        tax: f64,
        total: f64,
        currency: String,
        coupon_code: Option<String>,
        billing_name: Option<String>,
        billing_email: Option<String>,
        completed_at: Option<chrono::NaiveDateTime>,
        created_at: chrono::NaiveDateTime,
    }

    let order: OrderRow = sqlx::query_as(
        "SELECT id, user_id, order_number, status, subtotal, discount, tax, total, currency, coupon_code, billing_name, billing_email, completed_at, created_at FROM orders WHERE order_number = $1 AND user_id = $2"
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
        unit_price: f64,
        total: f64,
    }

    let items: Vec<OrderItemRow> = sqlx::query_as(
        "SELECT id, product_id, plan_id, name, quantity, unit_price, total FROM order_items WHERE order_id = $1"
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
        total: f64,
        currency: String,
        created_at: chrono::NaiveDateTime,
    }

    let orders: Vec<OrderSummary> = sqlx::query_as(
        "SELECT id, order_number, status, total, currency, created_at FROM orders WHERE user_id = $1 ORDER BY created_at DESC"
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
