//! Popups Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{errors::AppError, responses::ApiResponse, AppState};

pub async fn active(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn impression(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}

pub async fn conversion(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}
