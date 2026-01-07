//! Analytics Handlers
//!
//! Apple ICT 11+ Principal Engineer Grade
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! Design Principles:
//! 1. OBSERVABILITY: Log both success AND failure - silent failures are bugs
//! 2. FIRE-AND-FORGET: Return 204 No Content to prevent CORB (sendBeacon pattern)
//! 3. RESILIENCE: Accept malformed payloads gracefully but log them for debugging
//! 4. STRUCTURED LOGGING: Use tracing spans for distributed tracing correlation
//!
//! Why 204 No Content?
//! - sendBeacon is fire-and-forget (doesn't read responses)
//! - JSON responses trigger CORB (Cross-Origin Read Blocking)
//! - No body = nothing for browser to block
//!
//! ═══════════════════════════════════════════════════════════════════════════

use axum::{body::Bytes, http::StatusCode};
use serde::Deserialize;

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event: String,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PageViewRequest {
    pub url: String,
    #[serde(default)]
    pub referrer: Option<String>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PerformanceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default)]
    pub rating: Option<String>,
    #[serde(default)]
    pub delta: Option<f64>,
    #[serde(default)]
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchEvent {
    pub event_name: String,
    #[serde(default)]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BatchRequest {
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub events: Vec<BatchEvent>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/analytics/track - Track custom events
///
/// ICT 11+ Pattern: Fire-and-forget with full observability
#[tracing::instrument(name = "analytics.track", skip(body), fields(bytes = body.len()))]
pub async fn track_event(body: Bytes) -> StatusCode {
    match serde_json::from_slice::<TrackEventRequest>(&body) {
        Ok(payload) => {
            tracing::info!(
                event = %payload.event,
                has_properties = payload.properties.is_some(),
                "Analytics event tracked"
            );
        }
        Err(e) => {
            // ICT 11+: NEVER silently swallow errors - log for debugging
            tracing::warn!(
                error = %e,
                body_preview = %String::from_utf8_lossy(&body[..body.len().min(200)]),
                "Failed to parse analytics event"
            );
        }
    }
    StatusCode::NO_CONTENT
}

/// POST /api/analytics/pageview - Track page views
#[tracing::instrument(name = "analytics.pageview", skip(body), fields(bytes = body.len()))]
pub async fn track_pageview(body: Bytes) -> StatusCode {
    match serde_json::from_slice::<PageViewRequest>(&body) {
        Ok(payload) => {
            tracing::info!(
                url = %payload.url,
                referrer = ?payload.referrer,
                "Page view tracked"
            );
        }
        Err(e) => {
            tracing::warn!(
                error = %e,
                body_preview = %String::from_utf8_lossy(&body[..body.len().min(200)]),
                "Failed to parse pageview"
            );
        }
    }
    StatusCode::NO_CONTENT
}

/// POST /api/analytics/performance - Track Core Web Vitals
///
/// Accepts: LCP, INP, CLS, FCP, TTFB metrics from frontend
#[tracing::instrument(name = "analytics.performance", skip(body), fields(bytes = body.len()))]
pub async fn track_performance(body: Bytes) -> StatusCode {
    match serde_json::from_slice::<PerformanceRequest>(&body) {
        Ok(payload) => {
            // ICT 11+: Structured logging with all relevant fields
            tracing::info!(
                metric_name = ?payload.name,
                metric_value = ?payload.value,
                metric_rating = ?payload.rating,
                metric_delta = ?payload.delta,
                "Performance metric received"
            );
        }
        Err(e) => {
            tracing::warn!(
                error = %e,
                body_preview = %String::from_utf8_lossy(&body[..body.len().min(200)]),
                "Failed to parse performance metric"
            );
        }
    }
    StatusCode::NO_CONTENT
}

/// POST /api/analytics/batch - Batch analytics events
///
/// ICT 11+: Efficiently process multiple events in single request
#[tracing::instrument(name = "analytics.batch", skip(body), fields(bytes = body.len()))]
pub async fn track_batch(body: Bytes) -> StatusCode {
    match serde_json::from_slice::<BatchRequest>(&body) {
        Ok(payload) => {
            let event_count = payload.events.len();
            tracing::info!(
                session_id = ?payload.session_id,
                event_count = event_count,
                "Batch analytics received"
            );

            // Log individual events at debug level for detailed tracing
            for (i, event) in payload.events.iter().enumerate() {
                tracing::debug!(
                    batch_index = i,
                    event_name = %event.event_name,
                    has_properties = event.properties.is_some(),
                    "Batch event"
                );
            }
        }
        Err(e) => {
            tracing::warn!(
                error = %e,
                body_preview = %String::from_utf8_lossy(&body[..body.len().min(200)]),
                "Failed to parse batch analytics"
            );
        }
    }
    StatusCode::NO_CONTENT
}
