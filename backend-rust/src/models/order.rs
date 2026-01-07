//! Order Model
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_number: String,
    pub status: String,
    pub subtotal: f64,
    pub tax: f64,
    pub discount: f64,
    pub total: f64,
    pub currency: String,
    pub payment_method: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct OrderWithItems {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_number: String,
    pub status: String,
    pub subtotal: f64,
    pub tax: f64,
    pub discount: f64,
    pub total: f64,
    pub currency: String,
    pub payment_method: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub items: Vec<OrderItem>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct OrderSummary {
    pub id: Uuid,
    pub order_number: String,
    pub status: String,
    pub total: f64,
    pub currency: String,
    pub item_count: i32,
    pub created_at: DateTime<Utc>,
}
