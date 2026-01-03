//! Trading Rooms Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::Serialize;

use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct TradingRoomResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub async fn index(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<TradingRoomResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, _auth: AuthUser, Path(_slug): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Trading room not found".to_string()))
}

pub async fn videos(State(_state): State<AppState>, _auth: AuthUser, Path(_slug): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": []})))
}

pub async fn generate_sso(State(_state): State<AppState>, _auth: AuthUser, Path(_slug): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::Internal("Not implemented".to_string()))
}
