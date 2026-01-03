//! Videos Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{errors::AppError, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct VideoResponse {
    pub id: String,
    pub title: String,
    pub duration: Option<i32>,
    pub thumbnail_url: Option<String>,
}

pub async fn index(State(_state): State<AppState>) -> Result<Json<ApiResponse<Vec<VideoResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, Path(_id): Path<Uuid>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Video not found".to_string()))
}
