//! Cart and checkout routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::product::{CheckoutRequest, CartItem};
use crate::services::JwtService;

/// POST /api/cart/checkout - Process checkout
pub async fn checkout(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = get_authenticated_user(&req, &ctx).await?;
    
    let body: CheckoutRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    if body.items.is_empty() {
        return Err(ApiError::BadRequest("Cart is empty".to_string()).into());
    }

    // Calculate totals
    let mut subtotal: i64 = 0;
    let mut discount: i64 = 0;
    
    for item in &body.items {
        subtotal += item.price * item.quantity as i64;
    }

    // Apply coupon if provided
    if let Some(coupon_code) = &body.coupon_code {
        let coupon: Option<crate::models::product::Coupon> = ctx.data.db.query_one(
            r#"
            SELECT * FROM coupons 
            WHERE code = $1 AND is_active = true 
            AND (starts_at IS NULL OR starts_at <= $2)
            AND (expires_at IS NULL OR expires_at > $2)
            AND (usage_limit IS NULL OR usage_count < usage_limit)
            "#,
            vec![
                serde_json::json!(coupon_code.to_uppercase()),
                serde_json::json!(crate::utils::now().to_rfc3339()),
            ]
        ).await?;

        if let Some(c) = coupon {
            discount = match c.discount_type {
                crate::models::product::DiscountType::Percentage => {
                    (subtotal * c.discount_value) / 100
                }
                crate::models::product::DiscountType::FixedAmount => {
                    c.discount_value
                }
            };

            // Apply max discount cap if set
            if let Some(max) = c.max_discount {
                discount = discount.min(max);
            }
        }
    }

    // Calculate tax (simplified - in production use tax service)
    let tax_rate = 0; // 0% for digital goods in most cases
    let tax = ((subtotal - discount) * tax_rate) / 100;
    let total = subtotal - discount + tax;

    // Create Stripe payment intent
    let stripe = &ctx.data.services.stripe;
    let payment_intent = stripe.create_payment_intent(
        total,
        "usd",
        None, // customer_id
        Some(serde_json::json!({
            "user_id": user.id.to_string(),
            "items": body.items.len()
        })),
    ).await?;

    // Create order
    let order_id = uuid::Uuid::new_v4();
    let order_number = format!("RTP-{}", crate::utils::now().format("%Y%m%d%H%M%S"));
    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        INSERT INTO orders (
            id, user_id, order_number, status, subtotal, discount, tax, total,
            currency, payment_intent_id, created_at, updated_at
        ) VALUES ($1, $2, $3, 'pending', $4, $5, $6, $7, 'usd', $8, $9, $9)
        "#,
        vec![
            serde_json::json!(order_id.to_string()),
            serde_json::json!(user.id.to_string()),
            serde_json::json!(order_number),
            serde_json::json!(subtotal),
            serde_json::json!(discount),
            serde_json::json!(tax),
            serde_json::json!(total),
            serde_json::json!(payment_intent.id),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Create order items
    for item in &body.items {
        ctx.data.db.execute(
            r#"
            INSERT INTO order_items (id, order_id, product_id, product_name, quantity, unit_price, total_price, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            vec![
                serde_json::json!(uuid::Uuid::new_v4().to_string()),
                serde_json::json!(order_id.to_string()),
                serde_json::json!(item.product_id.to_string()),
                serde_json::json!(item.name),
                serde_json::json!(item.quantity),
                serde_json::json!(item.price),
                serde_json::json!(item.price * item.quantity as i64),
                serde_json::json!(now.to_rfc3339()),
            ]
        ).await?;
    }

    Response::from_json(&serde_json::json!({
        "order_id": order_id,
        "order_number": order_number,
        "client_secret": payment_intent.client_secret,
        "subtotal": subtotal,
        "discount": discount,
        "tax": tax,
        "total": total
    }))
}

/// POST /api/cart/calculate-tax - Calculate tax for cart
pub async fn calculate_tax(mut req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    #[derive(serde::Deserialize)]
    struct TaxRequest {
        items: Vec<CartItem>,
        country: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
    }

    let body: TaxRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let subtotal: i64 = body.items.iter()
        .map(|item| item.price * item.quantity as i64)
        .sum();

    // Simplified tax calculation
    // In production, integrate with a tax service like TaxJar or Avalara
    let tax_rate = match body.country.as_deref() {
        Some("US") => {
            // US sales tax varies by state
            match body.state.as_deref() {
                Some("CA") => 725, // 7.25%
                Some("NY") => 800, // 8%
                Some("TX") => 625, // 6.25%
                _ => 0, // Digital goods often exempt
            }
        }
        Some("GB") => 2000, // UK VAT 20%
        Some("DE") => 1900, // Germany VAT 19%
        Some("FR") => 2000, // France VAT 20%
        _ => 0,
    };

    let tax = (subtotal * tax_rate) / 10000;

    Response::from_json(&serde_json::json!({
        "subtotal": subtotal,
        "tax_rate": tax_rate as f64 / 100.0,
        "tax": tax,
        "total": subtotal + tax
    }))
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
