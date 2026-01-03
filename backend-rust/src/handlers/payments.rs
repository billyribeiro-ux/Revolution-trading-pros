//! Payments Handlers (Stripe)
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct PaymentConfigResponse {
    pub publishable_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateIntentRequest {
    pub amount: i64,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaymentIntentResponse {
    pub client_secret: String,
    pub payment_intent_id: String,
}

pub async fn config(State(state): State<AppState>) -> Json<ApiResponse<PaymentConfigResponse>> {
    Json(ApiResponse::success(PaymentConfigResponse {
        publishable_key: state.config.stripe.publishable_key.clone(),
    }))
}

pub async fn create_intent(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<CreateIntentRequest>) -> Result<Json<ApiResponse<PaymentIntentResponse>>, AppError> {
    // TODO: Implement Stripe payment intent creation
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn create_checkout(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn confirm(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn order_status(State(_state): State<AppState>, _auth: AuthUser, Path(_order_number): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "status": "pending"})))
}
