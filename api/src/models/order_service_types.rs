//! Order Service Types
//! Simplified types for order service queries

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub status: String,
    pub total: rust_decimal::Decimal,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub item_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderWithItems {
    pub id: Uuid,
    pub order_number: String,
    pub status: String,
    pub subtotal: rust_decimal::Decimal,
    pub tax: rust_decimal::Decimal,
    pub discount: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub currency: String,
    pub payment_method: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub items: Vec<OrderItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemData {
    pub id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub price: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
}
