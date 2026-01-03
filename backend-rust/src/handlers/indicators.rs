//! Indicators Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::{Path, State}, Json};
use serde::Serialize;

use crate::{errors::AppError, responses::ApiResponse, AppState};

#[derive(Debug, Serialize)]
pub struct IndicatorResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub async fn index(State(_state): State<AppState>) -> Result<Json<ApiResponse<Vec<IndicatorResponse>>>, AppError> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn show(State(_state): State<AppState>, Path(_slug): Path<String>) -> Result<Json<serde_json::Value>, AppError> {
    Err(AppError::NotFound("Indicator not found".to_string()))
}
