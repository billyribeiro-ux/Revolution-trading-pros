//! Cart Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{errors::AppError, extractors::AuthUser, AppState};

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub items: Vec<CartItem>,
    pub coupon_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CalculateTaxRequest {
    pub subtotal: f64,
    pub country: String,
    pub state: Option<String>,
}

pub async fn checkout(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<CheckoutRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn calculate_tax(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<CalculateTaxRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "tax": 0.0})))
}
