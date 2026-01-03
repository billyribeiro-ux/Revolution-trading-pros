//! Admin Members Handlers
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use crate::{errors::AppError, extractors::AuthUser, responses::ApiResponse, AppState};

pub async fn index(State(_state): State<AppState>, _auth: AuthUser) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, _auth: AuthUser, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Member not found".to_string()))
}
