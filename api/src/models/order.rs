//! Order & Payment models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
    Cancelled,
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub order_number: String,
    pub status: String,
    pub subtotal: f64,
    pub discount: f64,
    pub tax: f64,
    pub total: f64,
    pub currency: String,
    pub payment_provider: Option<String>,
    pub payment_intent_id: Option<String>,
    pub stripe_session_id: Option<String>,
    pub coupon_id: Option<i64>,
    pub coupon_code: Option<String>,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub completed_at: Option<NaiveDateTime>,
    pub refunded_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Order item entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub total: f64,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
}

/// Order with items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderWithItems {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
}

/// Create order request (checkout)
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrder {
    pub items: Vec<CreateOrderItem>,
    pub coupon_code: Option<String>,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub payment_method_id: Option<String>,
}

/// Create order item
#[derive(Debug, Deserialize)]
pub struct CreateOrderItem {
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub quantity: Option<i32>,
}

/// Checkout session response
#[derive(Debug, Serialize)]
pub struct CheckoutSession {
    pub session_id: String,
    pub url: String,
    pub order_id: i64,
    pub order_number: String,
}

/// Coupon entity - matches Laravel production schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Coupon {
    pub id: i64,
    pub code: String,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub coupon_type: String, // percentage, fixed
    pub value: Decimal,
    pub max_uses: i32,
    pub current_uses: i32,
    pub expiry_date: Option<NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub min_purchase_amount: Decimal,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create coupon request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCoupon {
    #[validate(length(min = 3, max = 50))]
    pub code: String,
    pub description: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub discount_type: String,
    #[validate(range(min = 0.0))]
    pub discount_value: f64,
    pub min_purchase: Option<f64>,
    pub max_discount: Option<f64>,
    pub usage_limit: Option<i32>,
    pub is_active: Option<bool>,
    pub starts_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub applicable_plans: Option<serde_json::Value>,
}

/// Validate coupon response
#[derive(Debug, Serialize)]
pub struct ValidateCouponResponse {
    pub valid: bool,
    pub coupon: Option<Coupon>,
    pub error: Option<String>,
    pub discount_amount: Option<f64>,
}

/// Invoice entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i64,
    pub user_id: i64,
    pub subscription_id: Option<i64>,
    pub order_id: Option<i64>,
    pub invoice_number: String,
    pub status: String,
    pub subtotal: f64,
    pub discount: f64,
    pub tax: f64,
    pub total: f64,
    pub currency: String,
    pub stripe_invoice_id: Option<String>,
    pub pdf_url: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub paid_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
