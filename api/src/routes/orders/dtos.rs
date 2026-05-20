//! Order DTOs — shared request/response types.
//!
//! R23-B (2026-05-20): extracted from the original 1,382-LOC
//! `orders.rs` as a pure structural move. The following identifiers
//! are re-exported from `mod.rs` so the public paths
//! `routes::orders::{OrderDetailResponse, OrderItemDetailResponse,
//! AdminOrderResponse, AdminOrderStats, RefundOrderRequest}` are
//! preserved byte-for-byte (the integration tests in
//! `tests/orders_test.rs` depend on them).

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// Response Types - ICT 11+ Grade
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub currency: String,
    pub item_count: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub payment_provider: Option<String>,
    pub coupon_code: Option<String>,
    pub items: Vec<OrderItemDetailResponse>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemDetailResponse {
    pub id: i64,
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub total_cents: i64,
    pub product_type: Option<String>,
    pub product_slug: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Database Row Types
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, sqlx::FromRow)]
pub(super) struct OrderRow {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub billing_name: Option<String>,
    pub billing_email: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub payment_provider: Option<String>,
    pub coupon_code: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct OrderItemRow {
    pub id: i64,
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub name: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub total_cents: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct ProductMeta {
    pub product_type: Option<String>,
    pub slug: Option<String>,
    pub thumbnail: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN DTOs
// ═══════════════════════════════════════════════════════════════════════════

/// Admin order response with user details
#[derive(Debug, Serialize)]
pub struct AdminOrderResponse {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub total_cents: i64,
    pub currency: String,
    pub user_email: String,
    pub user_name: Option<String>,
    pub payment_provider: Option<String>,
    pub item_count: i32,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// Admin order query params
#[derive(Debug, Deserialize)]
pub struct AdminOrderListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Admin order stats (monetary fields in integer cents)
#[derive(Debug, Serialize)]
pub struct AdminOrderStats {
    pub total_orders: i64,
    pub completed_orders: i64,
    pub pending_orders: i64,
    pub refunded_orders: i64,
    pub total_revenue_cents: i64,
    pub revenue_this_month_cents: i64,
    pub average_order_value_cents: i64,
}

/// Update order status request
#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
    pub notes: Option<String>,
}

/// Refund order request (amount in integer cents; None = full refund)
#[derive(Debug, Deserialize)]
pub struct RefundOrderRequest {
    pub amount_cents: Option<i64>,
    pub reason: Option<String>,
}
