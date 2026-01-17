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
    pub success_url: Option<String>,
    pub cancel_url: Option<String>,
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
                "SELECT id, name, price FROM membership_plans WHERE id = $1 AND is_active = true",
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

            subtotal += plan.price;

            line_items.push(json!({
                "type": "subscription",
                "id": plan.id,
                "name": plan.name,
                "price": plan.price,
                "quantity": 1,
                "total": plan.price
            }));
        }
    }

    // Apply coupon if provided
    let mut discount = 0.0_f64;
    let mut coupon_info = None;

    if let Some(ref code) = input.coupon_code {
        #[derive(sqlx::FromRow)]
        struct CouponInfo {
            id: i64,
            code: String,
            discount_type: String,
            discount_value: f64,
            max_discount: Option<f64>,
        }

        let coupon: Option<CouponInfo> = sqlx::query_as(
            "SELECT id, code, discount_type, discount_value, max_discount FROM coupons WHERE UPPER(code) = UPPER($1) AND is_active = true"
        )
        .bind(code)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        if let Some(c) = coupon {
            discount = if c.discount_type == "percent" {
                subtotal * (c.discount_value / 100.0)
            } else {
                c.discount_value
            };

            // Apply max discount if set
            if let Some(max) = c.max_discount {
                if discount > max {
                    discount = max;
                }
            }

            coupon_info = Some(json!({
                "id": c.id,
                "code": c.code,
                "discount": discount
            }));
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

    // Create order in database
    #[derive(sqlx::FromRow)]
    struct OrderId {
        id: i64,
    }

    let order: OrderId = sqlx::query_as(
        r#"
        INSERT INTO orders (user_id, order_number, status, subtotal, discount, tax, total, currency, billing_name, billing_email, billing_address, created_at, updated_at)
        VALUES ($1, $2, 'pending', $3, $4, $5, $6, 'USD', $7, $8, $9, NOW(), NOW())
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
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Insert order items
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
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }

    // TODO: Create Stripe checkout session for payment
    // For now, return order details
    let success_url = input
        .success_url
        .unwrap_or_else(|| "/checkout/success".to_string());
    let cancel_url = input
        .cancel_url
        .unwrap_or_else(|| "/checkout/cancel".to_string());

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
        "checkout_url": format!("/checkout/pay/{}", order_number),
        "success_url": success_url,
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
