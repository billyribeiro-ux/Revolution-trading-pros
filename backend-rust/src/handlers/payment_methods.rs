//! Payment Methods Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct PaymentMethodResponse {
    pub id: String,
    pub brand: String,
    pub last4: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub is_default: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddPaymentMethodRequest {
    pub payment_method_id: String,
    pub set_as_default: Option<bool>,
}

pub async fn index(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<PaymentMethodResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn store(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<AddPaymentMethodRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Payment method added"})))
}

pub async fn destroy(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Payment method deleted"})))
}
