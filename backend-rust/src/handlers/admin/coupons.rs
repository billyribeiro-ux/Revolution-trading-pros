//! Admin Coupons Handlers
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

pub async fn index(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn store(State(_state): State<AppState>, _auth: AuthUser, Json(_payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn show(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Coupon not found".to_string()))
}

pub async fn update(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>, Json(_payload): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn destroy(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}

pub async fn user_coupons(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}
