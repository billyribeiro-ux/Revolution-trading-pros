//! Subscriptions Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: String,
    pub plan_name: String,
    pub status: String,
    pub price: String,
    pub billing_period: String,
    pub start_date: String,
    pub next_payment_date: Option<String>,
}

pub async fn index(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<SubscriptionResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": null})))
}

pub async fn cancel(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Subscription cancelled"})))
}

pub async fn pause(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Subscription paused"})))
}

pub async fn resume(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Subscription resumed"})))
}

pub async fn reactivate(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "message": "Subscription reactivated"})))
}

pub async fn invoices(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": []})))
}

pub async fn payments(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": []})))
}
