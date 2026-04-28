//! Order Service Types
//! Simplified types for order service queries.
//! All monetary values are integer cents per architecture standard §1.2.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub status: String,
    pub total_cents: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub item_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderWithItems {
    pub id: Uuid,
    pub order_number: String,
    pub status: String,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub discount_cents: i64,
    pub total_cents: i64,
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
    pub unit_price_cents: i64,
    pub total_cents: i64,
}
