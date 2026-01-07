//! Analytics Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, body::Bytes, Json};
use serde::Deserialize;

use crate::{errors::AppError, AppState};

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event: String,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PageViewRequest {
    pub url: String,
    pub referrer: Option<String>,
}

pub async fn track_event(State(_state): State<AppState>, Json(_payload): Json<TrackEventRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}

pub async fn track_pageview(State(_state): State<AppState>, Json(_payload): Json<PageViewRequest>) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true})))
}

#[derive(Debug, Deserialize)]
pub struct PerformanceRequest {
    pub name: Option<String>,
    pub value: Option<f64>,
    pub rating: Option<String>,
    pub delta: Option<f64>,
    pub id: Option<String>,
}

/// POST /api/analytics/performance - Track performance metrics
/// ICT 11+: Accept raw bytes to support sendBeacon which sends text/plain
/// This avoids CORS preflight issues since text/plain is a "simple" content-type
pub async fn track_performance(body: Bytes) -> Result<Json<serde_json::Value>, AppError> {
    // Parse the body as JSON regardless of Content-Type
    // sendBeacon sends as text/plain but the content is JSON
    let _payload: PerformanceRequest = serde_json::from_slice(&body)
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON: {}", e)))?;
    
    // Log the metric (in production, you'd store this)
    tracing::debug!(
        name = ?_payload.name,
        value = ?_payload.value,
        rating = ?_payload.rating,
        "Performance metric received"
    );
    
    Ok(Json(serde_json::json!({"success": true})))
}
