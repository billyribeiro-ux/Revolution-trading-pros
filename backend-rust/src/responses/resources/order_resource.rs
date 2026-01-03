//! Order Resource
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OrderResource {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub currency: String,
    pub item_count: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResource {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub subtotal: String,
    pub tax: String,
    pub discount: String,
    pub currency: String,
    pub payment_method: Option<String>,
    pub items: Vec<OrderItemResource>,
    pub billing_address: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResource {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
}
