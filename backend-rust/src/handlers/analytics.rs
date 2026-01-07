//! Analytics Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{body::Bytes, http::StatusCode};
use serde::Deserialize;

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

/// POST /api/analytics/track - Track custom events
/// ICT 11+: Returns 204 No Content to prevent CORB blocking (sendBeacon doesn't read responses)
pub async fn track_event(body: Bytes) -> StatusCode {
    // Parse JSON from raw bytes (supports text/plain from sendBeacon)
    if let Ok(payload) = serde_json::from_slice::<TrackEventRequest>(&body) {
        tracing::debug!(event = %payload.event, "Analytics event tracked");
    }
    StatusCode::NO_CONTENT
}

/// POST /api/analytics/pageview - Track page views
/// ICT 11+: Returns 204 No Content to prevent CORB blocking
pub async fn track_pageview(body: Bytes) -> StatusCode {
    if let Ok(payload) = serde_json::from_slice::<PageViewRequest>(&body) {
        tracing::debug!(url = %payload.url, "Page view tracked");
    }
    StatusCode::NO_CONTENT
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
/// Returns 204 No Content to prevent CORB blocking (sendBeacon is fire-and-forget)
pub async fn track_performance(body: Bytes) -> StatusCode {
    // Parse the body as JSON regardless of Content-Type
    // sendBeacon sends as text/plain but the content is JSON
    if let Ok(payload) = serde_json::from_slice::<PerformanceRequest>(&body) {
        tracing::debug!(
            name = ?payload.name,
            value = ?payload.value,
            rating = ?payload.rating,
            "Performance metric received"
        );
    }
    // Return 204 No Content - no body means no CORB issues
    StatusCode::NO_CONTENT
}

/// POST /api/analytics/batch - Batch analytics events
/// ICT 11+: Returns 204 No Content to prevent CORB blocking
pub async fn track_batch(body: Bytes) -> StatusCode {
    // Just accept the batch - in production, parse and store
    tracing::debug!(bytes = body.len(), "Batch analytics received");
    StatusCode::NO_CONTENT
}
