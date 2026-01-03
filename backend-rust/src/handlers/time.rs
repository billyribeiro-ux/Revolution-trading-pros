//! Time Handler
//!
//! ICT 11+ Principal Engineer Grade

use axum::Json;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
pub struct TimeResponse {
    pub now: String,
    pub timestamp: i64,
}

/// Get current server time
pub async fn now() -> Json<TimeResponse> {
    let now = Utc::now();
    Json(TimeResponse {
        now: now.to_rfc3339(),
        timestamp: now.timestamp(),
    })
}
