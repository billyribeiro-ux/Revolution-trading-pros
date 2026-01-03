//! Posts Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::Serialize;

use crate::{errors::AppError, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub published_at: Option<String>,
}

pub async fn index(State(_state): State<AppState>) -> Result<Json<ApiResponse<Vec<PostResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, Path(_slug): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Post not found".to_string()))
}
